//! Popup parts (package K2): the layers that callers drew by hand with egui.
//!
//! - [`Dropdown`]: one choice from a list, under its control and as wide as it. It replaces
//!   `egui::ComboBox`.
//! - [`rich_tip`]: a tooltip of typed lines, on hover and on keyboard focus. It replaces
//!   `Response::on_hover_ui` and `show_tooltip_ui`.
//! - [`drag_source`]: a drag's payload and its ghost (icon and words by the pointer). It replaces
//!   `dnd_set_drag_payload` with a `layer_painter` ghost.
//! - [`image_viewer`]: the image viewer, sized before the texture loads when the size is known.
//!
//! Each part's tests (`popups/tests.rs`) prove its rule once. A caller that uses a part, with no
//! layer, `Area` or raw interaction of its own, has no duty left for that rule.
use std::{any::Any, borrow::Cow, marker::PhantomData, sync::Arc};

use eframe::egui::{
    self, Align, Align2, Id, Layout, Rect, RectAlign, Response, Sense, Ui, Vec2, WidgetInfo,
    WidgetType, pos2, text::LayoutJob, vec2,
};

use super::{
    Action, Density, FORM_WIDTH, FieldId, HAIRLINE_WIDTH, ICON_SLOT, M, RADIUS_CONTROL, S,
    ScrollColumn, Type, Word, XS, XXL, colors, combo_icon, focus_ring, icons, lane_width, one_line,
    rule,
};

/// Typestate of a [`Dropdown`]: no choice yet.
pub struct NoChoice;
/// Typestate of a [`Dropdown`]: at least one choice; it can show.
pub struct HasChoices;

/// One choice from a list. The control shows the chosen label, cut at its end, with the whole
/// label on hover and in its accessible name. Its list opens under it (above it when it does not
/// fit below), as wide as the control, and inside the body that owns the control: a pane's body
/// room above its footer when there is one ([`pane::body_room`](super::pane::body_room)), else
/// the visible part of the control's `Ui`. A list taller than its room scrolls inside itself.
/// Each row wraps its words inside the list's width, so two names that differ at their end stay
/// apart. The keyboard opens the list (Enter or Space on the control) on the chosen row, moves
/// through it (arrow keys, Tab), chooses (Enter or Space) and closes it (Escape); the focused row
/// is always scrolled into view, and focus goes back to the control when the list closes. The
/// chosen row has a check mark. A blocked choice shows its reason on its row and takes no
/// selection. A blocked or disabled drop-down closes its list and takes no selection and no
/// command. The control's id comes from a [`FieldId`], so a layout change keeps its focus.
///
/// Duties left to the caller: the choices and their words, and what a choice does.
///
/// ```
/// use splat::layout::Dropdown;
/// fn pick(ui: &mut eframe::egui::Ui, side: &mut u8) {
///     let _ = Dropdown::new(splat::field_id!("doc_side"), "Side")
///         .choice(0, "Allies")
///         .blocked_choice(1, "Enemies", "Close the run first.")
///         .show(ui, side);
/// }
/// ```
///
/// A drop-down with no choice has no `show`:
///
/// ```compile_fail,E0599
/// fn pick(ui: &mut eframe::egui::Ui, side: &mut u8) {
///     let _ = splat::layout::Dropdown::<u8>::new(splat::field_id!("doc_side"), "Side").show(ui, side);
/// }
/// ```
///
/// Its id is a `FieldId`, never an id from the layout:
///
/// ```compile_fail,E0308
/// let _ = splat::layout::Dropdown::<u8>::new(eframe::egui::Id::new("side"), "Side");
/// ```
///
/// A choice is a value and its words, never a closure that draws:
///
/// ```compile_fail,E0061
/// let _ = splat::layout::Dropdown::new(splat::field_id!("doc_side"), "Side").choice(0_u8);
/// ```
///
/// A blocked choice says why:
///
/// ```compile_fail,E0061
/// let _ = splat::layout::Dropdown::new(splat::field_id!("doc_side"), "Side").blocked_choice(0_u8, "Allies");
/// ```
///
/// The current value is of the choices' type:
///
/// ```compile_fail,E0308
/// fn pick(ui: &mut eframe::egui::Ui, side: &mut String) {
///     let _ = splat::layout::Dropdown::new(splat::field_id!("doc_side"), "Side")
///         .choice(0_u8, "Allies")
///         .show(ui, side);
/// }
/// ```
#[must_use = "call show"]
pub struct Dropdown<'a, T, State = NoChoice> {
    id: FieldId,
    name: &'a str,
    choices: Vec<Entry<'a, T>>,
    width: Option<f32>,
    blocked: Option<&'a str>,
    command: Option<Word>,
    state: PhantomData<State>,
}

/// One choice: its value, its words, and why it cannot be chosen now.
struct Entry<'a, T> {
    value: T,
    label: Cow<'a, str>,
    blocked: Option<&'a str>,
}

/// What the DM did with a [`Dropdown`] this frame.
pub struct DropdownResponse {
    /// The control.
    pub response: Response,
    /// The DM chose another value.
    pub changed: bool,
    /// The DM chose the command after the choices (`Add campaign`).
    pub command: bool,
}

impl<'a, T> Dropdown<'a, T, NoChoice> {
    /// `name` is the control's accessible name (`Linked campaign`); the chosen label follows it.
    pub fn new(id: FieldId, name: &'a str) -> Self {
        Self {
            id,
            name,
            choices: Vec::new(),
            width: None,
            blocked: None,
            command: None,
            state: PhantomData,
        }
    }
}

impl<'a, T, State> Dropdown<'a, T, State> {
    fn push(mut self, entry: Entry<'a, T>) -> Dropdown<'a, T, HasChoices> {
        self.choices.push(entry);
        Dropdown {
            id: self.id,
            name: self.name,
            choices: self.choices,
            width: self.width,
            blocked: self.blocked,
            command: self.command,
            state: PhantomData,
        }
    }

    pub fn choice(self, value: T, label: impl Into<Cow<'a, str>>) -> Dropdown<'a, T, HasChoices> {
        self.push(Entry {
            value,
            label: label.into(),
            blocked: None,
        })
    }

    /// A choice that cannot be chosen now, with its reason on its row (`Linked to Salt Road`).
    pub fn blocked_choice(
        self,
        value: T,
        label: impl Into<Cow<'a, str>>,
        reason: &'a str,
    ) -> Dropdown<'a, T, HasChoices> {
        self.push(Entry {
            value,
            label: label.into(),
            blocked: Some(reason),
        })
    }

    /// The control's width; the list has the same. Without it, the available width up to
    /// `FORM_WIDTH`.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// The control cannot open now, and why: a lock instead of the caret, and the reason on
    /// hover and as its accessible description. An open list closes.
    pub fn blocked(mut self, reason: Option<&'a str>) -> Self {
        self.blocked = reason;
        self
    }

    /// One command after the choices, below a rule (`Add campaign`).
    pub fn command(mut self, word: Word) -> Self {
        self.command = Some(word);
        self
    }
}

impl<'a, T> Dropdown<'a, T, HasChoices> {
    /// More choices after the first.
    pub fn choices<L: Into<Cow<'a, str>>>(
        mut self,
        more: impl IntoIterator<Item = (T, L)>,
    ) -> Self {
        self.choices
            .extend(more.into_iter().map(|(value, label)| Entry {
                value,
                label: label.into(),
                blocked: None,
            }));
        self
    }
}

/// The list's frame: no inner margin, so the list is exactly as wide as its control.
fn list_frame(ui: &Ui) -> egui::Frame {
    egui::Frame::popup(ui.style()).inner_margin(0.0)
}

/// The space above and below a row's words, so a one-line row is a `Read` control.
fn row_pad() -> f32 {
    ((Density::Read.control() - Type::Body.line()) / 2.0).max(XS)
}

/// A row's words, wrapped inside `width`: the label in `Body`, and a blocked choice's reason in
/// `Small` below it.
fn row_galleys(
    ui: &Ui,
    label: &str,
    reason: Option<&str>,
    width: f32,
    color: egui::Color32,
) -> (Arc<egui::Galley>, Option<Arc<egui::Galley>>) {
    let c = colors();
    let room = (width - 2.0 * S - ICON_SLOT).max(1.0);
    let wrap = |words: &str, font: egui::FontId, color| {
        ui.fonts_mut(|fonts| fonts.layout(words.to_owned(), font, color, room))
    };
    (
        wrap(label, Type::Body.font(ui), color),
        reason.map(|reason| wrap(reason, Type::Small.font(ui), c.muted)),
    )
}

/// A row's height in `width`: a `Read` control, or its wrapped words and pad.
fn row_height(ui: &Ui, label: &str, reason: Option<&str>, width: f32) -> f32 {
    let (label, reason) = row_galleys(ui, label, reason, width, colors().text);
    let words = label.size().y + reason.map_or(0.0, |reason| reason.size().y);
    (words + 2.0 * row_pad()).max(Density::Read.control())
}

impl<T: PartialEq + Clone> Dropdown<'_, T, HasChoices> {
    pub fn show(self, ui: &mut Ui, current: &mut T) -> DropdownResponse {
        let c = colors();
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().min(FORM_WIDTH));
        let height = Density::Read.control();
        let enabled = self.blocked.is_none() && ui.is_enabled();
        let control = self.id.id();
        let list = control.with("list");
        let was_open_id = list.with("was_open");
        let (_, rect) = ui.allocate_space(vec2(width, height));
        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let mut response = ui.interact(rect, control, sense);
        let chosen = self
            .choices
            .iter()
            .find(|entry| entry.value == *current)
            .map(|entry| entry.label.as_ref());
        // A blocked or disabled drop-down has no open list: it takes no selection or command.
        if !enabled {
            egui::Popup::close_id(ui.ctx(), list);
            ui.ctx()
                .data_mut(|data| data.insert_temp(was_open_id, false));
        }
        let open = egui::Popup::is_id_open(ui.ctx(), list);
        if ui.is_rect_visible(rect) {
            let fill = if response.hovered() && enabled {
                c.hover
            } else {
                c.well
            };
            ui.painter()
                .rect_filled(rect, egui::CornerRadius::same(RADIUS_CONTROL as u8), fill);
            ui.painter().rect_stroke(
                rect,
                egui::CornerRadius::same(RADIUS_CONTROL as u8),
                egui::Stroke::new(1.0, c.field_border),
                egui::StrokeKind::Inside,
            );
            let (words, color) = match chosen {
                Some(label) => (label, if enabled { c.text } else { c.muted }),
                None => (self.name, c.faint),
            };
            let room = (width - 2.0 * S - ICON_SLOT).max(1.0);
            let galley = one_line(ui, words, Type::Body.font(ui), color, room);
            if galley.elided {
                response = response.on_hover_text(words);
            }
            ui.painter().galley(
                pos2(rect.left() + S, rect.center().y - galley.size().y / 2.0),
                galley,
                color,
            );
            let icon = Rect::from_min_size(
                pos2(rect.right() - S - ICON_SLOT, rect.top()),
                vec2(ICON_SLOT, height),
            );
            if enabled {
                combo_icon(ui, icon, ui.style().interact(&response), open);
            } else {
                ui.painter().text(
                    icon.center(),
                    Align2::CENTER_CENTER,
                    icons::READ_ONLY,
                    Type::Body.font(ui),
                    c.muted,
                );
            }
            if response.has_focus() {
                focus_ring(ui, rect);
            }
        }
        // The accessible name says the whole chosen label, which the control may cut.
        let named = match chosen {
            Some(label) => format!("{}: {label}", self.name),
            None => self.name.to_owned(),
        };
        let value = chosen.unwrap_or_default().to_owned();
        response.widget_info(|| {
            let mut info = WidgetInfo::labeled(WidgetType::ComboBox, enabled, &named);
            info.current_text_value = Some(value.clone());
            info
        });
        if let Some(reason) = self.blocked {
            ui.ctx()
                .accesskit_node_builder(control, |node| node.set_description(reason));
            response = response.on_hover_text(reason);
        }
        let mut picked = None;
        let mut command = false;
        if enabled {
            (picked, command) = self.list(ui, &response, current, rect, width);
        }
        let changed = picked.as_ref().is_some_and(|value| value != current);
        if let Some(value) = picked {
            *current = value;
        }
        if changed {
            response.mark_changed();
        }
        DropdownResponse {
            response,
            changed,
            command,
        }
    }

    /// The open list, placed inside the owning body's usable rect. Returns the value the DM
    /// chose and whether the DM chose the command.
    #[allow(
        clippy::disallowed_methods,
        reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
    )]
    fn list(
        &self,
        ui: &mut Ui,
        response: &Response,
        current: &T,
        rect: Rect,
        width: f32,
    ) -> (Option<T>, bool) {
        let list = self.id.id().with("list");
        let was_open_id = list.with("was_open");
        let rows_id = list.with("rows");
        let revealed_id = list.with("revealed");
        let revealed = ui
            .ctx()
            .data(|data| data.get_temp::<Option<Id>>(revealed_id))
            .flatten();
        let mut reveal = None;
        let was_open = ui
            .ctx()
            .data(|data| data.get_temp::<bool>(was_open_id))
            .unwrap_or(false);
        // The list's frame draws its stroke outside its content: the content is that much
        // smaller, so the list is exactly as wide as its control.
        let stroke = ui.style().visuals.window_stroke.width;
        let inner = width - 2.0 * stroke;
        // Its room: the owning pane's body above its footer, or the visible part of `ui`.
        let bounds = super::pane::body_room(ui)
            .unwrap_or_else(|| ui.clip_rect())
            .intersect(ui.ctx().content_rect());
        let rows_width = inner - lane_width();
        // Each row's top and height in the list, the command's after its rule.
        let mut places = Vec::with_capacity(self.choices.len() + 1);
        let mut top = 0.0;
        for entry in &self.choices {
            let height = row_height(ui, &entry.label, entry.blocked, rows_width);
            places.push((top, height));
            top += height;
        }
        if let Some(word) = self.command {
            top += HAIRLINE_WIDTH;
            let height = row_height(ui, word.as_str(), None, rows_width);
            places.push((top, height));
            top += height;
        }
        let natural = top + 2.0 * stroke;
        let natural = natural.min(ui.style().spacing.combo_height);
        let below = bounds.bottom() - rect.bottom() - XS;
        let above = rect.top() - bounds.top() - XS;
        let (align, room) = if natural <= below || below >= above {
            (RectAlign::BOTTOM_START, below)
        } else {
            (RectAlign::TOP_START, above)
        };
        let max = (natural.min(room) - 2.0 * stroke).max(Density::Read.control());
        // A row that gained keyboard focus last pass comes into the list's view, centred: the
        // list opens at that offset, at once, so the next key sees the row.
        let reveal_index = list.with("reveal_index");
        let pending = ui
            .ctx()
            .data(|data| data.get_temp::<Option<usize>>(reveal_index))
            .flatten();
        let mut column = ScrollColumn::new(list.with("scroll"))
            .max_height(max)
            .fit_height();
        if let Some((top, height)) = pending.and_then(|at| places.get(at).copied()) {
            column = column.vertical_scroll_offset((top - (max - height) / 2.0).max(0.0));
        }
        let mut picked = None;
        let mut command = false;
        let mut rows = Vec::new();
        let shown = egui::Popup::from_toggle_button_response(response)
            .id(list)
            .width(inner)
            .frame(list_frame(ui))
            .align(align)
            .align_alternatives(&[])
            .gap(XS)
            .show(|ui| {
                ui.set_width(inner);
                ui.spacing_mut().item_spacing.y = 0.0;
                column.show(ui, |ui| {
                    for (at, entry) in self.choices.iter().enumerate() {
                        let selected = entry.value == *current;
                        let row = choice_row(ui, selected, &entry.label, entry.blocked);
                        // The list opens with the keyboard on the chosen row.
                        if !was_open && selected && entry.blocked.is_none() {
                            row.request_focus();
                        }
                        if newly_focused(ui, &row, revealed) {
                            reveal = Some((row.id, at));
                        }
                        if row.clicked() && entry.blocked.is_none() {
                            picked = Some(entry.value.clone());
                        }
                        rows.push(row.id);
                    }
                    if let Some(word) = self.command {
                        rule(ui);
                        let row = choice_row(ui, false, word.as_str(), None);
                        if newly_focused(ui, &row, revealed) {
                            reveal = Some((row.id, self.choices.len()));
                        }
                        command = row.clicked();
                        rows.push(row.id);
                    }
                });
            });
        // A choice or the command closes the list, also from the keyboard (egui closes a popup
        // on a pointer click only).
        if picked.is_some() || command {
            egui::Popup::close_id(ui.ctx(), list);
        }
        let open_now = shown.is_some() && egui::Popup::is_id_open(ui.ctx(), list);
        // Focus goes back to the control when the list closes with the keyboard on it, or on
        // nothing (Escape, a chosen row).
        if was_open && !open_now {
            let last_rows = ui
                .ctx()
                .data(|data| data.get_temp::<Vec<Id>>(rows_id))
                .unwrap_or_default();
            let focused = ui.ctx().memory(|memory| memory.focused());
            if focused.is_none_or(|id| last_rows.contains(&id)) {
                response.request_focus();
            }
        }
        ui.ctx().data_mut(|data| {
            data.insert_temp(reveal_index, reveal.map(|(_, at)| at));
            if let Some((row, _)) = reveal {
                data.insert_temp(revealed_id, Some(row));
            }
            if !open_now {
                data.insert_temp(revealed_id, None::<Id>);
            }
            data.insert_temp(was_open_id, open_now);
            if open_now {
                data.insert_temp(rows_id, rows);
            }
        });
        (picked, command)
    }
}

/// Whether `row` gained keyboard focus since the list last revealed a row, on a visible pass
/// (a popup's first pass only sizes it).
fn newly_focused(ui: &Ui, row: &Response, revealed: Option<Id>) -> bool {
    row.has_focus() && revealed != Some(row.id) && !ui.is_sizing_pass()
}

/// One row of a list: a check mark when it is the chosen one, then its words, wrapped inside
/// the row (never cut), and a blocked choice's reason below them; a blocked row takes no press
/// and no keyboard focus.
///
/// Package X's `design::choice` (branch `iter-28-X-extras`) draws a selected choice with its
/// check mark too, but cuts its words. The test `one_choice_row` fails when both exist, so
/// whoever lands second makes them one part.
fn choice_row(ui: &mut Ui, selected: bool, label: &str, blocked: Option<&str>) -> Response {
    let c = colors();
    let width = ui.available_width();
    let color = if blocked.is_some() { c.muted } else { c.text };
    let (words, reason) = row_galleys(ui, label, blocked, width, color);
    let height = row_height(ui, label, blocked, width);
    let sense = if blocked.is_some() {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), sense);
    response.widget_info(|| {
        WidgetInfo::selected(
            WidgetType::SelectableLabel,
            blocked.is_none() && ui.is_enabled(),
            selected,
            label,
        )
    });
    if let Some(reason) = blocked {
        ui.ctx()
            .accesskit_node_builder(response.id, |node| node.set_description(reason));
    }
    if ui.is_rect_visible(rect) {
        let fill = if blocked.is_some() {
            None
        } else if response.is_pointer_button_down_on() {
            Some(c.selected)
        } else if response.hovered() {
            Some(c.hover)
        } else {
            None
        };
        if let Some(fill) = fill {
            ui.painter()
                .rect_filled(rect, egui::CornerRadius::same(RADIUS_CONTROL as u8), fill);
        }
        let top = rect.top() + row_pad();
        if selected {
            ui.painter().text(
                pos2(rect.left() + S, top + Type::Body.line() / 2.0),
                Align2::LEFT_CENTER,
                icons::OK,
                Type::Body.font(ui),
                c.accent,
            );
        }
        let left = rect.left() + S + ICON_SLOT;
        let below = words.size().y;
        ui.painter().galley(pos2(left, top), words, color);
        if let Some(reason) = reason {
            ui.painter()
                .galley(pos2(left, top + below), reason, c.muted);
        }
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response
}

/// One line of a [`rich_tip`]. Typed, so the tip's type and colours are the part's.
#[derive(Clone, Debug)]
pub enum TipLine<'a> {
    /// A name or a heading: `Strong`, in the text colour.
    Title(Cow<'a, str>),
    /// Words that wrap: `Body`, in the text colour.
    Body(Cow<'a, str>),
    /// A detail: `Small`, muted.
    Detail(Cow<'a, str>),
    /// Words with their own colours (a roll's parts), made by a typed producer.
    Job(LayoutJob),
}

impl<'a> TipLine<'a> {
    pub fn title(words: impl Into<Cow<'a, str>>) -> Self {
        Self::Title(words.into())
    }

    pub fn body(words: impl Into<Cow<'a, str>>) -> Self {
        Self::Body(words.into())
    }

    pub fn detail(words: impl Into<Cow<'a, str>>) -> Self {
        Self::Detail(words.into())
    }

    fn words(&self) -> Cow<'_, str> {
        match self {
            Self::Title(words) | Self::Body(words) | Self::Detail(words) => {
                Cow::Borrowed(words.as_ref())
            }
            Self::Job(job) => Cow::Owned(job.text.clone()),
        }
    }
}

/// The widest a tip grows before its words wrap.
pub const TIP_WIDTH: f32 = 360.0;

/// A tooltip of typed lines on `response`: on hover, and while the control has keyboard focus,
/// so a keyboard user reads it too. The lines are also the control's accessible description.
///
/// ```
/// use splat::layout::{TipLine, rich_tip};
/// fn more(response: eframe::egui::Response) -> eframe::egui::Response {
///     rich_tip(response, &[TipLine::title("Brannoch Vale"), TipLine::detail("58 of 62 HP")])
/// }
/// ```
///
/// A closure that draws is not a line:
///
/// ```compile_fail,E0308
/// fn more(response: eframe::egui::Response) -> eframe::egui::Response {
///     splat::layout::rich_tip(response, |ui: &mut eframe::egui::Ui| { ui.label("Brannoch"); })
/// }
/// ```
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn rich_tip(response: Response, lines: &[TipLine<'_>]) -> Response {
    let words: Vec<_> = lines.iter().map(TipLine::words).collect();
    let description = words.join(". ");
    response
        .ctx
        .accesskit_node_builder(response.id, |node| node.set_description(description));
    let draw = |ui: &mut Ui| {
        let c = colors();
        ui.set_max_width(TIP_WIDTH);
        ui.spacing_mut().item_spacing.y = 0.0;
        for line in lines {
            match line {
                TipLine::Title(words) => {
                    ui.label(Type::Strong.rich(ui, words.as_ref()).color(c.text));
                }
                TipLine::Body(words) => {
                    ui.add(
                        egui::Label::new(Type::Body.rich(ui, words.as_ref()).color(c.text)).wrap(),
                    );
                }
                TipLine::Detail(words) => {
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, words.as_ref()).color(c.muted))
                            .wrap(),
                    );
                }
                TipLine::Job(job) => {
                    ui.add(egui::Label::new(job.clone()).wrap());
                }
            }
        }
    };
    if response.has_focus() {
        response.show_tooltip_ui(draw);
        response
    } else {
        response.on_hover_ui(draw)
    }
}

/// What a dragged item shows by the pointer: an icon and words, in the accent colour on a raised
/// plate, `M` right of the pointer.
#[derive(Clone, Copy, Debug)]
pub struct Ghost<'a> {
    icon: &'static str,
    words: &'a str,
}

impl<'a> Ghost<'a> {
    pub fn new(icon: &'static str, words: &'a str) -> Self {
        Self { icon, words }
    }
}

/// `grip` is a drag source: it carries `payload` (when there is one: an item that cannot move
/// carries none) and, while it is dragged, the grabbing cursor and its [`Ghost`] on the tooltip
/// layer by the pointer, kept inside the window. A drop target reads the payload with egui's
/// `Response::dnd_release_payload`.
///
/// ```
/// use splat::layout::{Ghost, drag_source};
/// fn grip(ui: &eframe::egui::Ui, response: &eframe::egui::Response) {
///     drag_source(ui, response, Some(7_u32), Ghost::new("+", "Cinder Wolf"));
/// }
/// ```
///
/// A ghost is typed words, never a closure that paints:
///
/// ```compile_fail,E0308
/// fn grip(ui: &eframe::egui::Ui, response: &eframe::egui::Response) {
///     splat::layout::drag_source(ui, response, Some(7_u32), |painter: &eframe::egui::Painter| {});
/// }
/// ```
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn drag_source<P: Any + Send + Sync>(
    ui: &Ui,
    grip: &Response,
    payload: Option<P>,
    ghost: Ghost<'_>,
) {
    if let Some(payload) = payload {
        grip.dnd_set_drag_payload(payload);
    }
    if !grip.dragged() {
        return;
    }
    ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
    let Some(pointer) = ui.input(|input| input.pointer.latest_pos()) else {
        return;
    };
    let c = colors();
    let painter = ui.ctx().layer_painter(egui::LayerId::new(
        egui::Order::Tooltip,
        ghost_layer(grip.id),
    ));
    let galley = painter.layout_no_wrap(
        format!("{} {}", ghost.icon, ghost.words),
        Type::Strong.font(ui),
        c.accent,
    );
    let window = ui.ctx().content_rect();
    let size = galley.size() + Vec2::splat(2.0 * XS);
    let mut place = pointer + vec2(M, -size.y / 2.0);
    place.x = place.x.min(window.right() - size.x).max(window.left());
    place.y = place.y.min(window.bottom() - size.y).max(window.top());
    let plate = Rect::from_min_size(place, size);
    painter.rect_filled(plate, RADIUS_CONTROL, c.raised);
    painter.galley(plate.min + Vec2::splat(XS), galley, c.accent);
}

/// The ghost's own layer, one per grip.
fn ghost_layer(grip: Id) -> Id {
    grip.with("drag_ghost")
}

/// The image viewer's size: known before the texture loads (from the file's header), or not.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageSize {
    /// The image's size in pixels. The viewer fits it to the window at once, so the modal does
    /// not grow when the texture arrives.
    Known(Vec2),
    /// The viewer sizes itself when the texture arrives.
    Unknown,
}

/// The room an image has in the viewer: the window less the margins, the Close line and the
/// name line.
fn viewer_room(viewport: Rect) -> Vec2 {
    viewport.size()
        - vec2(
            XXL * 2.0,
            XXL * 2.0 + Density::Scan.control() + Type::Small.line() + S,
        )
}

/// `size` shrunk to fit `room`, never grown.
pub fn fitted(size: Vec2, room: Vec2) -> Vec2 {
    size * (room.x / size.x).min(room.y / size.y).min(1.0)
}

/// An image in a modal, over a rail-coloured backdrop, with Close (Esc) and its name. A click on
/// the image closes it too. Returns true when the DM closed it.
///
/// ```
/// use splat::layout::{ImageSize, image_viewer};
/// fn view(ctx: &eframe::egui::Context) -> bool {
///     image_viewer(ctx, "Map", "file://map.png", ImageSize::Known(eframe::egui::vec2(800.0, 600.0)))
/// }
/// ```
///
/// The size is said, known or not:
///
/// ```compile_fail,E0061
/// fn view(ctx: &eframe::egui::Context) -> bool {
///     splat::layout::image_viewer(ctx, "Map", "file://map.png")
/// }
/// ```
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn image_viewer(ctx: &egui::Context, name: &str, uri: &str, size: ImageSize) -> bool {
    let room = viewer_room(ctx.content_rect());
    let response = egui::Modal::new(Id::new("image_viewer"))
        .backdrop_color(colors().rail)
        .frame(egui::Frame::new())
        .show(ctx, |ui| {
            let fitted = match size {
                ImageSize::Known(size) if size.min_elem() > 0.0 => Some(fitted(size, room)),
                _ => None,
            };
            if let Some(size) = fitted {
                ui.set_width(size.x);
            }
            let mut close = false;
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    close = Action::quiet("Close")
                        .icon(icons::CLOSE)
                        .key("Esc")
                        .show(ui)
                        .clicked();
                });
            });
            let image = egui::Image::from_uri(uri);
            let response = match fitted {
                // The target has its size at once, also in the modal's first (sizing) pass, so
                // the modal does not grow when the texture arrives.
                Some(size) => {
                    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
                    image.paint_at(ui, rect);
                    response
                }
                None => ui.add(
                    image
                        .sense(Sense::click())
                        .fit_to_original_size(1.0)
                        .max_size(room),
                ),
            }
            .on_hover_text(name);
            response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, name));
            ui.label(Type::Small.rich(ui, name).color(colors().text));
            close || response.clicked()
        });
    response.should_close() || response.inner
}

#[cfg(test)]
mod tests;
