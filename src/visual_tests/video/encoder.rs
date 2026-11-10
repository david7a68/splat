//! Which H.264 encoder ffmpeg uses. A hardware encoder takes less CPU than x264, so the
//! first one in `Encoder::priority` that encodes a short test clip on this computer is
//! used for the whole run. `SPLAT_VISUAL_ENCODER=vaapi`, `nvenc` or `x264` tries only
//! that kind, and x264 when it does not work.
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    process::{Command, Stdio},
    sync::OnceLock,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Encoder {
    /// Intel and AMD GPUs, through the VA-API render node `device`.
    Vaapi { device: PathBuf, low_power: bool },
    /// NVIDIA GPUs.
    Nvenc,
    /// The CPU.
    X264,
}

/// NVIDIA drivers allow a few encode sessions at once on consumer GPUs (eight in recent
/// drivers), across all programs. Up to three suites with four workers each can run at
/// once, so a scenario that finds no free session uses x264.
const NVENC_SESSIONS: usize = 6;

impl Encoder {
    /// The order of preference. On the laptop, a whole video run took 190 CPU-s and 41 s
    /// with VA-API's low-power mode on the Intel GPU, 187 CPU-s and 49 s with NVENC, and
    /// 213 CPU-s and 44 s with x264. VA-API comes first: it is faster, and it does not wake
    /// a discrete GPU. AMD GPUs have no low-power mode, so the normal mode follows.
    fn priority() -> Vec<Self> {
        let mut nodes: Vec<PathBuf> = std::fs::read_dir("/dev/dri")
            .into_iter()
            .flatten()
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with("renderD"))
            })
            .collect();
        nodes.sort();
        let mut list = Vec::new();
        for low_power in [true, false] {
            for device in &nodes {
                list.push(Self::Vaapi {
                    device: device.clone(),
                    low_power,
                });
            }
        }
        list.push(Self::Nvenc);
        list.push(Self::X264);
        list
    }

    fn kind(&self) -> &'static str {
        match self {
            Self::Vaapi { .. } => "vaapi",
            Self::Nvenc => "nvenc",
            Self::X264 => "x264",
        }
    }

    pub(super) fn describe(&self) -> String {
        match self {
            Self::Vaapi { device, low_power } => format!(
                "h264_vaapi on {}{}",
                device.display(),
                if *low_power { ", low power" } else { "" }
            ),
            Self::Nvenc => "h264_nvenc".to_owned(),
            Self::X264 => "libx264".to_owned(),
        }
    }

    /// Options before the input.
    pub(super) fn input_args(&self) -> Vec<String> {
        match self {
            Self::Vaapi { device, .. } => {
                vec!["-vaapi_device".into(), device.display().to_string()]
            }
            Self::Nvenc | Self::X264 => Vec::new(),
        }
    }

    /// Options for the output. The hardware encoders' videos match x264's at CRF 22 within
    /// 45-49 dB PSNR, with the same frames, in about 60% of the size.
    pub(super) fn output_args(&self) -> Vec<String> {
        let args: &[&str] = match self {
            Self::Vaapi {
                low_power: true, ..
            } => &[
                "-vf",
                "format=nv12,hwupload",
                "-c:v",
                "h264_vaapi",
                "-low_power",
                "1",
                "-qp",
                "22",
            ],
            Self::Vaapi { .. } => &[
                "-vf",
                "format=nv12,hwupload",
                "-c:v",
                "h264_vaapi",
                "-qp",
                "22",
            ],
            Self::Nvenc => &[
                "-c:v",
                "h264_nvenc",
                "-preset",
                "p1",
                "-rc",
                "constqp",
                "-qp",
                "22",
                "-pix_fmt",
                "yuv420p",
            ],
            // ultrafast took half the CPU of veryfast at the same quality, for files about
            // twice the size. The animation tune keeps flat UI colours smaller.
            Self::X264 => &[
                "-c:v",
                "libx264",
                "-preset",
                "ultrafast",
                "-tune",
                "animation",
                "-crf",
                "22",
                "-pix_fmt",
                "yuv420p",
                "-threads",
                "2",
            ],
        };
        args.iter().map(|&arg| arg.to_owned()).collect()
    }

    /// Encode a short clip and discard it.
    fn works(&self) -> bool {
        Command::new(super::ffmpeg_path())
            .env_remove("LD_LIBRARY_PATH")
            .args(["-hide_banner", "-loglevel", "error"])
            .args(self.input_args())
            .args([
                "-f",
                "lavfi",
                "-i",
                "testsrc2=size=256x160:rate=12:duration=0.5",
            ])
            .args(self.output_args())
            .args(["-f", "null", "-"])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success())
    }
}

/// The encoder for this run, chosen once.
pub(super) fn chosen() -> &'static Encoder {
    static CHOSEN: OnceLock<Encoder> = OnceLock::new();
    CHOSEN.get_or_init(|| {
        let wanted = std::env::var("SPLAT_VISUAL_ENCODER").ok();
        Encoder::priority()
            .into_iter()
            .filter(|encoder| wanted.as_deref().is_none_or(|kind| kind == encoder.kind()))
            .find(|encoder| *encoder == Encoder::X264 || encoder.works())
            .unwrap_or(Encoder::X264)
    })
}

/// The encoder for one video, and the session lock it holds while ffmpeg runs.
pub(super) fn for_video() -> (Encoder, Option<File>) {
    let encoder = chosen().clone();
    if encoder != Encoder::Nvenc {
        return (encoder, None);
    }
    match nvenc_session() {
        Some(lock) => (encoder, Some(lock)),
        None => (Encoder::X264, None),
    }
}

/// A free NVENC session. The locks are in /dev/shm, which every suite's namespace shares.
fn nvenc_session() -> Option<File> {
    (0..NVENC_SESSIONS).find_map(|index| {
        let lock = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(format!("/dev/shm/splat-visual-nvenc-{index}.lock"))
            .ok()?;
        lock.try_lock().ok().map(|()| lock)
    })
}

#[test]
fn hardware_encoders_come_before_x264() {
    let list = Encoder::priority();
    assert_eq!(list.last(), Some(&Encoder::X264));
    let nvenc = list.iter().position(|e| *e == Encoder::Nvenc).unwrap();
    assert!(
        list[..nvenc]
            .iter()
            .all(|e| matches!(e, Encoder::Vaapi { .. }))
    );
    // Low power first on every device, then the normal mode.
    let first_normal = list
        .iter()
        .position(|e| {
            matches!(
                e,
                Encoder::Vaapi {
                    low_power: false,
                    ..
                }
            )
        })
        .unwrap_or(nvenc);
    assert!(list[..first_normal].iter().all(|e| matches!(
        e,
        Encoder::Vaapi {
            low_power: true,
            ..
        }
    )));
}
