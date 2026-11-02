//! Audio capture for a session. The host's own recorder does the capture, as a child
//! process: `pw-record`, then `parecord`, then `arecord`. It writes 16 kHz mono PCM, which
//! is small enough for a long session and what speech services expect. The app reads the
//! growing file to confirm that sound arrives, and the child stops if the app dies.
//!
//! Test builds never start a process or touch a microphone (see `Recorder::start`).
use std::{
    fs,
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::{Duration, Instant},
};

pub const SAMPLE_RATE: u32 = 16_000;
const BYTES_PER_SECOND: u64 = SAMPLE_RATE as u64 * 2;
/// A 16-bit peak below this is quiet room noise or a muted input.
const SILENCE_PEAK: i32 = 180;
const CHECK_EVERY: Duration = Duration::from_secs(1);
/// A recorder buffers, so a file can pause briefly without anything being wrong.
const STALLED_AFTER: Duration = Duration::from_secs(10);
const STOP_WAIT: Duration = Duration::from_secs(5);
/// How far back from a part's limit to look for a quiet place to cut.
const CUT_SEARCH_SECONDS: u64 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    PwRecord,
    Parecord,
    Arecord,
    /// Test builds only: a fixed file, and no process.
    #[cfg(test)]
    Simulated,
}

impl Tool {
    pub fn name(self) -> &'static str {
        match self {
            Self::PwRecord => "pw-record",
            Self::Parecord => "parecord",
            Self::Arecord => "arecord",
            #[cfg(test)]
            Self::Simulated => "test recorder",
        }
    }

    /// Only `arecord` cannot use a source name from `pactl`.
    pub fn can_choose_input(self) -> bool {
        self != Self::Arecord
    }

    fn command(self, input: Option<&str>, path: &Path) -> Command {
        let mut command = match self {
            Self::PwRecord => {
                let mut command = Command::new("pw-record");
                command.args(["--rate", "16000", "--channels", "1", "--format", "s16"]);
                if let Some(input) = input {
                    // Without this, a missing input falls back to another microphone.
                    command.arg("--target").arg(input);
                    command.args(["--properties", "{ node.dont-fallback = true }"]);
                }
                command
            }
            Self::Parecord => {
                let mut command = Command::new("parecord");
                command.args([
                    "--rate=16000",
                    "--channels=1",
                    "--format=s16le",
                    "--file-format=wav",
                ]);
                if let Some(input) = input {
                    command.arg(format!("--device={input}"));
                }
                command
            }
            Self::Arecord => {
                let mut command = Command::new("arecord");
                command.args(["-q", "-f", "S16_LE", "-r", "16000", "-c", "1", "-t", "wav"]);
                command
            }
            #[cfg(test)]
            Self::Simulated => unreachable!("the simulated recorder starts no process"),
        };
        command.arg(path);
        command
    }
}

#[cfg(not(test))]
fn host_command(program: &str) -> Command {
    let mut command = Command::new(program);
    // The development shell points this at Nix libraries, which host tools must not load.
    command.env_remove("LD_LIBRARY_PATH");
    command
}

pub fn on_path(program: &str) -> bool {
    std::env::var_os("PATH").is_some_and(|paths| {
        std::env::split_paths(&paths).any(|folder| folder.join(program).is_file())
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    pub name: String,
    pub description: String,
}

/// `pactl list sources`, without anything that only captures what the computer plays.
fn parse_sources(text: &str) -> Vec<Input> {
    let mut inputs: Vec<Input> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if let Some(name) = line.strip_prefix("Name: ") {
            inputs.push(Input {
                name: name.to_owned(),
                description: name.to_owned(),
            });
        } else if let Some(description) = line.strip_prefix("Description: ")
            && let Some(input) = inputs.last_mut()
        {
            description.clone_into(&mut input.description);
        }
    }
    inputs.retain(|input| !input.name.ends_with(".monitor"));
    inputs
}

/// What this computer can record with. It can block, so it runs on its own thread.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Probe {
    pub tool: Option<Tool>,
    pub inputs: Vec<Input>,
}

#[cfg(not(test))]
pub fn probe() -> mpsc::Receiver<Probe> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let tool = [
            (Tool::PwRecord, "pw-record"),
            (Tool::Parecord, "parecord"),
            (Tool::Arecord, "arecord"),
        ]
        .into_iter()
        .find_map(|(tool, program)| on_path(program).then_some(tool));
        let inputs = host_command("pactl")
            .env("LC_ALL", "C")
            .args(["list", "sources"])
            .stderr(Stdio::null())
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| parse_sources(&String::from_utf8_lossy(&output.stdout)))
            .unwrap_or_default();
        let _ = tx.send(Probe { tool, inputs });
    });
    rx
}

#[cfg(test)]
pub fn probe() -> mpsc::Receiver<Probe> {
    let (tx, rx) = mpsc::channel();
    let _ = tx.send(Probe {
        tool: Some(Tool::Simulated),
        inputs: vec![Input {
            name: "table.mic".into(),
            description: "Table microphone".into(),
        }],
    });
    rx
}

/// How the recording looks right now. Each state has words where the DM looks.
#[derive(Clone, Debug, PartialEq)]
pub enum Health {
    /// Just started. Nothing says yet whether sound arrives.
    Starting,
    /// Sound in the last second.
    Hearing,
    /// The file grows, but it holds silence. The input can be muted.
    Silent,
    /// The file has not grown for some seconds.
    Stalled,
    /// The recorder stopped by itself, with its own words if it gave any.
    Stopped(String),
}

enum Capture {
    Child {
        child: Child,
        errors: fs::File,
    },
    #[cfg(test)]
    Simulated,
}

pub struct Recorder {
    capture: Capture,
    path: PathBuf,
    started: u64,
    health: Health,
    size: u64,
    grew: Instant,
    checked: Instant,
}

/// Ask the child to finish its file if this process dies. It must not go on listening.
fn stop_with_parent(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    // SAFETY: prctl is async-signal-safe, and nothing else runs between fork and exec.
    unsafe {
        command.pre_exec(|| {
            if libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGINT) == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        });
    }
}

impl Recorder {
    /// Start at `path`, which must not exist yet.
    pub fn start(tool: Tool, input: Option<&str>, path: PathBuf) -> Result<Self, String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("Could not create the folder: {e}"))?;
        }
        #[cfg(test)]
        if tool == Tool::Simulated {
            fs::write(&path, wav_bytes(&vec![1000_i16; SAMPLE_RATE as usize]))
                .map_err(|e| e.to_string())?;
            return Ok(Self::with(Capture::Simulated, path));
        }
        let command = tool.command(input, &path);
        Self::spawn(command, path)
            .map_err(|error| format!("{} did not start. {error}", tool.name()))
    }

    fn spawn(mut command: Command, path: PathBuf) -> Result<Self, String> {
        command.env_remove("LD_LIBRARY_PATH");
        stop_with_parent(&mut command);
        let errors = tempfile::tempfile().map_err(|e| e.to_string())?;
        let child = command
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(errors.try_clone().map_err(|e| e.to_string())?)
            .spawn()
            .map_err(|e| e.to_string())?;
        Ok(Self::with(Capture::Child { child, errors }, path))
    }

    fn with(capture: Capture, path: PathBuf) -> Self {
        let now = Instant::now();
        Self {
            capture,
            path,
            started: crate::hermetic::now(),
            health: Health::Starting,
            size: 0,
            grew: now,
            checked: now,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn seconds(&self) -> u64 {
        crate::hermetic::now().saturating_sub(self.started)
    }

    pub fn health(&self) -> &Health {
        &self.health
    }

    #[cfg(test)]
    pub fn simulate_health(&mut self, health: Health) {
        self.health = health;
    }

    /// Look at the child and the file, about once a second.
    #[cfg_attr(not(test), allow(irrefutable_let_patterns))]
    pub fn poll(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.checked) < CHECK_EVERY {
            return;
        }
        self.checked = now;
        let Capture::Child { child, errors } = &mut self.capture else {
            return;
        };
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut words = String::new();
                let _ = errors.seek(SeekFrom::Start(0));
                let _ = errors.read_to_string(&mut words);
                let words = words.lines().last().unwrap_or_default().trim().to_owned();
                self.health = Health::Stopped(if words.is_empty() {
                    format!("The recorder ended: {status}.")
                } else {
                    words
                });
                return;
            }
            Ok(None) => {}
            Err(error) => {
                self.health = Health::Stopped(error.to_string());
                return;
            }
        }
        let size = fs::metadata(&self.path).map_or(0, |metadata| metadata.len());
        if size > self.size {
            self.size = size;
            self.grew = now;
            self.health = match last_second_peak(&self.path, size) {
                Some(peak) if peak >= SILENCE_PEAK => Health::Hearing,
                Some(_) => Health::Silent,
                None => Health::Starting,
            };
        } else if now.duration_since(self.grew) >= STALLED_AFTER {
            self.health = Health::Stalled;
        }
    }

    /// Finish the file. What was captured stays, whatever the recorder did at the end.
    #[cfg_attr(not(test), allow(irrefutable_let_patterns))]
    pub fn stop(mut self) -> PathBuf {
        if let Capture::Child { child, .. } = &mut self.capture {
            // SAFETY: `kill` takes a PID and a signal. The child has not been reaped.
            let pid = child.id() as i32;
            if matches!(child.try_wait(), Ok(None)) {
                unsafe {
                    libc::kill(pid, libc::SIGINT);
                }
            }
            let waiting = Instant::now();
            while matches!(child.try_wait(), Ok(None)) && waiting.elapsed() < STOP_WAIT {
                std::thread::sleep(Duration::from_millis(50));
            }
            let _ = child.kill();
            let _ = child.wait();
        }
        // A recorder that died early leaves lengths of zero in its header.
        let _ = repair_header(&self.path);
        self.path.clone()
    }
}

impl Drop for Recorder {
    #[cfg_attr(not(test), allow(irrefutable_let_patterns))]
    fn drop(&mut self) {
        if let Capture::Child { child, .. } = &mut self.capture
            && matches!(child.try_wait(), Ok(None))
        {
            // SAFETY: as in `stop`. This is only the path where `stop` did not run.
            unsafe {
                libc::kill(child.id() as i32, libc::SIGINT);
            }
            let waiting = Instant::now();
            while matches!(child.try_wait(), Ok(None)) && waiting.elapsed() < STOP_WAIT {
                std::thread::sleep(Duration::from_millis(50));
            }
            let _ = child.kill();
            let _ = child.wait();
            let _ = repair_header(&self.path);
        }
    }
}

fn last_second_peak(path: &Path, size: u64) -> Option<i32> {
    let mut file = fs::File::open(path).ok()?;
    let length = BYTES_PER_SECOND.min(size.saturating_sub(WAV_HEADER as u64)) & !1;
    if length == 0 {
        return None;
    }
    let start = (size - length).max(WAV_HEADER as u64);
    // Stay on a sample boundary, counted from the end of a plain 44-byte header.
    let start = start + (start - WAV_HEADER as u64) % 2;
    file.seek(SeekFrom::Start(start)).ok()?;
    let mut bytes = Vec::with_capacity(length as usize);
    file.take(length).read_to_end(&mut bytes).ok()?;
    bytes
        .chunks_exact(2)
        .map(|sample| i32::from(i16::from_le_bytes([sample[0], sample[1]])).abs())
        .max()
}

const WAV_HEADER: usize = 44;

fn wav_header(data_length: u32, rate: u32, channels: u16, bits: u16) -> [u8; WAV_HEADER] {
    let align = channels * (bits / 8);
    let mut header = [0; WAV_HEADER];
    header[0..4].copy_from_slice(b"RIFF");
    header[4..8].copy_from_slice(&(data_length + 36).to_le_bytes());
    header[8..16].copy_from_slice(b"WAVEfmt ");
    header[16..20].copy_from_slice(&16_u32.to_le_bytes());
    header[20..22].copy_from_slice(&1_u16.to_le_bytes());
    header[22..24].copy_from_slice(&channels.to_le_bytes());
    header[24..28].copy_from_slice(&rate.to_le_bytes());
    header[28..32].copy_from_slice(&(rate * u32::from(align)).to_le_bytes());
    header[32..34].copy_from_slice(&align.to_le_bytes());
    header[34..36].copy_from_slice(&bits.to_le_bytes());
    header[36..40].copy_from_slice(b"data");
    header[40..44].copy_from_slice(&data_length.to_le_bytes());
    header
}

#[cfg(test)]
pub fn wav_bytes(samples: &[i16]) -> Vec<u8> {
    let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    let mut bytes = wav_header(data.len() as u32, SAMPLE_RATE, 1, 16).to_vec();
    bytes.extend(data);
    bytes
}

/// Where a WAV file's sound is, whatever else its recorder put in the file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layout {
    pub data_start: u64,
    pub data_length: u64,
    pub rate: u32,
    pub channels: u16,
    pub bits: u16,
    /// The header gave the length as zero or past the file's end.
    header_wrong: bool,
    data_length_offset: u64,
}

impl Layout {
    pub fn read(path: &Path) -> Result<Self, String> {
        let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
        let size = file.metadata().map_err(|e| e.to_string())?.len();
        let mut start = [0; 12];
        file.read_exact(&mut start)
            .map_err(|_| "The recording is too short to read.".to_owned())?;
        if &start[0..4] != b"RIFF" || &start[8..12] != b"WAVE" {
            return Err("The recording is not a WAV file.".into());
        }
        let mut format = None;
        let mut position = 12;
        while position + 8 <= size {
            file.seek(SeekFrom::Start(position))
                .map_err(|e| e.to_string())?;
            let mut head = [0; 8];
            file.read_exact(&mut head).map_err(|e| e.to_string())?;
            let length = u64::from(u32::from_le_bytes([head[4], head[5], head[6], head[7]]));
            match &head[0..4] {
                b"fmt " => {
                    let mut body = [0; 16];
                    file.read_exact(&mut body).map_err(|e| e.to_string())?;
                    format = Some((
                        u16::from_le_bytes([body[0], body[1]]),
                        u16::from_le_bytes([body[2], body[3]]),
                        u32::from_le_bytes([body[4], body[5], body[6], body[7]]),
                        u16::from_le_bytes([body[14], body[15]]),
                    ));
                }
                b"data" => {
                    let (code, channels, rate, bits) =
                        format.ok_or("The recording has no format before its sound.")?;
                    // 0xFFFE is the extensible form of the same PCM.
                    if !matches!(code, 1 | 0xFFFE) || bits == 0 || channels == 0 || rate == 0 {
                        return Err("The recording is not plain PCM audio.".into());
                    }
                    let data_start = position + 8;
                    let rest = size - data_start;
                    let header_wrong = length == 0 || length > rest;
                    return Ok(Self {
                        data_start,
                        data_length: if header_wrong { rest } else { length },
                        rate,
                        channels,
                        bits,
                        header_wrong,
                        data_length_offset: position + 4,
                    });
                }
                _ => {}
            }
            position += 8 + length + (length & 1);
        }
        Err("The recording has no sound in it.".into())
    }

    pub fn byte_rate(&self) -> u64 {
        u64::from(self.rate) * u64::from(self.channels) * u64::from(self.bits / 8)
    }

    pub fn seconds(&self) -> u64 {
        self.data_length / self.byte_rate().max(1)
    }

    fn block(&self) -> u64 {
        u64::from(self.channels) * u64::from(self.bits / 8)
    }

    /// Divide the sound into parts, each at most `seconds` long and `max_bytes` as a file.
    /// A cut in a sentence loses words, so each cut goes to the quietest moment shortly
    /// before the limit. The same file always gives the same parts, so a transcript resumes.
    pub fn parts(&self, path: &Path, seconds: u64, max_bytes: u64) -> Result<Vec<Part>, String> {
        let block = self.block().max(1);
        let limit = (self.byte_rate() * seconds).min(max_bytes.saturating_sub(WAV_HEADER as u64));
        let limit = (limit - limit % block).max(block);
        let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
        let mut parts = Vec::new();
        let mut start = 0;
        while start < self.data_length {
            let remaining = self.data_length - start;
            let length = if remaining <= limit {
                remaining
            } else {
                limit - self.quiet_offset(&mut file, start, limit)?
            };
            parts.push(Part { start, length });
            start += length;
        }
        Ok(parts)
    }

    /// How far before the limit to cut, in bytes.
    fn quiet_offset(&self, file: &mut fs::File, start: u64, limit: u64) -> Result<u64, String> {
        if self.bits != 16 {
            return Ok(0);
        }
        let block = self.block();
        let reach = (self.byte_rate() * CUT_SEARCH_SECONDS).min(limit / 2);
        let reach = reach - reach % block;
        let frame = (self.byte_rate() / 10).max(block);
        let frame = frame - frame % block;
        if reach < frame {
            return Ok(0);
        }
        file.seek(SeekFrom::Start(self.data_start + start + limit - reach))
            .map_err(|e| e.to_string())?;
        let mut bytes = vec![0; reach as usize];
        file.read_exact(&mut bytes).map_err(|e| e.to_string())?;
        // The latest frame wins a tie, so that silence costs no length.
        let quietest = bytes
            .chunks_exact(frame as usize)
            .enumerate()
            .min_by_key(|(index, samples)| {
                let loudness: u64 = samples
                    .chunks_exact(2)
                    .map(|s| u64::from(i16::from_le_bytes([s[0], s[1]]).unsigned_abs()))
                    .sum();
                (loudness, usize::MAX - index)
            })
            .map_or(0, |(index, _)| index as u64);
        // Cut in the middle of the quiet frame.
        Ok(reach - (quietest * frame + frame / 2 - (frame / 2) % block))
    }

    /// One part, as a complete WAV file of its own.
    pub fn read_part(&self, path: &Path, part: Part) -> Result<Vec<u8>, String> {
        let mut file = fs::File::open(path).map_err(|e| e.to_string())?;
        file.seek(SeekFrom::Start(self.data_start + part.start))
            .map_err(|e| e.to_string())?;
        let mut bytes =
            wav_header(part.length as u32, self.rate, self.channels, self.bits).to_vec();
        file.take(part.length)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        Ok(bytes)
    }

    /// When a part starts in the recording.
    pub fn seconds_at(&self, part: Part) -> u64 {
        let rate = self.byte_rate().max(1);
        (part.start + rate / 2) / rate
    }
}

/// A stretch of a recording's sound, in bytes from the start of the sound.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Part {
    pub start: u64,
    pub length: u64,
}

/// Write the true lengths into a header that a recorder left wrong.
fn repair_header(path: &Path) -> Result<(), String> {
    use std::io::Write;
    let layout = Layout::read(path)?;
    if !layout.header_wrong {
        return Ok(());
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    let data = u32::try_from(layout.data_length).unwrap_or(u32::MAX);
    let riff = u32::try_from(layout.data_start - 8 + layout.data_length).unwrap_or(u32::MAX);
    file.seek(SeekFrom::Start(4)).map_err(|e| e.to_string())?;
    file.write_all(&riff.to_le_bytes())
        .map_err(|e| e.to_string())?;
    file.seek(SeekFrom::Start(layout.data_length_offset))
        .map_err(|e| e.to_string())?;
    file.write_all(&data.to_le_bytes())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, name: &str, bytes: &[u8]) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn sources_leave_out_what_only_hears_the_computer() {
        let text = "Source #54\n\tName: alsa_output.analog-stereo.monitor\n\tDescription: Monitor of Speakers\n\
                    Source #55\n\tState: IDLE\n\tName: alsa_input.analog-stereo\n\tDescription: Built-in Audio\n\
                    Source #60\n\tName: usb_mic\n";
        assert_eq!(
            parse_sources(text),
            [
                Input {
                    name: "alsa_input.analog-stereo".into(),
                    description: "Built-in Audio".into()
                },
                Input {
                    name: "usb_mic".into(),
                    description: "usb_mic".into()
                },
            ]
        );
    }

    #[test]
    fn each_recorder_is_asked_for_the_same_audio() {
        let args = |tool: Tool, input| {
            tool.command(input, Path::new("out.wav"))
                .get_args()
                .map(|arg| arg.to_string_lossy().into_owned())
                .collect::<Vec<_>>()
                .join(" ")
        };
        assert_eq!(
            args(Tool::PwRecord, Some("usb")),
            "--rate 16000 --channels 1 --format s16 --target usb --properties { node.dont-fallback = true } out.wav"
        );
        assert_eq!(
            args(Tool::Parecord, Some("usb")),
            "--rate=16000 --channels=1 --format=s16le --file-format=wav --device=usb out.wav"
        );
        assert_eq!(
            args(Tool::Arecord, None),
            "-q -f S16_LE -r 16000 -c 1 -t wav out.wav"
        );
    }

    #[test]
    fn layout_finds_the_sound_after_other_chunks() {
        let dir = tempfile::tempdir().unwrap();
        let plain = wav_bytes(&[1, 2, 3, 4]);
        // A recorder can put a chunk of odd length before the data. It is padded.
        let mut extra = plain[..36].to_vec();
        extra.extend(b"LIST");
        extra.extend(3_u32.to_le_bytes());
        extra.extend(b"abc\0");
        extra.extend(&plain[36..]);
        let path = write(dir.path(), "extra.wav", &extra);
        let layout = Layout::read(&path).unwrap();
        assert_eq!(layout.data_start, 56);
        assert_eq!(layout.data_length, 8);
        assert_eq!((layout.rate, layout.channels, layout.bits), (16_000, 1, 16));
    }

    #[test]
    fn a_header_left_wrong_by_a_crash_is_repaired() {
        let dir = tempfile::tempdir().unwrap();
        let mut bytes = wav_bytes(&[5; 100]);
        bytes[40..44].copy_from_slice(&0_u32.to_le_bytes());
        bytes[4..8].copy_from_slice(&0_u32.to_le_bytes());
        let path = write(dir.path(), "crash.wav", &bytes);
        assert_eq!(Layout::read(&path).unwrap().data_length, 200);
        repair_header(&path).unwrap();
        let fixed = fs::read(&path).unwrap();
        assert_eq!(fixed, wav_bytes(&[5; 100]));
        assert!(!Layout::read(&path).unwrap().header_wrong);
    }

    #[test]
    fn what_is_not_a_recording_is_refused_with_words() {
        let dir = tempfile::tempdir().unwrap();
        for (bytes, words) in [
            (&b"RIFF"[..], "too short"),
            (&b"not a wave file at all"[..], "not a WAV"),
            (&b"RIFF\0\0\0\0WAVEjunk\0\0\0\0"[..], "no sound"),
        ] {
            let path = write(dir.path(), "bad.wav", bytes);
            let error = Layout::read(&path).unwrap_err();
            assert!(error.contains(words), "{error}");
        }
    }

    #[test]
    fn parts_cover_every_sample_once_each_as_its_own_file() {
        let dir = tempfile::tempdir().unwrap();
        let samples: Vec<i16> = (0..70_000).map(|n| (n % 1000 + 500) as i16).collect();
        let path = write(dir.path(), "long.wav", &wav_bytes(&samples));
        let layout = Layout::read(&path).unwrap();
        let parts = layout.parts(&path, 2, 1_000_000).unwrap();
        assert!(parts.len() >= 3);
        let mut joined = Vec::new();
        for part in &parts {
            assert!(part.length <= 2 * BYTES_PER_SECOND);
            let file = write(
                dir.path(),
                "part.wav",
                &layout.read_part(&path, *part).unwrap(),
            );
            let inner = Layout::read(&file).unwrap();
            assert_eq!(inner.data_length, part.length);
            joined.extend(fs::read(&file).unwrap()[44..].to_vec());
        }
        assert_eq!(joined, fs::read(&path).unwrap()[44..]);
    }

    #[test]
    fn a_cut_goes_to_the_quiet_moment_and_repeats_exactly() {
        let dir = tempfile::tempdir().unwrap();
        // Speech everywhere, but for a pause near 50 seconds, inside the search before 60.
        let mut samples = vec![4000_i16; 16_000 * 90];
        samples[16_000 * 50..16_000 * 51].fill(0);
        let path = write(dir.path(), "talk.wav", &wav_bytes(&samples));
        let layout = Layout::read(&path).unwrap();
        let parts = layout.parts(&path, 60, 24_000_000).unwrap();
        assert_eq!(parts.len(), 2);
        let cut = parts[0].length / 2;
        assert!(
            (16_000 * 50..16_000 * 51).contains(&cut),
            "cut at sample {cut}"
        );
        assert_eq!(parts[0].length % 2, 0);
        assert_eq!(layout.seconds_at(parts[1]), 51);
        assert_eq!(layout.parts(&path, 60, 24_000_000).unwrap(), parts);
    }

    #[test]
    fn silence_costs_a_part_no_length() {
        let dir = tempfile::tempdir().unwrap();
        let path = write(dir.path(), "quiet.wav", &wav_bytes(&vec![0; 16_000 * 90]));
        let layout = Layout::read(&path).unwrap();
        let parts = layout.parts(&path, 60, 24_000_000).unwrap();
        // Within one 100 ms frame of the limit.
        assert!(parts[0].length >= 60 * BYTES_PER_SECOND - BYTES_PER_SECOND / 10);
    }

    #[test]
    fn a_size_limit_also_bounds_a_part() {
        let dir = tempfile::tempdir().unwrap();
        let path = write(dir.path(), "a.wav", &wav_bytes(&[1; 1000]));
        let layout = Layout::read(&path).unwrap();
        // 145 bytes less the 44-byte header is 101, so 100 bytes of samples fit.
        let parts = layout.parts(&path, 600, 145).unwrap();
        assert_eq!(parts.len(), 20);
        assert_eq!(layout.read_part(&path, parts[0]).unwrap().len(), 144);
    }

    #[test]
    fn the_peak_tells_silence_from_sound() {
        let dir = tempfile::tempdir().unwrap();
        let quiet = write(dir.path(), "q.wav", &wav_bytes(&[3; 40_000]));
        let size = fs::metadata(&quiet).unwrap().len();
        assert_eq!(last_second_peak(&quiet, size), Some(3));
        let mut samples = vec![3; 40_000];
        samples[39_000] = -9000;
        let loud = write(dir.path(), "l.wav", &wav_bytes(&samples));
        assert_eq!(last_second_peak(&loud, size), Some(9000));
    }

    /// A real child process, with no microphone: a script stands in for the recorder.
    fn script(dir: &Path, body: &str) -> Command {
        let path = dir.join("recorder.sh");
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).unwrap();
        let mut command = Command::new("sh");
        command.arg(path);
        command
    }

    #[test]
    fn stop_lets_the_recorder_finish_its_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.wav");
        let header = wav_bytes(&[7; 4]);
        fs::write(dir.path().join("final.wav"), &header).unwrap();
        let body = format!(
            "cd '{}'\ntrap 'cp final.wav out.wav; exit 0' INT\ntouch out.wav\nwhile :; do sleep 0.05; done",
            dir.path().display()
        );
        let recorder = Recorder::spawn(script(dir.path(), &body), path.clone()).unwrap();
        std::thread::sleep(Duration::from_millis(200));
        assert_eq!(recorder.stop(), path);
        assert_eq!(fs::read(&path).unwrap(), header);
    }

    #[test]
    fn a_recorder_that_ends_by_itself_is_reported_with_its_own_words() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("out.wav");
        let mut recorder = Recorder::spawn(
            script(dir.path(), "echo 'no such device' >&2\nexit 3"),
            path,
        )
        .unwrap();
        let waiting = Instant::now();
        while !matches!(recorder.health(), Health::Stopped(_)) {
            assert!(waiting.elapsed() < Duration::from_secs(5));
            recorder.checked = Instant::now() - CHECK_EVERY;
            recorder.poll();
            std::thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(
            recorder.health(),
            &Health::Stopped("no such device".to_owned())
        );
    }

    #[test]
    fn a_file_that_stops_growing_is_stalled() {
        let dir = tempfile::tempdir().unwrap();
        let path = write(dir.path(), "out.wav", &wav_bytes(&[3; 100]));
        let mut recorder = Recorder::spawn(script(dir.path(), "exec sleep 30"), path).unwrap();
        recorder.checked = Instant::now() - CHECK_EVERY;
        recorder.poll();
        assert_eq!(recorder.health(), &Health::Silent);
        recorder.checked = Instant::now() - CHECK_EVERY;
        recorder.grew = Instant::now() - STALLED_AFTER;
        recorder.poll();
        assert_eq!(recorder.health(), &Health::Stalled);
    }

    #[test]
    fn dropping_a_recorder_stops_the_process() {
        let dir = tempfile::tempdir().unwrap();
        let marker = dir.path().join("stopped");
        let body = format!(
            "trap 'touch \"{}\"; exit 0' INT\nwhile :; do sleep 0.05; done",
            marker.display()
        );
        let recorder =
            Recorder::spawn(script(dir.path(), &body), dir.path().join("out.wav")).unwrap();
        std::thread::sleep(Duration::from_millis(200));
        drop(recorder);
        assert!(marker.exists());
    }

    /// The real `pw-record`, with no microphone: the person names a silent virtual source
    /// that exists. `pactl load-module module-null-sink media.class=Audio/Source/Virtual
    /// sink_name=silent channel_map=mono`, then
    /// `SPLAT_TEST_SOURCE=silent cargo test --bin splat live_pw_record -- --ignored`
    #[test]
    #[ignore]
    fn live_pw_record_writes_a_finished_file() {
        let source = std::env::var("SPLAT_TEST_SOURCE").expect("a silent source to record");
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("live.wav");
        let mut recorder = Recorder::start(Tool::PwRecord, Some(&source), path.clone()).unwrap();
        // Let it settle, because the stream takes a moment to open.
        std::thread::sleep(Duration::from_secs(1));
        for _ in 0..30 {
            std::thread::sleep(Duration::from_millis(100));
            recorder.checked = Instant::now() - CHECK_EVERY;
            recorder.poll();
        }
        let health = recorder.health().clone();
        recorder.stop();
        let layout = Layout::read(&path).unwrap();
        println!("{health:?} {layout:?}");
        assert_eq!(health, Health::Silent);
        assert_eq!((layout.rate, layout.channels, layout.bits), (16_000, 1, 16));
        assert!(layout.seconds() >= 1, "{layout:?}");
        assert!(!layout.header_wrong);
    }

    #[test]
    fn test_builds_simulate_and_start_no_process() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("out.wav");
        let recorder = Recorder::start(Tool::Simulated, None, path.clone()).unwrap();
        assert!(matches!(recorder.capture, Capture::Simulated));
        assert_eq!(recorder.stop(), path);
        assert_eq!(Layout::read(&path).unwrap().seconds(), 1);
        assert_eq!(probe().recv().unwrap().tool, Some(Tool::Simulated));
    }
}
