//! A draft whose file changed on disk: in the editor's place, its
//! facts as warnings, the choices after the last one (`Overwrite with my draft`, then the red
//! one), what differs part by part, then the draft beside the file.
use crate::gui::theme::{Density, S, Type, XXL, colors};
use crate::gui::{SplatRoot, kit};
use gpui::{AnyElement, Context, ElementId, IntoElement, ParentElement, Styled, div};
use splat_encounters_ui::encounters::model::DiskView;

/// The conflict. `choose` takes the DM's choice: `true` keeps the draft.
pub(super) fn view(
    id: &'static str,
    disk: &DiskView,
    choose: impl Fn(&mut SplatRoot, bool) + Clone + 'static,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let (keep, lose) = (choose.clone(), choose);
    let mut block = div().flex().flex_col().size_full().min_h_0().gap(S);
    for fact in &disk.facts {
        block = block.child(kit::Message::new(kit::Tone::Warning, fact.clone()));
    }
    block = block.child(
        kit::Message::new(kit::Tone::Warning, disk.last.clone())
            .action(
                kit::Button::secondary(
                    ElementId::Name(format!("{id}-keep").into()),
                    disk.keep.clone(),
                )
                .density(Density::Read)
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    keep(root, true);
                    cx.notify();
                })),
            )
            .destructive(
                kit::Button::destructive(
                    ElementId::Name(format!("{id}-lose").into()),
                    disk.lose.clone(),
                )
                .density(Density::Read)
                .on_click(cx.listener(move |root: &mut SplatRoot, _, _, cx| {
                    lose(root, false);
                    cx.notify();
                })),
            ),
    );
    if !disk.differences.is_empty() {
        let mut parts = div().flex().flex_col().pt(S);
        for (label, words) in &disk.differences {
            parts = parts.child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .child(div().flex_none().w(XXL * 3.0 + S).child(kit::text(
                        Type::Small,
                        c.muted,
                        label.clone(),
                    )))
                    .child(div().flex_1().min_w_0().child(kit::text(
                        Type::Small,
                        c.text,
                        words.clone(),
                    ))),
            );
        }
        block = block.child(parts);
    }
    if let Some((_, text)) = &disk.disk {
        block = block.child(
            div().flex_1().min_h_0().child(
                kit::DraftComparison::new(
                    ElementId::Name(format!("{id}-comparison").into()),
                    disk.draft.clone(),
                    text.clone(),
                )
                .source(),
            ),
        );
    }
    block.into_any_element()
}
