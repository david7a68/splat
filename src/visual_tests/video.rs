//! One video per scenario. Frames go straight to ffmpeg, so no frame files exist.
use ab_glyph::{Font, FontRef, PxScale, ScaleFont, point};
use eframe::egui;
use image::{Rgba, RgbaImage};
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
};

const FPS: usize = 12;
/// About one second, so a viewer can read each settled state.
pub const HOLD_FRAMES: usize = FPS;
/// A short pause on the target before it is used.
pub const AIM_FRAMES: usize = FPS / 3;
pub const PRESS_FRAMES: usize = FPS / 4;
const MOVE_FRAMES: usize = FPS / 2;

// Films are at the minimum window, where scenarios work.
const WIDTH: u32 = 960;
const HEIGHT: u32 = 640;
const POINTER_RADIUS: f32 = 9.0;
const PRESS_RADIUS: f32 = 17.0;
const CAPTION_SIZE: f32 = 17.0;
const ACCENT: Rgba<u8> = Rgba([255, 196, 0, 255]);
const SHADOW: Rgba<u8> = Rgba([0, 0, 0, 255]);

/// What the video adds on top of the app's own frame. The screenshots have none of it.
#[derive(Default)]
pub struct Overlay {
    pub pointer: Option<egui::Pos2>,
    pub target: Option<egui::Rect>,
    pub pressed: bool,
    pub caption: Option<String>,
}

impl Overlay {
    pub fn caption(text: &str) -> Self {
        Self {
            caption: Some(text.to_owned()),
            ..Default::default()
        }
    }
}

pub struct Recorder {
    child: Child,
    stdin: Option<ChildStdin>,
    path: PathBuf,
    font: FontRef<'static>,
}

impl Recorder {
    pub fn start(path: &Path) -> Result<Self, String> {
        // ffmpeg is a host program. The shell's library path holds Nix libraries.
        let child = Command::new(ffmpeg_path())
            .env_remove("LD_LIBRARY_PATH")
            .args(["-hide_banner", "-loglevel", "error", "-y"])
            .args(["-f", "rawvideo", "-pixel_format", "rgba"])
            .args(["-video_size", &format!("{WIDTH}x{HEIGHT}")])
            .args(["-framerate", &FPS.to_string(), "-i", "-", "-an"])
            .args(["-c:v", "libx264", "-preset", "veryfast", "-crf", "22"])
            .args(["-pix_fmt", "yuv420p", "-movflags", "+faststart"])
            .args(["-threads", "2"])
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .map_err(|error| format!("Could not start ffmpeg: {error}"))?;
        let mut child = child;
        let stdin = child.stdin.take();
        Ok(Self {
            child,
            stdin,
            path: path.to_path_buf(),
            font: FontRef::try_from_slice(epaint_default_fonts::UBUNTU_LIGHT)
                .map_err(|error| format!("Could not read the caption font: {error}"))?,
        })
    }

    /// Glide the pointer to a control and mark it, over the unchanged app frame.
    pub fn aim(&mut self, image: &RgbaImage, from: egui::Pos2, target: egui::Rect, caption: &str) {
        let to = target.center();
        for step in 1..=MOVE_FRAMES {
            let overlay = Overlay {
                pointer: Some(from.lerp(to, step as f32 / MOVE_FRAMES as f32)),
                caption: Some(caption.to_owned()),
                ..Default::default()
            };
            self.frames(image, &overlay, 1);
        }
        let overlay = Overlay {
            pointer: Some(to),
            target: target.is_positive().then_some(target),
            caption: Some(caption.to_owned()),
            ..Default::default()
        };
        self.frames(image, &overlay, AIM_FRAMES);
    }

    pub fn frames(&mut self, image: &RgbaImage, overlay: &Overlay, count: usize) {
        let mut frame = image.clone();
        assert_eq!(frame.dimensions(), (WIDTH, HEIGHT), "video frame size");
        if let Some(target) = overlay.target {
            outline(&mut frame, target.expand(3.0), ACCENT);
        }
        if let Some(pointer) = overlay.pointer {
            if overlay.pressed {
                disc(&mut frame, pointer, PRESS_RADIUS, Rgba([255, 196, 0, 110]));
            }
            ring(&mut frame, pointer, POINTER_RADIUS, SHADOW, 3.5);
            ring(&mut frame, pointer, POINTER_RADIUS, ACCENT, 2.0);
            disc(&mut frame, pointer, 2.0, ACCENT);
        }
        if let Some(caption) = &overlay.caption {
            draw_caption(&mut frame, &self.font, caption);
        }
        let bytes = frame.as_raw();
        if let Some(stdin) = &mut self.stdin {
            for _ in 0..count {
                // A failure here shows at `finish`, with ffmpeg's own exit status.
                if stdin.write_all(bytes).is_err() {
                    return;
                }
            }
        }
    }

    pub fn finish(mut self) -> Result<String, String> {
        self.stdin = None;
        let status = self
            .child
            .wait()
            .map_err(|error| format!("ffmpeg did not finish: {error}"))?;
        if !status.success() {
            return Err(format!(
                "ffmpeg failed for {}: {status}",
                self.path.display()
            ));
        }
        Ok(self.path.display().to_string())
    }
}

impl Drop for Recorder {
    /// A failed scenario still leaves its journey up to the failure.
    fn drop(&mut self) {
        self.stdin = None;
        let _ = self.child.wait();
    }
}

fn ffmpeg_path() -> PathBuf {
    let host = PathBuf::from("/usr/bin/ffmpeg");
    if host.exists() {
        host
    } else {
        PathBuf::from("ffmpeg")
    }
}

fn blend(image: &mut RgbaImage, x: i64, y: i64, color: Rgba<u8>, coverage: f32) {
    if x < 0 || y < 0 || x >= i64::from(image.width()) || y >= i64::from(image.height()) {
        return;
    }
    let alpha = f32::from(color[3]) / 255.0 * coverage.clamp(0.0, 1.0);
    let pixel = image.get_pixel_mut(x as u32, y as u32);
    for channel in 0..3 {
        pixel[channel] =
            (f32::from(pixel[channel]) * (1.0 - alpha) + f32::from(color[channel]) * alpha) as u8;
    }
    pixel[3] = 255;
}

fn disc(image: &mut RgbaImage, center: egui::Pos2, radius: f32, color: Rgba<u8>) {
    let reach = radius.ceil() as i64 + 1;
    for dy in -reach..=reach {
        for dx in -reach..=reach {
            let (x, y) = (center.x as i64 + dx, center.y as i64 + dy);
            let distance = egui::pos2(x as f32 + 0.5, y as f32 + 0.5).distance(center);
            blend(image, x, y, color, radius + 0.5 - distance);
        }
    }
}

fn ring(image: &mut RgbaImage, center: egui::Pos2, radius: f32, color: Rgba<u8>, width: f32) {
    let reach = (radius + width).ceil() as i64 + 1;
    for dy in -reach..=reach {
        for dx in -reach..=reach {
            let (x, y) = (center.x as i64 + dx, center.y as i64 + dy);
            let distance = egui::pos2(x as f32 + 0.5, y as f32 + 0.5).distance(center);
            blend(
                image,
                x,
                y,
                color,
                width / 2.0 + 0.5 - (distance - radius).abs(),
            );
        }
    }
}

fn outline(image: &mut RgbaImage, rect: egui::Rect, color: Rgba<u8>) {
    let (left, right) = (rect.min.x as i64, rect.max.x as i64);
    let (top, bottom) = (rect.min.y as i64, rect.max.y as i64);
    for offset in 0..2 {
        for x in left..=right {
            blend(image, x, top + offset, color, 1.0);
            blend(image, x, bottom - offset, color, 1.0);
        }
        for y in top..=bottom {
            blend(image, left + offset, y, color, 1.0);
            blend(image, right - offset, y, color, 1.0);
        }
    }
}

fn draw_caption(image: &mut RgbaImage, font: &FontRef<'static>, text: &str) {
    let scale = PxScale::from(CAPTION_SIZE);
    let scaled = font.as_scaled(scale);
    let text: String = text.chars().filter(|c| !c.is_control()).take(90).collect();
    let width: f32 = text
        .chars()
        .map(|c| scaled.h_advance(scaled.glyph_id(c)))
        .sum();
    let (left, top) = (12.0, HEIGHT as f32 - 38.0);
    for y in top as i64..(top + 28.0) as i64 {
        for x in left as i64..(left + width + 20.0) as i64 {
            blend(image, x, y, Rgba([0, 0, 0, 215]), 1.0);
        }
    }
    let mut x = left + 10.0;
    let baseline = top + 20.0;
    for c in text.chars() {
        let id = scaled.glyph_id(c);
        let glyph = id.with_scale_and_position(scale, point(x, baseline));
        if let Some(outline) = font.outline_glyph(glyph) {
            let bounds = outline.px_bounds();
            outline.draw(|gx, gy, coverage| {
                blend(
                    image,
                    i64::from(gx) + bounds.min.x as i64,
                    i64::from(gy) + bounds.min.y as i64,
                    Rgba([255, 255, 255, 255]),
                    coverage,
                );
            });
        }
        x += scaled.h_advance(id);
    }
}
