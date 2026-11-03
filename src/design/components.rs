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

/// A page title and one context line on the same baseline. Actions keep their room.
/// Actions are drawn right to left: put the bordered New action first in the closure.
pub fn page_header(ui: &mut Ui, title: &str, context: Option<&str>, actions: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), Density::Scan.control()),
        Layout::right_to_left(Align::Center),
        |ui| {
            ui.set_min_height(Density::Scan.control());
            actions(ui);
            ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
                ui.label(Type::Title.rich(ui, title).color(colors().text));
                if let Some(context) = context {
                    ui.add_space(XS);
                    ui.add(
                        egui::Label::new(Type::Small.rich(ui, context).color(colors().muted))
                            .truncate(),
                    )
                    .on_hover_text(context);
                }
            });
        },
    );
    rule(ui);
    ui.add_space(L);
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

/// Run state uses the full second line, so ended and round words fit in the rail.
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
        ui.painter().text(
            pos2(rect.left() + S, rect.top() + height * 0.56),
            Align2::LEFT_TOP,
            detail,
            Type::Small.font(ui),
            c.muted,
        );
        if response.has_focus() {
            focus_ring(ui, rect);
        }
    }
    response
}

/// The quiet menu at the right edge of a list row.
pub fn row_menu(ui: &mut Ui, name: &str, add: impl FnOnce(&mut Ui)) {
    let response = icon_button(ui, icons::EXPAND, &format!("More actions for {name}"));
    egui::Popup::menu(&response).show(add);
}

/// The visible reason beside a blocked control. A tooltip is only a second route.
pub fn blocked_reason(ui: &mut Ui, reason: Option<&str>) {
    if let Some(reason) = reason {
        ui.add(egui::Label::new(Type::Small.rich(ui, reason).color(colors().muted)).wrap());
    }
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
        if self.kind == Kind::Quiet { S } else { M }
    }

    /// The width before any text is cut, using the same parts as the painted button.
    pub fn natural_width(&self, ui: &Ui) -> f32 {
        let icon = self.icon.map_or(0.0, |_| Type::Body.size() + XS + 2.0);
        let key = self
            .key
            .map_or(0.0, |key| number_width(ui, key, &Type::Small.font(ui)) + S);
        (number_width(ui, self.label, &self.font().font(ui)) + 2.0 * self.padding() + icon + key)
            .max(self.height)
    }

    fn paint(&self, ui: &mut Ui) -> Response {
        let c = colors();
        let font = self.font().font(ui);
        let padding = self.padding();
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

/// An action with a default, and a caret beside it that opens a menu of the other ways
/// to do it. A click in the menu closes it. The response is the action's own.
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
    number_field_with_id(ui, focus_id(name), name, text, hint, width, height)
}

/// For a field whose name another field can share, as two things with one name do.
pub fn number_field_with_id(
    ui: &mut Ui,
    id: Id,
    name: &str,
    text: &mut String,
    hint: &str,
    width: f32,
    height: f32,
) -> Response {
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
            .hint_text(Type::Small.rich(ui, hint).color(colors().faint)),
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
    let mut changed = false;
    let c = colors();
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for (option, label) in options {
            let selected = *value == *option;
            let font = if selected { Type::Strong } else { Type::Body };
            let color = if selected { c.text } else { c.muted };
            let galley = one_line(ui, label, font.font(ui), color, ui.available_width());
            let (rect, response) = ui.allocate_exact_size(
                vec2(galley.size().x + 2.0 * S, Density::Read.control()),
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
    let c = colors();
    let (rect, mut response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
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

/// A state as an icon, a word and a colour. Colour is never the only signal.
#[derive(Clone, Copy)]
pub struct Status<'a> {
    pub icon: &'a str,
    pub word: &'a str,
    pub color: Color32,
}

pub fn status(ui: &mut Ui, state: Status<'_>) -> Response {
    status_label(ui, state, false)
}

/// Rail notices wrap so their cause stays visible in a narrow column.
pub fn wrapped_status(ui: &mut Ui, state: Status<'_>) -> Response {
    status_label(ui, state, true)
}

fn status_label(ui: &mut Ui, state: Status<'_>, wrap: bool) -> Response {
    let mut job = LayoutJob::default();
    let format = egui::TextFormat {
        font_id: Type::Small.font(ui),
        color: state.color,
        ..Default::default()
    };
    job.append(state.icon, 0.0, format.clone());
    job.append(" ", 0.0, format.clone());
    job.append(state.word, 0.0, format);
    let label = egui::Label::new(job);
    ui.add(if wrap { label.wrap() } else { label.truncate() })
        .on_hover_text(state.word)
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
pub fn stat_tile(ui: &mut Ui, label: &str, value: &str) {
    let c = colors();
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.add(
            egui::Label::new(Type::Label.rich(ui, label.to_uppercase()).color(c.muted)).truncate(),
        );
        ui.label(Type::NumeralLarge.rich(ui, value).color(c.text));
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
    if n == 0 {
        return String::new();
    }
    let digits = n.to_string();
    let mut grouped = String::new();
    for (i, digit) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(digit);
    }
    let unit = if n == 1 { one } else { many };
    if unit.is_empty() {
        grouped
    } else {
        format!("{grouped} {unit}")
    }
}

/// What this place is, and the one next step.
pub fn empty_state(ui: &mut Ui, icon: &str, title: &str, body: &str, action: impl FnOnce(&mut Ui)) {
    let c = colors();
    ui.vertical(|ui| {
        ui.set_max_width(ui.available_width().min(READER_MEASURE));
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

/// A message whose marked fragments use the code font.
pub fn message_with_code(ui: &mut Ui, tone: Tone, text: &str) -> Response {
    let (icon, color) = tone.parts();
    let mut job = LayoutJob::default();
    for (i, part) in text.split('`').enumerate() {
        job.append(
            part,
            0.0,
            egui::TextFormat {
                font_id: if i % 2 == 0 { Type::Body } else { Type::Mono }.font(ui),
                color: colors().text,
                ..Default::default()
            },
        );
    }
    ui.horizontal_top(|ui| {
        let start = ui.cursor().min;
        ui.add_space(S + 1.0);
        ui.label(Type::Body.rich(ui, icon).color(color));
        ui.add_space(-XS);
        let response = ui.add(egui::Label::new(job).wrap());
        let bar = Rect::from_min_max(start, pos2(start.x + 3.0, response.rect.bottom()));
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
    let button = Action::quiet(action).density(Density::Read);
    let button_width = button.natural_width(ui);
    let icon_width = number_width(ui, tone.parts().0, &Type::Body.font(ui));
    let inset = S + 1.0 + icon_width + ui.spacing().item_spacing.x * 2.0 - XS;
    let text_width = number_width(ui, text, &Type::Body.font(ui)) + inset;
    let gap = ui.spacing().item_spacing.x;
    let mut pressed = false;
    if ui.available_width() < text_width + gap + button_width {
        message(ui, tone, text);
        ui.horizontal(|ui| {
            ui.add_space((inset - gap).max(0.0));
            pressed = button.show(ui).clicked();
        });
    } else {
        ui.horizontal_top(|ui| {
            let width = (ui.available_width() - button_width - gap).max(0.0);
            ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
                message(ui, tone, text)
            });
            pressed = button.show(ui).clicked();
        });
    }
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

/// An image at full size, within the window. Returns true when the DM closes it, by a click
/// or Escape.
pub fn image_viewer(ctx: &egui::Context, name: &str, uri: &str) -> bool {
    let viewport = ctx.content_rect();
    let response = egui::Modal::new(Id::new("image_viewer"))
        .backdrop_color(Color32::from_black_alpha(200))
        .frame(egui::Frame::new())
        .show(ctx, |ui| {
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
            let image = egui::Image::from_uri(uri)
                .fit_to_original_size(1.0)
                .max_size(
                    viewport.size()
                        - vec2(
                            XXL * 2.0,
                            XXL * 2.0 + Density::Scan.control() + Type::Small.line() + S,
                        ),
                )
                .sense(Sense::click());
            let response = ui.add(image).on_hover_text(name);
            response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, true, name));
            ui.label(Type::Small.rich(ui, name).color(colors().text));
            close || response.clicked()
        });
    response.should_close() || response.inner
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

/// A collapsing section that stays open when its title changes, such as a count.
pub fn section_with_id<R>(
    ui: &mut Ui,
    id: impl std::hash::Hash,
    title: impl Into<egui::WidgetText>,
    add: impl FnOnce(&mut Ui) -> R,
) -> egui::CollapsingResponse<R> {
    egui::CollapsingHeader::new(title)
        .id_salt(id)
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

/// The shared Add strip. Callers supply quiet, worded actions in their screen's order.
pub fn add_strip(ui: &mut Ui, actions: impl FnOnce(&mut Ui)) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = XS;
        ui.label(Type::Label.rich(ui, "Add").color(colors().muted));
        actions(ui);
    });
}

/// Sheet tabs keep one line. Less frequent pages move to a named menu when needed.
pub fn overflow_tabs(ui: &mut Ui, selected: &mut usize, choices: &[(usize, &str)]) {
    let width = |label: &str| number_width(ui, label, &Type::Strong.font(ui)) + L;
    let all: f32 = choices.iter().map(|(_, label)| width(label)).sum();
    if all <= ui.available_width() {
        segmented(ui, selected, choices);
        return;
    }
    let mut used = width("More") + L;
    let mut count = 0;
    for (_, label) in choices {
        if used + width(label) > ui.available_width() {
            break;
        }
        used += width(label);
        count += 1;
    }
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        segmented(ui, selected, &choices[..count]);
        let menu = ui.menu_button(format!("More {}", icons::DOWN), |ui| {
            for (value, label) in &choices[count..] {
                if ui.selectable_label(*selected == *value, *label).clicked() {
                    *selected = *value;
                    ui.close();
                }
            }
        });
        named(ui, &menu.response, "More sheet pages");
        if choices[count..]
            .iter()
            .any(|(value, _)| *value == *selected)
        {
            ui.painter().hline(
                menu.response.rect.x_range(),
                menu.response.rect.bottom(),
                Stroke::new(HAIRLINE_WIDTH, colors().accent),
            );
        }
    });
}

/// Second-level sheet pages keep one Small tab line, with overflow in More.
pub fn overflow_tabs_small(ui: &mut Ui, selected: &mut usize, choices: &[(usize, &str)]) {
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
pub fn draft_comparison(ui: &mut Ui, draft: &str, disk: &str, id: impl std::hash::Hash) {
    let left: Vec<_> = draft.lines().collect();
    let right: Vec<_> = disk.lines().collect();
    let differences: Vec<_> = (0..left.len().max(right.len()))
        .filter(|&line| left.get(line) != right.get(line))
        .collect();
    let state = ui.make_persistent_id(("draft_difference", id));
    let first = differences.first().copied().unwrap_or(0);
    let current = ui.data(|data| data.get_temp::<usize>(state));
    let mut target = current.is_none().then_some(first);
    ui.horizontal(|ui| {
        let words = if differences.is_empty() {
            "No lines differ".into()
        } else {
            super::count(differences.len(), "line differs", "lines differ")
        };
        ui.label(Type::Small.rich(ui, words).color(colors().muted));
        if !differences.is_empty() && Action::quiet("Next difference").show(ui).clicked() {
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
        group_label(&mut columns[0], "Your draft");
        group_label(&mut columns[1], "In the file now");
    });
    let height = ui.available_height();
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
                            lines.get(line).copied().unwrap_or("No line").to_owned(),
                            Type::Body.font(ui),
                            colors().text,
                            text_width,
                        )
                    })
                    .size()
                    .y
                    .max(Type::Body.line())
                })
                .fold(0.0, f32::max)
                + ui.spacing().item_spacing.y
        })
        .collect();
    let mut scroll = egui::ScrollArea::vertical().id_salt(state);
    if let Some(line) = target {
        scroll = scroll.vertical_scroll_offset(heights.iter().take(line.saturating_sub(2)).sum());
    }
    scroll.auto_shrink([false, false]).show(ui, |ui| {
        for line in 0..left.len().max(right.len()) {
            let differs = left.get(line) != right.get(line);
            ui.columns(2, |columns| {
                for (at, lines) in [&left, &right].into_iter().enumerate() {
                    egui::Frame::new()
                        .fill(if differs {
                            colors().selected
                        } else {
                            colors().page
                        })
                        .show(&mut columns[at], |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.horizontal_top(|ui| {
                                let (mark, _) = ui.allocate_exact_size(
                                    egui::vec2(XS, Type::Body.line()),
                                    egui::Sense::hover(),
                                );
                                if differs {
                                    ui.painter().vline(
                                        mark.center().x,
                                        mark.y_range(),
                                        egui::Stroke::new(HAIRLINE_WIDTH, colors().accent),
                                    );
                                }
                                ui.add(
                                    egui::Label::new(
                                        Type::Body
                                            .rich(ui, lines.get(line).copied().unwrap_or("No line"))
                                            .color(if lines.get(line).is_some() {
                                                colors().text
                                            } else {
                                                colors().muted
                                            }),
                                    )
                                    .wrap(),
                                );
                            });
                        });
                }
            });
        }
        if heights.iter().sum::<f32>() > height {
            ui.add_space((height - Type::Body.line() * 3.0).max(0.0));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_draft_difference_is_visible_on_the_first_frame() {
        for prefix in [0, 3, 50] {
            let context = egui::Context::default();
            super::super::theme::install(&context);
            let context_lines = (0..prefix)
                .map(|n| format!("Unchanged line {n}\n"))
                .collect::<String>();
            let draft = format!("{context_lines}My changed draft line");
            let disk = format!("{context_lines}Changed elsewhere.\nExtra disk line");
            let output = context.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        vec2(600.0, 300.0),
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
                        &format!("Unchanged line {}", prefix - 2)
                    )),
                    "two context lines stay in view"
                );
            }
            if prefix == 0 {
                assert!(
                    output.shapes.iter().any(|shape| visible(
                        &shape.shape,
                        shape.clip_rect,
                        "No line"
                    )),
                    "missing lines need words"
                );
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
    fn a_wrapped_message_keeps_its_whole_action() {
        for width in [240.0, 360.0, 440.0, 880.0] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 240.0))
                .build_ui(|ui| {
                    message_with_action(ui, Tone::Info,
                        "The party has full HP after this long rest. Every changed value is kept for undo.",
                        "Undo delete saved run");
                });
            super::super::theme::install(&harness.ctx);
            harness.run_steps(2);
            let cut = crate::visual_tests::ergonomics::cut_text(
                &harness.output().shapes,
                &[],
                &[],
                "message",
                "wrapped",
                "test",
            );
            assert!(cut.is_empty(), "width {width}: {cut:?}");
        }
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
