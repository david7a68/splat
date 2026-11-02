//! Shared UI parts. Screens compose these instead of styling widgets themselves.
use super::{icons, tokens::*};
use eframe::egui::{
    self, Align, Align2, Color32, CornerRadius, FontId, Id, Layout, Rect, Response, Sense, Stroke,
    StrokeKind, Ui, Vec2, WidgetInfo, WidgetType, pos2, text::LayoutJob, vec2,
};
use std::sync::Arc;

fn corner(value: f32) -> CornerRadius {
    CornerRadius::same(value as u8)
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

/// `text` with its middle cut so that it fits on one line, for a path: its two ends say the
/// most. Whoever shows this has the whole on hover.
pub fn cut_middle(ui: &Ui, text: &str, font: &FontId, width: f32) -> String {
    if number_width(ui, text, font) <= width {
        return text.to_owned();
    }
    let chars: Vec<char> = text.chars().collect();
    let cut = |keep: usize| -> String {
        let front = keep / 2;
        let back = keep - front;
        let head: String = chars[..front].iter().collect();
        let tail: String = chars[chars.len() - back..].iter().collect();
        format!("{head}…{tail}")
    };
    let (mut fits, mut over) = (0, chars.len());
    while over - fits > 1 {
        let keep = usize::midpoint(fits, over);
        if number_width(ui, &cut(keep), font) <= width {
            fits = keep;
        } else {
            over = keep;
        }
    }
    cut(fits)
}

/// As many whole words of `text` as fit. For a place that repeats a name shown whole nearby:
/// no letter is cut, and the first word always stays.
pub fn words_that_fit(ui: &Ui, text: &str, font: &FontId, width: f32) -> String {
    let mut fitted = String::new();
    for word in text.split_whitespace() {
        let longer = if fitted.is_empty() {
            word.to_owned()
        } else {
            format!("{fitted} {word}")
        };
        if !fitted.is_empty() && number_width(ui, &longer, font) > width {
            break;
        }
        fitted = longer;
    }
    fitted
}

/// Temporary HP, with its mark, in every place that shows it.
pub fn temp_hp_text(temp: i64) -> String {
    format!("{} +{temp}", icons::TEMP_HP)
}

/// Controls that cannot be used for one reason, which each of them says on hover.
pub fn locked<R>(ui: &mut Ui, reason: Option<&str>, add: impl FnOnce(&mut Ui) -> R) -> R {
    let group = ui.add_enabled_ui(reason.is_none(), add);
    if let Some(reason) = reason {
        group.response.on_disabled_hover_text(reason);
    }
    group.inner
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
}

/// A heading at the top of a page: title, a line of context, and actions.
pub fn page_header(ui: &mut Ui, title: &str, context: Option<&str>, actions: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(Density::Scan.control());
        ui.label(Type::Title.rich(ui, title).color(colors().text));
        if let Some(context) = context {
            ui.add_space(XS);
            ui.add(
                egui::Label::new(Type::Small.rich(ui, context).color(colors().muted)).truncate(),
            );
        }
        ui.with_layout(Layout::right_to_left(Align::Center), actions);
    });
    rule(ui);
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

/// A rail control that opens a panel beside the page. It is not a place, so it never
/// looks like the selected page: an open panel shows an accent icon and the word Open.
pub fn nav_toggle(ui: &mut Ui, icon: &str, label: &str, key: &str, open: bool) -> Response {
    let c = colors();
    let (rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), Density::Scan.control()),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        if response.hovered() {
            ui.painter()
                .rect_filled(rect, corner(RADIUS_CONTROL), c.hover);
        }
        let color = if open { c.accent } else { c.muted };
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
            if open { c.accent } else { c.faint },
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
}

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
        }
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

    pub fn show(self, ui: &mut Ui) -> Response {
        let enabled = self.blocked.is_none();
        // `add_enabled` keeps the parent's layout, so wrapped rows still wrap.
        let mut response = ui.add_enabled(enabled, |ui: &mut Ui| self.paint(ui));
        if let Some(reason) = self.blocked {
            response = response.on_disabled_hover_text(reason);
        }
        if let Some(hint) = self.hint {
            response = response.on_hover_text(hint);
        }
        response
    }

    fn paint(&self, ui: &mut Ui) -> Response {
        let c = colors();
        let font = if self.kind == Kind::Primary {
            Type::Heading
        } else if self.plain {
            Type::Body
        } else {
            Type::Strong
        };
        let font = font.font(ui);
        let padding = if self.kind == Kind::Quiet { S } else { M };
        let icon_width = self.icon.map_or(0.0, |_| Type::Body.size() + XS + 2.0);
        let room =
            ui.available_width() - 2.0 * padding - icon_width - self.key.map_or(0.0, |_| 30.0);
        let foreground = match self.kind {
            Kind::Primary if !ui.is_enabled() => c.muted,
            Kind::Primary => c.on_accent,
            Kind::Secondary | Kind::Quiet => c.text,
            Kind::Destructive => c.destructive,
        };
        let galley = one_line(ui, self.label, font, foreground, room);
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
        let width = (galley.size().x + 2.0 * padding + icon_width + key_width).max(self.height);
        let (rect, response) = ui.allocate_exact_size(vec2(width, self.height), Sense::click());
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
            // On, the control has its own fill and edge: it can stand on any surface,
            // the dock's included.
            Kind::Quiet if self.selected => (c.current_row, Stroke::new(HAIRLINE_WIDTH, c.accent)),
            Kind::Quiet => (plain_fill, Stroke::NONE),
            Kind::Destructive => (plain_fill, Stroke::new(HAIRLINE_WIDTH, c.destructive)),
        };
        ui.painter().rect(
            rect,
            corner(RADIUS_CONTROL),
            fill,
            stroke,
            StrokeKind::Inside,
        );
        let mut x = rect.left() + padding;
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

/// How wide a secondary action with an icon and this label is, to share a line with something
/// that must keep its words.
pub fn action_width(ui: &Ui, label: &str, key: bool) -> f32 {
    let icon_width = Type::Body.size() + XS + 2.0;
    let key_width = if key {
        number_width(ui, "U", &Type::Small.font(ui)) + S
    } else {
        0.0
    };
    number_width(ui, label, &Type::Strong.font(ui)) + 2.0 * M + icon_width + key_width
}

/// A square button with one icon. The word is the accessible name and the tooltip.
pub fn icon_button(ui: &mut Ui, icon: &str, name: &str) -> Response {
    let (rect, response) =
        ui.allocate_exact_size(Vec2::splat(Density::Read.control()), Sense::click());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, ui.is_enabled(), name));
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
}

/// An icon button that stays pressed while its state is on: one mark for two states would
/// say nothing. On, it has the same fill and edge as a quiet action that is on.
pub fn icon_toggle(ui: &mut Ui, icon: &str, name: &str, on: bool) -> Response {
    let (rect, response) =
        ui.allocate_exact_size(Vec2::splat(Density::Read.control()), Sense::click());
    response.widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), on, name));
    if ui.is_rect_visible(rect) {
        let c = colors();
        if on {
            ui.painter().rect(
                rect,
                corner(RADIUS_CONTROL),
                c.current_row,
                Stroke::new(HAIRLINE_WIDTH, c.accent),
                StrokeKind::Inside,
            );
        } else if response.hovered() {
            ui.painter()
                .rect_filled(rect, corner(RADIUS_CONTROL), c.hover);
        }
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            icon,
            Type::Heading.font(ui),
            if on || response.hovered() {
                c.text
            } else {
                c.muted
            },
        );
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_text(name)
}

/// An inline link with a real `Link` role, and a target of at least 24 px.
pub fn link(ui: &mut Ui, label: &str) -> Response {
    let c = colors();
    let room = ui.available_width().max(1.0);
    let galley = one_line(ui, label, Type::Body.font(ui), c.accent, room);
    let (rect, response) = ui.allocate_exact_size(
        vec2(galley.size().x, galley.size().y.max(24.0)),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        let y = rect.center().y - galley.size().y / 2.0;
        let underline = y + galley.size().y - 1.0;
        let color = if response.hovered() {
            c.focus
        } else {
            c.accent
        };
        ui.painter().hline(
            rect.x_range(),
            underline,
            Stroke::new(1.0, color.gamma_multiply(0.6)),
        );
        ui.painter().galley(pos2(rect.left(), y), galley, color);
        if response.has_focus() {
            focus_ring(ui, rect);
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

/// A label above a control, with help and an error below it.
pub fn field<R>(
    ui: &mut Ui,
    label: &str,
    help: Option<&str>,
    error: Option<&str>,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    let c = colors();
    ui.spacing_mut().item_spacing.y = XS;
    ui.label(Type::Label.rich(ui, label.to_uppercase()).color(c.muted));
    let inner = add(ui);
    if let Some(help) = help {
        ui.label(Type::Small.rich(ui, help).color(c.muted));
    }
    if let Some(error) = error {
        message(ui, Tone::Error, error);
    }
    ui.add_space(XS);
    inner
}

/// A one-line text field with a label above it.
pub fn text_field(
    ui: &mut Ui,
    label: &str,
    text: &mut String,
    help: Option<&str>,
    error: Option<&str>,
) -> Response {
    field(ui, label, help, error, |ui| {
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

/// A short text field with a target of at least 26 px, and an accessible name.
pub fn text_input(ui: &mut Ui, name: &str, text: &mut String, hint: &str, width: f32) -> Response {
    // The desired width excludes the margins.
    let response = ui.add(
        egui::TextEdit::singleline(text)
            .desired_width(width - 2.0 * S)
            .min_size(vec2(width, 26.0))
            .margin(vec2(S, 4.0))
            .hint_text(Type::Small.rich(ui, hint).color(colors().faint)),
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
    let response = ui.add(
        egui::TextEdit::singleline(text)
            .id(focus_id(name))
            .desired_width(width - 2.0 * S)
            .min_size(vec2(width, height))
            // The caret stands a little clear of the first digit.
            .margin(vec2(
                S + 2.0,
                (height - Type::Numeral.size() - 6.0).max(4.0) / 2.0,
            ))
            .font(Type::Numeral.font(ui))
            // Its first letter stands clear of the caret, which sits at the field's start.
            .hint_text(
                Type::Small
                    .rich(ui, format!("  {hint}"))
                    .color(colors().faint),
            ),
    );
    text.retain(|c| c.is_ascii_digit() || c == '-');
    named(ui, &response, name);
    response
}

/// Search with an icon. The hint is what can be searched.
pub fn search_field(
    ui: &mut Ui,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
) -> Response {
    let c = colors();
    let height = Density::Scan.control();
    let response = ui.add(
        egui::TextEdit::singleline(text)
            .id(focus_id(name))
            .desired_width(width - 36.0)
            .min_size(vec2(width, height))
            .margin(egui::Margin {
                left: 28,
                right: 8,
                top: 6,
                bottom: 6,
            })
            .hint_text(Type::Body.rich(ui, hint).color(c.faint)),
    );
    named(ui, &response, name);
    ui.painter().text(
        pos2(response.rect.left() + S, response.rect.center().y),
        Align2::LEFT_CENTER,
        icons::SEARCH,
        Type::Body.font(ui),
        c.muted,
    );
    response
}

/// One choice from a few, as words with an underline. Returns true when it changed.
pub fn segmented<T: PartialEq + Copy>(ui: &mut Ui, value: &mut T, options: &[(T, &str)]) -> bool {
    let named: Vec<(T, &str, &str)> = options.iter().map(|(o, l)| (*o, *l, *l)).collect();
    segmented_named(ui, value, &named, None, None)
}

/// The same, for a tab with its own accessible name: `(value, painted, accessible name)`.
/// `marked` is the tab that paints the warning mark before its word.
pub fn segmented_named<T: PartialEq + Copy>(
    ui: &mut Ui,
    value: &mut T,
    options: &[(T, &str, &str)],
    marked: Option<T>,
    // A tab whose word changes, and its longest word. The tab keeps that width, so the tabs
    // after it hold their place.
    room_for: Option<(T, &str)>,
) -> bool {
    let mut changed = false;
    let c = colors();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for (option, label, name) in options {
            let selected = *value == *option;
            let font = if selected { Type::Strong } else { Type::Body };
            let color = if selected { c.text } else { c.muted };
            // A tab that needs the DM has a mark in the warning tone, beside its usual word.
            let galley = if marked == Some(*option) {
                let format = |color| egui::TextFormat::simple(font.font(ui), color);
                let mut job = LayoutJob::default();
                job.append(icons::WARN, 0.0, format(c.result.warning));
                job.append(&format!(" {label}"), 0.0, format(color));
                ui.fonts_mut(|fonts| fonts.layout_job(job))
            } else {
                one_line(ui, label, font.font(ui), color, ui.available_width())
            };
            let room = room_for
                .filter(|(tab, _)| tab == option)
                .map_or(0.0, |(_, word)| {
                    one_line(ui, word, font.font(ui), color, f32::INFINITY)
                        .size()
                        .x
                });
            let (rect, response) = ui.allocate_exact_size(
                vec2(galley.size().x.max(room) + 2.0 * S, Density::Read.control()),
                Sense::click(),
            );
            response.widget_info(|| {
                WidgetInfo::selected(
                    WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    *name,
                )
            });
            if ui.is_rect_visible(rect) {
                if response.hovered() {
                    ui.painter()
                        .rect_filled(rect, corner(RADIUS_CONTROL), c.hover);
                }
                ui.painter()
                    .galley(rect.center() - galley.size() / 2.0, galley, color);
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
    any_row(ui, height, selected, name, Sense::click(), paint)
}

/// A row that only says something: it takes the pointer's hover, and no click or Tab stop.
pub fn hover_row(ui: &mut Ui, height: f32, name: &str, paint: impl FnOnce(&Ui, Rect)) -> Response {
    any_row(ui, height, false, name, Sense::hover(), paint)
}

fn any_row(
    ui: &mut Ui,
    height: f32,
    selected: bool,
    name: &str,
    sense: Sense,
    paint: impl FnOnce(&Ui, Rect),
) -> Response {
    let c = colors();
    let (rect, mut response) = ui.allocate_exact_size(vec2(ui.available_width(), height), sense);
    // egui hit-tests the previous frame's rectangles. Paint hover at the current
    // position so it does not trail the pointer while the list scrolls.
    let hovered = response.enabled()
        && ui.rect_contains_pointer(rect)
        && (!ui.ctx().is_using_pointer() || response.is_pointer_button_down_on());
    response.flags.set(egui::response::Flags::HOVERED, hovered);
    response.widget_info(|| {
        let kind = if sense.senses_click() {
            WidgetType::SelectableLabel
        } else {
            WidgetType::Label
        };
        WidgetInfo::selected(kind, ui.is_enabled(), selected, name)
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

/// A state as an icon, a word and a colour. Colour is never the only signal.
#[derive(Clone, Copy)]
pub struct Status<'a> {
    pub icon: &'a str,
    pub word: &'a str,
    pub color: Color32,
}

pub fn status(ui: &mut Ui, state: Status<'_>) -> Response {
    let mut job = LayoutJob::default();
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: state.color,
        ..Default::default()
    };
    job.append(state.icon, 0.0, format.clone());
    job.append(" ", 0.0, format.clone());
    job.append(state.word, 0.0, format);
    ui.add(egui::Label::new(job).truncate())
}

/// A status that also leads to where the DM can act on it. The line wraps, because the
/// words are a warning and must not be cut.
pub fn status_link(ui: &mut Ui, state: Status<'_>) -> Response {
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: state.color,
        ..Default::default()
    };
    let mut job = LayoutJob::default();
    job.append(state.icon, 0.0, format.clone());
    job.append(" ", 0.0, format.clone());
    job.append(
        state.word,
        0.0,
        egui::TextFormat {
            underline: egui::Stroke::new(1.0, state.color),
            ..format
        },
    );
    job.wrap.max_width = ui.available_width().max(1.0);
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    let (rect, response) = ui.allocate_exact_size(
        vec2(galley.size().x, galley.size().y.max(24.0)),
        Sense::click(),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Link, ui.is_enabled(), state.word));
    if ui.is_rect_visible(rect) {
        let top = rect.center().y - galley.size().y / 2.0;
        ui.painter()
            .galley(pos2(rect.left(), top), galley, state.color);
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A label over a large value, with no box. The label stays on one line.
pub fn stat_tile(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    let c = colors();
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.add(
            egui::Label::new(Type::Label.rich(ui, label.to_uppercase()).color(c.muted)).truncate(),
        );
        ui.label(Type::NumeralLarge.rich(ui, value).color(color));
    });
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
        format!("/{max_text}"),
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

/// A modifier, with a true minus sign: `+3`, `−1`.
pub fn signed(value: i64) -> String {
    if value < 0 {
        format!("\u{2212}{}", value.unsigned_abs())
    } else {
        format!("+{value}")
    }
}

/// A count and the right word for it: `1 entry`, `2 entries`.
pub fn count(n: usize, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

/// What this place is, and the one next step.
pub fn empty_state(ui: &mut Ui, icon: &str, title: &str, body: &str, action: impl FnOnce(&mut Ui)) {
    let c = colors();
    ui.add_space(XXL);
    ui.vertical(|ui| {
        ui.set_max_width(ui.available_width().min(FORM_WIDTH));
        ui.spacing_mut().item_spacing.y = S;
        ui.label(Type::Title.rich(ui, icon).color(c.muted));
        ui.label(Type::Title.rich(ui, title).color(c.text));
        ui.label(Type::Body.rich(ui, body).color(c.muted));
        ui.add_space(XS);
        action(ui);
    });
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tone {
    Info,
    Success,
    Warning,
    Error,
}

impl Tone {
    fn parts(self) -> (&'static str, Color32) {
        let result = colors().result;
        match self {
            Self::Info => (icons::INFO, result.info),
            Self::Success => (icons::SUCCESS, result.success),
            Self::Warning => (icons::WARN, result.warning),
            Self::Error => (icons::ERROR, result.error),
        }
    }
}

/// An inline message: a bar, an icon and words. It never blocks anything.
pub fn message(ui: &mut Ui, tone: Tone, text: &str) -> Response {
    let (icon, color) = tone.parts();
    let c = colors();
    let inner = ui.horizontal_top(|ui| {
        let start = ui.cursor().min;
        ui.add_space(S + 1.0);
        ui.label(Type::Body.rich(ui, icon).color(color));
        ui.add_space(-XS);
        let response = ui.add(egui::Label::new(Type::Body.rich(ui, text).color(c.text)).wrap());
        let bar = Rect::from_min_max(start, pos2(start.x + 3.0, response.rect.bottom()));
        ui.painter().rect_filled(bar, 0.0, color);
        response
    });
    inner.inner
}

/// A message in the run's small form: a bar, an icon and small words. It shares the room
/// under the roster, where the normal form would cost a row its place.
pub fn compact_message(ui: &mut Ui, tone: Tone, text: &str) -> Response {
    let (icon, color) = tone.parts();
    let c = colors();
    ui.horizontal(|ui| {
        let start = ui.cursor().min;
        ui.add_space(S + 1.0);
        ui.label(Type::Small.rich(ui, icon).color(color));
        ui.add_space(-XS);
        let response = ui.add(egui::Label::new(Type::Small.rich(ui, text).color(c.text)).wrap());
        let bar = Rect::from_min_max(
            pos2(start.x, response.rect.top()),
            pos2(start.x + 3.0, response.rect.bottom()),
        );
        ui.painter().rect_filled(bar, 0.0, color);
        response
    })
    .inner
}

/// Work in progress. Test builds draw a still mark: the moving one is drawn from the clock,
/// so two captures of the same state would differ.
pub fn spinner(ui: &mut Ui) {
    #[cfg(test)]
    ui.label(Type::Body.rich(ui, icons::BUSY).color(colors().muted));
    #[cfg(not(test))]
    ui.spinner();
}

/// `Draft discarded.` beside its undo. Returns true when the undo is pressed.
pub fn draft_discarded(ui: &mut Ui) -> bool {
    let mut pressed = false;
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
            message(ui, Tone::Info, "Draft discarded.");
        });
        pressed = Action::secondary("Undo discard")
            .density(Density::Read)
            .show(ui)
            .clicked();
    });
    pressed
}

/// A message with one action beside it. Returns true when the action is used.
pub fn message_with_action(ui: &mut Ui, tone: Tone, text: &str, action: &str) -> bool {
    let room = number_width(ui, action, &Type::Strong.font(ui)) + 3.0 * S;
    let mut pressed = false;
    ui.horizontal_top(|ui| {
        let width = (ui.available_width() - room).max(0.0);
        ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
            // Level with the action, on the message's first line.
            ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
            message(ui, tone, text)
        });
        pressed = Action::quiet(action)
            .density(Density::Read)
            .show(ui)
            .clicked();
    });
    pressed
}

/// A focused dialog. Only the brief's three modal tasks use this. Escape is the caller's.
pub fn dialog<R>(ctx: &egui::Context, id: &str, title: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
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
            ui.set_max_width((viewport.width() - XXL * 2.0).min(FORM_WIDTH));
            ui.set_max_height(viewport.height() - XXL * 2.0);
            ui.label(Type::Section.rich(ui, title).color(c.text));
            ui.add_space(XS);
            egui::ScrollArea::vertical()
                .id_salt(("dialog_body", id))
                .auto_shrink([false, true])
                .show(ui, add)
                .inner
        })
        .inner
}

/// The width of a panel beside a page: Lookup, or the run's side panel.
pub fn side_panel_width(available: f32) -> f32 {
    (available * 0.36).clamp(PANEL_MIN_WIDTH, PANEL_MAX_WIDTH)
}

/// A panel's title row with its close control.
pub fn panel_header(ui: &mut Ui, title: &str, close: &str) -> bool {
    let mut closed = false;
    // The control first, and the title in what is left. A long title wraps.
    ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
        closed = icon_button(ui, icons::CLOSE, close).clicked();
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.add(egui::Label::new(Type::Heading.rich(ui, title).color(colors().text)).wrap());
        });
    });
    closed
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

/// A collapsing section with the icon set's arrow.
pub fn section<R>(
    ui: &mut Ui,
    title: impl Into<egui::WidgetText>,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    egui::CollapsingHeader::new(title)
        .icon(section_icon)
        .show(ui, add)
}

/// A collapsing section whose title changes while it is open, as a count or a state word
/// does. egui makes a section's id from its title, so without a key a new word closes it.
pub fn keyed_section<R>(
    ui: &mut Ui,
    key: &str,
    title: impl Into<egui::WidgetText>,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    egui::CollapsingHeader::new(title)
        .id_salt(key)
        .icon(section_icon)
        .show(ui, add)
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
    ui.add_space(2.0);
    rule(ui);
    ui.add_space(2.0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_keeps_its_two_ends_and_fits() {
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let font = Type::Small.font(ui);
                let path = "/home/dm/campaigns/the-emberfall-marches/quiet-harbor";
                assert_eq!(cut_middle(ui, path, &font, 2000.0), path);
                let cut = cut_middle(ui, path, &font, 120.0);
                assert!(cut.contains('…'), "{cut}");
                assert!(cut.starts_with('/') && cut.ends_with("harbor"), "{cut}");
                assert!(number_width(ui, &cut, &font) <= 120.0, "{cut}");
                // Not a letter is split, whatever the text.
                let cut = cut_middle(ui, "ééééééééééééééééééééééééééééééééééééééééé", &font, 60.0);
                assert!(cut.contains('…'), "{cut}");
            });
        });
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
