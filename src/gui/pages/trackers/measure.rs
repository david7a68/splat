//! Text widths in the theme's type, for columns that line up whatever their words are (the
//! trackers' name and value columns) and for words cut at a whole word with `…`.
use crate::gui::theme::{HAIRLINE_WIDTH, ICON_SLOT, M, Type};
use gpui::{Font, FontFeatures, Pixels, SharedString, TextRun, Window, px};

/// The width of `words` in `ty`, on one line.
pub(crate) fn width(window: &Window, ty: Type, words: &str) -> Pixels {
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
            ty.size(),
            &[run],
            None,
        )
        .width
}

/// How many lines `words` take in `ty` at `room`, wrapped between words as the text element
/// wraps them.
pub(crate) fn lines(window: &Window, ty: Type, words: &str, room: Pixels) -> usize {
    if words.is_empty() {
        return 1;
    }
    let run = TextRun {
        len: words.len(),
        font: Font {
            family: SharedString::new_static(ty.family()),
            features: FontFeatures::default(),
            fallbacks: None,
            weight: ty.weight(),
            style: ty.style(),
        },
        color: gpui::black(),
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_text(
            SharedString::from(words.to_owned()),
            ty.size(),
            &[run],
            Some(room),
            None,
        )
        .map_or(1, |lines| {
            lines
                .iter()
                .map(|line| line.wrap_boundaries().len() + 1)
                .sum()
        })
}

/// The natural width of a secondary or quiet-at-`M` kit button with these words (and an icon).
pub(crate) fn button_width(window: &Window, words: &str, icon: bool) -> Pixels {
    width(window, Type::Strong, words)
        + M * 2.0
        + HAIRLINE_WIDTH * 2.0
        + if icon { ICON_SLOT } else { px(0.0) }
}

/// `words` as a column of `room` shows them: whole, or cut at a whole word with `…` (`…` alone
/// when the first word does not fit).
pub(crate) fn fit_words(window: &Window, ty: Type, words: &str, room: Pixels) -> String {
    if width(window, ty, words) <= room {
        return words.to_owned();
    }
    let parts: Vec<&str> = words.split_whitespace().collect();
    let mut best = None;
    for count in 1..parts.len() {
        let cut = format!("{}…", parts[..count].join(" "));
        if width(window, ty, &cut) <= room {
            best = Some(cut);
        } else {
            break;
        }
    }
    best.unwrap_or_else(|| "…".to_owned())
}
