//! Gate invariants that compare what the app shows:
//!
//! - **G1a** a message came or went in a pane, and content below its top in that pane moved.
//! - **G1b** the control a script pressed moved after the press.
//! - **G1c** a scroll bar came or went in a pane, and a right edge in that pane moved.
//! - **G2** a header action at the widest size is gone, or paints other words, at a narrower size.
//! - **G3** at 1920 px, a row's first action at the right is far from the row's last words.
//! - **G4** one pane shows the same sentence twice.
//! - **G6** a pane's message footer sits more than `S` below a body that ends short of it.
//!
//! G1a, G1b, G1c and G3 block: a finding that `stillness_known::KNOWN` does not list fails the
//! run, and so does a listed state with fewer findings (the list may only shrink). G2, G4 and
//! G1b's renamed tier (`G1B_RENAMED`) are report-only.
//!
//! G1 compares two states in a row at one window size. It matches a control by its pane, role,
//! accessible name and place among equal names, never by AccessKit id alone: egui's automatic ids
//! shift when a widget is inserted before them. `Driver::expect_moves` is the one escape: it
//! marks the next state's G1a and G1b findings as intended, with a reason the report counts.
//!
//! Messages: until `src/design` registers each message's rect, a message is found in the painted
//! shapes by what `design::message` and `design::small_message` draw: a 3 px bar in a result tone,
//! a tone icon after it, then the words. The words' AccessKit label gives the pane.
use super::ergonomics::{Control, StateKey};
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
pub const CHECKS: [&str; 8] = [G1A, G1B, G1B_RENAMED, G1C, G2, G3, G4, G6];
/// The checks whose findings must be in `stillness_known::KNOWN`.
pub const BLOCKING: [&str; 4] = [G1A, G1B, G1C, G3];

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

/// The keys of a message's own block: what lies in its footer, or, in a body, its own line and the
/// lines under it that start right of its bar (its second lines and its choices, as
/// `message_choice_row` places them), down to the first line that starts at or left of the bar.
fn own_block<'a>(
    message: &Message,
    pane: &[(Key<'a>, &'a Item)],
    footers: &[egui::Rect],
) -> HashSet<Key<'a>> {
    let mut out = HashSet::new();
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
    for message in &new.messages {
        if !old_keys.contains(&(message.key().0, message.words.clone())) {
            changed.push((message, new, old, "appeared"));
        }
    }
    for message in &old.messages {
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

/// The selected-toggle exemption: the line and one side edge stay, the width changes by at most
/// `TOGGLE_WEIGHT`, and the press point is still on the control.
fn toggle_weight(old: egui::Rect, new: egui::Rect, point: egui::Pos2) -> bool {
    (old.top() - new.top()).abs() <= MOVE_TOLERANCE
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
    let (rect, by_name) = if let Some(node) = equal.get(before.place) {
        (node.rect, true)
    } else if equal.is_empty() {
        let node = after
            .iter()
            .find(|node| node.id == before.node.id && node.role == before.node.role)?;
        (node.rect, false)
    } else {
        return None;
    };
    let old = before.node.rect;
    moved(old, rect)?;
    if toggle_weight(old, rect, before.point) {
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

/// G6: a footer follows a short body. For each footer band (`design::footer_bands`: from `S` above
/// the footer to the pane's bottom), the last body item above it in the pane ends at most 1 px
/// above the band: the footer's first message is at most `S` + 1 px below the body. A body that
/// fills the pane, scrolled or not, ends at the band, so it passes; only a footer left far below a
/// short body is found.
pub fn footer_gaps(items: &[Item], footers: &[egui::Rect]) -> Vec<Raw> {
    let mut out = Vec::new();
    for band in footers {
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
        let gap = band.top() + S - bottom;
        if gap > S + MOVE_TOLERANCE {
            out.push(Raw {
                check: G6,
                control: display(item),
                old: item.rect,
                new: *band,
                trigger: format!("the footer is {gap:.0} px below the body's last item"),
            });
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

/// A known line: a blocking check, a state, and its exact count of findings.
pub type KnownLine = (&'static str, StateKey<'static>, usize);

/// Blocking-check lines by state: a state with more findings than `known` lists (or not listed),
/// then a listed state with fewer (the list is stale). Both fail a run. Escaped findings do not
/// count. A scenario that did not run gives no stale line.
pub fn compare_known(
    findings: &[Finding],
    known: &[KnownLine],
    scenarios: &[&str],
) -> (Vec<String>, Vec<String>) {
    let mut found: BTreeMap<(&str, StateKey<'_>), usize> = BTreeMap::new();
    for finding in findings
        .iter()
        .filter(|f| f.escaped.is_none() && BLOCKING.contains(&f.check))
    {
        let key = (
            finding.check,
            (
                finding.scenario.as_str(),
                finding.state.as_str(),
                finding.window.as_str(),
            ),
        );
        *found.entry(key).or_default() += 1;
    }
    let listed = |check: &str, key: StateKey<'_>| {
        known
            .iter()
            .filter(|(c, k, _)| *c == check && *k == key)
            .map(|(.., count)| count)
            .sum::<usize>()
    };
    let line =
        |check: &str, (scenario, state, window): StateKey<'_>, count: usize, limit: usize| {
            format!("{check} / {scenario} / {state} / {window}: {count} (known {limit})")
        };
    let worse = found
        .iter()
        .filter(|((check, key), count)| **count > listed(check, *key))
        .map(|((check, key), count)| line(check, *key, *count, listed(check, *key)))
        .collect();
    let stale = known
        .iter()
        .filter(|(_, (scenario, ..), _)| scenarios.contains(scenario))
        .filter_map(|(check, key, limit)| {
            let count = found.get(&(*check, *key)).copied().unwrap_or(0);
            (count < *limit).then(|| {
                format!(
                    "{}. Fixed: lower or remove its line in stillness_known::KNOWN",
                    line(check, *key, count, *limit)
                )
            })
        })
        .collect();
    (worse, stale)
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
        known: &[KnownLine],
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
            .map(|check| (*check, known.iter().filter(|(c, ..)| c == check).count()))
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
            "Stillness (stillness.json): {}. Known lines: {}. expect_moves: {} uses. Check time {:?} (summed over workers).",
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
            footers: Vec::new(),
        }
    }

    fn saved(rect: egui::Rect, clip: Option<egui::Rect>) -> Message {
        Message {
            words: "Changes saved.".into(),
            rect,
            scope: Some(1),
            clip,
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
        // Renamed: found by id, never blocking.
        let renamed = [node(7, "Stop", at(500.0, 50.0, 80.0, 24.0), false)];
        assert_eq!(
            press_moves(&before, &renamed, "Run", "Click").map(|raw| raw.check),
            Some(G1B_RENAMED)
        );
    }

    #[test]
    fn g1b_exempts_a_selected_toggle_up_to_its_weight_and_no_more() {
        let toggle = at(500.0, 70.0, 110.0, 28.0);
        let before = pressed(
            &[node(3, "Edit Markdown", toggle, false)],
            toggle.center(),
            "Library",
        )
        .expect("the toggle");
        let grown = |width| {
            [node(
                3,
                "Edit Markdown",
                at(500.0, 70.0, 110.0 + width, 28.0),
                false,
            )]
        };
        assert!(press_moves(&before, &grown(TOGGLE_WEIGHT), "Library", "Click").is_none());
        assert_eq!(
            press_moves(&before, &grown(TOGGLE_WEIGHT + 0.5), "Library", "Click").map(|r| r.check),
            Some(G1B)
        );
        // Its line must stay: the same growth one line lower is a move.
        let lower = [node(
            3,
            "Edit Markdown",
            at(500.0, 72.0, 112.0, 28.0),
            false,
        )];
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
        // The footer follows the body: its message is `S` below the last row.
        assert!(footer_gaps(std::slice::from_ref(&row), &[pane_band(328.0)]).is_empty());
        assert!(footer_gaps(std::slice::from_ref(&row), &[pane_band(329.0)]).is_empty());
        // 1.5 px past the rule, and 700 px (the round 28 faults).
        assert_eq!(
            footer_gaps(std::slice::from_ref(&row), &[pane_band(329.5)]).len(),
            1
        );
        assert_eq!(
            footer_gaps(std::slice::from_ref(&row), &[pane_band(1028.0)]).len(),
            1
        );
        // A body that fills the pane is clipped at the band: it passes.
        let tall = item(
            Role::Label,
            "Long note",
            at(420.0, 300.0, 400.0, 2000.0),
            Some(at(400.0, 100.0, 500.0, 928.0)),
        );
        assert!(footer_gaps(&[tall], &[pane_band(1028.0)]).is_empty());
        // Another pane's content beside it does not count.
        let beside = item(Role::Button, "Other", at(100.0, 900.0, 90.0, 28.0), None);
        assert_eq!(footer_gaps(&[row, beside], &[pane_band(1028.0)]).len(), 1);
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
        let known: &[KnownLine] = &[(G3, ("s", "01-a", "960x640"), 1)];
        let run = [finding(G3, "01-a"), finding(G2, "02-b")];
        assert_eq!(compare_known(&run, known, &["s"]), (vec![], vec![]));
        let more = [finding(G3, "01-a"), finding(G1A, "02-b")];
        let (worse, stale) = compare_known(&more, known, &["s"]);
        assert_eq!((worse.len(), stale.len()), (1, 0), "{worse:?}");
        let (worse, stale) = compare_known(&[], known, &["s"]);
        assert_eq!((worse.len(), stale.len()), (0, 1));
        assert_eq!(compare_known(&[], known, &["other"]), (vec![], vec![]));
        let mut escaped = finding(G1A, "02-b");
        escaped.escaped = Some("A reorder.");
        assert_eq!(
            compare_known(&[finding(G3, "01-a"), escaped], known, &["s"]),
            (vec![], vec![])
        );
    }
}
