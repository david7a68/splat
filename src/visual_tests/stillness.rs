//! Gate invariants that compare what the app shows, report-only for now:
//!
//! - **G1a** a message changed in a pane, and content below its top in that pane moved.
//! - **G1b** the control a script pressed moved after the press.
//! - **G1c** a scroll bar came or went in a pane, and a right edge in that pane moved.
//! - **G2** a header action at the widest size is gone, or paints other words, at a narrower size.
//! - **G3** at 1920 px, a row's first action at the right is far from the row's last words.
//! - **G4** one pane shows the same sentence twice.
//!
//! G1 compares two states in a row at one window size. It matches a control by its pane, role,
//! accessible name and place among equal names, never by AccessKit id: egui's automatic ids
//! shift when a widget is inserted before them. `Driver::expect_moves` is the one escape: it
//! marks the next state's G1 findings as intended, with a reason the report counts.
//!
//! Messages: until `src/design` registers each message's rect, a message is found in the painted
//! shapes by what `design::message` and `design::small_message` draw: a 3 px bar in a result tone,
//! a tone icon after it, then the words. The words' AccessKit label gives the pane.
use super::ergonomics::Control;
use crate::design::{L, S, XS};
use eframe::egui::{self, accesskit::Role, epaint::ClippedShape};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub const G1A: &str = "G1a/message-moved-content";
pub const G1B: &str = "G1b/pressed-control-moved";
pub const G1C: &str = "G1c/content-moved-with-bar";
pub const G2: &str = "G2/header-action-lost";
pub const G3: &str = "G3/far-action";
pub const G4: &str = "G4/same-sentence-twice";
pub const CHECKS: [&str; 6] = [G1A, G1B, G1C, G2, G3, G4];

/// Rounding, and a selected word's heavier weight, move an edge by up to this much. A person does
/// not see it.
const MOVE_TOLERANCE: f32 = 1.5;
/// G1c: the widest lane a vertical scroll bar takes (its width and outer margin).
const BAR_LANE: f32 = 24.0;
/// G2 reads a control as an action when it paints at most this many words (icons left out).
/// A longer control, such as a party member's chip, is a status that may show less when narrow.
const ACTION_WORDS: usize = 4;
/// G2 looks at actions whose top is this close to the window's top: the app bar, the page's
/// title line and the first title line of its pane.
pub const HEADER_BAND: f32 = 11.0 * L;
/// G3: the largest gap from a row's last words to its first action at the right.
pub const FAR_GAP: f32 = 3.0 * L;
/// G4 compares texts of at least this many words.
const G4_MIN_WORDS: usize = 4;

/// One finding: a line of `stillness.json`.
#[derive(Clone, Debug, Serialize)]
pub struct Finding {
    pub check: &'static str,
    pub scenario: String,
    pub state: String,
    pub window: String,
    pub control: String,
    /// x, y, width, height in points, before and after (for G3: the words, then the action;
    /// for G4: the first text, then the second).
    pub old: [f32; 4],
    pub new: [f32; 4],
    /// The message, press, bar or size that set the check off.
    pub trigger: String,
    /// The reason of the `expect_moves` that covers this finding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escaped: Option<&'static str>,
}

/// A finding before it has a scenario, state and window.
#[derive(Clone, Debug, PartialEq)]
pub struct Raw {
    pub check: &'static str,
    pub control: String,
    pub old: egui::Rect,
    pub new: egui::Rect,
    pub trigger: String,
}

fn xywh(rect: egui::Rect) -> [f32; 4] {
    [rect.min.x, rect.min.y, rect.width(), rect.height()]
}

/// A control or a free text, named so that the next state can find it again.
#[derive(Clone, Debug)]
pub struct Item {
    pub scope: u64,
    pub role: Role,
    pub name: String,
    pub rect: egui::Rect,
    pub clip: Option<egui::Rect>,
}

impl Item {
    fn is_label(&self) -> bool {
        self.role == Role::Label
    }
}

/// A drawn message: its words, the rect from its bar to its last word, and its pane.
#[derive(Clone, Debug)]
pub struct Message {
    pub words: String,
    pub rect: egui::Rect,
    pub scope: Option<u64>,
    pub clip: Option<egui::Rect>,
}

/// A painted text and the part of the screen it covers.
#[derive(Clone, Debug)]
pub struct Text {
    pub words: String,
    pub rect: egui::Rect,
}

/// What G1 keeps of one checked state at one size.
#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    /// The driver's state number.
    pub state: usize,
    /// The visible page: G1 does not compare across a page change.
    pub page: String,
    pub items: Vec<Item>,
    /// Vertical scroll bars.
    pub bars: Vec<egui::Rect>,
    pub messages: Vec<Message>,
}

/// Items from the ergonomics controls: controls and free texts (labels not inside a control),
/// outside popups and modal-blocked layers. `words` gives a label's words (egui puts them in the
/// value). Also the vertical scroll bars.
pub fn items(controls: &[Control], words: &HashMap<u64, String>) -> (Vec<Item>, Vec<egui::Rect>) {
    let mut items = Vec::new();
    let mut bars = Vec::new();
    for control in controls {
        if control.chrome {
            let rect = control.rect;
            if control.role == Role::Unknown
                && control.name.is_empty()
                && rect.width() <= 20.0
                && rect.height() > 2.0 * rect.width()
            {
                bars.push(rect);
            }
            continue;
        }
        if control.popup.is_some() || control.blocked {
            continue;
        }
        let name = if control.is_label() {
            if control.label_owner.is_some() {
                continue;
            }
            words.get(&control.id).cloned().unwrap_or_default()
        } else if control.name.is_empty() {
            control.placeholder.clone().unwrap_or_default()
        } else {
            control.name.clone()
        };
        if name.trim().is_empty() {
            continue;
        }
        items.push(Item {
            scope: control.scope,
            role: control.role,
            name,
            rect: control.rect,
            clip: control.clip,
        });
    }
    (items, bars)
}

/// Each painted text that is at least partly inside its clip.
pub fn painted_texts(shapes: &[ClippedShape]) -> Vec<(egui::epaint::TextShape, egui::Rect)> {
    fn walk(
        shape: &egui::Shape,
        clip: egui::Rect,
        out: &mut Vec<(egui::epaint::TextShape, egui::Rect)>,
    ) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, clip, out)),
            egui::Shape::Text(text)
                if !text.galley.job.text.trim().is_empty()
                    && clip.intersects(text.visual_bounding_rect()) =>
            {
                out.push((text.clone(), clip));
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, clipped.clip_rect, &mut out);
    }
    out
}

pub fn texts(painted: &[(egui::epaint::TextShape, egui::Rect)]) -> Vec<Text> {
    painted
        .iter()
        .map(|(text, clip)| Text {
            words: text.galley.job.text.clone(),
            rect: text.visual_bounding_rect().intersect(*clip),
        })
        .collect()
}

/// The messages that `design::message` and `design::small_message` drew this pass.
pub fn messages(
    shapes: &[ClippedShape],
    painted: &[(egui::epaint::TextShape, egui::Rect)],
    items: &[Item],
) -> Vec<Message> {
    fn bars(shape: &egui::Shape, tones: &[egui::Color32], out: &mut Vec<egui::Rect>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| bars(shape, tones, out)),
            egui::Shape::Rect(rect)
                if tones.contains(&rect.fill)
                    && (2.5..=3.5).contains(&rect.rect.width())
                    && rect.rect.height() >= S =>
            {
                out.push(rect.rect);
            }
            _ => {}
        }
    }
    let result = crate::design::colors().result;
    let tones = [result.info, result.success, result.warning, result.error];
    let icons = [
        crate::design::icons::INFO,
        crate::design::icons::SUCCESS,
        crate::design::icons::WARN,
        crate::design::icons::ERROR,
    ];
    let mut found = Vec::new();
    for clipped in shapes {
        bars(&clipped.shape, &tones, &mut found);
    }
    let mut out = Vec::new();
    for bar in found {
        let near_top = |text: &egui::epaint::TextShape| (text.pos.y - bar.top()).abs() <= S;
        // `message` puts its icon `S + 1` after the bar's left edge.
        let Some((icon, _)) = painted.iter().find(|(text, _)| {
            icons.contains(&text.galley.job.text.as_str())
                && near_top(text)
                && (bar.left() + S - 2.0..=bar.left() + S + 4.0).contains(&text.pos.x)
        }) else {
            continue;
        };
        let after = icon.pos.x + icon.galley.rect.width();
        // `message` adds `XS` after the icon; `small_message` takes `XS` back.
        let Some((words, clip)) = painted.iter().find(|(text, _)| {
            !icons.contains(&text.galley.job.text.as_str())
                && near_top(text)
                && (after - XS - 2.0..=after + XS + 2.0).contains(&text.pos.x)
        }) else {
            continue;
        };
        let words_rect = words.visual_bounding_rect();
        let text = words.galley.job.text.clone();
        let label = items.iter().find(|item| {
            item.is_label()
                && item.name == text
                && (item.rect.left() - words.pos.x).abs() <= 2.0
                && (item.rect.top() - words.pos.y).abs() <= 2.0
        });
        out.push(Message {
            words: text,
            rect: bar.union(words_rect).intersect(*clip),
            scope: label.map(|item| item.scope),
            clip: label.and_then(|item| item.clip),
        });
    }
    out
}

fn same_clip(a: Option<egui::Rect>, b: Option<egui::Rect>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => moved(a, b).is_none(),
        _ => false,
    }
}

/// The largest edge change, when it is past the tolerance.
fn moved(old: egui::Rect, new: egui::Rect) -> Option<f32> {
    let change = (old.min - new.min)
        .abs()
        .max_elem()
        .max((old.max - new.max).abs().max_elem());
    (change > MOVE_TOLERANCE).then_some(change)
}

type Key<'a> = (u64, Role, &'a str, usize);

/// Each item by pane, role, name and its place among items with that pane, role and name.
fn keyed(items: &[Item]) -> HashMap<Key<'_>, &Item> {
    let mut seen: HashMap<(u64, Role, &str), usize> = HashMap::new();
    let mut out = HashMap::new();
    for item in items {
        let place = seen.entry((item.scope, item.role, &item.name)).or_default();
        out.insert((item.scope, item.role, item.name.as_str(), *place), item);
        *place += 1;
    }
    out
}

fn display(item: &Item) -> String {
    format!("{:?} {:?}", item.role, item.name)
}

/// G1a: a message came or went between `old` and `new`; every item below its top in its pane
/// keeps its rect, unless its scroll area changed.
pub fn message_moves(old: &Snapshot, new: &Snapshot) -> Vec<Raw> {
    let words = |snapshot: &Snapshot| -> Vec<String> {
        snapshot
            .messages
            .iter()
            .map(|message| message.words.clone())
            .collect()
    };
    let (old_words, new_words) = (words(old), words(new));
    let mut changed = Vec::new();
    for message in &new.messages {
        if !old_words.contains(&message.words) {
            changed.push((message, new, old, "appeared"));
        }
    }
    for message in &old.messages {
        if !new_words.contains(&message.words) {
            changed.push((message, old, new, "went"));
        }
    }
    let mut reported = HashSet::new();
    let mut out = Vec::new();
    for (message, with, without, verb) in changed {
        let other = keyed(&without.items);
        let mine = keyed(&with.items);
        let in_pane = |item: &Item| {
            let same_pane = match message.scope {
                Some(scope) => item.scope == scope && same_clip(item.clip, message.clip),
                None => true,
            };
            same_pane
                && (message.clip.is_some()
                    || item.rect.right() > message.rect.left() - MOVE_TOLERANCE)
        };
        // A message's own line (its icon, words and actions) moves with it, and so does a
        // message below it in the same footer.
        let on_a_message_line = |item: &Item| {
            with.messages.iter().any(|m| {
                m.rect
                    .y_range()
                    .expand(MOVE_TOLERANCE)
                    .contains(item.rect.center().y)
                    && item.rect.left() >= m.rect.left() - S
            })
        };
        // Content above the message is the anchor: when part of it is not in the other state, the
        // pane shows something else now (another note), and a move says nothing.
        let mut above = mine
            .iter()
            .filter(|(_, item)| {
                in_pane(item) && item.rect.bottom() <= message.rect.top() + MOVE_TOLERANCE
            })
            .peekable();
        if above.peek().is_none() || !above.all(|(key, _)| other.contains_key(key)) {
            continue;
        }
        for (key, item) in &mine {
            if !in_pane(item)
                || item.rect.top() < message.rect.top() - MOVE_TOLERANCE
                || on_a_message_line(item)
                || reported.contains(key)
            {
                continue;
            }
            let Some(&there) = other.get(key) else {
                continue;
            };
            if !same_clip(item.clip, there.clip) {
                continue;
            }
            let (before, after) = if verb == "appeared" {
                (there.rect, item.rect)
            } else {
                (item.rect, there.rect)
            };
            if moved(before, after).is_some() {
                reported.insert(*key);
                out.push(Raw {
                    check: G1A,
                    control: display(item),
                    old: before,
                    new: after,
                    trigger: format!("message {verb}: {:?}", message.words),
                });
            }
        }
    }
    out
}

/// G1c: a vertical scroll bar came or went; no right edge in its pane moved.
pub fn bar_moves(old: &Snapshot, new: &Snapshot) -> Vec<Raw> {
    let same_bar = |a: &egui::Rect, b: &egui::Rect| {
        (a.left() - b.left()).abs() <= 2.0 && a.y_range().intersects(b.y_range())
    };
    let mut changed = Vec::new();
    for bar in &new.bars {
        if !old.bars.iter().any(|other| same_bar(bar, other)) {
            changed.push((*bar, new, old, "appeared"));
        }
    }
    for bar in &old.bars {
        if !new.bars.iter().any(|other| same_bar(bar, other)) {
            changed.push((*bar, old, new, "went"));
        }
    }
    let mut reported = HashSet::new();
    let mut out = Vec::new();
    for (bar, with, without, verb) in changed {
        let other = keyed(&without.items);
        for (key, item) in keyed(&with.items) {
            // The bar sits just right of its scroll area's visible part.
            let in_pane = item.clip.is_some_and(|clip| {
                (-MOVE_TOLERANCE..=21.0).contains(&(bar.left() - clip.right()))
                    && clip.y_range().intersects(bar.y_range())
            });
            if !in_pane || reported.contains(&key) {
                continue;
            }
            let Some(&there) = other.get(&key) else {
                continue;
            };
            let (before, after) = if verb == "appeared" {
                (there.rect, item.rect)
            } else {
                (item.rect, there.rect)
            };
            // A bar takes or gives back its lane: the right edge moves by at most the lane, and
            // the line stays. A larger move is a new layout, not the bar.
            let shift = (before.right() - after.right()).abs();
            if shift > MOVE_TOLERANCE
                && shift <= BAR_LANE
                && (before.top() - after.top()).abs() <= MOVE_TOLERANCE
            {
                reported.insert(key);
                out.push(Raw {
                    check: G1C,
                    control: display(item),
                    old: before,
                    new: after,
                    trigger: format!("scroll bar {verb} at x {:.0}", bar.left()),
                });
            }
        }
    }
    out
}

/// The control a press aims at, found again after the press.
#[derive(Clone, Debug)]
pub struct Pressed {
    pub role: Role,
    pub name: String,
    /// Its place among nodes with this role and name.
    pub place: usize,
    pub id: u64,
    pub rect: egui::Rect,
    pub page: String,
}

/// A node as the driver sees it: id, role, name and its whole rect.
pub type Node<'a> = (u64, Role, &'a str, egui::Rect);

/// The node a press at `aim` (the visible part of the node the driver chose) goes to. Nothing for
/// a link: it opens something else, so what was under the pointer is new.
pub fn pressed(
    nodes: &[Node<'_>],
    shown: &[egui::Rect],
    aim: egui::Rect,
    page: &str,
) -> Option<Pressed> {
    let index = nodes.iter().zip(shown).position(|((_, role, ..), shown)| {
        !matches!(role, Role::Label | Role::TextRun) && moved(*shown, aim).is_none()
    })?;
    let (id, role, name, rect) = nodes[index];
    if role == Role::Link {
        return None;
    }
    let place = nodes[..index]
        .iter()
        .filter(|(_, r, n, _)| *r == role && *n == name)
        .count();
    Some(Pressed {
        role,
        name: name.to_owned(),
        place,
        id,
        rect,
        page: page.to_owned(),
    })
}

/// G1b: after the press, the pressed control (found by role, name and place; by id when its name
/// changed) keeps its rect. Nothing when the page changed or the control is gone.
pub fn press_moves(before: &Pressed, after: &[Node<'_>], page: &str, caption: &str) -> Option<Raw> {
    if before.page != page {
        return None;
    }
    let equal: Vec<_> = after
        .iter()
        .filter(|(_, role, name, _)| *role == before.role && *name == before.name)
        .collect();
    let (rect, how) = if let Some(node) = equal.get(before.place) {
        (node.3, "")
    } else if equal.is_empty() {
        let node = after
            .iter()
            .find(|(id, role, ..)| *id == before.id && *role == before.role)?;
        (node.3, " (found by id; its name changed)")
    } else {
        return None;
    };
    moved(before.rect, rect)?;
    Some(Raw {
        check: G1B,
        control: format!("{:?} {:?}", before.role, before.name),
        old: before.rect,
        new: rect,
        trigger: format!("press: {caption}{how}"),
    })
}

/// One size of one state for G2: its window, width, the header controls with the words painted
/// in each, and every control name and role at that size.
pub struct HeaderView {
    pub window: String,
    pub width: f32,
    pub header: Vec<(Item, String)>,
    pub all: HashSet<(Role, String)>,
}

/// Controls that G2 reads as header actions: buttons, tabs, combo boxes and links near the
/// window's top that paint one to `ACTION_WORDS` words, with the words they paint.
pub fn header(items: &[Item], texts: &[Text], top: f32) -> Vec<(Item, String)> {
    let icon = |c: char| ('\u{e000}'..='\u{f8ff}').contains(&c);
    items
        .iter()
        .filter(|item| {
            matches!(
                item.role,
                Role::Button | Role::Tab | Role::ComboBox | Role::Link
            ) && item.rect.top() < top + HEADER_BAND
        })
        .filter_map(|item| {
            let mut words: Vec<&str> = texts
                .iter()
                .filter(|text| item.rect.expand(1.0).contains(text.rect.center()))
                .map(|text| text.words.as_str())
                .collect();
            words.sort_unstable();
            let words = words.join(" ");
            let count = words
                .split_whitespace()
                .filter(|word| !word.chars().all(icon))
                .count();
            (1..=ACTION_WORDS)
                .contains(&count)
                .then(|| (item.clone(), words))
        })
        .collect()
}

/// G2: every header action at the widest size is still there at each narrower size, with the
/// same painted words. Words are compared only for buttons, tabs and combo boxes: a link in
/// running text can wrap.
pub fn header_losses(views: &[HeaderView]) -> Vec<(String, Raw)> {
    let Some(widest) = views.iter().max_by(|a, b| a.width.total_cmp(&b.width)) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for view in views.iter().filter(|view| view.width < widest.width) {
        let more = view
            .all
            .iter()
            .any(|(_, name)| name == "More" || name.starts_with("More "));
        for (item, words) in &widest.header {
            if !view.all.contains(&(item.role, item.name.clone())) {
                out.push((
                    view.window.clone(),
                    Raw {
                        check: G2,
                        control: display(item),
                        old: item.rect,
                        new: egui::Rect::NOTHING,
                        trigger: format!(
                            "gone at {} (at {}: {words:?}){}",
                            view.window,
                            widest.window,
                            if more { "; a More menu is there" } else { "" }
                        ),
                    },
                ));
                continue;
            }
            if !matches!(item.role, Role::Button | Role::Tab | Role::ComboBox) {
                continue;
            }
            let Some((there, narrow)) = view
                .header
                .iter()
                .find(|(other, _)| other.role == item.role && other.name == item.name)
            else {
                continue;
            };
            if narrow != words {
                out.push((
                    view.window.clone(),
                    Raw {
                        check: G2,
                        control: display(item),
                        old: item.rect,
                        new: there.rect,
                        trigger: format!(
                            "paints {narrow:?} at {}, {words:?} at {}",
                            view.window, widest.window
                        ),
                    },
                ));
            }
        }
    }
    out
}

/// G3: in each registered row, the gap from the last words to the first action right of them is
/// at most `FAR_GAP`.
pub fn far_actions(rows: &[egui::Rect], items: &[Item], texts: &[Text]) -> Vec<Raw> {
    let mut out = Vec::new();
    for row in rows {
        let inside = |rect: egui::Rect| row.expand(MOVE_TOLERANCE).contains_rect(rect);
        let controls: Vec<&Item> = items
            .iter()
            .filter(|item| !item.is_label() && inside(item.rect))
            .collect();
        let words = texts.iter().filter(|text| {
            inside(text.rect)
                && !controls
                    .iter()
                    .any(|control| control.rect.expand(1.0).contains_rect(text.rect))
        });
        let Some(last) = words.max_by(|a, b| a.rect.right().total_cmp(&b.rect.right())) else {
            continue;
        };
        let Some(first) = controls
            .iter()
            .filter(|control| control.rect.left() >= last.rect.right() - MOVE_TOLERANCE)
            .min_by(|a, b| a.rect.left().total_cmp(&b.rect.left()))
        else {
            continue;
        };
        let gap = first.rect.left() - last.rect.right();
        if gap > FAR_GAP {
            out.push(Raw {
                check: G3,
                control: display(first),
                old: last.rect,
                new: first.rect,
                trigger: format!("{gap:.0} px after {:?}", last.words),
            });
        }
    }
    out
}

/// G4: in one pane, a text of `G4_MIN_WORDS` or more words that equals, or starts, another text.
/// Text inside a registered row is left out: list rows may share words.
pub fn repeated_sentences(items: &[Item], rows: &[egui::Rect]) -> Vec<Raw> {
    let in_row = |rect: egui::Rect| {
        rows.iter()
            .any(|row| row.expand(MOVE_TOLERANCE).contains_rect(rect))
    };
    let texts: Vec<&Item> = items
        .iter()
        .filter(|item| {
            item.is_label()
                && item.name.split_whitespace().count() >= G4_MIN_WORDS
                && !in_row(item.rect)
        })
        .collect();
    let starts = |long: &str, short: &str| {
        long.strip_prefix(short)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace))
    };
    let mut out = Vec::new();
    for (index, first) in texts.iter().enumerate() {
        for second in &texts[index + 1..] {
            if first.scope == second.scope
                && (starts(&first.name, &second.name) || starts(&second.name, &first.name))
            {
                out.push(Raw {
                    check: G4,
                    control: display(second),
                    old: first.rect,
                    new: second.rect,
                    trigger: format!("also {:?}", first.name),
                });
            }
        }
    }
    out
}

/// The driver's G state: the last snapshot per size, presses waiting for their state's name,
/// the escape for the next state, and what was found.
#[derive(Default)]
pub struct Tracker {
    last: HashMap<(u32, u32), Snapshot>,
    pending: Vec<Raw>,
    pending_window: String,
    escape: Option<&'static str>,
    pub column_dragged: bool,
    pub headers: Vec<HeaderView>,
    /// True while the driver checks a state: presses in its `prepare` are not the script's.
    pub in_state: bool,
    pub findings: Vec<Finding>,
    /// Each `expect_moves`: scenario, state and reason.
    pub escapes: Vec<(String, String, &'static str)>,
    pub time: Duration,
}

impl Tracker {
    /// A state begins: name the presses since the last one, and take the escape.
    pub fn begin(&mut self, scenario: &str, state: &str, escape: Option<&'static str>) {
        self.escape = escape;
        self.in_state = true;
        if let Some(reason) = escape {
            self.escapes
                .push((scenario.to_owned(), state.to_owned(), reason));
        }
        let window = std::mem::take(&mut self.pending_window);
        for raw in std::mem::take(&mut self.pending) {
            self.push(scenario, state, &window, raw);
        }
    }

    pub fn press(&mut self, window: String, raw: Raw) {
        self.pending_window = window;
        self.pending.push(raw);
    }

    pub fn push(&mut self, scenario: &str, state: &str, window: &str, raw: Raw) {
        let g1 = matches!(raw.check, G1A | G1B | G1C);
        self.findings.push(Finding {
            check: raw.check,
            scenario: scenario.to_owned(),
            state: state.to_owned(),
            window: window.to_owned(),
            control: raw.control,
            old: xywh(raw.old),
            new: xywh(raw.new),
            trigger: raw.trigger,
            escaped: if g1 { self.escape } else { None },
        });
    }

    /// Compare with the previous state at this size, when it is the state just before.
    pub fn observe(&mut self, size: (f32, f32), snapshot: Snapshot) -> Vec<Raw> {
        let key = (size.0 as u32, size.1 as u32);
        let mut out = Vec::new();
        if let Some(old) = self.last.get(&key)
            && old.state + 1 == snapshot.state
            && old.page == snapshot.page
            && !self.column_dragged
        {
            out.extend(message_moves(old, &snapshot));
            out.extend(bar_moves(old, &snapshot));
        }
        self.last.insert(key, snapshot);
        out
    }

    /// A state ends.
    pub fn end(&mut self) {
        self.in_state = false;
        self.column_dragged = false;
        self.escape = None;
    }

    /// Presses after the last state.
    pub fn finish(&mut self, scenario: &str) {
        let window = std::mem::take(&mut self.pending_window);
        for raw in std::mem::take(&mut self.pending) {
            self.push(scenario, "(after the last state)", &window, raw);
        }
    }
}

/// `stillness.json`: a JSON array with one finding per line, so `grep` and `diff` read it.
pub fn json_lines(findings: &[Finding]) -> String {
    let lines: Vec<String> = findings
        .iter()
        .map(|finding| serde_json::to_string(finding).expect("encode a finding"))
        .collect();
    format!("[\n{}\n]\n", lines.join(",\n"))
}

/// The counts the gate prints. Nothing here fails a run yet.
pub struct Report {
    /// Per check: findings, escaped findings, and states with a finding.
    counts: Vec<(&'static str, usize, usize, usize)>,
    /// `expect_moves` uses per scenario.
    escapes: std::collections::BTreeMap<String, Vec<String>>,
    time: Duration,
}

impl Report {
    pub fn new(
        findings: &[Finding],
        escapes: &[(String, String, &'static str)],
        time: Duration,
    ) -> Self {
        let counts = CHECKS
            .iter()
            .map(|check| {
                let of: Vec<&Finding> = findings.iter().filter(|f| f.check == *check).collect();
                let escaped = of.iter().filter(|f| f.escaped.is_some()).count();
                let states: HashSet<(&str, &str)> = of
                    .iter()
                    .filter(|f| f.escaped.is_none())
                    .map(|f| (f.scenario.as_str(), f.state.as_str()))
                    .collect();
                (*check, of.len() - escaped, escaped, states.len())
            })
            .collect();
        let mut by_scenario: std::collections::BTreeMap<String, Vec<String>> = Default::default();
        for (scenario, state, reason) in escapes {
            by_scenario
                .entry(scenario.clone())
                .or_default()
                .push(format!("{state}: {reason}"));
        }
        Self {
            counts,
            escapes: by_scenario,
            time,
        }
    }

    pub fn summary(&self) -> String {
        let counts: Vec<String> = self
            .counts
            .iter()
            .map(|(check, found, escaped, states)| {
                format!("{check} {found} in {states} states ({escaped} escaped)")
            })
            .collect();
        format!(
            "Stillness (report-only, stillness.json): {}. expect_moves: {} uses. Check time {:?} (summed over workers).",
            counts.join("; "),
            self.escapes.values().map(Vec::len).sum::<usize>(),
            self.time,
        )
    }

    pub fn markdown(&self) -> String {
        let mut out = String::from(
            "\n## Stillness and other G checks (report-only)\n\nEach finding is a line of `stillness.json`. They do not fail the run yet.\n\n| Check | Findings | States | Escaped by `expect_moves` |\n|---|---|---|---|\n",
        );
        for (check, found, escaped, states) in &self.counts {
            out.push_str(&format!("| {check} | {found} | {states} | {escaped} |\n"));
        }
        out.push_str(&format!(
            "\nCheck time, summed over workers: {:?}.\n",
            self.time
        ));
        if !self.escapes.is_empty() {
            out.push_str("\n### `expect_moves` uses\n\n");
            for (scenario, uses) in &self.escapes {
                out.push_str(&format!(
                    "- {scenario} ({}): {}\n",
                    uses.len(),
                    uses.join("; ")
                ));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(x: f32, y: f32, width: f32, height: f32) -> egui::Rect {
        egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, height))
    }

    fn item(role: Role, name: &str, rect: egui::Rect, clip: Option<egui::Rect>) -> Item {
        Item {
            scope: 1,
            role,
            name: name.into(),
            rect,
            clip,
        }
    }

    fn snapshot(items: Vec<Item>, messages: Vec<Message>, bars: Vec<egui::Rect>) -> Snapshot {
        Snapshot {
            state: 1,
            page: "World".into(),
            items,
            bars,
            messages,
        }
    }

    #[test]
    fn g1a_finds_content_a_new_message_pushed_down_and_not_content_above_it() {
        let body = Some(at(0.0, 0.0, 600.0, 600.0));
        let message = Message {
            words: "Changes saved.".into(),
            rect: at(10.0, 100.0, 200.0, 20.0),
            scope: Some(1),
            clip: body,
        };
        let above = item(Role::Label, "File line", at(10.0, 60.0, 200.0, 20.0), body);
        let row = |y| item(Role::Button, "Strength", at(10.0, y, 100.0, 24.0), body);
        let old = snapshot(vec![above.clone(), row(100.0)], vec![], vec![]);
        let new = snapshot(vec![above, row(128.0)], vec![message], vec![]);
        let found = message_moves(&old, &new);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].control, "Button \"Strength\"");
        assert_eq!(found[0].old.top(), 100.0);
        // A message that went is the same move backwards.
        assert_eq!(message_moves(&new, &old).len(), 1);
        // Content that stays still gives nothing; a message that stays gives nothing.
        let still = snapshot(new.items.clone(), new.messages.clone(), vec![]);
        assert!(message_moves(&new, &still).is_empty());
        let in_place = snapshot(
            vec![item(
                Role::Button,
                "Strength",
                at(10.0, 128.0, 100.0, 24.0),
                body,
            )],
            vec![],
            vec![],
        );
        assert!(message_moves(&in_place, &new).is_empty());
    }

    #[test]
    fn g1b_finds_a_pressed_control_that_moved_and_follows_a_renamed_one_by_id() {
        let nodes = [
            (7, Role::Button, "+ Success", at(300.0, 50.0, 80.0, 24.0)),
            (8, Role::Label, "1 success", at(200.0, 50.0, 60.0, 24.0)),
        ];
        let shown: Vec<_> = nodes.iter().map(|node| node.3).collect();
        let before = pressed(&nodes, &shown, nodes[0].3, "Run").expect("the pressed button");
        let after = [(7, Role::Button, "+ Success", at(310.0, 50.0, 80.0, 24.0))];
        let found = press_moves(&before, &after, "Run", "Click: + Success").expect("a move");
        assert_eq!(found.new.left(), 310.0);
        assert!(
            press_moves(&before, &nodes, "Run", "Click").is_none(),
            "it stayed"
        );
        assert!(
            press_moves(&before, &after, "World", "Click").is_none(),
            "another page"
        );
        let renamed = [(7, Role::Button, "Stop", at(320.0, 50.0, 80.0, 24.0))];
        assert!(press_moves(&before, &renamed, "Run", "Click").is_some());
    }

    #[test]
    fn g1c_finds_a_right_edge_that_moved_when_a_bar_came() {
        let clip = Some(at(0.0, 0.0, 600.0, 600.0));
        let field = |right: f32| {
            item(
                Role::TextInput,
                "Name",
                at(10.0, 10.0, right - 10.0, 24.0),
                clip,
            )
        };
        let bar = at(601.0, 0.0, 6.0, 600.0);
        let old = snapshot(vec![field(600.0)], vec![], vec![]);
        let new = snapshot(vec![field(588.0)], vec![], vec![bar]);
        assert_eq!(bar_moves(&old, &new).len(), 1);
        let kept = snapshot(vec![field(600.0)], vec![], vec![bar]);
        assert!(
            bar_moves(&old, &kept).is_empty(),
            "a kept lane moves nothing"
        );
    }

    #[test]
    fn g2_finds_a_header_action_that_is_gone_or_drops_its_words() {
        let action = |name: &str| item(Role::Button, name, at(500.0, 10.0, 80.0, 24.0), None);
        let view = |window: &str, width, header: Vec<(Item, String)>| HeaderView {
            window: window.into(),
            width,
            all: header
                .iter()
                .map(|(item, _)| (item.role, item.name.clone()))
                .collect(),
            header,
        };
        let wide = view(
            "1920x1080",
            1920.0,
            vec![
                (action("Edit"), "Edit".into()),
                (action("Open"), "Open".into()),
            ],
        );
        let narrow = view("960x640", 960.0, vec![(action("Edit"), String::new())]);
        let found = header_losses(&[narrow, wide]);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found.iter().any(|(_, raw)| raw.trigger.starts_with("gone")));
        assert!(
            found
                .iter()
                .any(|(_, raw)| raw.trigger.starts_with("paints \"\""))
        );
    }

    #[test]
    fn g3_finds_an_action_far_from_its_row_words() {
        let row = at(0.0, 0.0, 1600.0, 32.0);
        let words = Text {
            words: "Goblin".into(),
            rect: at(10.0, 4.0, 60.0, 20.0),
        };
        let action = |x| item(Role::Button, "Remove", at(x, 4.0, 70.0, 24.0), None);
        assert_eq!(
            far_actions(&[row], &[action(1500.0)], std::slice::from_ref(&words)).len(),
            1
        );
        assert!(far_actions(&[row], &[action(70.0 + FAR_GAP)], &[words]).is_empty());
    }

    #[test]
    fn g4_finds_a_sentence_twice_in_a_pane_but_not_in_rows() {
        let text = |name: &str, y| item(Role::Label, name, at(0.0, y, 300.0, 20.0), None);
        let cause = "The file has a table row with too few cells.";
        let items = [
            text(cause, 10.0),
            text("The file has a table", 200.0),
            text(cause, 300.0),
        ];
        assert_eq!(repeated_sentences(&items, &[]).len(), 3);
        let rows = [at(0.0, 290.0, 400.0, 40.0)];
        assert_eq!(repeated_sentences(&items, &rows).len(), 1);
        assert!(
            repeated_sentences(&[text(cause, 10.0), text("The file has tables", 50.0)], &[])
                .is_empty()
        );
    }
}
