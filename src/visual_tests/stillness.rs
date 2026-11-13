//! Gate invariants that compare what the app shows:
//!
//! - **G1a** a message came or went in a pane, and content below its top in that pane moved.
//! - **G1b** the control a script pressed moved after the press.
//! - **G1c** a scroll bar came or went in a pane, and a right edge in that pane moved.
//! - **G2** a header action at the widest size is gone, or paints other words, at a narrower size.
//! - **G3** at 1920 px, a row's first action at the right is far from the row's last words.
//! - **G4** one pane shows the same sentence twice.
//! - **G6** a pane's message footer sits more than `S` below a body that ends short of it.
//! - **N2** an icon control's centre is off the centre of the text line beside it.
//! - **N4** a quiet action that paints first on its line does not start its words at the line's
//!   text edge, or one that is not first hangs over what comes before it.
//! - **N5** a text that can wrap starts with an icon inside its own layout job.
//! - **N8** one word is shown twice in a row, in one text or two adjacent texts of a line.
//!
//! G1a, G1b, G1c and G3 block: a finding that `stillness_known::KNOWN` does not list fails the
//! run, and so does a listed state with fewer findings (the list may only shrink). So do G6, G7
//! (two live widgets with one id, package K's `layout::widget_id_clashes`), N2 and N5. G2, G4, N8
//! and G1b's renamed tier (`G1B_RENAMED`) are report-only.
//!
//! G1 compares two states in a row at one window size. It matches a control by its pane, role,
//! accessible name and place among equal names, never by AccessKit id alone: egui's automatic ids
//! shift when a widget is inserted before them. `Driver::expect_moves` is the one escape: it
//! marks the next state's G1a and G1b findings as intended, with a reason the report counts.
//!
//! Messages: a typed message is read from package M's registry (`outcome::message_rects`), which
//! gives its whole block and its place. A message drawn in a typed row, banner or page place may
//! move what follows it (the place is the rule), so it does not set G1a off; a typed footer
//! message does. A message that an old, untyped function draws registers nothing, so it is still
//! found in the painted shapes by what `design::message` and `design::small_message` draw: a 3 px
//! bar in a result tone, a tone icon after it, then the words. That search goes when Wave 2 has
//! moved every caller to the typed places.
use super::ergonomics::Control;
use crate::design::outcome::MessagePlace;
use crate::design::{L, S, XS};
use eframe::egui::{self, accesskit::Role, epaint::ClippedShape};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Duration;

pub const G1A: &str = "G1a/message-moved-content";
pub const G1B: &str = "G1b/pressed-control-moved";
/// A pressed control that was found again only by its id, because its name changed: the id may
/// now be another control's. Report-only.
pub const G1B_RENAMED: &str = "G1b/renamed-control-moved";
pub const G1C: &str = "G1c/content-moved-with-bar";
pub const G2: &str = "G2/header-action-lost";
pub const G3: &str = "G3/far-action";
pub const G4: &str = "G4/same-sentence-twice";
pub const G6: &str = "G6/footer-far-below-body";
pub const N2: &str = "N2/icon-off-text-centre";
pub const N5: &str = "N5/icon-starts-wrapping-text";
pub const N8: &str = "N8/doubled-word";
pub const N4: &str = "N4/quiet-action-line-start";
/// Package K's check: two live widgets with one id (two text fields with one `FieldId`).
pub const G7: &str = "G7/widget-id-clash";
pub const CHECKS: [&str; 13] = [
    G1A,
    G1B,
    G1B_RENAMED,
    G1C,
    G2,
    G3,
    G4,
    G6,
    G7,
    N2,
    N4,
    N5,
    N8,
];
/// The checks whose findings must be in `stillness_known::KNOWN`.
pub const BLOCKING: [&str; 8] = [G1A, G1B, G1C, G3, G6, G7, N2, N5];

/// An edge may move by rounding up to this much (the design's ±1 px).
const MOVE_TOLERANCE: f32 = 1.0;
/// The one exemption from `MOVE_TOLERANCE`: a pressed toggle that turns selected paints its word
/// heavier and grows by up to this much, with its line and one side edge kept and the press point
/// still on it.
pub const TOGGLE_WEIGHT: f32 = 3.0;
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
/// G3: a row with this many words on one line, each more than `L` apart, is a table row.
const TABLE_COLUMNS: usize = 3;
/// G4 compares texts of at least this many words.
const G4_MIN_WORDS: usize = 4;
/// G1a with nothing above the message in its pane: the share of the content below that must be
/// in the other state too, for the pane to show the same thing.
const SAME_VIEW: f32 = 0.8;

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

    /// A field the DM types or chooses in: what a form holds.
    fn is_field(&self) -> bool {
        matches!(
            self.role,
            Role::TextInput
                | Role::MultilineTextInput
                | Role::PasswordInput
                | Role::ComboBox
                | Role::SpinButton
                | Role::Slider
        )
    }
}

/// A drawn message: its words, the rect from its bar to its last word, and its pane.
#[derive(Clone, Debug)]
pub struct Message {
    pub words: String,
    pub rect: egui::Rect,
    pub scope: Option<u64>,
    pub clip: Option<egui::Rect>,
    /// The typed place that drew it (package M's registry); `None` for an old, untyped message.
    pub typed: Option<MessagePlace>,
}

impl Message {
    /// A typed row, banner or page message may move what follows it: its place is the rule.
    fn moves_by_place(&self) -> bool {
        matches!(
            self.typed,
            Some(MessagePlace::Row | MessagePlace::Banner | MessagePlace::Page)
        )
    }
}

/// A pane: its top-level scope and the left, top and right of its scroll area. The bottom is left
/// out: a footer that comes shortens the scroll area.
type PaneKey = (Option<u64>, Option<[i32; 3]>);

fn pane_key(scope: Option<u64>, clip: Option<egui::Rect>) -> PaneKey {
    (
        scope,
        clip.map(|clip| {
            [
                clip.left().round() as i32,
                clip.top().round() as i32,
                clip.right().round() as i32,
            ]
        }),
    )
}

impl Message {
    fn key(&self) -> (PaneKey, &str) {
        (pane_key(self.scope, self.clip), &self.words)
    }
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
    /// The panes' message footers (`design::footer_bands`).
    pub footers: Vec<egui::Rect>,
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
            typed: None,
        });
    }
    out
}

/// The typed messages of package M's registry, each with its words (the painted texts inside its
/// rect, in reading order) and its pane (a label inside it), then the untyped messages that the
/// shape search found outside every typed one.
pub fn with_typed(
    registry: &[(egui::Rect, MessagePlace)],
    texts: &[Text],
    items: &[Item],
    untyped: Vec<Message>,
) -> Vec<Message> {
    let mut out: Vec<Message> = registry
        .iter()
        .map(|(rect, place)| {
            let mut inside: Vec<&Text> = texts
                .iter()
                .filter(|text| rect.expand(MOVE_TOLERANCE).contains(text.rect.center()))
                .collect();
            inside.sort_by(|a, b| {
                a.rect
                    .top()
                    .total_cmp(&b.rect.top())
                    .then(a.rect.left().total_cmp(&b.rect.left()))
            });
            let label = items
                .iter()
                .find(|item| item.is_label() && rect.contains(item.rect.center()));
            Message {
                words: inside
                    .iter()
                    .map(|text| text.words.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
                rect: *rect,
                scope: label.map(|item| item.scope),
                clip: label.and_then(|item| item.clip),
                typed: Some(*place),
            }
        })
        .collect();
    out.extend(untyped.into_iter().filter(|message| {
        !registry
            .iter()
            .any(|(rect, _)| rect.expand(MOVE_TOLERANCE).contains(message.rect.center()))
    }));
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

/// The keys of a message's own block: what lies in its footer, or, in a body, its own line and the
/// lines under it that start right of its bar (its second lines and its choices, as
/// `message_choice_row` places them), down to the first line that starts at or left of the bar.
fn own_block<'a>(
    message: &Message,
    pane: &[(Key<'a>, &'a Item)],
    footers: &[egui::Rect],
) -> HashSet<Key<'a>> {
    // A typed message's rect is its whole block.
    let mut out: HashSet<Key<'a>> = pane
        .iter()
        .filter(|(_, item)| message.rect.contains(item.rect.center()))
        .map(|(key, _)| *key)
        .collect();
    if let Some(footer) = footers
        .iter()
        .find(|footer| footer.expand(MOVE_TOLERANCE).contains_rect(message.rect))
    {
        for (key, item) in pane {
            if footer.expand(MOVE_TOLERANCE).contains(item.rect.center()) {
                out.insert(*key);
            }
        }
        return out;
    }
    let mut below: Vec<&(Key<'a>, &'a Item)> = pane
        .iter()
        .filter(|(_, item)| item.rect.top() >= message.rect.top() - MOVE_TOLERANCE)
        .collect();
    below.sort_by(|a, b| {
        a.1.rect
            .top()
            .total_cmp(&b.1.rect.top())
            .then(a.1.rect.left().total_cmp(&b.1.rect.left()))
    });
    let mut bottom = message.rect.bottom();
    for (key, item) in below {
        let on_its_line = item.rect.top() <= message.rect.bottom() + MOVE_TOLERANCE;
        let under_it = item.rect.top() <= bottom + L && item.rect.left() > message.rect.left() + S;
        if !on_its_line && !under_it {
            break;
        }
        out.insert(*key);
        bottom = bottom.max(item.rect.bottom());
    }
    out
}

/// G1a: a message came or went in a pane between `old` and `new` (a message is its pane and its
/// words); every item below its top in that pane keeps its rect. Left out: the message's own
/// block, a pane whose content above the message changed or moved (another view, or a scroll), a
/// pane where a form field came or went (a form opened in place), and a scroll area that changed.
pub fn message_moves(old: &Snapshot, new: &Snapshot) -> Vec<Raw> {
    let keys = |snapshot: &Snapshot| -> HashSet<(PaneKey, String)> {
        snapshot
            .messages
            .iter()
            .map(|message| (message.key().0, message.words.clone()))
            .collect()
    };
    let (old_keys, new_keys) = (keys(old), keys(new));
    let mut changed = Vec::new();
    for message in new.messages.iter().filter(|m| !m.moves_by_place()) {
        if !old_keys.contains(&(message.key().0, message.words.clone())) {
            changed.push((message, new, old, "appeared"));
        }
    }
    for message in old.messages.iter().filter(|m| !m.moves_by_place()) {
        if !new_keys.contains(&(message.key().0, message.words.clone())) {
            changed.push((message, old, new, "went"));
        }
    }
    let mut reported = HashSet::new();
    let mut out = Vec::new();
    for (message, with, without, verb) in changed {
        let other = keyed(&without.items);
        let in_pane = |item: &Item| {
            let same_pane = match message.scope {
                Some(scope) => item.scope == scope && same_clip(item.clip, message.clip),
                None => true,
            };
            same_pane
                && (message.clip.is_some()
                    || item.rect.right() > message.rect.left() - MOVE_TOLERANCE)
        };
        let pane: Vec<(Key<'_>, &Item)> = keyed(&with.items)
            .into_iter()
            .filter(|(_, item)| in_pane(item))
            .collect();
        // A form that opened or closed in place moves what follows it on purpose.
        let fields_changed = pane
            .iter()
            .any(|(key, item)| item.is_field() && !other.contains_key(key))
            || keyed(&without.items).iter().any(|(key, item)| {
                item.is_field() && in_pane(item) && !pane.iter().any(|(mine, _)| mine == key)
            });
        if fields_changed {
            continue;
        }
        let own = own_block(message, &pane, &with.footers);
        let above: Vec<&(Key<'_>, &Item)> = pane
            .iter()
            .filter(|(key, item)| {
                !own.contains(key) && item.rect.bottom() <= message.rect.top() + MOVE_TOLERANCE
            })
            .collect();
        let below: Vec<&(Key<'_>, &Item)> = pane
            .iter()
            .filter(|(key, item)| {
                !own.contains(key)
                    && item.rect.top() >= message.rect.top() - MOVE_TOLERANCE
                    && !with
                        .footers
                        .iter()
                        .any(|footer| footer.contains(item.rect.center()))
            })
            .collect();
        if above.is_empty() {
            // The message is at the top of its pane: the pane must show the same content.
            let kept = below
                .iter()
                .filter(|(key, _)| other.contains_key(key))
                .count();
            if below.is_empty() || (kept as f32) < SAME_VIEW * below.len() as f32 {
                continue;
            }
        } else if !above.iter().all(|(key, item)| {
            other
                .get(key)
                .is_some_and(|there| moved(item.rect, there.rect).is_none())
        }) {
            // Content above changed or moved: another view, or the pane scrolled.
            continue;
        }
        for (key, item) in below {
            if reported.contains(key) {
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

/// G1c: a vertical scroll bar came or went; no right edge in its pane moved. A bar that moved
/// sideways by at most its lane is the same bar in a pane that changed width.
pub fn bar_moves(old: &Snapshot, new: &Snapshot) -> Vec<Raw> {
    let same_bar = |a: &egui::Rect, b: &egui::Rect| {
        (a.left() - b.left()).abs() <= BAR_LANE && a.y_range().intersects(b.y_range())
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

/// A control as a press sees it.
#[derive(Clone, Debug)]
pub struct PressNode {
    pub id: u64,
    pub role: Role,
    pub name: String,
    /// The whole control.
    pub rect: egui::Rect,
    /// The part a person can see.
    pub shown: egui::Rect,
    /// Its top-level pane.
    pub scope: u64,
    /// Drawn in a popup (a menu): a menu entry is not the button that opened the menu.
    pub popup: bool,
    /// Its selected or toggled state, when it is a toggle (AccessKit's selected or toggled).
    pub selected: Option<bool>,
}

/// The control a press goes to, found again after the press.
#[derive(Clone, Debug)]
pub struct Pressed {
    pub node: PressNode,
    /// Its place among controls with its role, name, pane and popup.
    pub place: usize,
    /// Where the pointer pressed.
    pub point: egui::Pos2,
    pub page: String,
}

fn same_identity(a: &PressNode, b: &PressNode) -> bool {
    a.role == b.role && a.name == b.name && a.scope == b.scope && a.popup == b.popup
}

/// The control a press at `point` goes to: the smallest visible one under it, a popup's first.
/// Nothing for a link: it opens something else, so what was under the pointer is new.
pub fn pressed(nodes: &[PressNode], point: egui::Pos2, page: &str) -> Option<Pressed> {
    let index = nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node.shown.contains(point))
        .min_by(|(_, a), (_, b)| {
            b.popup
                .cmp(&a.popup)
                .then(a.shown.area().total_cmp(&b.shown.area()))
        })
        .map(|(index, _)| index)?;
    let node = nodes[index].clone();
    if node.role == Role::Link {
        return None;
    }
    let place = nodes[..index]
        .iter()
        .filter(|other| same_identity(other, &node))
        .count();
    Some(Pressed {
        node,
        place,
        point,
        page: page.to_owned(),
    })
}

/// The selected-toggle exemption: the control is a toggle whose selection the press changed (its
/// selected state is known before and after, and differs), its line and one side edge stay, the
/// width changes by at most `TOGGLE_WEIGHT`, and the press point is still on it. An ordinary
/// button that grows is a move.
fn toggle_weight(before: &PressNode, after: &PressNode, point: egui::Pos2) -> bool {
    let (old, new) = (before.rect, after.rect);
    matches!((before.selected, after.selected), (Some(a), Some(b)) if a != b)
        && (old.top() - new.top()).abs() <= MOVE_TOLERANCE
        && (old.bottom() - new.bottom()).abs() <= MOVE_TOLERANCE
        && ((old.left() - new.left()).abs() <= MOVE_TOLERANCE
            || (old.right() - new.right()).abs() <= MOVE_TOLERANCE)
        && (old.width() - new.width()).abs() <= TOGGLE_WEIGHT
        && new.contains(point)
}

/// G1b after the press: the pressed control, found by role, name, pane, popup and place (by id
/// when its name changed), keeps its rect. `G1B` for a control found by name, `G1B_RENAMED` for
/// one found by id. The trigger says when the press point is no longer on it. Nothing when the
/// page changed or the control is gone.
pub fn press_moves(
    before: &Pressed,
    after: &[PressNode],
    page: &str,
    caption: &str,
) -> Option<Raw> {
    if before.page != page {
        return None;
    }
    let equal: Vec<&PressNode> = after
        .iter()
        .filter(|node| same_identity(node, &before.node))
        .collect();
    let (now, by_name) = if let Some(node) = equal.get(before.place) {
        (*node, true)
    } else if equal.is_empty() {
        let node = after
            .iter()
            .find(|node| node.id == before.node.id && node.role == before.node.role)?;
        (node, false)
    } else {
        return None;
    };
    let (old, rect) = (before.node.rect, now.rect);
    moved(old, rect)?;
    if toggle_weight(&before.node, now, before.point) {
        return None;
    }
    Some(Raw {
        check: if by_name { G1B } else { G1B_RENAMED },
        control: format!("{:?} {:?}", before.node.role, before.node.name),
        old,
        new: rect,
        trigger: format!(
            "press: {caption}{}{}",
            if rect.contains(before.point) {
                ""
            } else {
                "; the press point is off it"
            },
            if by_name {
                ""
            } else {
                " (found by id; its name changed)"
            }
        ),
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
/// window's top that paint at most `ACTION_WORDS` words, with the words they paint. A control that
/// paints only an icon is kept: at a narrower size that is the fault G2 looks for.
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
            (count <= ACTION_WORDS).then(|| (item.clone(), words))
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
/// at most `FAR_GAP`. Two kinds of row are exempt by name: a form row (it holds a field: a label
/// column, then the field) and a table row (`TABLE_COLUMNS` or more words on one line, each more
/// than `L` apart: columns line up across rows on purpose).
pub fn far_actions(rows: &[egui::Rect], items: &[Item], texts: &[Text]) -> Vec<Raw> {
    let mut out = Vec::new();
    for row in rows {
        let inside = |rect: egui::Rect| row.expand(MOVE_TOLERANCE).contains_rect(rect);
        let controls: Vec<&Item> = items
            .iter()
            .filter(|item| !item.is_label() && inside(item.rect))
            .collect();
        if controls.iter().any(|control| control.is_field()) {
            continue;
        }
        let words: Vec<&Text> = texts
            .iter()
            .filter(|text| {
                inside(text.rect)
                    && !controls
                        .iter()
                        .any(|control| control.rect.expand(1.0).contains_rect(text.rect))
            })
            .collect();
        if is_table_row(&words) {
            continue;
        }
        let Some(last) = words
            .iter()
            .max_by(|a, b| a.rect.right().total_cmp(&b.rect.right()))
        else {
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

fn is_table_row(words: &[&Text]) -> bool {
    words.iter().any(|first| {
        let mut line: Vec<&&Text> = words
            .iter()
            .filter(|text| text.rect.y_range().intersects(first.rect.y_range()))
            .collect();
        line.sort_by(|a, b| a.rect.left().total_cmp(&b.rect.left()));
        let apart = line
            .windows(2)
            .filter(|pair| pair[1].rect.left() - pair[0].rect.right() > L)
            .count();
        apart + 1 >= TABLE_COLUMNS
    })
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

/// G6: a footer follows a short body. A pane that keeps package P's record (`Footer::pane`) is
/// checked exactly by `pane::short_body_gap` in the driver; this is the check for a footer band
/// (`design::footer_bands`: from `S` above the footer to the pane's bottom) that no record covers,
/// a pane still placed by hand (`show_by_hand`). There, the last body item above the band (its
/// visible bottom) ends at most `2 × L` above the footer, as in P's check (row padding and a frame
/// under the last item end there). A body that fills the pane (its last item runs past its scroll
/// area) passes. `covered` are the footers of the panes with a record.
pub fn footer_gaps(items: &[Item], footers: &[egui::Rect], covered: &[egui::Rect]) -> Vec<Raw> {
    let mut out = Vec::new();
    for band in footers {
        if covered.iter().any(|footer| {
            (footer.top() - S - band.top()).abs() <= 2.0
                && footer.x_range().intersects(band.x_range())
        }) {
            continue;
        }
        let last = items
            .iter()
            .filter(|item| {
                band.x_range().contains(item.rect.center().x)
                    && item.rect.top() < band.top() - MOVE_TOLERANCE
            })
            .map(|item| {
                let bottom = item.clip.map_or(item.rect.bottom(), |clip| {
                    item.rect.bottom().min(clip.bottom())
                });
                (item, bottom)
            })
            .max_by(|a, b| a.1.total_cmp(&b.1));
        let Some((item, bottom)) = last else {
            continue;
        };
        if item.rect.bottom() > bottom + MOVE_TOLERANCE {
            continue;
        }
        let gap = band.top() + S - bottom;
        if gap > 2.0 * L {
            out.push(Raw {
                check: G6,
                control: display(item),
                old: item.rect,
                new: *band,
                trigger: format!(
                    "the footer is {gap:.0} px below the body's last item (a pane placed by hand)"
                ),
            });
        }
    }
    out
}

/// G6 for a pane with package P's record: the exact gap that `pane::short_body_gap` found.
pub fn exact_footer_gap(pane: &str, footer: egui::Rect, gap: f32) -> Raw {
    Raw {
        check: G6,
        control: format!("Pane {pane}"),
        old: egui::Rect::from_min_size(
            footer.min - egui::vec2(0.0, gap),
            egui::vec2(footer.width(), 0.0),
        ),
        new: footer,
        trigger: format!("the footer is {gap:.0} px below the body (Footer::pane)"),
    }
}

/// G7: package K's `layout::widget_id_clashes` words for one state.
pub fn id_clash(words: String) -> Raw {
    Raw {
        check: G7,
        control: words,
        old: egui::Rect::NOTHING,
        new: egui::Rect::NOTHING,
        trigger: "two live widgets share one id: give each its own FieldId key".to_owned(),
    }
}

/// A painted text as N2, N5 and N8 read it.
#[derive(Clone, Debug)]
pub struct Line {
    pub words: String,
    /// The visible part of the whole text.
    pub rect: egui::Rect,
    /// Its first row, on screen.
    pub first: egui::Rect,
    pub rows: usize,
    /// Its layout job wraps at a width.
    pub wraps: bool,
}

/// A private-use character: one of the app's icon glyphs.
fn is_icon(c: char) -> bool {
    ('\u{e000}'..='\u{f8ff}').contains(&c)
}

fn icon_only(words: &str) -> bool {
    let words = words.trim();
    !words.is_empty() && words.chars().all(|c| is_icon(c) || c.is_whitespace())
}

pub fn lines(painted: &[(egui::epaint::TextShape, egui::Rect)]) -> Vec<Line> {
    painted
        .iter()
        .map(|(text, clip)| {
            let galley = &text.galley;
            let first = galley
                .rows
                .first()
                .map_or(galley.rect, |row| row.rect())
                .translate(text.pos.to_vec2());
            Line {
                words: galley.job.text.clone(),
                rect: text.visual_bounding_rect().intersect(*clip),
                first,
                rows: galley.rows.len(),
                wraps: galley.job.wrap.max_width.is_finite(),
            }
        })
        .collect()
}

/// N2: an icon control (a control that paints only icons) sits on the line of the text beside
/// it: a text within `3 × L` to its left or right whose first row holds the control's centre;
/// of those, the row whose centre is nearest the control's. A control more than twice as tall as
/// that row is a tile and is left out. The control's centre is within 1 px
/// of that row's centre. Text inside a text field counts as the field, whose box is the line;
/// text in a multi-line editor is left out.
pub fn icon_centres(items: &[Item], lines: &[Line]) -> Vec<Raw> {
    let fields: Vec<&Item> = items
        .iter()
        .filter(|item| matches!(item.role, Role::TextInput | Role::MultilineTextInput))
        .collect();
    let mut out = Vec::new();
    for item in items.iter().filter(|item| !item.is_label()) {
        let inside = |line: &Line| item.rect.expand(1.0).contains(line.rect.center());
        let own: Vec<&Line> = lines.iter().filter(|line| inside(line)).collect();
        if own.is_empty() || !own.iter().all(|line| icon_only(&line.words)) {
            continue;
        }
        let centre = item.rect.center();
        let beside = lines
            .iter()
            .filter(|line| !inside(line) && !icon_only(&line.words))
            // The file in a Markdown editor is not a line of the app's layout.
            .filter(|line| {
                !items.iter().any(|editor| {
                    editor.role == Role::MultilineTextInput
                        && editor.rect.contains(line.first.center())
                })
            })
            .map(|line| {
                let field = fields
                    .iter()
                    .find(|field| field.rect.contains(line.first.center()))
                    .map(|field| field.rect);
                (line, field.unwrap_or(line.first))
            })
            // The row must hold the control's centre: a long text's first row elsewhere is not its
            // line.
            .filter(|(_, row)| row.y_range().contains(centre.y))
            // A control twice as tall as the line is a tile, not an icon on the line.
            .filter(|(_, row)| item.rect.height() <= 2.0 * row.height())
            .filter(|(_, row)| {
                let gap = if row.right() <= item.rect.left() {
                    item.rect.left() - row.right()
                } else if row.left() >= item.rect.right() {
                    row.left() - item.rect.right()
                } else {
                    f32::INFINITY
                };
                gap <= 3.0 * L
            })
            .min_by(|a, b| {
                (a.1.center().y - centre.y)
                    .abs()
                    .total_cmp(&(b.1.center().y - centre.y).abs())
            });
        let Some((line, row)) = beside else {
            continue;
        };
        let off = centre.y - row.center().y;
        if off.abs() > MOVE_TOLERANCE {
            out.push(Raw {
                check: N2,
                control: display(item),
                old: row,
                new: item.rect,
                trigger: format!(
                    "{:.0} px {} the centre of {:?}",
                    off.abs(),
                    if off < 0.0 { "above" } else { "below" },
                    line.words
                ),
            });
        }
    }
    out
}

/// N5: a text that starts with an icon wrapped inside its own layout job: it has more rows than
/// written lines, so a second row starts under the icon, not under the words
/// (`design::wrapped_status` keeps the icon outside the job). A text that has not wrapped yet is
/// not found: nearly every egui label can wrap.
pub fn icon_wraps(lines: &[Line]) -> Vec<Raw> {
    lines
        .iter()
        .filter(|line| {
            line.wraps
                && line.rows > line.words.lines().count().max(1)
                && line.words.trim_start().chars().next().is_some_and(is_icon)
                && !icon_only(&line.words)
        })
        .map(|line| Raw {
            check: N5,
            control: format!("Text {:?}", line.words),
            old: line.first,
            new: line.rect,
            trigger: format!(
                "wrapped onto {} rows from {} written lines",
                line.rows,
                line.words.lines().count().max(1)
            ),
        })
        .collect()
}

/// A painted text as N4 reads it: where its layout starts, its rect, and the clip it is painted in.
#[derive(Clone, Debug)]
pub struct Placed {
    pub words: String,
    /// The x of its first word: its first row's first glyph that is not an icon or a space. Text
    /// lines up on this edge; an icon in front of the words hangs before it.
    pub start: f32,
    pub rect: egui::Rect,
    pub clip: egui::Rect,
}

pub fn placed(painted: &[(egui::epaint::TextShape, egui::Rect)]) -> Vec<Placed> {
    painted
        .iter()
        .map(|(text, clip)| Placed {
            words: text.galley.job.text.clone(),
            start: text
                .galley
                .rows
                .first()
                .and_then(|row| {
                    row.glyphs
                        .iter()
                        .find(|glyph| !is_icon(glyph.chr) && !glyph.chr.is_whitespace())
                        .map(|glyph| text.pos.x + row.pos.x + glyph.pos.x)
                })
                .unwrap_or(text.pos.x + text.galley.rect.min.x),
            rect: text.visual_bounding_rect(),
            clip: *clip,
        })
        .collect()
}

/// The rects that a quiet action paints: `design::Action` paints its whole rect with a
/// transparent fill and no stroke when it is quiet (a secondary action strokes a border, a
/// primary one fills).
pub fn quiet_rects(shapes: &[ClippedShape]) -> Vec<egui::Rect> {
    fn walk(shape: &egui::Shape, out: &mut Vec<egui::Rect>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            egui::Shape::Rect(rect)
                if rect.fill == egui::Color32::TRANSPARENT
                    && rect.stroke.width == 0.0
                    && rect.rect.is_positive() =>
            {
                out.push(rect.rect);
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

/// N4. A quiet action is a button whose rect is one of `quiet` (see `quiet_rects`). Its words
/// start where its first non-icon text starts. It paints first on its line when no text of its
/// clip, outside it, lies on its line to its left. Then its words start, within 1 px, at the text
/// edge: the start of the nearest line above or of the nearest line below it in its clip (within
/// `2 × L`, and within `3 × L` sideways); it is a finding when they match neither. A quiet action that is not first does not reach left over the text before it
/// (`line_start` hangs its padding into the gutter; mid-line it covers its neighbour).
pub fn line_starts(items: &[Item], quiet: &[egui::Rect], texts: &[Placed]) -> Vec<Raw> {
    let mut out = Vec::new();
    for item in items.iter().filter(|item| item.role == Role::Button) {
        if !quiet.iter().any(|rect| moved(*rect, item.rect).is_none()) {
            continue;
        }
        let own: Vec<&Placed> = texts
            .iter()
            .filter(|text| item.rect.expand(1.0).contains(text.rect.center()))
            .filter(|text| !icon_only(&text.words))
            .collect();
        let Some(words) = own.iter().min_by(|a, b| a.start.total_cmp(&b.start)) else {
            continue;
        };
        let clip = words.clip;
        // On its line: the heights overlap by at least half the smaller one (a tall title beside
        // a button is on the button's line).
        let on_line = |rect: egui::Rect| {
            let overlap = rect.bottom().min(item.rect.bottom()) - rect.top().max(item.rect.top());
            overlap >= 0.5 * rect.height().min(item.rect.height())
        };
        let outside = |text: &&Placed| {
            text.clip == clip && !item.rect.expand(1.0).contains(text.rect.center())
        };
        let before: Vec<&Placed> = texts
            .iter()
            .filter(outside)
            .filter(|text| on_line(text.rect) && text.rect.right() <= words.start)
            .collect();
        if let Some(neighbour) = before
            .iter()
            .max_by(|a, b| a.rect.right().total_cmp(&b.rect.right()))
        {
            if item.rect.left() < neighbour.rect.right() - MOVE_TOLERANCE {
                out.push(Raw {
                    check: N4,
                    control: display(item),
                    old: neighbour.rect,
                    new: item.rect,
                    trigger: format!(
                        "not first on its line, but reaches {:.0} px over {:?}",
                        neighbour.rect.right() - item.rect.left(),
                        neighbour.words
                    ),
                });
            }
            continue;
        }
        // The nearest line above and the nearest below, in its clip, within `2 × L`.
        let near = |above: bool| {
            texts
                .iter()
                .filter(outside)
                .filter(|text| !icon_only(&text.words))
                .filter(|text| (text.start - words.start).abs() <= 3.0 * L)
                .filter(|text| !on_line(text.rect))
                .filter_map(|text| {
                    let gap = if above {
                        item.rect.top() - text.rect.bottom()
                    } else {
                        text.rect.top() - item.rect.bottom()
                    };
                    (0.0..=2.0 * L).contains(&gap).then_some((text, gap))
                })
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(text, _)| text)
        };
        let edges: Vec<&Placed> = [near(true), near(false)].into_iter().flatten().collect();
        // The words line up with either neighbour: that is the block's text edge.
        if edges.is_empty()
            || edges
                .iter()
                .any(|edge| (words.start - edge.start).abs() <= MOVE_TOLERANCE)
        {
            continue;
        }
        let edge = edges[0];
        let off = words.start - edge.start;
        out.push(Raw {
            check: N4,
            control: display(item),
            old: edge.rect,
            new: item.rect,
            trigger: format!(
                "first on its line: its words start {:.0} px {} the text edge of {:?}",
                off.abs(),
                if off > 0.0 { "right of" } else { "left of" },
                edge.words
            ),
        });
    }
    out
}

/// A word for N8: lower case and letters only, so a number or a dice term (`2d6`) is not a word.
fn word(raw: &str) -> Option<String> {
    let word: String = raw
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    (word.chars().count() > 1 && word.chars().all(char::is_alphabetic)).then_some(word)
}

fn is_number(raw: &str) -> bool {
    let core: String = raw.chars().filter(|c| c.is_alphanumeric()).collect();
    !core.is_empty() && core.chars().all(|c| c.is_ascii_digit())
}

/// N8: one word twice in a row: in one text (`Travel Travel`, or across a number: `Day 1: Day`),
/// or as the last word of a text and the first word of the text just after it on its line. Text in
/// a multi-line field is the file the DM writes, not the app's words, and is left out.
pub fn doubled_words(items: &[Item], lines: &[Line]) -> Vec<Raw> {
    let editors: Vec<egui::Rect> = items
        .iter()
        .filter(|item| item.role == Role::MultilineTextInput)
        .map(|item| item.rect)
        .collect();
    let shown: Vec<&Line> = lines
        .iter()
        .filter(|line| {
            !editors
                .iter()
                .any(|editor| editor.contains(line.first.center()))
        })
        .collect();
    let lines = shown;
    let mut out = Vec::new();
    for line in &lines {
        let raw: Vec<&str> = line.words.split_whitespace().collect();
        for (index, current) in raw.iter().enumerate() {
            let Some(first) = word(current) else {
                continue;
            };
            let next = raw.get(index + 1).and_then(|next| word(next));
            let after_number = raw
                .get(index + 1)
                .filter(|between| is_number(between))
                .and(raw.get(index + 2))
                .and_then(|next| word(next));
            if next.as_ref() == Some(&first) || after_number.as_ref() == Some(&first) {
                out.push(Raw {
                    check: N8,
                    control: format!("Text {:?}", line.words),
                    old: line.rect,
                    new: line.rect,
                    trigger: format!("{first:?} twice in one text"),
                });
                break;
            }
        }
    }
    for left in &lines {
        let Some(last) = left.words.split_whitespace().last().and_then(word) else {
            continue;
        };
        for right in &lines {
            let same_line = (left.first.center().y - right.first.center().y).abs() <= 2.0;
            let gap = right.first.left() - left.rect.right();
            if same_line
                && (-MOVE_TOLERANCE..=L).contains(&gap)
                && right.words.split_whitespace().next().and_then(word) == Some(last.clone())
            {
                out.push(Raw {
                    check: N8,
                    control: format!("Text {:?}", right.words),
                    old: left.rect,
                    new: right.rect,
                    trigger: format!("{last:?} ends {:?} and starts the next text", left.words),
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
    /// Each press finding with the window it was pressed at.
    pending: Vec<(String, Raw)>,
    escape: Option<&'static str>,
    /// A column edge was dragged or moved by keyboard since the last state.
    pub column_resized: bool,
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
        for (window, raw) in std::mem::take(&mut self.pending) {
            self.push(scenario, state, &window, raw);
        }
    }

    pub fn press(&mut self, window: String, raw: Raw) {
        self.pending.push((window, raw));
    }

    pub fn push(&mut self, scenario: &str, state: &str, window: &str, raw: Raw) {
        // A reorder or a sort explains a moved message or press, never a bar's lane.
        let escapable = matches!(raw.check, G1A | G1B | G1B_RENAMED);
        self.findings.push(Finding {
            check: raw.check,
            scenario: scenario.to_owned(),
            state: state.to_owned(),
            window: window.to_owned(),
            control: raw.control,
            old: xywh(raw.old),
            new: xywh(raw.new),
            trigger: raw.trigger,
            escaped: if escapable { self.escape } else { None },
        });
    }

    /// Compare with the previous state at this size, when it is the state just before.
    pub fn observe(&mut self, size: (f32, f32), snapshot: Snapshot) -> Vec<Raw> {
        let key = (size.0 as u32, size.1 as u32);
        let mut out = Vec::new();
        if let Some(old) = self.last.get(&key)
            && old.state + 1 == snapshot.state
            && old.page == snapshot.page
            && !self.column_resized
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
        self.column_resized = false;
        self.escape = None;
    }

    /// Presses after the last state.
    pub fn finish(&mut self, scenario: &str) {
        for (window, raw) in std::mem::take(&mut self.pending) {
            self.push(scenario, "(after the last state)", &window, raw);
        }
    }
}

/// A known line's key: a blocking check, a scenario, a state, a window, and the name of the control
/// or text it found. Counting per name means a fix in one control cannot hide a new fault in
/// another of the same state.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KnownKey {
    pub scenario: String,
    pub check: String,
    pub state: String,
    pub window: String,
    pub name: String,
}

impl std::fmt::Display for KnownKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} / {} / {} / {} / {}",
            self.check, self.scenario, self.state, self.window, self.name
        )
    }
}

/// The known lines: each key with its exact count of findings.
pub type Known = BTreeMap<KnownKey, usize>;

/// `stillness_known.json` and `stillness-known.json`: per scenario, its lines as
/// `[check, state, window, name, count]`.
type KnownFile = BTreeMap<String, Vec<(String, String, String, String, usize)>>;

pub fn parse_known(text: &str) -> Result<Known, String> {
    let file: KnownFile = serde_json::from_str(text).map_err(|error| error.to_string())?;
    let mut out = Known::new();
    for (scenario, lines) in file {
        for (check, state, window, name, count) in lines {
            if !BLOCKING.contains(&check.as_str()) {
                return Err(format!("{check} is not a blocking check"));
            }
            let key = KnownKey {
                scenario: scenario.clone(),
                check,
                state,
                window,
                name,
            };
            if out.insert(key.clone(), count).is_some() {
                return Err(format!("{key} is listed twice"));
            }
        }
    }
    Ok(out)
}

/// The list in this source.
pub fn known() -> Known {
    parse_known(include_str!("stillness_known.json"))
        .unwrap_or_else(|error| panic!("src/visual_tests/stillness_known.json: {error}"))
}

/// The list as its file: one scenario per block, one line per key, so a diff shows each change.
pub fn known_file(known: &Known) -> String {
    let mut by_scenario: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for (key, count) in known {
        let line = serde_json::to_string(&(&key.check, &key.state, &key.window, &key.name, count))
            .expect("encode a known line");
        by_scenario.entry(&key.scenario).or_default().push(line);
    }
    let blocks: Vec<String> = by_scenario
        .iter()
        .map(|(scenario, lines)| {
            format!(
                "  {}: [\n    {}\n  ]",
                serde_json::to_string(scenario).expect("encode a scenario"),
                lines.join(",\n    ")
            )
        })
        .collect();
    format!("{{\n{}\n}}\n", blocks.join(",\n"))
}

/// What this run's findings count per key.
pub fn found_lines(findings: &[Finding]) -> Known {
    let mut found = Known::new();
    for finding in findings
        .iter()
        .filter(|f| f.escaped.is_none() && BLOCKING.contains(&f.check))
    {
        *found
            .entry(KnownKey {
                scenario: finding.scenario.clone(),
                check: finding.check.to_owned(),
                state: finding.state.clone(),
                window: finding.window.clone(),
                name: finding.control.clone(),
            })
            .or_default() += 1;
    }
    found
}

/// Lines by key: a key with more findings than `known` lists (or not listed), then a listed key of a
/// scenario that ran with fewer (the list is stale). Both fail a run. Escaped findings do not count.
pub fn compare_known(
    findings: &[Finding],
    known: &Known,
    scenarios: &[&str],
) -> (Vec<String>, Vec<String>) {
    let found = found_lines(findings);
    let listed = |key: &KnownKey| known.get(key).copied().unwrap_or(0);
    let worse = found
        .iter()
        .filter(|(key, count)| **count > listed(key))
        .map(|(key, count)| format!("{key}: {count} (known {})", listed(key)))
        .collect();
    let stale = known
        .iter()
        .filter(|(key, _)| scenarios.contains(&key.scenario.as_str()))
        .filter_map(|(key, limit)| {
            let count = found.get(key).copied().unwrap_or(0);
            (count < *limit).then(|| {
                format!(
                    "{key}: {count} (known {limit}). Fixed: lower or remove its line in \
                     stillness_known.json"
                )
            })
        })
        .collect();
    (worse, stale)
}

/// `stillness-known.json` of a run: the lines its scenarios need, in the list's form. A package
/// that fixed lines replaces its scenarios' blocks in `stillness_known.json` with these; the gate
/// refuses a line that is higher than, or not in, the list of the merge base with `origin/main`.
pub fn known_lines(findings: &[Finding], scenarios: &[&str]) -> String {
    let mut found = found_lines(findings);
    found.retain(|key, _| scenarios.contains(&key.scenario.as_str()));
    known_file(&found)
}

/// The list of the merge base with `origin/main`, as the gate could read it.
pub enum Base {
    List(Known),
    /// The merge base has no list yet: this list is the first.
    Absent(String),
    /// The merge base or its list could not be read.
    Unreadable(String),
}

/// Each `SPLAT_STILLNESS_RAISE` entry, one per line: `<key> => <reason>`.
pub fn raises(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|line| {
            let (key, reason) = line.split_once(" => ")?;
            Some((key.trim().to_owned(), reason.trim().to_owned()))
        })
        .filter(|(key, reason)| !key.is_empty() && !reason.is_empty())
        .collect()
}

/// The list may only shrink: against the merge base's list, a new key or a higher count fails with
/// its key, unless `raises` names that key with a reason. Returns the failures and the notes the
/// report prints (each raise used, the base's state). A base that cannot be read fails only in the
/// landing gate (`landing`), and is a loud note everywhere.
pub fn compare_base(
    current: &Known,
    base: &Base,
    raises: &[(String, String)],
    landing: bool,
) -> (Vec<String>, Vec<String>) {
    let mut failures = Vec::new();
    let mut notes = Vec::new();
    match base {
        Base::Absent(why) => notes.push(format!(
            "FIRST LIST: the merge base has no stillness_known.json ({why}); this list is the first \
             and is not compared"
        )),
        Base::Unreadable(why) => {
            let line = format!("BASE NOT READ: the known lists were not compared with origin/main: {why}");
            if landing {
                failures.push(line);
            } else {
                notes.push(line);
            }
        }
        Base::List(base) => {
            for (key, count) in current {
                let before = base.get(key).copied().unwrap_or(0);
                if *count <= before {
                    continue;
                }
                let name = key.to_string();
                match raises.iter().find(|(raised, _)| raised == name.trim()) {
                    Some((_, reason)) => notes.push(format!(
                        "RAISED (SPLAT_STILLNESS_RAISE): {name}: {before} -> {count}: {reason}"
                    )),
                    None => failures.push(format!(
                        "Known line raised against the merge base: {name}: {before} -> {count}. \
                         Fix it, or name it in SPLAT_STILLNESS_RAISE=\"{name} => <reason>\""
                    )),
                }
            }
        }
    }
    (failures, notes)
}

/// `stillness.json`: a JSON array with one finding per line, so `grep` and `diff` read it.
pub fn json_lines(findings: &[Finding]) -> String {
    let lines: Vec<String> = findings
        .iter()
        .map(|finding| serde_json::to_string(finding).expect("encode a finding"))
        .collect();
    format!("[\n{}\n]\n", lines.join(",\n"))
}

/// The counts the gate prints.
pub struct Report {
    /// Per check: findings, escaped findings, and states with a finding.
    counts: Vec<(&'static str, usize, usize, usize)>,
    /// `expect_moves` uses per scenario.
    escapes: BTreeMap<String, Vec<String>>,
    /// Known lines per blocking check.
    known: Vec<(&'static str, usize)>,
    time: Duration,
}

impl Report {
    pub fn new(
        findings: &[Finding],
        escapes: &[(String, String, &'static str)],
        known: &Known,
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
        let mut by_scenario: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (scenario, state, reason) in escapes {
            by_scenario
                .entry(scenario.clone())
                .or_default()
                .push(format!("{state}: {reason}"));
        }
        let known = BLOCKING
            .iter()
            .map(|check| {
                (
                    *check,
                    known.keys().filter(|key| key.check == *check).count(),
                )
            })
            .collect();
        Self {
            counts,
            escapes: by_scenario,
            known,
            time,
        }
    }

    fn blocks(check: &str) -> bool {
        BLOCKING.contains(&check)
    }

    pub fn summary(&self) -> String {
        let counts: Vec<String> = self
            .counts
            .iter()
            .map(|(check, found, escaped, states)| {
                format!(
                    "{check} {found} in {states} states ({escaped} escaped{})",
                    if Self::blocks(check) {
                        ", blocking"
                    } else {
                        ", report-only"
                    }
                )
            })
            .collect();
        let known: Vec<String> = self
            .known
            .iter()
            .map(|(check, lines)| format!("{check} {lines}"))
            .collect();
        format!(
            "Stillness (stillness.json): {}. Known lines: {}. expect_moves: {} uses. Check time {:?} (elapsed inside the checks, summed over workers).",
            counts.join("; "),
            known.join(", "),
            self.escapes.values().map(Vec::len).sum::<usize>(),
            self.time,
        )
    }

    pub fn markdown(&self) -> String {
        let mut out = String::from(
            "\n## Stillness and other G checks\n\nEach finding is a line of `stillness.json`. A blocking check fails the run on a line that `stillness_known::KNOWN` does not list.\n\n| Check | Findings | States | Escaped by `expect_moves` | Blocks | Known lines |\n|---|---|---|---|---|---|\n",
        );
        for (check, found, escaped, states) in &self.counts {
            let known = self
                .known
                .iter()
                .find(|(c, _)| c == check)
                .map_or_else(|| "—".to_owned(), |(_, n)| n.to_string());
            out.push_str(&format!(
                "| {check} | {found} | {states} | {escaped} | {} | {known} |\n",
                if Self::blocks(check) { "yes" } else { "no" }
            ));
        }
        out.push_str(&format!(
            "\nCheck time: elapsed time inside the checks, summed over the workers: {:?}.\n",
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
            footers: Vec::new(),
        }
    }

    fn saved(rect: egui::Rect, clip: Option<egui::Rect>) -> Message {
        Message {
            words: "Changes saved.".into(),
            rect,
            scope: Some(1),
            clip,
            typed: None,
        }
    }

    #[test]
    fn g1a_finds_content_a_new_message_pushed_down_and_not_content_above_it() {
        let body = Some(at(0.0, 0.0, 600.0, 600.0));
        let message = saved(at(10.0, 100.0, 200.0, 20.0), body);
        let above = item(Role::Label, "File line", at(10.0, 60.0, 200.0, 20.0), body);
        let row = |y| item(Role::Button, "Strength", at(10.0, y, 100.0, 24.0), body);
        let old = snapshot(vec![above.clone(), row(100.0)], vec![], vec![]);
        let new = snapshot(
            vec![above.clone(), row(128.0)],
            vec![message.clone()],
            vec![],
        );
        let found = message_moves(&old, &new);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].control, "Button \"Strength\"");
        assert_eq!(found[0].old.top(), 100.0);
        // A message that went is the same move backwards.
        assert_eq!(message_moves(&new, &old).len(), 1);
        // Content that stays still gives nothing; a message that stays gives nothing.
        let still = snapshot(new.items.clone(), new.messages.clone(), vec![]);
        assert!(message_moves(&new, &still).is_empty());
        let in_place = snapshot(vec![above.clone(), row(128.0)], vec![], vec![]);
        assert!(message_moves(&in_place, &new).is_empty());
        // The tolerance is the design's 1 px: 1 px passes, 1.5 px is a move.
        let below = snapshot(vec![above.clone(), row(140.0)], vec![], vec![]);
        let by = |dy: f32| {
            snapshot(
                vec![above.clone(), row(140.0 + dy)],
                vec![message.clone()],
                vec![],
            )
        };
        assert!(message_moves(&below, &by(1.0)).is_empty());
        assert_eq!(message_moves(&below, &by(1.5)).len(), 1);
        // The message's own second line and choice row are its block, not content it moved.
        let choice = item(Role::Button, "Undo", at(40.0, 124.0, 60.0, 24.0), body);
        let with_choice = snapshot(
            vec![above.clone(), choice.clone(), row(152.0)],
            vec![message.clone()],
            vec![],
        );
        let choice_before = snapshot(
            vec![
                above.clone(),
                item(Role::Button, "Undo", at(40.0, 300.0, 60.0, 24.0), body),
                row(100.0),
            ],
            vec![],
            vec![],
        );
        let found = message_moves(&choice_before, &with_choice);
        assert_eq!(found.len(), 1, "only the row: {found:?}");
        // A form field that came in the pane: a form opened in place.
        let field = item(Role::TextInput, "Name", at(10.0, 160.0, 200.0, 24.0), body);
        let with_form = snapshot(
            vec![above.clone(), row(128.0), field],
            vec![message.clone()],
            vec![],
        );
        assert!(message_moves(&old, &with_form).is_empty());
        // Content above that moved: the pane scrolled, so the message did not move it.
        let scrolled = snapshot(
            vec![
                item(Role::Label, "File line", at(10.0, 40.0, 200.0, 20.0), body),
                row(128.0),
            ],
            vec![message.clone()],
            vec![],
        );
        assert!(message_moves(&old, &scrolled).is_empty());
    }

    #[test]
    fn g1a_lets_a_typed_row_banner_or_page_message_move_what_follows() {
        let body = Some(at(0.0, 0.0, 600.0, 600.0));
        let above = item(Role::Label, "File line", at(10.0, 60.0, 200.0, 20.0), body);
        let row = |y| item(Role::Button, "Strength", at(10.0, y, 100.0, 24.0), body);
        let old = snapshot(vec![above.clone(), row(100.0)], vec![], vec![]);
        for (place, found) in [
            (MessagePlace::Row, 0),
            (MessagePlace::Banner, 0),
            (MessagePlace::Page, 0),
            (MessagePlace::Footer, 1),
        ] {
            let mut message = saved(at(10.0, 100.0, 200.0, 20.0), body);
            message.typed = Some(place);
            let new = snapshot(vec![above.clone(), row(128.0)], vec![message], vec![]);
            assert_eq!(message_moves(&old, &new).len(), found, "{place:?}");
        }
        // The registry gives the words and the block; a shape-found copy inside it is dropped.
        let texts = [Text {
            words: "Changes saved.".into(),
            rect: at(30.0, 102.0, 100.0, 16.0),
        }];
        let block = at(10.0, 100.0, 300.0, 50.0);
        let found = with_typed(
            &[(block, MessagePlace::Footer)],
            &texts,
            &[],
            vec![saved(at(10.0, 100.0, 200.0, 20.0), body)],
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].words, "Changes saved.");
        assert_eq!(found[0].typed, Some(MessagePlace::Footer));
    }

    #[test]
    fn g1a_compares_messages_by_pane_and_words() {
        let a = Some(at(0.0, 0.0, 300.0, 600.0));
        let b = Some(at(320.0, 0.0, 300.0, 600.0));
        let a_message = saved(at(10.0, 100.0, 200.0, 20.0), a);
        let b_message = saved(at(330.0, 100.0, 200.0, 20.0), b);
        let a_title = item(Role::Label, "A title", at(10.0, 60.0, 200.0, 20.0), a);
        let b_title = item(Role::Label, "B title", at(330.0, 60.0, 200.0, 20.0), b);
        let a_row = item(Role::Button, "A row", at(10.0, 128.0, 100.0, 24.0), a);
        let b_row = |y| item(Role::Button, "B row", at(330.0, y, 100.0, 24.0), b);
        // Pane A already says the words; pane B gains the same words and its row moves.
        let old = snapshot(
            vec![
                a_title.clone(),
                a_row.clone(),
                b_title.clone(),
                b_row(100.0),
            ],
            vec![a_message.clone()],
            vec![],
        );
        let new = snapshot(
            vec![a_title, a_row, b_title, b_row(128.0)],
            vec![a_message, b_message],
            vec![],
        );
        let found = message_moves(&old, &new);
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].control, "Button \"B row\"");
    }

    #[test]
    fn g1a_at_the_top_of_a_pane_compares_only_the_same_view() {
        let body = Some(at(0.0, 0.0, 600.0, 600.0));
        let message = saved(at(10.0, 0.0, 200.0, 20.0), body);
        let rows = |names: &[&str], y: f32| -> Vec<Item> {
            names
                .iter()
                .enumerate()
                .map(|(i, name)| {
                    item(
                        Role::Button,
                        name,
                        at(10.0, y + 30.0 * i as f32, 100.0, 24.0),
                        body,
                    )
                })
                .collect()
        };
        let old = snapshot(rows(&["A", "B", "C"], 0.0), vec![], vec![]);
        let new = snapshot(rows(&["A", "B", "C"], 28.0), vec![message.clone()], vec![]);
        assert_eq!(message_moves(&old, &new).len(), 3);
        let other_view = snapshot(rows(&["D", "E", "C"], 28.0), vec![message], vec![]);
        assert!(message_moves(&old, &other_view).is_empty());
    }

    fn node(id: u64, name: &str, rect: egui::Rect, popup: bool) -> PressNode {
        PressNode {
            id,
            role: Role::Button,
            name: name.into(),
            rect,
            shown: rect,
            scope: 1,
            popup,
            selected: None,
        }
    }

    #[test]
    fn g1b_finds_a_pressed_control_that_moved() {
        let button = at(300.0, 50.0, 80.0, 24.0);
        let before = pressed(
            &[node(7, "+ Success", button, false)],
            button.center(),
            "Run",
        )
        .expect("the pressed button");
        let after = |rect| [node(7, "+ Success", rect, false)];
        // Off the press point.
        let found = press_moves(&before, &after(at(390.0, 50.0, 80.0, 24.0)), "Run", "Click")
            .expect("a move");
        assert_eq!(found.check, G1B);
        assert!(found.trigger.contains("the press point is off it"));
        // Moved 12 px, with the point still on it (count words with no fixed width): blocking too.
        let found = press_moves(&before, &after(at(312.0, 50.0, 80.0, 24.0)), "Run", "Click")
            .expect("a shift");
        assert_eq!(found.check, G1B);
        assert!(!found.trigger.contains("off it"));
        assert!(press_moves(&before, &after(button), "Run", "Click").is_none());
        assert!(
            press_moves(
                &before,
                &after(at(390.0, 50.0, 80.0, 24.0)),
                "World",
                "Click"
            )
            .is_none()
        );
        // 1 px is the design's tolerance; 1.5 px is a move.
        assert!(
            press_moves(&before, &after(at(301.0, 50.0, 80.0, 24.0)), "Run", "Click").is_none()
        );
        assert!(
            press_moves(&before, &after(at(301.5, 51.5, 80.0, 24.0)), "Run", "Click").is_some()
        );
        // An ordinary button that only grows 2 px is a move: no selection changed.
        assert_eq!(
            press_moves(&before, &after(at(300.0, 50.0, 82.0, 24.0)), "Run", "Click")
                .map(|raw| raw.check),
            Some(G1B)
        );
        // Renamed: found by id, never blocking.
        let renamed = [node(7, "Stop", at(500.0, 50.0, 80.0, 24.0), false)];
        assert_eq!(
            press_moves(&before, &renamed, "Run", "Click").map(|raw| raw.check),
            Some(G1B_RENAMED)
        );
    }

    #[test]
    fn g1b_exempts_a_selected_toggle_up_to_its_weight_and_no_more() {
        let toggle = |rect, selected| PressNode {
            selected: Some(selected),
            ..node(3, "Edit Markdown", rect, false)
        };
        let off = at(500.0, 70.0, 110.0, 28.0);
        let before = pressed(&[toggle(off, false)], off.center(), "Library").expect("the toggle");
        let grown = |width, selected| [toggle(at(500.0, 70.0, 110.0 + width, 28.0), selected)];
        // The press selected it: it may grow by its weight.
        assert!(press_moves(&before, &grown(TOGGLE_WEIGHT, true), "Library", "Click").is_none());
        assert_eq!(
            press_moves(
                &before,
                &grown(TOGGLE_WEIGHT + 0.5, true),
                "Library",
                "Click"
            )
            .map(|r| r.check),
            Some(G1B)
        );
        // A toggle whose selection did not change, and a control with no selected state, may not.
        assert!(press_moves(&before, &grown(2.0, false), "Library", "Click").is_some());
        let plain = pressed(
            &[node(3, "Edit Markdown", off, false)],
            off.center(),
            "Library",
        )
        .expect("a plain button");
        assert!(press_moves(&plain, &grown(2.0, true), "Library", "Click").is_some());
        // Its line must stay: the same growth one line lower is a move.
        let lower = [toggle(at(500.0, 72.0, 112.0, 28.0), true)];
        assert!(press_moves(&before, &lower, "Library", "Click").is_some());
    }

    #[test]
    fn g1b_tells_a_menu_entry_from_the_button_that_opened_it() {
        let button = at(400.0, 450.0, 80.0, 26.0);
        let before = pressed(
            &[node(1, "On the road", button, false)],
            button.center(),
            "Travel",
        )
        .expect("the menu button");
        // After the press the menu shows an entry with the same name elsewhere; the button stays.
        let after = [
            node(2, "On the road", at(390.0, 380.0, 100.0, 30.0), true),
            node(1, "On the road", button, false),
        ];
        assert!(press_moves(&before, &after, "Travel", "Click").is_none());
        // A press on the entry goes to the entry, the popup's node, not the button under it.
        let entry = at(400.0, 450.0, 100.0, 30.0);
        let both = [
            node(1, "On the road", button, false),
            node(2, "On the road", entry, true),
        ];
        assert!(pressed(&both, button.center(), "Travel").is_some_and(|p| p.node.popup));
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
        // The pane got 8 px wider: its bar moved by less than a lane. Not a new bar.
        let wider = snapshot(vec![field(608.0)], vec![], vec![at(609.0, 0.0, 6.0, 600.0)]);
        let narrow = snapshot(vec![field(600.0)], vec![], vec![bar]);
        assert!(bar_moves(&narrow, &wider).is_empty());
    }

    #[test]
    fn expect_moves_covers_g1a_and_g1b_but_not_g1c() {
        let mut tracker = Tracker::default();
        tracker.begin("s", "01-x", Some("A reorder."));
        for check in [G1A, G1B, G1C] {
            tracker.push(
                "s",
                "01-x",
                "960x640",
                Raw {
                    check,
                    control: String::new(),
                    old: egui::Rect::NOTHING,
                    new: egui::Rect::NOTHING,
                    trigger: String::new(),
                },
            );
        }
        let escaped: Vec<bool> = tracker
            .findings
            .iter()
            .map(|f| f.escaped.is_some())
            .collect();
        assert_eq!(escaped, [true, true, false]);
    }

    #[test]
    fn g2_finds_a_header_action_that_is_gone_or_turns_icon_only() {
        let edit = item(Role::Button, "Edit", at(500.0, 10.0, 80.0, 24.0), None);
        let open = item(Role::Button, "Open", at(600.0, 10.0, 80.0, 24.0), None);
        let text = |words: &str, x: f32| Text {
            words: words.into(),
            rect: at(x, 14.0, 10.0, 16.0),
        };
        let pencil = "\u{e3ae}";
        let view = |window: &str, width, items: &[Item], texts: &[Text]| HeaderView {
            window: window.into(),
            width,
            header: header(items, texts, 0.0),
            all: items
                .iter()
                .map(|item| (item.role, item.name.clone()))
                .collect(),
        };
        let wide = view(
            "1920x1080",
            1920.0,
            &[edit.clone(), open.clone()],
            &[
                text(pencil, 505.0),
                text("Edit", 530.0),
                text("Open", 630.0),
            ],
        );
        // Unchanged words: nothing.
        let same = view(
            "960x640",
            960.0,
            &[edit.clone(), open.clone()],
            &[
                text(pencil, 505.0),
                text("Edit", 530.0),
                text("Open", 630.0),
            ],
        );
        assert!(header_losses(&[same, wide]).is_empty());
        let wide = view(
            "1920x1080",
            1920.0,
            &[edit.clone(), open.clone()],
            &[
                text(pencil, 505.0),
                text("Edit", 530.0),
                text("Open", 630.0),
            ],
        );
        // `Edit` keeps its accessible name but paints only its icon; `Open` is gone.
        let narrow = view("960x640", 960.0, &[edit], &[text(pencil, 505.0)]);
        let found = header_losses(&[narrow, wide]);
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found.iter().any(|(_, raw)| raw.trigger.starts_with("gone")));
        assert!(
            found
                .iter()
                .any(|(_, raw)| raw.control == "Button \"Edit\""
                    && raw.trigger.starts_with("paints"))
        );
    }

    #[test]
    fn g3_finds_an_action_far_from_its_row_words_but_not_in_form_or_table_rows() {
        let row = at(0.0, 0.0, 1600.0, 32.0);
        let words = |name: &str, x: f32| Text {
            words: name.into(),
            rect: at(x, 4.0, 60.0, 20.0),
        };
        let action = |x| item(Role::Button, "Remove", at(x, 4.0, 70.0, 24.0), None);
        let goblin = [words("Goblin", 10.0)];
        assert_eq!(far_actions(&[row], &[action(1500.0)], &goblin).len(), 1);
        assert!(far_actions(&[row], &[action(70.0 + FAR_GAP)], &goblin).is_empty());
        // A form row: a label, then its field.
        let field = item(Role::ComboBox, "Speed", at(400.0, 4.0, 90.0, 24.0), None);
        assert!(far_actions(&[row], &[field], &goblin).is_empty());
        // A table row: three columns of words.
        let table = [
            words("Goblin", 10.0),
            words("CR 1/4", 200.0),
            words("50 XP", 400.0),
        ];
        assert!(far_actions(&[row], &[action(1500.0)], &table).is_empty());
        let two = [words("Goblin", 10.0), words("CR 1/4", 200.0)];
        assert_eq!(far_actions(&[row], &[action(1500.0)], &two).len(), 1);
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

    #[test]
    fn g6_finds_a_footer_far_below_a_short_body() {
        let pane_band = |top: f32| at(400.0, top, 500.0, 1080.0 - top);
        let row = item(Role::Button, "Process", at(420.0, 300.0, 90.0, 28.0), None);
        // The footer follows the body; up to `2 × L` (row padding, a frame) passes, as in P's check.
        assert!(footer_gaps(std::slice::from_ref(&row), &[pane_band(328.0)], &[]).is_empty());
        assert!(footer_gaps(std::slice::from_ref(&row), &[pane_band(352.0)], &[]).is_empty());
        // 0.5 px past it, and 700 px (the round 28 faults).
        assert_eq!(
            footer_gaps(std::slice::from_ref(&row), &[pane_band(352.5)], &[]).len(),
            1
        );
        assert_eq!(
            footer_gaps(std::slice::from_ref(&row), &[pane_band(1028.0)], &[]).len(),
            1
        );
        // A pane with P's record is checked exactly in the driver, not here.
        let footer = at(400.0, 1036.0, 500.0, 44.0);
        assert!(
            footer_gaps(std::slice::from_ref(&row), &[pane_band(1028.0)], &[footer]).is_empty()
        );
        // A body that fills the pane runs past its scroll area: it passes, also when the scroll
        // area ends above the band (an editor's frame margin).
        let tall = item(
            Role::Label,
            "Long note",
            at(420.0, 300.0, 400.0, 2000.0),
            Some(at(400.0, 100.0, 500.0, 800.0)),
        );
        assert!(footer_gaps(&[tall], &[pane_band(1028.0)], &[]).is_empty());
        // Another pane's content beside it does not count.
        let beside = item(Role::Button, "Other", at(100.0, 900.0, 90.0, 28.0), None);
        assert_eq!(
            footer_gaps(&[row, beside], &[pane_band(1028.0)], &[]).len(),
            1
        );
    }

    fn line(words: &str, rect: egui::Rect) -> Line {
        Line {
            words: words.into(),
            rect,
            first: rect,
            rows: 1,
            wraps: false,
        }
    }

    #[test]
    fn n2_finds_an_icon_off_the_centre_of_its_line() {
        let title = line("2026-01-01 19:00", at(372.0, 230.0, 120.0, 20.0));
        let menu = |y: f32| {
            item(
                Role::Button,
                "More for the recording",
                at(510.0, y, 24.0, 24.0),
                None,
            )
        };
        let dots = |y: f32| line("\u{e3e0}", at(515.0, y + 4.0, 14.0, 16.0));
        // On the line: centre 240 both.
        assert!(icon_centres(&[menu(228.0)], &[title.clone(), dots(228.0)]).is_empty());
        // 1 px passes; 6 px above (routing row 41) is found.
        assert!(icon_centres(&[menu(229.0)], &[title.clone(), dots(229.0)]).is_empty());
        let found = icon_centres(&[menu(222.0)], &[title.clone(), dots(222.0)]);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].trigger.starts_with("6 px above"));
        // A control with words is not an icon control; a text far away is not beside it.
        let words = item(Role::Button, "Process", at(510.0, 222.0, 80.0, 24.0), None);
        assert!(
            icon_centres(
                &[words],
                &[title.clone(), line("Process", at(515.0, 226.0, 60.0, 16.0))]
            )
            .is_empty()
        );
        // A two-line row: the title line is the one beside the icon, not the detail line.
        let detail = line("5 party · 1 × Wolf", at(372.0, 252.0, 120.0, 16.0));
        assert!(icon_centres(&[menu(228.0)], &[title.clone(), detail, dots(228.0)]).is_empty());
        // Text in a field: the field's box is the line.
        let field = item(Role::TextInput, "Name", at(372.0, 226.0, 120.0, 28.0), None);
        let typed = line("New encounter", at(380.0, 232.0, 100.0, 18.0));
        let back = item(Role::Button, "Back", at(340.0, 228.0, 24.0, 24.0), None);
        let arrow = line("\u{e058}", at(345.0, 232.0, 14.0, 16.0));
        assert!(
            icon_centres(
                &[field.clone(), back.clone()],
                &[typed.clone(), arrow.clone()]
            )
            .is_empty()
        );
        let low = item(Role::Button, "Back", at(340.0, 234.0, 24.0, 24.0), None);
        let low_arrow = line("\u{e058}", at(345.0, 238.0, 14.0, 16.0));
        assert_eq!(icon_centres(&[field, low], &[typed, low_arrow]).len(), 1);
        // A Markdown editor's text beside the icon is the DM's file, not a line.
        let editor = item(
            Role::MultilineTextInput,
            "Markdown editor",
            at(372.0, 200.0, 130.0, 400.0),
            None,
        );
        let file = line("# Session 1", at(380.0, 210.0, 100.0, 300.0));
        assert!(icon_centres(&[editor, menu(222.0)], &[file, dots(222.0)]).is_empty());
        // A tall tile with an icon box beside a short number is not on that number's line.
        let tile = item(
            Role::CheckBox,
            "Half damage",
            at(152.0, 130.0, 24.0, 40.0),
            None,
        );
        let box_icon = line("\u{e184}", at(156.0, 140.0, 14.0, 16.0));
        let hp = line("17", at(180.0, 140.0, 18.0, 16.0));
        assert!(icon_centres(&[tile], &[hp, box_icon]).is_empty());
        let far = item(Role::Button, "Close", at(900.0, 222.0, 24.0, 24.0), None);
        assert!(
            icon_centres(
                &[far],
                &[title, line("\u{e4f6}", at(905.0, 226.0, 14.0, 16.0))]
            )
            .is_empty()
        );
    }

    #[test]
    fn n5_finds_a_wrapping_text_that_starts_with_an_icon() {
        let mut status = line(
            "\u{e4e0} Captain Varra Black: told the party",
            at(170.0, 340.0, 380.0, 48.0),
        );
        status.wraps = true;
        status.rows = 2;
        let found = icon_wraps(std::slice::from_ref(&status));
        assert_eq!(found.len(), 1);
        assert!(found[0].trigger.starts_with("wrapped onto 2"));
        // Words first, a job that does not wrap, an icon alone, one row, or a written second line:
        // nothing.
        let mut one_row = status.clone();
        one_row.rows = 1;
        let mut written = status.clone();
        written.words = "\u{e3b2} Changed\n1 × Rival Captain".into();
        assert!(icon_wraps(&[one_row, written]).is_empty());
        let mut words_first = status.clone();
        words_first.words = "Captain Varra Black \u{e4e0}".into();
        let mut no_wrap = status.clone();
        no_wrap.wraps = false;
        let mut icon = status;
        icon.words = "\u{e4e0}".into();
        assert!(icon_wraps(&[words_first, no_wrap, icon]).is_empty());
    }

    #[test]
    fn n8_finds_a_word_twice_in_a_row() {
        let at_line = |words: &str, x: f32, width: f32| line(words, at(x, 100.0, width, 20.0));
        assert_eq!(
            doubled_words(&[], &[at_line("Travel Travel", 0.0, 100.0)]).len(),
            1
        );
        assert_eq!(
            doubled_words(&[], &[at_line("Day 1: Day", 0.0, 100.0)]).len(),
            1
        );
        // Two texts on one line: `Travel` then `Travel ·`.
        let pair = [
            at_line("Travel", 0.0, 50.0),
            at_line("Travel · 2 hours", 58.0, 120.0),
        ];
        assert_eq!(doubled_words(&[], &pair).len(), 1);
        // Passing: different words, a word after other words, a number twice, texts far apart.
        assert!(doubled_words(&[], &[at_line("Day 1: Harvest Eve", 0.0, 140.0)]).is_empty());
        assert!(doubled_words(&[], &[at_line("Rule · Creating a Rule", 0.0, 140.0)]).is_empty());
        assert!(
            doubled_words(
                &[],
                &[at_line("2d6 2d6", 0.0, 60.0), at_line("1 1", 80.0, 20.0)]
            )
            .is_empty()
        );
        let apart = [at_line("Travel", 0.0, 50.0), at_line("Travel", 300.0, 50.0)];
        assert!(doubled_words(&[], &apart).is_empty());
        // The file in a Markdown editor is the DM's text.
        let editor = item(
            Role::MultilineTextInput,
            "Markdown editor",
            at(0.0, 0.0, 400.0, 400.0),
            None,
        );
        assert!(doubled_words(&[editor], &[at_line("attack attack", 0.0, 100.0)]).is_empty());
    }

    #[test]
    fn n4_finds_a_quiet_action_off_the_line_start() {
        let clip = at(0.0, 0.0, 900.0, 600.0);
        let text = |words: &str, x: f32, y: f32, width: f32| Placed {
            words: words.into(),
            start: x,
            rect: at(x, y, width, 18.0),
            clip,
        };
        let progress = text("Processing: cleaning the transcript", 372.0, 200.0, 260.0);
        let cancel = |x: f32| item(Role::Button, "Cancel", at(x, 226.0, 64.0, 28.0), None);
        let words = |x: f32| text("Cancel", x, 231.0, 48.0);
        // `line_start`: the rect hangs left by its padding and the words start at the edge.
        let found = line_starts(
            &[cancel(364.0)],
            &[at(364.0, 226.0, 64.0, 28.0)],
            &[progress.clone(), words(372.0)],
        );
        assert!(found.is_empty(), "{found:?}");
        // Without it, the words start 8 px right of the edge (routing 81): found. 1 px passes.
        let found = line_starts(
            &[cancel(372.0)],
            &[at(372.0, 226.0, 64.0, 28.0)],
            &[progress.clone(), words(380.0)],
        );
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].trigger.contains("8 px right of"));
        assert!(
            line_starts(
                &[cancel(365.0)],
                &[at(365.0, 226.0, 64.0, 28.0)],
                &[progress.clone(), words(373.0),]
            )
            .is_empty()
        );
        // A neighbour that agrees is enough: the line above lines up, the status line below does
        // not.
        let status = text("Not connected", 358.0, 258.0, 90.0);
        assert!(
            line_starts(
                &[cancel(364.0)],
                &[at(364.0, 226.0, 64.0, 28.0)],
                &[progress.clone(), words(372.0), status,]
            )
            .is_empty()
        );
        // A tall title on its line comes before it: the action is not first.
        let title = Placed {
            rect: at(372.0, 220.0, 120.0, 40.0),
            ..text("Rook Ambry", 372.0, 220.0, 120.0)
        };
        let edit = item(Role::Button, "Edit", at(500.0, 226.0, 64.0, 28.0), None);
        assert!(
            line_starts(
                &[edit],
                &[at(500.0, 226.0, 64.0, 28.0)],
                &[
                    title,
                    text("Edit", 508.0, 231.0, 30.0),
                    text("NPC · Campaign", 372.0, 266.0, 100.0),
                ]
            )
            .is_empty()
        );
        // A bordered (not quiet) action is not N4's.
        assert!(line_starts(&[cancel(372.0)], &[], &[progress.clone(), words(380.0)]).is_empty());
        // Not first on its line: normal padding passes; a `line_start` hang over the words before
        // it is found.
        let label = text("Saved", 372.0, 231.0, 40.0);
        let after = |x: f32| item(Role::Button, "Undo", at(x, 226.0, 56.0, 28.0), None);
        let undo = |x: f32| text("Undo", x, 231.0, 34.0);
        assert!(
            line_starts(
                &[after(416.0)],
                &[at(416.0, 226.0, 56.0, 28.0)],
                &[label.clone(), undo(424.0),]
            )
            .is_empty()
        );
        assert_eq!(
            line_starts(
                &[after(404.0)],
                &[at(404.0, 226.0, 56.0, 28.0)],
                &[label, undo(416.0)]
            )
            .len(),
            1
        );
    }

    fn finding(check: &'static str, state: &str) -> Finding {
        Finding {
            check,
            scenario: "s".into(),
            state: state.into(),
            window: "960x640".into(),
            control: String::new(),
            old: [0.0; 4],
            new: [0.0; 4],
            trigger: String::new(),
            escaped: None,
        }
    }

    #[test]
    fn known_lines_fail_a_new_finding_and_a_stale_line() {
        let key = |check: &str, state: &str| KnownKey {
            scenario: "s".into(),
            check: check.into(),
            state: state.into(),
            window: "960x640".into(),
            name: String::new(),
        };
        let known: Known = [(key(G3, "01-a"), 1)].into_iter().collect();
        let run = [finding(G3, "01-a"), finding(G2, "02-b")];
        assert_eq!(compare_known(&run, &known, &["s"]), (vec![], vec![]));
        let more = [finding(G3, "01-a"), finding(G1A, "02-b")];
        let (worse, stale) = compare_known(&more, &known, &["s"]);
        assert_eq!((worse.len(), stale.len()), (1, 0), "{worse:?}");
        let (worse, stale) = compare_known(&[], &known, &["s"]);
        assert_eq!((worse.len(), stale.len()), (0, 1));
        assert_eq!(compare_known(&[], &known, &["other"]), (vec![], vec![]));
        let mut escaped = finding(G1A, "02-b");
        escaped.escaped = Some("A reorder.");
        assert_eq!(
            compare_known(&[finding(G3, "01-a"), escaped], &known, &["s"]),
            (vec![], vec![])
        );
        // One name per key: a fault exchanged for another in the same state is a new key.
        let mut other = finding(G3, "01-a");
        other.control = "Button \"Edit\"".into();
        assert_eq!(compare_known(&[other], &known, &["s"]).0.len(), 1);
        // The file a package pastes from: the run's scenarios, in the list's form, and it reads back.
        let file = known_lines(&[finding(G3, "01-a"), finding(G2, "02-b")], &["s", "t"]);
        assert_eq!(
            file,
            "{\n  \"s\": [\n    [\"G3/far-action\",\"01-a\",\"960x640\",\"\",1]\n  ]\n}\n"
        );
        assert_eq!(parse_known(&file), Ok(known.clone()));
        // The workflow cannot raise the list: against the merge base's list, a higher count and an
        // added key each fail with their key; only a named raise lets one through.
        let base = Base::List(known.clone());
        let mut raised = known.clone();
        raised.insert(key(G3, "01-a"), 2);
        raised.insert(key(N5, "03-c"), 1);
        let (failures, _) = compare_base(&raised, &base, &[], false);
        assert_eq!(failures.len(), 2, "{failures:?}");
        assert!(
            failures
                .iter()
                .any(|f| f.contains("G3/far-action / s / 01-a") && f.contains("1 -> 2"))
        );
        assert!(
            failures
                .iter()
                .any(|f| f.contains("N5/icon-starts-wrapping-text / s / 03-c"))
        );
        let named = raises(
            "N5/icon-starts-wrapping-text / s / 03-c / 960x640 /  => a new wrap, fixed in #400",
        );
        let (failures, notes) = compare_base(&raised, &base, &named, false);
        assert_eq!(failures.len(), 1, "{failures:?}");
        assert!(
            notes
                .iter()
                .any(|n| n.starts_with("RAISED") && n.contains("fixed in #400"))
        );
        // Lowering passes; an unread base fails only in the landing gate; a first list passes.
        assert!(compare_base(&Known::new(), &base, &[], true).0.is_empty());
        let unread = Base::Unreadable("no origin/main".into());
        assert!(compare_base(&known, &unread, &[], false).0.is_empty());
        assert_eq!(compare_base(&known, &unread, &[], true).0.len(), 1);
        assert!(
            compare_base(&raised, &Base::Absent("x".into()), &[], true)
                .0
                .is_empty()
        );
    }
}
