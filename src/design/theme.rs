//! Fonts and the global `egui::Style`. Plain egui widgets inherit all of this.
use super::tokens::*;
use eframe::egui::{
    self, Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin, Shadow,
    Stroke, TextStyle, Theme, ThemePreference, Vec2, style::ScrollStyle, style::WidgetVisuals,
    vec2,
};
use std::sync::Arc;

fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    for &(name, bytes) in &splat_assets::FONTS {
        fonts
            .font_data
            .insert(name.into(), Arc::new(FontData::from_static(bytes)));
    }
    // Every family ends in the same chain: Source Serif for arrows and fractions
    // as a backstop, Phosphor for icons, then egui's own defaults.
    let defaults = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    let chain = |first: &str| {
        let mut family = vec![first.to_owned(), "serif-regular".to_owned()];
        family.extend(defaults.iter().cloned());
        family
    };
    fonts
        .families
        .insert(FontFamily::Proportional, chain("plex-regular"));
    for (family, face) in [
        (FAMILY_UI, "plex-regular"),
        (FAMILY_UI_MEDIUM, "plex-medium"),
        (FAMILY_UI_SEMIBOLD, "plex-semibold"),
        (FAMILY_UI_BOLD, "plex-bold"),
        (FAMILY_SERIF, "serif-regular"),
        (FAMILY_SERIF_ITALIC, "serif-italic"),
        (FAMILY_SERIF_SEMIBOLD, "serif-semibold"),
        (FAMILY_SERIF_TITLE, "serif-title"),
        (FAMILY_SERIF_SMALL, "serif-small"),
        (FAMILY_SERIF_SMALL_STRONG, "serif-small-strong"),
    ] {
        fonts
            .families
            .insert(FontFamily::Name(family.into()), chain(face));
    }
    fonts
}

fn radius(value: f32) -> CornerRadius {
    CornerRadius::same(value as u8)
}

fn visuals(fill: Color32, stroke: Stroke, text: Color32) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: fill,
        weak_bg_fill: fill,
        bg_stroke: stroke,
        corner_radius: radius(RADIUS_CONTROL),
        fg_stroke: Stroke::new(1.0, text),
        expansion: 0.0,
    }
}

fn apply_colors(style: &mut egui::Style, c: &Colors) {
    let v = &mut style.visuals;
    v.dark_mode = true;
    v.override_text_color = None;
    v.weak_text_color = Some(c.muted);
    v.hyperlink_color = c.accent;
    v.faint_bg_color = c.raised;
    v.extreme_bg_color = c.well;
    v.text_edit_bg_color = Some(c.well);
    v.code_bg_color = c.raised;
    v.warn_fg_color = c.result.warning;
    v.error_fg_color = c.result.error;
    v.panel_fill = c.page;
    v.window_fill = c.raised;
    v.window_stroke = Stroke::new(HAIRLINE_WIDTH, c.rule);
    v.window_corner_radius = radius(RADIUS_LAYER);
    v.menu_corner_radius = radius(RADIUS_LAYER);
    v.window_highlight_topmost = false;
    let shadow = Shadow {
        offset: [0, 6],
        blur: 18,
        spread: 0,
        color: Color32::from_black_alpha(110),
    };
    v.window_shadow = shadow;
    v.popup_shadow = shadow;
    v.selection.bg_fill = c.accent.gamma_multiply(0.28);
    v.selection.stroke = Stroke::new(FOCUS_WIDTH, c.focus);
    v.text_cursor.stroke = Stroke::new(2.0, c.focus);
    v.text_cursor.blink = false;
    v.striped = false;
    v.indent_has_left_vline = false;
    v.collapsing_header_frame = false;
    v.disabled_alpha = 0.6;
    v.widgets.noninteractive = visuals(c.page, Stroke::new(HAIRLINE_WIDTH, c.hairline), c.text);
    // A field at rest needs a 3:1 edge, so it shows the same one a secondary button does.
    v.widgets.inactive = visuals(
        c.raised,
        Stroke::new(HAIRLINE_WIDTH, c.field_border),
        c.text,
    );
    v.widgets.hovered = visuals(c.hover, Stroke::new(HAIRLINE_WIDTH, c.text), c.text);
    v.widgets.active = visuals(c.selected, Stroke::new(FOCUS_WIDTH, c.focus), c.text);
    v.widgets.open = visuals(c.selected, Stroke::new(HAIRLINE_WIDTH, c.rule), c.text);
}

fn apply_type(style: &mut egui::Style) {
    for role in Type::ALL {
        if role != Type::Mono {
            style.text_styles.insert(
                role.style(),
                FontId::new(role.size(), FontFamily::Name(role.family().into())),
            );
        }
    }
    let ui = |size: f32, family: &str| FontId::new(size, FontFamily::Name(family.into()));
    style.text_styles.insert(
        TextStyle::Heading,
        ui(Type::Title.size(), FAMILY_SERIF_TITLE),
    );
    style
        .text_styles
        .insert(TextStyle::Body, ui(Type::Body.size(), FAMILY_UI));
    style
        .text_styles
        .insert(TextStyle::Button, ui(Type::Body.size(), FAMILY_UI_MEDIUM));
    style
        .text_styles
        .insert(TextStyle::Small, ui(Type::Small.size(), FAMILY_UI));
    style
        .text_styles
        .insert(TextStyle::Monospace, FontId::monospace(Type::Mono.size()));
}

fn apply_spacing(style: &mut egui::Style) {
    let s = &mut style.spacing;
    s.item_spacing = vec2(S, 6.0);
    s.button_padding = vec2(10.0, 4.0);
    s.interact_size = Vec2::new(40.0, 26.0);
    s.menu_margin = Margin::same(6);
    s.window_margin = Margin::same(L as i8);
    s.indent = L;
    s.icon_width = 16.0;
    s.icon_width_inner = 10.0;
    s.icon_spacing = 6.0;
    s.combo_height = 320.0;
    s.tooltip_width = 320.0;
    s.scroll = scroll_style();
    style.interaction.tooltip_delay = 0.25;
}

/// Force the one dark theme and install its fonts and style.
/// The app's solid scroll bars. `ScrollColumn` sets it on its own `Ui`, so its lane does not
/// depend on a caller's style.
pub fn scroll_style() -> ScrollStyle {
    ScrollStyle {
        foreground_color: false,
        dormant_background_opacity: 0.0,
        active_background_opacity: 0.4,
        interact_background_opacity: 0.6,
        dormant_handle_opacity: 0.5,
        active_handle_opacity: 0.8,
        interact_handle_opacity: 1.0,
        bar_width: 8.0,
        floating_width: 8.0,
        floating_allocated_width: 0.0,
        ..ScrollStyle::solid()
    }
}

pub fn install(ctx: &egui::Context) {
    // The app reads focused widgets' roles, and assistive technology can attach at any time.
    ctx.enable_accesskit();
    ctx.set_fonts(fonts());
    ctx.set_theme(ThemePreference::Dark);
    let mut style = (*ctx.style_of(Theme::Dark)).clone();
    apply_colors(&mut style, colors());
    apply_type(&mut style);
    apply_spacing(&mut style);
    ctx.set_style_of(Theme::Dark, style);
}
