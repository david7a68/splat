//! Shared UI parts. Screens compose these instead of styling widgets themselves.
use super::{comparison::Comparison, extras::space, icons, status::Tinted, tokens::*};
use egui::{
    self, Align, Align2, Color32, CornerRadius, FontId, Id, Layout, Pos2, Rect, Response, Sense,
    Stroke, StrokeKind, Ui, Vec2, WidgetInfo, WidgetType, pos2, text::LayoutJob, vec2,
};
use std::sync::Arc;

fn corner(value: f32) -> CornerRadius {
    CornerRadius::same(value as u8)
}

/// A quiet menu button with its caret after the word.
pub fn menu_button(ui: &mut Ui, word: &str, selected: bool) -> Response {
    draw_menu_button(ui, word, selected, false, None, Density::Scan)
}

/// A menu button whose word starts at the text edge of its own row.
pub fn menu_button_at_line_start(ui: &mut Ui, word: &str, selected: bool) -> Response {
    draw_menu_button(ui, word, selected, true, None, Density::Scan)
}

fn draw_menu_button(
    ui: &mut Ui,
    word: &str,
    selected: bool,
    line_start: bool,
    blocked: Option<&str>,
    density: Density,
) -> Response {
    let label = format!("{word}    ");
    let button = Action::quiet(&label)
        .name(word)
        .selected(selected)
        .blocked(blocked)
        .density(density);
    let button = if line_start {
        button.line_start()
    } else {
        button
    };
    let response = button.show(ui);
    let caret = Rect::from_min_max(
        pos2(response.rect.right() - L - XS, response.rect.top()),
        response.rect.right_bottom(),
    );
    super::combo_icon(ui, caret, ui.style().interact(&response), false);
    response
}

/// The width `menu_button` takes for this word.
pub fn menu_button_width(ui: &Ui, word: &str) -> f32 {
    Action::quiet(&format!("{word}    ")).natural_width(ui)
}

/// One line of text cut to fit, with an ellipsis. The tree still has the full text.
pub fn one_line(
    ui: &Ui,
    text: &str,
    font: FontId,
    color: Color32,
    max_width: f32,
) -> Arc<egui::Galley> {
    let mut job = LayoutJob::simple_singleline(text.replace(['\n', '\r'], " "), font, color);
    job.wrap.max_width = max_width.max(1.0);
    job.wrap.max_rows = 1;
    job.wrap.break_anywhere = true;
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

/// Whole words of `text` that fit. A cut ends with an ellipsis, never a joining word.
/// The first word stays unless it is itself a joining word.
pub fn words_that_fit(ui: &Ui, text: &str, font: &FontId, width: f32) -> String {
    let mut words: Vec<_> = text.split_whitespace().collect();
    let full = words.join(" ");
    if number_width(ui, &full, font) <= width || words.is_empty() {
        return full;
    }
    while let Some(word) = words.last() {
        let joining = ["to", "of", "on", "from", "and", "the", "de", "von", "·"]
            .iter()
            .any(|joining| word.eq_ignore_ascii_case(joining));
        if !joining {
            let fitted = format!("{}…", words.join(" "));
            if words.len() == 1 || number_width(ui, &fitted, font) <= width {
                return fitted;
            }
        }
        words.pop();
    }
    "…".to_owned()
}

/// A search hint inside its text area (X2): whole words that fit, as [`words_that_fit`];
/// when not even the first word fits, its start that fits with an ellipsis (a word with no
/// spaces, in any script, is cut between characters); nothing when even an ellipsis does not
/// fit. A hint is never wider than its area.
fn hint_that_fits(ui: &Ui, hint: &str, font: &FontId, width: f32) -> String {
    let words = words_that_fit(ui, hint, font, width);
    if number_width(ui, &words, font) <= width {
        return words;
    }
    let chars: Vec<char> = words.trim_end_matches('…').chars().collect();
    for end in (1..chars.len()).rev() {
        let start = format!("{}…", chars[..end].iter().collect::<String>());
        if number_width(ui, &start, font) <= width {
            return start;
        }
    }
    if number_width(ui, "…", font) <= width {
        "…".to_owned()
    } else {
        String::new()
    }
}

/// Temporary HP, with its mark, in every place that shows it.
pub fn temp_hp_text(temp: i64) -> String {
    format!("{} +{temp}", icons::TEMP_HP)
}

// ------------------------------------------------------------------------------------------------
// Reader toolbar (M2): one availability value per action gives its lock, its hover, its
// accessible description and its painted reason.

/// Whether one action of a [`ReaderToolbar`] runs now, and why not. This ONE value gives the
/// control's disabled state, its hover and accessible description, and the reason that is
/// painted: the toolbar takes no `blocked` from its caller and no flag that hides a reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Availability {
    /// It waits for a steady cause that the toolbar states: painted once under its row, in
    /// `Small` muted words at the row's start (ruling R35), whatever other action waits for it.
    Waits(super::outcome::Sentence),
    /// It runs while the pane's open draft has no changes, and waits while it has: the toolbar
    /// paints the draft's reason itself, on the save row's line or under the toolbar.
    Draft,
}

/// The reader toolbar's mode switch lock. It is not an [`Availability`]: no action can take
/// its hover-only form, which does not compile as an action's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModeLock {
    Open,
    /// The entry's file cannot be read, and the reader's body states that fault with its
    /// recovery in painted words in the same pane: the switch has the reason as its hover and
    /// accessible description only, so one cause is said once.
    BodyStatesFault(super::outcome::Sentence),
}

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

/// Where the pane draws its draft's save row: a closed layout value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DraftRow<'r> {
    /// The save row, `label` on Save, at most `width` wide.
    Save { label: &'r str, width: f32 },
    /// No save row: a comparison owns the pane, or the entry is not editable.
    None,
}

/// What [`ReaderToolbar::show`] gives back: the press, the source mode after the switch, and
/// the save row's press when it drew one.
#[must_use = "act on the press and the save row"]
pub struct ToolbarShown<A> {
    pub press: Option<ToolbarPress<A>>,
    pub source: bool,
    pub save: Option<super::save_row::SaveRow>,
}

/// One action of the toolbar: its words, its icon, the caller's id, and its availability.
struct GroupAction<'a, A> {
    label: &'a str,
    id: A,
    availability: Availability,
}

impl<A> GroupAction<'_, A> {
    fn action<'r>(&'r self, draft: Option<&'r str>) -> Action<'r> {
        let reason = match &self.availability {
            Availability::Waits(reason) => Some(reason.as_str()),
            Availability::Draft => draft,
        };
        Action::quiet(self.label)
            .density(Density::Read)
            .blocked(reason)
    }
}

/// What the DM pressed in a [`ReaderToolbar`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolbarPress<A> {
    Back,
    Action(A),
}

/// The words of `More`, the toolbar's overflow menu.
const MORE: &str = "More";

/// The reader's toolbar, a closed part (ruling R38) that also draws the pane's save row. The
/// mode switch (`Read` and the source mode), then `Back`, then the extras. The mode switch keeps
/// the room of its wider source label and `Back` decides its key hint from the prefix and a
/// reserved `More` alone, so `Back` has one place whatever the entry's kind or its extras. The
/// extras follow on the row when they fit, else behind `More` (Read density, as tall as the
/// other controls) at the row's right end. Each action's availability gives its lock and its
/// reason: a steady cause is painted once under the row; the draft's reason is painted by
/// [`show`](Self::show) itself, on the save row's line, or under the toolbar with no save row.
#[must_use = "draw it with `show`"]
pub struct ReaderToolbar<'a, A> {
    source: bool,
    source_label: &'a str,
    wider_label: &'a str,
    mode: ModeLock,
    /// `Back`'s hint and icon, while there is history.
    back: Option<&'a str>,
    actions: Vec<GroupAction<'a, A>>,
}

impl<'a, A: Copy> ReaderToolbar<'a, A> {
    /// `Read` and `source_label`, showing the source when `source`; the switch keeps the room of
    /// `wider_label` (the other entries' source label); `mode` is the switch's own lock.
    pub fn new(source: bool, source_label: &'a str, wider_label: &'a str, mode: ModeLock) -> Self {
        Self {
            source,
            source_label,
            wider_label,
            mode,
            back: None,
            actions: Vec::new(),
        }
    }

    /// `Back` ([`Action::history_back`]), with `hint` on hover, while there is history to go
    /// back to. The part draws its arrow; a caller gives none.
    ///
    /// ```
    /// use splat_design::layout::doc_parts::{ModeLock, ReaderToolbar};
    /// let _ = ReaderToolbar::<u8>::new(false, "Markdown", "Markdown", ModeLock::Open)
    ///     .back(true, "Return to the entry before this link.");
    /// ```
    /// ```compile_fail,E0061
    /// use splat_design::layout::doc_parts::{ModeLock, ReaderToolbar};
    /// let _ = ReaderToolbar::<u8>::new(false, "Markdown", "Markdown", ModeLock::Open)
    ///     .back(true, "←", "Return to the entry before this link.");
    /// ```
    pub fn back(mut self, history: bool, hint: &'a str) -> Self {
        self.back = history.then_some(hint);
        self
    }

    /// An extra, quiet at `Density::Read`, in its words only, that gives `id` back when pressed
    /// (round 31 reader-toolbar-icon-form, M0355: every toolbar action has one form; `Back` is
    /// navigation and keeps its arrow). It takes no icon.
    ///
    /// ```
    /// use splat_design::layout::doc_parts::{Availability, ModeLock, ReaderToolbar};
    /// let _ = ReaderToolbar::new(false, "Markdown", "Markdown", ModeLock::Open)
    ///     .action("Copy to…", 1, Availability::Draft);
    /// ```
    /// ```compile_fail,E0061
    /// use splat_design::layout::doc_parts::{Availability, ModeLock, ReaderToolbar};
    /// let _ = ReaderToolbar::new(false, "Markdown", "Markdown", ModeLock::Open)
    ///     .action("Copy to…", Some("c"), 1, Availability::Draft);
    /// ```
    pub fn action(mut self, label: &'a str, id: A, availability: Availability) -> Self {
        self.actions.push(GroupAction {
            label,
            id,
            availability,
        });
        self
    }

    /// The `Back` control, with or without its key.
    fn back_action(hint: &'a str, key: bool) -> Action<'a> {
        Action::history_back(key).hint(hint)
    }

    /// Everything from the toolbar to the save row, in `ui`: the toolbar row and each steady
    /// cause once under it; with no save row (`row` of the source mode after the switch), the
    /// draft's reason while it holds; the caller's `between` (a file line); then the save row of
    /// `draft`, with the draft's reason on its own line when a whole form fits there, else in a
    /// measured slot under it that keeps its room in both states. `blocks` names what the draft
    /// blocks; the part holds its words. Nothing here can leave the reason unpainted.
    pub fn show(
        self,
        ui: &mut Ui,
        draft: &super::DraftState<'_>,
        blocks: DraftBlocks,
        row: impl Fn(bool) -> DraftRow<'a>,
        between: impl FnOnce(&mut Ui),
    ) -> ToolbarShown<A> {
        let on_disk = draft.file_changed();
        let held = draft.discard_shown() || on_disk;
        let (long, short) = if on_disk {
            (
                super::save_row::CHANGED_ON_DISK,
                super::save_row::CHANGED_ON_DISK,
            )
        } else {
            (blocks.long(), blocks.short())
        };
        let draft_reason = held.then_some(long);
        let cites = self
            .actions
            .iter()
            .any(|action| action.availability == Availability::Draft);
        // With no save row, the draft's reason goes on the toolbar's own line after the actions
        // when it fits whole there (measured before paint), else on the line under the toolbar.
        // It is always painted: no state and no caller leaves it out.
        let no_row_reason = (row(self.source) == DraftRow::None && cites && held).then_some(long);
        let mut inline = false;
        let mut pressed = None;
        let mut source = self.source;
        let gap = ui.spacing().item_spacing.x;
        let more_width = Action::quiet(MORE).density(Density::Read).natural_width(ui);
        let mode_reason = match &self.mode {
            ModeLock::BodyStatesFault(reason) => Some(reason.as_str()),
            ModeLock::Open => None,
        };
        ui.horizontal(|ui| {
            let left = ui.cursor().left();
            let room = ui.available_width();
            let before = segment_width(ui, self.source_label);
            locked(ui, mode_reason, |ui| {
                segmented(
                    ui,
                    &mut source,
                    &[(false, "Read"), (true, self.source_label)],
                )
            });
            // The wider source label's room: `Back` starts at one x for every entry. In a reader
            // too narrow for it and `More`, `More` keeps its room first (beside Lookup at 960).
            let reserve = (segment_width(ui, self.wider_label) - before).min(
                ui.available_width()
                    - more_width
                    - gap
                    - self.back.map_or(0.0, |hint| {
                        Self::back_action(hint, false).natural_width(ui) + gap
                    }),
            );
            if reserve > 0.0 {
                space(ui, reserve);
            }
            if let Some(hint) = self.back {
                let prefix = ui.cursor().left() - left;
                let with_key = Self::back_action(hint, true).natural_width(ui);
                let key = prefix + gap + with_key + gap + more_width <= room;
                if Self::back_action(hint, key).show(ui).clicked() {
                    pressed = Some(ToolbarPress::Back);
                }
            }
            if self.actions.is_empty() {
                return;
            }
            let width: f32 = self
                .actions
                .iter()
                .map(|action| action.action(draft_reason).natural_width(ui) + gap)
                .sum();
            if width <= ui.available_width() {
                for action in &self.actions {
                    if action.action(draft_reason).show(ui).clicked() {
                        pressed = Some(ToolbarPress::Action(action.id));
                    }
                }
                if let Some(reason) = no_row_reason
                    && number_width(ui, reason, &Type::Small.font(ui)) + gap <= ui.available_width()
                {
                    blocked_reason(ui, Some(reason));
                    inline = true;
                }
            } else {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let response = Action::quiet(MORE)
                        .density(Density::Read)
                        .min_width(more_width)
                        .show(ui);
                    super::menu_popup(&*ui, &response).show(|ui| {
                        for action in &self.actions {
                            if action.action(draft_reason).show(ui).clicked() {
                                pressed = Some(ToolbarPress::Action(action.id));
                                ui.close();
                            }
                        }
                    });
                });
            }
        });
        // Each steady cause once, in the order of the controls that wait for it.
        let mut said: Vec<&str> = Vec::new();
        for availability in self.actions.iter().map(|action| &action.availability) {
            if let Availability::Waits(reason) = availability
                && !said.contains(&reason.as_str())
            {
                said.push(reason.as_str());
            }
        }
        for reason in said {
            blocked_reason(ui, Some(reason));
        }
        let layout = row(source);
        if layout == DraftRow::None && cites && held && !inline {
            blocked_reason(ui, Some(long));
        }
        between(ui);
        let save = match layout {
            DraftRow::Save { label, width } => Some(
                ui.scope(|ui| {
                    ui.set_max_width(width);
                    super::save_row::save_row_with(
                        ui,
                        draft,
                        label,
                        cites.then_some(super::save_row::RowReason {
                            long,
                            short,
                            locked: held,
                        }),
                    )
                })
                .inner,
            ),
            DraftRow::None => None,
        };
        ToolbarShown {
            press: pressed,
            source,
            save,
        }
    }
}

/// Controls that cannot be used for one reason, which each of them says on hover.
pub fn locked<R>(ui: &mut Ui, reason: Option<&str>, add: impl FnOnce(&mut Ui) -> R) -> R {
    let key = Id::new("locked_control_reason");
    let previous = ui.data_mut(|data| {
        let previous = data.get_temp::<String>(key);
        if let Some(reason) = reason {
            data.insert_temp(key, reason.to_owned());
        }
        previous
    });
    let group = ui.add_enabled_ui(reason.is_none(), add);
    ui.data_mut(|data| {
        data.remove::<String>(key);
        if let Some(previous) = previous {
            data.insert_temp(key, previous);
        }
    });
    if let Some(reason) = reason {
        group.response.on_disabled_hover_text(reason);
    }
    group.inner
}

/// Keep the enclosing lock's reason on each disabled control.
pub fn locked_control(ui: &Ui, response: &Response) {
    if !ui.is_enabled()
        && let Some(reason) =
            ui.data(|data| data.get_temp::<String>(Id::new("locked_control_reason")))
    {
        ui.ctx()
            .accesskit_node_builder(response.id, |node| node.set_description(reason.clone()));
        response.clone().on_disabled_hover_text(reason);
    }
}

/// The width of a number. IBM Plex Sans has tabular digits, so these do not jitter.
pub fn number_width(ui: &Ui, text: &str, font: &FontId) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(text.to_owned(), font.clone(), Color32::PLACEHOLDER)
            .size()
            .x
    })
}

/// Paint a number with its right edge at `right`. Returns its left edge.
pub fn number(
    painter: &egui::Painter,
    right: f32,
    center_y: f32,
    text: &str,
    font: &FontId,
    color: Color32,
) -> f32 {
    painter
        .text(
            pos2(right, center_y),
            Align2::RIGHT_CENTER,
            text,
            font.clone(),
            color,
        )
        .left()
}

/// Give a widget with no visible label its accessible name.
pub fn named(ui: &Ui, response: &Response, name: &str) {
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(name.to_owned());
    });
    locked_control(ui, response);
}

/// A page title and one context line on the same baseline. Actions keep their room.
/// Actions are drawn right to left: put the bordered New action first in the closure.
pub fn page_header(ui: &mut Ui, title: &str, context: Option<&str>, actions: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), Density::Scan.control()),
        Layout::right_to_left(Align::Center),
        |ui| {
            ui.set_min_height(Density::Scan.control());
            // The title and its count go first: the actions get the width they leave, and a
            // status link among them wraps there before the count is cut.
            let room = (ui.available_width() - page_title_width(ui, title, context)).max(0.0);
            ui.allocate_ui_with_layout(
                vec2(room, Density::Scan.control()),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.set_min_height(Density::Scan.control());
                    actions(ui);
                },
            );
            page_title(ui, title, context);
        },
    );
    rule(ui);
    space(ui, L);
}

/// The width `page_title` takes with its whole count, and the gap before the actions.
fn page_title_width(ui: &Ui, title: &str, context: Option<&str>) -> f32 {
    number_width(ui, title, &Type::Title.font(ui))
        + context.map_or(0.0, |context| {
            ui.spacing().item_spacing.x + XS + number_width(ui, context, &Type::Small.font(ui))
        })
        + ui.spacing().item_spacing.x
        // Rounding must not cut the count by a fraction of a point.
        + 1.0
}

// Keep rendering that does not depend on the caller's closure out of the generic
// function. Otherwise rustc generates another copy for every page header.
fn page_title(ui: &mut Ui, title: &str, context: Option<&str>) {
    ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
        ui.label(Type::Title.rich(ui, title).color(colors().text));
        if let Some(context) = context {
            space(ui, XS);
            ui.add(
                egui::Label::new(Type::Small.rich(ui, context).color(colors().muted)).truncate(),
            )
            .on_hover_text(context);
        }
    });
}

/// A list group, with its nonzero count in muted type and no rule.
pub fn group_heading(ui: &mut Ui, title: &str, count: usize) {
    ui.label(group_heading_text(ui, title, count));
}

pub fn group_heading_text(ui: &Ui, title: &str, count: usize) -> LayoutJob {
    let mut job = LayoutJob::default();
    for (text, color) in [
        (title.to_uppercase(), colors().text),
        (
            if count == 0 {
                String::new()
            } else {
                format!(" {}", self::count(count, "", ""))
            },
            colors().muted,
        ),
    ] {
        job.append(
            &text,
            0.0,
            egui::TextFormat {
                font_id: Type::Label.font(ui),
                extra_letter_spacing: Type::Label.tracking(),
                line_height: Some(Type::Label.line()),
                color,
                ..Default::default()
            },
        );
    }
    job
}

/// Run state aligns with the label and keeps its full words on hover.
pub fn run_nav_item(ui: &mut Ui, detail: &str, selected: bool) -> Response {
    let c = colors();
    let height = Density::Glance.control() + XS;
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::Button,
            ui.is_enabled(),
            selected,
            format!("Run · {detail}"),
        )
    });
    if ui.is_rect_visible(rect) {
        let fill = if selected {
            c.selected
        } else if response.hovered() {
            c.hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, corner(RADIUS_CONTROL), fill);
        if selected {
            ui.painter().rect_filled(
                Rect::from_min_size(rect.min, vec2(3.0, rect.height())),
                corner(1.0),
                c.accent,
            );
        }
        let color = if selected { c.text } else { c.muted };
        let center = rect.top() + height * 0.35;
        ui.painter().text(
            pos2(rect.left() + S + 2.0, center),
            Align2::LEFT_CENTER,
            icons::RUN,
            Type::Body.font(ui),
            color,
        );
        ui.painter().text(
            pos2(rect.left() + 28.0, center),
            Align2::LEFT_CENTER,
            "Run",
            if selected { Type::Strong } else { Type::Body }.font(ui),
            color,
        );
        let shown = run_nav_detail(ui, detail, rect.width() - 28.0 - S);
        ui.painter().text(
            pos2(rect.left() + 28.0, rect.top() + height * 0.56),
            Align2::LEFT_TOP,
            shown,
            Type::Small.font(ui),
            c.muted,
        );
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_text(format!("Run · {detail}"))
}

fn run_nav_detail(ui: &Ui, detail: &str, width: f32) -> String {
    let font = Type::Small.font(ui);
    if number_width(ui, detail, &font) <= width {
        detail.to_owned()
    } else if detail.starts_with("Ended ·") {
        "Ended".to_owned()
    } else if detail.ends_with("· Paused") {
        "Paused".to_owned()
    } else {
        words_that_fit(ui, detail, &font, width)
    }
}

/// The quiet menu at the right edge of a list row. It opens under its `⋯`. When it would
/// cross the right edge of the row's pane (a splitter or a scroll bar), it ends at the
/// `⋯`'s right edge instead, so it stays inside the pane.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn row_menu(ui: &mut Ui, name: &str, add: impl FnOnce(&mut Ui)) {
    let response = icon_button(ui, icons::EXPAND, &format!("More actions for {name}"));
    let popup = egui::Popup::menu(&response);
    // Egui sizes a popup on the frame it opens, before it shows it.
    let width = popup.get_expected_size().map_or(0.0, |size| size.x);
    let align = if response.rect.left() + width > ui.clip_rect().right() - S {
        egui::RectAlign::BOTTOM_END
    } else {
        egui::RectAlign::BOTTOM_START
    };
    popup.align(align).show(add);
}

/// The visible reason beside a blocked control. A tooltip is only a second route. Its wrapped
/// words never spread: they are laid out in a column that does not justify, also inside a
/// justified layout such as a menu's (M-P0.26; egui takes a label's justification from its
/// layout).
pub fn blocked_reason(ui: &mut Ui, reason: Option<&str>) {
    if let Some(reason) = reason {
        let label = egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted)).wrap();
        // Only a justified layout changes: a row, a right-to-left save row and a column keep
        // the reason where it always was.
        if ui.layout().horizontal_justify() {
            ui.scope_builder(
                egui::UiBuilder::new().layout(Layout::top_down(Align::Min)),
                |ui| ui.add(label),
            );
        } else {
            ui.add(label);
        }
    }
}

/// A note under the menu choice drawn just before it: why it is blocked, or what it does
/// (M-P0.26; the Encounters menus' `Already first.`, the Timeline table menu's `Clear the
/// results first.`). It starts at the choice's words, in by the quiet action's padding `S` and,
/// with `icon_slot`, by the icon slot the choice or its menu keeps; it ends `S` before the menu's
/// edge and wraps there, in a column that never justifies its words. The menu asks for its width
/// with it, so the note keeps the menu's right margin.
pub fn choice_note(ui: &mut Ui, icon_slot: bool, note: Option<&str>) {
    if note.is_none() {
        return;
    }
    let inset = S + if icon_slot { ICON_SLOT } else { 0.0 };
    let room = ui.available_rect_before_wrap();
    let column = Rect::from_min_max(
        pos2(room.left() + inset, room.top()),
        pos2((room.right() - S).max(room.left() + inset), room.bottom()),
    );
    let note = ui
        .scope_builder(
            egui::UiBuilder::new()
                .max_rect(column)
                .layout(Layout::top_down(Align::Min)),
            |ui| blocked_reason(ui, note),
        )
        .response
        .rect;
    // The menu takes its width from what it holds: the note keeps the padding `S` after it.
    ui.expand_to_include_x(note.right() + S);
}

/// Reserve the primary action for the run strip while another page is open.
pub fn reserve_primary(ctx: &egui::Context, reserved: bool) {
    ctx.data_mut(|data| data.insert_temp(Id::new("primary_reserved"), reserved));
}

/// A hairline across the available width, with no extra space around it.
pub fn rule(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.center().y,
        Stroke::new(HAIRLINE_WIDTH, colors().hairline),
    );
}

/// A small group heading in capitals.
pub fn group_label(ui: &mut Ui, text: &str) -> Response {
    ui.label(
        Type::Label
            .rich(ui, text.to_uppercase())
            .color(colors().muted),
    )
}

/// A top-level destination. The icon supports the word.
pub fn nav_item(
    ui: &mut Ui,
    icon: &str,
    label: &str,
    detail: Option<&str>,
    selected: bool,
) -> Response {
    let c = colors();
    let height = if detail.is_some() {
        Density::Glance.control() + XS
    } else {
        Density::Scan.control()
    };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    let name = detail.map_or_else(|| label.to_owned(), |d| format!("{label} · {d}"));
    response
        .widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), selected, &name));
    if ui.is_rect_visible(rect) {
        let fill = if selected {
            c.selected
        } else if response.hovered() {
            c.hover
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, corner(RADIUS_CONTROL), fill);
        if selected {
            let bar = Rect::from_min_size(rect.min, vec2(3.0, rect.height()));
            ui.painter().rect_filled(bar, corner(1.0), c.accent);
        }
        let color = if selected { c.text } else { c.muted };
        let center = if detail.is_some() {
            rect.top() + height * 0.35
        } else {
            rect.center().y
        };
        ui.painter().text(
            pos2(rect.left() + S + 2.0, center),
            Align2::LEFT_CENTER,
            icon,
            Type::Body.font(ui),
            color,
        );
        let font = if selected { Type::Strong } else { Type::Body };
        let galley = one_line(ui, label, font.font(ui), color, rect.width() - 34.0);
        ui.painter().galley(
            pos2(rect.left() + 28.0, center - galley.size().y / 2.0),
            galley,
            color,
        );
        if let Some(detail) = detail {
            let galley = one_line(
                ui,
                detail,
                Type::Small.font(ui),
                c.muted,
                rect.width() - 34.0,
            );
            ui.painter().galley(
                pos2(rect.left() + 28.0, rect.top() + height * 0.56),
                galley,
                c.muted,
            );
        }
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response
}

/// A paused run's name may shorten in the rail; hover and accessibility keep it whole.
pub fn paused_run_nav(ui: &mut Ui, fights: usize, name: &str, state: &str) -> Response {
    let label = if fights == 1 {
        "Paused run"
    } else {
        "Paused runs"
    };
    let detail = if fights == 1 {
        name.to_owned()
    } else {
        count(fights, "fight", "fights")
    };
    let full = if fights == 1 {
        format!("{name} · {state}")
    } else {
        detail.clone()
    };
    let width = (ui.available_width() - S * 4.0 - XS).max(S);
    let mut fitted = detail.clone();
    while !fitted.is_empty() && number_width(ui, &fitted, &Type::Small.font(ui)) > width {
        fitted.pop();
    }
    if fitted != detail {
        while !fitted.is_empty()
            && number_width(ui, &format!("{fitted}…"), &Type::Small.font(ui)) > width
        {
            fitted.pop();
        }
        fitted.push('…');
    }
    let response = nav_item(ui, icons::RUN, label, Some(&fitted), false);
    named(ui, &response, &format!("{label} · {full}"));
    ui.ctx()
        .accesskit_node_builder(response.id, |node| node.set_description(full.clone()));
    response.on_hover_text(full)
}

/// A rail control that opens a panel beside the page. It is not a place, so it never
/// uses a navigation bar: an open panel shows the selected surface and the word Open.
pub fn nav_toggle(ui: &mut Ui, icon: &str, label: &str, key: &str, open: bool) -> Response {
    let c = colors();
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), Density::Scan.control()),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        if open || response.hovered() {
            ui.painter().rect_filled(
                rect,
                corner(RADIUS_CONTROL),
                if open { c.selected } else { c.hover },
            );
        }
        let color = if open { c.text } else { c.muted };
        ui.painter().text(
            pos2(rect.left() + S + 2.0, rect.center().y),
            Align2::LEFT_CENTER,
            icon,
            Type::Body.font(ui),
            color,
        );
        ui.painter().text(
            pos2(rect.left() + 28.0, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            Type::Body.font(ui),
            color,
        );
        ui.painter().text(
            pos2(rect.right() - XS, rect.center().y),
            Align2::RIGHT_CENTER,
            if open { "Open" } else { key },
            Type::Small.font(ui),
            if open { c.text } else { c.faint },
        );
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response
}

pub fn focus_ring(ui: &Ui, rect: Rect) {
    ui.painter().rect_stroke(
        rect,
        corner(RADIUS_CONTROL),
        Stroke::new(FOCUS_WIDTH, colors().focus),
        StrokeKind::Inside,
    );
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Primary,
    Secondary,
    Quiet,
    Destructive,
}

/// A button. Say why it is disabled with `blocked` (G1). egui hides hover text
/// on disabled widgets unless it is added through `on_disabled_hover_text`.
#[must_use = "call show"]
pub struct Action<'a> {
    label: &'a str,
    name: Option<&'a str>,
    kind: Kind,
    icon: Option<&'a str>,
    blocked: Option<&'a str>,
    hint: Option<&'a str>,
    height: f32,
    selected: bool,
    key: Option<&'a str>,
    plain: bool,
    line_start: bool,
    /// A `line_start` Undo whose target stays in its message's column: the row's left edge.
    column: bool,
    min_width: Option<f32>,
    /// Its words wrap onto more lines in the room it has, and it grows taller, in place of an
    /// ellipsis (a choice whose fixed words must all show).
    wraps: bool,
}

/// How far a quiet `line_start` action hangs left of the text edge: its padding.
pub const LINE_START_HANG: f32 = S;

/// The most a `line_start` action hangs left of the text edge: its padding and its icon slot (as
/// `Action::paint` lays it out). A pane's body clip reaches this far into the margin left of
/// the pane, inside the parent's clip (`pane::body_clip`).
pub const LINE_START_REACH: f32 = LINE_START_HANG + super::tokens::ICON_SLOT;

impl<'a> Action<'a> {
    fn new(kind: Kind, label: &'a str) -> Self {
        Self {
            label,
            name: None,
            kind,
            icon: None,
            blocked: None,
            hint: None,
            height: Density::Scan.control(),
            selected: false,
            key: None,
            plain: false,
            line_start: false,
            column: false,
            min_width: None,
            wraps: false,
        }
    }

    /// Its words wrap in the room it has, and it grows taller, instead of being cut: an action
    /// whose words must all show (a repair that names a file). It keeps its padding, border and
    /// target; words that fit stay on one line.
    pub fn wraps(mut self) -> Self {
        self.wraps = true;
        self
    }

    pub fn primary(label: &'a str) -> Self {
        Self::new(Kind::Primary, label)
    }
    pub fn secondary(label: &'a str) -> Self {
        Self::new(Kind::Secondary, label)
    }
    /// In body weight, for an action repeated on every row of a list.
    pub fn plain(mut self) -> Self {
        self.plain = true;
        self
    }

    pub fn quiet(label: &'a str) -> Self {
        Self::new(Kind::Quiet, label)
    }

    /// The step back through a reader's history: `Back`, quiet at `Read`, its arrow before the
    /// word, and its key `Alt+Left` after it when `key`. One form for every reader (the
    /// Library's toolbar, Lookup's title line); the arrow never stands alone.
    ///
    /// ```
    /// let _ = splat_design::layout::doc_parts::Action::history_back(true);
    /// ```
    ///
    /// The arrow is the design system's: a caller cannot place it, and the old bare back arrow
    /// is gone.
    ///
    /// ```compile_fail,E0603
    /// let _ = splat_design::layout::doc_parts::icons::HISTORY_BACK;
    /// ```
    /// ```compile_fail,E0425
    /// let _ = splat_design::layout::doc_parts::icons::LEGACY_BACK;
    /// ```
    pub fn history_back(key: bool) -> Self {
        let back = Self::quiet("Back")
            .icon(super::icons::HISTORY_BACK)
            .density(Density::Read);
        if key { back.key("Alt+Left") } else { back }
    }
    /// Align a quiet action's words with the text above it. Its padding, and any icon, hang left
    /// of that edge. Only an action inside a message has an icon when it starts a line: the icon
    /// then sits in the message's icon column. Outside a message, give it no icon, so its target
    /// reaches only `S` into the pane's gutter.
    pub fn line_start(mut self) -> Self {
        self.line_start = true;
        self
    }

    /// `line_start` for an Undo on its own line under its message: its target stays in the
    /// message's column, which starts at the row's left edge.
    pub(super) fn line_start_in_column(mut self) -> Self {
        self.line_start = true;
        self.column = true;
        self
    }

    pub fn destructive(label: &'a str) -> Self {
        Self::new(Kind::Destructive, label)
    }
    pub fn icon(mut self, icon: &'a str) -> Self {
        self.icon = Some(icon);
        self
    }
    /// The accessible name, when the visible text is shorter than the meaning.
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }
    /// Disable this action and state the reason.
    pub fn blocked(mut self, reason: Option<&'a str>) -> Self {
        self.blocked = reason;
        self
    }
    pub fn blocked_if(mut self, condition: bool, reason: &'a str) -> Self {
        if condition {
            self.blocked = Some(reason);
        }
        self
    }
    pub fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }
    pub fn density(mut self, density: Density) -> Self {
        self.height = density.control();
        self
    }
    /// Show the keyboard command beside the label, so it can be learned at the table.
    pub fn key(mut self, key: &'a str) -> Self {
        self.key = (!key.is_empty()).then_some(key);
        self
    }
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// A quiet action that ends a line of text, at the line's height (`XL`), so that the line
    /// keeps the height it has without the action.
    pub fn in_line(mut self) -> Self {
        self.height = XL;
        self
    }

    /// Keep a repeated action as wide as its shared column.
    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    pub fn show(mut self, ui: &mut Ui) -> Response {
        if self.kind == Kind::Primary
            && (self.selected
                || ui.ctx().data(|data| {
                    data.get_temp::<bool>(Id::new("primary_reserved"))
                        .unwrap_or(false)
                }))
        {
            self.kind = Kind::Secondary;
        }
        let enabled = self.blocked.is_none();
        // `add_enabled` keeps the parent's layout, so wrapped rows still wrap.
        let mut response = ui.add_enabled(enabled, |ui: &mut Ui| self.paint(ui));
        if let Some(reason) = self.blocked {
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_description(reason);
            });
            response = response.on_disabled_hover_text(reason);
        }
        if let Some(hint) = self.hint {
            response = response.on_hover_text(hint);
        }
        locked_control(ui, &response);
        response
    }

    fn font(&self) -> Type {
        if self.kind == Kind::Primary {
            Type::Heading
        } else if self.plain {
            Type::Body
        } else {
            Type::Strong
        }
    }

    fn padding(&self) -> f32 {
        if self.kind == Kind::Quiet {
            LINE_START_HANG
        } else {
            M
        }
    }

    fn icon_width(&self) -> f32 {
        self.icon.map_or(0.0, |_| ICON_SLOT)
    }

    /// What its natural width is made of (`natural_width`), apart from the fonts and the scale.
    fn width_facts(&self, state: &mut impl std::hash::Hasher) {
        use std::hash::Hash;
        self.label.hash(state);
        self.key.hash(state);
        self.icon.is_some().hash(state);
        (
            self.kind == Kind::Primary,
            self.kind == Kind::Quiet,
            self.plain,
        )
            .hash(state);
        self.height.to_bits().hash(state);
    }

    /// The width before any text is cut, using the same parts as the painted button.
    pub fn natural_width(&self, ui: &Ui) -> f32 {
        #[cfg(any(test, feature = "test-support"))]
        text_measured();
        let icon = self.icon_width();
        let key = self
            .key
            .map_or(0.0, |key| number_width(ui, key, &Type::Small.font(ui)) + S);
        (number_width(ui, self.label, &self.font().font(ui)) + 2.0 * self.padding() + icon + key)
            .max(self.height)
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn paint(&self, ui: &mut Ui) -> Response {
        let c = colors();
        let font = self.font().font(ui);
        let padding = self.padding();
        let icon_width = self.icon_width();
        let key_room = self
            .key
            .map_or(0.0, |key| number_width(ui, key, &Type::Small.font(ui)) + S);
        let room = ui.available_width() - 2.0 * padding - icon_width - key_room;
        let foreground = match self.kind {
            Kind::Primary if !ui.is_enabled() => c.muted,
            Kind::Primary => c.on_accent,
            Kind::Secondary | Kind::Quiet => c.text,
            Kind::Destructive => c.destructive,
        };
        let galley = if self.wraps {
            let mut job = LayoutJob::simple(
                self.label.replace(['\n', '\r'], " "),
                font,
                foreground,
                room.max(1.0),
            );
            job.wrap.break_anywhere = false;
            ui.fonts_mut(|fonts| fonts.layout_job(job))
        } else {
            one_line(ui, self.label, font, foreground, room)
        };
        // A wrapped label grows the control by its extra lines; one line keeps its height.
        let height = if self.wraps {
            self.height.max(galley.size().y + 2.0 * XS)
        } else {
            self.height
        };
        let key = self.key.map(|key| {
            one_line(
                ui,
                key,
                Type::Small.font(ui),
                foreground.gamma_multiply(0.75),
                60.0,
            )
        });
        let key_width = key.as_ref().map_or(0.0, |key| key.size().x + S);
        let width = (galley.size().x + 2.0 * padding + icon_width + key_width)
            .max(self.height)
            .max(self.min_width.unwrap_or_default());
        // The padding before the icon and the words: less only for a `line_start` hang that
        // the clip leaves short (below).
        let mut left_padding = padding;
        let (rect, response) = if self.kind == Kind::Quiet && self.line_start {
            // The words start at the text edge; the padding and any icon hang in the gutter. The
            // row allocates all but the icon, so the next item keeps the same gap after the
            // target as it has after a `line_start` action without an icon.
            // P3a: the action is whole in its room (`pane::hang_limit`: this `Ui`'s clip, its
            // pane column's surface, and every band a layout part reserved across its hang).
            // Where the room is shorter than the hang, the action first gives up its left
            // padding (blank but for the hover fill), so its words stay at the text edge (a
            // menu's choices under its heading); then it moves its icon and words by what is
            // still missing, also past the text edge when the room starts there.
            let hang = padding + icon_width;
            let start = ui.cursor().min;
            let limit = super::pane::hang_limit(
                ui,
                Rect::from_min_max(
                    pos2(start.x - hang, start.y),
                    pos2(start.x, start.y + self.height),
                ),
            );
            let short = (limit - (start.x - hang)).max(0.0);
            let given = short.min(padding);
            let shift = short - given;
            let size = vec2(width - icon_width + shift, self.height);
            let (id, allocated) = ui.allocate_space(size);
            let aligned = ui.layout().align_size_within_rect(size, allocated);
            left_padding = padding - given;
            let rect = Rect::from_min_size(
                pos2(aligned.left() - hang + short, aligned.top()),
                vec2(width - given, self.height),
            );
            // An Undo under its message keeps its target in the message's column, so it never
            // hangs past a pane whose text edge is the message's (Lookup's reader). The icon
            // and the words keep their places.
            let target = if self.column {
                rect.with_min_x(rect.min.x.max(ui.max_rect().left()))
            } else {
                rect
            };
            // Ruling R19: its press area is its painted area inside its room, registered once
            // for clicks; egui's nearest-target rule applies to it as to every control. It is
            // never inside a reserved band (`hang_limit`), so a press directly on a splitter
            // band or a scroll lane is that control's direct hit.
            let room = ui.clip_rect().with_min_x(ui.clip_rect().left().max(limit));
            let clip = ui.clip_rect();
            ui.set_clip_rect(room);
            let response = ui.interact(target, id, Sense::click());
            ui.set_clip_rect(clip);
            (rect, response)
        } else {
            ui.allocate_exact_size(vec2(width, height), Sense::click())
        };
        let name = self.name.unwrap_or(self.label);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), name));
        if !ui.is_rect_visible(rect) {
            return response;
        }
        let hovered = response.hovered() && ui.is_enabled();
        let pressed = response.is_pointer_button_down_on() && ui.is_enabled();
        let plain_fill = if pressed {
            c.selected
        } else if hovered {
            c.hover
        } else {
            Color32::TRANSPARENT
        };
        let (fill, stroke) = match self.kind {
            _ if self.selected => (c.selected, Stroke::new(HAIRLINE_WIDTH, c.field_border)),
            // Disabled, it loses its fill, so it cannot outweigh what can be pressed.
            Kind::Primary if !ui.is_enabled() => (
                Color32::TRANSPARENT,
                Stroke::new(HAIRLINE_WIDTH, c.field_border),
            ),
            Kind::Primary => (
                if hovered {
                    c.accent_hover
                } else {
                    c.accent_fill
                },
                Stroke::NONE,
            ),
            Kind::Secondary => (plain_fill, Stroke::new(HAIRLINE_WIDTH, c.field_border)),
            Kind::Quiet => (plain_fill, Stroke::NONE),
            Kind::Destructive => (plain_fill, Stroke::new(HAIRLINE_WIDTH, c.destructive)),
        };
        ui.painter().rect(
            response.rect,
            corner(RADIUS_CONTROL),
            fill,
            stroke,
            StrokeKind::Inside,
        );
        let mut x = rect.left() + left_padding;
        if let Some(icon) = self.icon {
            ui.painter().text(
                pos2(x, rect.center().y),
                Align2::LEFT_CENTER,
                icon,
                Type::Body.font(ui),
                foreground,
            );
            x += icon_width;
        }
        let label_width = galley.size().x;
        ui.painter().galley(
            pos2(x, rect.center().y - galley.size().y / 2.0),
            galley,
            foreground,
        );
        if let Some(key) = key {
            ui.painter().galley(
                pos2(x + label_width + S, rect.center().y - key.size().y / 2.0),
                key,
                foreground,
            );
        }
        if response.has_focus() {
            focus_ring(ui, rect);
        }
        response
    }
}

/// An action with a default, and a caret beside it that opens a menu of the other ways
/// to do it. A click in the menu closes it. The response is the action's own.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn split_action(
    ui: &mut Ui,
    action: Action<'_>,
    menu_name: &str,
    menu: impl FnOnce(&mut Ui),
) -> Response {
    let caret = Action {
        height: action.height,
        blocked: action.blocked,
        ..Action::secondary(icons::DOWN)
            .name(menu_name)
            .hint(menu_name)
    };
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.x = HAIRLINE_WIDTH;
        // The caret stays on the right in either direction of layout.
        let (response, caret) = if ui.layout().prefer_right_to_left() {
            let caret = caret.show(ui);
            (action.show(ui), caret)
        } else {
            let response = action.show(ui);
            (response, caret.show(ui))
        };
        egui::Popup::menu(&caret)
            .align(egui::RectAlign::BOTTOM_END)
            .show(menu);
        response
    })
    .inner
}

/// A square button with one icon. The word is the accessible name and the tooltip.
pub fn icon_button(ui: &mut Ui, icon: &str, name: &str) -> Response {
    icon_button_with_hint(ui, icon, name, name)
}

/// An `icon_button` whose tooltip says more than its name, such as its keyboard command.
pub fn icon_button_with_hint(ui: &mut Ui, icon: &str, name: &str, hint: &str) -> Response {
    hinted_icon_button(ui, icon, name, hint, None)
}

/// A view toggle (`TitleLine::toggle`): an `icon_button_with_hint` that also tells assistive
/// tools whether it is on, as a toggle button's pressed state, so they need not read it from the
/// name.
pub(crate) fn icon_toggle(ui: &mut Ui, icon: &str, name: &str, hint: &str, on: bool) -> Response {
    hinted_icon_button(ui, icon, name, hint, Some(on))
}

fn hinted_icon_button(
    ui: &mut Ui,
    icon: &str,
    name: &str,
    hint: &str,
    pressed: Option<bool>,
) -> Response {
    let (rect, response) =
        ui.allocate_exact_size(Vec2::splat(Density::Read.control()), Sense::click());
    response.widget_info(|| match pressed {
        Some(on) => WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), on, name),
        None => WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), name),
    });
    locked_control(ui, &response);
    if ui.is_rect_visible(rect) {
        let c = colors();
        if response.hovered() {
            ui.painter()
                .rect_filled(rect, corner(RADIUS_CONTROL), c.hover);
        }
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            icon,
            Type::Heading.font(ui),
            if response.hovered() { c.text } else { c.muted },
        );
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_text(hint)
}

/// A whole number that the DM can change. The value is in a well with a field border,
/// as every input is, with − and + beside it. Drag, type or step it. Plain text beside a
/// stepper is a value that cannot change, so the two never look alike. Returns true when
/// the value changed. A value outside the range widens it: the stepper never changes a
/// value that the DM did not touch.
pub fn stepper(
    ui: &mut Ui,
    name: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
) -> bool {
    stepper_with_density(ui, name, value, range, Density::Read, None, None)
}

/// The shared stepper at the control height of its row. At a limit, the disabled − or + says
/// why on hover and in its accessible description when the caller gives that limit's reason
/// (`low_reason`, `high_reason`). With None, the button is only disabled. An enclosing lock's
/// reason comes first.
pub fn stepper_with_density(
    ui: &mut Ui,
    name: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
    density: Density,
    low_reason: Option<&str>,
    high_reason: Option<&str>,
) -> bool {
    stepper_as(
        ui,
        name,
        value,
        (range, density, low_reason, high_reason),
        Shown::Plain,
    )
}

/// A stepper whose value is a modifier: it shows `+2` and `−1`, with a true minus sign
/// (`signed`), and reads back what the DM types in either form (`−3`, `-3`, `+2`, `2`). The
/// same − and + and limits as [`stepper`] (the Run's Correct tab: an initiative bonus).
pub fn signed_stepper(
    ui: &mut Ui,
    name: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
) -> bool {
    stepper_as(
        ui,
        name,
        value,
        (range, Density::Read, None, None),
        Shown::Signed,
    )
}

/// A stepper whose value is a count that can reach the thousands (a group's member count): it
/// shows `1,000` with its digits grouped, as every count in the app reads (`count`), and reads
/// back what the DM types with or without the commas (`1,000`, `1000`). The same − and + and
/// limits as [`stepper`].
pub fn grouped_stepper(
    ui: &mut Ui,
    name: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
) -> bool {
    stepper_as(
        ui,
        name,
        value,
        (range, Density::Read, None, None),
        Shown::Grouped,
    )
}

/// How a stepper's value reads.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Shown {
    /// `3`
    Plain,
    /// `+3`, `−1`: a modifier.
    Signed,
    /// `1,000`: a count with its digits grouped.
    Grouped,
}

/// A count with its digits grouped (`1,000`), a negative one with a true minus.
fn grouped_value(value: i64) -> String {
    let digits = grouped_number(value.unsigned_abs() as usize);
    if value < 0 {
        format!("\u{2212}{digits}")
    } else {
        digits
    }
}

/// A count as the DM may type it: `1,000`, `1000`, `1 000`, `−3`.
fn parse_grouped(text: &str) -> Option<f64> {
    text.trim()
        .replace('\u{2212}', "-")
        .chars()
        .filter(|c| *c != ',' && !c.is_whitespace())
        .collect::<String>()
        .parse()
        .ok()
}

/// The stepper's read-back of a signed value, through [`parse_signed`].
fn parse_signed_value(text: &str) -> Option<f64> {
    parse_signed(text).map(|value| value as f64)
}

/// The one stepper: its value in the form `shown`, its range, density and limits' reasons.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn stepper_as(
    ui: &mut Ui,
    name: &str,
    value: &mut i64,
    (range, density, low_reason, high_reason): (
        std::ops::RangeInclusive<i64>,
        Density,
        Option<&str>,
        Option<&str>,
    ),
    shown: Shown,
) -> bool {
    let button = |ui: &mut Ui, icon: &str, name: &str| {
        if density == Density::Read {
            return icon_button(ui, icon, name);
        }
        let (rect, response) =
            ui.allocate_exact_size(Vec2::splat(density.control()), Sense::click());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), name));
        locked_control(ui, &response);
        if ui.is_rect_visible(rect) {
            let c = colors();
            if response.hovered() {
                ui.painter()
                    .rect_filled(rect, corner(RADIUS_CONTROL), c.hover);
            }
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                icon,
                Type::Heading.font(ui),
                if response.hovered() { c.text } else { c.muted },
            );
            if response.has_focus() {
                focus_ring(ui, rect);
            }
        }
        response.on_hover_text(name)
    };
    let before = *value;
    let (low, high) = ((*range.start()).min(before), (*range.end()).max(before));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = XS / 2.0;
        // A button at its limit is disabled. When its caller says why, the button says so on
        // hover and in its description, unless an enclosing lock already gives its own reason.
        let limited = |ui: &mut Ui, open: bool, reason: Option<&str>, icon: &str, label: &str| {
            let response = ui.add_enabled_ui(open, |ui| button(ui, icon, label)).inner;
            match reason.filter(|_| !open && ui.is_enabled()) {
                Some(reason) => {
                    ui.ctx().accesskit_node_builder(response.id, |node| {
                        node.set_description(reason.to_owned());
                    });
                    response.on_disabled_hover_text(reason)
                }
                None => response,
            }
        };
        let down = limited(
            ui,
            *value > low,
            low_reason,
            icons::SUBTRACT,
            &format!("Decrease {name}"),
        );
        if down.clicked() {
            *value -= 1;
        }
        let field = ui
            .scope(|ui| {
                let c = colors();
                let widgets = &mut ui.visuals_mut().widgets;
                for state in [
                    &mut widgets.inactive,
                    &mut widgets.hovered,
                    &mut widgets.active,
                ] {
                    state.bg_fill = c.well;
                    state.weak_bg_fill = c.well;
                }
                widgets.inactive.bg_stroke = Stroke::new(HAIRLINE_WIDTH, c.field_border);
                ui.spacing_mut().interact_size.y = density.control();
                // The field's arrow keys step by one whole number (issue #633). egui's drag
                // value steps by its drag speed, 0.1, which a whole number rounds back. The keys
                // are taken here, before the field reads them, and the field's open words go,
                // so it shows the new number.
                let id = ui.next_auto_id();
                if ui.is_enabled() && ui.memory(|memory| memory.has_focus(id)) {
                    let step = ui.input_mut(|input| {
                        input.count_and_consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
                            as i64
                            - input
                                .count_and_consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
                                as i64
                    });
                    if step != 0 {
                        *value = (*value + step).clamp(low, high);
                        ui.data_mut(|data| data.remove::<String>(id));
                    }
                }
                let mut field = egui::DragValue::new(value).range(low..=high).speed(0.1);
                match shown {
                    Shown::Plain => {}
                    Shown::Signed => {
                        field = field
                            .custom_formatter(|value, _| signed(value as i64))
                            .custom_parser(parse_signed_value);
                    }
                    Shown::Grouped => {
                        field = field
                            .custom_formatter(|value, _| grouped_value(value as i64))
                            .custom_parser(parse_grouped);
                    }
                }
                ui.add(field)
            })
            .inner;
        named(ui, &field, name);
        let up = limited(
            ui,
            *value < high,
            high_reason,
            icons::ADD,
            &format!("Increase {name}"),
        );
        if up.clicked() {
            *value += 1;
        }
    });
    *value = (*value).clamp(low, high);
    *value != before
}

/// An inline link with a real `Link` role, and a target of at least 24 px.
pub fn link(ui: &mut Ui, label: &str) -> Response {
    text_link(ui, label, Type::Body)
}

/// A Small inline link with the same target and accessible role as the body link.
pub fn small_link(ui: &mut Ui, label: &str) -> Response {
    text_link(ui, label, Type::Small)
}

/// A body link whose target is one `Read` control high: a title line's link action
/// (`TitleAction::link`), level with the actions beside it (M-Y0.2).
pub fn title_link(ui: &mut Ui, label: &str) -> Response {
    link_of_height(ui, label, Type::Body, Density::Read.control())
}

fn text_link(ui: &mut Ui, label: &str, role: Type) -> Response {
    link_of_height(ui, label, role, XL)
}

/// A one-line link whose target is at least `XL` wide and at least `height` high.
fn link_of_height(ui: &mut Ui, label: &str, role: Type, height: f32) -> Response {
    let c = colors();
    let room = ui.available_width().max(1.0);
    let galley = one_line(ui, label, role.font(ui), c.accent, room);
    // The press target is the link's own space in the layout, at least `XL` wide. A short
    // word such as `Ox` sits at its left, and the next item starts after the whole target,
    // so two short links side by side never share a press area.
    let (target, response) = ui.allocate_exact_size(
        vec2(galley.size().x.max(XL), galley.size().y.max(height)),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, ui.is_enabled(), label));
    if ui.is_rect_visible(target) {
        let word = Rect::from_min_size(target.min, vec2(galley.size().x, target.height()));
        let y = word.center().y - galley.size().y / 2.0;
        let underline = y + galley.size().y - 1.0;
        let color = if response.hovered() {
            c.focus
        } else {
            c.accent
        };
        ui.painter().hline(
            word.x_range(),
            underline,
            Stroke::new(1.0, color.gamma_multiply(0.6)),
        );
        ui.painter().galley(pos2(word.left(), y), galley, color);
        if response.has_focus() {
            focus_ring(ui, target);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A link that wraps, for text of unknown length such as a title an LLM wrote.
pub fn wrapped_link(ui: &mut Ui, label: &str) -> Response {
    let c = colors();
    let room = ui.available_width().max(1.0);
    let text = Type::Body.rich(ui, label).color(c.accent).underline();
    let galley = egui::WidgetText::from(text).into_galley(
        ui,
        Some(egui::TextWrapMode::Wrap),
        room,
        egui::TextStyle::Body,
    );
    let (rect, response) = ui.allocate_exact_size(
        vec2(galley.size().x, galley.size().y.max(24.0)),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        ui.painter()
            .galley(pos2(rect.left(), top), galley, c.accent);
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Small words that wrap and can be pressed, with a quiet label before them. It is a link to
/// the page the words are from, and it reads as that page's text and not as a control.
pub fn line_link(ui: &mut Ui, label: &str, words: &str) -> Response {
    let c = colors();
    let name = format!("{label}{words}");
    let mut job = LayoutJob::default();
    job.wrap.max_width = ui.available_width().max(1.0);
    for (part, color) in [(label, c.muted), (words, c.text)] {
        job.append(
            part,
            0.0,
            egui::TextFormat {
                font_id: Type::Small.font(ui),
                line_height: Some(Type::Small.line()),
                color,
                ..Default::default()
            },
        );
    }
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, response) = ui.allocate_exact_size(
        vec2(galley.size().x, galley.size().y.max(24.0)),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, ui.is_enabled(), &name));
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        ui.painter().galley(pos2(rect.left(), top), galley, c.text);
        if response.hovered() {
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 1.0,
                Stroke::new(1.0, c.focus.gamma_multiply(0.6)),
            );
        }
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A label above a control, with help and an error below it.
pub fn field<R>(
    ui: &mut Ui,
    label: &str,
    help: Option<&str>,
    error: Option<&str>,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    field_label(ui, label, Type::Small, (None, help, error), add)
}

/// A form field whose value something needs now (layout PR 10c, round 31 SR4/AR4): `needs`, when
/// given, is what the DM must enter, drawn directly below the control as a warning (its icon in
/// the warning colour, then the words), before the field's own `help`. Advice stays help; a
/// missing value is never drawn as help. The field block (`field_label`) holds the warning too.
pub fn small_field_needing<R>(
    ui: &mut Ui,
    label: &str,
    needs: Option<&str>,
    help: Option<&str>,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    field_label(ui, label, Type::Small, (needs, help, None), add)
}

/// A form field with the same sentence-case label as `field`.
pub fn small_field<R>(
    ui: &mut Ui,
    label: &str,
    help: Option<&str>,
    error: Option<&str>,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    field_label(ui, label, Type::Small, (None, help, error), add)
}

/// A field block (layout PR 10): its caption, its control, its help and its error. A focused
/// control of the block is brought into view with the whole block (`reveal_with`), so the pane's
/// focus arbiter never leaves the caption or the help cut by the footer or the view's edge.
fn field_label<R>(
    ui: &mut Ui,
    label: &str,
    role: Type,
    (warning, help, error): (Option<&str>, Option<&str>, Option<&str>),
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    let top = ui.cursor().top();
    let layer = ui.layer_id();
    let count = |ui: &Ui| {
        ui.ctx()
            .viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count())
    };
    let first = count(ui);
    field_caption(ui, label, role);
    let controls = count(ui);
    // The block's top while its control draws (`field_top`); the field around it, if any, after.
    let pass = ui.ctx().cumulative_pass_nr();
    let outer = ui.ctx().data_mut(|data| {
        let outer = data
            .get_temp::<Option<(u64, egui::LayerId, f32)>>(Id::new(FIELD_TOP))
            .flatten();
        data.insert_temp(Id::new(FIELD_TOP), Some((pass, layer, top)));
        outer
    });
    let inner = add(ui);
    ui.ctx()
        .data_mut(|data| data.insert_temp(Id::new(FIELD_TOP), outer));
    let last = count(ui);
    let bottom = field_feedback(ui, warning, help, error);
    // A warning is the field's own: its controls carry it as their accessible description, so a
    // screen reader says it when the empty field takes the focus (layout PR 10d). Only while a
    // warning shows: no work for a field without one.
    if let Some(warning) = warning {
        let ids: Vec<Id> = ui.ctx().viewport(|viewport| {
            viewport
                .this_pass
                .widgets
                .get_layer(layer)
                .skip(controls)
                .take(last.saturating_sub(controls))
                .map(|widget| widget.id)
                .collect()
        });
        for id in ids {
            ui.ctx()
                .accesskit_node_builder(id, |node| node.set_description(warning));
        }
    }
    // Only a focused control of this block, or the control whose list is open: one or two
    // lookups, nothing that grows with the form.
    let inside = |id: Id| {
        ui.ctx().viewport(|viewport| {
            viewport
                .this_pass
                .widgets
                .order(id)
                .is_some_and(|(at_layer, at)| at_layer == layer && (first..last).contains(&at))
        })
    };
    let subject = ui
        .memory(|memory| memory.focused())
        .filter(|id| inside(*id))
        .or_else(|| open_owner(ui.ctx()).filter(|id| inside(*id)));
    if let Some(focused) = subject {
        let span = ui.max_rect();
        reveal_with(
            ui,
            focused,
            Rect::from_min_max(pos2(span.left(), top), pos2(span.right(), bottom)),
        );
    }
    inner
}

fn field_caption(ui: &mut Ui, label: &str, role: Type) {
    let c = colors();
    ui.spacing_mut().item_spacing.y = XS;
    let label = if matches!(role, Type::Label) {
        label.to_uppercase()
    } else {
        label.to_owned()
    };
    ui.label(role.rich(ui, label).color(c.muted));
}

/// The field's help and error, then its own space: `L` from the last line of help or error to the
/// next thing, so help never reads as the next field's (N3); `XS` more when it has neither. A
/// caller adds no space after a field.
///
/// Returns the block's bottom: its last line of help or error, else its control's.
fn field_feedback(
    ui: &mut Ui,
    warning: Option<&str>,
    help: Option<&str>,
    error: Option<&str>,
) -> f32 {
    let mut last = None;
    if let Some(warning) = warning {
        last = Some(tone_message(ui, Tone::Warning, warning).rect.bottom());
    }
    if let Some(text) = help {
        last = Some(self::help(ui, text).rect.bottom());
    }
    if let Some(error) = error {
        last = Some(tone_message(ui, Tone::Error, error).rect.bottom());
    }
    match last {
        Some(bottom) => {
            space(ui, bottom + L - ui.cursor().top());
            bottom
        }
        None => {
            let bottom = ui.min_rect().bottom();
            space(ui, XS);
            bottom
        }
    }
}

/// A one-line text field with a label above it.
pub fn text_field(
    ui: &mut Ui,
    label: &str,
    text: &mut String,
    help: Option<&str>,
    error: Option<&str>,
) -> Response {
    text_field_label(ui, label, Type::Small, text, help, error)
}

#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn text_field_label(
    ui: &mut Ui,
    label: &str,
    role: Type,
    text: &mut String,
    help: Option<&str>,
    error: Option<&str>,
) -> Response {
    field_label(ui, label, role, (None, help, error), |ui| {
        let width = ui.available_width().min(FORM_WIDTH) - 2.0 * S;
        let response = ui.add(
            egui::TextEdit::singleline(text)
                .desired_width(width)
                .min_size(vec2(0.0, Density::Read.control()))
                .margin(vec2(S, 5.0)),
        );
        named(ui, &response, label);
        response
    })
}

/// Placeholder text for any text field: `Body` size in the faint colour.
pub fn placeholder(ui: &Ui, hint: &str) -> egui::RichText {
    Type::Body.rich(ui, hint).color(colors().faint)
}

/// A short text field with a target of at least 26 px, and an accessible name.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn text_input(ui: &mut Ui, name: &str, text: &mut String, hint: &str, width: f32) -> Response {
    // The desired width excludes the margins.
    let response = ui.add(
        egui::TextEdit::singleline(text)
            .desired_width(width - 2.0 * S)
            .min_size(vec2(width, 26.0))
            .margin(vec2(S, 4.0))
            .hint_text(placeholder(ui, hint)),
    );
    named(ui, &response, name);
    response
}

/// A short whole-number field, with no label of its own. Letters typed into it are
/// dropped, so the run can use them as commands while the field has focus.
pub fn number_field(
    ui: &mut Ui,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
    height: f32,
) -> Response {
    number_field_with_id(ui, focus_id(name), name, text, hint, width, height)
}

/// For a field whose name another field can share, as two things with one name do.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn number_field_with_id(
    ui: &mut Ui,
    id: Id,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
    height: f32,
) -> Response {
    // The text Undo guard of the field parts: a number typed for one object never comes back
    // by Ctrl+Z in another under this id.
    let response = super::layout::guarded_field(ui, id, text, |ui, text| {
        let response = ui.add(
            egui::TextEdit::singleline(text)
                .id(id)
                .desired_width(width - 2.0 * S)
                .min_size(vec2(width, height))
                // The caret stands a little clear of the first digit.
                .margin(vec2(
                    S + 2.0,
                    (height - Type::Numeral.size() - 6.0).max(4.0) / 2.0,
                ))
                .font(Type::Numeral.font(ui))
                .hint_text(placeholder(ui, hint)),
        );
        // Inside the guard: a letter the field drops is not another text. A true minus stays,
        // so the DM can type what the app shows (`parse_signed`).
        text.retain(number_char);
        response
    });
    named(ui, &response, name);
    response
}

/// The search field of `FieldId::search`, with this id.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub(super) fn search_box(
    ui: &mut Ui,
    id: Id,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
    kept: f32,
) -> Response {
    let c = colors();
    let height = Density::Scan.control();
    // The frame is the whole row. Inside it: the icon's margin at the left, then the text area,
    // then `kept` px for `Clear` and the frame's own margin at the right. When the row is
    // narrower than all of that, the text area gives way first, then the right margin: the frame
    // never leaves the row.
    // egui's margins are whole pixels: the right one is rounded down and the text area takes
    // the rest, so the margins and the text area add up to exactly the row.
    let width = width.max(0.0);
    let left = SEARCH_LEFT.min(width).floor();
    let right = (SEARCH_RIGHT + kept).min(width - left).max(0.0).floor();
    let text_room = (width - left - right).max(0.0);
    // The hint keeps whole words inside the text area: a narrow field cuts it with an
    // ellipsis, never past the area. It shows only while the field is empty, so it is measured
    // only then.
    let hint = if text.is_empty() {
        hint_that_fits(ui, hint, &Type::Body.font(ui), text_room)
    } else {
        String::new()
    };
    let response = ui.add(
        egui::TextEdit::singleline(text)
            .id(id)
            .desired_width(text_room)
            .min_size(vec2(width, height))
            .margin(egui::Margin {
                left: left as i8,
                right: right.min(i8::MAX as f32) as i8,
                top: 6,
                bottom: 6,
            })
            .hint_text(Type::Body.rich(ui, hint).color(c.faint))
            // With no text area the words have no room: they are kept but not shown, so no
            // edge of them lies under `Clear`.
            .text_color(if text_room < 1.0 {
                Color32::TRANSPARENT
            } else {
                c.text
            }),
    );
    named(ui, &response, name);
    // The icon's place meets `Clear`'s place below this width: the icon is not painted there.
    if width >= SEARCH_LEFT + kept {
        ui.painter().text(
            pos2(response.rect.left() + S, response.rect.center().y),
            Align2::LEFT_CENTER,
            icons::SEARCH,
            Type::Body.font(ui),
            c.muted,
        );
    }
    response
}

/// A search field's left margin (the icon's place) and right margin, around its text area.
pub(super) const SEARCH_LEFT: f32 = 28.0;
pub(super) const SEARCH_RIGHT: f32 = 8.0;

/// The width of a segment's words in its widest state: selected words are `Strong`, the others
/// `Body`. A segment takes this width in both states, so no word moves when the choice changes
/// (the rule of `layout::Slot`, for words known only at run time).
fn segment_width(ui: &Ui, words: &str) -> f32 {
    [Type::Strong, Type::Body]
        .iter()
        .map(|role| number_width(ui, words, &role.font(ui)))
        .fold(0.0, f32::max)
}

/// One choice from a few, as words with an underline. Returns true when it changed. Each segment
/// is as wide as its words in their widest state, so nothing moves when the choice changes.
pub fn segmented<T: PartialEq + Copy>(ui: &mut Ui, value: &mut T, options: &[(T, &str)]) -> bool {
    let mut changed = false;
    let c = colors();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for (option, label) in options {
            let selected = *value == *option;
            let font = if selected { Type::Strong } else { Type::Body };
            let color = if selected { c.text } else { c.muted };
            let width = segment_width(ui, label).min(ui.available_width());
            let galley = one_line(ui, label, font.font(ui), color, width.max(1.0) + 1.0);
            let (rect, response) = ui.allocate_exact_size(
                vec2(width + 2.0 * S, Density::Read.control()),
                Sense::click(),
            );
            response.widget_info(|| {
                WidgetInfo::selected(
                    WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    *label,
                )
            });
            if ui.is_rect_visible(rect) {
                if response.hovered() {
                    ui.painter()
                        .rect_filled(rect, corner(RADIUS_CONTROL), c.hover);
                }
                ui.painter().galley(
                    pos2(rect.left() + S, rect.center().y - galley.size().y / 2.0),
                    galley,
                    color,
                );
                if selected {
                    let bar = Rect::from_min_max(
                        pos2(rect.left() + 2.0, rect.bottom() - 2.0),
                        pos2(rect.right() - 2.0, rect.bottom()),
                    );
                    ui.painter().rect_filled(bar, 0.0, c.accent);
                }
                if response.has_focus() {
                    focus_ring(ui, rect);
                }
            }
            if response.clicked() && !selected {
                *value = *option;
                changed = true;
            }
        }
    });
    changed
}

/// A full-width row of fixed height, safe inside `ScrollArea::show_rows`. The closure
/// paints the content inside the rect. Nothing inside a row may be interactive.
pub fn row(
    ui: &mut Ui,
    height: f32,
    selected: bool,
    name: &str,
    paint: impl FnOnce(&Ui, Rect),
) -> Response {
    let c = colors();
    let (rect, mut response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    register_row(ui, rect);
    // egui hit-tests the previous frame's rectangles. Paint hover at the current
    // position so it does not trail the pointer while the list scrolls.
    let hovered = response.enabled()
        && ui.rect_contains_pointer(rect)
        && (!ui.ctx().is_using_pointer() || response.is_pointer_button_down_on());
    response.flags.set(egui::response::Flags::HOVERED, hovered);
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::SelectableLabel, ui.is_enabled(), selected, name)
    });
    if ui.is_rect_visible(rect) {
        if selected {
            ui.painter()
                .rect_filled(rect, corner(RADIUS_CONTROL), c.selected);
        } else if hovered {
            ui.painter()
                .rect_filled(rect, corner(RADIUS_CONTROL), c.hover);
        }
        paint(ui, rect);
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response
}

/// The common list row: a title over a second line.
pub fn two_line_row(
    ui: &mut Ui,
    height: f32,
    selected: bool,
    title: &str,
    detail: &str,
) -> Response {
    let c = colors();
    row(
        ui,
        height,
        selected,
        &format!("{title} · {detail}"),
        |ui, rect| {
            let width = rect.width() - 2.0 * S;
            let font = if selected { Type::Strong } else { Type::Body };
            let galley = one_line(ui, title, font.font(ui), c.text, width);
            let total = galley.size().y + Type::Small.line();
            let top = rect.center().y - total / 2.0;
            ui.painter()
                .galley(pos2(rect.left() + S, top), galley, c.text);
            let detail = one_line(ui, detail, Type::Small.font(ui), c.muted, width);
            ui.painter().galley(
                pos2(rect.left() + S, top + total - Type::Small.line()),
                detail,
                c.muted,
            );
        },
    )
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

/// What the add field did this frame.
pub struct AddFieldResult<T> {
    pub field: Response,
    /// A suggestion, or a choice of the control beside the field.
    pub chosen: Option<T>,
    /// Enter, as the field let go, or `Add`. With nothing `chosen`, the caller adds the typed
    /// words.
    pub enter: bool,
    /// Escape with no chosen suggestion, or `Cancel`: the typed words are cleared, and the
    /// caller closes the field.
    pub cancel: bool,
}

/// The room an add field keeps under its action line for its one-line result (R30): the gap
/// before a message and one message line, as tall as a control at reading density.
fn result_room() -> f32 {
    XS + Density::Read.control()
}

/// Typestate of an [`AddFieldForm`]: its actions are not given yet, so it cannot show.
pub struct NoActions;
/// Typestate of an [`AddFieldForm`]: `Add` and `Cancel` are given.
pub struct WithActions;
/// Typestate of an [`AddFieldForm`]: keys only, no actions (`keys_only`).
pub struct KeysOnly;

/// A control right of a keys-only add field, its width, which can choose a value too.
type Beside<'a, T> = (f32, Box<dyn FnOnce(&mut Ui) -> Option<T> + 'a>);

/// The suggestions of an add field for its typed words.
type Suggest<'a, T> = Box<dyn FnOnce(&str) -> Vec<Suggestions<T>> + 'a>;

/// What draws an add field's result in the room the form keeps for it.
type ResultDraw<'a> = Box<dyn FnOnce(&mut Ui) + 'a>;

/// A field that an action opened, which adds typed words to a list (`Add to Log`, `Scene`): an
/// [`InlineCreate`](super::layout::InlineCreate) form of one field, with its hint in place of a
/// caption. `Add` (bordered, key `Enter`), then quiet `Cancel` (key `Esc`), are on the form's
/// action line under the field (`InlineCreate::actions_under`), so the field keeps the whole
/// room, as wide as a field without actions; in a narrow column `Cancel` wraps under `Add` at the
/// text edge. So the DM sees how to finish without knowing the keys, and the form's layout, keys
/// and input-method guard are the one form's. Suggestions for the typed words go under the
/// action line: an arrow key and Enter, or a click, chooses one; Escape lets go of the chosen
/// one first (`InlineCreate::escape_closes_above`). It shows only with its actions:
///
/// ```
/// use splat_design::layout::doc_parts::{AddField, add_field};
/// fn f(ui: &mut egui::Ui, state: &mut AddField) {
///     let id = splat_design::field_id!("doc_add_line").once(splat_design::layout::Once::Example);
///     let _ = add_field::<()>(id, "Log line for Hob", "What happened?")
///         .actions(splat_design::word!("Add"), splat_design::word!("Cancel"))
///         .show(ui, state);
/// }
/// ```
///
/// ```compile_fail,E0599
/// use splat_design::layout::doc_parts::{AddField, add_field};
/// fn f(ui: &mut egui::Ui, state: &mut AddField) {
///     let id = splat_design::field_id!("doc_add_bare").once(splat_design::layout::Once::Example);
///     let _ = add_field::<()>(id, "Log line for Hob", "What happened?").show(ui, state);
/// }
/// ```
#[must_use = "give its actions, then call show"]
pub struct AddFieldForm<'a, T, A> {
    id: super::layout::FieldId,
    name: &'a str,
    hint: &'a str,
    actions: Option<(super::layout::Word, super::layout::Word)>,
    suggest: Option<Suggest<'a, T>>,
    /// The caller's one-line result, drawn in the room the form keeps for it.
    result: Option<ResultDraw<'a>>,
    /// A keys-only field's control on its right.
    beside: Option<Beside<'a, T>>,
    state: std::marker::PhantomData<A>,
}

/// An add field, `name` its accessible name and `hint` the words inside it. See
/// [`AddFieldForm`].
pub fn add_field<'a, T>(
    id: super::layout::FieldId,
    name: &'a str,
    hint: &'a str,
) -> AddFieldForm<'a, T, NoActions> {
    AddFieldForm {
        id,
        name,
        hint,
        actions: None,
        suggest: None,
        result: None,
        beside: None,
        state: std::marker::PhantomData,
    }
}

impl<'a, T> AddFieldForm<'a, T, NoActions> {
    /// The named action that adds (`Add`), and the one that closes the field (`Cancel`).
    pub fn actions(
        self,
        add: super::layout::Word,
        cancel: super::layout::Word,
    ) -> AddFieldForm<'a, T, WithActions> {
        AddFieldForm {
            id: self.id,
            name: self.name,
            hint: self.hint,
            actions: Some((add, cancel)),
            suggest: self.suggest,
            result: self.result,
            beside: None,
            state: std::marker::PhantomData,
        }
    }

    /// Keys only, no actions: Enter adds; Escape lets go of a chosen suggestion, then clears
    /// the words. The field is keyed by its owner (`id`), and its line keeps one `Read` row's
    /// height, empty or not. `beside` draws a control right of the field, `beside_width` wide,
    /// and can choose a value too. Staged: Clippy refuses it outside
    /// `src/design/` (`Ban::Extras`), and each use is a row of `design::results::STAGED_ALLOWS`
    /// with its reason (the run's `Effect` line, the loot list's own field).
    pub fn keys_only(
        self,
        beside_width: f32,
        beside: impl FnOnce(&mut Ui) -> Option<T> + 'a,
    ) -> AddFieldForm<'a, T, KeysOnly> {
        AddFieldForm {
            id: self.id,
            name: self.name,
            hint: self.hint,
            actions: None,
            suggest: self.suggest,
            result: None,
            beside: Some((beside_width, Box::new(beside))),
            state: std::marker::PhantomData,
        }
    }
}

impl<T> AddFieldForm<'_, T, KeysOnly> {
    pub fn show(self, ui: &mut Ui, state: &mut AddField) -> AddFieldResult<T> {
        let (beside_width, beside) = self.beside.expect("KeysOnly holds its control");
        let (id, name, hint) = (self.id, self.name, self.hint);
        keys_field(
            ui,
            state,
            beside_width,
            beside,
            self.suggest,
            |ui, words, width| {
                ui.set_min_height(Density::Read.control());
                let hint = placeholder(ui, hint);
                let response = id.single_line(ui, words, |edit| {
                    edit.desired_width(width - 2.0 * S)
                        .min_size(vec2(width, Density::Read.control()))
                        .margin(vec2(S, 4.0))
                        .hint_text(hint)
                });
                named(ui, &response, name);
                response
            },
        )
    }
}

impl<'a, T, A> AddFieldForm<'a, T, A> {
    /// The suggestions for the typed words, which show only while there are words.
    pub fn suggest(mut self, suggest: impl FnOnce(&str) -> Vec<Suggestions<T>> + 'a) -> Self {
        self.suggest = Some(Box::new(suggest));
        self
    }

    /// Ruling R30: room for the caller's one-line result (`Scene added to tonight.` with its
    /// Undo) under the action line, kept while the field is open, whether or not a result shows,
    /// so a press on `Add` moves nothing: the result comes in room that is already in view.
    /// `draw` draws the result, or nothing, in that room; suggestions go under it.
    pub fn result_room(mut self, draw: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.result = Some(Box::new(draw));
        self
    }
}

impl<T> AddFieldForm<'_, T, WithActions> {
    /// The named action that adds, as the form draws it.
    fn create(&self) -> super::layout::Create<super::layout::NoReason> {
        let (add, _) = self.actions.expect("WithActions holds its actions");
        super::layout::Create::secondary(add).key("Enter")
    }

    /// The text field's width, margins included, when the form has `room`, as the form lays it
    /// out: all the room, since `Add` is on the action line under it. For a caller that puts
    /// words before the field on its line (World's stamp), so it never measures `Add` itself.
    pub fn field_width(&self, ui: &Ui, room: f32) -> f32 {
        super::layout::create_field_width::<super::layout::NoReason>(ui, room, None)
    }

    pub fn show(self, ui: &mut Ui, state: &mut AddField) -> AddFieldResult<T> {
        use super::layout::{CreateField, InlineCreate, KeptLine, NoReason};
        let create = self.create();
        let (_, cancel) = self.actions.expect("WithActions holds its actions");
        let focus = std::mem::take(&mut state.focus);
        let top = ui.cursor().top();
        // The layout room the part is given: its width and the window's size. The paint clip
        // changes when the DM scrolls (a pane's whole-row cut), so it is not the room.
        let room_now = (ui.available_width(), ui.ctx().content_rect().size());
        // A chosen suggestion is a layer above the form: Escape lets go of it first, through
        // the form's input-method guard.
        let out = InlineCreate::<_, NoReason>::new()
            .field(
                CreateField::new(self.id, self.name, &mut state.words)
                    .placeholder(self.hint)
                    .focus(focus),
            )
            .create(create)
            .kept_line(KeptLine::new().cancel(cancel, true).cancel_key("Esc"))
            .escape_closes_above(state.highlight.is_some())
            .actions_under()
            .show(ui);
        if out.closed_above {
            state.highlight = None;
        }
        let field = out
            .fields
            .into_iter()
            .next()
            .unwrap_or_else(|| ui.response());
        let (enter, cancel) = (out.create, out.cancel);
        if cancel {
            state.clear();
        }
        // The bottom of what the form drew: its room (with the result in it), else its actions.
        let mut bottom = ui.min_rect().bottom();
        if let Some(draw) = self.result {
            let width = ui.available_width();
            let room =
                ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                    ui.set_min_height(result_room());
                    draw(ui);
                });
            // A trailing extent to the result's bottom rounded up to a whole physical pixel: the
            // scroll area clamps its offset to the content and places the content on physical
            // pixels, so the least whole-pixel scroll that holds the result needs this room.
            let ppp = ui.ctx().pixels_per_point();
            let end = (room.response.rect.bottom() * ppp).ceil() / ppp;
            super::extras::space(ui, (end - ui.min_rect().bottom()).max(0.0));
            bottom = end;
        }
        // R30: when the field takes the keyboard (it opens, or the caller gives it back after
        // an act), and when its view changes size while it is open (a resize is not the DM's
        // own scrolling), the least scroll that shows the field, its actions and the result's
        // room. Already in view, nothing moves; only the DM's own scrolling can hide them.
        let view = self.id.named_id(self.name).with("add_field_room");
        let resized = ui
            .data(|data| data.get_temp::<(f32, Vec2)>(view))
            .is_some_and(|last| last != room_now);
        ui.data_mut(|data| data.insert_temp(view, room_now));
        if focus || resized {
            reveal_least(ui, top..=bottom);
        }
        let chosen = suggestion_rows(ui, state, &field, enter, self.suggest);
        AddFieldResult {
            field,
            chosen,
            enter,
            cancel,
        }
    }
}

/// R30: the least scroll, in whole physical pixels rounded up, that puts `block` (the field's top to the
/// bottom of its result's room) inside the view. Its bottom wins when the block is taller than
/// the view: the result is the answer to the act, and cut feedback is the worse fault. Already
/// in view, nothing moves. The scroll is the block's own: no spacing is added to it.
fn reveal_least(ui: &Ui, block: std::ops::RangeInclusive<f32>) {
    let view = ui.clip_rect();
    let (top, bottom) = (*block.start(), *block.end());
    let ppp = ui.ctx().pixels_per_point();
    let whole = |points: f32| (points * ppp).ceil() / ppp;
    let down = if bottom > view.bottom() || bottom - top > view.height() {
        whole((bottom - view.bottom()).max(0.0))
    } else if top < view.top() {
        -whole(view.top() - top)
    } else {
        0.0
    };
    if down != 0.0 {
        // A positive delta moves the content down: the view goes down by `down`.
        ui.scroll_with_delta(vec2(0.0, -down));
    }
}

/// The suggestions under an add field, and the arrow keys that choose one: Enter takes the
/// chosen one, as does a click. Shared by both forms of [`AddFieldForm`].
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn suggestion_rows<T>(
    ui: &mut Ui,
    state: &mut AddField,
    field: &Response,
    enter: bool,
    suggest: Option<Suggest<'_, T>>,
) -> Option<T> {
    if field.changed() {
        state.highlight = None;
    }
    let groups = match suggest {
        Some(suggest) if !state.words.trim().is_empty() => suggest(&state.words),
        _ => Vec::new(),
    };
    let count: usize = groups.iter().map(|group| group.rows.len()).sum();
    if field.has_focus() && count > 0 {
        ui.input_mut(|input| {
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown) {
                state.highlight = Some(state.highlight.map_or(0, |at| (at + 1).min(count - 1)));
            }
            if input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp) {
                state.highlight = state.highlight.and_then(|at| at.checked_sub(1));
            }
        });
    }
    // While there are typed words, so that a click on a suggestion lands after the field
    // lets go of the keyboard.
    let mut chosen = None;
    let mut place = 0;
    for group in groups.into_iter().filter(|group| !group.rows.is_empty()) {
        group_label(ui, group.label);
        for (title, detail, value) in group.rows {
            let selected = state.highlight == Some(place);
            let clicked = two_line_row(ui, XXL + S, selected, &title, &detail).clicked();
            if clicked || (enter && selected) {
                chosen = Some(value);
            }
            place += 1;
        }
    }
    chosen
}

/// The keys-only add field's line and keys (`AddFieldForm::keys_only`), with its text field,
/// keyed by its owner's `FieldId`, drawn by `field` in the width it is given.
fn keys_field<T>(
    ui: &mut Ui,
    state: &mut AddField,
    beside_width: f32,
    beside: impl FnOnce(&mut Ui) -> Option<T>,
    suggest: Option<Suggest<'_, T>>,
    field: impl FnOnce(&mut Ui, &mut String, f32) -> Response,
) -> AddFieldResult<T> {
    let mut chosen = None;
    let field = ui
        .horizontal(|ui| {
            let width = (ui.available_width() - beside_width).max(160.0);
            let field = field(ui, &mut state.words, width);
            chosen = beside(ui);
            field
        })
        .inner;
    if std::mem::take(&mut state.focus) {
        field.request_focus();
    }
    let enter = field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    // Escape closes the top layer first: the chosen suggestion, then the typed words.
    let cancel = ui.input(|input| input.key_pressed(egui::Key::Escape))
        && (field.has_focus() || field.lost_focus())
        && state.highlight.take().is_none();
    if cancel {
        state.words.clear();
    }
    let suggested = suggestion_rows(ui, state, &field, enter, suggest);
    AddFieldResult {
        field,
        chosen: suggested.or(chosen),
        enter,
        cancel,
    }
}

/// A state as an icon, a word and a colour. Colour is never the only signal. A status whose
/// icon and word differ in colour is a `KindStatus` (`design::status`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status<'a> {
    pub icon: &'a str,
    pub word: &'a str,
    pub color: Color32,
}

pub fn status<'a>(ui: &mut Ui, state: impl Into<Tinted<'a>>) -> Response {
    status_label(ui, state.into())
}

/// The look of a status in a painted row: one colour for its icon and word, or a
/// [`super::StatusKind`]'s, which decides both apart (R5: a live recording's icon alone is red).
#[derive(Clone, Copy, Debug)]
pub enum InTextLook {
    One(Color32),
    Kind(super::StatusKind),
}

impl From<Color32> for InTextLook {
    fn from(color: Color32) -> Self {
        Self::One(color)
    }
}

impl From<super::StatusKind> for InTextLook {
    fn from(kind: super::StatusKind) -> Self {
        Self::Kind(kind)
    }
}

/// A painted row keeps its status word in the shared status colour. With a kind, the part of
/// the status up to its kind's icon takes the icon's colour, and the rest the word's.
///
/// A disabled parent dims the words around the status and a one-colour status, but never a
/// kind's colours (R5: a live recording stays in its own colours under any lock).
pub fn status_in_text<L: Into<InTextLook>>(
    ui: &Ui,
    text: &str,
    status: Option<(&str, L)>,
    width: f32,
) -> StatusText {
    let mut job = LayoutJob::default();
    job.wrap.max_width = width;
    let dim = |color: Color32| color.gamma_multiply(ui.opacity());
    let muted = colors().muted;
    let (before, word, after, look) = status
        .and_then(|(word, look)| {
            text.split_once(word)
                .map(|(before, after)| (before, word, after, look.into()))
        })
        .unwrap_or((text, "", "", InTextLook::One(muted)));
    let (icon, word, icon_color, word_color) = match look {
        InTextLook::One(color) => ("", word, dim(color), dim(color)),
        InTextLook::Kind(kind) => {
            let at = word
                .find(kind.icon())
                .map_or(0, |at| at + kind.icon().len());
            (
                &word[..at],
                &word[at..],
                kind.icon_color(),
                kind.word_color(),
            )
        }
    };
    for (text, color) in [
        (before, dim(muted)),
        (icon, icon_color),
        (word, word_color),
        (after, dim(muted)),
    ] {
        job.append(
            text,
            0.0,
            egui::TextFormat {
                font_id: Type::Small.font(ui),
                color,
                ..Default::default()
            },
        );
    }
    StatusText(ui.fonts_mut(|fonts| fonts.layout_job(job)))
}

/// A row's text with its status, from [`status_in_text`]. It holds the parent's dimming in its
/// own colours, so its one way to paint is [`StatusText::paint`], at full opacity: a parent's
/// painter would dim it again, and dim a kind's colours (R5).
pub struct StatusText(Arc<egui::Galley>);

impl StatusText {
    pub fn size(&self) -> Vec2 {
        self.0.size()
    }

    /// Each laid-out line: its characters and the left edge of its first one.
    #[cfg(any(test, feature = "test-support"))]
    pub fn lines(&self) -> Vec<(String, f32)> {
        self.0
            .rows
            .iter()
            .map(|row| {
                (
                    row.glyphs.iter().map(|g| g.chr).collect(),
                    row.pos.x + row.glyphs.first().map_or(0.0, |g| g.pos.x),
                )
            })
            .collect()
    }

    /// Paints the text with its top left at `pos`, in the parent's clip, at full opacity.
    pub fn paint(self, ui: &Ui, pos: Pos2) {
        let mut painter = ui.painter().clone();
        painter.set_opacity(1.0);
        painter.galley(pos, self.0, colors().muted);
    }
}

/// Rail notices wrap so their cause stays visible in a narrow column. A second line starts at
/// the word, not under the icon, as in every other status that wraps.
pub fn wrapped_status<'a>(ui: &mut Ui, state: impl Into<Tinted<'a>>) -> Response {
    wrapped_status_rows(ui, state, usize::MAX)
}

/// [`wrapped_status`] in at most `rows` lines, for a row that reserves that many for it (a
/// creature result's warning, issue #585): past them its word ends in `…`, and its whole word
/// stays its hover and its accessible name.
pub fn wrapped_status_rows<'a>(ui: &mut Ui, state: impl Into<Tinted<'a>>, rows: usize) -> Response {
    let state = state.into();
    if status_fits(ui, state) {
        return status_label(ui, state);
    }
    let status = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        status_icon_column(ui, state);
        let mut job = egui::text::LayoutJob::single_section(
            state.word.to_owned(),
            egui::TextFormat::simple(Type::Small.font(ui), state.word_color),
        );
        job.wrap = egui::text::TextWrapping {
            max_width: ui.available_width(),
            max_rows: rows,
            break_anywhere: false,
            overflow_character: Some('…'),
        };
        let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
        ui.add(egui::Label::new(galley))
    });
    // The whole status, icon included, as one label's rect was before it wrapped at the word.
    status
        .inner
        .union(status.response)
        .on_hover_text(state.word)
}

/// Whether a status's icon, a space and its word fit on one line in what is left of `ui`. Then
/// it is one text, as a one-line status always was; otherwise its lines start at the word.
fn status_fits(ui: &Ui, state: Tinted<'_>) -> bool {
    let width = ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap(
                format!("{} {}", state.icon, state.word),
                Type::Small.font(ui),
                Color32::PLACEHOLDER,
            )
            .size()
            .x
    });
    width <= ui.available_width()
}

/// Where a one-line status starts its word: the glyph after the icon and one space. A galley's
/// size rounds up to whole pixels, so the glyph's own position is the measure.
fn status_word_x(ui: &Ui, state: Tinted<'_>) -> f32 {
    ui.fonts_mut(|fonts| {
        let galley = fonts.layout_no_wrap(
            format!("{} {}", state.icon, state.word),
            Type::Small.font(ui),
            Color32::PLACEHOLDER,
        );
        let at = state.icon.chars().count() + 1;
        galley.rows[0]
            .glyphs
            .get(at)
            .map_or(galley.size().x, |glyph| glyph.pos.x)
    })
}

/// The icon of a status in its own column, as wide as the icon and one space, so the words
/// after it start where a one-line status starts them and every line of them starts there.
fn status_icon_column(ui: &mut Ui, state: Tinted<'_>) {
    let column = status_word_x(ui, state);
    ui.allocate_ui_with_layout(
        vec2(column, Type::Small.line()),
        Layout::left_to_right(Align::Min),
        |ui| {
            ui.set_min_width(column);
            ui.label(Type::Small.rich(ui, state.icon).color(state.icon_color))
        },
    );
}

/// Two rail lines share the word's left edge, leaving the icon in its own column. The icon and
/// the word start where a one-line `status` starts them: the icon at the left edge, the word
/// after the icon and one space. Returns the whole status, and the rect of its first line's word.
pub fn rail_status_detail<'a>(
    ui: &mut Ui,
    state: impl Into<Tinted<'a>>,
    detail: &str,
) -> (Response, Rect) {
    let state = state.into();
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        status_icon_column(ui, state);
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let response = ui.label(Type::Small.rich(ui, state.word).color(state.word_color));
            let word = response.rect;
            (
                response.union(ui.label(Type::Small.rich(ui, detail).color(state.word_color))),
                word,
            )
        })
        .inner
    })
    .inner
}

fn status_label(ui: &mut Ui, state: Tinted<'_>) -> Response {
    let job = status_job(ui, state);
    ui.add(egui::Label::new(job).truncate())
        .on_hover_text(state.word)
}

/// A status's words as one job (its icon in the icon's colour, then its word), in `Small`: what
/// `status` draws, for a part that lays its words out before paint (layout 11c).
pub fn status_job<'a>(ui: &Ui, state: impl Into<Tinted<'a>>) -> LayoutJob {
    let state = state.into();
    let mut job = LayoutJob::default();
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: state.word_color,
        ..Default::default()
    };
    job.append(
        state.icon,
        0.0,
        egui::TextFormat {
            color: state.icon_color,
            ..format.clone()
        },
    );
    job.append(" ", 0.0, format.clone());
    job.append(state.word, 0.0, format);
    job
}

/// A status that also leads to where the DM can act on it. The line wraps, because the
/// words are a warning and must not be cut. A second line starts at the word, not under the
/// icon, as in every other status that wraps.
pub fn status_link<'a>(ui: &mut Ui, state: impl Into<Tinted<'a>>) -> Response {
    let state = state.into();
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: state.word_color,
        ..Default::default()
    };
    // On one line the icon and the word are one text; when it wraps, the word gets its own
    // column, so its second line starts at the word.
    let (column, icon) = if status_fits(ui, state) {
        (0.0, None)
    } else {
        let icon = ui.fonts_mut(|fonts| {
            fonts.layout_no_wrap(
                state.icon.to_owned(),
                format.font_id.clone(),
                state.icon_color,
            )
        });
        (status_word_x(ui, state), Some(icon))
    };
    let mut job = LayoutJob::default();
    if icon.is_none() {
        job.append(
            state.icon,
            0.0,
            egui::TextFormat {
                color: state.icon_color,
                ..format.clone()
            },
        );
        job.append(" ", 0.0, format.clone());
    }
    job.append(
        state.word,
        0.0,
        egui::TextFormat {
            underline: egui::Stroke::new(1.0, state.word_color),
            ..format
        },
    );
    job.wrap.max_width = (ui.available_width() - column).max(1.0);
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, response) = ui.allocate_exact_size(
        vec2(column + galley.size().x, galley.size().y.max(24.0)),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, ui.is_enabled(), state.word));
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        if let Some(icon) = icon {
            ui.painter()
                .galley(pos2(rect.left(), top), icon, state.icon_color);
        }
        ui.painter()
            .galley(pos2(rect.left() + column, top), galley, state.word_color);
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A status link whose 24 px hit target does not add height to its text line.
pub fn inline_status_link<'a>(ui: &mut Ui, state: impl Into<Tinted<'a>>) -> Response {
    let state = state.into();
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: state.word_color,
        ..Default::default()
    };
    let mut job = LayoutJob::default();
    job.append(
        state.icon,
        0.0,
        egui::TextFormat {
            color: state.icon_color,
            ..format.clone()
        },
    );
    job.append(" ", 0.0, format.clone());
    job.append(
        state.word,
        0.0,
        egui::TextFormat {
            underline: egui::Stroke::new(1.0, state.word_color),
            ..format
        },
    );
    job.wrap.max_width = ui.available_width().max(1.0);
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, allocated) = ui.allocate_exact_size(galley.size(), Sense::hover());
    let target = Rect::from_min_max(
        rect.min - vec2(0.0, (24.0 - rect.height()).max(0.0)),
        rect.max,
    );
    let response = ui.interact(target, allocated.id, Sense::click());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, ui.is_enabled(), state.word));
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        ui.painter()
            .galley(pos2(rect.left(), top), galley, state.word_color);
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A label over a large value, with no box. The label stays on one line.
pub fn stat_tile(ui: &mut Ui, label: &str, value: &str) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        super::fact::tile_label_line(ui, label);
        ui.label(Type::NumeralLarge.rich(ui, value).color(colors().text));
    });
}

/// The word that goes with an HP colour, as an icon, a word and a colour. Colour is never the
/// only sign. Healthy and damaged HP need no word.
pub fn hp_word(hp: Option<i64>, max: Option<i64>) -> Option<(&'static str, &'static str, Color32)> {
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

pub fn hp_color(state: HpState) -> Color32 {
    let hp = colors().hp;
    match state {
        HpState::Unknown => hp.unknown,
        HpState::Healthy => hp.healthy,
        HpState::Damaged => hp.damaged,
        HpState::Bloodied => hp.bloodied,
        HpState::Zero => hp.zero,
    }
}

/// A thin bar. Temporary HP is a separate segment at the far end.
pub fn hp_bar(painter: &egui::Painter, rect: Rect, hp: Option<i64>, max: Option<i64>, temp: i64) {
    let c = colors();
    painter.rect_filled(rect, corner(1.0), c.hp.track);
    let (Some(hp), Some(max)) = (hp, max) else {
        return;
    };
    if max <= 0 {
        return;
    }
    let fraction = (hp as f32 / max as f32).clamp(0.0, 1.0);
    let fill = Rect::from_min_size(rect.min, vec2(rect.width() * fraction, rect.height()));
    painter.rect_filled(fill, corner(1.0), hp_color(hp_state(Some(hp), Some(max))));
    // Above the bar, so that it never hides damage on a nearly full one.
    if temp > 0 {
        let width = (rect.width() * (temp as f32 / max as f32)).clamp(3.0, rect.width());
        let segment = Rect::from_min_size(pos2(rect.left(), rect.top() - 2.0), vec2(width, 2.0));
        painter.rect_filled(segment, 0.0, c.hp.temp);
    }
}

/// The fonts an HP readout uses, so the run's scale can grow them.
pub struct HpFonts {
    pub current: FontId,
    pub max: FontId,
}

impl HpFonts {
    pub fn standard(ui: &Ui) -> Self {
        Self {
            current: Type::NumeralLarge.font(ui),
            max: Type::Numeral.font(ui),
        }
    }
}

/// `26/33`. The slash holds its place between rows, whatever the digits.
pub fn hp_readout(
    ui: &Ui,
    painter: &egui::Painter,
    right: f32,
    center_y: f32,
    hp: Option<i64>,
    max: Option<i64>,
    fonts: &HpFonts,
) -> f32 {
    let c = colors();
    let state = hp_state(hp, max);
    let current_color = match state {
        HpState::Zero => c.hp.zero,
        HpState::Bloodied => c.hp.bloodied,
        HpState::Damaged => c.hp.damaged,
        HpState::Unknown => c.muted,
        HpState::Healthy => c.text,
    };
    let max_text = max.map_or_else(|| "?".to_owned(), |max| max.to_string());
    let slot = number_width(ui, &"0".repeat(3.max(max_text.len())), &fonts.max);
    let slash = right - slot;
    let offset = (fonts.current.size - fonts.max.size) * 0.22;
    painter.text(
        pos2(slash, center_y + offset),
        Align2::LEFT_CENTER,
        fraction("", &max_text),
        fonts.max.clone(),
        c.muted,
    );
    let current = hp.map_or_else(|| "?".to_owned(), |hp| hp.to_string());
    number(
        painter,
        slash,
        center_y,
        &current,
        &fonts.current,
        current_color,
    )
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
pub(super) fn grouped_number(n: usize) -> String {
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

/// What this place is, and the one next step.
pub fn empty_state(ui: &mut Ui, icon: &str, title: &str, body: &str, action: impl FnOnce(&mut Ui)) {
    empty_state_with_body(
        ui,
        icon,
        title,
        |ui| {
            ui.label(Type::Body.rich(ui, body).color(colors().muted));
        },
        action,
    );
}

/// An empty state whose next step can include a link in its sentence.
pub fn empty_state_with_body(
    ui: &mut Ui,
    icon: &str,
    title: &str,
    body: impl FnOnce(&mut Ui),
    action: impl FnOnce(&mut Ui),
) {
    let c = colors();
    ui.vertical(|ui| {
        ui.set_max_width(ui.available_width().min(READER_MEASURE));
        ui.spacing_mut().item_spacing.y = S;
        ui.label(Type::Title.rich(ui, icon).color(c.muted));
        ui.label(Type::Title.rich(ui, title).color(c.text));
        body(ui);
        space(ui, XS);
        action(ui);
    });
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tone {
    Info,
    Success,
    Warning,
    Error,
}

impl Tone {
    pub(super) fn parts(self) -> (&'static str, Color32) {
        let result = colors().result;
        match self {
            Self::Info => (icons::INFO, result.info),
            Self::Success => (icons::SUCCESS, result.success),
            Self::Warning => (icons::WARN, result.warning),
            Self::Error => (icons::ERROR, result.error),
        }
    }
}

/// The row of a choice without an icon on its own line below a result message. The row starts
/// at the message words: `message` puts them `S + 1`, the icon and `XS` from its left edge. Put a
/// secondary (bordered) action in it, so its border starts there; on a result, only Undo is quiet.
pub fn message_choice_row<R>(ui: &mut Ui, tone: Tone, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.horizontal(|ui| {
        space(
            ui,
            S + 1.0 + number_width(ui, tone.parts().0, &Type::Body.font(ui)) + XS,
        );
        add(ui)
    })
    .inner
}

/// An inline message: a bar, an icon and words. It never blocks anything. The gaps are its
/// own, so the words start the same distance after the icon in every caller.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn message(ui: &mut Ui, tone: Tone, text: &str) -> Response {
    tone_message(ui, tone, text)
}

/// `message`'s drawing, for the typed places of `outcome` and the parts of this file.
pub(super) fn tone_message(ui: &mut Ui, tone: Tone, text: &str) -> Response {
    let (icon, color) = tone.parts();
    let c = colors();
    let inner = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let start = ui.cursor().min;
        space(ui, S + 1.0);
        ui.label(Type::Body.rich(ui, icon).color(color));
        space(ui, XS);
        let galley = wrapped_words(ui, Type::Body.rich(ui, text).color(c.text));
        let response = ui.add(egui::Label::new(galley));
        let bar = Rect::from_min_max(start, pos2(start.x + 3.0, response.rect.bottom()));
        ui.painter().rect_filled(bar, 0.0, color);
        response
    });
    inner.inner
}

/// The words of a message, wrapped as a wrapping `Label` wraps them, with a rect that ends at the
/// last painted word. A wrapped row keeps the space it broke at; without this, that space widens
/// the rect past the wrap width.
fn wrapped_words(ui: &Ui, words: egui::RichText) -> std::sync::Arc<egui::Galley> {
    let mut job = std::sync::Arc::unwrap_or_clone(egui::WidgetText::from(words).into_layout_job(
        ui.style(),
        egui::FontSelection::Default,
        ui.text_valign(),
    ));
    job.wrap.max_width = ui.available_width();
    job.halign = ui.layout().horizontal_placement();
    job.justify = ui.layout().horizontal_justify();
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let right = galley
        .rows
        .iter()
        .map(|row| {
            row.glyphs
                .iter()
                .rev()
                .find(|glyph| !glyph.chr.is_whitespace())
                .map_or(row.pos.x, |glyph| row.pos.x + glyph.max_x())
        })
        .fold(galley.rect.left(), f32::max);
    if right >= galley.rect.right() {
        return galley;
    }
    let mut trimmed = (*galley).clone();
    trimmed.rect.max.x = right;
    std::sync::Arc::new(trimmed)
}

/// A compact run notice. Tone marks the icon and bar; words keep the text colour.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn small_message(ui: &mut Ui, tone: Tone, text: &str) -> Response {
    small_tone_message(ui, tone, text)
}

/// `small_message`'s drawing, for the typed places of `outcome`.
pub(super) fn small_tone_message(ui: &mut Ui, tone: Tone, text: &str) -> Response {
    let (icon, color) = tone.parts();
    ui.horizontal_top(|ui| {
        let start = ui.cursor().min;
        space(ui, S + HAIRLINE_WIDTH);
        ui.label(Type::Small.rich(ui, icon).color(color));
        space(ui, -XS);
        let response =
            ui.add(egui::Label::new(Type::Small.rich(ui, text).color(colors().text)).wrap());
        ui.painter().rect_filled(
            Rect::from_min_max(
                start,
                pos2(start.x + HAIRLINE_WIDTH * 3.0, response.rect.bottom()),
            ),
            0.0,
            color,
        );
        response
    })
    .inner
}

/// Work in progress. Test builds draw a still mark: the moving one is drawn from the clock,
/// so two captures of the same state would differ.
pub fn spinner(ui: &mut Ui) {
    #[cfg(any(test, feature = "test-support"))]
    ui.label(Type::Body.rich(ui, icons::BUSY).color(colors().muted));
    #[cfg(not(any(test, feature = "test-support")))]
    ui.spinner();
}

/// A small spinner just after a rail status, so work in progress needs no line of its own and
/// nothing below it moves. It is as tall as the text's capitals, with a line near the text's weight;
/// egui's own spinner has a 3-point line, which is heavy beside small text.
pub fn busy_after(ui: &Ui, status: Rect, color: Color32) {
    const STROKE: f32 = 1.5;
    // The outer edge is 9 points across, the height of the small text's capitals.
    let radius = (9.0 - STROKE) / 2.0;
    let center = pos2(
        status.right() + XS + radius + STROKE / 2.0,
        status.top() + Type::Small.line() / 2.0,
    );
    // Tests draw one fixed frame, so captures repeat between runs.
    #[cfg(any(test, feature = "test-support"))]
    let turn = 0.0;
    #[cfg(not(any(test, feature = "test-support")))]
    let turn = {
        ui.ctx().request_repaint();
        ui.input(|i| i.time) as f32 * std::f32::consts::TAU
    };
    let arc = 1.5 * std::f32::consts::PI;
    let points = (0..=24)
        .map(|i| {
            let angle = turn + arc * i as f32 / 24.0;
            center + radius * vec2(angle.cos(), angle.sin())
        })
        .collect();
    ui.painter()
        .add(egui::Shape::line(points, Stroke::new(STROKE, color)));
}

/// The state comes before Discard on every save row.
pub fn save_row_state(ui: &mut Ui, state: Status<'_>, dirty: bool, blocked: Option<&str>) -> bool {
    status(ui, state);
    dirty && Action::quiet("Discard").blocked(blocked).show(ui).clicked()
}

/// A message with one action beside it, capped at `LIST_MEASURE` like every notice. Returns true
/// when the action is used.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn message_with_action(ui: &mut Ui, tone: Tone, text: &str, action: &str) -> bool {
    single_action_message(ui, tone, text, action).inner
}

/// One bordered repair beside its message when it fits, or under the words when it wraps.
pub(super) fn single_action_message(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    action: &str,
) -> egui::InnerResponse<bool> {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        message_with_action_inside(ui, tone, text, action)
    })
}

fn message_with_action_inside(ui: &mut Ui, tone: Tone, text: &str, action: &str) -> bool {
    let button = Action::secondary(action).density(Density::Read);
    let button_width = button.natural_width(ui);
    let icon_width = number_width(ui, tone.parts().0, &Type::Body.font(ui));
    // The words start `XS` after the icon whatever the caller's spacing; the gap follows them.
    let inset = S + 1.0 + icon_width + XS + ui.spacing().item_spacing.x;
    let text_width = number_width(ui, text, &Type::Body.font(ui)) + inset;
    let spacing = ui.spacing().item_spacing.x;
    let mut pressed = false;
    // The action follows the words after `L` in all: the layout's own item gap is part of it.
    if ui.available_width() < text_width + L + button_width {
        tone_message(ui, tone, text);
        ui.horizontal(|ui| {
            space(ui, (inset - spacing).max(0.0));
            pressed = button.show(ui).clicked();
        });
    } else {
        ui.horizontal_top(|ui| {
            let width = (ui.available_width() - button_width - L).max(0.0);
            ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                space(ui, (Density::Read.control() - Type::Body.line()) / 2.0);
                tone_message(ui, tone, text)
            });
            space(ui, (L - spacing).max(0.0));
            pressed = button.show(ui).clicked();
        });
    }
    pressed
}

/// A message with several actions below it, aligned with the message text.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn message_with_actions<R>(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    actions: impl FnOnce(&mut Ui) -> R,
) -> R {
    actions_message(ui, tone, text, actions)
}

/// `message_with_actions`'s drawing.
pub(super) fn actions_message<R>(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    actions: impl FnOnce(&mut Ui) -> R,
) -> R {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        let response = tone_message(ui, tone, text);
        let inset = response.rect.left() - ui.cursor().left();
        ui.horizontal_top(|ui| {
            space(ui, inset);
            ui.vertical(actions).inner
        })
        .inner
    })
    .inner
}

/// Where test builds keep the labels that `row_detail` marked in the last pass.
#[cfg(any(test, feature = "test-support"))]
pub const ROW_DETAILS: &str = "design_row_details";
/// Where `register_row` keeps the rects it registered in this pass: the pane's whole-row cut
/// (N12) and the gate's `help-measure` check read them.
pub const ROW_RECTS: &str = "design_row_rects";

/// Keep `item` under `key` for this pass only: a new pass starts a new list.
pub(super) fn keep_for_pass<T: Clone + Send + std::marker::Sync + 'static>(
    ui: &Ui,
    key: &str,
    item: T,
) {
    let pass = ui.ctx().cumulative_pass_nr();
    ui.ctx().data_mut(|data| {
        let kept = data.get_temp_mut_or_default::<(u64, Vec<T>)>(Id::new(key));
        if kept.0 != pass {
            *kept = (pass, Vec::new());
        }
        kept.1.push(item);
    });
}

/// A row component registers its whole rect: a pane's footer and the session banner then never
/// cut through it (N12, ruling R4), and the gate's `help-measure` check can tell a row's own
/// detail line from help.
pub fn register_row(ui: &Ui, rect: Rect) {
    keep_for_pass(ui, ROW_RECTS, rect);
}

/// Where `register_field_action` keeps this pass's pairs: a field's id and the id of the button
/// that the field's part draws inside it (the search field's `Clear`).
pub const FIELD_ACTIONS: &str = "design_field_actions";

/// A field part registers the button it draws inside its field (the search field's `Clear`),
/// so the gate's `no-overlap` check knows that pair, and only that pair, is meant.
pub(super) fn register_field_action(ui: &Ui, field: Id, action: Id) {
    keep_for_pass(ui, FIELD_ACTIONS, (field.value(), action.value()));
}

/// The pairs `register_field_action` registered in the last pass: (field id, button id), as
/// AccessKit ids. Only the gate reads them.
#[cfg(any(test, feature = "test-support"))]
pub fn field_actions(ctx: &egui::Context) -> Vec<(u64, u64)> {
    let pass = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<(u64, Vec<(u64, u64)>)>(Id::new(FIELD_ACTIONS)))
        .filter(|(kept, _)| kept + 1 >= pass)
        .map(|(_, pairs)| pairs)
        .unwrap_or_default()
}

/// The rects `register_row` registered so far in this pass.
pub(super) fn rows_this_pass(ctx: &egui::Context) -> Vec<Rect> {
    let pass = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<(u64, Vec<Rect>)>(Id::new(ROW_RECTS)))
        .filter(|(kept, _)| *kept == pass)
        .map(|(_, rows)| rows)
        .unwrap_or_default()
}

/// Marks a label as its row's own detail line (its second line), not help. The gate's
/// `help-measure` check passes it whatever its width, but only while it lies inside a row that
/// `register_row` registered: a mark on a label outside any row changes nothing. Call it on the
/// label's response. Only test builds keep the mark.
pub fn row_detail(ui: &Ui, response: &Response) {
    #[cfg(any(test, feature = "test-support"))]
    keep_for_pass(ui, ROW_DETAILS, response.id);
    #[cfg(not(any(test, feature = "test-support")))]
    let _ = (ui, response);
}

/// Help under a field or a section: `Small` muted words that wrap at `READER_MEASURE`, so a long
/// help line keeps the reader's measure in a wide pane.
pub fn help(ui: &mut Ui, text: &str) -> Response {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(READER_MEASURE));
        ui.add(egui::Label::new(Type::Small.rich(ui, text).color(colors().muted)).wrap())
    })
    .inner
}

/// How many text lines the message space below a save row holds.
pub const MESSAGE_SPACE_LINES: f32 = 2.0;

/// The height of the message space below a save row: `MESSAGE_SPACE_LINES` body lines.
pub fn message_space_height() -> f32 {
    MESSAGE_SPACE_LINES * Type::Body.line()
}

/// One message in a `MessageSpace`: its whole words, the part's accessible name, and how to
/// draw it.
type SpaceMessage<'a, R> = (&'a str, Box<dyn FnOnce(&mut Ui) -> R + 'a>);

/// The one message space below a save row: a `width` by `MESSAGE_SPACE_LINES` part at the
/// cursor, whatever it holds, so no message moves what is below it. It holds Save's reason and
/// the latest result in one part, in one order: a result that carries a live Undo comes first,
/// then the reason; otherwise the reason comes first, then the result. With neither, the space
/// stays empty. Longer words wrap and scroll inside the part (see `fixed_part`), whose accessible
/// name is every message's words in that order.
///
/// ```ignore
/// let pressed = design::MessageSpace::new("library_message", width)
///     .reason(problem, |ui| { design::message(ui, Tone::Error, problem); })
///     .result(&words, true, |ui| design::message(ui, Tone::Info, &words))
///     .show(ui);
/// ```
// Library, World repair and Campaign repair adopt it this round (iter-25 packages L, D and C).
#[allow(dead_code)]
pub struct MessageSpace<'a, R> {
    id: &'a str,
    width: f32,
    reason: Option<SpaceMessage<'a, ()>>,
    result: Option<(SpaceMessage<'a, R>, bool)>,
}

#[allow(dead_code)]
impl<'a, R> MessageSpace<'a, R> {
    /// An empty space `width` wide. `id` keeps the part's scroll and focus.
    pub fn new(id: &'a str, width: f32) -> Self {
        Self {
            id,
            width,
            reason: None,
            result: None,
        }
    }

    /// Why Save or the draft waits: a refusal, a parse error, a failed write or guidance.
    pub fn reason(mut self, words: &'a str, add: impl FnOnce(&mut Ui) + 'a) -> Self {
        self.reason = Some((words, Box::new(add)));
        self
    }

    /// The latest result. `has_undo` is true while its Undo is live; then it comes before the
    /// reason, so a reason that does not clear never hides the way back.
    pub fn result(
        mut self,
        words: &'a str,
        has_undo: bool,
        add: impl FnOnce(&mut Ui) -> R + 'a,
    ) -> Self {
        self.result = Some(((words, Box::new(add)), has_undo));
        self
    }

    /// Reserve the space and draw what it holds. Returns what the result's `add` returned, when
    /// there is a result.
    pub fn show(self, ui: &mut Ui) -> Option<R> {
        let (rect, _) =
            ui.allocate_exact_size(vec2(self.width, message_space_height()), Sense::hover());
        let undo_first = self.result.as_ref().is_some_and(|(_, has_undo)| *has_undo);
        let (reason, result) = (self.reason, self.result.map(|(result, _)| result));
        if reason.is_none() && result.is_none() {
            return None;
        }
        let reason_words = reason.as_ref().map(|(words, _)| *words);
        let result_words = result.as_ref().map(|(words, _)| *words);
        let name = if undo_first {
            [result_words, reason_words]
        } else {
            [reason_words, result_words]
        }
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
        fixed_part(ui, rect, self.id, &name, |ui| {
            if undo_first {
                let out = result.map(|(_, add)| add(ui));
                if let Some((_, add)) = reason {
                    add(ui);
                }
                out
            } else {
                if let Some((_, add)) = reason {
                    add(ui);
                }
                result.map(|(_, add)| add(ui))
            }
        })
    }
}

/// One fixed part of a reserved space: a fixed rect whatever it holds. Longer words wrap and
/// scroll inside it, beside a lane kept for the scroll bar, so nothing is cut and nothing below
/// moves. The part is one focusable widget named with its whole message, so Tab reaches it and a
/// screen reader reads all of it. While it has focus, the up and down arrows scroll it by a line
/// and Page Up and Page Down by the part; the wheel scrolls it too.
pub fn fixed_part<R>(
    ui: &mut Ui,
    rect: Rect,
    id: &str,
    name: &str,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    scroll_part(ui, rect, Id::new(id), name, false, add).0
}

/// How far a message part's words sit inside its edge: the focus ring's width and one hairline
/// more, so a focus ring at the edge never crosses a letter.
pub const PART_INSET: f32 = FOCUS_WIDTH + HAIRLINE_WIDTH;

/// A part at `rect` whose words wrap and scroll inside it, `PART_INSET` from its edge, beside a
/// lane kept for its scroll bar. It is one widget named with its whole message, and Tab reaches
/// it and each control in it, scrolling that control into view; while it has focus the up and down arrows scroll it by a line and
/// Page Up and Page Down by the part, and its ring is at the part's edge, clear of the words.
/// The wheel scrolls it too. Returns what `add` returned and the height of what it drew.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
/// `fit`: the part is laid out in `rect` but is only as tall as what it shows (up to `rect`): a
/// footer drawn in its whole room before it was measured (`pane::Footer::pane`). Returns what
/// `add` returns, the content's height, and the part's rect.
pub(super) fn scroll_part<R>(
    ui: &mut Ui,
    rect: Rect,
    id: Id,
    name: &str,
    fit: bool,
    add: impl FnOnce(&mut Ui) -> R,
) -> (R, f32, Rect) {
    let focus = id.with("focus");
    let inner_rect = rect.shrink(PART_INSET);
    let focused = ui.ctx().memory(|memory| memory.has_focus(focus));
    let step = if focused {
        // The arrows scroll the part instead of moving focus.
        ui.ctx().memory_mut(|memory| {
            memory.set_focus_lock_filter(
                focus,
                egui::EventFilter {
                    vertical_arrows: true,
                    ..Default::default()
                },
            );
        });
        let line = Type::Body.line();
        ui.input_mut(|input| {
            let mut step = 0.0;
            for (key, by) in [
                (egui::Key::ArrowDown, line),
                (egui::Key::ArrowUp, -line),
                (egui::Key::PageDown, inner_rect.height()),
                (egui::Key::PageUp, -inner_rect.height()),
            ] {
                while input.consume_key(egui::Modifiers::NONE, key) {
                    step += by;
                }
            }
            step
        })
    } else {
        0.0
    };
    let mut part = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(inner_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    // egui paints a scroll area's content a few points past its edge; here that would put words
    // under the focus ring, so the words stop at the part's inset.
    part.visuals_mut().clip_rect_margin = 0.0;
    let lane = part.spacing().scroll.bar_width + part.spacing().scroll.bar_outer_margin;
    let output = egui::ScrollArea::vertical()
        .id_salt(id)
        .max_height(inner_rect.height())
        // egui's default minimum would let a short overflow grow past the part.
        .min_scrolled_height(inner_rect.height())
        .auto_shrink([false, true])
        .show(&mut part, |ui| {
            // The words end `PART_INSET` before the ring, which ends before the bar's lane.
            ui.set_max_width(inner_rect.width() - lane - PART_INSET);
            if step != 0.0 {
                ui.scroll_with_delta(vec2(0.0, -step));
            }
            let out = add(ui);
            // A control inside that gains keyboard focus scrolls into view, once: Tab to an Undo
            // the part has scrolled away shows it. A later wheel is not pulled back. A change of
            // focus in a pass with a pointer press, hold or release is not looked up: egui's
            // `read_response` drops the focus of a widget that such a press is not on, so a click
            // elsewhere (Lookup in the rail) would lose the focus it just gave its search field.
            let focused = ui.ctx().memory(|memory| memory.focused());
            let seen = id.with("seen_focus");
            let pointer = ui.input(|input| {
                let pointer = &input.pointer;
                pointer.any_down() || pointer.any_pressed() || pointer.any_released()
            });
            if ui.ctx().data(|data| data.get_temp::<Option<Id>>(seen)) != Some(focused) {
                ui.ctx().data_mut(|data| data.insert_temp(seen, focused));
                if let Some(rect) = focused
                    .filter(|_| !pointer)
                    .filter(|focused| *focused != focus)
                    .and_then(|focused| ui.ctx().read_response(focused))
                    .map(|response| reveal_rect(ui.ctx(), response.id, response.rect))
                    .filter(|rect| ui.min_rect().contains_rect(*rect))
                {
                    ui.scroll_to_rect(rect, None);
                }
            }
            out
        });
    let rect = if fit {
        let tall = (output.content_size.y + 2.0 * PART_INSET)
            .ceil()
            .min(rect.height());
        Rect::from_min_size(rect.min, vec2(rect.width(), tall))
    } else {
        rect
    };
    let response = ui.interact(rect, focus, Sense::focusable_noninteractive());
    // A named group that holds the message and its actions: a screen reader reads the whole
    // message, and the group is not a second target over its own Undo.
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::GenericContainer);
        node.set_label(name);
    });
    if response.has_focus() {
        // At the part's edge, `PART_INSET` outside the words, and clear of the scroll bar lane.
        focus_ring(
            ui,
            Rect::from_min_max(rect.min, pos2(inner_rect.right() - lane, rect.bottom())),
        );
    }
    (output.inner, output.content_size.y, rect)
}

/// The key of a control's `reveal_with` rect in egui's temporary data.
const REVEAL_WITH: &str = "reveal_with";

/// A control and the words that belong to it (an action's help line under it): the rect a
/// keyboard focus on the control brings into view, so `scroll_part` and a pane's body never
/// show the control with its words cut off below. Set it in each pass the control is drawn,
/// after its words; a rect from an earlier pass is not used.
pub(super) fn reveal_with(ui: &Ui, control: Id, rect: Rect) {
    set_reveal(ui, control, rect, false);
}

/// The control's whole shape in place of its own rect: an editor viewport's fixed frame, whose
/// text edit is as tall as all its text (layout PR 10b). Words set with `reveal_with` join it.
pub(super) fn reveal_as(ui: &Ui, control: Id, rect: Rect) {
    set_reveal(ui, control, rect, true);
}

fn set_reveal(ui: &Ui, control: Id, rect: Rect, replaces: bool) {
    let pass = ui.ctx().cumulative_pass_nr();
    // Words set for the control earlier in this pass (its field's caption and help) stay: the
    // rect is their union, and a shape set in place of the control stays in place of it.
    ui.ctx().data_mut(|data| {
        let key = control.with(REVEAL_WITH);
        let (rect, replaces) = match data.get_temp::<(u64, Rect, bool)>(key) {
            Some((set, before, was)) if set == pass => (before.union(rect), was || replaces),
            _ => (rect, replaces),
        };
        data.insert_temp(key, (pass, rect, replaces));
    });
}

/// What to bring into view for the focused control `id`, whose own rect is `own`: with the
/// words `reveal_with` set for it in this pass, else only itself; a shape `reveal_as` set
/// stands in place of `own`.
pub(super) fn reveal_rect(ctx: &egui::Context, id: Id, own: Rect) -> Rect {
    let pass = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<(u64, Rect, bool)>(id.with(REVEAL_WITH)))
        .filter(|(set, _, _)| *set == pass)
        .map_or(
            own,
            |(_, rect, replaces)| {
                if replaces { rect } else { rect.union(own) }
            },
        )
}

/// The key of the control whose list is open in this pass (`popups::Dropdown`).
const OPEN_OWNER: &str = "design_open_owner";

/// The control whose list is open now: the pane's focus arbiter keeps its whole field in view
/// as it does a focused control's, while the keyboard is on the list's rows (layout PR 10b).
pub(super) fn set_open_owner(ui: &Ui, control: Id) {
    let pass = ui.ctx().cumulative_pass_nr();
    ui.ctx()
        .data_mut(|data| data.insert_temp(Id::new(OPEN_OWNER), Some((pass, control))));
}

/// The control whose list is open in this pass, if one is.
pub(super) fn open_owner(ctx: &egui::Context) -> Option<Id> {
    let pass = ctx.cumulative_pass_nr();
    ctx.data(|data| data.get_temp::<Option<(u64, Id)>>(Id::new(OPEN_OWNER)))
        .flatten()
        .filter(|(set, _)| *set == pass)
        .map(|(_, control)| control)
}

/// The key of the field block being drawn, for a list that opens from a control in it.
const FIELD_TOP: &str = "design_field_top";

/// The top (its caption's top) of the field block whose control is being drawn now on `ui`'s
/// layer, while `field_label` draws it: a list that opens from the control never covers its
/// own field's caption (layout PR 10b).
pub(super) fn field_top(ui: &Ui) -> Option<f32> {
    let pass = ui.ctx().cumulative_pass_nr();
    let layer = ui.layer_id();
    ui.ctx()
        .data(|data| data.get_temp::<Option<(u64, egui::LayerId, f32)>>(Id::new(FIELD_TOP)))
        .flatten()
        .filter(|(set, at, _)| *set == pass && *at == layer)
        .map(|(_, _, top)| top)
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

type FooterMessage<'a> = (
    FooterRank,
    // A result's order; `None` for a reason or a notice, which keep the caller's order.
    Option<ResultOrder>,
    std::borrow::Cow<'a, str>,
    Box<dyn FnOnce(&mut Ui) + 'a>,
);

/// The one message footer of a pane. Results, reasons and notices that come and go sit in it,
/// below the pane's scroll area or editor, so none of them moves what the DM works on: the
/// content above keeps its top, and only its visible height shrinks. It sits `S` below the body
/// when the body is shorter than the pane (the body's end is the last pass's, kept per footer
/// id, so the first pass with a new message already knows it), and is pinned at the bottom of
/// the pane when the body fills the pane. When it cuts the body, a hairline marks its top edge,
/// so the cut line reads as a list that goes on. The footer is absent when it holds nothing. It
/// is as tall as what it holds, up to one third of the pane when pinned (and always when placed
/// by hand), or up to the pane's end when it follows a short body in `Footer::pane` (ruling Q-c),
/// and scrolls inside beyond that (see `scroll_part`). Its order is fixed: results
/// with a live Undo first, newest first by their `ResultOrder`; then reasons, in the order the
/// caller added them; then other notices in that order, and then results with no live Undo,
/// newest first. Each result is one item with its own order: never put two results in one. A state word stays on the save row; a fault that belongs to one row
/// stays on that row.
///
/// Build it, then give it the pane's body with `Footer::pane` (`src/design/pane.rs`): the part
/// draws the body in its own `Ui`, measures where the body's content ends, and places the
/// footer. The caller gets no y. `show_by_hand` is the old way, staged out (Clippy's
/// `disallowed-methods` and the lint rule `footer-by-hand`).
///
/// Every app footer is typed: its lines are data (`outcome::Messages`), which builds the footer
/// and gives it to the pane.
///
/// ```ignore
/// let frame = design::outcome::PaneFrame::new(ui);
/// let mut messages = design::outcome::Messages::new(&frame, ("library_footer", path), width);
/// if let Some(shown) = results.get(&owner) {
///     messages.kept(shown.outcome(&frame, |kept| kept.outcome()));
/// }
/// messages.outcome(design::outcome::FaultMessage::new(&fault));
/// let paned = messages.pane(ui, design::Body::Content, |ui| {
///     // ...the body...
/// });
/// ```
pub struct Footer<'a> {
    id: Id,
    width: f32,
    /// Lines lowered to a closure (`result`, `legacy_reason`, `legacy_notice`): only the design
    /// system's own pane tests make them (the entries are `#[cfg(test)]`); every app footer is
    /// typed. Empty outside tests.
    messages: Vec<FooterMessage<'a>>,
    /// A typed footer's lines (`outcome::Messages`): data, drawn in the order of the footer's
    /// retained plan (layout PR 2b).
    typed: Option<Box<dyn FooterLines + 'a>>,
    /// The body's own bottom is an edge as the footer is: its scroll column ends at its last
    /// whole registered row there too, with the hairline (`outcome::Messages::rows_end_whole`).
    pub(super) whole_rows: bool,
}

/// A typed footer's lines, as the pane places and draws them (`outcome::Messages`).
pub(super) trait FooterLines {
    fn is_empty(&self) -> bool;
    /// Before the pane places the footer: its whole demand at `width`, when its retained plan
    /// was made from these lines at this width, scale and style. Then the pane knows the
    /// footer's height before it draws, and the footer draws from the plan.
    fn planned(&mut self, ui: &Ui, id: Id, width: f32) -> Option<f32>;
    /// Draw the lines in `rect`, as `Footer::draw_messages` does: from the plan, or measuring
    /// them and making the plan. `fit`: the footer is only as tall as what it shows. Returns the
    /// footer's rect and the height its lines want.
    #[allow(clippy::too_many_arguments)]
    fn draw(
        &mut self,
        ui: &mut Ui,
        id: Id,
        rect: Rect,
        cut: bool,
        most: f32,
        stored: f32,
        fit: bool,
    ) -> (Rect, f32);
}

impl<'a> Footer<'a> {
    /// An empty footer `width` wide. `id` keeps its height, scroll and focus; give each pane
    /// instance its own.
    pub fn new(id: impl std::hash::Hash, width: f32) -> Self {
        Self {
            id: Id::new(id),
            width,
            messages: Vec::new(),
            typed: None,
            whole_rows: false,
        }
    }

    /// A typed footer (`outcome::Messages`): its lines are data, not closures.
    pub(super) fn typed(id: Id, width: f32, lines: Box<dyn FooterLines + 'a>) -> Self {
        Self {
            id,
            width,
            messages: Vec::new(),
            typed: Some(lines),
            whole_rows: false,
        }
    }

    /// A typed footer's whole demand at `width`, when its retained plan holds (`FooterLines`).
    pub(super) fn planned(&mut self, ui: &Ui, width: f32) -> Option<f32> {
        let id = self.id;
        self.typed.as_mut()?.planned(ui, id, width)
    }

    /// A result, made by the action done at `order`. `has_undo` is true while its Undo is live:
    /// it then leads the footer, so a reason that does not clear never hides the way back.
    /// Without one it follows the notices. Results go newest first by `order`, whatever order
    /// the caller adds them in.
    #[deprecated(
        note = "Give the footer a typed `Outcome` with `Footer::outcome` (src/design/outcome.rs)."
    )]
    #[cfg(test)]
    pub fn result(
        &mut self,
        order: ResultOrder,
        words: impl Into<std::borrow::Cow<'a, str>>,
        has_undo: bool,
        draw: impl FnOnce(&mut Ui) + 'a,
    ) -> &mut Self {
        let rank = if has_undo {
            FooterRank::Undo
        } else {
            FooterRank::Notice
        };
        self.messages
            .push((rank, Some(order), words.into(), Box::new(draw)));
        self
    }

    /// Why Save or the draft waits: a refusal, a parse error or a failed write.
    #[deprecated(
        note = "Give the footer a typed `Outcome` with `Footer::outcome` (src/design/outcome.rs)."
    )]
    #[cfg(test)]
    pub fn legacy_reason(
        &mut self,
        words: impl Into<std::borrow::Cow<'a, str>>,
        draw: impl FnOnce(&mut Ui) + 'a,
    ) -> &mut Self {
        self.messages
            .push((FooterRank::Reason, None, words.into(), Box::new(draw)));
        self
    }

    /// Any other notice: a warning, guidance, or why other actions wait.
    #[deprecated(
        note = "Give the footer a typed `Outcome` with `Footer::outcome` (src/design/outcome.rs)."
    )]
    #[cfg(test)]
    pub fn legacy_notice(
        &mut self,
        words: impl Into<std::borrow::Cow<'a, str>>,
        draw: impl FnOnce(&mut Ui) + 'a,
    ) -> &mut Self {
        self.messages
            .push((FooterRank::Notice, None, words.into(), Box::new(draw)));
        self
    }

    /// Staged out: use `Footer::pane`. Clippy's `disallowed-methods` refuses a call outside
    /// `src/design/`, and the lint counts the calls left (`footer-by-hand`), shrink only.
    ///
    /// Draw the footer `S` below the body, or at the bottom of what is left of `ui` when the body
    /// fills it, and return the y where the body ends: `S` above the footer's pinned place, or
    /// the pane's bottom when the footer is empty. Each message's
    /// `draw` runs here, in the footer's order. The body sizes itself to end there (a scroll
    /// area's `max_height`, an editor's height). `ui` also ends there when it holds nothing below
    /// the cursor already, but egui keeps a `Ui`'s bottom below what it holds, so do not rely on
    /// `ui.available_height()` alone. `ui` lays out top down. It measures the body from the last
    /// pass's widgets, and takes a `Ui` that starts at the body's top and holds a scroll area
    /// that does not shrink for a body that fills the pane (round 28, `footer-escape.md`).
    #[must_use = "the body must end at this y: size it with the returned value"]
    pub fn show_by_hand(mut self, ui: &mut Ui) -> f32 {
        debug_assert!(
            ui.layout().main_dir() == egui::Direction::TopDown,
            "a pane's message footer ends a top-down Ui"
        );
        let key = self.id.with("height");
        let stored = ui
            .ctx()
            .data(|data| data.get_temp::<f32>(key))
            .unwrap_or(0.0);
        let pane = ui.available_rect_before_wrap();
        let body_key = self.id.with("body");
        let top = ui.cursor().top();
        let whole = ui.max_rect();
        let last = ui.ctx().data(|data| data.get_temp::<FooterBody>(body_key));
        let remember = |ui: &Ui, limit: f32, footer: f32| {
            ui.ctx().data_mut(|data| {
                data.insert_temp(
                    body_key,
                    FooterBody {
                        top,
                        limit,
                        footer,
                        whole,
                    },
                );
            });
        };
        if self.messages.is_empty() {
            remember(ui, pane.bottom(), pane.bottom());
            if stored != 0.0 {
                ui.ctx().data_mut(|data| data.remove::<f32>(key));
                ui.ctx().request_discard("the message footer emptied");
            }
            return pane.bottom();
        }
        let x = pane.left()..=pane.left() + self.width.min(pane.width());
        let own = ui
            .ctx()
            .data(|data| data.get_temp::<std::collections::HashSet<Id>>(self.id.with("own")))
            .unwrap_or_default();
        let body = last.and_then(|last| last.body_height(ui, x, &own));
        if last.is_none() {
            // A pane that opens with a message: the body is measured in this pass, and the
            // pass is drawn again, so the footer shows first where it belongs.
            ui.ctx()
                .request_discard("the message footer measures its body");
        }
        // Staged out, this placement keeps the old cap: a third of the pane also for a footer
        // that follows a short body. Ruling Q-c (a following footer may take the room down to the
        // pane's end) is `Footer::pane`'s; a legacy pane does not get it (migration-32 design §3).
        let most = (pane.height() / 3.0).floor().max(0.0);
        // Last pass's demand (whole), up to the cap; a change is measured below, and the pass is
        // drawn again.
        let height = stored.min(most);
        // Pinned at the pane's bottom, or `S` below a body that ends above that.
        let pinned = pane.bottom() - height;
        let footer_top = body.map_or(pinned, |body| (top + body + S).min(pinned));
        debug_assert!(footer_top <= pinned);
        // The body has the same room either way, so it never moves or changes its own size
        // when the footer follows it; the footer cuts it only when it is pinned.
        let limit = pinned - S;
        remember(ui, limit, footer_top);
        // The body's end comes from the last pass, so a body that changed in it (a scroll area
        // takes its new height a pass late) moves the footer one pass late. Draw once more, and
        // again on the next frame if egui allows no more passes in this one, until it is still.
        if last.is_some_and(|last| {
            last.footer < pane.bottom() && (last.footer - footer_top).abs() > 0.5
        }) {
            ui.ctx()
                .request_discard("the message footer moved with its body");
            ui.ctx().request_repaint();
        }
        let cut = body.is_none_or(|body| top + body + S >= pinned);

        let rect = Rect::from_min_size(
            pos2(pane.left(), footer_top),
            vec2(self.width.min(pane.width()), height),
        );
        // The widgets the footer adds, so the next pass does not take them for the body.
        let layer = ui.layer_id();
        let added = |ui: &Ui| {
            ui.ctx()
                .viewport(|viewport| viewport.this_pass.widgets.get_layer(layer).count())
        };
        let before = added(ui);
        let _ = self.draw_messages(ui, rect, cut, most, stored, false);
        let own: std::collections::HashSet<Id> = ui.ctx().viewport(|viewport| {
            let widgets = viewport.this_pass.widgets.get_layer(layer);
            widgets.skip(before).map(|widget| widget.id).collect()
        });
        ui.ctx()
            .data_mut(|data| data.insert_temp(self.id.with("own"), own));
        if height == 0.0 {
            // Not measured yet: this pass is drawn again, with the body's room as it was.
            remember(ui, pane.bottom(), pane.bottom());
            return pane.bottom();
        }
        #[cfg(any(test, feature = "test-support"))]
        record_band(ui, pane, rect);
        // egui's `set_max_height` also moves the cursor back to the top of `ui`, so it goes
        // back to where the body starts.
        ui.set_max_height((limit - top).max(0.0));
        space(ui, top - ui.cursor().top());
        // The body is drawn after the footer, so it would take a press on the footer, and paint
        // over it, where it reaches under it: a scroll area as tall as its room below short
        // content, or a line typed this frame before the footer moves down. Nothing drawn next
        // in `ui` shows or takes a press below the body's room, `S` above the footer: egui gives
        // a press to a widget a few points away, so a body clipped at the footer's very top would
        // still take a press on the footer's top edge.
        let mut clip = ui.clip_rect();
        clip.max.y = clip.max.y.min(rect.top() - S);
        ui.set_clip_rect(clip);
        limit
    }

    /// Take `from`'s typed lines (a footer of the same id): `outcome::Messages::pane_then` gives
    /// the pane its lines after the body.
    pub(super) fn take_typed(&mut self, from: Footer<'a>) {
        self.typed = from.typed;
    }

    /// Whether the footer's lines are typed (`outcome::Messages`), measured by their plan.
    pub(super) fn is_typed(&self) -> bool {
        self.typed.is_some()
    }

    /// Whether the footer holds a message.
    pub(super) fn is_empty(&self) -> bool {
        self.messages.is_empty() && self.typed.as_ref().is_none_or(|typed| typed.is_empty())
    }

    /// A key of the messages' words, in the caller's order: a footer measured with other words
    /// is measured again (`pane::Footer::pane`). It reads each message's words once a pass, as
    /// the footer's name does.
    pub(super) fn words_key(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for (_, _, words, _) in &self.messages {
            words.hash(&mut hasher);
        }
        hasher.finish()
    }

    /// Scope the footer's id to `parent` (a pane's parent `Ui`); returns the id the caller gave.
    pub(super) fn rescope(&mut self, parent: Id) -> Id {
        let local = self.id;
        self.id = parent.with(local);
        local
    }

    /// The footer's id and its width in a pane `room` wide.
    pub(super) fn id_and_width(&self, room: f32) -> (Id, f32) {
        (self.id, self.width.min(room))
    }

    /// Draw the messages in `rect`, in the footer's one order, as one focusable group named with
    /// all their words, with a hairline at its top when it `cut`s the body. `most` is the most
    /// the footer may take where it stands: a third of the pane when pinned, the room down to the
    /// pane's end when it follows a short body (ruling Q-c). Keep the height they want, whole
    /// (the caller caps it where it places the footer); a change from `stored` draws the pass
    /// again. `fit`: the messages are laid out in `rect`, and the footer is only as tall as they
    /// are (`scroll_part`). Returns the footer's rect and the height its messages want.
    pub(super) fn draw_messages(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        cut: bool,
        most: f32,
        stored: f32,
        fit: bool,
    ) -> (Rect, f32) {
        if let Some(typed) = self.typed.as_mut() {
            return typed.draw(ui, self.id, rect, cut, most, stored, fit);
        }
        // The one footer order (`outcome::footer_order`): results with a live Undo newest
        // first, reasons and notices in the caller's order, then results with no live Undo.
        super::outcome::footer_order(&mut self.messages);
        if cut && rect.height() > 0.0 {
            // The cut body's last line reads as a list that goes on, not as a drawing fault.
            ui.painter().hline(
                rect.x_range(),
                rect.top() + HAIRLINE_WIDTH / 2.0,
                Stroke::new(HAIRLINE_WIDTH, colors().hairline),
            );
        }
        let name = self
            .messages
            .iter()
            .map(|(_, _, words, _)| words.as_ref())
            .filter(|words| !words.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let messages = std::mem::take(&mut self.messages);
        let (_, content, part) = scroll_part(ui, rect, self.id, &name, fit, |ui| {
            for (_, _, _, draw) in messages {
                draw(ui);
            }
        });
        // The demand, whole: a pinned footer and a following one take it up to their own `most`
        // (ruling Q-c), so neither is measured at the other's cap. (Closure lines keep no
        // reservations: an open lock's reason room is a typed footer's, ruling R29.)
        let _ = most;
        let wanted = (content + 2.0 * PART_INSET).ceil();
        if (wanted - stored).abs() >= HAIRLINE_WIDTH {
            ui.ctx()
                .data_mut(|data| data.insert_temp(self.id.with("height"), wanted));
            ui.ctx()
                .request_discard("the message footer changed height");
        }
        (part, wanted)
    }
}

/// Where a pane's body started in the last pass, the y it had to end at, the footer's top (the
/// pane's bottom when the footer was empty), and the pane `Ui`'s whole rect.
#[derive(Clone, Copy, Debug)]
struct FooterBody {
    top: f32,
    limit: f32,
    footer: f32,
    whole: Rect,
}

impl FooterBody {
    /// How tall the body was in this last pass, from the widgets egui kept for it; `None` when
    /// it filled its room (it ended less than `S` above its limit: a caller that sizes a scroll
    /// area to leave room for a row below it does not leave the exact room) or showed nothing.
    ///
    /// The pane's own `Ui` cannot tell: egui's panels make their `Ui` as tall as the panel. So
    /// the body is each widget on the pane's layer, across `x`, that is not one of the footer's
    /// own (`own`, by id: egui can keep an older place for them), starts at or below the body's
    /// top (also under the footer: a body grows under a footer that follows it), and does not
    /// hold the pane `Ui`'s whole rect: those are the pane, its panel and their frames. A body
    /// widget as tall as its room (an editor, a scroll area that does not shrink) fills it. This
    /// runs only while the footer holds a message, and looks at each widget egui kept once.
    fn body_height(
        self,
        ui: &Ui,
        x: std::ops::RangeInclusive<f32>,
        own: &std::collections::HashSet<Id>,
    ) -> Option<f32> {
        let layer = ui.layer_id();
        let end = ui.ctx().viewport(|viewport| {
            viewport
                .prev_pass
                .widgets
                .get_layer(layer)
                .filter(|widget| !own.contains(&widget.id))
                .map(|widget| widget.rect)
                .filter(|rect| {
                    rect.is_finite()
                        && rect.right() > *x.start()
                        && rect.left() < *x.end()
                        && rect.top() >= self.top - 0.5
                        && !rect.expand(0.5).contains_rect(self.whole)
                })
                .map(|rect| rect.bottom())
                .reduce(f32::max)
        })?;
        (end <= self.limit - S).then(|| (end - self.top).max(0.0))
    }
}

/// For checks: the band each message footer took from its pane in this pass, from `S` above the
/// footer to the pane's bottom, across the whole pane. A pane's content ends at its top.
#[cfg(any(test, feature = "test-support"))]
pub(super) fn record_band(ui: &Ui, pane: Rect, footer: Rect) {
    let pass = ui.ctx().cumulative_pass_nr();
    let band = Rect::from_min_max(pos2(pane.left(), footer.top() - S), pane.right_bottom());
    ui.ctx().data_mut(|data| {
        let bands = data.get_temp_mut_or_default::<(u64, Vec<Rect>)>(Id::new("footer_bands"));
        if bands.0 != pass {
            *bands = (pass, Vec::new());
        }
        bands.1.push(band);
    });
}

/// The bands that the message footers of the last finished pass took from their panes.
#[cfg(any(test, feature = "test-support"))]
pub fn footer_bands(ctx: &egui::Context) -> Vec<Rect> {
    let last = ctx.cumulative_pass_nr().saturating_sub(1);
    ctx.data(|data| data.get_temp::<(u64, Vec<Rect>)>(Id::new("footer_bands")))
        .filter(|(pass, _)| *pass == last)
        .map(|(_, bands)| bands)
        .unwrap_or_default()
}

/// The same safe-first choice for a draft whose file changed on disk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskConflictChoice {
    Overwrite,
    Reload,
}

/// What changed on disk, in a disk conflict's message: the thing's own name, or, only when it
/// has none, `This <noun>`. An enum, so a caller says which: a noun cannot pass as a name.
#[derive(Clone, Copy, Debug)]
pub enum ConflictSubject<'a> {
    /// The name the DM knows the thing by (`Bridge Ambush`), and the noun for it when the name
    /// is blank.
    Named { name: &'a str, noun: &'static str },
    /// A thing with no name of its own: `note` gives `This note`.
    This(&'static str),
}

/// The message names the thing the DM knows (`Bridge Ambush changed on disk.`), not its file
/// (ruling R10); `This <noun>` only for a thing with no name of its own. Callers: World's edit
/// form and World's repair editor (`src/world/edit.rs`, `src/world/repair.rs`).
pub fn disk_conflict_choices(
    ui: &mut Ui,
    subject: ConflictSubject<'_>,
) -> Option<DiskConflictChoice> {
    let thing = match subject {
        ConflictSubject::Named { name, .. } if !name.trim().is_empty() => name.trim().to_owned(),
        ConflictSubject::Named { noun, .. } | ConflictSubject::This(noun) => format!("This {noun}"),
    };
    // The choices start at the message text, like every other message with choices.
    actions_message(
        ui,
        Tone::Warning,
        &format!("{thing} changed on disk. Your draft is safe until you choose."),
        |ui| {
            ui.horizontal_wrapped(|ui| {
                let mut choice = None;
                if Action::secondary("Overwrite with my draft")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    choice = Some(DiskConflictChoice::Overwrite);
                }
                // The layout adds its own item gap after each control; together they make `L`.
                space(ui, L - ui.spacing().item_spacing.x);
                if Action::destructive("Reload and lose my draft")
                    .density(Density::Read)
                    .show(ui)
                    .clicked()
                {
                    choice = Some(DiskConflictChoice::Reload);
                }
                choice
            })
            .inner
        },
    )
}

/// A recovery message whose action restores the previous state.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn undo_message(ui: &mut Ui, tone: Tone, text: &str, action: &str) -> bool {
    undo_message_capped(ui, tone, text, action, None)
}

/// A kept line for an act's result and its action, which `outcome::PlacedResult::show_line`
/// draws its typed result with (M0561; the old `undo_line`, whose last callers moved onto it):
/// one `Read` control high whether or not a result shows. `result` is the words, the action's
/// whole words and its short words. The words come first: the action shows its whole words only
/// while the result's words still fit whole beside them, else its short words; the words that
/// still do not fit are cut by whole words (the whole words on hover and as their name). The
/// action's accessible name is always its whole words.
pub(in crate::design) fn result_line(
    ui: &mut Ui,
    tone: Tone,
    result: Option<(&str, &str, &str)>,
) -> bool {
    let size = vec2(
        ui.available_width().min(LIST_MEASURE),
        Density::Read.control(),
    );
    let mut pressed = false;
    ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Center), |ui| {
        ui.set_min_size(size);
        let Some((text, whole, short)) = result else {
            return;
        };
        let (icon, color) = tone.parts();
        fn action<'w>(words: &'w str, name: &'w str) -> Action<'w> {
            Action::quiet(words)
                .icon(icons::UNDO)
                .density(Density::Read)
                .name(name)
        }
        let gap = ui.spacing().item_spacing.x;
        let left = ui.cursor().left();
        ui.spacing_mut().item_spacing.x = 0.0;
        space(ui, S + 1.0);
        ui.label(Type::Body.rich(ui, icon).color(color));
        space(ui, XS);
        let font = Type::Body.font(ui);
        let needed = number_width(ui, text, &font);
        let fits =
            |words: &str| ui.available_width() - gap - action(words, whole).natural_width(ui);
        let shown = if needed <= fits(whole) { whole } else { short };
        let button = action(shown, whole);
        let room = fits(shown);
        // App round 2: the words shorten by whole words, never inside one.
        let fitted = words_that_fit(ui, text, &font, room);
        let cut = fitted != text;
        let words = one_line(ui, &fitted, font, colors().text, room);
        let response = ui.add(egui::Label::new(words));
        if cut {
            // The cut words are what shows; the whole words are its name and its hover.
            named(ui, &response, text);
            response.clone().on_hover_text(text);
        }
        let bar = Rect::from_min_max(
            pos2(left, response.rect.top()),
            pos2(left + 3.0, response.rect.bottom()),
        );
        ui.painter().rect_filled(bar, 0.0, color);
        space(ui, gap);
        pressed = button.show(ui).clicked();
    });
    pressed
}

/// `undo_message`, with an accessible name for the action when its words are not enough. Capped
/// at `LIST_MEASURE` like every notice.
pub(super) fn undo_message_capped(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    action: &str,
    name: Option<&str>,
) -> bool {
    undo_message_line(ui, tone, text, action, name, None, false).0
}

/// `undo_message_capped`, with Undo blocked for `blocked`'s reason (only Undo greys, and the line
/// keeps its layout, so blocking moves nothing on it; the caller paints the reason), and the y of
/// the centre of the words' first line.
pub(super) fn undo_message_line(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    action: &str,
    name: Option<&str>,
    blocked: Option<&str>,
    tipped: bool,
) -> (bool, f32) {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        undo_message_inside(ui, tone, text, action, name, blocked, tipped)
    })
    .inner
}

fn undo_message_inside(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    action: &str,
    name: Option<&str>,
    blocked: Option<&str>,
    tipped: bool,
) -> (bool, f32) {
    // With `tipped`, a cut label (`name` differs) shows its whole words on hover and on focus.
    let tip = |response: Response| match (tipped, name) {
        (true, Some(name)) => {
            let mut lines = vec![super::popups::TipLine::body(name)];
            lines.extend(blocked.map(super::popups::TipLine::detail));
            super::popups::rich_tip(response, &lines)
        }
        _ => response,
    };
    let button = Action::quiet(action)
        .icon(icons::UNDO)
        .density(Density::Read)
        .blocked(blocked);
    let button = match name {
        Some(name) => button.name(name),
        None => button,
    };
    let button_width = button.natural_width(ui);
    let icon_width = number_width(ui, tone.parts().0, &Type::Body.font(ui));
    // The words start `XS` after the icon whatever the caller's spacing; the gap follows them.
    // App round 2, X13: the line is measured as `message` draws it, with one gap before the
    // action, so the action wraps only when it is wider than the room after the words.
    let inset = S + 1.0 + icon_width + XS;
    let text_width = number_width(ui, text, &Type::Body.font(ui)) + inset;
    let gap = ui.spacing().item_spacing.x;
    let mut pressed = false;
    let words = if ui.available_width() < text_width + gap + button_width {
        let words = tone_message(ui, tone, text);
        ui.horizontal(|ui| {
            space(ui, inset);
            pressed = tip(button.line_start_in_column().show(ui)).clicked();
        });
        words
    } else {
        ui.horizontal_top(|ui| {
            let width = (ui.available_width() - button_width - gap).max(0.0);
            let words = ui
                .allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                    space(ui, (Density::Read.control() - Type::Body.line()) / 2.0);
                    tone_message(ui, tone, text)
                })
                .inner;
            pressed = tip(button.show(ui)).clicked();
            words
        })
        .inner
    };
    (pressed, first_line_centre(&words))
}

/// The y of the centre of a message's first line of words.
pub(super) fn first_line_centre(words: &Response) -> f32 {
    words.rect.top() + Type::Body.line() / 2.0
}

/// One dialog action line: red at the left, keep and commit at the right.
/// Add right-hand actions in reverse order, with the commit action first.
pub fn dialog_actions<L, R>(
    ui: &mut Ui,
    left: impl FnOnce(&mut Ui) -> L,
    right: impl FnOnce(&mut Ui) -> R,
) -> (L, R) {
    ui.horizontal(|ui| {
        let left = left(ui);
        space(ui, L);
        let right = ui
            .with_layout(Layout::right_to_left(Align::Center), right)
            .inner;
        (left, right)
    })
    .inner
}

/// A focused dialog. Only the brief's three modal tasks use this. Escape is the caller's.
pub fn dialog<R>(ctx: &egui::Context, id: &str, title: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
    dialog_width(ctx, id, title, FORM_WIDTH, add)
}

#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn dialog_width<R>(
    ctx: &egui::Context,
    id: &str,
    title: &str,
    width: f32,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    let c = colors();
    let viewport = ctx.content_rect();
    // Anchored at the top, so a dialog does not jump as its steps change its height.
    let modal_id = Id::new(("dialog", id));
    egui::Modal::new(modal_id)
        .area(egui::Modal::default_area(modal_id).anchor(Align2::CENTER_TOP, vec2(0.0, XXL)))
        .backdrop_color(Color32::from_black_alpha(120))
        .frame(
            egui::Frame::new()
                .fill(c.raised)
                .stroke(Stroke::new(HAIRLINE_WIDTH, c.rule))
                .corner_radius(corner(RADIUS_LAYER))
                .inner_margin(egui::Margin::same(L as i8)),
        )
        .show(ctx, |ui| {
            ui.set_max_width((viewport.width() - XXL * 2.0).min(width));
            ui.set_max_height(viewport.height() - XXL * 2.0);
            ui.label(Type::Section.rich(ui, title).color(c.text));
            space(ui, XS);
            let inner = egui::ScrollArea::vertical()
                .id_salt(("dialog_body", id))
                .auto_shrink([false, true])
                .show(ui, add)
                .inner;
            keep_focus_inside(ui);
            inner
        })
        .inner
}

fn default_key(layer: egui::LayerId) -> Id {
    Id::new("dialog_default").with(layer.id)
}

fn before_key(owner: Id) -> Id {
    Id::new("dialog_focus_before").with(owner)
}

/// A focus return that waits for the closing dialog's modal layer to clear.
#[derive(Clone, Copy)]
struct Return {
    owner: Id,
    before: Id,
    default: Id,
}

/// The dialog's default control (G44, `docs/design-system.md:218`): while the dialog shows and
/// is the TOP modal layer, the keyboard focus stays inside it. Where the focus is outside it
/// (nothing has it, or a control under the dialog asked for it), this control takes it, on every
/// frame. A dialog under another one leaves the focus to the top one. `owner` names this
/// dialog's instance (the run whose close dialog it is), so its remembered focus is its own.
pub fn dialog_default(ui: &Ui, response: &Response, owner: Id) {
    let key = default_key(ui.layer_id());
    ui.ctx().data_mut(|data| {
        data.insert_temp(key, (response.id, owner));
        data.insert_temp(Id::new("dialog_default_of").with(owner), response.id);
    });
}

/// The dialog part's rule: after the body, while this dialog is the top modal layer, a focus
/// outside its layer goes to its default control, and the control that had it is kept for
/// `dialog_return_focus`.
fn keep_focus_inside(ui: &Ui) {
    let ctx = ui.ctx();
    let layer = ui.layer_id();
    let Some((default, owner)) = ctx.data(|data| data.get_temp::<(Id, Id)>(default_key(layer)))
    else {
        return;
    };
    // The top modal layer is the previous pass's: none on this dialog's first pass.
    let top = ctx.memory(|memory| memory.top_modal_layer());
    if top.is_some_and(|top| top != layer) {
        return;
    }
    let focused = ctx.memory(|memory| memory.focused());
    let focused_layer = focused
        .and_then(|focused| ctx.read_response(focused))
        .map(|response| response.layer_id);
    if focused_layer == Some(layer) {
        return;
    }
    // Never from another dialog: a modal that opened over this one in this pass (its first
    // request comes after this dialog drew) or that is open above it holds its own focus. Only
    // a lost focus (none, or a control that is gone) or one on the page under the dialogs is
    // taken; so when the dialog above closes, its focus goes and this one takes it back.
    if focused_layer.is_some_and(|other| other.order == egui::Order::Foreground) {
        return;
    }
    // Only a control of the page under the dialogs is the place to give the focus back to,
    // never one of another dialog (a prompt that was open over this one).
    if let Some(focused) = focused
        && focused_layer.is_some_and(|other| other.order != egui::Order::Foreground)
    {
        ctx.data_mut(|data| data.insert_temp(before_key(owner), focused));
    }
    ctx.memory_mut(|memory| memory.request_focus(default));
}

/// When `owner`'s dialog closes, the focus goes back to the control that had it before the
/// dialog took it (the run's Amount field, say), if one did. The return is cancelled when
/// something newer takes the focus first (a field that the next page focuses), and it never
/// gives the focus to a control that is not drawn any more.
pub fn dialog_return_focus(ctx: &egui::Context, owner: Id) {
    let key = before_key(owner);
    let Some(before) = ctx.data(|data| data.get_temp::<Id>(key)) else {
        return;
    };
    ctx.data_mut(|data| data.remove::<Id>(key));
    // The dialog's own default control: while the closing dialog's layer clears, it may
    // still hold (or take back) the focus; that is not a newer choice.
    let default = ctx
        .data(|data| data.get_temp::<Id>(Id::new("dialog_default_of").with(owner)))
        .unwrap_or(before);
    // The key that closed the dialog (Enter on its control, or Escape) is spent: the
    // control that gets the focus back must not read it (a text field ends its input
    // on Enter and would drop the focus again).
    ctx.input_mut(|input| {
        input.consume_key(egui::Modifiers::NONE, egui::Key::Enter);
        input.consume_key(egui::Modifiers::NONE, egui::Key::Escape);
    });
    // egui still counts the closing dialog as the top modal layer on the next pass, and a
    // control under it gives up a focus it cannot use. So the focus returns at the start
    // of the first pass with no modal layer on top.
    ctx.data_mut(|data| {
        let pending = data.get_temp_mut_or_default::<Vec<Return>>(Id::new(RETURN_PENDING));
        pending.retain(|waiting| waiting.owner != owner);
        pending.push(Return {
            owner,
            before,
            default,
        });
    });
    if ctx
        .data(|data| data.get_temp::<bool>(Id::new(RETURN_HOOK)))
        .is_none()
    {
        ctx.data_mut(|data| data.insert_temp(Id::new(RETURN_HOOK), true));
        ctx.on_begin_pass("dialog_return_focus", std::sync::Arc::new(return_pending));
    }
}

/// At the start of a pass: each waiting return either gives the focus back (no modal layer on
/// top, the control was drawn on the last pass, nothing newer has the focus) or is cancelled
/// (a newer focus, or the control is gone); while a modal layer is on top it waits.
fn return_pending(ctx: &egui::Context) {
    let key = Id::new(RETURN_PENDING);
    let Some(mut pending) = ctx.data(|data| data.get_temp::<Vec<Return>>(key)) else {
        return;
    };
    if pending.is_empty() {
        return;
    }
    let focused = ctx.memory(|memory| memory.focused());
    let modal = ctx.memory(|memory| memory.top_modal_layer()).is_some();
    pending.retain(|waiting| {
        let newer = focused.is_some_and(|focused| focused != waiting.default);
        if newer {
            return false;
        }
        if modal {
            return true;
        }
        if ctx.read_response(waiting.before).is_some() {
            ctx.memory_mut(|memory| memory.request_focus(waiting.before));
        }
        false
    });
    ctx.data_mut(|data| data.insert_temp(key, pending));
}

const RETURN_PENDING: &str = "dialog_focus_return_pending";
const RETURN_HOOK: &str = "dialog_focus_return_hook";

/// The width of a panel beside a page: Lookup, or the run's side panel.
pub fn side_panel_width(available: f32) -> f32 {
    (available * 0.36).clamp(PANEL_MIN_WIDTH, PANEL_MAX_WIDTH)
}

fn phosphor_icon(ui: &Ui, rect: Rect, icon: &str, color: Color32) {
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        icon,
        Type::Body.font(ui),
        color,
    );
}

/// The dropdown arrow from the icon set, for `ComboBox::icon`.
pub fn combo_icon(ui: &Ui, rect: Rect, visuals: &egui::style::WidgetVisuals, open: bool) {
    let icon = if open { icons::UP } else { icons::DOWN };
    phosphor_icon(ui, rect, icon, visuals.fg_stroke.color);
}

pub fn section_icon(ui: &mut Ui, openness: f32, response: &Response) {
    let icon = if openness > 0.5 {
        icons::DOWN
    } else {
        icons::NEXT
    };
    phosphor_icon(ui, response.rect, icon, colors().muted);
}

/// A collapsing section with the icon set's arrow, the one form of every section (M0412). Its
/// header says whether it is open (expanded) to a screen reader. It starts closed and keeps the
/// state the DM leaves it in, under its title. `id` keeps that state under the owner's id, so a
/// title that changes (a count, a state word) does not close it; `default_open` is its state the
/// first time it is drawn (a list that has little in it, a form with nothing to add yet);
/// `open` opens (`Some(true)`) or closes (`Some(false)`) it this frame, as a search that shows
/// only its matches does, or a press that asks for it. `None` leaves it as the DM left it.
///
/// ```
/// fn f(ui: &mut egui::Ui, empty: bool) {
///     let _ = splat_design::design::Section::new("Add campaign")
///         .id("campaign_form")
///         .default_open(empty)
///         .show(ui, |ui| ui.label("The form"));
/// }
/// ```
#[must_use = "call show"]
pub struct Section<H = ()> {
    title: egui::WidgetText,
    id: Option<H>,
    default_open: bool,
    open: Option<bool>,
}

impl Section {
    pub fn new(title: impl Into<egui::WidgetText>) -> Self {
        Self {
            title: title.into(),
            id: None,
            default_open: false,
            open: None,
        }
    }
}

impl<H: std::hash::Hash> Section<H> {
    /// The owner's id for its state, in place of its title.
    pub fn id<K: std::hash::Hash>(self, id: K) -> Section<K> {
        Section {
            title: self.title,
            id: Some(id),
            default_open: self.default_open,
            open: self.open,
        }
    }

    /// Its state the first time it is drawn; after that, the DM's.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Its owner opens (`Some(true)`) or closes (`Some(false)`) it this frame.
    pub fn open(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add: impl FnOnce(&mut Ui) -> R,
    ) -> egui::CollapsingResponse<R> {
        let mut header = egui::CollapsingHeader::new(self.title)
            .icon(section_icon)
            .default_open(self.default_open)
            .open(self.open);
        if let Some(id) = self.id {
            header = header.id_salt(id);
        }
        let response = header.show(ui, add);
        with_expanded(ui, response)
    }
}

/// A section whose header is a row the caller draws, not a title: a list whose rows open in
/// place (a sheet's spells, each a fixed row with its name and its facts). `header` draws the row
/// for its state (`open`, for its caret) and gives back its response. The row says whether it is
/// open to a screen reader, a click on it opens or closes it, and the body opens under it, at
/// the row's left edge. It starts closed; its state is kept under `id`. The body's value, while
/// it is open. Review of PR 730: the one other way to open a section; `clippy.toml` refuses
/// egui's `CollapsingState` drawing and `Ui::collapsing` outside the design system.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn section_row<R>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    header: impl FnOnce(&mut Ui, bool) -> Response,
    add: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let id = ui.make_persistent_id(id);
    let mut state =
        egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
    let row = header(ui, state.is_open());
    ui.ctx()
        .accesskit_node_builder(row.id, |node| node.set_expanded(state.is_open()));
    if row.clicked() {
        state.toggle(ui);
    }
    state.show_body_unindented(ui, add).map(|body| body.inner)
}

/// A collapsing section ([`Section`]) under its title.
pub fn section<R>(
    ui: &mut Ui,
    title: impl Into<egui::WidgetText>,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    Section::new(title).show(ui, add)
}

/// A collapsing section that stays open when its title changes, such as a count.
pub fn section_with_id<R>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    title: impl Into<egui::WidgetText>,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    section_opened(ui, id, title, None, add)
}

/// `section_with_id` whose owner opens (`Some(true)`) or closes (`Some(false)`) it this frame,
/// as a search that shows only its matches does; `None` leaves it as the DM left it. Its header
/// says the state it is drawn in to a screen reader either way.
pub fn section_opened<R>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    title: impl Into<egui::WidgetText>,
    open: Option<bool>,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    Section::new(title).id(id).open(open).show(ui, add)
}

/// egui gives a collapsing header no expanded state: set it from the section's own state, after
/// this frame's click, so the tree says what the DM sees.
fn with_expanded<R>(ui: &Ui, response: egui::CollapsingResponse<R>) -> egui::CollapsingResponse<R> {
    let id = response.header_response.id;
    let open = egui::collapsing_header::CollapsingState::load(ui.ctx(), id)
        .is_some_and(|state| state.is_open());
    ui.ctx()
        .accesskit_node_builder(id, |node| node.set_expanded(open));
    response
}

/// A collapsing section for what matters least: technical details at the end of a page.
pub fn quiet_section<R>(
    ui: &mut Ui,
    title: &str,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    let text = Type::Small.rich(ui, title).color(colors().muted);
    section(ui, text, add)
}

/// One table row of cells with fixed widths, and a hairline under it. Stripes are not
/// used: a row's height follows its wrapped text, and a stripe cannot.
pub fn table_row(ui: &mut Ui, widths: &[f32], gap: f32, mut cell: impl FnMut(&mut Ui, usize)) {
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        for (position, width) in widths.iter().enumerate() {
            ui.allocate_ui_with_layout(vec2(*width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.set_width(*width);
                cell(ui, position);
            });
        }
    });
    space(ui, 2.0);
    rule(ui);
    space(ui, 2.0);
}

/// A stable ID for a field that takes focus by request.
pub fn focus_id(name: &str) -> Id {
    Id::new(("splat_focus", name))
}

/// Move keyboard focus to a widget and select all its text.
pub fn focus_and_select(ui: &Ui, id: Id, text: &str) {
    ui.ctx().memory_mut(|memory| memory.request_focus(id));
    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), id) {
        state
            .cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(text.chars().count()),
            )));
        egui::TextEdit::store_state(ui.ctx(), id, state);
    }
}

/// The strip's actions' natural widths with their gaps, kept by the actions' own facts, the font
/// generation and the scale: a quiet frame measures none (layout 11c A2c, counted from a host's
/// entry through its paint).
fn kept_strip_width(ui: &Ui, actions: &[Action<'_>]) -> f32 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for action in actions {
        action.width_facts(&mut hasher);
    }
    (
        super::theme::font_generation(ui.ctx()),
        ui.ctx().pixels_per_point().to_bits(),
    )
        .hash(&mut hasher);
    let key = hasher.finish();
    let id = ui.id().with("kept_strip_width");
    if let Some((kept, width)) = ui.ctx().data(|data| data.get_temp::<(u64, f32)>(id))
        && kept == key
    {
        return width;
    }
    let width = actions
        .iter()
        .map(|action| action.natural_width(ui))
        .sum::<f32>()
        + XS * actions.len().saturating_sub(1) as f32;
    ui.ctx().data_mut(|data| data.insert_temp(id, (key, width)));
    width
}

/// Keep frequent Add actions visible. Other actions share More when space is short.
/// `always_visible` names the actions that stay beside the caller's frequent actions.
/// `menu_only` are lines of More alone, after the others: they take no part in the fit, so no
/// action moves for them, and a strip with no More has none of them.
/// Returns the chosen action's original index, whether inline or in the menu. A `menu_only`
/// line's index follows the last of `actions`.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn hanging_add_strip(
    ui: &mut Ui,
    first: impl FnOnce(&mut Ui),
    actions: Vec<Action<'_>>,
    always_visible: &[usize],
    menu_only: Vec<Action<'_>>,
) -> Option<usize> {
    let after = actions.len();
    let mut chosen = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = XS;
        ui.label(Type::Label.rich(ui, "Add").color(colors().muted));
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = XS;
            first(ui);
            let width = kept_strip_width(ui, &actions);
            if width <= ui.available_size_before_wrap().x {
                for (index, action) in actions.into_iter().enumerate() {
                    if action.show(ui).clicked() {
                        chosen = Some(index);
                    }
                }
            } else {
                let mut overflow = Vec::new();
                for (index, action) in actions.into_iter().enumerate() {
                    if always_visible.contains(&index) {
                        if action.show(ui).clicked() {
                            chosen = Some(index);
                        }
                    } else {
                        overflow.push((index, action));
                    }
                }
                overflow.extend((after..).zip(menu_only));
                let hint = overflow
                    .iter()
                    .map(|(_, action)| action.label)
                    .collect::<Vec<_>>()
                    .join(", ");
                let response = Action::quiet(&format!("{}{}", icons::EXPAND, icons::DOWN))
                    .name("More Add actions")
                    .hint(&hint)
                    .density(Density::Read)
                    .show(ui);
                egui::Popup::menu(&response).show(|ui| {
                    for (index, action) in overflow {
                        if action.show(ui).clicked() {
                            chosen = Some(index);
                            ui.close();
                        }
                    }
                });
            }
        });
    });
    chosen
}

/// A submenu with the same left inset, type and height as a quiet Read action. It opens to the
/// right of its menu when it fits there inside `inside` with `S` to spare, else to the left;
/// in a pane too narrow for either (a 316 px column), it opens below its own row (above it when
/// it does not fit below), so it never covers the row that opened it and the parent menu stays
/// reachable. `inside` is the usable area of the pane that opened the menu: without its
/// splitter and scroll bar, so the submenu and its choices never lie over either.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn menu_submenu<R>(
    ui: &mut Ui,
    word: &str,
    inside: Rect,
    contents: impl FnOnce(&mut Ui) -> R,
) -> egui::InnerResponse<Option<R>> {
    use egui::containers::menu::{MenuState, SubMenu};
    ui.scope(|ui| {
        ui.spacing_mut().button_padding = vec2(S, XS);
        ui.spacing_mut().interact_size.y = Density::Read.control();
        // The same button and open state as egui's submenu button.
        let id = SubMenu::id_from_widget_id(ui.next_auto_id());
        let open = MenuState::from_ui(ui, |state, _| state.open_item == Some(id));
        let inactive = ui.style().visuals.widgets.inactive;
        if open {
            ui.style_mut().visuals.widgets.inactive = ui.style().visuals.widgets.open;
        }
        let button = ui.add(
            // The icon set's caret, as sections and dropdowns use, not egui's solid triangle.
            egui::Button::new(Type::Strong.rich(ui, word))
                .right_text(Type::Body.rich(ui, icons::NEXT).color(colors().muted)),
        );
        ui.style_mut().visuals.widgets.inactive = inactive;
        // Its name is its word, without the arrow.
        named(ui, &button, word);
        // egui puts a submenu `gap` to the right of its button's interact rect, and keeps it
        // inside the screen only. Moving that rect moves the submenu; the open and hover checks
        // still use the button's own rect.
        let gap = egui::Frame::menu(ui.style()).total_margin().sum().x / 2.0 + 2.0;
        let mut anchor = button.clone();
        if let Some(size) = egui::AreaState::load(ui.ctx(), id).and_then(|state| state.size) {
            let place = submenu_place(button.interact_rect, size, gap, inside);
            anchor.interact_rect = anchor.interact_rect.translate(vec2(
                place.x - (button.interact_rect.right() + gap),
                place.y - button.interact_rect.top(),
            ));
        }
        let inner = SubMenu::new().show(ui, &anchor, contents);
        egui::InnerResponse::new(inner.map(|inner| inner.inner), button)
    })
    .inner
}

/// A menu of several submenus (the creature Filter's facts, M0051). Each submenu is a row of the
/// menu, with `menu_submenu`'s inset, type and height and its caret after its word; its choices
/// open under it, in the same menu. So no submenu lies over the next one's row at any width:
/// the menu stays the one layer `menu_popup` placed in its owning pane, clear of its splitter
/// and scroll bar. Each row opens and closes on its own, so a press moves no row above it and
/// the row the DM pressed stays under the pointer (G1b); the rows scroll in the menu when the
/// open ones are taller than 60% of the window. The keyboard reaches each row and then its open
/// choices in order (Tab); Space or Enter on a row opens or closes it; Escape closes the menu,
/// as any menu. Each row tells a screen reader whether it is open. `words` are the submenus'
/// words; `contents(ui, at)` draws an open one's choices, from the row's words; `width` is the
/// widest submenu's choices, so the menu keeps one width whichever are open (the choices from
/// the rows' inset, or the widest row, and the scroll lane). Only
/// [`Submenus::show`](super::layout::Submenus::show) calls it, in a menu that a click inside
/// does not close ([`MenuPopup::show_submenus`](super::layout::MenuPopup::show_submenus)).
pub(in crate::design) fn menu_submenus<W: AsRef<str>>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    width: f32,
    words: &[W],
    mut contents: impl FnMut(&mut Ui, usize),
) {
    let key = ui.make_persistent_id(id);
    let mut state: SubmenuState = ui.data(|data| data.get_temp(key)).unwrap_or_default();
    // Review of PR 759: Escape closes the innermost open submenu first (the one that held the
    // keyboard at the end of the last frame, else the last opened) and gives the keyboard to its
    // row; the menu closes on an Escape only when no submenu is open. egui drops the focus on an
    // Escape before the menu draws, and moves it by the arrow keys after the menu drew, so the
    // widget that had it at the end of the last frame (`had_focus_last_frame`, after that move)
    // is found among the menu layer's widgets of the last pass, on an Escape only.
    if !state.open.is_empty() && ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        let layer = ui.layer_id();
        let widgets: Vec<(Id, Rect)> = ui.ctx().viewport(|viewport| {
            viewport
                .prev_pass
                .widgets
                .get_layer(layer)
                .map(|widget| (widget.id, widget.rect))
                .collect()
        });
        let had = ui.memory(|memory| {
            widgets
                .iter()
                .copied()
                .find(|(id, _)| memory.had_focus_last_frame(*id))
        });
        let held = had.and_then(|(focus, rect)| {
            state.open.iter().rev().copied().find(|at| {
                state.rows.get(at) == Some(&focus)
                    || state
                        .blocks
                        .get(at)
                        .is_some_and(|block| block.contains_rect(rect))
            })
        });
        if let Some(at) = held.or_else(|| state.open.last().copied()) {
            state.open.retain(|open| *open != at);
            ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
            if let Some(row) = state.rows.get(&at) {
                ui.memory_mut(|memory| memory.request_focus(*row));
            }
        }
    }
    // A row: its padding, its word and its caret, as `menu_submenu`'s.
    let font = Type::Strong.font(ui);
    let rows = words
        .iter()
        .map(|word| {
            S + number_width(ui, word.as_ref(), &font) + ui.spacing().item_spacing.x + L + S
        })
        .fold(0.0, f32::max);
    let full = (width + S).max(rows) + super::layout::lane_width();
    // Review of PR 759 (P1): never wider than the menu has. In the narrowest column the menu is
    // narrower than `full`: the block is laid out in what is there, and its choices wrap.
    ui.set_width(full.min(ui.available_width()));
    let room = ui.ctx().content_rect().height() * 0.6;
    // Review of PR 759 (P1): the rows and the open submenus are one block of the menu, `full`
    // wide (wrapped choices and the scroll lane included): the choices in it note no width of
    // their own, so the menu is as wide as the block, not as its widest single choice.
    super::layout::menu_block(ui, full, |ui| {
        super::layout::ScrollColumn::new(key)
            .max_height(room)
            .fit_height()
            .show(ui, |ui| {
                for (at, word) in words.iter().enumerate() {
                    submenu_row(ui, &mut state, at, word.as_ref(), &mut contents);
                }
            });
    });
    ui.data_mut(|data| data.insert_temp(key, state));
}

/// What `menu_submenus` keeps between frames: the open submenus in the order they opened, and,
/// from the last frame, each row's id and each open submenu's block.
#[derive(Clone, Default)]
struct SubmenuState {
    open: Vec<usize>,
    rows: std::collections::HashMap<usize, Id>,
    blocks: std::collections::HashMap<usize, Rect>,
}

/// One row of `menu_submenus` and, while it is open, its choices under it. A click, Space or
/// Enter opens or closes it; with the keyboard on it, Right opens it and Left closes it.
fn submenu_row(
    ui: &mut Ui,
    state: &mut SubmenuState,
    at: usize,
    word: &str,
    contents: &mut impl FnMut(&mut Ui, usize),
) {
    let here = state.open.contains(&at);
    let row = ui
        .scope(|ui| {
            ui.spacing_mut().button_padding = vec2(S, XS);
            ui.spacing_mut().interact_size.y = Density::Read.control();
            if here {
                ui.style_mut().visuals.widgets.inactive = ui.style().visuals.widgets.open;
            }
            let caret = if here { icons::DOWN } else { icons::NEXT };
            ui.add(
                egui::Button::new(Type::Strong.rich(ui, word))
                    .right_text(Type::Body.rich(ui, caret).color(colors().muted)),
            )
        })
        .inner;
    // Its name is its word, without the caret; its state is open or closed.
    named(ui, &row, word);
    ui.ctx()
        .accesskit_node_builder(row.id, |node| node.set_expanded(here));
    state.rows.insert(at, row.id);
    // An arrow the row takes is its own: egui read it as a focus move before the menu drew, so
    // the row cancels that move and keeps the keyboard.
    let key = |key| {
        let taken = ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, key));
        if taken {
            ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
        }
        taken
    };
    let toggle = row.clicked()
        || (row.has_focus() && !here && key(egui::Key::ArrowRight))
        || (row.has_focus() && here && key(egui::Key::ArrowLeft));
    if toggle {
        if here {
            state.open.retain(|open| *open != at);
        } else {
            state.open.push(at);
        }
    }
    if state.open.contains(&at) {
        let block = ui
            .horizontal_top(|ui| {
                space(ui, S);
                ui.vertical(|ui| contents(ui, at));
            })
            .response
            .rect;
        state.blocks.insert(at, block);
    } else {
        state.blocks.remove(&at);
    }
}

/// Where a submenu of `size` starts (its top left): right of its button, else left of it, each
/// level with the button; else below the button's row, or above it when it does not fit below,
/// as far left as the button and inside `inside`. It never covers the button, and it stays `S`
/// inside `inside` whenever `inside` is wide enough to hold it.
fn submenu_place(button: Rect, size: Vec2, gap: f32, inside: Rect) -> egui::Pos2 {
    let right = button.right() + gap;
    let left = button.left() - gap - size.x;
    if right + size.x <= inside.right() - S {
        pos2(right, button.top())
    } else if left >= inside.left() + S {
        pos2(left, button.top())
    } else {
        let x = button
            .left()
            .min(inside.right() - S - size.x)
            .max(inside.left() + S);
        let y = if button.bottom() + size.y <= inside.bottom() - S {
            button.bottom()
        } else {
            (button.top() - size.y).max(inside.top())
        };
        pos2(x, y)
    }
}

/// Sheet tabs keep one line. Less frequent pages move to a named menu when needed.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn overflow_tabs(ui: &mut Ui, selected: &mut usize, choices: &[(usize, &'static str)]) {
    let width = |label: &str| number_width(ui, label, &Type::Strong.font(ui)) + L;
    let all: f32 = choices.iter().map(|(_, label)| width(label)).sum();
    if all <= ui.available_width() {
        segmented(ui, selected, choices);
        return;
    }
    // When the selected page is in the menu, the menu button names it, so the selected page
    // always shows in the row.
    let fit = |reserve: f32| {
        let mut used = reserve + L;
        let mut count = 0;
        for (_, label) in choices {
            if used + width(label) > ui.available_width() {
                break;
            }
            used += width(label);
            count += 1;
        }
        count
    };
    let mut count = fit(menu_button_width(ui, "More"));
    let mut menu_label = "More";
    if let Some((_, label)) = choices[count..]
        .iter()
        .find(|(value, _)| *value == *selected)
    {
        menu_label = label;
        count = fit(menu_button_width(ui, label).max(menu_button_width(ui, "More")));
    }
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        segmented(ui, selected, &choices[..count]);
        let menu = Action::quiet(&format!("{menu_label}    "))
            .density(Density::Read)
            .name("More sheet pages")
            .show(ui);
        let caret = Rect::from_min_max(
            pos2(menu.rect.right() - L - XS, menu.rect.top()),
            menu.rect.right_bottom(),
        );
        super::combo_icon(ui, caret, ui.style().interact(&menu), false);
        egui::Popup::menu(&menu).show(|ui| {
            for (value, label) in &choices[count..] {
                if super::extras::choice(ui, *selected == *value, label).clicked() {
                    *selected = *value;
                    ui.close();
                }
            }
        });
        named(ui, &menu, "More sheet pages");
        if choices[count..]
            .iter()
            .any(|(value, _)| *value == *selected)
        {
            ui.painter().hline(
                menu.rect.x_range(),
                menu.rect.bottom(),
                Stroke::new(HAIRLINE_WIDTH, colors().accent),
            );
        }
    });
}

/// An underlined tab with a muted, nonzero count.
pub fn counted_tab<T: PartialEq + Copy>(
    ui: &mut Ui,
    shown: &mut T,
    option: T,
    label: &str,
    count: usize,
    units: (&str, &str),
) -> bool {
    if count == 0 {
        return segmented(ui, shown, &[(option, label)]);
    }
    let c = colors();
    let selected = *shown == option;
    let font = if selected { Type::Strong } else { Type::Body }.font(ui);
    let color = if selected { c.text } else { c.muted };
    let mut job = egui::text::LayoutJob::default();
    job.append(label, 0.0, egui::TextFormat::simple(font.clone(), color));
    job.append(
        &format!(" {}", super::count(count, "", "")),
        0.0,
        egui::TextFormat::simple(font, c.muted),
    );
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    // As wide as its widest state, as a segment is, so nothing moves when the tab changes.
    let width = segment_width(ui, &format!("{label} {}", super::count(count, "", "")));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            width.max(galley.size().x) + 2.0 * S,
            Density::Read.control(),
        ),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            format!("{label}, {}", super::count(count, units.0, units.1)),
        )
    });
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter().rect_filled(rect, RADIUS_CONTROL, c.hover);
        }
        ui.painter().galley(
            pos2(rect.left() + S, rect.center().y - galley.size().y / 2.0),
            galley,
            color,
        );
        if selected {
            let inset = XS / 2.0;
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.left() + inset, rect.bottom() - inset),
                    egui::pos2(rect.right() - inset, rect.bottom()),
                ),
                0.0,
                c.accent,
            );
        }
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    if response.clicked() && !selected {
        *shown = option;
        return true;
    }
    false
}

/// Second-level sheet pages keep one Small tab line, with overflow in More.
pub fn overflow_tabs_small(ui: &mut Ui, selected: &mut usize, choices: &[(usize, &'static str)]) {
    ui.scope(|ui| {
        let small = Type::Small.font(ui);
        for role in [
            Type::Body.style(),
            Type::Strong.style(),
            egui::TextStyle::Button,
        ] {
            ui.style_mut().text_styles.insert(role, small.clone());
        }
        overflow_tabs(ui, selected, choices);
    });
}

/// Compare a draft with its file. The first draw and Next difference scroll to a marked line.
#[cfg(test)]
pub fn draft_comparison(ui: &mut Ui, draft: &str, disk: &str, id: impl std::hash::Hash) {
    draft_comparison_styled(ui, draft, disk, id, Type::Body.font(ui), Type::Body.line());
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
fn iter3_comparison_navigation_skips_whole_change_groups() {
    use egui_kittest::{Harness, kittest::Queryable};
    for (before, after, groups) in [
        ("old\nolder\nsame", "new\nnewer\nsame", vec![0]),
        (
            "old\nolder\nsame\nlast",
            "new\nnewer\nsame\nend",
            vec![0, 3],
        ),
    ] {
        let state = std::cell::Cell::new(Id::NULL);
        let installed = std::cell::Cell::new(false);
        let mut harness = Harness::new(|ctx| {
            if !installed.replace(true) {
                super::theme::install(ctx);
                ctx.request_repaint();
                return;
            }
            egui::CentralPanel::default().show(ctx, |ui| {
                state.set(ui.make_persistent_id(("draft_difference", "test")));
                grouped_draft_comparison(ui, before, after, "test", true);
            });
        });
        harness.run_steps(2);
        if groups.len() == 1 {
            assert!(harness.query_by_label("Next difference").is_none());
        } else {
            for expected in [3, 0, 3] {
                harness.get_by_label("Next difference").click();
                harness.run();
                assert_eq!(
                    harness.ctx.data(|data| data.get_temp::<usize>(state.get())),
                    Some(expected)
                );
            }
        }
    }
}

#[test]
fn iter3_unchanged_text_aligns_without_highlights_in_the_draft_comparison() {
    for width in [600.0, 1500.0] {
        let ctx = egui::Context::default();
        super::theme::install(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(width, 300.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let before = "old\nremoved\n# Rook\nA note for tonight.";
                    let after = "new\n# Rook\nA note for tonight.";
                    draft_comparison(ui, before, after, "aligned");
                });
            },
        );
        for line in ["# Rook", "A note for tonight."] {
            let text: Vec<_> = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.text() == line => Some(text),
                    _ => None,
                })
                .collect();
            assert_eq!(text.len(), 2);
            assert_eq!(text[0].pos.y, text[1].pos.y);
            for text in text {
                assert!(!output.shapes.iter().any(|shape| matches!(&shape.shape,
                    egui::Shape::Rect(rect) if rect.fill == colors().selected && rect.rect.contains(text.visual_bounding_rect().center())
                )), "Unchanged text must not have a change highlight");
            }
        }
        assert_eq!(
            output
                .platform_output
                .accesskit_update
                .unwrap()
                .nodes
                .iter()
                .filter(|(_, node)| node.label() == Some("No line"))
                .count(),
            1
        );
    }
}

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn iter3_text_input_placeholder_uses_body_and_faint() {
    let ctx = egui::Context::default();
    super::theme::install(&ctx);
    let output = ctx.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            text_input(
                ui,
                "Folder",
                &mut String::new(),
                "Choose a folder",
                FORM_WIDTH,
            );
        });
    });
    let text = output
        .shapes
        .iter()
        .find_map(|shape| match &shape.shape {
            egui::Shape::Text(text) if text.galley.text() == "Choose a folder" => Some(text),
            _ => None,
        })
        .unwrap();
    assert_eq!(
        text.galley.job.sections[0].format.font_id.size,
        Type::Body.size()
    );
    assert_eq!(text.galley.job.sections[0].format.color, colors().faint);
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

#[test]
fn grouped_comparison_shows_the_first_change_without_a_next_control() {
    fn visible(shape: &egui::Shape, clip: egui::Rect, text: &str) -> bool {
        match shape {
            egui::Shape::Text(shape) => {
                shape.galley.text() == text && clip.contains_rect(shape.visual_bounding_rect())
            }
            egui::Shape::Vec(shapes) => shapes.iter().any(|shape| visible(shape, clip, text)),
            _ => false,
        }
    }
    for width in [600.0, 900.0] {
        let context = egui::Context::default();
        super::theme::install(&context);
        let prefix = "Same line\n".repeat(50);
        let output = context.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(width, 300.0),
                )),
                ..Default::default()
            },
            |context| {
                egui::CentralPanel::default().show(context, |ui| {
                    grouped_draft_comparison(
                        ui,
                        &format!("{prefix}Changed draft\nNext changed line"),
                        &format!("{prefix}Disk text\nNext disk line"),
                        "new",
                        true,
                    );
                });
            },
        );
        assert!(output.shapes.iter().any(|shape| visible(
            &shape.shape,
            shape.clip_rect,
            "Changed draft"
        )));
        assert!(!output.shapes.iter().any(|shape| visible(
            &shape.shape,
            shape.clip_rect,
            "Next difference"
        )));
    }
}

/// Keep the count and its heading in one sentence, on one baseline.
fn comparison_summary(ui: &mut Ui, count: &str, lines: &[Option<&str>], line: usize) {
    let heading = lines
        .iter()
        .take(line + 1)
        .rev()
        .flatten()
        .find(|line| line.trim_start().starts_with('#'));
    let words = heading.map_or_else(
        || count.to_owned(),
        |heading| {
            format!(
                "{count} · Under the heading “{}”",
                heading.trim_start_matches('#').trim()
            )
        },
    );
    ui.label(Type::Small.rich(ui, words).color(colors().muted));
}

/// A source comparison that navigates separate changes, rather than adjacent lines.
pub fn grouped_draft_comparison(
    ui: &mut Ui,
    draft: &str,
    disk: &str,
    id: impl std::hash::Hash,
    source: bool,
) {
    let face = if source {
        egui::TextStyle::Monospace.resolve(ui.style())
    } else {
        super::reader::prose_type(ui).font(ui)
    };
    let line_height = if source {
        ui.fonts_mut(|fonts| fonts.row_height(&face))
    } else {
        super::reader::prose_type(ui).line()
    };
    draft_comparison_styled(ui, draft, disk, id, face, line_height);
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

    /// Whether this is the campaign folder itself.
    pub fn is_root(self) -> bool {
        self.0 == Folder::Root
    }

    /// The words of its heading, without the count.
    pub fn words(self) -> String {
        match self.0 {
            Folder::Root => Self::ROOT_NAME.to_owned(),
            Folder::Inside(path) => format!("{path}/"),
        }
    }
}

/// A folder's heading in a list of notes: the folder icon, the folder's words, the count. The
/// root has its own name in one place ([`FolderGroup::ROOT_NAME`]); only a folder ends in `/`.
///
/// ```
/// use splat_design::layout::doc_parts::FolderGroup;
/// let words = |path| FolderGroup::of_path(path).words();
/// for root in ["", ".", "/", "./", "//"] {
///     assert!(FolderGroup::of_path(root).is_root(), "{root:?}");
///     assert_eq!(words(root), "Campaign folder");
/// }
/// assert_eq!(words("Lore/Old"), "Lore/Old/");
/// assert_eq!(words("Notes/"), "Notes/");
/// assert_eq!(words("./Notes"), "Notes/");
/// // A real folder that has the root's name stays a folder.
/// assert!(!FolderGroup::of_path("Campaign folder").is_root());
/// assert_eq!(words("Campaign folder"), "Campaign folder/");
/// ```
///
/// The form is private: a folder cannot be made by hand, so an empty one cannot be shown as `/`.
///
/// ```compile_fail,E0423
/// let _ = splat_design::layout::doc_parts::FolderGroup("");
/// ```
///
/// The heading takes a `FolderGroup`, never a path string that it would edit:
///
/// ```compile_fail,E0308
/// # fn draw(ui: &mut egui::Ui) {
/// splat_design::layout::doc_parts::folder_group_heading(ui, "Places", 3);
/// # }
/// ```
pub fn folder_group_heading(ui: &mut Ui, folder: FolderGroup<'_>, count: usize) {
    ui.label(
        Type::Small
            .rich(
                ui,
                format!(
                    "{} {}  {count}",
                    egui_phosphor::regular::FOLDER,
                    folder.words()
                ),
            )
            .color(colors().muted),
    );
}

/// A source comparison in the caller's editor face and line height.
pub fn draft_comparison_styled(
    ui: &mut Ui,
    draft: &str,
    disk: &str,
    id: impl std::hash::Hash,
    face: egui::FontId,
    line_height: f32,
) {
    draft_comparison_labeled(
        ui,
        [draft, disk],
        ["Your draft", "In the file now"],
        id,
        face,
        line_height,
    );
}

/// A missing source row has no file text. Its name remains available to screen readers.
fn comparison_missing_row(ui: &mut Ui, rect: egui::Rect) {
    let response = ui.interact(rect, ui.next_auto_id(), Sense::hover());
    ui.skip_ahead_auto_ids(1);
    response.widget_info(|| WidgetInfo::new(WidgetType::Label));
    named(ui, &response, "No line");
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    let mut x = rect.left() - rect.height();
    while x < rect.right() {
        painter.line_segment(
            [
                egui::pos2(x, rect.bottom()),
                egui::pos2(x + rect.height(), rect.top()),
            ],
            Stroke::new(HAIRLINE_WIDTH, colors().rule),
        );
        x += S;
    }
}

fn comparison_offset(heights: &[f32], line: usize, height: f32) -> f32 {
    let first_y: f32 = heights.iter().take(line).sum();
    if first_y <= height * 0.5 {
        return 0.0;
    }
    let wanted: f32 = heights.iter().take(line.saturating_sub(2)).sum();
    let limit = (heights.iter().sum::<f32>() - height).max(0.0);
    let mut offset = 0.0;
    for row in heights {
        if offset + row > wanted.min(limit) {
            break;
        }
        offset += row;
    }
    offset
}

#[test]
fn i19_s2_s3_comparison_offset_fills_the_view_and_starts_on_a_whole_row() {
    assert_eq!(comparison_offset(&[20.0; 100], 99, 210.0), 1780.0);
    assert_eq!(comparison_offset(&[20.0; 100], 10, 600.0), 0.0);
    assert_eq!(comparison_offset(&[24.0; 40], 20, 600.0), 360.0);
    assert_eq!(comparison_offset(&[20.0; 4], 3, 200.0), 0.0);
}

/// The shared wrapped comparison with caller-supplied column headings.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn draft_comparison_labeled(
    ui: &mut Ui,
    text: [&str; 2],
    headings: [&str; 2],
    id: impl std::hash::Hash,
    face: egui::FontId,
    line_height: f32,
) {
    let state = ui.make_persistent_id(("draft_difference", id));
    let Comparison {
        columns: [left, right],
        changed,
        groups: differences,
    } = Comparison::cached(ui.ctx(), state.with("layout"), text);
    let first = differences.first().copied().unwrap_or(0);
    let current = ui.data(|data| data.get_temp::<usize>(state));
    let size = ui.available_size();
    let previous_size = ui.data(|data| data.get_temp::<egui::Vec2>(state.with("size")));
    ui.data_mut(|data| data.insert_temp(state.with("size"), size));
    let mut target =
        (current.is_none() || previous_size != Some(size)).then_some(current.unwrap_or(first));
    ui.horizontal_wrapped(|ui| {
        let words = if differences.is_empty() {
            "No lines differ".into()
        } else {
            super::count(changed, "line differs", "lines differ")
        };
        comparison_summary(ui, &words, &left, target.or(current).unwrap_or(first));
        if differences.len() > 1 && Action::quiet("Next difference").show(ui).clicked() {
            target = Some(
                differences
                    .iter()
                    .copied()
                    .find(|&line| line > current.unwrap_or(first))
                    .unwrap_or(first),
            );
        }
    });
    if let Some(line) = target {
        ui.data_mut(|data| data.insert_temp(state, line));
    }
    ui.columns(2, |columns| {
        group_label(&mut columns[0], headings[0]);
        group_label(&mut columns[1], headings[1]);
    });
    let height = ui.available_height();
    let row_gap = ui.spacing().item_spacing.y;
    ui.spacing_mut().item_spacing.y = 0.0;
    // Set the offset before painting. A scroll request inside the body takes effect
    // on a later frame and can inherit the old reader's position on opening.
    let column_width = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
    let text_width = column_width - XS - ui.spacing().item_spacing.x;
    let heights: Vec<f32> = (0..left.len().max(right.len()))
        .map(|line| {
            [&left, &right]
                .into_iter()
                .map(|lines| {
                    ui.fonts_mut(|fonts| {
                        fonts.layout(
                            lines.get(line).copied().flatten().unwrap_or("").to_owned(),
                            face.clone(),
                            colors().text,
                            text_width,
                        )
                    })
                    .size()
                    .y
                    .max(line_height)
                    .ceil()
                })
                .fold(0.0, f32::max)
                + ui.spacing().item_spacing.y
        })
        .collect();
    let padding = if face.family != egui::FontFamily::Monospace {
        (height - line_height * 3.0).max(0.0)
    } else {
        // A whole row lets the clamped offset keep the final line fully in view.
        line_height
    };
    let mut scroll = egui::ScrollArea::vertical().id_salt(state);
    if let Some(line) = target {
        let mut scroll_heights = heights.clone();
        scroll_heights.push(padding);
        scroll = scroll.vertical_scroll_offset(comparison_offset(&scroll_heights, line, height));
    }
    scroll
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, viewport| {
            // Keep the previous row's descenders outside the comparison viewport.
            let viewport = viewport.translate(ui.max_rect().min.to_vec2());
            ui.set_clip_rect(ui.clip_rect().intersect(viewport));
            for (line, &row_height) in heights.iter().enumerate() {
                let differs = left.get(line) != right.get(line);
                let width = ui.available_width();
                let (row, _) = ui.allocate_exact_size(vec2(width, row_height), Sense::hover());
                let gap = ui.spacing().item_spacing.x;
                let column = (width - gap) / 2.0;
                for (side, lines) in [&left, &right].into_iter().enumerate() {
                    let rect = egui::Rect::from_min_size(
                        row.min + vec2(side as f32 * (column + gap), 0.0),
                        vec2(column, row_height),
                    );
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        if differs {
                            colors().selected
                        } else {
                            colors().page
                        },
                    );
                    if differs {
                        ui.painter().vline(
                            rect.left() + XS / 2.0,
                            rect.y_range(),
                            Stroke::new(HAIRLINE_WIDTH, colors().accent),
                        );
                    }
                    let text_rect =
                        egui::Rect::from_min_max(rect.min + vec2(XS + gap, 0.0), rect.max);
                    let mut cell = ui.new_child(
                        egui::UiBuilder::new()
                            .max_rect(text_rect)
                            .layout(Layout::top_down(Align::Min)),
                    );
                    if lines.get(line).is_none_or(Option::is_none) {
                        comparison_missing_row(&mut cell, text_rect);
                        continue;
                    }
                    let color = colors().text;
                    let galley = cell.fonts_mut(|fonts| {
                        fonts.layout(
                            lines.get(line).copied().flatten().unwrap_or("").to_owned(),
                            face.clone(),
                            color,
                            text_rect.width(),
                        )
                    });
                    cell.add(egui::Label::new(galley));
                }
            }
            space(ui, padding);
        });
    ui.spacing_mut().item_spacing.y = row_gap;
}

#[cfg(test)]
// The tests draw the old message functions until Wave 3 makes them private.
#[allow(deprecated)]
mod tests {
    use super::*;

    /// M2 (B2 packet 3, F1): no program leaves the draft's reason unpainted. A changed-on-disk
    /// draft with no save row and nothing between (the omission) paints it on the toolbar's
    /// line or under it; a dirty draft whose own refusal has the changed-file words (the same
    /// words, another cause) paints the toolbar's own reason on its save row.
    #[test]
    fn every_toolbar_program_paints_the_draft_reason() {
        use super::super::save_row::{Changed, Diagnosis, DraftOrigin, DraftState, Primary};
        let painted = |state: &DraftState<'_>, row: DraftRow<'static>, width: f32| {
            let ctx = egui::Context::default();
            super::super::theme::install(&ctx);
            let mut texts = Vec::new();
            for _ in 0..3 {
                let output = ctx.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.set_max_width(width);
                        let toolbar = ReaderToolbar::new(
                            false,
                            "Edit Markdown",
                            "Markdown source",
                            ModeLock::Open,
                        )
                        .action("Copy to…", 1, Availability::Draft);
                        let _ =
                            toolbar.show(ui, state, DraftBlocks::EntryInPackage, |_| row, |_| {});
                    });
                });
                texts = output
                    .shapes
                    .into_iter()
                    .filter_map(|shape| match shape.shape {
                        egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>();
            }
            texts
        };
        for width in [900.0, 300.0] {
            let on_disk =
                DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft).changed_on_disk();
            let texts = painted(&on_disk, DraftRow::None, width);
            assert_eq!(
                texts
                    .iter()
                    .filter(|text| *text == super::super::save_row::CHANGED_ON_DISK)
                    .count(),
                1,
                "at {width}, the omission paints the reason: {texts:?}"
            );
            let same_words = DraftState::new(DraftOrigin::Saved, Changed::Yes, Primary::Draft)
                .check(Diagnosis::refused(super::super::save_row::CHANGED_ON_DISK));
            let texts = painted(
                &same_words,
                DraftRow::Save {
                    label: "Save",
                    width,
                },
                width,
            );
            assert!(
                texts.iter().any(|text| text.starts_with("Save or discard")),
                "at {width}, the same words with another cause paint the draft's reason: {texts:?}"
            );
        }
    }

    /// iter-22-B SD2: a submenu opens right while it fits `S` inside the pane, else left, else
    /// as far right as fits; each branch is reached.
    #[test]
    fn iter22_b_submenu_opens_right_then_left_then_inward() {
        let button = Rect::from_min_max(pos2(500.0, 100.0), pos2(720.0, 128.0));
        let (gap, size) = (8.0, vec2(126.0, 90.0));
        let pane = |right: f32| Rect::from_min_max(pos2(150.0, 0.0), pos2(right, 800.0));
        let place = |inside| submenu_place(button, size, gap, inside);
        // Room on the right: unchanged, level with the button.
        assert_eq!(place(pane(1200.0)), pos2(728.0, 100.0));
        // Exactly `S` to spare still opens right; one point less opens left.
        assert_eq!(place(pane(728.0 + size.x + S)), pos2(728.0, 100.0));
        assert_eq!(
            place(pane(727.0 + size.x + S)),
            pos2(500.0 - gap - size.x, 100.0)
        );
        // The pane of the encounter editor at 1200: left of the menu, inside the pane.
        let left = place(pane(735.0)).x;
        assert!(left + size.x <= 735.0 - S && left >= 150.0 + S, "{left}");
        // K2 (gap K-4): no room on either side: below the button's row, never over it.
        let narrow = Rect::from_min_max(pos2(400.0, 0.0), pos2(760.0, 800.0));
        let below = place(narrow);
        assert_eq!(below.y, button.bottom(), "below the row that opened it");
        assert!(below.x >= 400.0 + S && below.x + size.x <= 760.0 - S);
        // No room below either: above it.
        let low = Rect::from_min_max(pos2(400.0, 0.0), pos2(760.0, 200.0));
        assert_eq!(place(low).y, button.top() - size.y, "above the row");
    }

    #[test]
    fn iter10_b_stepper_limit_reasons_are_optional_and_yield_to_a_lock() {
        use crate::design::test_harness::Themed as _;
        let describe = |low: Option<&str>, high: Option<&str>, lock: Option<&str>, value: i64| {
            let ctx = egui::Context::default();
            super::super::theme::install(&ctx);
            ctx.enable_accesskit();
            let output = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    locked(ui, lock, |ui| {
                        let mut value = value;
                        stepper_with_density(
                            ui,
                            "Level",
                            &mut value,
                            0..=6,
                            Density::Scan,
                            low,
                            high,
                        );
                    });
                });
            });
            let nodes = output.platform_output.accesskit_update.unwrap().nodes;
            ["Decrease Level", "Increase Level"].map(|label| {
                let (_, node) = nodes
                    .iter()
                    .find(|(_, node)| node.label() == Some(label))
                    .unwrap();
                (node.is_disabled(), node.description().map(str::to_owned))
            })
        };
        // Existing callers pass None: disabled at the limit, with no description, as before.
        assert_eq!(describe(None, None, None, 0), [(true, None), (false, None)]);
        assert_eq!(describe(None, None, None, 6), [(false, None), (true, None)]);
        // With reasons, only the button at its limit says why.
        let reasons = (Some("Already 0."), Some("Stops at 6."));
        assert_eq!(
            describe(reasons.0, reasons.1, None, 0),
            [(true, Some("Already 0.".into())), (false, None)]
        );
        assert_eq!(
            describe(reasons.0, reasons.1, None, 6),
            [(false, None), (true, Some("Stops at 6.".into()))]
        );
        // An enclosing lock's reason comes first, on both buttons.
        let locked = Some("Close the run first.".to_owned());
        assert_eq!(
            describe(reasons.0, reasons.1, Some("Close the run first."), 6),
            [(true, locked.clone()), (true, locked)]
        );
        // The signed stepper (M-R0.2's request, the Run's Correct tab): a modifier shows `+2`
        // and `−1` with a true minus, reads back what the DM types in either form, and keeps
        // the stepper's buttons and limits.
        for (value, shown) in [(-1, "\u{2212}1"), (2, "+2"), (0, "+0")] {
            let ctx = egui::Context::default();
            super::super::theme::install(&ctx);
            ctx.enable_accesskit();
            let output = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut value = value;
                    signed_stepper(ui, "Initiative bonus", &mut value, -1..=10);
                });
            });
            let painted: Vec<String> = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                    _ => None,
                })
                .collect();
            assert!(
                painted.iter().any(|text| text == shown),
                "{value} shows {shown}: {painted:?}"
            );
            let nodes = output.platform_output.accesskit_update.unwrap().nodes;
            let decrease = nodes
                .iter()
                .find(|(_, node)| node.label() == Some("Decrease Initiative bonus"))
                .map(|(_, node)| node.is_disabled());
            assert_eq!(decrease, Some(value == -1), "{value}: − at its limit only");
        }
        // The shared reader of typed signed numbers (M-P0 item 11): what the app writes reads
        // back, a true minus included; words that are not one whole number read as nothing (the
        // tracker's Amount cases, PR 638 review, moved here with its reader).
        for (typed, read) in [
            ("\u{2212}3", -3),
            ("-3", -3),
            ("+2", 2),
            ("2", 2),
            (" \u{2212}12 ", -12),
            (&signed(-7), -7),
        ] {
            assert_eq!(parse_signed(typed), Some(read), "{typed}");
        }
        for typed in ["3 points", "three", "", "++4", "+\u{2212}3"] {
            assert_eq!(parse_signed(typed), None, "{typed}");
        }
        // The grouped stepper (M-P0 item 8, the group editor's member count): a count shows its
        // digits grouped, `1,000` stays `1,000`, and reads back with or without the commas.
        for (value, shown) in [(1_000, "1,000"), (12_345, "12,345"), (999, "999"), (1, "1")] {
            let ctx = egui::Context::default();
            super::super::theme::install(&ctx);
            ctx.enable_accesskit();
            let output = ctx.run(Default::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let mut value = value;
                    grouped_stepper(ui, "How many", &mut value, 1..=99_999);
                });
            });
            let painted: Vec<String> = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
                    _ => None,
                })
                .collect();
            assert!(
                painted.iter().any(|text| text == shown),
                "{value} shows {shown}: {painted:?}"
            );
            let nodes = output.platform_output.accesskit_update.unwrap().nodes;
            let decrease = nodes
                .iter()
                .find(|(_, node)| node.label() == Some("Decrease How many"))
                .map(|(_, node)| node.is_disabled());
            assert_eq!(decrease, Some(value == 1), "{value}: − at its limit only");
        }
        for (typed, read) in [
            ("1,000", 1_000.0),
            ("1000", 1_000.0),
            ("12 345", 12_345.0),
            (" 7 ", 7.0),
        ] {
            assert_eq!(parse_grouped(typed), Some(read), "{typed}");
        }
        // Review of PR 626 (P2): typed, not only painted and parsed. The DM clicks the count,
        // types `1,000` over it and leaves: the value is 1000. While typing, the field keeps
        // exactly the typed words (the caret stays at their end), and Undo gives back fewer of
        // them, not a reformatted number.
        use egui_kittest::kittest::Queryable;
        let mut h = egui_kittest::Harness::builder()
            .with_size(vec2(400.0, 120.0))
            .themed_ui_state(
                |ui, value: &mut i64| {
                    grouped_stepper(ui, "How many", value, 1..=99_999);
                },
                7_i64,
            );
        h.run_steps(2);
        // A pointer click on the count opens it for typing.
        let at = h.get_by_label("How many").rect().center();
        h.hover_at(at);
        h.run_steps(1);
        for pressed in [true, false] {
            h.event(egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            h.run_steps(1);
        }
        h.run_steps(1);
        h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
        h.run_steps(1);
        // While it is open for typing, the count keeps its typed words under its id (egui's
        // drag value), and its text edit state holds the caret.
        let id = h
            .ctx
            .memory(|memory| memory.focused())
            .expect("the count has the keyboard");
        let field = |h: &egui_kittest::Harness<'_, i64>| {
            h.ctx
                .data(|data| data.get_temp::<String>(id))
                .unwrap_or_default()
        };
        let caret = |h: &egui_kittest::Harness<'_, i64>| {
            egui::text_edit::TextEditState::load(&h.ctx, id)
                .and_then(|state| state.cursor.char_range())
                .map(|range| range.primary.index)
        };
        let mut typed = String::new();
        for letter in "1,000".chars() {
            h.event(egui::Event::Text(letter.to_string()));
            h.run_steps(1);
            typed.push(letter);
            assert_eq!(
                (field(&h), caret(&h)),
                (typed.clone(), Some(typed.chars().count())),
                "the field keeps the typed words while typing, the caret at their end"
            );
        }
        h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::Z);
        h.run_steps(1);
        assert_eq!(
            field(&h),
            "7",
            "Undo gives back the words from before the typing, not a reformatted number"
        );
        h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
        h.run_steps(1);
        for letter in typed.chars() {
            h.event(egui::Event::Text(letter.to_string()));
            h.run_steps(1);
        }
        assert_eq!(field(&h), "1,000", "typed again");
        h.key_press(egui::Key::Tab);
        h.run_steps(2);
        assert_eq!(*h.state(), 1_000, "typed `1,000`, the value is 1000");
        // Issue #633: with the count open, an arrow key steps by one whole number (egui's step
        // of 0.1 rounded back): Down at 1000 gives 999, and Up at 999 gives 1000 again.
        let at = h.get_by_label("How many").rect().center();
        for pressed in [true, false] {
            h.event(egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            });
            h.run_steps(1);
        }
        h.run_steps(1);
        for (key, after) in [(egui::Key::ArrowDown, 999), (egui::Key::ArrowUp, 1_000)] {
            h.key_press(key);
            h.run_steps(2);
            assert_eq!(*h.state(), after, "{key:?} steps by one");
        }
    }

    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn enclosing_lock_describes_each_control_and_stays_inside_its_scope() {
        let ctx = egui::Context::default();
        super::super::theme::install(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                locked(ui, Some("Close the run first."), |ui| {
                    Action::quiet("Open").blocked(Some("Other reason")).show(ui);
                    icon_button(ui, icons::EXPAND, "More actions");
                    text_input(ui, "Folder", &mut String::new(), "", FORM_WIDTH);
                });
                Action::quiet("Outside").show(ui);
            });
        });
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        for label in ["Open", "More actions", "Folder"] {
            let (_, node) = nodes
                .iter()
                .find(|(_, node)| node.label() == Some(label))
                .unwrap();
            assert_eq!(node.description(), Some("Close the run first."));
        }
        let (_, outside) = nodes
            .iter()
            .find(|(_, node)| node.label() == Some("Outside"))
            .unwrap();
        assert!(outside.description().is_none());
    }

    #[test]
    fn cut_words_end_with_an_ellipsis_and_no_joining_word() {
        let context = egui::Context::default();
        super::super::theme::install(&context);
        let _ = context.run(Default::default(), |context| {
            egui::CentralPanel::default().show(context, |ui| {
                let font = Type::Strong.font(ui);
                let text = "Undo: 5 damage to Ghost of the Mill";
                for (room, expected) in [
                    ("Undo: 5 damage to Ghost of", "Undo: 5 damage to Ghost…"),
                    ("Undo: 5 damage to", "Undo: 5 damage…"),
                ] {
                    let width = number_width(ui, room, &font);
                    let fitted = words_that_fit(ui, text, &font, width);
                    assert_eq!(fitted, expected);
                    assert!(number_width(ui, &fitted, &font) <= width);
                }
                for joining in ["to", "of", "on", "from", "and", "the", "de", "von", "·"] {
                    let text = format!("Undo: 5 damage {joining} Ghost of the Mill");
                    // The joining word and ellipsis fit, so the word rule must remove it.
                    let width = number_width(ui, &format!("Undo: 5 damage {joining}…"), &font);
                    assert_eq!(words_that_fit(ui, &text, &font, width), "Undo: 5 damage…");
                }
                assert_eq!(
                    words_that_fit(ui, text, &font, number_width(ui, text, &font)),
                    text
                );
                for width in [40.0, 84.0, 120.0, 200.0] {
                    for detail in ["Ended · Round 2", "Round 2 · Paused"] {
                        let shown = run_nav_detail(ui, detail, width);
                        assert!(!shown.trim_end_matches('…').trim_end().ends_with('·'));
                        if number_width(ui, detail, &Type::Small.font(ui)) <= width {
                            assert_eq!(shown, detail);
                        } else {
                            assert_eq!(
                                shown,
                                if detail.starts_with("Ended") {
                                    "Ended"
                                } else {
                                    "Paused"
                                }
                            );
                        }
                    }
                }
            });
        });
    }

    #[test]
    fn first_draft_difference_is_visible_on_the_first_frame() {
        for (prefix, wrapped) in [(0, false), (3, false), (50, false), (50, true)] {
            let context = egui::Context::default();
            super::super::theme::install(&context);
            context.enable_accesskit();
            let context_lines = (0..prefix)
                .map(|n| {
                    if wrapped && n % 3 == 0 {
                        format!(
                            "Unchanged line {n}: {}\n",
                            "A longer unchanged paragraph. ".repeat(3)
                        )
                    } else if wrapped && n % 3 == 1 {
                        "\n".into()
                    } else {
                        format!("Unchanged line {n}\n")
                    }
                })
                .collect::<String>();
            let draft = format!("{context_lines}My changed draft line");
            let disk = format!("{context_lines}Changed elsewhere.\nExtra disk line");
            for width in [600.0, 900.0] {
                let output = context.run(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            vec2(width, 300.0),
                        )),
                        ..Default::default()
                    },
                    |context| {
                        egui::CentralPanel::default().show(context, |ui| {
                            draft_comparison(ui, &draft, &disk, "new conflict")
                        });
                    },
                );
                fn visible(shape: &egui::Shape, clip: egui::Rect, text: &str) -> bool {
                    match shape {
                        egui::Shape::Text(shape) => {
                            shape.galley.text() == text
                                && clip.contains_rect(shape.visual_bounding_rect())
                        }
                        egui::Shape::Vec(shapes) => {
                            shapes.iter().any(|shape| visible(shape, clip, text))
                        }
                        _ => false,
                    }
                }
                assert!(
                    output.shapes.iter().any(|shape| visible(
                        &shape.shape,
                        shape.clip_rect,
                        "My changed draft line"
                    )),
                    "prefix {prefix}: first draft difference is out of view"
                );
                if prefix >= 2 {
                    assert!(
                        output.shapes.iter().any(|shape| visible(
                            &shape.shape,
                            shape.clip_rect,
                            draft.lines().nth(prefix - 2).unwrap()
                        )),
                        "two context lines stay in view"
                    );
                }
                if prefix == 0 {
                    assert!(
                        !output.shapes.iter().any(|shape| visible(
                            &shape.shape,
                            shape.clip_rect,
                            "No line"
                        )),
                        "missing rows must not look like file text"
                    );
                    assert!(
                        output
                            .platform_output
                            .accesskit_update
                            .as_ref()
                            .unwrap()
                            .nodes
                            .iter()
                            .any(|(_, node)| node.label() == Some("No line")),
                        "missing rows keep their accessible name"
                    );
                }
            }
        }
    }

    #[test]
    fn counts_use_separators_and_hide_zero() {
        assert_eq!(count(0, "entry", "entries"), "");
        assert_eq!(count(1, "entry", "entries"), "1 entry");
        assert_eq!(count(2108, "entry", "entries"), "2,108 entries");
        assert_eq!(count(1234567, "", ""), "1,234,567");
    }

    #[test]
    fn comparison_heading_sits_beside_the_line_count() {
        for disk in ["# Recovery\nsame\nnew\nnext", "# Recovery\nsame\nnew\nlast"] {
            let ctx = egui::Context::default();
            super::super::theme::install(&ctx);
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        vec2(524.0, 300.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        grouped_draft_comparison(
                            ui,
                            "# Recovery\nsame\nold\nlast",
                            disk,
                            disk,
                            true,
                        );
                    });
                },
            );
            let count = if disk.ends_with("next") {
                "2 lines differ"
            } else {
                "1 line differs"
            };
            let words = format!("{count} · Under the heading “Recovery”");
            let text = output
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.text() == words => Some(text),
                    _ => None,
                })
                .expect("The count and heading share one label");
            assert_eq!(text.galley.rows.len(), 1, "The summary fits on one line");
        }
    }

    use crate::input_frame::painted_texts;

    /// SD1: an Undo below its message starts its words at the message's words; its icon hangs
    /// over the message's icon column. Beside the message, it keeps its icon before its words.
    #[test]
    fn iter18_a_sd1_a_wrapped_undo_starts_its_words_at_the_message_words() {
        use crate::design::test_harness::Themed as _;
        use crate::design::test_harness::ThemedPage as _;
        const TEXT: &str = "Ashen Reaches is registered and used here.";
        const UNDO: &str = "Undo package registration";
        for (width, wraps) in [(260.0, true), (360.0, true), (900.0, false)] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 200.0))
                .themed_ui(|ui| {
                    undo_message(ui, Tone::Success, TEXT, UNDO);
                });
            harness.run_steps(2);
            let texts = painted_texts(&harness);
            let left = |words: &str| {
                texts
                    .iter()
                    .find(|(text, _)| text == words)
                    .map(|(_, rect)| *rect)
                    .unwrap_or_else(|| panic!("{words} is painted at {width}: {texts:?}"))
            };
            let (message, action, icon) = (left(TEXT), left(UNDO), left(icons::UNDO));
            if wraps {
                assert!(
                    action.top() >= message.bottom() - 1.0
                        && (action.left() - message.left()).abs() <= 1.0
                        && icon.right() <= message.left(),
                    "SD1-wrapped at {width}: message {message:?}, Undo {action:?}, icon {icon:?}"
                );
            } else {
                assert!(
                    (action.center().y - message.center().y).abs() < Density::Read.control()
                        && icon.left() > message.right()
                        && action.left() > icon.right(),
                    "SD1-beside at {width}: message {message:?}, Undo {action:?}, icon {icon:?}"
                );
            }
        }
        // P3a, ruling R19 (proof 3): a hanging Undo keeps keyboard focus, Enter, Space and an
        // AccessKit click with the pointer anywhere; one input frame per step, counted in it.
        use crate::input_frame::one_pass;
        use egui_kittest::kittest::{NodeT, Queryable};
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(360.0, 200.0))
            .themed_ui_state(
                |ui, count: &mut usize| {
                    // Behind a parent clip from x 100, so a pointer at x 40 is outside its room;
                    // 260 px of room wraps the line, so the Undo hangs (`line_start_in_column`).
                    let room = ui.available_rect_before_wrap().with_min_x(100.0);
                    ui.scope_builder(egui::UiBuilder::new().max_rect(room), |ui| {
                        let mut clip = ui.clip_rect();
                        clip.min.x = 100.0;
                        ui.set_clip_rect(clip);
                        *count += usize::from(undo_message(ui, Tone::Success, TEXT, UNDO));
                    });
                },
                0_usize,
            );
        harness.run_steps(2);
        let undo = harness.get_by_label(UNDO).accesskit_node().id();
        harness.get_by_label(UNDO).focus();
        harness.run_steps(2);
        let focused = |harness: &egui_kittest::Harness<'_, usize>| {
            harness
                .ctx
                .memory(|memory| memory.focused())
                .map(|id| id.value())
                == Some(undo.0)
        };
        assert!(focused(&harness), "undo-focus-setup: the Undo has focus");
        let undo_rect = harness.get_by_label(UNDO).rect();
        one_pass(
            &mut harness,
            [egui::Event::PointerMoved(pos2(40.0, undo_rect.center().y))],
        );
        assert!(
            focused(&harness) && *harness.state() == 0,
            "undo-pointer-away: the pointer away from the Undo keeps its focus and presses nothing ({})",
            harness.state()
        );
        let key = |key, pressed| egui::Event::Key {
            key,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        let mut counts = Vec::new();
        for event in [
            key(egui::Key::Enter, true),
            key(egui::Key::Enter, false),
            key(egui::Key::Space, true),
            key(egui::Key::Space, false),
            egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
                action: egui::accesskit::Action::Click,
                target: undo,
                data: None,
            }),
        ] {
            one_pass(&mut harness, [event]);
            counts.push(*harness.state());
        }
        assert_eq!(
            counts,
            vec![1, 1, 2, 2, 3],
            "undo-keys: Enter, release, Space, release and an AccessKit click with the pointer away"
        );
    }

    /// Every section form says to a screen reader whether it is open, also one its owner opens,
    /// and the state follows a click on its own header only.
    #[test]
    fn a_section_says_whether_it_is_open() {
        use crate::design::test_harness::ThemedPage as _;
        let notes = std::rc::Rc::new(std::cell::Cell::new("3 notes"));
        let title = notes.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(400.0, 300.0))
            .themed_ui(move |ui| {
                let _ = section(ui, "Details", |ui| ui.label("Inside details"));
                let _ = section_with_id(ui, "notes", title.get(), |ui| ui.label("Inside notes"));
                let _ = quiet_section(ui, "Technical", |ui| ui.label("Inside technical"));
                // A section its owner opens (a History run read from a link).
                let _ = section_opened(ui, "runs", "2 runs", Some(true), |ui| {
                    ui.label("Inside runs")
                });
                // M0412: open the first time it is drawn, then the DM's (an empty list's form).
                let _ = Section::new("Add campaign")
                    .id("campaign_form")
                    .default_open(true)
                    .show(ui, |ui| ui.label("Inside the form"));
            });
        harness.run_steps(2);
        let expanded = |harness: &egui_kittest::Harness<'_>| {
            use egui_kittest::kittest::{NodeT, Queryable};
            ["Details", "3 notes", "Technical", "2 runs", "Add campaign"].map(|name| {
                harness
                    .get_by_label(name)
                    .accesskit_node()
                    .data()
                    .is_expanded()
            })
        };
        assert_eq!(
            expanded(&harness),
            [
                Some(false),
                Some(false),
                Some(false),
                Some(true),
                Some(true)
            ],
            "closed at first; the owner's open one and the one open by default say they are open"
        );
        {
            use egui_kittest::kittest::Queryable;
            harness.get_by_label("3 notes").click();
        }
        harness.run_steps(3);
        assert_eq!(
            expanded(&harness),
            [Some(false), Some(true), Some(false), Some(true), Some(true)],
            "only the clicked section opens"
        );
        {
            use egui_kittest::kittest::Queryable;
            harness.get_by_label("Add campaign").click();
        }
        harness.run_steps(3);
        assert_eq!(
            expanded(&harness)[4],
            Some(false),
            "m-p0-section-default-open: the DM closes a section open by default, and it stays closed"
        );
        // Review of PR 730: a section with an id keeps its state when its count changes (a
        // repair in Campaign content's issues).
        notes.set("2 notes");
        harness.run_steps(3);
        assert_eq!(
            {
                use egui_kittest::kittest::{NodeT, Queryable};
                harness
                    .get_by_label("2 notes")
                    .accesskit_node()
                    .data()
                    .is_expanded()
            },
            Some(true),
            "m-p0-section-id: the open section stays open under its new count"
        );
        assert!(
            {
                use egui_kittest::kittest::Queryable;
                harness.query_by_label("Inside the form").is_none()
            },
            "m-p0-section-default-open: its body goes with it"
        );
    }

    /// SD1, ST2: a `line_start` quiet action puts its words at the text edge. With an icon, the
    /// icon and the padding hang left of it (`S + ICON_SLOT`); without one, only the padding.
    /// A normal quiet action keeps its icon and words inside its target from the edge.
    #[test]
    fn iter18_a_a_line_start_action_hangs_its_icon_and_padding() {
        use crate::design::test_harness::ThemedPage as _;
        const EDGE: f32 = 60.0;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(400.0, 200.0))
            .themed_ui(|ui| {
                // P3a: the last two rows' clip leaves the hang short: none at all (a menu's
                // content edge), and 10 px for an icon's 28.
                for (label, icon, line_start, room) in [
                    ("Undo heading change", true, true, None),
                    ("Clear value", false, true, None),
                    ("Undo replacement", true, false, None),
                    ("Menu choice", false, true, Some(0.0)),
                    ("Undo tight", true, true, Some(10.0)),
                ] {
                    ui.horizontal(|ui| {
                        if let Some(room) = room {
                            let mut clip = ui.clip_rect();
                            clip.min.x = EDGE - room;
                            ui.set_clip_rect(clip);
                        }
                        space(ui, EDGE - ui.cursor().left());
                        let action = Action::quiet(label);
                        let action = if icon {
                            action.icon(icons::UNDO)
                        } else {
                            action
                        };
                        let action = if line_start {
                            action.line_start()
                        } else {
                            action
                        };
                        let _ = action.show(ui);
                    });
                }
            });
        harness.run_steps(2);
        let texts = painted_texts(&harness);
        let node = |name: &str| {
            use egui_kittest::kittest::Queryable;
            harness.get_by_label(name).rect()
        };
        let words = |label: &str| {
            texts
                .iter()
                .find(|(text, _)| text == label)
                .map(|(_, rect)| rect.left())
                .unwrap()
        };
        let hanging = node("Undo heading change");
        assert!(
            (words("Undo heading change") - EDGE).abs() <= 0.5
                && (hanging.left() - (EDGE - S - ICON_SLOT)).abs() <= 0.5,
            "hang-icon: words {}, target {hanging:?}",
            words("Undo heading change")
        );
        let plain = node("Clear value");
        assert!(
            (words("Clear value") - EDGE).abs() <= 0.5 && (plain.left() - (EDGE - S)).abs() <= 0.5,
            "hang-plain: words {}, target {plain:?}",
            words("Clear value")
        );
        let normal = node("Undo replacement");
        assert!(
            (normal.left() - EDGE).abs() <= 0.5
                && (words("Undo replacement") - (EDGE + S + ICON_SLOT)).abs() <= 0.5,
            "no-hang: words {}, target {normal:?}",
            words("Undo replacement")
        );
        // P3a: the action is whole in its clip. With no room, it gives up its left padding and
        // its words stay at the text edge (a menu's choices under its heading); with 10 px for
        // an icon (20 px slot), the words move by the 10 px that are missing.
        let menu = node("Menu choice");
        assert!(
            (words("Menu choice") - EDGE).abs() <= 0.5 && (menu.left() - EDGE).abs() <= 0.5,
            "hang-none: words {} at the edge {EDGE}, target {menu:?} from the clip",
            words("Menu choice")
        );
        let tight = node("Undo tight");
        assert!(
            (words("Undo tight") - (EDGE + ICON_SLOT - 10.0)).abs() <= 0.5
                && (tight.left() - (EDGE - 10.0)).abs() <= 0.5,
            "hang-short: words {} after the icon, target {tight:?} from the clip at {}",
            words("Undo tight"),
            EDGE - 10.0
        );
    }

    /// iter-25-A: `help` and a field's help wrap at `READER_MEASURE` in a wide pane.
    #[test]
    fn iter25_a_help_keeps_the_reader_measure() {
        use crate::design::test_harness::ThemedPage as _;
        const HELP: &str = "Built-in names stay as they are. A name you add here shows in every \
            list of this campaign, and Lookup finds it by its first word or its whole name.";
        for in_field in [false, true] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(1600.0, 240.0))
                .themed_ui(move |ui| {
                    if in_field {
                        field(ui, "Name", Some(HELP), None, |ui| ui.label("control"));
                        // N3: the next field's label, with no space from the caller.
                        field(ui, "Next label", None, Some("State the fix."), |ui| {
                            ui.label("second control")
                        });
                        field(ui, "Last label", None, None, |ui| ui.label("third control"));
                        // Layout PR 10d: a field's warning is its control's description.
                        small_field_needing(
                            ui,
                            "Needed label",
                            Some("Enter the address."),
                            None,
                            |ui| ui.button("Needed control"),
                        );
                    } else {
                        help(ui, HELP);
                    }
                });
            harness.run_steps(2);
            let texts = painted_texts(&harness);
            let rect = texts
                .iter()
                .find(|(text, _)| text == HELP)
                .map(|(_, rect)| *rect)
                .unwrap_or_else(|| panic!("the help is painted: {texts:?}"));
            assert!(
                rect.width() <= READER_MEASURE + 0.5 && rect.height() > Type::Small.line(),
                "field {in_field}: help {rect:?}"
            );
            if in_field {
                // N3: a field adds `L` after its help and after its error, so neither reads as
                // the next field's.
                let top = |label: &str| {
                    texts
                        .iter()
                        .find(|(text, _)| text == label)
                        .map(|(_, rect)| rect.top())
                        .unwrap_or_else(|| panic!("{label} is painted"))
                };
                let gap = top("Next label") - rect.bottom();
                assert!((gap - L).abs() <= 0.5, "n3-help: {gap} from help to label");
                let error = texts
                    .iter()
                    .find(|(text, _)| text == "State the fix.")
                    .map(|(_, rect)| rect.bottom())
                    .expect("the error is painted");
                let gap = top("Last label") - error;
                assert!(
                    (gap - L).abs() <= 0.5,
                    "n3-error: {gap} from error to label"
                );
                use egui_kittest::kittest::{NodeT, Queryable};
                let description = harness
                    .get_by_label("Needed control")
                    .accesskit_node()
                    .description()
                    .map(|words| words.to_string());
                assert_eq!(
                    description.as_deref(),
                    Some("Enter the address."),
                    "warning-described: the control says its field's warning"
                );
            }
        }
    }

    /// Row 86: a segment is as wide as its words in their widest state, so no word and no
    /// segment moves when the choice changes.
    #[test]
    fn segments_keep_their_place_when_the_choice_changes() {
        use crate::design::test_harness::ThemedPage as _;
        let place = |chosen: u8| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(600.0, 80.0))
                .themed_ui(move |ui| {
                    let mut value = chosen;
                    ui.horizontal(|ui| {
                        segmented(
                            ui,
                            &mut value,
                            &[(0, "Recordings"), (1, "Transcript"), (2, "Notes")],
                        );
                        ui.label("After");
                    });
                });
            harness.run_steps(2);
            let texts = painted_texts(&harness);
            ["Recordings", "Transcript", "Notes", "After"].map(|word| {
                texts
                    .iter()
                    .find(|(text, _)| text == word)
                    .map(|(_, rect)| rect.left())
                    .unwrap_or_else(|| panic!("{word} is painted"))
            })
        };
        let first = place(0);
        for chosen in [1, 2] {
            assert_eq!(
                place(chosen),
                first,
                "row-86: words moved when {chosen} was chosen"
            );
        }
    }

    /// iter-25-A contract: the message space is two body lines high whether it is empty, holds
    /// one line, holds more than fits or holds a reason and a result, so what follows it never
    /// moves. Empty, it draws nothing. A result with a live Undo comes before the reason; without
    /// one, the reason comes first. The part is one focusable group named with every message in
    /// the order drawn.
    #[test]
    fn iter25_a_message_space_keeps_what_follows_still() {
        use crate::design::test_harness::ThemedPage as _;
        const LONG: &str = "Line 4 opens a list with [ that never closes. Close the list with ], \
            or remove the [ at the start of the line. Then save the entry again to check it.";
        const REASON: &str = "Splat cannot read Notes/Broken.md as text.";
        const RESULT: &str = "Broken draft discarded.";
        // (reason, result and its has_undo) → (gap from above to below, tops of the reason and
        // the result, whether the result's add ran, the part's name)
        type Seen = (f32, Option<f32>, Option<f32>, bool, Option<String>);
        let space = |reason: Option<&'static str>, result: Option<(&'static str, bool)>| -> Seen {
            let ran = std::rc::Rc::new(std::cell::Cell::new(false));
            let seen = ran.clone();
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(330.0, 400.0))
                .themed_ui(move |ui| {
                    ui.label("Above the space");
                    let mut space = MessageSpace::new("test_space", 330.0);
                    if let Some(words) = reason {
                        space = space.reason(words, move |ui| {
                            message(ui, Tone::Error, words);
                        });
                    }
                    if let Some((words, has_undo)) = result {
                        let seen = seen.clone();
                        space = space.result(words, has_undo, move |ui| {
                            seen.set(true);
                            message(ui, Tone::Info, words);
                        });
                    }
                    space.show(ui);
                    ui.label("Below the space");
                });
            harness.run_steps(2);
            let texts = painted_texts(&harness);
            let top = |words: &str| {
                texts
                    .iter()
                    .find(|(text, _)| text == words)
                    .map(|(_, rect)| rect.top())
            };
            use egui_kittest::kittest::NodeT;
            let name = harness
                .root()
                .children_recursive()
                .find(|node| {
                    node.accesskit_node().role() == egui::accesskit::Role::GenericContainer
                        && node.accesskit_node().label().is_some()
                })
                .and_then(|node| node.accesskit_node().label());
            (
                top("Below the space").unwrap() - top("Above the space").unwrap(),
                reason.and_then(&top),
                result.and_then(|(words, _)| top(words)),
                ran.get(),
                name,
            )
        };
        let empty = space(None, None);
        let short = space(None, Some(("Typed 4 on 1d6: Fog", false)));
        let long = space(Some(LONG), None);
        let undo_first = space(Some(REASON), Some((RESULT, true)));
        let reason_first = space(Some(REASON), Some((RESULT, false)));
        assert!(
            !empty.3 && empty.4.is_none() && short.3 && undo_first.3 && reason_first.3,
            "the result's add runs only with a result; an empty space has no part"
        );
        for seen in [&short, &long, &undo_first, &reason_first] {
            assert!(
                (empty.0 - seen.0).abs() < 0.5,
                "below the space: empty {}, now {}",
                empty.0,
                seen.0
            );
        }
        assert!(empty.0 >= message_space_height(), "{}", empty.0);
        assert_eq!(long.4.as_deref(), Some(LONG));
        // With a live Undo, the result comes first; without one, the reason does.
        let (reason, result) = (undo_first.1.unwrap(), undo_first.2.unwrap());
        assert!(
            result < reason,
            "undo first: result {result}, reason {reason}"
        );
        assert_eq!(undo_first.4, Some(format!("{RESULT} {REASON}")));
        let (reason, result) = (reason_first.1.unwrap(), reason_first.2.unwrap());
        assert!(
            reason < result,
            "reason first: reason {reason}, result {result}"
        );
        assert_eq!(reason_first.4, Some(format!("{REASON} {RESULT}")));
    }

    /// iter-25-A (carried, World at 960 beside Lookup): the header's count stays whole; a status
    /// link among the actions wraps in the width the title and count leave. Must stay true: in a
    /// wide pane, the links stay on one line and the header keeps its height.
    #[test]
    fn iter25_a_page_header_keeps_its_count_whole() {
        use crate::design::test_harness::ThemedPage as _;
        const COUNT: &str = "5 notes";
        const FIXING: &str = "1 note needs fixing";
        const UNREAD: &str = "2 notes cannot be read";
        let mut wide_below = Vec::new();
        for (width, actions) in [(465.0, true), (1200.0, true), (1200.0, false)] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 200.0))
                .themed_ui(move |ui| {
                    page_header(ui, "World", Some(COUNT), |ui| {
                        if !actions {
                            return;
                        }
                        Action::secondary("New").icon(icons::ADD).show(ui);
                        for (word, icon) in [(FIXING, icons::WARN), (UNREAD, icons::ERROR)] {
                            status_link(
                                ui,
                                Status {
                                    icon,
                                    word,
                                    color: colors().result.warning,
                                },
                            );
                        }
                    });
                    ui.label("Below the header");
                });
            harness.run_steps(2);
            let mut painted = Vec::new();
            for shape in &harness.output().shapes {
                if let egui::Shape::Text(text) = &shape.shape {
                    painted.push((
                        text.galley.job.text.clone(),
                        !text.galley.elided,
                        text.galley.rows.len(),
                        Rect::from_min_size(text.pos, text.galley.size()),
                    ));
                }
            }
            let find = |words: &str| {
                painted
                    .iter()
                    .find(|(text, ..)| text.ends_with(words))
                    .unwrap_or_else(|| panic!("{words} is painted at {width}: {painted:?}"))
                    .clone()
            };
            let count = find(COUNT);
            if width > 1000.0 {
                wide_below.push(find("Below the header").3.top());
            }
            if !actions {
                continue;
            }
            let unread = find(UNREAD);
            assert!(
                count.1 && unread.1 && count.3.right() <= unread.3.left(),
                "at {width}: count {count:?}, link {unread:?}"
            );
            if width > 1000.0 {
                assert!(
                    painted.iter().all(|(_, _, rows, _)| *rows == 1),
                    "at {width}, every header text is one line: {painted:?}"
                );
            }
        }
        assert!(
            (wide_below[0] - wide_below[1]).abs() < 0.5,
            "a wide header keeps its height with its actions: {wide_below:?}"
        );
    }

    /// iter-25-A FG6: a two-line rail status starts its icon and its words where a one-line
    /// status starts them.
    #[test]
    fn iter25_a_two_line_status_shares_the_one_line_edges() {
        use crate::design::test_harness::ThemedPage as _;
        let state = || Status {
            icon: icons::WARN,
            word: "Connected",
            color: colors().result.warning,
        };
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(200.0, 200.0))
            .themed_ui(move |ui| {
                status(ui, state());
                rail_status_detail(ui, state(), "Not remembered");
            });
        harness.run_steps(2);
        // Each painted text with the x of each of its glyphs.
        let mut glyphs = Vec::new();
        for shape in &harness.output().shapes {
            if let egui::Shape::Text(text) = &shape.shape {
                let row = &text.galley.rows[0];
                let xs = row
                    .glyphs
                    .iter()
                    .map(|glyph| (glyph.chr, text.pos.x + glyph.pos.x))
                    .collect::<Vec<_>>();
                glyphs.push((text.galley.job.text.clone(), xs));
            }
        }
        let one_line = format!("{} Connected", icons::WARN);
        let line = glyphs
            .iter()
            .find(|(text, _)| *text == one_line)
            .expect("the one-line status is painted");
        let icon_x = line.1[0].1;
        let word_x = line.1.iter().find(|(chr, _)| *chr == 'C').unwrap().1;
        let x_of = |words: &str| {
            glyphs
                .iter()
                .find(|(text, _)| text == words)
                .unwrap_or_else(|| panic!("{words} is painted: {glyphs:?}"))
                .1[0]
                .1
        };
        let (icon, word, detail) = (x_of(icons::WARN), x_of("Connected"), x_of("Not remembered"));
        assert!(
            (icon - icon_x).abs() <= 0.5
                && (word - word_x).abs() <= 0.5
                && (detail - word).abs() <= 0.5,
            "one line: icon {icon_x}, word {word_x}; two lines: icon {icon}, word {word}, detail {detail}"
        );
    }

    /// iter-26-A FB3, AJ1, FM3: a rail status that wraps, through `wrapped_status` or
    /// `status_link`, starts its second line at the word, where a one-line status starts it.
    #[test]
    fn iter26_a_a_wrapped_status_starts_its_second_line_at_the_word() {
        use crate::design::test_harness::ThemedPage as _;
        const WRAPPED: &str = "Timeline not readable";
        const LINKED: &str = "Session folder not found";
        // Issue #585: a status that reserves two lines keeps to them however long its word.
        const CAPPED: &str = "Not in this campaign’s packages, so it cannot join the roster yet";
        let state = |word: &'static str| Status {
            icon: icons::WARN,
            word,
            color: colors().result.warning,
        };
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(200.0, 300.0))
            .themed_ui(move |ui| {
                status(ui, state("Connected"));
                // As narrow as a rail at 960, so both wrap.
                ui.set_max_width(100.0);
                wrapped_status(ui, state(WRAPPED));
                let _ = status_link(ui, state(LINKED));
                wrapped_status_rows(ui, state(CAPPED), 2);
            });
        harness.run_steps(2);
        // The x of each row's first glyph, by the words of the painted text.
        let mut rows = Vec::new();
        for shape in &harness.output().shapes {
            if let egui::Shape::Text(text) = &shape.shape {
                let xs = text
                    .galley
                    .rows
                    .iter()
                    .map(|row| text.pos.x + row.pos.x + row.glyphs[0].pos.x)
                    .collect::<Vec<_>>();
                rows.push((text.galley.job.text.clone(), xs));
            }
        }
        let one_line = format!("{} Connected", icons::WARN);
        let word_x = harness
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == one_line => {
                    let glyph = text.galley.rows[0].glyphs.iter().find(|g| g.chr == 'C')?;
                    Some(text.pos.x + glyph.pos.x)
                }
                _ => None,
            });
        let word_x = word_x.expect("the one-line status is painted");
        for words in [WRAPPED, LINKED] {
            let xs = &rows
                .iter()
                .find(|(text, _)| text == words)
                .unwrap_or_else(|| panic!("{words} is painted on its own: {rows:?}"))
                .1;
            assert!(
                xs.len() == 2 && xs.iter().all(|x| (x - word_x).abs() <= 0.5),
                "{words} wraps to two lines, each at the word x {word_x}: {xs:?}"
            );
        }
        // Issue #585: two lines at the word, the second ending in `…`; the whole words are its
        // accessible name.
        let capped = harness
            .output()
            .shapes
            .iter()
            .find_map(|shape| match &shape.shape {
                egui::Shape::Text(text) if text.galley.job.text == CAPPED => {
                    Some(text.galley.clone())
                }
                _ => None,
            })
            .expect("the capped status is painted");
        let last = capped
            .rows
            .last()
            .and_then(|row| row.glyphs.last())
            .map(|glyph| glyph.chr);
        assert!(
            capped.rows.len() == 2 && last == Some('…'),
            "#585: the capped status keeps to two lines and ends in …: {} rows, last {last:?}",
            capped.rows.len()
        );
        use egui_kittest::kittest::Queryable;
        assert!(
            harness.query_by_label(CAPPED).is_some(),
            "#585: its whole words stay its accessible name"
        );
    }

    /// iter-26-A footer contract: the body's top is the same with 0, 1, 3 and 10 messages, at
    /// 960, 1200 and 1920; the footer is never more than a third of the pane; and with 20, the last
    /// message is reached by Tab with the arrow keys and by the wheel. Its focus ring stays clear
    /// of every word. The last Undo, reached by keyboard and by pointer, ends its own result only:
    /// the other 19 stay, with their Undo.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    fn iter26_a_the_footer_keeps_the_body_top_and_reaches_its_last_message() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::{NodeT, Queryable};
        use std::{cell::RefCell, rc::Rc};
        const BODY: &str = "The body starts here.";
        let last = |count: usize| format!("Result {} of {count} kept with its Undo.", count - 1);
        // Which results are still live; pressing a result's Undo ends that one.
        let harness_with = |size: Vec2, live: Rc<RefCell<Vec<bool>>>| {
            let count = live.borrow().len();
            // Each result's action, made before the harness: result 0 is the newest, so the
            // footer lists them from 0 down.
            let mut orders: Vec<ResultOrder> = (0..count).map(|_| ResultOrder::issue()).collect();
            orders.reverse();
            let mut harness =
                egui_kittest::Harness::builder()
                    .with_size(size)
                    .themed_ui(move |ui| {
                        let words: Vec<String> = (0..count)
                            .map(|at| format!("Result {at} of {count} kept with its Undo."))
                            .collect();
                        // A save row above the footer, as in a pane: the body starts below it.
                        ui.label("Save row");
                        let mut footer = Footer::new("test_footer", ui.available_width());
                        let shown: Vec<bool> = live.borrow().clone();
                        for (at, words) in words.iter().enumerate().filter(|(at, _)| shown[*at]) {
                            let live = &live;
                            footer.result(orders[at], words.as_str(), true, move |ui| {
                                if undo_message_capped(
                                    ui,
                                    Tone::Success,
                                    words,
                                    "Undo",
                                    Some(&format!("Undo {at}")),
                                ) {
                                    live.borrow_mut()[at] = false;
                                }
                            });
                        }
                        if count > 1 {
                            footer.legacy_reason("Line 3 opens a list.", |ui| {
                                message(ui, Tone::Error, "Line 3 opens a list.");
                            });
                        }
                        let bottom = footer.show_by_hand(ui);
                        let body = egui::ScrollArea::vertical()
                            .id_salt("test_body")
                            .max_height(bottom - ui.cursor().top())
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                ui.label(BODY);
                            })
                            .inner_rect;
                        ui.ctx()
                            .data_mut(|data| data.insert_temp(Id::new("test_body"), body));
                    });
            harness.run_steps(3);
            harness
        };
        let harness =
            |size: Vec2, count: usize| harness_with(size, Rc::new(RefCell::new(vec![true; count])));
        // The last Undo ended its result alone: the other 19 stay, with their Undo.
        let only_last_ended = |harness: &egui_kittest::Harness<'_>,
                               live: &Rc<RefCell<Vec<bool>>>| {
            let live = live.borrow();
            !live[19]
                && live[..19].iter().all(|live| *live)
                && harness.query_by_label("Undo 19").is_none()
                && (0..19).all(|at| harness.query_by_label(&format!("Undo {at}")).is_some())
        };
        let text_at = |harness: &egui_kittest::Harness<'_>, words: &str| {
            harness
                .output()
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == words => Some((
                        Rect::from_min_size(text.pos, text.galley.size()),
                        shape.clip_rect,
                    )),
                    _ => None,
                })
        };
        let footer_rect = |harness: &egui_kittest::Harness<'_>| {
            harness
                .ctx
                .read_response(Id::new("test_footer").with("focus"))
                .map(|response| response.rect)
        };
        for size in [
            vec2(960.0, 640.0),
            vec2(1200.0, 800.0),
            vec2(1920.0, 1080.0),
        ] {
            let tops: Vec<_> = [0, 1, 3, 10]
                .map(|count| {
                    let harness = harness(size, count);
                    let body = harness
                        .ctx
                        .data(|data| data.get_temp::<Rect>(Id::new("test_body")))
                        .expect("the body is drawn");
                    let (row, _) = text_at(&harness, "Save row").unwrap();
                    assert!(
                        body.top() >= row.bottom(),
                        "{count} at {size:?}: the body {body:?} starts below the save row {row:?}"
                    );
                    if count > 0 {
                        // iter-27-A: the body here is a scroll area as tall as its room, so it
                        // fills the pane and the footer is pinned; its words end above the
                        // footer, and the body is clipped at the footer's top, so it never shows
                        // or takes a press under it.
                        let footer = footer_rect(&harness).expect("the footer is drawn");
                        let (words, clip) = text_at(&harness, BODY).expect("the body's words show");
                        assert!(
                            footer.height() <= size.y / 3.0 + 1.0
                                && words.bottom() <= footer.top() - S + 0.5
                                && clip.bottom() <= footer.top() + 0.5
                                && footer.bottom() <= size.y,
                            "{count} at {size:?}: the body's words {words:?} end `S` above the footer {footer:?}, a third of the pane at most, and the body is clipped at it: {clip:?}"
                        );
                    } else {
                        assert!(
                            body.bottom() >= size.y - L,
                            "With no message the body {body:?} takes the pane at {size:?}"
                        );
                    }
                    text_at(&harness, BODY).map(|(rect, _)| rect.top())
                })
                .into();
            assert!(
                tops[0].is_some() && tops.iter().all(|top| *top == tops[0]),
                "The body's top at {size:?} with 0, 1, 3, 10 messages: {tops:?}"
            );
            // The last of 20 messages (more than fit at every size), fully in the footer's view.
            let shown = |harness: &egui_kittest::Harness<'_>| {
                text_at(harness, &last(20)).is_some_and(|(rect, clip)| {
                    footer_rect(harness).is_some_and(|footer| {
                        clip.intersect(footer).expand(0.5).contains_rect(rect)
                    })
                })
            };
            // Tab: from the start, to the footer, then the arrow keys scroll it.
            let mut keyboard = harness(size, 20);
            assert!(!shown(&keyboard), "20 messages overflow at {size:?}");
            let focus = Id::new("test_footer").with("focus");
            // Each Undo is a stop before the footer itself.
            for _ in 0..48 {
                if keyboard.ctx.memory(|memory| memory.has_focus(focus)) {
                    break;
                }
                keyboard.key_press(egui::Key::Tab);
                keyboard.run_steps(2);
            }
            assert!(
                keyboard.ctx.memory(|memory| memory.has_focus(focus)),
                "Tab reaches the footer at {size:?}"
            );
            for _ in 0..40 {
                if shown(&keyboard) {
                    break;
                }
                keyboard.key_press(egui::Key::ArrowDown);
                keyboard.run_steps(2);
            }
            assert!(
                shown(&keyboard),
                "the arrows reach the last message at {size:?}"
            );
            // Every word in the footer is inside the painted ring, clear of its stroke.
            let footer = footer_rect(&keyboard).unwrap();
            let mut rings = Vec::new();
            for shape in &keyboard.output().shapes {
                if let egui::Shape::Rect(ring) = &shape.shape
                    && ring.stroke.color == colors().focus
                {
                    rings.push(ring.rect);
                }
            }
            assert!(
                rings.len() == 1 && footer.contains_rect(rings[0]),
                "one focus ring, the footer's, at {size:?}: {rings:?}"
            );
            let ring = rings[0].shrink(FOCUS_WIDTH);
            for shape in &keyboard.output().shapes {
                if let egui::Shape::Text(text) = &shape.shape {
                    let rect = text.visual_bounding_rect().intersect(shape.clip_rect);
                    if rect.is_positive() && footer.intersects(rect) {
                        assert!(
                            ring.contains_rect(rect),
                            "{:?} at {rect:?} is clear of the ring {footer:?}",
                            text.galley.job.text
                        );
                    }
                }
            }
            // The last Undo by keyboard: Tab from the start until it has focus, as each Undo is
            // a stop before the footer group; it is then in view, and Enter presses it.
            let keyboard_live = Rc::new(RefCell::new(vec![true; 20]));
            let mut keyboard = harness_with(size, keyboard_live.clone());
            let undo_id = keyboard
                .query_by_label("Undo 19")
                .map(|undo| undo.accesskit_node().id());
            let focused = |harness: &egui_kittest::Harness<'_>| {
                harness
                    .ctx
                    .memory(|memory| memory.focused())
                    .map(|id| egui::accesskit::NodeId(id.value()))
            };
            for _ in 0..48 {
                if undo_id.is_some() && focused(&keyboard) == undo_id {
                    break;
                }
                keyboard.key_press(egui::Key::Tab);
                keyboard.run_steps(2);
            }
            assert!(
                undo_id.is_some() && focused(&keyboard) == undo_id && shown(&keyboard),
                "Tab reaches the last Undo, in view, at {size:?}: focus {}, shown {}",
                focused(&keyboard) == undo_id,
                shown(&keyboard)
            );
            keyboard.key_press(egui::Key::Enter);
            keyboard.run_steps(3);
            assert!(
                only_last_ended(&keyboard, &keyboard_live),
                "Enter on the last Undo ends its result only at {size:?}: {:?}",
                keyboard_live.borrow()
            );
            // The wheel over the footer.
            let wheel_live = Rc::new(RefCell::new(vec![true; 20]));
            let mut wheel = harness_with(size, wheel_live.clone());
            let over = footer_rect(&wheel).unwrap().center();
            wheel.event(egui::Event::PointerMoved(over));
            for _ in 0..40 {
                if shown(&wheel) {
                    break;
                }
                wheel.event(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: vec2(0.0, -Type::Body.line()),
                    modifiers: egui::Modifiers::NONE,
                });
                wheel.run_steps(2);
            }
            assert!(
                shown(&wheel),
                "the wheel reaches the last message at {size:?}"
            );
            // The last Undo by pointer, where the wheel showed it.
            let undo = wheel.get_by_label("Undo 19").rect();
            let footer = footer_rect(&wheel).unwrap();
            assert!(
                footer.contains_rect(undo),
                "the last Undo {undo:?} is in the footer {footer:?} at {size:?}"
            );
            wheel.event(egui::Event::PointerMoved(undo.center()));
            for pressed in [true, false] {
                wheel.event(egui::Event::PointerButton {
                    pos: undo.center(),
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                });
                wheel.run_steps(1);
            }
            wheel.run_steps(2);
            assert!(
                only_last_ended(&wheel, &wheel_live),
                "A click on the last Undo ends its result only at {size:?}: {:?}",
                wheel_live.borrow()
            );
        }
        iter27_a_the_footer_follows_a_short_body();
        iter27_a_footer_results_go_newest_first_by_their_action();
    }

    /// Lead note (optimizer scan 26): each footer result is one item with the order its action
    /// issued. The footer lists results with a live Undo newest first whatever order the caller
    /// adds them in: A then B, B then A; removing a middle result keeps the others' places; A,
    /// B, Undo A, then A again is newer than B. A redraw or changed words issue nothing. A reason
    /// stays after the results, and a result with no live Undo after the notices.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state` (design::results::STAGED_ALLOWS)"
    )]
    fn iter27_a_footer_results_go_newest_first_by_their_action() {
        // The words, top down, of a footer that holds `results` (added in this order) with a
        // reason and a notice added first.
        let painted = |results: &[(&str, ResultOrder, bool)]| -> Vec<String> {
            let results: Vec<(String, ResultOrder, bool)> = results
                .iter()
                .map(|(words, order, undo)| ((*words).to_owned(), *order, *undo))
                .collect();
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(960.0, 640.0))
                .build_ui(move |ui| {
                    let mut footer = Footer::new("order_footer", ui.available_width());
                    footer.legacy_notice("Notice.", |ui| {
                        ui.label("Notice.");
                    });
                    footer.legacy_reason("Reason.", |ui| {
                        ui.label("Reason.");
                    });
                    for (words, order, undo) in &results {
                        footer.result(*order, words.as_str(), *undo, move |ui| {
                            ui.label(words.as_str());
                        });
                    }
                    footer.pane(ui, crate::design::Body::Content, |_| ());
                });
            harness.run_steps(3);
            let mut rows: Vec<(f32, String)> = harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some((text.pos.y, text.galley.job.text.clone())),
                    _ => None,
                })
                .collect();
            rows.sort_by(|a, b| a.0.total_cmp(&b.0));
            rows.into_iter().map(|(_, words)| words).collect()
        };
        let words = |list: &[&str]| list.iter().map(|w| (*w).to_owned()).collect::<Vec<_>>();
        // A, then B: B is newer, whatever order the caller adds them in.
        let (a, b) = (ResultOrder::issue(), ResultOrder::issue());
        let newest_b = words(&["B.", "A.", "Reason.", "Notice."]);
        assert_eq!(
            painted(&[("A.", a, true), ("B.", b, true)]),
            newest_b,
            "A then B"
        );
        assert_eq!(
            painted(&[("B.", b, true), ("A.", a, true)]),
            newest_b,
            "A then B, added B first"
        );
        // B, then A.
        let (b, a) = (ResultOrder::issue(), ResultOrder::issue());
        assert_eq!(
            painted(&[("B.", b, true), ("A.", a, true)]),
            words(&["A.", "B.", "Reason.", "Notice."]),
            "B then A"
        );
        // A, B, C; B is removed: C and A keep their orders and places.
        let (a, b, c) = (
            ResultOrder::issue(),
            ResultOrder::issue(),
            ResultOrder::issue(),
        );
        assert_eq!(
            painted(&[("A.", a, true), ("B.", b, true), ("C.", c, true)]),
            words(&["C.", "B.", "A.", "Reason.", "Notice."]),
            "A, B, C"
        );
        assert_eq!(
            painted(&[("C.", c, true), ("A.", a, true)]),
            words(&["C.", "A.", "Reason.", "Notice."]),
            "B removed"
        );
        // A, B, Undo A (A goes), then A again: a new action with a new order, so it leads B. The
        // same A's words changing issue nothing: it stays after B.
        let (a, b) = (ResultOrder::issue(), ResultOrder::issue());
        assert_eq!(
            painted(&[("A, changed.", a, true), ("B.", b, true)]),
            words(&["B.", "A, changed.", "Reason.", "Notice."]),
            "changed words keep A's order"
        );
        assert_eq!(
            painted(&[("B.", b, true)]),
            words(&["B.", "Reason.", "Notice."]),
            "Undo A"
        );
        let again = ResultOrder::issue();
        assert!(again > b && b > a, "an action again issues a newer order");
        assert_eq!(
            painted(&[("B.", b, true), ("A again.", again, true)]),
            words(&["A again.", "B.", "Reason.", "Notice."]),
            "A again leads B"
        );
        // Results with no live Undo follow the notices, newest first.
        let (old, new) = (ResultOrder::issue(), ResultOrder::issue());
        assert_eq!(
            painted(&[("Old.", old, false), ("New.", new, false), ("B.", b, true)]),
            words(&["B.", "Reason.", "Notice.", "New.", "Old."]),
            "plain results"
        );
    }

    /// iter-27-A footer contract: a 4-line editor with a reason has the footer `S` below its save
    /// row, not at the pane's bottom, from the first frame the reason shows; 60 typed lines pin
    /// it, and it then draws a hairline at its top, which it does not draw while it follows. The
    /// editor's top is the same in every frame, at 960, 1200 and 1920. Presses: the footer's
    /// Undo takes a press where it follows, and where it is pinned; a press on the hairline goes
    /// to the footer, never to the cut line under it; once the footer empties, a press where it
    /// was goes to the body line now shown there. Review of PR 298, item 3: the last body action
    /// above the footer takes its press while the footer follows, while it is pinned, and while
    /// its content grows; the pane's own press area, which lies under a following footer, never
    /// takes the footer's press; a click on an outside field moves focus there and typing goes
    /// there; Tab reaches the last Undo of an overflowing footer, in view. Round 28, package P:
    /// all of this for the footer placed by hand and for `Footer::pane`, after the pane's
    /// placement for every body shape (`iter28_p_the_pane_places_its_footer_for_every_body`).
    /// A caller of `pane` does not repeat this proof.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: calls `ResultOrder::issue`, `Footer::result` or `save_row_state`, and raw egui calls (design::results::STAGED_ALLOWS)"
    )]
    fn iter27_a_the_footer_follows_a_short_body() {
        use crate::design::test_harness::ThemedPage as _;
        use egui_kittest::kittest::{NodeT, Queryable};
        use std::{
            cell::{Cell, RefCell},
            rc::Rc,
        };
        const SAVE_ROW: &str = "Save repaired note";
        const REASON: &str = "Line 3 opens a list with [ that never closes.";
        let click = |harness: &mut egui_kittest::Harness<'_>, pos: egui::Pos2| {
            harness.event(egui::Event::PointerMoved(pos));
            for pressed in [true, false] {
                harness.event(egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                });
                harness.run_steps(1);
            }
            harness.run_steps(2);
        };
        iter28_p_the_pane_places_its_footer_for_every_body();
        // The press proof, for the footer placed by hand (staged out) and by `Footer::pane`.
        for (by_hand, size) in [true, false].into_iter().flat_map(|by_hand| {
            [
                vec2(960.0, 640.0),
                vec2(1200.0, 800.0),
                vec2(1920.0, 1080.0),
            ]
            .map(|size| (by_hand, size))
        }) {
            let mode = if by_hand { "By hand" } else { "Pane" };
            // How many lines the editor holds, and whether the reason shows.
            let lines = Rc::new(Cell::new(4_usize));
            let reason = Rc::new(Cell::new(false));
            // Presses: the footer's Undo, and the last body line pressed.
            let (undone, line_pressed) = (Rc::new(Cell::new(0)), Rc::new(Cell::new(None)));
            // More results that come into the footer (each with its Undo), presses on the pane's
            // own press area, and the outside field's words.
            let (extra, background) = (Rc::new(Cell::new(0_usize)), Rc::new(Cell::new(0)));
            let outside = Rc::new(RefCell::new(String::new()));
            let (lines_in, reason_in) = (lines.clone(), reason.clone());
            let (undone_in, line_in) = (undone.clone(), line_pressed.clone());
            let (extra_in, background_in, outside_in) =
                (extra.clone(), background.clone(), outside.clone());
            // The extra results' actions came first, oldest to newest, then the discard.
            let extra_orders: Vec<ResultOrder> = (0..30).map(|_| ResultOrder::issue()).collect();
            let order = ResultOrder::issue();
            let mut harness =
                egui_kittest::Harness::builder()
                    .with_size(size)
                    .themed_ui(move |ui| {
                        // The outside field in a column of its own, beside the pane.
                        let (left, right) = ui.max_rect().split_left_right_at_fraction(0.7);
                        let mut column = ui.new_child(egui::UiBuilder::new().max_rect(right));
                        column.add(
                            egui::TextEdit::singleline(&mut *outside_in.borrow_mut())
                                .id(Id::new("outside_field")),
                        );
                        let mut pane = ui.new_child(
                            egui::UiBuilder::new()
                                .max_rect(left)
                                .layout(Layout::top_down(Align::Min)),
                        );
                        let ui = &mut pane;
                        let whole = ui.max_rect();
                        ui.label("Title");
                        let mut footer = Footer::new("follow_footer", ui.available_width());
                        if reason_in.get() {
                            let undone = &undone_in;
                            footer.result(order, "Draft discarded.", true, move |ui| {
                                if ui.button("Undo it").clicked() {
                                    undone.set(undone.get() + 1);
                                }
                            });
                            for (at, order) in extra_orders.iter().enumerate().take(extra_in.get())
                            {
                                footer.result(*order, format!("Extra {at}."), true, move |ui| {
                                    let _ = ui.button(format!("Undo extra {at}"));
                                });
                            }
                            footer.legacy_reason(REASON, |ui| {
                                message(ui, Tone::Error, REASON);
                            });
                        }
                        // The pane's own press area, as a pane that ends a selection on a press
                        // has: it holds the whole pane, so it lies under a following footer.
                        let background = |ui: &mut Ui| {
                            if ui
                                .interact(whole, Id::new("pane_background"), Sense::click())
                                .clicked()
                            {
                                background_in.set(background_in.get() + 1);
                            }
                        };
                        let lines = |ui: &mut Ui| {
                            for line in 0..lines_in.get() {
                                if ui.button(format!("Editor line {line}")).clicked() {
                                    line_in.set(Some(line));
                                }
                            }
                        };
                        if by_hand {
                            let bottom = footer.show_by_hand(ui);
                            background(ui);
                            // The editor leaves room for its save row above `bottom`, as World's
                            // does.
                            let row = Type::Body.line() + ui.spacing().item_spacing.y;
                            super::super::layout::ScrollColumn::new("follow_body")
                                .fit_height()
                                .max_height((bottom - ui.cursor().top() - row).max(0.0))
                                .show(ui, lines);
                            ui.label(SAVE_ROW);
                        } else {
                            // The pane is the scroll column: the lines, then the save row as its
                            // last line.
                            footer.pane(ui, super::super::pane::Body::Content, |ui| {
                                background(ui);
                                lines(ui);
                                ui.label(SAVE_ROW);
                            });
                        }
                    });
            harness.run_steps(3);
            let text = |harness: &egui_kittest::Harness<'_>, words: &str| {
                harness
                    .output()
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) if text.galley.job.text == words => Some(
                            Rect::from_min_size(text.pos, text.galley.size())
                                .intersect(shape.clip_rect),
                        ),
                        _ => None,
                    })
            };
            let footer = |harness: &egui_kittest::Harness<'_>| {
                if by_hand {
                    harness
                        .ctx
                        .read_response(Id::new("follow_footer").with("focus"))
                        .map(|response| response.rect)
                } else {
                    super::super::pane::footer_rect(&harness.ctx, Id::new("follow_footer"))
                }
                .filter(|_| reason.get())
            };
            let hairline = |harness: &egui_kittest::Harness<'_>, at: Rect| {
                harness.output().shapes.iter().any(|shape| {
                    matches!(&shape.shape, egui::Shape::LineSegment { points, stroke }
                        if stroke.color == colors().hairline
                            && (points[0].y - at.top()).abs() <= 1.0
                            && (points[0].x - at.left()).abs() <= 1.0)
                })
            };
            // The last body action whole in the editor's view (which ends at the save row, `S` or
            // more above the footer), by its laid-out place, clipped or not: its index and centre.
            // Places by the accessibility tree, as laid out whether painted or clipped: a fault
            // that hides a control fails at its press, not at a missing paint.
            let place = |harness: &egui_kittest::Harness<'_>, name: &str| {
                harness.query_by_label(name).map(|node| node.rect())
            };
            // The last body action whole in the editor's view (which ends at the save row, `S` or
            // more above the footer): its index and its centre.
            // Where the editor's view ends: by hand, at its save row; in the pane, `S` above the
            // footer (the save row is the column's last line).
            let last_line = |harness: &egui_kittest::Harness<'_>, at: Rect| {
                let end = if by_hand {
                    let save = place(harness, SAVE_ROW)?;
                    assert!(
                        save.bottom() <= at.top() - S + 0.5,
                        "the save row is above the footer"
                    );
                    save.top()
                } else {
                    at.top() - S + 0.5
                };
                (0..lines.get()).rev().find_map(|line| {
                    place(harness, &format!("Editor line {line}"))
                        .filter(|rect| rect.bottom() <= end)
                        .map(|rect| (line, rect.center()))
                })
            };
            let focused = |harness: &egui_kittest::Harness<'_>| {
                harness
                    .ctx
                    .memory(|memory| memory.focused())
                    .map(|id| egui::accesskit::NodeId(id.value()))
            };
            let top = text(&harness, "Editor line 0")
                .expect("the editor shows")
                .top();
            // The first frame with the reason: no pass after it to settle.
            reason.set(true);
            harness.step();
            let (save, at) = (place(&harness, SAVE_ROW).unwrap(), footer(&harness));
            assert!(
                at.is_some_and(|at| (at.top() - (save.bottom() + S)).abs() <= 1.0
                    && at.bottom() < size.y - L
                    && !hairline(&harness, at)),
                "{mode} at {size:?} the first frame's footer {at:?} is `S` below the save row {save:?}, with no hairline"
            );
            harness.run_steps(3);
            let (save, at) = (place(&harness, SAVE_ROW).unwrap(), footer(&harness));
            assert!(
                at.is_some_and(|at| (at.top() - (save.bottom() + S)).abs() <= 1.0),
                "{mode} at {size:?} the footer {at:?} stays `S` below the save row {save:?}"
            );
            // The Undo where the footer follows the body takes the press.
            let undo = text(&harness, "Undo it").expect("the Undo shows").center();
            click(&mut harness, undo);
            assert_eq!(
                (undone.get(), line_pressed.get(), background.get()),
                (1, None, 0),
                "{mode} at {size:?} the following footer's Undo takes the press, not the pane's press area under it"
            );
            // The blank band `S` above the following footer, under which the pane's press area
            // lies: nothing in the body takes that press. Only the pane holds this; the footer
            // placed by hand gives it to the pane's press area (round 28, package P).
            let at = footer(&harness).expect("the footer shows");
            click(&mut harness, pos2(at.right() - 4.0, at.top() - S / 2.0));
            assert!(
                by_hand || (line_pressed.get(), background.get()) == (None, 0),
                "{mode} at {size:?} a press just above the following footer {at:?} reaches no body target: {:?}",
                (line_pressed.get(), background.get())
            );
            background.set(0);
            line_pressed.set(None);
            // The last body action takes its press while the footer follows, and again once three
            // more results have grown the footer.
            for grown in [0, 3] {
                extra.set(grown);
                harness.run_steps(3);
                let at = footer(&harness).expect("the footer shows");
                let (line, centre) = last_line(&harness, at).expect("a body line shows");
                line_pressed.set(None);
                click(&mut harness, centre);
                assert_eq!(
                    (line_pressed.get(), background.get()),
                    (Some(line), 0),
                    "{mode} at {size:?} with {grown} more results, the last body action (line {line}) above the following footer {at:?} takes its press"
                );
            }
            extra.set(0);
            harness.run_steps(3);
            line_pressed.set(None);
            // Typed lines, one at a time, past the pane (40 of these label rows still fit at
            // 1920): the editor's top never moves, and at the end the footer is pinned at the
            // pane's bottom with a hairline at its top.
            for count in 5..=60 {
                lines.set(count);
                harness.step();
                assert_eq!(
                    text(&harness, "Editor line 0").map(|line| line.top()),
                    Some(top),
                    "{mode} at {size:?} with {count} lines the editor's top stays"
                );
            }
            harness.run_steps(3);
            let at = footer(&harness).expect("the footer shows");
            let save = place(&harness, SAVE_ROW).unwrap();
            // Pinned with its hairline. By hand the save row stays above it; in the pane the save
            // row is the column's last line, scrolled below the footer.
            assert!(
                at.bottom() >= size.y - L
                    && (!by_hand || save.bottom() <= at.top() - S + 0.5)
                    && hairline(&harness, at),
                "{mode} at {size:?} with 60 lines the footer {at:?} is pinned with its hairline (save row {save:?})"
            );
            assert_eq!(
                text(&harness, "Editor line 0").map(|line| line.top()),
                Some(top),
                "{mode} at {size:?} the editor's top stays with 60 lines"
            );
            // Astra re-check 2: in the pane, Tab into a body row hidden under the pinned footer
            // brings it whole into view above the footer, and Enter presses that row.
            if !by_hand {
                use egui_kittest::kittest::{NodeT, Queryable};
                let at = footer(&harness).expect("the footer shows");
                let view_end = at.top() - S + 0.5;
                let line_of = |harness: &egui_kittest::Harness<'_>| {
                    let focus = focused(harness)?;
                    (0..60).find(|line| {
                        harness
                            .query_by_label(&format!("Editor line {line}"))
                            .is_some_and(|node| node.accesskit_node().id() == focus)
                    })
                };
                let mut revealed = None;
                for _ in 0..90 {
                    let hidden: Vec<usize> = (0..60)
                        .filter(|line| {
                            place(&harness, &format!("Editor line {line}"))
                                .is_some_and(|rect| rect.bottom() > view_end)
                        })
                        .collect();
                    harness.key_press(egui::Key::Tab);
                    harness.run_steps(2);
                    if let Some(line) = line_of(&harness)
                        && hidden.contains(&line)
                    {
                        let rect = place(&harness, &format!("Editor line {line}")).unwrap();
                        assert!(
                            rect.bottom() <= view_end && rect.top() >= top - 0.5,
                            "{mode} at {size:?} Tab brings hidden line {line} {rect:?} whole into view above the footer {at:?}"
                        );
                        revealed = Some(line);
                        break;
                    }
                }
                let line = revealed.expect("Tab reaches a body row hidden under the footer");
                line_pressed.set(None);
                harness.key_press(egui::Key::Enter);
                harness.run_steps(3);
                assert_eq!(
                    (line_pressed.get(), background.get()),
                    (Some(line), 0),
                    "{mode} at {size:?} Enter presses the revealed line {line}"
                );
                line_pressed.set(None);
            }
            // On the hairline, over the cut line: the footer has the press, never the cut line
            // under it nor the pane's press area.
            click(&mut harness, pos2(at.left() + 40.0, at.top() + 0.5));
            assert_eq!(
                (line_pressed.get(), background.get()),
                (None, 0),
                "{mode} at {size:?} a press on the hairline reaches the footer, not a cut body line"
            );
            let before = undone.get();
            let undo = text(&harness, "Undo it").expect("the Undo shows").center();
            click(&mut harness, undo);
            assert_eq!(
                (undone.get(), background.get()),
                (before + 1, 0),
                "{mode} at {size:?} the pinned footer's Undo takes the press"
            );
            // The last body action whole above the pinned footer takes its press, also after the
            // footer grows by six results. Pinned, the footer is a third of the pane at most
            // (ruling Q-c).
            for grown in [0, 6] {
                extra.set(grown);
                harness.run_steps(3);
                let at = footer(&harness).expect("the footer shows");
                assert!(
                    at.height() <= size.y / 3.0 + 1.0 && at.bottom() >= size.y - L,
                    "{mode} at {size:?} with {grown} more results, the pinned footer {at:?} is a third of the pane at most"
                );
                let (line, centre) = last_line(&harness, at).expect("a body line shows");
                line_pressed.set(None);
                click(&mut harness, centre);
                assert_eq!(
                    (line_pressed.get(), background.get()),
                    (Some(line), 0),
                    "{mode} at {size:?} with {grown} more results, the last body action (line {line}) above the pinned footer {at:?} takes its press"
                );
            }
            // Still pinned and grown: the wheel scrolls the editor to its end, and its very last
            // line (59) comes whole into view above the footer and takes its press.
            let at = footer(&harness).expect("the footer shows");
            let over = pos2(at.left() + 40.0, top + 40.0);
            harness.event(egui::Event::PointerMoved(over));
            for _ in 0..80 {
                if last_line(&harness, at).is_some_and(|(line, _)| line == 59) {
                    break;
                }
                harness.event(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: vec2(0.0, -3.0 * Type::Body.line()),
                    modifiers: egui::Modifiers::NONE,
                });
                harness.run_steps(2);
            }
            let end = last_line(&harness, at);
            assert!(
                end.is_some_and(|(line, _)| line == 59),
                "{mode} at {size:?} the wheel brings line 59 whole into view above the pinned footer {at:?}: {end:?}"
            );
            let (_, centre) = end.unwrap();
            line_pressed.set(None);
            click(&mut harness, centre);
            assert_eq!(
                (line_pressed.get(), background.get()),
                (Some(59), 0),
                "{mode} at {size:?} the last body line, scrolled to above the pinned and grown footer {at:?}, takes its press"
            );
            extra.set(0);
            line_pressed.set(None);
            // Back to 4 lines: the footer follows the body up again, without its hairline. The
            // pointer leaves first: egui keeps a scroll area's height while the pointer is on it.
            harness.event(egui::Event::PointerGone);
            lines.set(4);
            harness.run_steps(3);
            let (save, at) = (place(&harness, SAVE_ROW).unwrap(), footer(&harness));
            assert!(
                at.is_some_and(
                    |at| (at.top() - (save.bottom() + S)).abs() <= 1.0 && !hairline(&harness, at)
                ),
                "{mode} at {size:?} back at 4 lines the footer {at:?} follows the save row {save:?}"
            );
            // Focus in the footer, then a click on the outside field: focus goes there, and the
            // typed words too.
            let undo_id = harness
                .query_by_label("Undo it")
                .map(|undo| undo.accesskit_node().id());
            for _ in 0..12 {
                if undo_id.is_some() && focused(&harness) == undo_id {
                    break;
                }
                harness.key_press(egui::Key::Tab);
                harness.run_steps(2);
            }
            assert!(
                undo_id.is_some() && focused(&harness) == undo_id,
                "{mode} at {size:?} Tab reaches the footer's Undo"
            );
            let field = harness
                .ctx
                .read_response(Id::new("outside_field"))
                .expect("the outside field shows")
                .rect;
            click(&mut harness, field.center());
            harness.event(egui::Event::Text("abc".into()));
            harness.run_steps(2);
            assert!(
                harness
                    .ctx
                    .memory(|memory| memory.has_focus(Id::new("outside_field")))
                    && *outside.borrow() == "abc",
                "{mode} at {size:?} a click on the outside field takes the focus from the footer, and the typed words go there: {:?}",
                outside.borrow()
            );
            // Twenty more results: in the pane, below the short body, the footer is not held to a
            // third of the pane (ruling Q-c). It starts `S` below the save row and takes the room
            // down to the pane's end, and scrolls inside only beyond that: Tab from the field
            // reaches the last Undo, which scrolls into the footer's view.
            extra.set(20);
            harness.run_steps(3);
            let last_undo = harness
                .query_by_label("Undo extra 0")
                .map(|undo| undo.accesskit_node().id());
            for _ in 0..48 {
                if last_undo.is_some() && focused(&harness) == last_undo {
                    break;
                }
                harness.key_press(egui::Key::Tab);
                harness.run_steps(2);
            }
            let at = footer(&harness).expect("the footer shows");
            let save = place(&harness, SAVE_ROW).unwrap();
            let shown = text(&harness, "Undo extra 0");
            assert!(
                last_undo.is_some()
                    && focused(&harness) == last_undo
                    && shown.is_some_and(|undo| undo.is_positive() && at.contains_rect(undo)),
                "{mode} at {size:?} Tab reaches the last Undo of the footer {at:?}, in view: {shown:?}"
            );
            // Placed by hand (staged out), the footer keeps the old cap: a third of the pane.
            let tall = if by_hand {
                at.height() <= size.y / 3.0 + 1.0
            } else {
                at.height() > size.y / 3.0 + 1.0
            };
            assert!(
                (at.top() - (save.bottom() + S)).abs() <= 1.0
                    && tall
                    && at.bottom() <= size.y + 0.5,
                "{mode} at {size:?} the footer {at:?} of 21 results follows the save row {save:?} by `S`, and ends in the pane; in the pane it is taller than a third of it, by hand a third at most"
            );
            extra.set(0);
            harness.run_steps(3);
            // 60 lines again, then the footer empties: a press where it was reaches the body
            // line shown there now.
            lines.set(60);
            harness.run_steps(3);
            let at = footer(&harness).expect("the footer shows");
            reason.set(false);
            harness.run_steps(3);
            assert!(
                footer(&harness).is_none(),
                "{mode} at {size:?} the footer empties"
            );
            // The body line now shown whole inside the footer's old place.
            let there = harness
                .output()
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => {
                        let line = text
                            .galley
                            .job
                            .text
                            .strip_prefix("Editor line ")?
                            .parse::<usize>()
                            .ok()?;
                        let rect = Rect::from_min_size(text.pos, text.galley.size());
                        (at.contains_rect(rect) && shape.clip_rect.contains_rect(rect))
                            .then_some((line, rect.center()))
                    }
                    _ => None,
                });
            let (line, centre) = there.expect("a body line shows where the footer was");
            line_pressed.set(None);
            click(&mut harness, centre);
            assert_eq!(
                (line_pressed.get(), background.get()),
                (Some(line), 0),
                "{mode} at {size:?} the body line {line} now where the footer was {at:?} takes the press"
            );
        }
    }

    /// Round 28, package P (repair of PR 305's review): `Footer::pane` places its footer for each
    /// body shape, checked against rects the test reads itself (the accessibility tree, painted
    /// words and their clip), never against the part's own placement. `Body::Content` is the
    /// pane's scroll column: rows, rows in a scope, rows in a `Frame` with a margin, and two
    /// `ui.columns` with a control as tall as the room. `Body::Fills`: a side `Column` with two
    /// labels and an action (Astra 1a), a nested `Content` pane, and an editor as tall as the
    /// room. With 2 rows a `Content` footer's top is `S` (±1) below the last body rect, with no
    /// hairline, and G6 finds nothing; a body that takes all the room, or 200 rows, pins the
    /// footer below the room: every painted body word ends `S` above it, the 200 rows draw the
    /// hairline, and the body control next to the footer takes its press. The pane's record is
    /// of the last completed pass only: present, absent (no record), present. A vertical
    /// `ScrollColumn` inside a `Content` pane is refused.
    #[allow(
        clippy::disallowed_methods,
        reason = "staged: raw egui calls that a design part replaces (design::results::STAGED_ALLOWS)"
    )]
    fn iter28_p_the_pane_places_its_footer_for_every_body() {
        use super::super::{
            layout::ScrollColumn,
            pane::{
                Body, Fills, FillsReason, Placement, footer_rect, pane_record, pane_records,
                short_body_gap,
            },
        };
        use crate::design::test_harness::ThemedPage as _;
        use std::{cell::Cell, rc::Rc};
        #[derive(Clone, Copy, Debug, PartialEq)]
        enum Shape {
            Rows,
            InScope,
            InFrame,
            TallColumn,
            SideColumn,
            Nested,
            Editor,
            /// Two unequal columns of one task (`PaneColumns::shared_task`, layout step 7): a
            /// resizable column of `count` rows that scrolls on its own, and a column of four
            /// rows and the last action.
            Composite,
            /// `Composite` with a fit `BlockRegion` of `count` row blocks as its left column
            /// (note C0-parts Part A): as tall as its rows up to the room, so a short region
            /// leaves the composite to its taller column.
            RegionComposite,
        }
        const REASON: &str = "Line 1 opens a list.";
        let rows = |ui: &mut Ui, count: usize| {
            for line in 0..count {
                ui.label(format!("Line {line}"));
            }
        };
        let click = |harness: &mut egui_kittest::Harness<'_>, pos: egui::Pos2| {
            harness.event(egui::Event::PointerMoved(pos));
            for pressed in [true, false] {
                harness.event(egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                });
                harness.run_steps(1);
            }
            harness.run_steps(2);
        };
        // Every accessible rect on screen, as the gate reads them: the whole tree.
        let access = |harness: &egui_kittest::Harness<'_>| -> Vec<(String, Rect)> {
            use egui_kittest::kittest::{NodeT, Queryable};
            harness
                .query_all(egui_kittest::kittest::By::new().predicate(|_| true))
                .filter_map(|node| {
                    let b = node.accesskit_node().bounding_box()?;
                    Some((
                        node.accesskit_node()
                            .label()
                            .or_else(|| node.accesskit_node().value())
                            .unwrap_or_default(),
                        Rect::from_min_max(
                            pos2(b.x0 as f32, b.y0 as f32),
                            pos2(b.x1 as f32, b.y1 as f32),
                        ),
                    ))
                })
                .collect()
        };
        for size in [vec2(960.0, 640.0), vec2(1920.0, 1080.0)] {
            for shape in [
                Shape::Rows,
                Shape::InScope,
                Shape::InFrame,
                Shape::TallColumn,
                Shape::SideColumn,
                Shape::Nested,
                Shape::Editor,
                Shape::Composite,
                Shape::RegionComposite,
            ] {
                for count in [2, 200] {
                    let mut region = crate::design::RegionState::new("shape_region");
                    let row_blocks = |count: usize| {
                        crate::design::memo::Revised::new(vec![
                            crate::design::Block::Row(
                                crate::design::Row::OneLine
                            );
                            count
                        ])
                    };
                    let blocks = Rc::new(std::cell::RefCell::new(row_blocks(count)));
                    let blocks_in = blocks.clone();
                    let pressed = Rc::new(Cell::new(None::<&'static str>));
                    let pressed_in = pressed.clone();
                    let mut harness =
                        egui_kittest::Harness::builder()
                            .with_size(size)
                            .themed_ui(move |ui| {
                                ui.label("Title");
                                let mut footer = Footer::new("shape_footer", ui.available_width());
                                footer.legacy_reason(REASON, |ui| {
                                    message(ui, Tone::Error, REASON);
                                });
                                let body = match shape {
                                    Shape::SideColumn | Shape::Nested => {
                                        Body::Fills(Fills::for_test(FillsReason::Columns))
                                    }
                                    Shape::Editor => {
                                        Body::Fills(Fills::for_test(FillsReason::NoteEditor))
                                    }
                                    _ => Body::Content,
                                };
                                let press = |ui: &mut Ui, name: &'static str| {
                                    if ui.button(name).clicked() {
                                        pressed_in.set(Some(name));
                                    }
                                };
                                footer.pane(ui, body, |ui| match shape {
                                    Shape::Rows => {
                                        rows(ui, count);
                                        press(ui, "Last action");
                                    }
                                    Shape::InScope => {
                                        let rect = ui.available_rect_before_wrap();
                                        ui.scope_builder(
                                            egui::UiBuilder::new().max_rect(rect),
                                            |ui| {
                                                rows(ui, count);
                                                press(ui, "Last action");
                                            },
                                        );
                                    }
                                    Shape::InFrame => {
                                        egui::Frame::new().inner_margin(4).show(ui, |ui| {
                                            ui.push_id("inner", |ui| {
                                                rows(ui, count);
                                                press(ui, "Last action");
                                            });
                                        });
                                    }
                                    Shape::TallColumn => {
                                        ui.columns(2, |columns| {
                                            rows(&mut columns[0], count);
                                            let room = columns[1].available_size();
                                            if columns[1]
                                                .add_sized(room, egui::Button::new("Tall action"))
                                                .clicked()
                                            {
                                                pressed_in.set(Some("Tall action"));
                                            }
                                        });
                                    }
                                    Shape::SideColumn => {
                                        super::super::Column::left(
                                            "shape_left",
                                            200.0,
                                            150.0..=300.0,
                                        )
                                        .show_inside(
                                            ui,
                                            |ui| {
                                                ui.label("Left row 0");
                                                ui.label("Left row 1");
                                                press(ui, "Left action");
                                            },
                                        );
                                        // A column of its own may scroll inside a `Fills` body.
                                        egui::CentralPanel::default()
                                            .frame(egui::Frame::new())
                                            .show_inside(ui, |ui| {
                                                ScrollColumn::new("shape_rows")
                                                    .show(ui, |ui| rows(ui, count));
                                            });
                                    }
                                    Shape::Nested => {
                                        let mut inner =
                                            Footer::new("inner_footer", ui.available_width());
                                        inner.legacy_reason("Inner reason.", |ui| {
                                            message(ui, Tone::Warning, "Inner reason.");
                                        });
                                        inner.pane(ui, Body::Content, |ui| {
                                            rows(ui, count);
                                            press(ui, "Last action");
                                        });
                                    }
                                    Shape::Composite => {
                                        super::super::pane::PaneColumns::shared_task(
                                            "shape_columns",
                                        )
                                        .resizable(
                                            0.0,
                                            super::super::Column::left(
                                                "shape_left",
                                                200.0,
                                                150.0..=300.0,
                                            ),
                                        )
                                        .rest(S)
                                        .edges(ui)
                                        .show_children(
                                            ui,
                                            |ui, index| {
                                                if index == 0 {
                                                    ScrollColumn::new("shape_left_rows")
                                                        .fit_height()
                                                        .show(ui, |ui| rows(ui, count));
                                                } else {
                                                    for line in 0..4 {
                                                        ui.label(format!("Line R{line}"));
                                                    }
                                                    press(ui, "Last action");
                                                }
                                            },
                                        );
                                    }
                                    Shape::RegionComposite => {
                                        super::super::pane::PaneColumns::shared_task(
                                            "shape_columns",
                                        )
                                        .resizable(
                                            0.0,
                                            super::super::Column::left(
                                                "shape_left",
                                                200.0,
                                                150.0..=300.0,
                                            ),
                                        )
                                        .rest(S)
                                        .edges(ui)
                                        .show_children(
                                            ui,
                                            |ui, index| {
                                                if index == 0 {
                                                    let blocks = blocks_in.borrow();
                                                    crate::design::BlockRegion::new(
                                                        "shape_region",
                                                        &mut region,
                                                        &blocks,
                                                    )
                                                    .fit_height()
                                                    .show(ui, |ui, at| {
                                                        ui.label(format!("Line {at}"));
                                                    });
                                                } else {
                                                    for line in 0..4 {
                                                        ui.label(format!("Line R{line}"));
                                                    }
                                                    press(ui, "Last action");
                                                }
                                            },
                                        );
                                    }
                                    Shape::Editor => {
                                        let mut text = (0..count)
                                            .map(|line| format!("Line {line}"))
                                            .collect::<Vec<_>>()
                                            .join("\n");
                                        ui.add_sized(
                                            ui.available_size(),
                                            egui::TextEdit::multiline(&mut text)
                                                .id(Id::new("editor")),
                                        );
                                    }
                                });
                            });
                    harness.run_steps(4);
                    let case = format!("{shape:?} with {count} lines at {size:?}");
                    let footer = footer_rect(&harness.ctx, Id::new("shape_footer"))
                        .unwrap_or_else(|| panic!("{case}: the footer shows"));
                    let hairline = harness.output().shapes.iter().any(|shape| {
                        matches!(&shape.shape, egui::Shape::LineSegment { points, stroke }
                            if stroke.color == colors().hairline
                                && (points[0].y - footer.top()).abs() <= 1.0
                                && (points[0].x - footer.left()).abs() <= 1.0)
                    });
                    let record = pane_record(&harness.ctx, Id::new("shape_footer"))
                        .unwrap_or_else(|| panic!("{case}: the pane keeps its record"));
                    let room = record.room();
                    let nodes = access(&harness);
                    // The body's accessible rects: below the title, above the footer, not the
                    // footer's own and not the nested pane's footer.
                    let inner_footer = footer_rect(&harness.ctx, Id::new("inner_footer"));
                    let body: Vec<(String, Rect)> = nodes
                        .iter()
                        .filter(|(_, rect)| {
                            rect.is_positive()
                                && rect.top() >= room.top() - 0.5
                                && rect.bottom() <= footer.top() + 0.5
                                && rect.bottom() < room.bottom()
                        })
                        .cloned()
                        .collect();
                    let end = body
                        .iter()
                        .map(|(_, rect)| rect.bottom())
                        .chain(inner_footer.map(|rect| rect.bottom()))
                        .fold(room.top(), f32::max);
                    // Every painted body word, as clipped, ends `S` above the footer: it never
                    // lies under it. Body words are known by their text (the rows, the actions
                    // and labels this test draws), not by where they are.
                    let body_word = |words: &str| {
                        (words.starts_with("Line ") && words != REASON)
                            || [
                                "Last action",
                                "Left row 0",
                                "Left row 1",
                                "Left action",
                                "Tall action",
                            ]
                            .contains(&words)
                    };
                    let mut words_seen = 0;
                    for shape in &harness.output().shapes {
                        if let egui::Shape::Text(text) = &shape.shape
                            && body_word(&text.galley.job.text)
                        {
                            let rect = Rect::from_min_size(text.pos, text.galley.size())
                                .intersect(shape.clip_rect);
                            if rect.is_positive() {
                                words_seen += 1;
                                assert!(
                                    rect.bottom() <= footer.top() - S + 0.5,
                                    "{case}: {:?} at {rect:?} ends `S` above the footer {footer:?}",
                                    text.galley.job.text
                                );
                            }
                        }
                    }
                    assert!(words_seen > 0, "{case}: body words are painted");
                    // The composite follows its taller column (the four rows and the last
                    // action) while its 2 rows are shorter.
                    let follows = count == 2
                        && matches!(
                            shape,
                            Shape::Rows
                                | Shape::InScope
                                | Shape::InFrame
                                | Shape::Composite
                                | Shape::RegionComposite
                        );
                    if follows {
                        // The frame's 4 px margin is under its last rect.
                        let margin = if shape == Shape::InFrame { 4.0 } else { 0.0 };
                        assert!(
                            (footer.top() - (end + margin + S)).abs() <= 1.0
                                && !hairline
                                && short_body_gap(
                                    &record,
                                    1.0,
                                    footer,
                                    body.iter().map(|(_, rect)| *rect)
                                )
                                .is_none(),
                            "{case}: the footer {footer:?} is `S` below the last body rect {end} (and the frame's margin {margin}), with no hairline ({hairline})"
                        );
                    } else {
                        let cut = count == 200
                            && matches!(
                                shape,
                                Shape::Rows | Shape::InScope | Shape::InFrame | Shape::TallColumn
                            );
                        assert!(
                            (footer.bottom() - room.bottom()).abs() <= 1.0 && hairline == cut,
                            "{case}: the footer {footer:?} is pinned at the pane's bottom {}, with its hairline only over cut rows ({hairline}, expected {cut})",
                            room.bottom()
                        );
                    }
                    // Layout step 7: the composite is as tall as its taller column, measured by
                    // its content, not by the room its columns were given. With 2 rows the
                    // footer starts `S` below the last action of the taller column, and the
                    // splitter's rule ends at that action; with 200 the left column scrolls in
                    // the whole room and the rule ends above the footer.
                    if matches!(shape, Shape::Composite | Shape::RegionComposite) {
                        let last = nodes
                            .iter()
                            .find(|(label, _)| label == "Last action")
                            .map(|(_, rect)| *rect)
                            .unwrap_or_else(|| panic!("{case}: Last action shows"));
                        let rule = harness
                            .output()
                            .shapes
                            .iter()
                            .filter_map(|shape| match &shape.shape {
                                egui::Shape::LineSegment { points, .. }
                                    if (points[0].x - points[1].x).abs() < 0.5
                                        && points[0].x > room.left() + 100.0
                                        && points[0].x < room.left() + 400.0 =>
                                {
                                    Some(points[0].y.max(points[1].y))
                                }
                                _ => None,
                            })
                            .reduce(f32::max);
                        let ends = if count == 2 {
                            (footer.top() - (last.bottom() + S)).abs() <= 1.0
                                && rule.is_some_and(|rule| (rule - last.bottom()).abs() <= 1.0)
                        } else {
                            rule.is_some_and(|rule| {
                                rule <= footer.top() - S + 1.0 && rule > last.bottom()
                            })
                        };
                        assert!(
                            ends,
                            "{case}: the footer {footer:?} follows the taller column's Last action {last:?} by `S` (2 rows), and the splitter's rule ends at it: {rule:?}"
                        );
                    }
                    // A fit region that grows under a composite measured short (a package with
                    // more entries chosen next), and a window that shrinks under it (note
                    // C0-parts Part A; the Campaign page showed 3 of 6 rows). The FIRST presented
                    // frame after each change is checked, before any settling (Sol 2-021), with
                    // one pass and with the normal budget: with one pass the composite's discard
                    // is denied and it may keep its old height for that frame, but the frame is
                    // safe (no body words under the footer or cut by their clip, the required
                    // controls whole and pressable); with the normal budget it is already final.
                    // The quiet frame after it is final in both, and the next one moves nothing.
                    if shape == Shape::RegionComposite && count == 2 {
                        let row = crate::design::Density::Scan.row();
                        // The footer as THIS frame painted it: `footer_rect` reads the pane's
                        // record of the last completed pass, one frame behind on a first frame.
                        // Its painted reason stands at a fixed place in it (taken once, settled).
                        let reason_at = |harness: &egui_kittest::Harness<'_>| {
                            harness
                                .output()
                                .shapes
                                .iter()
                                .find_map(|shape| match &shape.shape {
                                    egui::Shape::Text(text) if text.galley.job.text == REASON => {
                                        Some(text.pos.y)
                                    }
                                    _ => None,
                                })
                                .unwrap_or_else(|| panic!("{case}: the footer's reason is painted"))
                        };
                        let settled = footer_rect(&harness.ctx, Id::new("shape_footer"))
                            .unwrap_or_else(|| panic!("{case}: the footer shows"));
                        let inset = reason_at(&harness) - settled.top();
                        let footer_of = |harness: &egui_kittest::Harness<'_>| {
                            let top = reason_at(harness) - inset;
                            Rect::from_min_size(pos2(settled.left(), top), settled.size())
                        };
                        let node = |harness: &egui_kittest::Harness<'_>, name: &str| {
                            access(harness)
                                .into_iter()
                                .find(|(label, _)| label == name)
                                .map(|(_, rect)| rect)
                        };
                        // The safe frame: every painted body word lies whole in its clip and
                        // `S` above the footer; Line 0 and Last action show above it.
                        let safe = |harness: &egui_kittest::Harness<'_>, step: &str| {
                            let footer = footer_of(harness);
                            for shape in &harness.output().shapes {
                                if let egui::Shape::Text(text) = &shape.shape
                                    && body_word(&text.galley.job.text)
                                {
                                    let rect = Rect::from_min_size(text.pos, text.galley.size());
                                    let shown = rect.intersect(shape.clip_rect);
                                    if shown.is_positive() {
                                        assert!(
                                            shape.clip_rect.expand(0.5).contains_rect(rect)
                                                && rect.bottom() <= footer.top() - S + 0.5,
                                            "{case}, {step}, a presented frame: {:?} at {rect:?} is whole in its clip {:?} and `S` above the footer {footer:?}",
                                            text.galley.job.text,
                                            shape.clip_rect
                                        );
                                    }
                                }
                            }
                            for name in ["Line 0", "Last action"] {
                                let rect = node(harness, name);
                                assert!(
                                    rect.is_some_and(|rect| rect.bottom()
                                        <= footer.top() - S + 0.5),
                                    "{case}, {step}, a presented frame: {name} {rect:?} shows above the footer {footer:?}"
                                );
                            }
                        };
                        // The final geometry of each step.
                        let grown = |harness: &egui_kittest::Harness<'_>, step: &str| {
                            let footer = footer_of(harness);
                            let last = node(harness, "Line 11");
                            assert!(
                                last.is_some_and(|last| (footer.top() - (last.top() + row + S))
                                    .abs()
                                    <= 1.0),
                                "{case}, {step}: the last row's label {last:?} shows, and the footer {footer:?} follows its row ({row} px) by `S`"
                            );
                        };
                        let pinned = |harness: &egui_kittest::Harness<'_>, step: &str| {
                            let footer = footer_of(harness);
                            let room = pane_record(&harness.ctx, Id::new("shape_footer"))
                                .map(|record| record.room());
                            assert!(
                                room.is_some_and(
                                    |room| (footer.bottom() - room.bottom()).abs() <= 1.0
                                ) && node(harness, "Line 11").is_none(),
                                "{case}, {step}: the composite stops at the room {room:?}: the footer {footer:?} is pinned and the 12th row is out of view"
                            );
                        };
                        // With one pass a measured composite takes a frame more (its discard
                        // is denied): every frame stays safe until nothing moves, within three.
                        // With the normal budget nothing moves after the first frame.
                        let settle = |harness: &mut egui_kittest::Harness<'_>, step: &str| {
                            let mut frames = 1;
                            loop {
                                let seen = |harness: &egui_kittest::Harness<'_>| {
                                    (
                                        footer_of(harness),
                                        node(harness, "Line 0"),
                                        node(harness, "Line 11"),
                                    )
                                };
                                let before = seen(harness);
                                harness.step();
                                safe(harness, step);
                                if before == seen(harness) {
                                    break frames;
                                }
                                frames += 1;
                                assert!(
                                    frames <= 3,
                                    "{case}, {step}: the frame settles within 3 frames"
                                );
                            }
                        };
                        let quiet = |harness: &mut egui_kittest::Harness<'_>| {
                            let before = (footer_of(harness), node(harness, "Line 0"));
                            harness.step();
                            assert_eq!(
                                before,
                                (footer_of(harness), node(harness, "Line 0")),
                                "{case}: a quiet frame moves nothing"
                            );
                        };
                        let short = vec2(size.x, 360.0);
                        for passes in [1, 2] {
                            let budget = std::num::NonZeroUsize::new(passes).unwrap();
                            harness
                                .ctx
                                .options_mut(|options| options.max_passes = budget);
                            // Back to 2 rows at the full size, settled.
                            *blocks.borrow_mut() = row_blocks(2);
                            harness.set_size(size);
                            harness.run_steps(4);
                            // Growth: 12 rows, one presented frame.
                            *blocks.borrow_mut() = row_blocks(12);
                            harness.step();
                            let step = format!("grown to 12 rows with {passes} pass(es)");
                            safe(&harness, &step);
                            if passes > 1 {
                                grown(&harness, &step);
                            }
                            let frames = settle(&mut harness, &step);
                            assert!(
                                passes == 1 || frames == 1,
                                "{case}, {step}: with the normal budget the first frame is final ({frames} frames)"
                            );
                            grown(&harness, &format!("{step}, settled"));
                            quiet(&mut harness);
                            // Resize: a window too short for the 12 rows, one presented frame.
                            harness.set_size(short);
                            harness.step();
                            let step = format!("resized to {short:?} with {passes} pass(es)");
                            safe(&harness, &step);
                            if passes > 1 {
                                pinned(&harness, &step);
                            }
                            let frames = settle(&mut harness, &step);
                            assert!(
                                passes == 1 || frames == 1,
                                "{case}, {step}: with the normal budget the first frame is final ({frames} frames)"
                            );
                            pinned(&harness, &format!("{step}, settled"));
                            quiet(&mut harness);
                            // The control next to the footer takes its press there.
                            let last = node(&harness, "Last action")
                                .unwrap_or_else(|| panic!("{case}, {step}: Last action shows"));
                            pressed.set(None);
                            click(&mut harness, pos2(last.center().x, last.bottom() - 2.0));
                            assert_eq!(
                                pressed.get(),
                                Some("Last action"),
                                "{case}, {step}: Last action, next to the footer, takes its press"
                            );
                        }
                        harness.ctx.options_mut(|options| {
                            options.max_passes = std::num::NonZeroUsize::new(2).unwrap()
                        });
                        *blocks.borrow_mut() = row_blocks(2);
                        harness.set_size(size);
                        harness.run_steps(4);
                    }
                    // The body control next to the footer takes its press.
                    let target = match shape {
                        Shape::TallColumn => Some("Tall action"),
                        Shape::SideColumn => Some("Left action"),
                        Shape::Editor => None,
                        Shape::Composite | Shape::RegionComposite => Some("Last action"),
                        _ if count == 2 => Some("Last action"),
                        _ => None,
                    };
                    if let Some(name) = target {
                        let rect = nodes
                            .iter()
                            .find(|(label, _)| label == name)
                            .map(|(_, rect)| *rect)
                            .unwrap_or_else(|| panic!("{case}: {name} shows"));
                        assert!(
                            rect.bottom() <= footer.top() - S + 0.5,
                            "{case}: {name} {rect:?} ends `S` above the footer {footer:?}"
                        );
                        click(&mut harness, pos2(rect.center().x, rect.bottom() - 2.0));
                        assert_eq!(
                            pressed.get(),
                            Some(name),
                            "{case}: {name}, next to the footer, takes its press"
                        );
                    }
                }
            }
        }
        // A small room (Astra re-check 1): the viewport is the real room, even below egui's own
        // 64 px minimum, so the wheel brings the last row whole into view above the footer, and it
        // takes its press. One size gives a room under 64 px, so the case can fail.
        let mut under_64 = false;
        for size in [vec2(420.0, 130.0), vec2(420.0, 150.0), vec2(420.0, 190.0)] {
            let pressed = Rc::new(Cell::new(false));
            let pressed_in = pressed.clone();
            let mut harness =
                egui_kittest::Harness::builder()
                    .with_size(size)
                    .themed_ui(move |ui| {
                        ui.label("Title");
                        let mut footer = Footer::new("small_footer", ui.available_width());
                        footer.legacy_reason(REASON, |ui| {
                            message(ui, Tone::Error, REASON);
                        });
                        footer.pane(ui, Body::Content, |ui| {
                            rows(ui, 8);
                            if ui.button("Last action").clicked() {
                                pressed_in.set(true);
                            }
                        });
                    });
            harness.run_steps(4);
            let footer = footer_rect(&harness.ctx, Id::new("small_footer")).expect("the footer");
            let room = pane_record(&harness.ctx, Id::new("small_footer"))
                .unwrap()
                .room();
            let body = footer.top() - S - room.top();
            under_64 |= body < 64.0;
            let last = |harness: &egui_kittest::Harness<'_>| {
                access(harness)
                    .into_iter()
                    .find(|(label, _)| label == "Last action")
                    .map(|(_, rect)| rect)
            };
            harness.event(egui::Event::PointerMoved(pos2(
                room.left() + 20.0,
                room.top() + 4.0,
            )));
            for _ in 0..40 {
                if last(&harness).is_some_and(|rect| rect.bottom() <= footer.top() - S + 0.5) {
                    break;
                }
                harness.event(egui::Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: vec2(0.0, -Type::Body.line()),
                    modifiers: egui::Modifiers::NONE,
                });
                harness.run_steps(2);
            }
            let rect = last(&harness).expect("Last action shows");
            assert!(
                rect.top() >= room.top() - 0.5 && rect.bottom() <= footer.top() - S + 0.5,
                "At {size:?} (body {body} px) the wheel brings Last action {rect:?} whole above the footer {footer:?}"
            );
            click(&mut harness, rect.center());
            assert!(pressed.get(), "At {size:?} Last action takes its press");
        }
        assert!(under_64, "one small room is under egui's 64 px minimum");

        // Ruling Q-c (migration-32 L1a): the third-of-the-pane cap is for a pinned footer only.
        // The body is a block `body` px tall and a last line; the footer holds `reasons` one-line
        // reasons. With none there is no footer and the body's view is the whole room. Below the
        // short body the footer starts `S` after the last line: with more than a third of the
        // pane's messages and room left under them, it is as tall as they are, every one whole;
        // with more than the room holds, it takes the room down to the pane's end and scrolls
        // inside. At the threshold, a body 2 px shorter than its view (the room less `S` and the
        // pinned footer) is followed; one as tall as its view pins the footer (equality pins), a
        // third of the pane tall.
        for size in [vec2(960.0, 640.0), vec2(1920.0, 1080.0)] {
            let body = Rc::new(Cell::new(40.0_f32));
            let reasons = Rc::new(Cell::new(0_usize));
            let (body_in, reasons_in) = (body.clone(), reasons.clone());
            let mut harness =
                egui_kittest::Harness::builder()
                    .with_size(size)
                    .themed_ui(move |ui| {
                        ui.label("Title");
                        let mut footer = Footer::new("qc_footer", ui.available_width());
                        for at in 0..reasons_in.get() {
                            let words = format!("Reason {at} of this footer.");
                            footer.legacy_reason(words.clone(), move |ui| {
                                message(ui, Tone::Error, &words);
                            });
                        }
                        footer.pane(ui, Body::Content, |ui| {
                            super::super::extras::space(ui, body_in.get());
                            ui.label("Body end");
                        });
                    });
            let id = Id::new("qc_footer");
            let run = |harness: &mut egui_kittest::Harness<'_>| {
                harness.run_steps(4);
                let record = pane_record(&harness.ctx, id).expect("the pane keeps its record");
                let end = access(harness)
                    .into_iter()
                    .find(|(label, _)| label == "Body end")
                    .map(|(_, rect)| rect.bottom())
                    .expect("the body's last line");
                (record, footer_rect(&harness.ctx, id), end)
            };
            // A reason painted whole inside the footer's view.
            let whole = |harness: &egui_kittest::Harness<'_>, at: usize, footer: Rect| {
                let words = format!("Reason {at} of this footer.");
                harness
                    .output()
                    .shapes
                    .iter()
                    .any(|shape| match &shape.shape {
                        egui::Shape::Text(text) if text.galley.job.text == words => {
                            let rect = Rect::from_min_size(text.pos, text.galley.size());
                            shape
                                .clip_rect
                                .intersect(footer)
                                .expand(0.5)
                                .contains_rect(rect)
                        }
                        _ => false,
                    })
            };
            let (record, footer, _) = run(&mut harness);
            let room = record.room();
            let third = (room.height() / 3.0).floor();
            assert!(
                footer.is_none()
                    && record
                        .viewport()
                        .is_some_and(|view| (view.bottom() - room.bottom()).abs() <= 0.5),
                "Q-c at {size:?}: no message, no footer {footer:?}, and the body's view {:?} is the room {room:?}",
                record.viewport()
            );
            // The fewest reasons that make the footer taller than a third of the pane.
            let mut taller = None;
            for count in 1..=40 {
                reasons.set(count);
                let (record, footer, end) = run(&mut harness);
                let footer = footer.expect("the footer shows");
                assert!(
                    record.placement() == Placement::Follows
                        && (footer.top() - (end + S)).abs() <= 1.0,
                    "Q-c at {size:?} with {count} reasons: the footer {footer:?} follows the body's end {end} by `S`"
                );
                if footer.height() > third + 1.0 {
                    taller = Some((count, footer));
                    break;
                }
            }
            let (count, footer) = taller.unwrap_or_else(|| {
                panic!("Q-c at {size:?}: a following footer grows taller than a third ({third}) of the pane")
            });
            assert!(
                footer.bottom() < room.bottom() - L
                    && (0..count).all(|at| whole(&harness, at, footer)),
                "Q-c at {size:?}: the footer {footer:?} of {count} reasons, taller than a third of the pane, shows every reason whole, with room left under it (pane {room:?})"
            );
            // More than the room below the body holds: the footer takes it all, and scrolls.
            reasons.set(80);
            let (record, footer, end) = run(&mut harness);
            let footer = footer.expect("the footer shows");
            assert!(
                record.placement() == Placement::Follows
                    && (footer.top() - (end + S)).abs() <= 1.0
                    && (footer.bottom() - room.bottom()).abs() <= 1.0
                    && whole(&harness, 0, footer)
                    && !whole(&harness, 79, footer),
                "Q-c at {size:?}: the footer {footer:?} of 80 reasons follows the body's end {end} by `S`, takes the room down to the pane's end {}, and scrolls inside",
                room.bottom()
            );
            // The threshold: the body's view is the room less `S` and the pinned footer, a third.
            let view = record.viewport().expect("a content pane's view").height();
            let line = end - (room.top() + body.get());
            body.set((view - line - 2.0).floor());
            let (record, footer, end) = run(&mut harness);
            let footer = footer.expect("the footer shows");
            assert!(
                record.placement() == Placement::Follows
                    && (footer.top() - (end + S)).abs() <= 1.0
                    && (footer.bottom() - room.bottom()).abs() <= 1.0,
                "Q-c at {size:?}: a body {:?} px tall in a view of {view} is followed: footer {footer:?}, body end {end}",
                record.content_height()
            );
            body.set((view - line).ceil());
            let (record, footer, _) = run(&mut harness);
            let footer = footer.expect("the footer shows");
            assert!(
                record.placement() == Placement::Pinned
                    && (footer.bottom() - room.bottom()).abs() <= 1.0
                    && (footer.height() - third).abs() <= 1.0,
                "Q-c at {size:?}: a body {:?} px tall in a view of {view} pins the footer {footer:?} at a third ({third}) of the pane",
                record.content_height()
            );
        }

        // Migration-32 L1c: `pane_then` takes the footer's messages after the body, and a footer
        // that was not measured with its messages is laid out and placed in that same pass. So
        // with egui's reruns denied (one pass a frame), the frame of a press shows the messages
        // it brought, whole, never the ones it ended, and the next frame moves nothing: none →
        // one reason (a footer that comes) → three in its place (a footer that grew). Below a
        // short body the footer follows it by `S`; below a tall body (review of PR 456) it is
        // pinned at the pane's bottom, a third of the pane at most, with no body line under it.
        // The footer's plan is made before it is placed and its lines drawn once, in their place
        // (finding 2 of PR 452's review): in that first frame its controls' accessible bounds
        // are where they are painted, and a press there reaches them.
        const PRESS_WORDS: [&str; 4] = ["Press 0", "Press 1", "Press 2", "Press 3"];
        let shown = Rc::new(Cell::new(0_usize));
        let pressed = Rc::new(Cell::new(None::<usize>));
        let tall = Rc::new(Cell::new(false));
        let row_pressed = Rc::new(Cell::new(None::<usize>));
        let (shown_in, pressed_in, tall_in) = (shown.clone(), pressed.clone(), tall.clone());
        let row_in = row_pressed.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(960.0, 640.0))
            .themed_ui(move |ui| {
                ui.label("Title");
                // The typed footer (layout PR 2b): each reason a notice with its choice.
                use super::super::outcome::{Choice, MessageChoice, Messages, Notice, PaneFrame};
                const PRESS: [&str; 4] = PRESS_WORDS;
                let frame = PaneFrame::new(ui);
                let paned = Messages::<usize>::new(&frame, "then_footer", ui.available_width())
                    .pane_then(
                        ui,
                        Body::Content,
                        |ui| {
                            for (at, name) in ["Show one", "Show three"].into_iter().enumerate() {
                                if ui.button(name).clicked() {
                                    shown_in.set(at + 1);
                                }
                            }
                            if tall_in.get() {
                                // Registered rows, each a button over two lines (review of PR
                                // 456, P2: the footer's new edge cuts a row below its button).
                                for line in 0..60 {
                                    let row = ui.vertical(|ui| {
                                        if ui
                                            .button(format!("Row {line}"))
                                            .on_hover_text(format!("Tip {line}"))
                                            .clicked()
                                        {
                                            row_in.set(Some(line));
                                        }
                                        ui.label(format!("Line {line}"));
                                        ui.label("Detail");
                                    });
                                    super::register_row(ui, row.response.rect);
                                }
                            }
                            shown_in.get()
                        },
                        |shown, messages| {
                            let count = [0, 1, 3][shown];
                            for (line, press) in PRESS.iter().enumerate().take(count) {
                                messages.outcome(
                                    Notice::new(crate::sentence!("Reason {} of {}.", line, count))
                                        .choice(Choice::new(MessageChoice::Choice(press), line)),
                                );
                            }
                        },
                    );
                if let Some(press) = paned.pressed {
                    pressed_in.set(Some(press.id));
                }
            });
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::MIN);
        let id = Id::new("then_footer");
        let painted = |harness: &egui_kittest::Harness<'_>, words: &str| {
            harness
                .output()
                .shapes
                .iter()
                .find_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text == words => Some((
                        Rect::from_min_size(text.pos, text.galley.size()),
                        shape.clip_rect,
                    )),
                    _ => None,
                })
        };
        // What one frame shows: the footer (this pass's, from the pane's record: `footer_rect`
        // reads egui's response, which keeps the last pass's rect for a widget that had one),
        // each reason's painted rect when it is whole inside the footer, and the lowest body
        // line painted (its part inside its clip).
        let view = |harness: &egui_kittest::Harness<'_>, count: usize| {
            let footer = pane_record(&harness.ctx, id).and_then(|record| record.footer());
            let reasons: Vec<Option<Rect>> = (0..count)
                .map(|line| {
                    painted(harness, &format!("Reason {line} of {count}.")).and_then(
                        |(rect, clip)| {
                            footer
                                .is_some_and(|footer| {
                                    footer.contains_rect(rect)
                                        && clip.expand(0.5).contains_rect(rect)
                                })
                                .then_some(rect)
                        },
                    )
                })
                .collect();
            let body_low = harness
                .output()
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) if text.galley.job.text.starts_with("Line ") => {
                        let rect = Rect::from_min_size(text.pos, text.galley.size())
                            .intersect(shape.clip_rect);
                        rect.is_positive().then_some(rect.bottom())
                    }
                    _ => None,
                })
                .fold(f32::MIN, f32::max);
            (footer, reasons, body_low)
        };
        let same = |a: &(Option<Rect>, Vec<Option<Rect>>, f32),
                    b: &(Option<Rect>, Vec<Option<Rect>>, f32)| {
            let near = |a: Option<Rect>, b: Option<Rect>| match (a, b) {
                (Some(a), Some(b)) => {
                    (a.min - b.min).length() <= 0.5 && (a.max - b.max).length() <= 0.5
                }
                (a, b) => a.is_none() && b.is_none(),
            };
            near(a.0, b.0)
                && a.1.len() == b.1.len()
                && a.1.iter().zip(&b.1).all(|(a, b)| near(*a, *b))
                && (a.2 - b.2).abs() <= 0.5
        };
        // A press and release at `at`, all in the next frame's input (`Harness::event` would give
        // each event a frame of its own).
        let click = |harness: &mut egui_kittest::Harness<'_>, at: egui::Pos2| {
            harness
                .input_mut()
                .events
                .push(egui::Event::PointerMoved(at));
            for pressed in [true, false] {
                harness.input_mut().events.push(egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                });
            }
        };
        // Issue #510: a hovered body button's tooltip goes with the button. Below the tall body,
        // with no footer, the lowest row button's tooltip shows; a keyboard act brings three
        // reasons in one pass, and the footer they make covers that row: in that frame the row
        // is hidden, and no tooltip of it is painted.
        {
            use egui_kittest::kittest::{NodeT, Queryable};
            tall.set(true);
            shown.set(0);
            harness.event(egui::Event::PointerGone);
            harness.run_steps(6);
            let delay = harness.ctx.style().interaction.tooltip_delay;
            harness
                .ctx
                .all_styles_mut(|style| style.interaction.tooltip_delay = 0.0);
            let (row, at) = harness
                .query_all_by_role(egui::accesskit::Role::Button)
                .filter(|node| !node.accesskit_node().is_hidden() && node.rect().bottom() <= 640.0)
                .filter_map(|node| {
                    let label = node.accesskit_node().label()?;
                    label
                        .strip_prefix("Row ")?
                        .parse::<usize>()
                        .ok()
                        .map(|row| (row, node.rect()))
                })
                .max_by(|a, b| a.1.bottom().total_cmp(&b.1.bottom()))
                .expect("the tall body draws its rows");
            harness
                .input_mut()
                .events
                .push(egui::Event::PointerMoved(at.center()));
            harness.run_steps(4);
            let tip = format!("Tip {row}");
            assert!(
                painted(&harness, &tip).is_some(),
                "#510 setup: the hovered row {row}'s tooltip shows"
            );
            harness.get_by_label("Show three").focus();
            harness.step();
            harness.input_mut().events.push(egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            });
            harness.step();
            let hidden = harness
                .query_by_label(&format!("Row {row}"))
                .is_none_or(|node| node.accesskit_node().is_hidden());
            assert!(
                shown.get() == 2 && hidden && painted(&harness, &tip).is_none(),
                "#510: in the frame the footer covers row {row}, the row is hidden ({hidden}) and \
                 its tooltip is gone (shown {})",
                shown.get()
            );
            harness
                .ctx
                .all_styles_mut(|style| style.interaction.tooltip_delay = delay);
        }
        let mut press_checked = false;
        let mut gap_checked = false;
        for (is_tall, probe_gap) in [(false, false), (true, false), (true, true)] {
            tall.set(is_tall);
            row_pressed.set(None);
            shown.set(0);
            harness.event(egui::Event::PointerGone);
            harness.run_steps(6);
            assert!(
                footer_rect(&harness.ctx, id).is_none(),
                "pane_then (tall {is_tall}): no message, no footer"
            );
            let room = pane_record(&harness.ctx, id)
                .expect("the pane's record")
                .room();
            for (press, count, ended) in [
                ("Show one", 1, None),
                ("Show three", 3, Some("Reason 0 of 1.")),
            ] {
                // The press's own frame: hover first, then press and release, one pass each.
                let at = access(&harness)
                    .into_iter()
                    .find(|(label, _)| label == press)
                    .map(|(_, rect)| rect.center())
                    .unwrap_or_else(|| panic!("pane_then: {press} shows"));
                harness.event(egui::Event::PointerMoved(at));
                harness.step();
                for down in [true, false] {
                    harness.event(egui::Event::PointerButton {
                        pos: at,
                        button: egui::PointerButton::Primary,
                        pressed: down,
                        modifiers: egui::Modifiers::NONE,
                    });
                    harness.step();
                }
                let first = view(&harness, count);
                let (footer, reasons, body_low) = &first;
                let body_end = access(&harness)
                    .into_iter()
                    .filter(|(label, _)| label.starts_with("Show "))
                    .map(|(_, rect)| rect.bottom())
                    .fold(f32::MIN, f32::max);
                let placed = footer.is_some_and(|footer| {
                    if is_tall {
                        (footer.bottom() - room.bottom()).abs() <= 1.0
                            && footer.height() <= (room.height() / 3.0).floor() + 1.0
                            && *body_low <= footer.top() - S + 0.5
                    } else {
                        (footer.top() - (body_end + S)).abs() <= 1.0
                    }
                });
                assert!(
                    shown.get() == count.min(2)
                        && reasons.iter().all(Option::is_some)
                        && ended.is_none_or(|ended| painted(&harness, ended).is_none())
                        && placed,
                    "pane_then (tall {is_tall}), one pass a frame: the frame of {press} shows its {count} reason(s) whole in the footer {footer:?} (body ends {body_end}, lowest body line {body_low}, pane {room:?}), and not {ended:?}: {reasons:?}"
                );
                // The next frame, one pass. Below the tall body, after three reasons: in the act's
                // frame the first choice's accessible bounds are where it is painted, and a press
                // there, decided on that frame's targets, reaches it.
                let mut press_first = false;
                // Review of PR 456 (re-review, P2): the body's new edge holds for presses and for
                // a screen reader as for the paint. In the act's frame every row button below the
                // body's lowest painted line is hidden from a screen reader, and a press on one
                // that lies above the footer (between the cut row's top and the body's new edge)
                // reaches nothing in the next frame.
                if is_tall && count == 3 && probe_gap {
                    use egui_kittest::kittest::NodeT;
                    let footer = first.0.expect("the footer");
                    let hidden: Vec<(Rect, bool)> =
                        egui_kittest::kittest::Queryable::query_all_by_role(
                            &harness,
                            egui::accesskit::Role::Button,
                        )
                        .filter(|node| {
                            node.accesskit_node()
                                .label()
                                .is_some_and(|label| label.starts_with("Row "))
                        })
                        .filter(|node| node.rect().top() >= first.2 - 0.5)
                        .map(|node| (node.rect(), node.accesskit_node().is_hidden()))
                        .collect();
                    let gap = hidden
                        .iter()
                        .map(|(rect, _)| *rect)
                        .find(|rect| rect.bottom() <= footer.top() - 0.5);
                    assert!(
                        hidden.iter().all(|(_, is_hidden)| *is_hidden) && gap.is_some(),
                        "pane_then: in the act's frame each row button below the lowest painted line {} is hidden, one of them above the footer {footer:?}: {hidden:?}",
                        first.2
                    );
                    click(&mut harness, gap.expect("a button in the gap").center());
                    gap_checked = true;
                }
                if is_tall && count == 3 && !probe_gap {
                    let bounds =
                        egui_kittest::kittest::Queryable::query_by_label(&harness, PRESS_WORDS[0])
                            .expect("the first choice")
                            .rect();
                    let first_painted = painted(&harness, PRESS_WORDS[0])
                        .expect("the first choice is painted")
                        .0;
                    assert!(
                        bounds.expand(1.0).contains_rect(first_painted),
                        "pane_then: in the act's frame, the first choice's accessible bounds {bounds:?} hold where it is painted, {first_painted:?}"
                    );
                    click(&mut harness, first_painted.center());
                    press_first = true;
                }
                harness.step();
                let next = view(&harness, count);
                assert!(
                    same(&first, &next)
                        && pressed.get() == press_first.then_some(0)
                        && row_pressed.get().is_none(),
                    "pane_then (tall {is_tall}): the frame after {press} moves nothing ({first:?} then {next:?}), a press on the first choice where it is painted reaches it, and one in the body's hidden gap reaches no row: {:?}, row {:?}",
                    pressed.get(),
                    row_pressed.get()
                );
                if press_first {
                    pressed.set(None);
                    press_checked = true;
                }
                harness.event(egui::Event::PointerGone);
                harness.run_steps(4);
            }
        }
        assert!(press_checked, "pane_then: the press check ran");
        assert!(gap_checked, "pane_then: the gap check ran");
        // With reruns allowed again, an unchanged frame draws the footer in one pass: a footer
        // whose messages come after the body keeps its measured room and words.
        harness
            .ctx
            .options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(2).unwrap());
        harness.run_steps(4);
        let before = harness.ctx.cumulative_pass_nr();
        harness.step();
        assert_eq!(
            harness.ctx.cumulative_pass_nr() - before,
            1,
            "pane_then: an unchanged frame is drawn in one pass"
        );

        // One pane function in two `push_id` scopes (Astra re-check 3): A holds a message, B's
        // comes and goes. A's footer keeps its place, its focused Undo and its press. Two panes
        // with one identity in one scope are refused.
        let b_shows = Rc::new(Cell::new(false));
        let undone = Rc::new(Cell::new((0, 0)));
        let (b_in, undone_in) = (b_shows.clone(), undone.clone());
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(960.0, 640.0))
            .themed_ui(move |ui| {
                let pane_of = |ui: &mut Ui, side: usize, message: bool| {
                    let mut footer = Footer::new("shared_footer", ui.available_width());
                    if message {
                        let undone = &undone_in;
                        footer.legacy_reason(REASON, move |ui| {
                            if ui.button(format!("Undo {side}")).clicked() {
                                let (a, b) = undone.get();
                                undone.set(if side == 0 { (a + 1, b) } else { (a, b + 1) });
                            }
                        });
                    }
                    footer.pane(ui, Body::Content, |ui| rows(ui, 2));
                };
                ui.columns(2, |columns| {
                    columns[0].push_id("a", |ui| pane_of(ui, 0, true));
                    let b = b_in.get();
                    columns[1].push_id("b", |ui| pane_of(ui, 1, b));
                });
            });
        harness.run_steps(4);
        let a_footer = |harness: &egui_kittest::Harness<'_>| {
            pane_records(&harness.ctx)
                .into_iter()
                .filter(|record| record.local() == Id::new("shared_footer"))
                .min_by(|x, y| x.room().left().total_cmp(&y.room().left()))
                .and_then(|record| record.footer())
        };
        let harness_undo_id = |harness: &egui_kittest::Harness<'_>| {
            use egui_kittest::kittest::{NodeT, Queryable};
            harness
                .query_by_label("Undo 0")
                .map(|node| node.accesskit_node().id())
        };
        let focused = |harness: &egui_kittest::Harness<'_>| {
            harness
                .ctx
                .memory(|memory| memory.focused())
                .map(|id| egui::accesskit::NodeId(id.value()))
        };
        for _ in 0..20 {
            if focused(&harness).is_some() && focused(&harness) == harness_undo_id(&harness) {
                break;
            }
            harness.key_press(egui::Key::Tab);
            harness.run_steps(2);
        }
        let before = a_footer(&harness);
        assert!(
            before.is_some() && focused(&harness) == harness_undo_id(&harness),
            "A's Undo has keyboard focus"
        );
        for b in [true, false, true] {
            b_shows.set(b);
            harness.run_steps(4);
            assert_eq!(
                a_footer(&harness),
                before,
                "B's message ({b}) leaves A's footer where it is"
            );
            assert_eq!(
                focused(&harness),
                harness_undo_id(&harness),
                "B's message ({b}) leaves A's Undo focused"
            );
        }
        harness.key_press(egui::Key::Enter);
        harness.run_steps(3);
        assert_eq!(undone.get(), (1, 0), "Enter on A's Undo acts on A only");
        let twice = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(960.0, 640.0))
                .build_ui(|ui| {
                    ui.columns(2, |columns| {
                        for column in columns.iter_mut().take(1) {
                            Footer::new("one_name", 400.0).pane(column, Body::Content, |_| ());
                            Footer::new("one_name", 400.0).pane(column, Body::Content, |_| ());
                        }
                    });
                });
            harness.run_steps(1);
        }));
        let refusal = twice
            .err()
            .and_then(|panic| {
                panic.downcast_ref::<String>().cloned().or_else(|| {
                    panic
                        .downcast_ref::<&str>()
                        .map(|words| (*words).to_owned())
                })
            })
            .unwrap_or_default();
        assert!(
            refusal.contains("two panes with one identity"),
            "two panes with one identity in one scope are refused: {refusal:?}"
        );

        // A vertical `ScrollColumn` inside a `Content` pane in a release build (Astra re-check 6):
        // its content is drawn in place, and the pane scrolls it. With 2 rows the footer follows
        // them; with 200 the pane is pinned, cut, and the wheel brings the last row whole into
        // view above the footer.
        for count in [2, 200] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(960.0, 640.0))
                .themed_ui(move |ui| {
                    let mut footer = Footer::new("inline_footer", ui.available_width());
                    footer.legacy_reason(REASON, |ui| {
                        message(ui, Tone::Error, REASON);
                    });
                    footer.pane(ui, Body::Content, |ui| {
                        let _ = ScrollColumn::new("inner").inline(ui, |ui| rows(ui, count));
                    });
                });
            harness.run_steps(4);
            let footer = footer_rect(&harness.ctx, Id::new("inline_footer")).expect("the footer");
            let record = pane_record(&harness.ctx, Id::new("inline_footer")).unwrap();
            let line = |harness: &egui_kittest::Harness<'_>, name: &str| {
                access(harness)
                    .into_iter()
                    .find(|(label, _)| label == name)
                    .map(|(_, rect)| rect)
            };
            if count == 2 {
                let end = line(&harness, "Line 1").expect("Line 1").bottom();
                assert!(
                    (footer.top() - (end + S)).abs() <= 1.0,
                    "inline 2 rows: the footer {footer:?} follows them ({end})"
                );
            } else {
                assert!(record.cut(), "inline 200 rows: the pane is cut");
                harness.event(egui::Event::PointerMoved(pos2(
                    record.room().left() + 20.0,
                    record.room().top() + 20.0,
                )));
                for _ in 0..400 {
                    if line(&harness, "Line 199")
                        .is_some_and(|rect| rect.bottom() <= footer.top() - S + 0.5)
                    {
                        break;
                    }
                    harness.event(egui::Event::MouseWheel {
                        unit: egui::MouseWheelUnit::Point,
                        delta: vec2(0.0, -10.0 * Type::Body.line()),
                        modifiers: egui::Modifiers::NONE,
                    });
                    harness.run_steps(1);
                }
                let last = line(&harness, "Line 199").expect("Line 199");
                assert!(
                    last.bottom() <= footer.top() - S + 0.5 && last.top() >= record.room().top(),
                    "inline 200 rows: the pane scrolls Line 199 {last:?} whole into view above {footer:?}"
                );
            }
        }

        // The record is of the last completed pass: present, absent, present.
        let present = Rc::new(Cell::new(true));
        let present_in = present.clone();
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(960.0, 640.0))
            .build_ui(move |ui| {
                if present_in.get() {
                    let mut footer = Footer::new("toggled_footer", ui.available_width());
                    footer.legacy_reason(REASON, |ui| {
                        message(ui, Tone::Error, REASON);
                    });
                    footer.pane(ui, Body::Content, |ui| ui.label("A row"));
                }
            });
        let id = Id::new("toggled_footer");
        for shown in [true, false, true] {
            present.set(shown);
            harness.run_steps(3);
            let record = pane_record(&harness.ctx, id);
            assert_eq!(
                (record.is_some(), pane_records(&harness.ctx).is_empty()),
                (shown, !shown),
                "the pane's record is there exactly while the pane is ({shown})"
            );
            assert!(
                !shown || record.is_some_and(|record| record.footer().is_some()),
                "a present pane's record holds its footer"
            );
        }
        // A vertical scroll column inside a `Content` pane is refused.
        let refused = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(960.0, 640.0))
                .build_ui(|ui| {
                    Footer::new("refusing_footer", ui.available_width()).pane(
                        ui,
                        Body::Content,
                        |ui| {
                            ScrollColumn::new("inside").show(ui, |ui| ui.label("A row"));
                        },
                    );
                });
            harness.run_steps(1);
        }));
        let message = refused
            .err()
            .and_then(|panic| {
                panic.downcast_ref::<String>().cloned().or_else(|| {
                    panic
                        .downcast_ref::<&str>()
                        .map(|words| (*words).to_owned())
                })
            })
            .unwrap_or_default();
        assert!(
            message.contains("inside a `Body::Content` pane"),
            "a ScrollColumn inside a Content pane panics with the rule: {message:?}"
        );
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

    /// Lays out links side by side in one horizontal row, as World Problems does. Returns each
    /// link's press target and the left edge of its painted words.
    fn links_in_a_row(names: &[&str]) -> Vec<(Rect, f32, f32)> {
        let ctx = egui::Context::default();
        crate::design::theme::install(&ctx);
        let mut targets = Vec::new();
        let output = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    for name in names {
                        targets.push(link(ui, name).rect);
                    }
                });
            });
        });
        names
            .iter()
            .zip(targets)
            .map(|(name, target)| {
                let (left, width) = output
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text) if text.galley.job.text == *name => {
                            Some((text.pos.x, text.galley.size().x))
                        }
                        _ => None,
                    })
                    .unwrap_or_else(|| panic!("{name} is painted"));
                (target, left, width)
            })
            .collect()
    }

    #[test]
    fn iter16_a_sd1_short_links_side_by_side_never_share_a_target() {
        let links = links_in_a_row(&["Al", "Il", "Ilsa"]);
        for pair in links.windows(2) {
            let shared = pair[0].0.intersect(pair[1].0);
            assert!(
                shared.width() <= 0.0 || shared.height() <= 0.0,
                "SD1-no-overlap: {:?} and {:?} share {shared:?}",
                pair[0].0,
                pair[1].0
            );
        }
        for (target, left, width) in &links {
            assert!(
                target.width() >= XL && target.height() >= XL,
                "SD1-size: {target:?}"
            );
            // Must stay true: the word starts at the target's left edge.
            assert!(
                (left - target.left()).abs() <= 0.5 && *width < XL,
                "SD1-word-left: words at {left}, target {target:?}"
            );
        }
    }

    #[test]
    fn iter16_a_sd1_a_link_as_wide_as_its_target_keeps_its_look() {
        // Must stay true: words of `XL` or wider take exactly their own width, as before.
        for (target, left, width) in links_in_a_row(&["Whole run", "Timeline"]) {
            assert!(width >= XL, "fixture: {width}");
            assert!(
                (target.width() - width).abs() <= 1.0 && (left - target.left()).abs() <= 0.5,
                "SD1-wide-look: words {left}+{width}, target {target:?}"
            );
        }
    }

    /// Round 29 P-1 (G167, G168, G180, G184; iter-28 row 146): an add field shows how to finish
    /// it, as the `InlineCreate` form places its actions: `Add` after the field on its line, or
    /// under it in a narrow column; quiet `Cancel` on the kept line under them, at the field's
    /// left edge. Enter and `Add` add the words; Escape and `Cancel` clear them and say so.
    /// During an input method's composition, Enter belongs to the input method: it adds nothing
    /// and the commit lands in the field.
    #[test]
    #[allow(
        clippy::disallowed_methods,
        reason = "the test of the staged keys-only add field itself"
    )]
    fn iter29_p1_an_opened_add_field_shows_add_and_cancel() {
        use crate::design::test_harness::Themed as _;
        use egui_kittest::kittest::Queryable;
        const NAME: &str = "Log line for Hob";
        /// The field, and what each frame gave back: (enter, cancel, the words then).
        type Seen = (AddField, Vec<(bool, bool, String)>);
        let harness = |width: f32| {
            let field = AddField {
                focus: true,
                ..AddField::default()
            };
            let id = crate::field_id!("test_add_field").once(super::super::layout::Once::Example);
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 240.0))
                .themed_ui_state(
                    move |ui, (field, seen): &mut Seen| {
                        let result = add_field::<()>(id, NAME, "What happened?")
                            .actions(crate::word!("Add"), crate::word!("Cancel"))
                            .suggest(|words| {
                                let rows = if words.starts_with("Ash") {
                                    vec![("Ashfall Keep".into(), "Location".into(), ())]
                                } else {
                                    Vec::new()
                                };
                                vec![Suggestions {
                                    label: "World",
                                    rows,
                                }]
                            })
                            .show(ui, field);
                        if result.enter || result.cancel {
                            seen.push((result.enter, result.cancel, field.words.clone()));
                        }
                    },
                    (field, Vec::new()),
                );
            harness.run_steps(2);
            harness
        };
        // A reader's measure at 1200 px, where Cancel follows Add, and a column too narrow for
        // both on one line, where Cancel wraps under Add. Either way the field keeps the whole
        // room, and every action is whole and at least 24 px high.
        for (width, one_line) in [(480.0, true), (160.0, false)] {
            let harness = harness(width);
            let field = harness.get_by_label(NAME).rect();
            let (add, cancel) = (
                harness.get_by_label("Add").rect(),
                harness.get_by_label("Cancel").rect(),
            );
            let room = harness.ctx.content_rect().width();
            assert!(
                field.width() >= room - 2.0 * XL,
                "{width}: the field keeps the room: field {field:?} in {room}"
            );
            assert!(
                add.top() >= field.bottom() - 0.5 && (add.left() - field.left()).abs() <= 0.5,
                "{width}: Add starts the action line under the field: field {field:?}, Add {add:?}"
            );
            assert!(
                add.height() >= 24.0 && cancel.height() >= 24.0 && cancel.right() <= room,
                "{width}: whole actions: Add {add:?}, Cancel {cancel:?}"
            );
            if one_line {
                assert!(
                    (cancel.center().y - add.center().y).abs() <= 0.5
                        && cancel.left() >= add.right(),
                    "{width}: quiet Cancel follows Add: Add {add:?}, Cancel {cancel:?}"
                );
            } else {
                // Cancel that starts a line holds its words at the text edge (`line_start`).
                assert!(
                    cancel.top() >= add.bottom() - 0.5
                        && cancel.left() <= field.left() + 0.5
                        && cancel.left() >= field.left() - LINE_START_HANG - 0.5,
                    "{width}: Cancel wraps under Add at the text edge: Add {add:?}, \
                     Cancel {cancel:?}"
                );
            }
        }
        let mut harness = harness(480.0);
        let act = |harness: &mut egui_kittest::Harness<'_, Seen>, words: &str, end: &str| {
            harness.get_by_label(NAME).focus();
            harness.run_steps(1);
            harness.get_by_label(NAME).type_text(words);
            harness.run_steps(1);
            match end {
                "Enter" => harness.key_press(egui::Key::Enter),
                "Escape" => harness.key_press(egui::Key::Escape),
                action => harness.get_by_label(action).click(),
            }
            harness.run_steps(2);
            let seen = std::mem::take(&mut harness.state_mut().1);
            harness.state_mut().0.clear();
            seen
        };
        let added = |words: &str| vec![(true, false, words.to_owned())];
        let cancelled = vec![(false, true, String::new())];
        assert_eq!(
            act(&mut harness, "Met the ferryman", "Enter"),
            added("Met the ferryman")
        );
        assert_eq!(
            act(&mut harness, "Paid the toll", "Add"),
            added("Paid the toll")
        );
        assert_eq!(act(&mut harness, "Lost the map", "Escape"), cancelled);
        assert_eq!(act(&mut harness, "Lost the map", "Cancel"), cancelled);
        // A chosen suggestion is the layer Escape closes first, after the form's input-method
        // guard: during a composition Escape is the input method's, and the choice stays.
        harness.get_by_label(NAME).focus();
        harness.run_steps(1);
        harness.get_by_label(NAME).type_text("Ash");
        harness.run_steps(1);
        harness.key_press(egui::Key::ArrowDown);
        harness.run_steps(1);
        assert_eq!(
            harness.state().0.highlight,
            Some(0),
            "Down chooses the suggestion"
        );
        harness.event(egui::Event::Ime(egui::ImeEvent::Enabled));
        harness.step();
        harness.key_press(egui::Key::Escape);
        harness.run_steps(2);
        assert_eq!(
            (harness.state().0.highlight, harness.state().1.clone()),
            (Some(0), Vec::new()),
            "Escape of a composition keeps the choice and cancels nothing"
        );
        harness.event(egui::Event::Ime(egui::ImeEvent::Disabled));
        harness.step();
        harness.key_press(egui::Key::Escape);
        harness.run_steps(2);
        assert_eq!(
            (harness.state().0.highlight, harness.state().1.clone()),
            (None, Vec::new()),
            "Escape lets go of the choice first, and cancels nothing"
        );
        assert!(
            harness.get_by_label(NAME).is_focused() && harness.state().0.words == "Ash",
            "the field keeps the keyboard and its words"
        );
        harness.key_press(egui::Key::Escape);
        harness.run_steps(2);
        assert_eq!(std::mem::take(&mut harness.state_mut().1), cancelled);
        // An input method: a preedit, then Enter in the next frame to commit it, then the
        // commit. Nothing is added or cancelled, the field keeps the keyboard and the commit,
        // other words than the preedit, lands.
        harness.get_by_label(NAME).focus();
        harness.run_steps(1);
        harness.get_by_label(NAME).type_text("Ash");
        harness.run_steps(1);
        harness.event(egui::Event::Ime(egui::ImeEvent::Enabled));
        harness.event(egui::Event::Ime(egui::ImeEvent::Preedit("か".to_owned())));
        harness.step();
        harness.key_press(egui::Key::Enter);
        harness.step();
        harness.event(egui::Event::Ime(egui::ImeEvent::Commit("火".to_owned())));
        harness.event(egui::Event::Ime(egui::ImeEvent::Disabled));
        harness.run_steps(2);
        assert_eq!(
            harness.state().1,
            Vec::new(),
            "Enter of a composition adds nothing"
        );
        assert!(
            harness.state().0.words.ends_with('火'),
            "the commit lands: {:?}",
            harness.state().0.words
        );
        assert!(
            harness.get_by_label(NAME).is_focused(),
            "the field keeps the keyboard"
        );

        // R30 review: the reveal follows the part's layout room, not its paint clip. A pane's
        // whole-row cut changes the clip as the DM scrolls; here the clip's bottom moves by 6 px
        // on every other frame. After the field opened (and revealed itself), the DM's wheel
        // scroll stays where the wheel put it.
        struct Scrolled {
            field: AddField,
            frame: u32,
            offset: f32,
        }
        let mut wheel = egui_kittest::Harness::builder()
            .with_size(vec2(480.0, 300.0))
            .themed_ui_state(
                |ui, state: &mut Scrolled| {
                    state.frame += 1;
                    let cut = state.frame.is_multiple_of(2);
                    let field = &mut state.field;
                    let output = super::super::layout::ScrollColumn::new("iter29_p1_wheel")
                        .max_height(240.0)
                        .show(ui, |ui| {
                            let mut clip = ui.clip_rect();
                            if cut {
                                clip.max.y -= 6.0;
                            }
                            ui.set_clip_rect(clip);
                            space(ui, 400.0);
                            let id = crate::field_id!("test_add_field_wheel")
                                .once(super::super::layout::Once::Example);
                            let _ = add_field::<()>(id, "Plan a scene", "Plan a scene")
                                .actions(crate::word!("Add"), crate::word!("Cancel"))
                                .result_room(|_| {})
                                .show(ui, field);
                        });
                    state.offset = output.state.offset.y;
                },
                Scrolled {
                    field: AddField {
                        focus: true,
                        ..AddField::default()
                    },
                    frame: 0,
                    offset: 0.0,
                },
            );
        wheel.run_steps(6);
        let opened = wheel.state().offset;
        assert!(
            opened > 100.0,
            "the open field revealed itself: offset {opened}"
        );
        wheel.hover_at(egui::pos2(240.0, 120.0));
        wheel.event(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: vec2(0.0, 60.0),
            modifiers: egui::Modifiers::NONE,
        });
        wheel.run_steps(10);
        let now = wheel.state().offset;
        assert!(
            (now - (opened - 60.0)).abs() <= 1.0,
            "the DM's scroll stays: offset {opened} then {now}, the wheel put it at {}",
            opened - 60.0
        );

        // The keys-only add field (M-R0's run `Effect` line and loot field): no actions; the
        // field is its owner's (`FieldId`), so focus and its text history follow the owner, not
        // its place; its line keeps one `Read` row's height, empty or with words; Enter adds.
        let owner = crate::field_id!("test_keys_field").of(uuid::Uuid::from_u128(7));
        let mut keys = egui_kittest::Harness::builder()
            .with_size(vec2(480.0, 200.0))
            .themed_ui_state(
                move |ui, (field, seen): &mut Seen| {
                    let result = add_field::<()>(owner, NAME, "What happened?")
                        .keys_only(0.0, |_| None)
                        .show(ui, field);
                    if result.enter || result.cancel {
                        seen.push((result.enter, result.cancel, field.words.clone()));
                    }
                },
                (
                    AddField {
                        focus: true,
                        ..AddField::default()
                    },
                    Vec::new(),
                ),
            );
        keys.run_steps(2);
        assert!(
            keys.query_by_label("Add").is_none() && keys.query_by_label("Cancel").is_none(),
            "keys only: no actions"
        );
        assert_eq!(
            keys.ctx.memory(|memory| memory.focused()),
            Some(owner.id()),
            "the field is its owner's"
        );
        let empty = keys.get_by_label(NAME).rect().height();
        keys.get_by_label(NAME).type_text("Ash");
        keys.run_steps(2);
        let typed = keys.get_by_label(NAME).rect().height();
        assert!(
            (empty - Density::Read.control()).abs() < 0.5
                && (typed - Density::Read.control()).abs() < 0.5,
            "one Read row, empty {empty} and typed {typed}"
        );
        keys.key_press(egui::Key::Enter);
        keys.run_steps(2);
        assert_eq!(
            keys.state().1,
            [(true, false, "Ash".to_owned())],
            "Enter adds the words"
        );
    }
}

#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn iter_two_form_labels_share_sentence_case_small_muted_type() {
    let context = egui::Context::default();
    super::theme::install(&context);
    let output = context.run(Default::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            field(ui, "Default side", None, None, |_| ());
            text_field(ui, "Copy name", &mut String::new(), None, None);
        });
    });
    let labels: Vec<_> = output
        .shapes
        .iter()
        .filter_map(|shape| match &shape.shape {
            egui::Shape::Text(text) if !text.galley.job.text.is_empty() => Some(&text.galley.job),
            _ => None,
        })
        .collect();
    assert_eq!(
        labels
            .iter()
            .map(|job| job.text.as_str())
            .collect::<Vec<_>>(),
        ["Default side", "Copy name"]
    );
    for label in labels {
        assert_eq!(label.sections[0].format.font_id.size, Type::Small.size());
        assert_eq!(label.sections[0].format.color, colors().muted);
    }
}

#[test]
fn dialog_action_row_fits_the_long_run_close_choices() {
    for size in [vec2(960.0, 640.0), vec2(1920.0, 1080.0)] {
        let context = egui::Context::default();
        super::theme::install(&context);
        let mut rects = None;
        let _ = context.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.set_width(FORM_WIDTH);
                    let area = ui.max_rect();
                    let (lose, (save, keep)) = dialog_actions(
                        ui,
                        |ui| Action::destructive("Close and lose changes").show(ui).rect,
                        |ui| {
                            let save = Action::primary("Save as a new run and close").show(ui).rect;
                            let keep = Action::secondary("Keep running").show(ui).rect;
                            (save, keep)
                        },
                    );
                    rects = Some((lose, keep, save, area));
                });
            },
        );
        let (lose, keep, save, area) = rects.unwrap();
        assert!(lose.right() < keep.left(), "{lose:?} overlaps {keep:?}");
        assert!(keep.right() < save.left());
        assert_eq!(lose.center().y, keep.center().y);
        assert_eq!(save.center().y, keep.center().y);
        assert!(area.contains_rect(lose) && area.contains_rect(keep) && area.contains_rect(save));
    }
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Text measured outside a widget's own drawing: an action's natural width, a recipe's
    /// facts or layout, a host's fit. A host's quiet frames measure none, counted from its entry
    /// through its paint (layout 11c A2c).
    pub static TEXT_MEASURES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Count one measure of text (`TEXT_MEASURES`).
#[cfg(any(test, feature = "test-support"))]
pub fn text_measured() {
    TEXT_MEASURES.with(|measures| measures.set(measures.get() + 1));
}
