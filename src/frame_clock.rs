//! The app's one clock for words that end by themselves (round 30 B7d): `Recording saved` on the
//! rail, `Hearing sound`, and the Add results of the plan and the Timeline with their Undo. It is
//! the time since the app started, set once at the start of each pass of the frame logic
//! (`tick`, from `StateContext::begin_pass`). It moves with real seconds, so every timing stays
//! as it was. A unit test sets it (`set`), so the steps a test takes, not the machine's speed,
//! decide when such a word ends.
use std::{cell::Cell, time::Duration};

thread_local! {
    static NOW: Cell<f64> = const { Cell::new(0.0) };
}

/// Set the frame clock to `seconds`: at the start of each pass, before anything reads `now`.
pub fn tick(seconds: f64) {
    NOW.with(|now| now.set(seconds));
}

/// The frame clock, in seconds, as the pass's `tick` set it.
pub fn now() -> f64 {
    NOW.with(Cell::get)
}

/// Has `duration` passed on the frame clock since `at` (a value of `now`)?
pub fn passed(at: f64, duration: Duration) -> bool {
    now() - at >= duration.as_secs_f64()
}

/// Set the frame clock, for a unit test that steps it past a timer's end.
#[cfg(any(test, feature = "test-support"))]
pub fn set(seconds: f64) {
    NOW.with(|now| now.set(seconds));
}

#[cfg(test)]
mod tests {
    /// Every read of the frame clock outside this file, by file, outside test modules: a count
    /// that can only shrink. Ruling L9 (layout PR 2a): a timed result or word ends in its
    /// owner's update, which also asks for a repaint at its end; a reader or a footer's
    /// construction never reads the clock. The reads left are the acts that start a timer and
    /// the owner updates that end one:
    ///
    /// - `design::results`: `Results::record_timed` (the act's time), `Results::expire` and
    ///   the entry's own end check;
    /// - `recording`: the recorder's start, a heard sound (`poll`, `simulate_health`), and
    ///   `Recorder::expire`;
    /// - `sessions::page`: the stop (`stop_recorder`) and `Sessions::settle_words`.
    const READS: &[(&str, usize)] = &[
        ("src/design/results.rs", 4),
        ("src/recording.rs", 5),
        ("src/sessions/page.rs", 3),
    ];

    #[test]
    fn the_frame_clock_is_read_only_where_a_timer_starts_or_ends() {
        fn collect(folder: &std::path::Path, files: &mut Vec<(String, String)>) {
            for item in std::fs::read_dir(folder).expect("read source folder") {
                let path = item.expect("source entry").path();
                if path.is_dir() {
                    collect(&path, files);
                } else if path.extension().is_some_and(|ext| ext == "rs") {
                    let text = std::fs::read_to_string(&path).expect("read source file");
                    files.push((
                        path.strip_prefix(crate::workspace())
                            .expect("path in workspace")
                            .to_string_lossy()
                            .replace('\\', "/"),
                        text,
                    ));
                }
            }
        }
        let mut files = Vec::new();
        collect(&crate::workspace().join("src"), &mut files);
        let mut found: Vec<(String, usize)> = files
            .iter()
            .filter(|(path, _)| path != "src/frame_clock.rs")
            .filter_map(|(path, text)| {
                let code = text
                    .split_once("\n#[cfg(test)]\nmod tests {")
                    .map_or(text.as_str(), |(code, _)| code);
                let reads = code.matches("frame_clock::now(").count()
                    + code.matches("frame_clock::passed(").count();
                (reads > 0).then(|| (path.clone(), reads))
            })
            .collect();
        found.sort();
        let listed: Vec<(String, usize)> = READS
            .iter()
            .map(|(path, reads)| ((*path).to_owned(), *reads))
            .collect();
        assert_eq!(
            found, listed,
            "a timed word ends in its owner's update, not where it is read (ruling L9); a \
             removed read lowers its row"
        );
    }
}
