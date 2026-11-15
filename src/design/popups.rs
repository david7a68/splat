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
use std::{any::Any, borrow::Cow, marker::PhantomData};

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
///     let _ = Dropdown::new(splat::field_id!("doc_side").once(splat::layout::Once::Example), "Side")
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
///     let _ = splat::layout::Dropdown::<u8>::new(splat::field_id!("doc_side").once(splat::layout::Once::Example), "Side").show(ui, side);
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
/// let _ = splat::layout::Dropdown::new(splat::field_id!("doc_side").once(splat::layout::Once::Example), "Side").choice(0_u8);
/// ```
///
/// A blocked choice says why:
///
/// ```compile_fail,E0061
/// let _ = splat::layout::Dropdown::new(splat::field_id!("doc_side").once(splat::layout::Once::Example), "Side").blocked_choice(0_u8, "Allies");
/// ```
///
/// Plain labels and short labels compile:
///
/// ```
/// let _ = splat::layout::Dropdown::new(splat::field_id!("doc_short").once(splat::layout::Once::Example), "Session")
///     .choice_with_short_label(0_u8, "Session 1: Tonight", "Session 1");
/// ```
///
/// A caller cannot give a choice a weight (including a short closed label):
///
/// ```compile_fail,E0277
/// fn pick(ui: &eframe::egui::Ui) {
///     let _ = splat::layout::Dropdown::new(splat::field_id!("doc_weight").once(splat::layout::Once::Example), "Side")
///         .choice(0_u8, eframe::egui::RichText::new("Allies").strong());
/// }
/// ```
///
/// ```compile_fail,E0061
/// let _ = splat::layout::Dropdown::new(splat::field_id!("doc_short_weight").once(splat::layout::Once::Example), "Session")
///     .choice_with_short_label(0_u8, "Session 1: Tonight", "Session 1", eframe::egui::FontId::default());
/// ```
///
/// The current value is of the choices' type:
///
/// ```compile_fail,E0308
/// fn pick(ui: &mut eframe::egui::Ui, side: &mut String) {
///     let _ = splat::layout::Dropdown::new(splat::field_id!("doc_side").once(splat::layout::Once::Example), "Side")
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
    room: Option<Rect>,
    blocked: Option<&'a str>,
    command: Option<Word>,
    state: PhantomData<State>,
}

/// One choice: its value, its words, and why it cannot be chosen now.
struct Entry<'a, T> {
    value: T,
    label: Cow<'a, str>,
    short: Option<Cow<'a, str>>,
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
            room: None,
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
            room: self.room,
            blocked: self.blocked,
            command: self.command,
            state: PhantomData,
        }
    }

    pub fn choice(self, value: T, label: impl Into<Cow<'a, str>>) -> Dropdown<'a, T, HasChoices> {
        self.push(Entry {
            value,
            label: label.into(),
            short: None,
            blocked: None,
        })
    }

    /// The closed control may use a short name; the menu and accessible name keep the full
    /// label. Both are plain words: the part owns their regular weight.
    pub fn choice_with_short_label(
        self,
        value: T,
        label: impl Into<Cow<'a, str>>,
        short: impl Into<Cow<'a, str>>,
    ) -> Dropdown<'a, T, HasChoices> {
        self.push(Entry {
            value,
            label: label.into(),
            short: Some(short.into()),
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
            short: None,
            blocked: Some(reason),
        })
    }

    /// The control's width; the list has the same. Without it, the available width up to
    /// `FORM_WIDTH`.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// A smaller room owned by the control's layout (for example, a header above two
    /// panes). The part intersects it with the owning pane and clip; it chooses the menu's
    /// side, width and position inside that room. This cannot widen the parent's room.
    pub fn within(mut self, room: Rect) -> Self {
        self.room = Some(room);
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
                short: None,
                blocked: None,
            }));
        self
    }
}

/// The one font for a chosen value, including an editable value. A caller supplies words.
fn value_font(ui: &Ui) -> egui::FontId {
    Type::Body.font(ui)
}

impl Dropdown<'_, ()> {
    /// Accessibility marker for the popup group; the gate keeps checking its choices against
    /// neighbouring splitters and scroll bars after the group receives its proper role.
    pub(crate) const CHOICES_ROLE: &'static str = "Dropdown choices";

    /// The editable value inside a drop-down. Keeps typed and empty words on one baseline,
    /// in the same regular weight as a closed choice. The caller owns the surrounding field
    /// and its choices, and names the response. No font or weight can be passed.
    ///
    /// ```
    /// fn field(ui: &mut eframe::egui::Ui, value: &mut String) {
    ///     splat::layout::Dropdown::editable_value(ui,
    ///         splat::field_id!("doc_folder").once(splat::layout::Once::Example),
    ///         value, "Main folder", 200.0);
    /// }
    /// ```
    ///
    /// ```compile_fail,E0061
    /// fn field(ui: &mut eframe::egui::Ui, value: &mut String) {
    ///     splat::layout::Dropdown::editable_value(ui,
    ///         splat::field_id!("doc_folder").once(splat::layout::Once::Example),
    ///         value, "Main folder", 200.0, eframe::egui::FontId::default());
    /// }
    /// ```
    pub fn editable_value(
        ui: &mut Ui,
        id: FieldId,
        value: &mut String,
        empty: &str,
        width: f32,
    ) -> Response {
        let font = value_font(ui);
        let response = id.single_line(ui, value, |edit| {
            edit.frame(false)
                .font(font)
                .margin(Vec2::ZERO)
                .vertical_align(Align::Center)
                .desired_width(width)
                .min_size(vec2(width, super::XL))
        });
        if value.is_empty() {
            ui.painter().text(
                response.rect.left_center(),
                Align2::LEFT_CENTER,
                empty,
                value_font(ui),
                colors().text,
            );
        }
        response
    }

    /// Regular chosen words for a staged raw control. Its popup and events stay with the
    /// caller until it migrates to `Dropdown`.
    pub fn selected_words(ui: &Ui, words: impl Into<String>) -> egui::RichText {
        egui::RichText::new(words).font(value_font(ui))
    }
}

/// The list's frame: no inner margin, so the list is exactly as wide as its control.
fn list_frame(ui: &Ui) -> egui::Frame {
    egui::Frame::popup(ui.style()).inner_margin(0.0)
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
        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let sense = if enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let mut response = ui.interact(rect, control, sense);
        let index_key = control.with("chosen_index");
        let kept = ui.ctx().data(|data| data.get_temp::<usize>(index_key));
        let chosen_index = kept
            .filter(|&at| {
                self.choices
                    .get(at)
                    .is_some_and(|entry| entry.value == *current)
            })
            .or_else(|| {
                self.choices
                    .iter()
                    .position(|entry| entry.value == *current)
            });
        if let Some(at) = chosen_index {
            ui.ctx().data_mut(|data| data.insert_temp(index_key, at));
        }
        let chosen_entry = chosen_index.and_then(|at| self.choices.get(at));
        let chosen = chosen_entry.map(|entry| entry.label.as_ref());
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
                Some(label) => (
                    chosen_entry
                        .and_then(|entry| entry.short.as_deref())
                        .unwrap_or(label),
                    if enabled { c.text } else { c.muted },
                ),
                None => (self.name, c.faint),
            };
            let room = (width - 2.0 * S - ICON_SLOT).max(1.0);
            let galley = one_line(ui, words, value_font(ui), color, room);
            if galley.elided || chosen_entry.is_some_and(|entry| entry.short.is_some()) {
                response = response.on_hover_text(chosen.unwrap_or(words));
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
            (picked, command) = self.follow_list(ui, &response, current, rect, width);
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
    fn follow_list(
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
        let reveal_index = list.with("reveal_index");
        let revealed = ui
            .ctx()
            .data(|data| data.get_temp::<Option<Id>>(revealed_id))
            .flatten();
        let was_open = ui
            .ctx()
            .data(|data| data.get_temp::<bool>(was_open_id))
            .unwrap_or(false);
        // Toggle before any row measurement. A closed control draws only its chosen words.
        if response.clicked() {
            egui::Popup::toggle_id(ui.ctx(), list);
        }
        let drawn = egui::Popup::is_id_open(ui.ctx(), list)
            .then(|| self.list(ui, response, current, rect, width, (was_open, revealed)))
            .flatten();
        let ListFrame {
            picked,
            command,
            rows,
            reveal,
        } = drawn.unwrap_or_default();
        let shown = egui::Popup::is_id_open(ui.ctx(), list);
        // A choice or the command closes the list, also from the keyboard (egui closes a popup
        // on a pointer click only).
        if picked.is_some() || command {
            egui::Popup::close_id(ui.ctx(), list);
        }
        let open_now = shown && egui::Popup::is_id_open(ui.ctx(), list);
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
        previous: (bool, Option<Id>),
    ) -> Option<ListFrame<T>> {
        let (was_open, revealed) = previous;
        let list = self.id.id().with("list");
        let mut reveal = None;
        // The list's frame draws its stroke outside its content: the content is that much
        // smaller, so the list is exactly as wide as its control.
        let stroke = ui.style().visuals.window_stroke.width;
        // Its room: the owning pane's body above its footer, or the visible part of `ui`.
        let mut bounds = super::pane::body_room(ui)
            .unwrap_or_else(|| ui.clip_rect())
            .intersect(ui.clip_rect())
            .intersect(ui.ctx().content_rect());
        if let Some(room) = self.room {
            bounds = bounds.intersect(room);
        }
        if !bounds.is_positive() || rect.top() >= bounds.bottom() || rect.bottom() <= bounds.top() {
            egui::Popup::close_id(ui.ctx(), list);
            return None;
        }
        // Retained metrics belong to this control and these words, fonts and width.
        // A closed control never enters this function; an unchanged open frame reuses them.
        let words: Vec<_> = self
            .choices
            .iter()
            .map(|entry| (entry.label.as_ref(), entry.blocked))
            .chain(self.command.map(|word| (word.as_str(), None)))
            .collect();
        let key = list.with("metrics");
        let font = Type::Body.font(ui);
        let small = Type::Small.font(ui);
        let scale = ui.ctx().pixels_per_point();
        let kept = ui
            .ctx()
            .data(|data| data.get_temp::<std::sync::Arc<RowMetrics>>(key));
        let metrics = kept
            .filter(|kept| {
                kept.requested == width
                    && kept.room_width == bounds.width()
                    && kept.bounded == self.room.is_some()
                    && kept.font == font
                    && kept.small == small
                    && kept.scale == scale
                    && kept.command == self.command.is_some()
                    && kept.words.len() == words.len()
                    && kept
                        .words
                        .iter()
                        .zip(&words)
                        .all(|((label, reason), (now, blocked))| {
                            label == now && reason.as_deref() == *blocked
                        })
            })
            .unwrap_or_else(|| {
                #[cfg(test)]
                ROW_METRICS_MADE.with(|made| made.set(made.get() + words.len()));
                let longest = words
                    .iter()
                    .flat_map(|(label, reason)| {
                        let labels: Box<dyn Iterator<Item = &str>> = if self.room.is_some() {
                            Box::new(std::iter::once(*label))
                        } else {
                            Box::new(label.split_whitespace())
                        };
                        labels
                            .map(|word| (word, &font))
                            .chain(reason.iter().flat_map(|reason| {
                                reason.split_whitespace().map(|word| (word, &small))
                            }))
                    })
                    .map(|(word, font)| super::number_width(ui, word, font))
                    .fold(0.0, f32::max);
                let menu_width = width
                    .max(longest + 2.0 * S + ICON_SLOT + lane_width() + 2.0 * stroke)
                    .min(bounds.width());
                let rows_width = (menu_width - 2.0 * stroke - lane_width()).max(1.0);
                let mut places = Vec::with_capacity(words.len());
                let mut top = 0.0;
                for (at, (label, reason)) in words.iter().enumerate() {
                    if at == self.choices.len() {
                        top += HAIRLINE_WIDTH;
                    }
                    let height =
                        super::extras::choice_wrapped_height(ui, label, *reason, rows_width);
                    places.push((top, height));
                    top += height;
                }
                let metrics = std::sync::Arc::new(RowMetrics {
                    words: words
                        .iter()
                        .map(|(label, reason)| ((*label).to_owned(), reason.map(str::to_owned)))
                        .collect(),
                    requested: width,
                    room_width: bounds.width(),
                    bounded: self.room.is_some(),
                    menu_width,
                    font,
                    small,
                    scale,
                    command: self.command.is_some(),
                    places,
                    top,
                });
                ui.ctx()
                    .data_mut(|data| data.insert_temp(key, metrics.clone()));
                metrics
            });
        let width = metrics.menu_width;
        let inner = (width - 2.0 * stroke).max(1.0);
        let left = rect.left().clamp(bounds.left(), bounds.right() - width);
        let anchor = Rect::from_min_max(pos2(left, rect.top()), pos2(left + width, rect.bottom()));
        let places = &metrics.places;
        let top = metrics.top;
        let natural = top + 2.0 * stroke;
        let natural = if self.room.is_some() {
            natural
        } else {
            natural.min(ui.style().spacing.combo_height)
        };
        let below = bounds.bottom() - rect.bottom() - XS;
        let above = rect.top() - bounds.top() - XS;
        let (align, room) = if natural <= below || below >= above {
            (RectAlign::BOTTOM_START, below)
        } else {
            (RectAlign::TOP_START, above)
        };
        if room < Density::Read.control() + 2.0 * stroke {
            egui::Popup::close_id(ui.ctx(), list);
            return None;
        }
        let max = natural.min(room) - 2.0 * stroke;
        // A row that gained keyboard focus last pass comes into the list's view, centred: the
        // list opens at that offset, at once, so the next key sees the row.
        let reveal_index = list.with("reveal_index");
        let pending = ui
            .ctx()
            .data(|data| data.get_temp::<Option<usize>>(reveal_index))
            .flatten();
        let mut column = ScrollColumn::new(list.with("scroll"))
            .max_height(max)
            .min_scrolled_height(max)
            .fit_height();
        if let Some((top, height)) = pending.and_then(|at| places.get(at).copied()) {
            column = column.vertical_scroll_offset((top - (max - height) / 2.0).max(0.0));
        }
        let mut picked = None;
        let mut command = false;
        let mut rows = Vec::new();
        let shown = egui::Popup::from_response(response)
            .open_memory(None)
            .id(list)
            .anchor(anchor)
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
                        let row = super::extras::choice_wrapped(
                            ui,
                            selected,
                            &entry.label,
                            entry.blocked,
                        );
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
                        let row = super::extras::choice_wrapped(ui, false, word.as_str(), None);
                        if newly_focused(ui, &row, revealed) {
                            reveal = Some((row.id, self.choices.len()));
                        }
                        command = row.clicked();
                        rows.push(row.id);
                    }
                });
            });
        if let Some(shown) = &shown {
            ui.ctx().accesskit_node_builder(shown.response.id, |node| {
                node.set_role(egui::accesskit::Role::GenericContainer);
                node.set_label(format!("{} choices", self.name));
                node.set_role_description(Dropdown::CHOICES_ROLE);
            });
        }
        shown.map(|_| ListFrame {
            picked,
            command,
            rows,
            reveal,
        })
    }
}

#[derive(Clone)]
struct RowMetrics {
    words: Vec<(String, Option<String>)>,
    requested: f32,
    room_width: f32,
    bounded: bool,
    menu_width: f32,
    font: egui::FontId,
    small: egui::FontId,
    scale: f32,
    command: bool,
    places: Vec<(f32, f32)>,
    top: f32,
}
#[cfg(test)]
thread_local! {
    static ROW_METRICS_MADE: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

struct ListFrame<T> {
    picked: Option<T>,
    command: bool,
    rows: Vec<Id>,
    reveal: Option<(Id, usize)>,
}
impl<T> Default for ListFrame<T> {
    fn default() -> Self {
        Self {
            picked: None,
            command: false,
            rows: Vec::new(),
            reveal: None,
        }
    }
}

/// Whether `row` gained keyboard focus since the list last revealed a row, on a visible pass
/// (a popup's first pass only sizes it).
fn newly_focused(ui: &Ui, row: &Response, revealed: Option<Id>) -> bool {
    row.has_focus() && revealed != Some(row.id) && !ui.is_sizing_pass()
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
