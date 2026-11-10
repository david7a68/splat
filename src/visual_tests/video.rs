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
    last: Option<Last>,
}

struct Last {
    rgba: Vec<u8>,
    yuv: Vec<u8>,
    length: usize,
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
        let start = self.shown;
        self.shown += count;
        // The same picture again only lengthens the last one.
        if let Some(last) = &mut self.last
            && last.rgba == *frame.as_raw()
        {
            last.length += count;
            return;
        }
        // A moving pointer or a new caption changes a few rows. Convert only the rows that
        // changed since the last picture, which took a fifth of a video run's CPU before.
        let yuv = match self.last.take() {
            Some(mut last) => {
                if !yuv420_changed(&mut last.yuv, &last.rgba, &frame) {
                    // Changes the video cannot show, such as alpha, still make the same picture.
                    last.rgba = frame.into_raw();
                    last.length += count;
                    self.last = Some(last);
                    return;
                }
                last.yuv
            }
            None => yuv420(&frame),
        };
        if let Some(stdin) = &mut self.stdin {
            // A failure here shows at `finish`, with ffmpeg's own exit status.
            let _ = mkv::write_frame(stdin, start, FPS, &yuv);
        }
        self.last = Some(Last {
            rgba: frame.into_raw(),
            yuv,
            length: count,
        });
    }

    pub fn finish(mut self) -> Result<String, String> {
        // A frame's time ends where the next one starts. Repeat the last picture on the last
        // frame, so the final hold keeps its length.
        if let (Some(last), Some(stdin)) = (&self.last, &mut self.stdin)
            && last.length > 1
        {
            let _ = mkv::write_frame(stdin, self.shown - 1, FPS, &last.yuv);
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

    /// One picture in its own cluster, starting at frame `index`. The picture goes to the
    /// writer as it is, without a copy.
    pub(super) fn write_frame(
        out: &mut impl std::io::Write,
        index: usize,
        fps: usize,
        yuv: &[u8],
    ) -> std::io::Result<()> {
        let time = uint(&[0xE7], millis(index, fps));
        // Track 1, no offset from the cluster's time, a key frame.
        let block = [0x81, 0x00, 0x00, 0x80];
        let block_size = block.len() + yuv.len();
        let mut head = vec![0x1F, 0x43, 0xB6, 0x75, 0x01];
        let cluster_size = time.len() + 1 + 8 + block_size;
        head.extend_from_slice(&(cluster_size as u64).to_be_bytes()[1..]);
        head.extend_from_slice(&time);
        head.extend_from_slice(&[0xA3, 0x01]);
        head.extend_from_slice(&(block_size as u64).to_be_bytes()[1..]);
        head.extend_from_slice(&block);
        out.write_all(&head)?;
        out.write_all(yuv)
    }

    #[test]
    fn a_frame_is_a_cluster_with_its_time_and_one_simple_block() {
        let yuv = [1, 2, 3, 4, 5, 6];
        let mut written = Vec::new();
        write_frame(&mut written, 3, 12, &yuv).unwrap();
        let block = [&[0x81, 0x00, 0x00, 0x80][..], &yuv].concat();
        let expected = element(
            &[0x1F, 0x43, 0xB6, 0x75],
            &[uint(&[0xE7], 250), element(&[0xA3], &block)].concat(),
        );
        assert_eq!(written, expected);
    }
}

/// The frame as planar YUV 4:2:0, with BT.601 limited-range values as ffmpeg makes them
/// from RGB by default. Each chroma sample is the mean of a 2x2 block. Video frames have
/// even sizes.
fn yuv420(image: &RgbaImage) -> Vec<u8> {
    let (width, height) = (image.width() as usize, image.height() as usize);
    let mut out = vec![0; width * height * 3 / 2];
    convert(&mut out, width, height, image.as_raw(), None);
    out
}

/// Bring `yuv`, the conversion of `old`, up to date with `image`. Only the row pairs that
/// differ are converted again. False when the conversion is the same as before.
fn yuv420_changed(yuv: &mut [u8], old: &[u8], image: &RgbaImage) -> bool {
    let (width, height) = (image.width() as usize, image.height() as usize);
    convert(yuv, width, height, image.as_raw(), Some(old))
}

/// Convert `rgba` into `out`, skipping row pairs that equal `old`. True if `out` changed.
fn convert(out: &mut [u8], width: usize, height: usize, rgba: &[u8], old: Option<&[u8]>) -> bool {
    assert!(
        width.is_multiple_of(2) && height.is_multiple_of(2),
        "video frames have even sizes"
    );
    let (luma, chroma) = out.split_at_mut(width * height);
    let (blue, red) = chroma.split_at_mut(width * height / 4);
    let pairs = rgba
        .chunks_exact(width * 8)
        .zip(luma.chunks_exact_mut(width * 2))
        .zip(
            blue.chunks_exact_mut(width / 2)
                .zip(red.chunks_exact_mut(width / 2)),
        );
    // Until a row pair converts to new values, keep each pair's old values to compare.
    let mut changed = old.is_none();
    let mut before = Vec::with_capacity(width * 3);
    for (index, ((pair, luma), (blue, red))) in pairs.enumerate() {
        let range = index * width * 8..(index + 1) * width * 8;
        if old.is_some_and(|old| old[range] == *pair) {
            continue;
        }
        if !changed {
            before.clear();
            before.extend_from_slice(luma);
            before.extend_from_slice(blue);
            before.extend_from_slice(red);
        }
        let (top, bottom) = pair.split_at(width * 4);
        let (luma_top, luma_bottom) = luma.split_at_mut(width);
        convert_row_pair(top, bottom, luma_top, luma_bottom, blue, red);
        if !changed {
            let (old_luma, old_chroma) = before.split_at(luma.len());
            let (old_blue, old_red) = old_chroma.split_at(blue.len());
            changed = old_luma != luma || old_blue != blue || old_red != red;
        }
    }
    changed
}

#[inline(always)]
fn luma_of(r: i32, g: i32, b: i32) -> u8 {
    (((66 * r + 129 * g + 25 * b + 128) >> 8) + 16) as u8
}

fn convert_row_pair(
    top: &[u8],
    bottom: &[u8],
    luma_top: &mut [u8],
    luma_bottom: &mut [u8],
    blue: &mut [u8],
    red: &mut [u8],
) {
    let blocks = top.chunks_exact(8).zip(bottom.chunks_exact(8));
    let luma = luma_top
        .chunks_exact_mut(2)
        .zip(luma_bottom.chunks_exact_mut(2));
    for (((top, bottom), (luma_top, luma_bottom)), (blue, red)) in
        blocks.zip(luma).zip(blue.iter_mut().zip(red.iter_mut()))
    {
        let [r0, g0, b0, _, r1, g1, b1, _] = <[u8; 8]>::try_from(top).unwrap().map(i32::from);
        let [r2, g2, b2, _, r3, g3, b3, _] = <[u8; 8]>::try_from(bottom).unwrap().map(i32::from);
        luma_top[0] = luma_of(r0, g0, b0);
        luma_top[1] = luma_of(r1, g1, b1);
        luma_bottom[0] = luma_of(r2, g2, b2);
        luma_bottom[1] = luma_of(r3, g3, b3);
        let r = (r0 + r1 + r2 + r3 + 2) >> 2;
        let g = (g0 + g1 + g2 + g3 + 2) >> 2;
        let b = (b0 + b1 + b2 + b3 + 2) >> 2;
        *blue = (((-38 * r - 74 * g + 112 * b + 128) >> 8) + 128) as u8;
        *red = (((112 * r - 94 * g - 18 * b + 128) >> 8) + 128) as u8;
    }
}

#[test]
fn a_changed_frame_converts_like_a_new_one() {
    let mut old = RgbaImage::from_pixel(8, 6, Rgba([10, 200, 30, 255]));
    old.put_pixel(7, 5, Rgba([0, 0, 0, 255]));
    let mut new = old.clone();
    new.put_pixel(2, 2, Rgba([250, 10, 10, 255]));
    new.put_pixel(7, 5, Rgba([255, 255, 255, 255]));
    let mut yuv = yuv420(&old);
    assert!(yuv420_changed(&mut yuv, old.as_raw(), &new));
    assert_eq!(yuv, yuv420(&new));
    // Alpha alone does not change the video.
    let mut faded = new.clone();
    faded.put_pixel(0, 0, Rgba([10, 200, 30, 0]));
    assert!(!yuv420_changed(&mut yuv, new.as_raw(), &faded));
    assert_eq!(yuv, yuv420(&new));
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
