//! A round portrait (`art::portrait`): the picture, cropped to a circle, or the name's initial
//! on a quiet disc when there is none.
use crate::gui::theme::{Type, TypeExt, colors};
use gpui::{Div, ObjectFit, ParentElement, Pixels, Styled, StyledImage, div, img};
use std::path::Path;

/// A portrait `size` across, of `name`.
pub fn portrait(path: Option<&Path>, name: &str, size: Pixels) -> Div {
    let c = colors();
    let disc = div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(size)
        .rounded_full()
        .overflow_hidden()
        .bg(c.hover);
    match path.filter(|path| path.is_file()) {
        Some(path) => disc.child(
            img(path.to_path_buf())
                .size(size)
                .rounded_full()
                .object_fit(ObjectFit::Cover),
        ),
        None => disc.child(
            div().ty(Type::Name).text_color(c.muted).child(
                name.chars()
                    .next()
                    .map(|letter| letter.to_uppercase().collect::<String>())
                    .unwrap_or_default(),
            ),
        ),
    }
}
