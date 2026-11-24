//! Shared UI parts. Screens compose these instead of styling widgets themselves.
use super::{comparison::Comparison, icons, tokens::*};

/// [`words_that_fit`] with any measure of a line's width (the GPUI text system's).
pub fn words_that_fit_by(text: &str, width: f32, measure: impl Fn(&str) -> f32) -> String {
    let mut words: Vec<_> = text.split_whitespace().collect();
    let full = words.join(" ");
    if measure(&full) <= width || words.is_empty() {
        return full;
    }
    while let Some(word) = words.last() {
        let joining = ["to", "of", "on", "from", "and", "the", "de", "von", "·"]
            .iter()
            .any(|joining| word.eq_ignore_ascii_case(joining));
        if !joining {
            let fitted = format!("{}…", words.join(" "));
            if words.len() == 1 || measure(&fitted) <= width {
                return fitted;
            }
        }
        words.pop();
    }
    "…".to_owned()
}

/// Temporary HP, with its mark, in every place that shows it.
pub fn temp_hp_text(temp: i64) -> String {
    format!("{} +{temp}", icons::TEMP_HP)
}

// ------------------------------------------------------------------------------------------------
// Reader toolbar (M2): one availability value per action gives its lock, its hover, its
// accessible description and its painted reason.

/// What an open draft blocks, so the part says why in its own words: a long whole sentence,
/// and a short whole one for a narrow save row. The words belong to the part, not to a call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DraftBlocks {
    /// A campaign entry's draft: copying, a new entry, the package list and the campaign switch.
    EntryInCampaign,
    /// A package entry's draft: copying, a new entry and the package list.
    EntryInPackage,
}

impl DraftBlocks {
    /// The whole reason.
    pub const fn long(self) -> &'static str {
        match self {
            Self::EntryInCampaign => {
                "Save or discard to copy, add an entry, change packages or switch campaign."
            }
            Self::EntryInPackage => "Save or discard to copy, add an entry or change packages.",
        }
    }

    /// The short whole reason, for a save row with less room; hover keeps the long one.
    pub const fn short(self) -> &'static str {
        "Save or discard first."
    }
}

/// What the add field keeps between frames. Its caller owns one for each list.
#[derive(Clone, Debug, Default)]
pub struct AddField {
    /// The words typed so far.
    pub words: String,
    /// The suggestion that Enter takes. Only arrows choose one, so Enter adds the words.
    highlight: Option<usize>,
    /// The field takes focus on the next frame.
    pub focus: bool,
}

impl AddField {
    /// A line was added from these words: the field is empty again.
    pub fn clear(&mut self) {
        self.words.clear();
        self.highlight = None;
    }
}

/// One group of suggestions under the add field: its label, then each row's title, its second
/// line and the value a choice of it gives.
pub struct Suggestions<T> {
    pub label: &'static str,
    pub rows: Vec<(String, String, T)>,
}

/// A state as an icon, a word and a colour. Colour is never the only signal. A status whose
/// icon and word differ in colour is a `KindStatus` (`design::status`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status<'a> {
    pub icon: &'a str,
    pub word: &'a str,
    pub color: Color,
}

/// The word that goes with an HP colour, as an icon, a word and a colour. Colour is never the
/// only sign. Healthy and damaged HP need no word.
pub fn hp_word(hp: Option<i64>, max: Option<i64>) -> Option<(&'static str, &'static str, Color)> {
    let c = colors();
    match hp_state(hp, max) {
        HpState::Zero => Some((icons::ZERO_HP, "0 HP", c.hp.zero)),
        HpState::Bloodied => Some((icons::BLOODIED, "Bloodied", c.hp.bloodied)),
        HpState::Unknown => Some((icons::WARN, "HP unknown", c.muted)),
        _ => None,
    }
}

/// How HP stands. Bloodied is half or less, and it is a label with no rule behind it.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum HpState {
    Unknown,
    Healthy,
    Damaged,
    Bloodied,
    Zero,
}

pub fn hp_state(hp: Option<i64>, max: Option<i64>) -> HpState {
    match (hp, max) {
        (Some(0), _) => HpState::Zero,
        (Some(hp), Some(max)) if max > 0 && hp * 2 <= max => HpState::Bloodied,
        (Some(hp), Some(max)) if hp < max => HpState::Damaged,
        (Some(_), Some(_)) => HpState::Healthy,
        _ => HpState::Unknown,
    }
}

pub fn hp_color(state: HpState) -> Color {
    let hp = colors().hp;
    match state {
        HpState::Unknown => hp.unknown,
        HpState::Healthy => hp.healthy,
        HpState::Damaged => hp.damaged,
        HpState::Bloodied => hp.bloodied,
        HpState::Zero => hp.zero,
    }
}

/// A displayed current/maximum fraction. Callers supply their own unknown-value words.
pub fn fraction(current: impl std::fmt::Display, maximum: impl std::fmt::Display) -> String {
    format!("{current}/{maximum}")
}

/// A whole number as the DM may type it, also as the app writes it ([`signed`]): `−3` (a true
/// minus), `-3`, `+2`, `2`, with spaces around it. The one reader of typed signed numbers (a
/// run's amounts and initiative, a tracker's Amount, World's initiative), so a DM can type what
/// the app shows.
pub fn parse_signed(text: &str) -> Option<i64> {
    // `i64`'s own parse takes one `+` or `-`; the true minus becomes `-`.
    text.trim().replace('\u{2212}', "-").parse().ok()
}

/// A character of a typed whole number: a digit, `+`, `-` or a true minus (`−`, as [`signed`]
/// writes it). The number fields keep these and drop the rest; [`parse_signed`] reads them.
pub fn number_char(c: char) -> bool {
    c.is_ascii_digit() || matches!(c, '+' | '-' | '\u{2212}')
}

/// A modifier, with a true minus sign: `+3`, `−1`.
pub fn signed(value: i64) -> String {
    if value < 0 {
        format!("\u{2212}{}", value.unsigned_abs())
    } else {
        format!("+{value}")
    }
}

/// A count and the right word for it: `1 entry`, `2 entries`.
/// The app's one number form: digits grouped by threes with commas (`2,310`).
pub fn grouped_number(n: usize) -> String {
    let digits = n.to_string();
    let mut grouped = String::new();
    for (i, digit) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    grouped
}

pub fn count(n: usize, one: &str, many: &str) -> String {
    if n == 0 {
        return String::new();
    }
    let grouped = grouped_number(n);
    let unit = if n == 1 { one } else { many };
    if unit.is_empty() {
        grouped
    } else {
        format!("{grouped} {unit}")
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tone {
    Info,
    Success,
    Warning,
    Error,
}

impl Tone {
    pub(super) fn parts(self) -> (&'static str, Color) {
        let result = colors().result;
        match self {
            Self::Info => (icons::INFO, result.info),
            Self::Success => (icons::SUCCESS, result.success),
            Self::Warning => (icons::WARN, result.warning),
            Self::Error => (icons::ERROR, result.error),
        }
    }
}

/// Where a footer message stands in the footer's one order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(super) enum FooterRank {
    /// The answer to the DM's last act, when it failed (ruling Q-x, `Messages::answer`): it is
    /// newer than every kept result, so it comes first.
    Answer,
    /// A result whose Undo is live.
    Undo,
    /// Why Save or the draft waits: Save's reason, a parse error, a disk failure.
    Reason,
    /// Every other notice, and a result with no live Undo.
    Notice,
}

/// When the action that made a footer result was done: a later action has a larger order. Issue
/// one with `ResultOrder::issue` when the action is done (an Undo and then the same action again
/// issue a new one), and keep it with the result. A draw, changed words, or hiding or removing
/// another result never issues one: the footer puts results newest first by it, so a result that
/// gets its order at draw time would jump ahead of newer ones.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ResultOrder(u64);

/// Older than every issued order: only for a field whose result does not exist yet. A result
/// that shows holds the order its action issued.
impl Default for ResultOrder {
    fn default() -> Self {
        Self(0)
    }
}

impl ResultOrder {
    /// A new order, later than every order issued before it in this run of the app.
    pub fn issue() -> Self {
        static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self(NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}

/// The same safe-first choice for a draft whose file changed on disk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskConflictChoice {
    Overwrite,
    Reload,
}

/// The width of a panel beside a page: Lookup, or the run's side panel.
pub fn side_panel_width(available: f32) -> f32 {
    (available * 0.36).clamp(PANEL_MIN_WIDTH, PANEL_MAX_WIDTH)
}

/// Consecutive differing lines are one destination for Next difference.
#[cfg(test)]
pub fn comparison_changes(draft: &str, disk: &str) -> (usize, Vec<usize>) {
    let comparison = Comparison::new([draft, disk]);
    (comparison.changed, comparison.groups)
}

#[test]
fn iter3_comparisons_count_only_actual_insertions_and_deletions() {
    for (left, right, expected) in [
        ("one\nremoved\ntwo\nthree", "one\ntwo\nthree", (1, vec![1])),
        ("one\ntwo\nthree", "added\none\ntwo\nthree", (1, vec![0])),
        (
            "one\nold\ntwo\nthree",
            "one\nnew\nextra\ntwo\nthree",
            (2, vec![1]),
        ),
        (
            "same\nold\nsame\nlast",
            "same\nsame\nnew\nlast",
            (2, vec![1, 3]),
        ),
        ("", "added", (1, vec![0])),
        ("one\ntwo", "one\ntwo", (0, vec![])),
    ] {
        assert_eq!(
            comparison_changes(left, right),
            expected,
            "{left:?} vs {right:?}"
        );
    }
}

#[test]
fn adjacent_draft_lines_form_one_change() {
    assert_eq!(
        comparison_changes("one\ntwo\nthree", "one\nx\ny"),
        (2, vec![1])
    );
    assert_eq!(
        comparison_changes("one\ntwo\nthree", "x\ntwo\ny"),
        (2, vec![0, 2])
    );
}

/// The comparison's summary: the count, then the nearest heading at or above `line` of the left
/// column (`5 lines differ · Under the heading “Actions”`).
pub fn comparison_summary_words(count: &str, lines: &[Option<&str>], line: usize) -> String {
    let heading = lines
        .iter()
        .take(line + 1)
        .rev()
        .flatten()
        .find(|line| line.trim_start().starts_with('#'));
    heading.map_or_else(
        || count.to_owned(),
        |heading| {
            format!(
                "{count} · Under the heading “{}”",
                heading.trim_start_matches('#').trim()
            )
        },
    )
}

/// A draft and the file's text aligned line by line, for a UI that draws the comparison itself
/// (the GPUI port): the same alignment, count and change groups `draft_comparison_labeled` uses.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ComparedLines {
    /// Each row's line of the draft and of the file; `None` where that side has no line.
    pub rows: Vec<[Option<String>; 2]>,
    /// How many rows differ.
    pub changed: usize,
    /// The row where each change starts.
    pub groups: Vec<usize>,
}

impl ComparedLines {
    /// Align `draft` and `disk` (about n × m line comparisons: keep the result while both texts
    /// stay the same).
    pub fn of(draft: &str, disk: &str) -> Self {
        let comparison = Comparison::new([draft, disk]);
        let [left, right] = comparison.columns;
        Self {
            rows: left
                .into_iter()
                .zip(right)
                .map(|(left, right)| [left.map(str::to_owned), right.map(str::to_owned)])
                .collect(),
            changed: comparison.changed,
            groups: comparison.groups,
        }
    }

    /// The summary over the comparison at change `group` (the first when there is none):
    /// `No lines differ`, or the count and the heading the change is under.
    pub fn summary(&self, group: Option<usize>) -> String {
        if self.groups.is_empty() {
            return "No lines differ".into();
        }
        let line = group
            .and_then(|at| self.groups.get(at))
            .copied()
            .unwrap_or(self.groups[0]);
        let left: Vec<Option<&str>> = self.rows.iter().map(|row| row[0].as_deref()).collect();
        comparison_summary_words(
            &super::count(self.changed, "line differs", "lines differ"),
            &left,
            line,
        )
    }
}

/// The folder a group of notes is in: the campaign folder itself (the root), or a folder inside
/// it. Its form is private: build it with [`FolderGroup::of_path`], which decides the root.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FolderGroup<'a>(Folder<'a>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Folder<'a> {
    /// Notes directly in the campaign folder. No folder of that name exists, so the heading
    /// names the place, [`FolderGroup::ROOT_NAME`], with no `/`.
    Root,
    /// A folder inside the campaign folder, by its path from there, with no `/` at either end
    /// and no `./` before it. The heading shows the path with one `/` after it.
    Inside(&'a str),
}

impl<'a> FolderGroup<'a> {
    /// The root group's name: the campaign's folder (CONTEXT.md: a campaign has a folder for its
    /// files).
    pub const ROOT_NAME: &'static str = "Campaign folder";

    /// The group of a folder path relative to the campaign folder, as `/`-separated names
    /// (`Places`, `Lore/Old`). `/` at either end and `./` before it are dropped; what is left
    /// empty, or `.`, is the root.
    pub fn of_path(path: &'a str) -> Self {
        let mut path = path.trim_matches('/');
        while let Some(rest) = path.strip_prefix("./") {
            path = rest.trim_start_matches('/');
        }
        if path.is_empty() || path == "." {
            Self(Folder::Root)
        } else {
            Self(Folder::Inside(path))
        }
    }

    /// The words of its heading, without the count.
    pub fn words(self) -> String {
        match self.0 {
            Folder::Root => Self::ROOT_NAME.to_owned(),
            Folder::Inside(path) => format!("{path}/"),
        }
    }
}

#[cfg(test)]
// The tests draw the old message functions until Wave 3 makes them private.
#[allow(deprecated)]
mod tests {
    use super::*;

    #[test]
    fn counts_use_separators_and_hide_zero() {
        assert_eq!(count(0, "entry", "entries"), "");
        assert_eq!(count(1, "entry", "entries"), "1 entry");
        assert_eq!(count(2108, "entry", "entries"), "2,108 entries");
        assert_eq!(count(1234567, "", ""), "1,234,567");
    }

    #[test]
    fn bloodied_is_half_or_less_and_zero_stands_alone() {
        assert_eq!(hp_state(None, Some(10)), HpState::Unknown);
        assert_eq!(hp_state(Some(10), None), HpState::Unknown);
        assert_eq!(hp_state(Some(10), Some(10)), HpState::Healthy);
        assert_eq!(hp_state(Some(9), Some(10)), HpState::Damaged);
        assert_eq!(hp_state(Some(5), Some(10)), HpState::Bloodied);
        assert_eq!(hp_state(Some(6), Some(11)), HpState::Damaged);
        assert_eq!(hp_state(Some(0), Some(10)), HpState::Zero);
        assert_eq!(hp_state(Some(0), None), HpState::Zero);
    }
}
