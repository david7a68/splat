//! The renderer for captures and video frames.
//!
//! It draws as `egui_kittest::wgpu::WgpuTestRenderer` does, with two differences. It uses a
//! GPU when one is available: on the laptop's Intel GPU, a capture run took 94 CPU-s
//! instead of 133 with Mesa's software rasterizer. And kittest reads the frame back one
//! byte at a time, which cost about a tenth of a capture run's CPU. This one copies each
//! row at once.
//!
//! Compare captures only with captures from the same computer and renderer. Two runs on
//! one GPU give the same pixels. A GPU and the software rasterizer differ on a few edge
//! pixels, so `renderer.txt` in the output folder records which one drew a run.
use eframe::{egui, egui_wgpu, wgpu};
use egui_kittest::wgpu::{create_render_state, default_wgpu_setup};
use image::RgbaImage;
use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

const WAIT: Duration = Duration::from_secs(10);

static ADAPTER: OnceLock<String> = OnceLock::new();

/// The device that drew this run's frames, once one has drawn.
pub(super) fn adapter() -> Option<&'static str> {
    ADAPTER.get().map(String::as_str)
}

/// Lower is preferred. An integrated GPU comes first: it shares memory with the CPU, so
/// reading frames back is cheap, and it does not wake a discrete GPU.
/// `SPLAT_VISUAL_RENDERER=software` puts Mesa's software rasterizer first.
fn rank(device: wgpu::DeviceType, software: bool) -> u8 {
    match device {
        wgpu::DeviceType::Cpu if software => 0,
        wgpu::DeviceType::IntegratedGpu => 1,
        wgpu::DeviceType::DiscreteGpu => 2,
        wgpu::DeviceType::Cpu => 3,
        wgpu::DeviceType::VirtualGpu | wgpu::DeviceType::Other => 4,
    }
}

pub(super) struct CaptureRenderer {
    state: egui_wgpu::RenderState,
}

impl CaptureRenderer {
    /// Each worker thread keeps one device for all its scenarios and restarts.
    /// Every app still gets its own egui renderer, so no textures are shared.
    pub(super) fn new() -> Self {
        thread_local! {
            static DEVICE: egui_wgpu::RenderState = {
                let software =
                    std::env::var("SPLAT_VISUAL_RENDERER").is_ok_and(|value| value == "software");
                let mut setup = default_wgpu_setup();
                if let egui_wgpu::WgpuSetup::CreateNew(new) = &mut setup {
                    // devenv lists Mesa's Vulkan drivers. Other backends load drivers for nothing.
                    new.instance_descriptor.backends = wgpu::Backends::VULKAN;
                    new.native_adapter_selector = Some(Arc::new(move |adapters, _| {
                        adapters
                            .iter()
                            .min_by_key(|adapter| rank(adapter.get_info().device_type, software))
                            .cloned()
                            .ok_or_else(|| {
                                "No Vulkan adapter. Use `devenv shell -- visual-test`.".to_owned()
                            })
                    }));
                }
                let state = create_render_state(setup);
                let info = state.adapter.get_info();
                let _ = ADAPTER.set(format!(
                    "{} ({:?}, {} {})",
                    info.name, info.device_type, info.driver, info.driver_info
                ));
                state
            };
        }
        let mut state = DEVICE.with(Clone::clone);
        state.renderer = Arc::new(egui::mutex::RwLock::new(egui_wgpu::Renderer::new(
            &state.device,
            state.target_format,
            egui_wgpu::RendererOptions::PREDICTABLE,
        )));
        Self { state }
    }
}

impl egui_kittest::TestRenderer for CaptureRenderer {
    fn setup_eframe(&self, cc: &mut eframe::CreationContext<'_>, frame: &mut eframe::Frame) {
        cc.wgpu_render_state = Some(self.state.clone());
        frame.wgpu_render_state = Some(self.state.clone());
    }

    fn handle_delta(&mut self, delta: &egui::TexturesDelta) {
        let mut renderer = self.state.renderer.write();
        for (id, image) in &delta.set {
            renderer.update_texture(&self.state.device, &self.state.queue, *id, image);
        }
    }

    fn render(
        &mut self,
        ctx: &egui::Context,
        output: &egui::FullOutput,
    ) -> Result<RgbaImage, String> {
        let (device, queue) = (&self.state.device, &self.state.queue);
        let mut renderer = self.state.renderer.write();
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Capture encoder"),
        });
        let size = ctx.content_rect().size() * ctx.pixels_per_point();
        let screen = egui_wgpu::ScreenDescriptor {
            pixels_per_point: ctx.pixels_per_point(),
            size_in_pixels: [size.x.round() as u32, size.y.round() as u32],
        };
        let tessellated = ctx.tessellate(output.shapes.clone(), ctx.pixels_per_point());
        let user_buffers =
            renderer.update_buffers(device, queue, &mut encoder, &tessellated, &screen);
        let extent = wgpu::Extent3d {
            width: screen.size_in_pixels[0],
            height: screen.size_in_pixels[1],
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Capture"),
            size: extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.state.target_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        {
            let mut pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Capture pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    ..Default::default()
                })
                .forget_lifetime();
            renderer.render(&mut pass, &tessellated, &screen);
        }
        // Copy the frame into a buffer in the same submission. Rows are padded to the
        // copy alignment.
        let row = extent.width as usize * 4;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize;
        let padded = row.div_ceil(align) * align;
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Capture readback"),
            size: (padded * extent.height as usize) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded as u32),
                    rows_per_image: None,
                },
            },
            extent,
        );
        let submission = queue.submit(user_buffers.into_iter().chain([encoder.finish()]));
        let slice = buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| drop(sender.send(result)));
        device
            .poll(wgpu::PollType::Wait {
                submission_index: Some(submission),
                timeout: Some(WAIT),
            })
            .map_err(|error| format!("PollError: {error}"))?;
        receiver
            .recv()
            .map_err(|error| error.to_string())?
            .map_err(|error| error.to_string())?;
        let mapped = slice.get_mapped_range();
        let mut pixels = Vec::with_capacity(row * extent.height as usize);
        for line in mapped.chunks_exact(padded) {
            pixels.extend_from_slice(&line[..row]);
        }
        RgbaImage::from_raw(extent.width, extent.height, pixels)
            .ok_or_else(|| "The capture has the wrong size.".to_owned())
    }
}

#[test]
fn renderers_prefer_a_gpu_unless_software_is_asked_for() {
    use wgpu::DeviceType::{Cpu, DiscreteGpu, IntegratedGpu, Other};
    let order = |software| {
        let mut devices = [Other, Cpu, DiscreteGpu, IntegratedGpu];
        devices.sort_by_key(|device| rank(*device, software));
        devices
    };
    assert_eq!(order(false), [IntegratedGpu, DiscreteGpu, Cpu, Other]);
    assert_eq!(order(true), [Cpu, IntegratedGpu, DiscreteGpu, Other]);
}
