//! The width words take on one line, measured by GPUI's text system in a type role (egui's
//! `layout_no_wrap(..).size().x`). For a part that fits its contents to the room it has (the
//! party bar) or lines numbers up in a column (a stepper's well).
use crate::gui::icons;
use crate::gui::theme::{Type, colors};
use gpui::{Pixels, SharedString, Window, px};

/// The width of `words` on one line in `ty`.
pub fn width(window: &Window, ty: Type, words: &str) -> Pixels {
    shaped(window, ty, ty.family(), words)
}

/// The width of an icon glyph at `ty`'s size, in the icon font (`TypeExt::icon_font`).
pub fn icon_width(window: &Window, ty: Type, glyph: &str) -> Pixels {
    shaped(window, ty, icons::FAMILY, glyph)
}

/// The widest of the ten digits in `ty`, so a column of numbers can hold `n` of any.
pub fn digit_width(window: &Window, ty: Type) -> Pixels {
    ('0'..='9')
        .map(|digit| width(window, ty, &digit.to_string()))
        .fold(px(0.0), Pixels::max)
}

fn shaped(window: &Window, ty: Type, family: &'static str, words: &str) -> Pixels {
    if words.is_empty() {
        return px(0.0);
    }
    let font = gpui::Font {
        family: SharedString::new_static(family),
        features: gpui::FontFeatures::default(),
        fallbacks: None,
        weight: ty.weight(),
        style: ty.style(),
    };
    let run = gpui::TextRun {
        len: words.len(),
        font,
        color: colors().text,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(
            SharedString::from(words.to_owned()),
            ty.size(),
            &[run],
            None,
        )
        .width
        .ceil()
}
