//! Text widths in the theme's type, for the run's fitted words:
//! the turn name's size, the panel's tabs, the Undo label's form, the roster's columns.
use crate::gui::theme::{HAIRLINE_WIDTH, ICON_SLOT, M, Type};
use gpui::{Font, FontFeatures, Pixels, SharedString, TextRun, Window, px};

/// The width of `words` in `ty` at `size` (the role's own size when `None`), on one line.
pub(super) fn width_at(window: &Window, ty: Type, size: Option<Pixels>, words: &str) -> Pixels {
    if words.is_empty() {
        return px(0.0);
    }
    let font = Font {
        family: SharedString::new_static(ty.family()),
        features: FontFeatures::default(),
        fallbacks: None,
        weight: ty.weight(),
        style: ty.style(),
    };
    let run = TextRun {
        len: words.len(),
        font,
        color: gpui::black(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(
            SharedString::from(words.to_owned()),
            size.unwrap_or(ty.size()),
            &[run],
            None,
        )
        .width
}

/// The width of an icon glyph (`gui::icons`) at `ty`'s size, in the icon font it is drawn in.
pub(super) fn icon_width(window: &Window, ty: Type, glyph: &str) -> Pixels {
    let font = Font {
        family: SharedString::new_static(crate::gui::theme::family::ICONS),
        features: FontFeatures::default(),
        fallbacks: None,
        weight: gpui::FontWeight::NORMAL,
        style: gpui::FontStyle::Normal,
    };
    let run = TextRun {
        len: glyph.len(),
        font,
        color: gpui::black(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(
            SharedString::from(glyph.to_owned()),
            ty.size(),
            &[run],
            None,
        )
        .width
}

/// The width of `words` in `ty`, on one line.
pub(super) fn width(window: &Window, ty: Type, words: &str) -> Pixels {
    width_at(window, ty, None, words)
}

/// The natural width of a secondary or quiet kit button at `M` with these words, an icon and a
/// key letter (`kit::Button`'s frame: `M` at each side, the hairline, the icon's slot, and the
/// key after `S`).
pub(super) fn button_width(window: &Window, words: &str, icon: bool, key: Option<&str>) -> Pixels {
    width(window, Type::Strong, words)
        + M * 2.0
        + HAIRLINE_WIDTH * 2.0
        + if icon { ICON_SLOT } else { px(0.0) }
        + key.map_or(px(0.0), |key| {
            crate::gui::theme::S + width(window, Type::Small, key)
        })
}

/// A creature's name in `room`: whole, or whole words then `…`, keeping a copy number or letter
/// (`Ember Cultist… 12`; `turn_heading::whole_words_name`).
pub(super) fn fit_name(window: &Window, ty: Type, name: &str, room: Pixels) -> String {
    crate::design::turn_heading::whole_words_name_by(name, f32::from(room), |words| {
        f32::from(width(window, ty, words))
    })
}
