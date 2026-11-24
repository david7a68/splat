//! Words, icons, headings, rules and gaps: the text styles of the type ramp.
use crate::gui::theme::{Type, TypeExt, XL, XS, colors};
use gpui::{Div, Hsla, IntoElement, ParentElement, Pixels, SharedString, Styled, div, px};

/// Words in one role of the type ramp and one colour token. Use it for any text a page shows:
/// `text(Type::Body, colors().text, "Words")`. Long words wrap at the parent's width; add
/// `.truncate()` for one line cut with an ellipsis, or `.whitespace_nowrap()` to keep one line.
pub fn text(ty: Type, color: Hsla, words: impl Into<SharedString>) -> Div {
    let words = words.into();
    let words = if ty.capitals() {
        SharedString::from(tracked(&words.to_uppercase()))
    } else {
        words
    };
    div().ty(ty).text_color(color).child(words)
}

/// A hair space (U+200A): 0.059 em in IBM Plex Sans, 0.65 px at `Label`'s 11 px.
const HAIR_SPACE: char = '\u{200A}';

/// `words` with `Label`'s tracking (egui: `extra_letter_spacing` 0.6): a hair space between the
/// letters of each word. Spaces between words are left as they are, so a label still wraps only
/// between its words.
fn tracked(words: &str) -> String {
    let mut out = String::with_capacity(words.len() * 2);
    let mut previous: Option<char> = None;
    for letter in words.chars() {
        if previous.is_some_and(|p| !p.is_whitespace()) && !letter.is_whitespace() {
            out.push(HAIR_SPACE);
        }
        out.push(letter);
        previous = Some(letter);
    }
    out
}

/// A Phosphor icon (`gui::icons`) at a role's size, in a colour token. An icon supports words:
/// put its name beside it, or give the control around it a name.
pub fn icon(glyph: &'static str, ty: Type, color: Hsla) -> Div {
    div()
        .icon_font(ty)
        .text_color(color)
        .flex_none()
        .child(SharedString::new_static(glyph))
}

/// A column or compact group heading: `Label` capitals in the text colour, then a nonzero count
/// in muted (`CAMPAIGNS 3`).
pub fn label(words: impl Into<SharedString>, count: usize) -> Div {
    let c = colors();
    let words: SharedString = words.into();
    let title = words.clone();
    let row =
        div()
            .flex()
            .flex_row()
            .items_baseline()
            .gap(XS)
            .child(text(Type::Label, c.text, words));
    if count == 0 {
        row
    } else {
        // Named by the heading and its count, so a test finds the count as the DM reads it.
        let counted = crate::design::count(count, "", "");
        let name = format!("{title} {counted}");
        row.child(super::target(text(Type::Label, c.muted, counted), name))
    }
}

/// The kinds of heading (`design::Heading`).
#[derive(Clone, Copy, Debug)]
pub enum Heading<'a> {
    /// A group of controls: `Label` capitals and a nonzero count.
    Group { title: &'a str, count: usize },
    /// A reader's or stat block's section, in the serif.
    Section(&'a str),
    /// A panel's, form's or band's own title, in `Heading` type.
    Panel(&'a str),
    /// A sheet's own name, in `Display` type.
    Name(&'a str),
}

/// A heading and its own space: `L` above (`XL` for a serif section or a name) unless it is
/// first in its column (`first`), and `XS` under it, so it belongs to what follows.
pub fn heading(heading: Heading<'_>, first: bool) -> Div {
    let c = colors();
    let (above, body) = match heading {
        Heading::Group { title, count } => (crate::gui::theme::L, label(title.to_owned(), count)),
        Heading::Section(title) => (XL, text(Type::Section, c.text, title.to_owned())),
        Heading::Panel(title) => (
            crate::gui::theme::L,
            text(Type::Heading, c.text, title.to_owned()),
        ),
        Heading::Name(title) => (XL, text(Type::Display, c.text, title.to_owned())),
    };
    div()
        .when_some_pt(if first { None } else { Some(above) })
        .pb(XS)
        .child(body)
}

/// A hairline rule across its parent.
pub fn rule() -> Div {
    div()
        .w_full()
        .h(crate::gui::theme::HAIRLINE_WIDTH)
        .flex_none()
        .bg(colors().hairline)
}

/// Empty room of a token's height (a space between blocks in a column).
pub fn gap(height: Pixels) -> Div {
    div().h(height).w(px(0.0)).flex_none()
}

trait WhenSome: Styled + Sized {
    fn when_some_pt(self, top: Option<Pixels>) -> Self {
        match top {
            Some(top) => self.pt(top),
            None => self,
        }
    }
}
impl<T: Styled> WhenSome for T {}

/// The width of `words` set in `ty` on one line (egui's `number_width`): to choose between a
/// long and a short form of words by what fits.
pub fn width(window: &mut gpui::Window, ty: Type, words: &str) -> Pixels {
    if words.is_empty() {
        return px(0.0);
    }
    let font = gpui::Font {
        family: SharedString::new_static(ty.family()),
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
        .shape_line(words.to_owned().into(), ty.size(), &[run], None)
        .width
}

/// One line that is cut with an ellipsis when it does not fit.
pub fn one_line(ty: Type, color: Hsla, words: impl Into<SharedString>) -> Div {
    text(ty, color, words).truncate()
}

/// Turn any `IntoElement` into a boxed element, for heterogeneous lists of children.
pub fn any(element: impl IntoElement) -> gpui::AnyElement {
    element.into_any_element()
}

#[cfg(test)]
mod tests {
    /// Label tracking goes between the letters of a word, never beside a space.
    #[test]
    fn tracking_is_between_letters() {
        assert_eq!(
            super::tracked("IN PLAY 2"),
            "I\u{200A}N P\u{200A}L\u{200A}A\u{200A}Y 2"
        );
    }
}
