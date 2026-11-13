//! Shared UI parts. Screens compose these instead of styling widgets themselves.
use super::{comparison::Comparison, icons, status::Tinted, tokens::*};
use eframe::egui::{
    self, Align, Align2, Color32, CornerRadius, FontId, Id, Layout, Rect, Response, Sense, Stroke,
    StrokeKind, Ui, Vec2, WidgetInfo, WidgetType, pos2, text::LayoutJob, vec2,
};
use std::sync::Arc;

fn corner(value: f32) -> CornerRadius {
    CornerRadius::same(value as u8)
}

/// A quiet menu button with its caret after the word.
pub fn menu_button(ui: &mut Ui, word: &str, selected: bool) -> Response {
    let response = Action::quiet(&format!("{word}    "))
        .name(word)
        .selected(selected)
        .show(ui);
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

/// Temporary HP, with its mark, in every place that shows it.
pub fn temp_hp_text(temp: i64) -> String {
    format!("{} +{temp}", icons::TEMP_HP)
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
    ui.add_space(L);
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
            ui.add_space(XS);
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
    line_start: bool,
    /// A `line_start` Undo whose target stays in its message's column: the row's left edge.
    column: bool,
    min_width: Option<f32>,
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
            line_start: false,
            column: false,
            min_width: None,
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
        if self.kind == Kind::Quiet { S } else { M }
    }

    fn icon_width(&self) -> f32 {
        self.icon.map_or(0.0, |_| ICON_SLOT)
    }

    /// The width before any text is cut, using the same parts as the painted button.
    pub fn natural_width(&self, ui: &Ui) -> f32 {
        let icon = self.icon_width();
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
        let width = (galley.size().x + 2.0 * padding + icon_width + key_width)
            .max(self.height)
            .max(self.min_width.unwrap_or_default());
        let (rect, response) = if self.kind == Kind::Quiet && self.line_start {
            // The words start at the text edge; the padding and any icon hang in the gutter. The
            // row allocates all but the icon, so the next item keeps the same gap after the
            // target as it has after a `line_start` action without an icon.
            let size = vec2(width - icon_width, self.height);
            let (id, allocated) = ui.allocate_space(size);
            let rect = ui
                .layout()
                .align_size_within_rect(size, allocated)
                .translate(vec2(-padding, 0.0));
            let rect = rect.with_min_x(rect.min.x - icon_width);
            // An Undo under its message keeps its target in the message's column, so it never
            // hangs past a pane whose text edge is the message's (Lookup's reader). The icon
            // and the words keep their places.
            let target = if self.column {
                rect.with_min_x(rect.min.x.max(ui.max_rect().left()))
            } else {
                rect
            };
            (rect, ui.interact(target, id, Sense::click()))
        } else {
            ui.allocate_exact_size(vec2(width, self.height), Sense::click())
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
    let (rect, response) =
        ui.allocate_exact_size(Vec2::splat(Density::Read.control()), Sense::click());
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
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn stepper_with_density(
    ui: &mut Ui,
    name: &str,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
    density: Density,
    low_reason: Option<&str>,
    high_reason: Option<&str>,
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
                ui.add(egui::DragValue::new(value).range(low..=high).speed(0.1))
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

fn text_link(ui: &mut Ui, label: &str, role: Type) -> Response {
    let c = colors();
    let room = ui.available_width().max(1.0);
    let galley = one_line(ui, label, role.font(ui), c.accent, room);
    // The press target is the link's own space in the layout, at least `XL` wide. A short
    // word such as `Ox` sits at its left, and the next item starts after the whole target,
    // so two short links side by side never share a press area.
    let (target, response) = ui.allocate_exact_size(
        vec2(galley.size().x.max(XL), galley.size().y.max(XL)),
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
    field_label(ui, label, Type::Small, help, error, add)
}

/// A form field with the same sentence-case label as `field`.
pub fn small_field<R>(
    ui: &mut Ui,
    label: &str,
    help: Option<&str>,
    error: Option<&str>,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    field_label(ui, label, Type::Small, help, error, add)
}

fn field_label<R>(
    ui: &mut Ui,
    label: &str,
    role: Type,
    help: Option<&str>,
    error: Option<&str>,
    add: impl FnOnce(&mut Ui) -> R,
) -> R {
    field_caption(ui, label, role);
    let inner = add(ui);
    field_feedback(ui, help, error);
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

fn field_feedback(ui: &mut Ui, help: Option<&str>, error: Option<&str>) {
    if let Some(text) = help {
        self::help(ui, text);
    }
    if let Some(error) = error {
        tone_message(ui, Tone::Error, error);
    }
    ui.add_space(XS);
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

pub fn small_text_field(
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
    field_label(ui, label, role, help, error, |ui| {
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
    text.retain(|c| c.is_ascii_digit() || c == '-');
    named(ui, &response, name);
    response
}

/// Search with an icon. The hint is what can be searched.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
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
    /// Enter, as the field let go. With nothing `chosen`, the caller adds the typed words.
    pub enter: bool,
}

/// Add typed words to a list: one field with no button. Enter adds. A suggestion adds by an arrow
/// key and Enter, or a click. Escape clears the highlight, then the words. `beside` draws a
/// control right of the field, `beside_width` wide, and can choose a value too. `suggest` lists
/// the suggestions for the typed words, which show only while there are words.
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
pub fn add_field<T>(
    ui: &mut Ui,
    state: &mut AddField,
    name: &str,
    hint: &str,
    beside_width: f32,
    beside: impl FnOnce(&mut Ui) -> Option<T>,
    suggest: impl FnOnce(&str) -> Vec<Suggestions<T>>,
) -> AddFieldResult<T> {
    let mut chosen = None;
    let field = ui
        .horizontal(|ui| {
            let width = (ui.available_width() - beside_width).max(160.0);
            let field = text_input(ui, name, &mut state.words, hint, width);
            chosen = beside(ui);
            field
        })
        .inner;
    if std::mem::take(&mut state.focus) {
        field.request_focus();
    }
    if field.changed() {
        state.highlight = None;
    }
    let groups = if state.words.trim().is_empty() {
        Vec::new()
    } else {
        suggest(&state.words)
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
    let enter = field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
    // Escape closes the top layer first: the chosen suggestion, then the typed words.
    if ui.input(|input| input.key_pressed(egui::Key::Escape))
        && (field.has_focus() || field.lost_focus())
        && state.highlight.take().is_none()
    {
        state.words.clear();
    }
    // While there are typed words, so that a click on a suggestion lands after the field
    // lets go of the keyboard.
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
    AddFieldResult {
        field,
        chosen,
        enter,
    }
}

/// A state as an icon, a word and a colour. Colour is never the only signal. A status whose
/// icon and word differ in colour is a `KindStatus` (`design::status`).
#[derive(Clone, Copy)]
pub struct Status<'a> {
    pub icon: &'a str,
    pub word: &'a str,
    pub color: Color32,
}

pub fn status<'a>(ui: &mut Ui, state: impl Into<Tinted<'a>>) -> Response {
    status_label(ui, state.into())
}

/// A painted row keeps its status word in the shared status colour.
pub fn status_in_text(
    ui: &Ui,
    text: &str,
    status: Option<(&str, Color32)>,
    width: f32,
) -> Arc<egui::Galley> {
    let mut job = LayoutJob::default();
    job.wrap.max_width = width;
    let (before, word, after, color) = status
        .and_then(|(word, color)| {
            text.split_once(word)
                .map(|(before, after)| (before, word, after, color))
        })
        .unwrap_or((text, "", "", colors().muted));
    for (text, color) in [
        (before, colors().muted),
        (word, color),
        (after, colors().muted),
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
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

/// Rail notices wrap so their cause stays visible in a narrow column. A second line starts at
/// the word, not under the icon, as in every other status that wraps.
pub fn wrapped_status<'a>(ui: &mut Ui, state: impl Into<Tinted<'a>>) -> Response {
    let state = state.into();
    if status_fits(ui, state) {
        return status_label(ui, state);
    }
    let status = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        status_icon_column(ui, state);
        ui.add(egui::Label::new(Type::Small.rich(ui, state.word).color(state.word_color)).wrap())
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
    ui.add(egui::Label::new(job).truncate())
        .on_hover_text(state.word)
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
    let c = colors();
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        ui.add(egui::Label::new(Type::Small.rich(ui, label).color(c.muted)).truncate());
        ui.label(Type::NumeralLarge.rich(ui, value).color(c.text));
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
        ui.add_space(XS);
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
        ui.add_space(S + 1.0 + number_width(ui, tone.parts().0, &Type::Body.font(ui)) + XS);
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
        ui.add_space(S + 1.0);
        ui.label(Type::Body.rich(ui, icon).color(color));
        ui.add_space(XS);
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
        ui.add_space(S + HAIRLINE_WIDTH);
        ui.label(Type::Small.rich(ui, icon).color(color));
        ui.add_space(-XS);
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

/// A message whose marked fragments use the code font.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
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
    #[cfg(test)]
    let turn = 0.0;
    #[cfg(not(test))]
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

/// Names the discarded draft in an Undo message. Undo's accessible name also names the draft.
/// Returns true when undo is pressed.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn draft_discarded(ui: &mut Ui, name: &str) -> bool {
    undo_message_capped(
        ui,
        Tone::Info,
        &format!("{name} draft discarded."),
        "Undo discard",
        Some(&format!("Undo discard of {name}")),
    )
}

/// A message with one action beside it, capped at `LIST_MEASURE` like every notice. Returns true
/// when the action is used.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn message_with_action(ui: &mut Ui, tone: Tone, text: &str, action: &str) -> bool {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        message_with_action_inside(ui, tone, text, action)
    })
    .inner
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
            ui.add_space((inset - spacing).max(0.0));
            pressed = button.show(ui).clicked();
        });
    } else {
        ui.horizontal_top(|ui| {
            let width = (ui.available_width() - button_width - L).max(0.0);
            ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
                tone_message(ui, tone, text)
            });
            ui.add_space((L - spacing).max(0.0));
            pressed = button.show(ui).clicked();
        });
    }
    pressed
}

/// A sentence of entry links, with explicit word spaces around its conjunctions.
pub fn linked_sentence(ui: &mut Ui, names: &[&str], suffix: &str) -> Option<usize> {
    let mut clicked = None;
    let space = number_width(ui, " ", &Type::Body.font(ui));
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.horizontal_wrapped(|ui| {
            for (at, name) in names.iter().enumerate() {
                if at > 0 {
                    ui.add_space(space);
                    ui.add(
                        egui::Label::new(Type::Body.rich(ui, "and"))
                            .wrap_mode(egui::TextWrapMode::Extend),
                    );
                    ui.add_space(space);
                }
                if link(ui, name).clicked() {
                    clicked = Some(at);
                }
            }
            if !names.is_empty() {
                ui.add_space(space);
            }
            ui.label(Type::Body.rich(ui, suffix));
        });
    });
    clicked
}

/// A message whose first words are entry links, such as the encounters a package removal leaves
/// without creatures. Capped at `LIST_MEASURE` like every notice. Round 27 FE5: a result's Undo
/// stays on its own success sentence (`undo_message`), and this message follows it, so it has no
/// action of its own. The pressed link's index.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn message_with_links(ui: &mut Ui, tone: Tone, names: &[&str], suffix: &str) -> Option<usize> {
    let (icon, color) = tone.parts();
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        ui.horizontal_top(|ui| {
            let start = ui.cursor().min;
            ui.add_space(S + 1.0);
            // The sentence starts with a link, so its first line is a link's height. The icon
            // centres on that line, as `message` centres it on its words.
            let first_line = XL.max(Type::Body.line());
            ui.allocate_ui_with_layout(
                vec2(number_width(ui, icon, &Type::Body.font(ui)), first_line),
                Layout::left_to_right(Align::Center),
                |ui| ui.label(Type::Body.rich(ui, icon).color(color)),
            );
            ui.add_space(-XS);
            let content = ui.vertical(|ui| linked_sentence(ui, names, suffix));
            let bar =
                Rect::from_min_max(start, pos2(start.x + 3.0, content.response.rect.bottom()));
            ui.painter().rect_filled(bar, 0.0, color);
            content.inner
        })
        .inner
    })
    .inner
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
            ui.add_space(inset);
            ui.vertical(actions).inner
        })
        .inner
    })
    .inner
}

/// Where test builds keep the labels that `row_detail` marked in the last pass.
#[cfg(test)]
pub const ROW_DETAILS: &str = "design_row_details";
/// Where test builds keep the rects that `register_row` registered in the last pass.
#[cfg(test)]
pub const ROW_RECTS: &str = "design_row_rects";

/// Keep `item` under `key` for this pass only: a new pass starts a new list.
#[cfg(test)]
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

/// A row component registers its whole rect, so that the gate's `help-measure` check can tell a
/// row's own detail line from help. Only test builds keep it.
pub fn register_row(ui: &Ui, rect: Rect) {
    #[cfg(test)]
    keep_for_pass(ui, ROW_RECTS, rect);
    #[cfg(not(test))]
    let _ = (ui, rect);
}

/// Marks a label as its row's own detail line (its second line), not help. The gate's
/// `help-measure` check passes it whatever its width, but only while it lies inside a row that
/// `register_row` registered: a mark on a label outside any row changes nothing. Call it on the
/// label's response. Only test builds keep the mark.
pub fn row_detail(ui: &Ui, response: &Response) {
    #[cfg(test)]
    keep_for_pass(ui, ROW_DETAILS, response.id);
    #[cfg(not(test))]
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
///     .result(&words, true, |ui| design::draft_discarded(ui, name))
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
    scroll_part(ui, rect, Id::new(id), name, add).0
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
fn scroll_part<R>(
    ui: &mut Ui,
    rect: Rect,
    id: Id,
    name: &str,
    add: impl FnOnce(&mut Ui) -> R,
) -> (R, f32) {
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
                    .map(|response| response.rect)
                    .filter(|rect| ui.min_rect().contains_rect(*rect))
                {
                    ui.scroll_to_rect(rect, None);
                }
            }
            out
        });
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
    (output.inner, output.content_size.y)
}

/// Where a footer message stands in the footer's one order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(super) enum FooterRank {
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
/// so the cut line reads as a list that goes on. The footer is absent when it holds nothing, as
/// tall as what it holds up to one third
/// of the pane, and scrolls inside beyond that (see `scroll_part`). Its order is fixed: results
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
/// ```ignore
/// let mut undo = false;
/// let mut footer = design::Footer::new(("library_footer", path), width);
/// // `discarded_order` was issued with `ResultOrder::issue()` when the draft was discarded.
/// footer.result(discarded_order, &words, true, |ui| undo = design::draft_discarded(ui, name));
/// footer.reason(problem, |ui| { design::message(ui, Tone::Error, problem); });
/// footer.pane(ui, design::Body::Content, |ui| {
///     // ...the body...
/// });
/// ```
pub struct Footer<'a> {
    id: Id,
    width: f32,
    messages: Vec<FooterMessage<'a>>,
}

impl<'a> Footer<'a> {
    /// An empty footer `width` wide. `id` keeps its height, scroll and focus; give each pane
    /// instance its own.
    pub fn new(id: impl std::hash::Hash, width: f32) -> Self {
        Self {
            id: Id::new(id),
            width,
            messages: Vec::new(),
        }
    }

    /// A result, made by the action done at `order`. `has_undo` is true while its Undo is live:
    /// it then leads the footer, so a reason that does not clear never hides the way back.
    /// Without one it follows the notices. Results go newest first by `order`, whatever order
    /// the caller adds them in.
    #[deprecated(
        note = "Give the footer a typed `Outcome` with `Footer::outcome` (src/design/outcome.rs)."
    )]
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
    pub fn reason(
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
    pub fn notice(
        &mut self,
        words: impl Into<std::borrow::Cow<'a, str>>,
        draw: impl FnOnce(&mut Ui) + 'a,
    ) -> &mut Self {
        self.messages
            .push((FooterRank::Notice, None, words.into(), Box::new(draw)));
        self
    }

    /// One message at its place in the footer's order: the typed form is `Footer::outcome`.
    pub(super) fn push(
        &mut self,
        rank: FooterRank,
        order: Option<ResultOrder>,
        words: std::borrow::Cow<'a, str>,
        draw: impl FnOnce(&mut Ui) + 'a,
    ) -> &mut Self {
        self.messages.push((rank, order, words, Box::new(draw)));
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
        let most = (pane.height() / 3.0).floor().max(0.0);
        // Last pass's height; a change is measured below, and the pass is drawn again.
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
        self.draw_messages(ui, rect, cut, most, stored);
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
        #[cfg(test)]
        record_band(ui, pane, rect);
        // egui's `set_max_height` also moves the cursor back to the top of `ui`, so it goes
        // back to where the body starts.
        ui.set_max_height((limit - top).max(0.0));
        ui.add_space(top - ui.cursor().top());
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

    /// Whether the footer holds a message.
    pub(super) fn is_empty(&self) -> bool {
        self.messages.is_empty()
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
    /// all their words, with a hairline at its top when it `cut`s the body. Keep the height they
    /// want, up to `most`; a change from `stored` draws the pass again.
    pub(super) fn draw_messages(
        &mut self,
        ui: &mut Ui,
        rect: Rect,
        cut: bool,
        most: f32,
        stored: f32,
    ) {
        // Results newest first by their order; a stable sort keeps the caller's order for
        // reasons and notices, and puts plain notices before results with no live Undo.
        self.messages
            .sort_by_key(|(rank, order, ..)| (*rank, order.map(std::cmp::Reverse)));
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
        let (_, content) = scroll_part(ui, rect, self.id, &name, |ui| {
            for (_, _, _, draw) in messages {
                draw(ui);
            }
        });
        let wanted = (content + 2.0 * PART_INSET).ceil().min(most);
        if (wanted - stored).abs() >= HAIRLINE_WIDTH {
            ui.ctx()
                .data_mut(|data| data.insert_temp(self.id.with("height"), wanted));
            ui.ctx()
                .request_discard("the message footer changed height");
        }
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
#[cfg(test)]
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
#[cfg(test)]
pub(crate) fn footer_bands(ctx: &egui::Context) -> Vec<Rect> {
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

pub fn disk_conflict_choices(ui: &mut Ui, noun: &str) -> Option<DiskConflictChoice> {
    // The choices start at the message text, like every other message with choices.
    actions_message(
        ui,
        Tone::Warning,
        &format!("This {noun} changed on disk. Your draft is safe until you choose."),
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
                ui.add_space(L - ui.spacing().item_spacing.x);
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

/// After Reload and lose my draft: the old draft can still come back. True when it should.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn reloaded_draft(ui: &mut Ui) -> bool {
    undo_message_capped(
        ui,
        Tone::Info,
        "Reloaded from disk. Your previous draft is kept until you edit or the file changes again.",
        "Undo reload",
        None,
    )
}

/// A recovery message whose action restores the previous state.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn undo_message(ui: &mut Ui, tone: Tone, text: &str, action: &str) -> bool {
    undo_message_capped(ui, tone, text, action, None)
}

/// A kept line for an act's result and its Undo: one control high whether or not a result
/// shows, so the result moves nothing below it. Words that do not fit beside Undo end in `…`,
/// and their hover keeps them whole. Capped at `LIST_MEASURE` like every notice.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn undo_line(ui: &mut Ui, tone: Tone, result: Option<(&str, &str)>) -> bool {
    let size = vec2(
        ui.available_width().min(LIST_MEASURE),
        Density::Read.control(),
    );
    let mut pressed = false;
    ui.allocate_ui_with_layout(size, Layout::left_to_right(Align::Center), |ui| {
        ui.set_min_size(size);
        let Some((text, action)) = result else {
            return;
        };
        let (icon, color) = tone.parts();
        let button = Action::quiet(action)
            .icon(icons::UNDO)
            .density(Density::Read);
        let gap = ui.spacing().item_spacing.x;
        let left = ui.cursor().left();
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(S + 1.0);
        ui.label(Type::Body.rich(ui, icon).color(color));
        ui.add_space(XS);
        let font = Type::Body.font(ui);
        let room = ui.available_width() - gap - button.natural_width(ui);
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
        ui.add_space(gap);
        pressed = button.show(ui).clicked();
    });
    pressed
}

/// `undo_message`, with an accessible name for the action when its words are not enough. Capped
/// at `LIST_MEASURE` like every notice.
#[deprecated(
    note = "Draw messages through `Footer::outcome`, `design::banner`, `design::row_fault` or `PagePlaceholder` (src/design/outcome.rs)."
)]
pub fn undo_message_named(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    action: &str,
    name: Option<&str>,
) -> bool {
    undo_message_capped(ui, tone, text, action, name)
}

/// `undo_message_named`'s drawing.
pub(super) fn undo_message_capped(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    action: &str,
    name: Option<&str>,
) -> bool {
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        undo_message_inside(ui, tone, text, action, name)
    })
    .inner
}

fn undo_message_inside(
    ui: &mut Ui,
    tone: Tone,
    text: &str,
    action: &str,
    name: Option<&str>,
) -> bool {
    let button = Action::quiet(action)
        .icon(icons::UNDO)
        .density(Density::Read);
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
    if ui.available_width() < text_width + gap + button_width {
        tone_message(ui, tone, text);
        ui.horizontal(|ui| {
            ui.add_space(inset);
            pressed = button.line_start_in_column().show(ui).clicked();
        });
    } else {
        ui.horizontal_top(|ui| {
            let width = (ui.available_width() - button_width - gap).max(0.0);
            ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.add_space((Density::Read.control() - Type::Body.line()) / 2.0);
                tone_message(ui, tone, text)
            });
            pressed = button.show(ui).clicked();
        });
    }
    pressed
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
        ui.add_space(L);
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
            ui.add_space(XS);
            egui::ScrollArea::vertical()
                .id_salt(("dialog_body", id))
                .auto_shrink([false, true])
                .show(ui, add)
                .inner
        })
        .inner
}

/// An image at full size, within the window, its size not known. Returns true when the DM closes
/// it, by a click or Escape. Use `design::popups::image_viewer` with an `ImageSize`: Clippy bans
/// this one outside `src/design/`.
pub fn image_viewer(ctx: &egui::Context, name: &str, uri: &str) -> bool {
    super::popups::image_viewer(ctx, name, uri, super::popups::ImageSize::Unknown)
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
            let width = actions
                .iter()
                .map(|action| action.natural_width(ui))
                .sum::<f32>()
                + XS * actions.len().saturating_sub(1) as f32;
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
pub fn overflow_tabs(ui: &mut Ui, selected: &mut usize, choices: &[(usize, &str)]) {
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
                if ui.selectable_label(*selected == *value, *label).clicked() {
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
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x + 2.0 * S, Density::Read.control()),
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
        ui.painter()
            .galley(rect.center() - galley.size() / 2.0, galley, color);
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

pub fn folder_group_heading(ui: &mut Ui, path: &str, count: usize) {
    ui.label(
        Type::Small
            .rich(
                ui,
                format!("{} {path}/  {count}", egui_phosphor::regular::FOLDER),
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
            ui.add_space(padding);
        });
    ui.spacing_mut().item_spacing.y = row_gap;
}

#[cfg(test)]
// The tests draw the old message functions until Wave 3 makes them private.
#[allow(deprecated)]
mod tests {
    use super::*;

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

    /// The painted text shapes of a harness: each galley's words and where they are drawn.
    fn painted_texts(harness: &egui_kittest::Harness<'_>) -> Vec<(String, Rect)> {
        fn walk(shape: &egui::Shape, out: &mut Vec<(String, Rect)>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
                egui::Shape::Text(text) => out.push((
                    text.galley.job.text.clone(),
                    Rect::from_min_size(text.pos, text.galley.size()),
                )),
                _ => {}
            }
        }
        let mut out = Vec::new();
        for shape in &harness.output().shapes {
            walk(&shape.shape, &mut out);
        }
        out
    }

    /// SD1: an Undo below its message starts its words at the message's words; its icon hangs
    /// over the message's icon column. Beside the message, it keeps its icon before its words.
    #[test]
    fn iter18_a_sd1_a_wrapped_undo_starts_its_words_at_the_message_words() {
        const TEXT: &str = "Ashen Reaches is registered and used here.";
        const UNDO: &str = "Undo package registration";
        for (width, wraps) in [(260.0, true), (360.0, true), (900.0, false)] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 200.0))
                .build_ui(|ui| {
                    undo_message(ui, Tone::Success, TEXT, UNDO);
                });
            super::super::theme::install(&harness.ctx);
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
    }

    /// SD1, ST2: a `line_start` quiet action puts its words at the text edge. With an icon, the
    /// icon and the padding hang left of it (`S + ICON_SLOT`); without one, only the padding.
    /// A normal quiet action keeps its icon and words inside its target from the edge.
    #[test]
    fn iter18_a_a_line_start_action_hangs_its_icon_and_padding() {
        const EDGE: f32 = 60.0;
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(400.0, 200.0))
            .build_ui(|ui| {
                for (label, icon, line_start) in [
                    ("Undo heading change", true, true),
                    ("Clear value", false, true),
                    ("Undo replacement", true, false),
                ] {
                    ui.horizontal(|ui| {
                        ui.add_space(EDGE - ui.cursor().left());
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
        super::super::theme::install(&harness.ctx);
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
            // iter-23-S FK3: beside the words, the action starts `L` after their last letter.
            if width >= 880.0 {
                use egui_kittest::kittest::Queryable;
                let words = harness
                    .output()
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text)
                            if text.galley.job.text.starts_with("The party has full HP") =>
                        {
                            Some(Rect::from_min_size(text.pos, text.galley.size()))
                        }
                        _ => None,
                    })
                    .expect("the words are painted");
                let action = harness.get_by_label("Undo delete saved run").rect();
                assert!(
                    action.top() < words.bottom()
                        && (action.left() - words.right() - L).abs() <= 1.0,
                    "width {width}: words {words:?}, action {action:?}"
                );
            }
        }
        // iter-25-A AB2: in a wide pane, the message and its action stay inside `LIST_MEASURE`,
        // as every notice does. The words alone are wider than the measure.
        const LONG: &str = "This is the open session, not the session you played. Changes here go to \
            the open session's note, so go back to the played session before you add what happened.";
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(1800.0, 240.0))
            .build_ui(|ui| {
                message_with_action(ui, Tone::Warning, LONG, "Go to the played session");
            });
        super::super::theme::install(&harness.ctx);
        harness.run_steps(2);
        let texts = painted_texts(&harness);
        let words = texts
            .iter()
            .find(|(text, _)| text == LONG)
            .map(|(_, rect)| *rect)
            .expect("the words are painted");
        use egui_kittest::kittest::Queryable;
        let action = harness.get_by_label("Go to the played session").rect();
        let left = words.left().min(action.left()) - S - 1.0;
        let right = words.right().max(action.right());
        assert!(
            right - left <= LIST_MEASURE + 0.5 && words.height() > Type::Body.line(),
            "the message spans {left}..{right}: words {words:?}, action {action:?}"
        );
        // The same cap holds for the Undo message and the message with links.
        for links in [false, true] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(1800.0, 240.0))
                .build_ui(move |ui| {
                    if links {
                        message_with_links(ui, Tone::Warning, &["Ashen Marches"], LONG);
                    } else {
                        undo_message(ui, Tone::Success, LONG, "Undo");
                    }
                });
            super::super::theme::install(&harness.ctx);
            harness.run_steps(2);
            let texts = painted_texts(&harness);
            let right = texts
                .iter()
                .map(|(_, rect)| rect.right())
                .fold(0.0, f32::max);
            let left = texts
                .iter()
                .map(|(_, rect)| rect.left())
                .fold(f32::MAX, f32::min);
            assert!(
                !texts.is_empty() && right - left <= LIST_MEASURE,
                "links {links}: the message spans {left}..{right}"
            );
        }
    }

    /// iter-25-A: `help` and a field's help wrap at `READER_MEASURE` in a wide pane.
    #[test]
    fn iter25_a_help_keeps_the_reader_measure() {
        const HELP: &str = "Built-in names stay as they are. A name you add here shows in every \
            list of this campaign, and Lookup finds it by its first word or its whole name.";
        for in_field in [false, true] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(1600.0, 240.0))
                .build_ui(move |ui| {
                    if in_field {
                        field(ui, "Name", Some(HELP), None, |ui| ui.label("control"));
                    } else {
                        help(ui, HELP);
                    }
                });
            super::super::theme::install(&harness.ctx);
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
        }
    }

    /// iter-25-A contract: the message space is two body lines high whether it is empty, holds
    /// one line, holds more than fits or holds a reason and a result, so what follows it never
    /// moves. Empty, it draws nothing. A result with a live Undo comes before the reason; without
    /// one, the reason comes first. The part is one focusable group named with every message in
    /// the order drawn.
    #[test]
    fn iter25_a_message_space_keeps_what_follows_still() {
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
                .build_ui(move |ui| {
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
            super::super::theme::install(&harness.ctx);
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
        const COUNT: &str = "5 notes";
        const FIXING: &str = "1 note needs fixing";
        const UNREAD: &str = "2 notes cannot be read";
        let mut wide_below = Vec::new();
        for (width, actions) in [(465.0, true), (1200.0, true), (1200.0, false)] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 200.0))
                .build_ui(move |ui| {
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
            super::super::theme::install(&harness.ctx);
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
        let state = || Status {
            icon: icons::WARN,
            word: "Connected",
            color: colors().result.warning,
        };
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(200.0, 200.0))
            .build_ui(move |ui| {
                status(ui, state());
                rail_status_detail(ui, state(), "Not remembered");
            });
        super::super::theme::install(&harness.ctx);
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
        const WRAPPED: &str = "Timeline not readable";
        const LINKED: &str = "Session folder not found";
        let state = |word: &'static str| Status {
            icon: icons::WARN,
            word,
            color: colors().result.warning,
        };
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(200.0, 300.0))
            .build_ui(move |ui| {
                status(ui, state("Connected"));
                // As narrow as a rail at 960, so both wrap.
                ui.set_max_width(100.0);
                wrapped_status(ui, state(WRAPPED));
                let _ = status_link(ui, state(LINKED));
            });
        super::super::theme::install(&harness.ctx);
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
                    .build_ui(move |ui| {
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
                                if undo_message_named(
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
                            footer.reason("Line 3 opens a list.", |ui| {
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
            super::super::theme::install(&harness.ctx);
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
                    footer.notice("Notice.", |ui| {
                        ui.label("Notice.");
                    });
                    footer.reason("Reason.", |ui| {
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
                    .build_ui(move |ui| {
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
                            footer.reason(REASON, |ui| {
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
            super::super::theme::install(&harness.ctx);
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
            // footer grows by six results.
            for grown in [0, 6] {
                extra.set(grown);
                harness.run_steps(3);
                let at = footer(&harness).expect("the footer shows");
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
            // Twenty more results overflow the footer: Tab from the field reaches the last Undo,
            // which scrolls into the footer's view.
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
            let shown = text(&harness, "Undo extra 0");
            assert!(
                last_undo.is_some()
                    && focused(&harness) == last_undo
                    && at.height() <= size.y / 3.0 + 1.0
                    && shown.is_some_and(|undo| undo.is_positive() && at.contains_rect(undo)),
                "{mode} at {size:?} Tab reaches the last Undo of the overflowing footer {at:?}, in view: {shown:?}"
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
                Body, Fills, FillsReason, footer_rect, pane_record, pane_records, short_body_gap,
            },
        };
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
            ] {
                for count in [2, 200] {
                    let pressed = Rc::new(Cell::new(None::<&'static str>));
                    let pressed_in = pressed.clone();
                    let mut harness =
                        egui_kittest::Harness::builder()
                            .with_size(size)
                            .build_ui(move |ui| {
                                ui.label("Title");
                                let mut footer = Footer::new("shape_footer", ui.available_width());
                                footer.reason(REASON, |ui| {
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
                                        inner.reason("Inner reason.", |ui| {
                                            message(ui, Tone::Warning, "Inner reason.");
                                        });
                                        inner.pane(ui, Body::Content, |ui| {
                                            rows(ui, count);
                                            press(ui, "Last action");
                                        });
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
                    super::super::theme::install(&harness.ctx);
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
                    let follows = count == 2
                        && matches!(shape, Shape::Rows | Shape::InScope | Shape::InFrame);
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
                    // The body control next to the footer takes its press.
                    let target = match shape {
                        Shape::TallColumn => Some("Tall action"),
                        Shape::SideColumn => Some("Left action"),
                        Shape::Editor => None,
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
                    .build_ui(move |ui| {
                        ui.label("Title");
                        let mut footer = Footer::new("small_footer", ui.available_width());
                        footer.reason(REASON, |ui| {
                            message(ui, Tone::Error, REASON);
                        });
                        footer.pane(ui, Body::Content, |ui| {
                            rows(ui, 8);
                            if ui.button("Last action").clicked() {
                                pressed_in.set(true);
                            }
                        });
                    });
            super::super::theme::install(&harness.ctx);
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

        // One pane function in two `push_id` scopes (Astra re-check 3): A holds a message, B's
        // comes and goes. A's footer keeps its place, its focused Undo and its press. Two panes
        // with one identity in one scope are refused.
        let b_shows = Rc::new(Cell::new(false));
        let undone = Rc::new(Cell::new((0, 0)));
        let (b_in, undone_in) = (b_shows.clone(), undone.clone());
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(960.0, 640.0))
            .build_ui(move |ui| {
                let pane_of = |ui: &mut Ui, side: usize, message: bool| {
                    let mut footer = Footer::new("shared_footer", ui.available_width());
                    if message {
                        let undone = &undone_in;
                        footer.reason(REASON, move |ui| {
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
        super::super::theme::install(&harness.ctx);
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
                .build_ui(move |ui| {
                    let mut footer = Footer::new("inline_footer", ui.available_width());
                    footer.reason(REASON, |ui| {
                        message(ui, Tone::Error, REASON);
                    });
                    footer.pane(ui, Body::Content, |ui| {
                        let _ = ScrollColumn::new("inner").inline(ui, |ui| rows(ui, count));
                    });
                });
            super::super::theme::install(&harness.ctx);
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
                    footer.reason(REASON, |ui| {
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
