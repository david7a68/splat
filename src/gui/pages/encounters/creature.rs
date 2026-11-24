//! The creature that `Make it yours` opened, beside the roster in Add participants' place
//! (egui: `Encounters::creature_pane`): `This campaign’s creature` with Done while nothing is
//! changed, the save row (its state word, `Discard creature changes`, `Save creature`), the
//! name, hit points and armour class, and its text as Markdown under a quiet section. A failed
//! Save says why in the pane's footer. While its file changed on disk, the conflict shows here.
use super::common::with_page;
use crate::gui::theme::{Density, S, Type, XS, colors};
use crate::gui::{SplatRoot, kit};
use gpui::{AnyElement, Context, ElementId, IntoElement, ParentElement, Styled, div, px};
use splat_encounters_ui::encounters::model::{CreatureAct, CreatureField, CreatureView};

fn act(
    cx: &mut Context<SplatRoot>,
    act: CreatureAct,
) -> impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static {
    cx.listener(move |root: &mut SplatRoot, _, _, cx| {
        with_page(root, |page, _, active, campaigns| {
            page.creature_act(act, active, campaigns.modules())
        });
        cx.notify();
    })
}

fn field(
    cx: &mut Context<SplatRoot>,
    which: CreatureField,
) -> impl Fn(&str, &mut gpui::Window, &mut gpui::App) + 'static {
    cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
        root.app.encounters.edit_creature(which, words);
        cx.notify();
    })
}

/// The pane.
pub(super) fn pane(view: &CreatureView, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    if let Some(disk) = &view.disk {
        return super::disk::view(
            "creature-disk",
            disk,
            |root: &mut SplatRoot, keep| {
                with_page(root, |page, _, active, campaigns| {
                    page.creature_disk_choice(keep, active, campaigns.modules())
                });
            },
            cx,
        );
    }
    let mut heading = div()
        .flex()
        .flex_row()
        .items_center()
        .min_h(Density::Read.control())
        .child(
            div()
                .flex_1()
                .child(kit::text(Type::Heading, c.text, "This campaign’s creature")),
        );
    if !view.dirty {
        heading = heading.child(
            kit::Button::quiet("creature-done", "Done")
                .name("Done with this creature")
                .density(Density::Read)
                .on_click(act(cx, CreatureAct::Close)),
        );
    }
    // The save row by its one rule: the word and Discard at the left, Save creature at the right.
    // The encounter beside it owns the primary, so Save stays secondary.
    let (icon, word, color) = &view.state;
    let mut state = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .child(kit::status(&kit::Status::new(
            icon.clone(),
            word.clone(),
            crate::gui::bridge::color(*color),
        )));
    if view.discard {
        state = state.child(
            kit::Button::quiet("creature-discard", "Discard creature changes")
                .density(Density::Read)
                .on_click(act(cx, CreatureAct::Close)),
        );
    }
    let save_row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(Density::Read.control())
        .child(div().flex_1().child(state))
        .child(
            kit::Button::secondary("creature-save", "Save creature")
                .key("Ctrl+S")
                .blocked(view.save_blocked.clone())
                .on_click(act(cx, CreatureAct::Save)),
        );
    let key = view.path.clone();
    let fields = div()
        .flex()
        .flex_col()
        .gap(XS)
        .child(kit::field(
            "Name",
            None,
            None,
            kit::TextField::new(
                ElementId::Name(format!("creature-name:{key}").into()),
                view.name.clone(),
            )
            .name("Name")
            .placeholder("Name")
            .on_change(field(cx, CreatureField::Name)),
        ))
        .child(
            div()
                .flex()
                .flex_row()
                .gap(S)
                .child(kit::field(
                    "Hit points",
                    None,
                    None,
                    kit::TextField::new(
                        ElementId::Name(format!("creature-hp:{key}").into()),
                        view.hp.clone(),
                    )
                    .name("Hit points")
                    .placeholder("HP")
                    .width(px(56.0))
                    .focus_selected(view.focus_hp)
                    .on_change(field(cx, CreatureField::Hp)),
                ))
                .child(kit::field(
                    "Armor class",
                    None,
                    None,
                    kit::TextField::new(
                        ElementId::Name(format!("creature-ac:{key}").into()),
                        view.armor_class.clone(),
                    )
                    .name("Armor class")
                    .placeholder("AC")
                    .width(px(56.0))
                    .on_change(field(cx, CreatureField::ArmorClass)),
                )),
        )
        .child(
            kit::QuietSection::new(
                ElementId::Name(format!("creature-text-section:{key}").into()),
                "Its text, as Markdown",
            )
            .child(
                kit::TextEditor::new(
                    ElementId::Name(format!("creature-text:{key}").into()),
                    view.body.clone(),
                )
                .name("Creature text")
                .mono()
                .rows(12)
                .on_change(field(cx, CreatureField::Body)),
            ),
        );
    let mut pane = kit::Pane::new(ElementId::Name(format!("creature-pane:{key}").into()))
        .flush()
        .child(kit::scroll_column(
            ElementId::Name(format!("creature-scroll:{key}").into()),
            div().flex().flex_col().gap(S).child(fields),
        ));
    if let Some(error) = &view.error {
        pane = pane.footer(kit::Message::new(kit::Tone::Error, error.clone()));
    }
    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(XS)
        .child(heading)
        .child(save_row)
        .child(div().flex_1().min_h_0().child(pane))
        .into_any_element()
}
