//! One video per scenario. Frames go straight to ffmpeg, so no frame files exist.
//!
//! Most frames repeat the one before: a held state is a second of the same picture. The
//! recorder sends each picture once, as YUV, with the time it appears, in a minimal
//! Matroska stream. ffmpeg then neither converts nor encodes repeats. Sending every
//! frame as RGBA cost about four times the CPU.
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
    /// Frames shown so far, at `FPS`. The next picture appears after them.
    shown: usize,
    /// The last picture sent, and how many frames it is shown for.
    last: Option<(Vec<u8>, usize)>,
}

impl Recorder {
    pub fn start(path: &Path) -> Result<Self, String> {
        // ffmpeg is a host program. The shell's library path holds Nix libraries.
        let child = Command::new(ffmpeg_path())
            .env_remove("LD_LIBRARY_PATH")
            .args(["-hide_banner", "-loglevel", "error", "-y"])
            .args(["-f", "matroska", "-i", "-", "-an"])
            // Keep each picture's own time: a held picture is one encoded frame.
            .args(["-fps_mode", "vfr"])
            // ultrafast took half the CPU of veryfast at the same quality, for files about
            // twice the size. The animation tune keeps flat UI colours smaller.
            .args([
                "-c:v",
                "libx264",
                "-preset",
                "ultrafast",
                "-tune",
                "animation",
            ])
            .args(["-crf", "22"])
            .args(["-pix_fmt", "yuv420p", "-movflags", "+faststart"])
            .args(["-threads", "2"])
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()
            .map_err(|error| format!("Could not start ffmpeg: {error}"))?;
        let mut child = child;
        let mut stdin = child.stdin.take();
        if let Some(stdin) = &mut stdin {
            // A failure here shows at `finish`, with ffmpeg's own exit status.
            let _ = stdin.write_all(&mkv::header(WIDTH, HEIGHT, FPS));
        }
        Ok(Self {
            child,
            stdin,
            path: path.to_path_buf(),
            font: FontRef::try_from_slice(epaint_default_fonts::UBUNTU_LIGHT)
                .map_err(|error| format!("Could not read the caption font: {error}"))?,
            shown: 0,
            last: None,
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
        let yuv = yuv420(&frame);
        let start = self.shown;
        self.shown += count;
        // The same picture again only lengthens the last one.
        if let Some((last, length)) = &mut self.last
            && *last == yuv
        {
            *length += count;
            return;
        }
        if let Some(stdin) = &mut self.stdin {
            // A failure here shows at `finish`, with ffmpeg's own exit status.
            let _ = stdin.write_all(&mkv::frame(start, FPS, &yuv));
        }
        self.last = Some((yuv, count));
    }

    pub fn finish(mut self) -> Result<String, String> {
        // A frame's time ends where the next one starts. Repeat the last picture on the last
        // frame, so the final hold keeps its length.
        if let (Some((last, length)), Some(stdin)) = (&self.last, &mut self.stdin)
            && *length > 1
        {
            let _ = stdin.write_all(&mkv::frame(self.shown - 1, FPS, last));
        }
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

/// The few Matroska elements ffmpeg needs to read raw I420 frames with their times.
mod mkv {
    /// An element with an eight-byte size, which every reader accepts.
    fn element(id: &[u8], body: &[u8]) -> Vec<u8> {
        let mut out = id.to_vec();
        out.push(0x01);
        out.extend_from_slice(&(body.len() as u64).to_be_bytes()[1..]);
        out.extend_from_slice(body);
        out
    }

    fn uint(id: &[u8], value: u64) -> Vec<u8> {
        let bytes = value.to_be_bytes();
        let first = bytes.iter().position(|&b| b != 0).unwrap_or(7);
        element(id, &bytes[first..])
    }

    /// The milliseconds at which frame `index` starts.
    fn millis(index: usize, fps: usize) -> u64 {
        (index as u64 * 1000 + fps as u64 / 2) / fps as u64
    }

    /// The stream header: one uncompressed video track. The segment's size is unknown, as
    /// for a live stream, so frames can follow until the pipe closes.
    pub(super) fn header(width: u32, height: u32, fps: usize) -> Vec<u8> {
        let mut out = element(
            &[0x1A, 0x45, 0xDF, 0xA3],
            &[
                uint(&[0x42, 0x86], 1),
                uint(&[0x42, 0xF7], 1),
                uint(&[0x42, 0xF2], 4),
                uint(&[0x42, 0xF3], 8),
                element(&[0x42, 0x82], b"matroska"),
                uint(&[0x42, 0x87], 4),
                uint(&[0x42, 0x85], 2),
            ]
            .concat(),
        );
        out.extend_from_slice(&[
            0x18, 0x53, 0x80, 0x67, 0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ]);
        out.extend(element(
            &[0x15, 0x49, 0xA9, 0x66],
            &[
                uint(&[0x2A, 0xD7, 0xB1], 1_000_000),
                element(&[0x4D, 0x80], b"splat"),
                element(&[0x57, 0x41], b"splat"),
            ]
            .concat(),
        ));
        let video = element(
            &[0xE0],
            &[
                uint(&[0xB0], u64::from(width)),
                uint(&[0xBA], u64::from(height)),
                element(&[0x2E, 0xB5, 0x24], b"I420"),
            ]
            .concat(),
        );
        let track = element(
            &[0xAE],
            &[
                uint(&[0xD7], 1),
                uint(&[0x73, 0xC5], 1),
                uint(&[0x83], 1),
                uint(&[0x9C], 0),
                uint(&[0x23, 0xE3, 0x83], 1_000_000_000 / fps as u64),
                element(&[0x86], b"V_UNCOMPRESSED"),
                video,
            ]
            .concat(),
        );
        out.extend(element(&[0x16, 0x54, 0xAE, 0x6B], &track));
        out
    }

    /// One picture in its own cluster, starting at frame `index`.
    pub(super) fn frame(index: usize, fps: usize, yuv: &[u8]) -> Vec<u8> {
        // Track 1, no offset from the cluster's time, a key frame.
        let mut block = vec![0x81, 0x00, 0x00, 0x80];
        block.extend_from_slice(yuv);
        element(
            &[0x1F, 0x43, 0xB6, 0x75],
            &[uint(&[0xE7], millis(index, fps)), element(&[0xA3], &block)].concat(),
        )
    }
}

/// The frame as planar YUV 4:2:0, with BT.601 limited-range values as ffmpeg makes them
/// from RGB by default. Each chroma sample is the mean of a 2x2 block. Video frames have
/// even sizes.
fn yuv420(image: &RgbaImage) -> Vec<u8> {
    let (width, height) = (image.width() as usize, image.height() as usize);
    assert!(
        width % 2 == 0 && height % 2 == 0,
        "video frames have even sizes"
    );
    let mut out = vec![0; width * height * 3 / 2];
    let (luma, chroma) = out.split_at_mut(width * height);
    let (blue, red) = chroma.split_at_mut(width * height / 4);
    let luma_of = |p: &[u8]| {
        let [r, g, b] = [p[0], p[1], p[2]].map(i32::from);
        (((66 * r + 129 * g + 25 * b + 128) >> 8) + 16) as u8
    };
    let rows = image.as_raw().chunks_exact(width * 8);
    let luma_rows = luma.chunks_exact_mut(width * 2);
    let chroma_rows = blue
        .chunks_exact_mut(width / 2)
        .zip(red.chunks_exact_mut(width / 2));
    for ((pair, luma_pair), (blue_row, red_row)) in rows.zip(luma_rows).zip(chroma_rows) {
        let (top, bottom) = pair.split_at(width * 4);
        let (luma_top, luma_bottom) = luma_pair.split_at_mut(width);
        let blocks = top.chunks_exact(8).zip(bottom.chunks_exact(8));
        let luma_blocks = luma_top
            .chunks_exact_mut(2)
            .zip(luma_bottom.chunks_exact_mut(2));
        for (((top, bottom), (luma_top, luma_bottom)), (blue, red)) in blocks
            .zip(luma_blocks)
            .zip(blue_row.iter_mut().zip(red_row.iter_mut()))
        {
            luma_top[0] = luma_of(&top[..4]);
            luma_top[1] = luma_of(&top[4..]);
            luma_bottom[0] = luma_of(&bottom[..4]);
            luma_bottom[1] = luma_of(&bottom[4..]);
            let sum = |channel: usize| {
                [
                    top[channel],
                    top[channel + 4],
                    bottom[channel],
                    bottom[channel + 4],
                ]
                .map(i32::from)
                .iter()
                .sum::<i32>()
            };
            let [r, g, b] = [0, 1, 2].map(|channel| (sum(channel) + 2) / 4);
            *blue = (((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128) as u8;
            *red = (((112 * r - 94 * g - 18 * b + 128) >> 8) + 128) as u8;
        }
    }
    out
}

#[test]
fn yuv420_matches_bt601_limited_range() {
    let mut image = RgbaImage::new(2, 2);
    for pixel in image.pixels_mut() {
        *pixel = Rgba([255, 255, 255, 255]);
    }
    assert_eq!(yuv420(&image), [235, 235, 235, 235, 128, 128]);
    for pixel in image.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 255]);
    }
    assert_eq!(yuv420(&image), [16, 16, 16, 16, 128, 128]);
    for pixel in image.pixels_mut() {
        *pixel = Rgba([255, 0, 0, 255]);
    }
    assert_eq!(yuv420(&image), [82, 82, 82, 82, 90, 240]);
    // Planes are in order, and each chroma sample covers its own 2x2 block.
    let mut image = RgbaImage::new(4, 2);
    for (x, _, pixel) in image.enumerate_pixels_mut() {
        *pixel = if x < 2 {
            Rgba([0, 0, 0, 255])
        } else {
            Rgba([255, 255, 255, 255])
        };
    }
    assert_eq!(
        yuv420(&image),
        [16, 16, 235, 235, 16, 16, 235, 235, 128, 128, 128, 128]
    );
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
