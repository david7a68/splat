//! A number that rolls: an ability's modifier, a save, a skill, a sheet's
//! bonus. Accent words with a quiet underline, at least 24 px each way, the focus colour on hover
//! and the roll's words (`rolls::hint`) as its hover words. A press asks for the roll through the
//! same rolls API as a Markdown dice link (`rolls::ask_pressed`, Shift for advantage, Ctrl for
//! disadvantage): beside the run, a number in a participant's own sheet rolls for them
//! (`.roller(Some(row))`); anywhere else it is a preview. The frame logic routes the ask
//! (`Splat::route_rolls`).
//!
//! ```ignore
//! kit::RollNumber::new(("save", at), "+5", Type::Numeral, link).entry(&name)
//! ```
use super::markdown::Host;
use super::{WithTooltip as _, target};
use crate::gui::theme::{HAIRLINE_WIDTH, MIN_TARGET, Type, TypeExt, colors};
use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, prelude::*,
};
use splat::content::dice_text::RollLink;

#[derive(IntoElement)]
pub struct RollNumber {
    id: ElementId,
    words: SharedString,
    ty: Type,
    link: RollLink,
    entry: Option<String>,
    roller: Option<usize>,
}

impl RollNumber {
    pub fn new(
        id: impl Into<ElementId>,
        words: impl Into<SharedString>,
        ty: Type,
        link: RollLink,
    ) -> Self {
        Self {
            id: id.into(),
            words: words.into(),
            ty,
            link,
            entry: None,
            roller: None,
        }
    }

    /// Who rolls, as the result names them: the sheet's name.
    pub fn entry(mut self, entry: impl Into<String>) -> Self {
        self.entry = Some(entry.into());
        self
    }

    /// The run's row that rolls it, or `None` for a preview.
    pub fn roller(mut self, roller: Option<usize>) -> Self {
        self.roller = roller;
        self
    }
}

impl RenderOnce for RollNumber {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let c = colors();
        let hint = crate::rolls::hint(&self.link);
        let name = format!("{} {}", crate::rolls::link_label(&self.link), self.words);
        let (link, entry, roller) = (self.link, self.entry, self.roller);
        let element = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .min_w(MIN_TARGET)
            .min_h(MIN_TARGET)
            .cursor_pointer()
            .ty(self.ty)
            .text_color(c.accent)
            .hover(move |s| s.text_color(c.focus))
            .whitespace_nowrap()
            .aria_label(SharedString::from(hint.clone()))
            .child(
                div()
                    .border_b(HAIRLINE_WIDTH)
                    .border_color(c.accent.opacity(0.6))
                    .child(self.words),
            )
            .with_tooltip(hint)
            .on_click(move |_, window, cx| {
                let Some(ctx) = cx.try_global::<Host>().map(|host| host.0.clone()) else {
                    return;
                };
                // What the ask names is set as it goes, so whatever drew last does not decide.
                if let Some(entry) = &entry {
                    crate::rolls::set_entry(&ctx, entry);
                }
                crate::rolls::set_roller(&ctx, roller);
                let keys = window.modifiers();
                crate::rolls::ask_pressed(&ctx, &link, keys.shift, keys.control || keys.platform);
                crate::rolls::set_roller(&ctx, None);
                // The next frame's logic routes the ask (`Splat::route_rolls`).
                window.refresh();
            });
        target(element, name)
    }
}
