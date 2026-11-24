//! The copy dialog (`copy_dialog::CopyDialog`): where a copy of an entry goes, its name and
//! folder, what it also copies, then its result with the next steps.
use super::act;
use crate::copy_dialog::CopyDialog;
use crate::gui::kit::WithTooltip as _;
use crate::gui::theme::{HAIRLINE_WIDTH, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, kit};
use gpui::{
    AnyElement, Context, InteractiveElement, IntoElement, ParentElement, Styled, Window, div,
    prelude::*,
};
use splat::content::Entry;

pub(super) fn render(
    root: &mut SplatRoot,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let draft = root.app.encounters.draft_name();
    let campaigns = &root.app.campaigns;
    let Some(dialog) = root.app.copy_dialog.as_mut() else {
        return div().into_any_element();
    };
    dialog.draft = draft;
    dialog.poll();
    let destinations = CopyDialog::destinations(campaigns);
    let blocked = if dialog.completed().is_none() {
        dialog.prepare(&destinations)
    } else {
        None
    };
    let mut frame = kit::Dialog::new("copy-entry", "Copy entry")
        .on_escape(act(cx, |root, _, cx| {
            let ctx = root.ctx.clone();
            root.app.follow_copy_dialog(&ctx, false);
            cx.notify();
        }))
        .child(kit::text(
            Type::Heading,
            c.text,
            dialog.source_name().to_owned(),
        ));
    if let Some((message, path, add_to)) = dialog.completed() {
        let mut result = div()
            .id("copy-result")
            .child(kit::Message::new(kit::Tone::Success, message.to_owned()));
        if let Some(path) = path {
            result = result.with_tooltip(path.display().to_string());
        }
        frame = frame.child(result);
        if let Some(draft) = add_to {
            frame = frame.action(
                kit::Button::primary("copy-add-to-draft", format!("Add to {draft}")).on_click(
                    cx.listener(|root, _, _, cx| {
                        if let Some(dialog) = &mut root.app.copy_dialog {
                            dialog.add_copy_to_draft();
                        }
                        let ctx = root.ctx.clone();
                        root.app.follow_copy_dialog(&ctx, false);
                        cx.notify();
                    }),
                ),
            );
        }
        let edit = if add_to.is_some() {
            kit::Button::secondary("copy-edit", "Edit the copy")
        } else {
            kit::Button::primary("copy-edit", "Edit the copy")
        };
        frame = frame
            .action(edit.on_click(cx.listener(|root, _, _, cx| {
                if let Some(dialog) = &mut root.app.copy_dialog {
                    dialog.edit_the_copy();
                }
                let ctx = root.ctx.clone();
                root.app.follow_copy_dialog(&ctx, false);
                cx.notify();
            })))
            .action(
                kit::Button::secondary("copy-another", "Copy another").on_click(cx.listener(
                    |root, _, _, cx| {
                        if let Some(dialog) = &mut root.app.copy_dialog {
                            dialog.copy_another();
                        }
                        cx.notify();
                    },
                )),
            )
            .action(
                kit::Button::quiet("copy-close", "Close").on_click(cx.listener(
                    |root, _, _, cx| {
                        let ctx = root.ctx.clone();
                        root.app.follow_copy_dialog(&ctx, false);
                        cx.notify();
                    },
                )),
            );
        // The result's refresh, as egui's frame does after the dialog draws.
        follow(root);
        return frame.into_any_element();
    }
    frame = frame.child(kit::text(
        Type::Body,
        c.text,
        "This makes a separate copy you can change. Links to the original stay as they are.",
    ));
    if dialog.loading() {
        frame = frame.child(kit::text(
            Type::Body,
            c.muted,
            "Checking content dependencies…",
        ));
    }
    if let Some((error, warning)) = dialog.error() {
        frame = frame.child(kit::Message::new(
            if warning {
                kit::Tone::Warning
            } else {
                kit::Tone::Error
            },
            error.to_owned(),
        ));
    }
    let chosen = destinations
        .iter()
        .position(|(campaign, id, ..)| Some((*campaign, *id)) == dialog.destination());
    let choices: Vec<_> = destinations.iter().map(|(c, id, ..)| (*c, *id)).collect();
    frame = frame.child(kit::field(
        "Destination",
        None,
        None,
        kit::Dropdown::new(
            "copy-destination",
            "Destination",
            destinations.iter().map(|d| d.2.clone()),
            chosen,
        )
        .on_select(cx.listener(move |root, at: &usize, _, cx| {
            if let Some(dialog) = &mut root.app.copy_dialog {
                dialog.set_destination(choices.get(*at).copied());
            }
            cx.notify();
        })),
    ));
    let _ = dialog.take_focus_name();
    frame = frame.child(kit::field(
        "Copy name",
        None,
        None,
        kit::TextField::new("copy-name", dialog.name().to_owned())
            .name("Copy name")
            .on_change(cx.listener(|root, words: &str, _, cx| {
                if let Some(dialog) = &mut root.app.copy_dialog {
                    dialog.set_name(words);
                }
                cx.notify();
            })),
    ));
    let destination = chosen.map(|at| &destinations[at]);
    if let Some(destination) = destination {
        frame = frame.child(kit::field(
            "Folder",
            None,
            None,
            kit::TextField::new("copy-folder", dialog.folder().to_owned())
                .name("Folder")
                .on_change(cx.listener(|root, words: &str, _, cx| {
                    if let Some(dialog) = &mut root.app.copy_dialog {
                        dialog.set_folder(words);
                    }
                    cx.notify();
                })),
        ));
        if !destination.0 && campaigns.active_campaign().is_some() {
            frame = frame.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(XS)
                    .child(
                        kit::Checkbox::new("copy-used", "Choose for this campaign", dialog.used())
                            .on_click(cx.listener(|root, _, _, cx| {
                                if let Some(dialog) = &mut root.app.copy_dialog {
                                    dialog.toggle_used();
                                }
                                cx.notify();
                            })),
                    )
                    .child(kit::help(
                        "Adds the copy to this campaign’s package choices.",
                    )),
            );
        }
    }
    let cancel =
        kit::Button::quiet("copy-cancel", "Cancel").on_click(cx.listener(|root, _, _, cx| {
            let ctx = root.ctx.clone();
            root.app.follow_copy_dialog(&ctx, false);
            cx.notify();
        }));
    let Some(plan) = dialog.plan() else {
        if let Some(reason) = blocked {
            frame = frame.child(kit::text(Type::Small, c.muted, reason));
        }
        return frame.action(cancel).into_any_element();
    };
    let name = destination
        .map(|d| d.2.split_once(" · ").map_or(d.2.as_str(), |(_, name)| name))
        .unwrap_or_default()
        .to_owned();
    frame = frame.child(kit::rule()).child(
        div()
            .id("copy-path")
            .with_tooltip(format!(
                "Will create {}",
                plan.root.join(&plan.relative).display()
            ))
            .child(kit::text(
                Type::Small,
                c.muted,
                format!("Will create {name}/{}", plan.relative),
            )),
    );
    for dependency in &plan.shared {
        if let Ok(entry) = Entry::parse(&dependency.relative, &dependency.text) {
            frame = frame.child(kit::text(
                Type::Small,
                c.muted,
                if plan.shared.len() == 1 {
                    format!(
                        "Also copies {}, because {} links to it.",
                        entry.name,
                        dialog.source_name()
                    )
                } else {
                    format!(
                        "Also copies {}, because a copied entry links to it.",
                        entry.name
                    )
                },
            ));
        }
        frame = frame.child(kit::text(
            Type::Small,
            c.muted,
            format!("Will create {name}/{}", dependency.relative),
        ));
    }
    let shared_names: Vec<_> = plan
        .shared
        .iter()
        .filter_map(|dependency| {
            Entry::parse(&dependency.relative, &dependency.text)
                .ok()
                .map(|entry| entry.name)
        })
        .collect();
    if plan.dependencies.is_empty() {
        frame = frame.child(kit::text(Type::Small, c.muted, "Copies no other entry."));
    } else if plan
        .dependencies
        .iter()
        .any(|name| !shared_names.contains(name))
    {
        frame = frame.child(kit::heading(
            kit::Heading::Group {
                title: &format!("Required entries: {}", plan.dependencies.len()),
                count: 0,
            },
            false,
        ));
        for name in &plan.dependencies {
            frame = frame.child(kit::text(Type::Body, c.text, name.clone()));
        }
    }
    if !plan.dependencies.is_empty() {
        frame = frame
            .child(kit::heading(
                kit::Heading::Group {
                    title: "Review Markdown",
                    count: 0,
                },
                false,
            ))
            .child(
                div()
                    .id("copy-review")
                    .max_h(crate::gui::theme::XXL * 5.0)
                    .overflow_y_scroll()
                    .p(S)
                    .bg(c.well)
                    .border(HAIRLINE_WIDTH)
                    .border_color(c.field_border)
                    .ty(Type::Mono)
                    .text_color(c.text)
                    .child(plan.text.clone()),
            );
    }
    let create = kit::Button::primary(
        "copy-create",
        if plan.shared.is_empty() {
            "Create copy"
        } else {
            "Copy both"
        },
    )
    .on_click(cx.listener(|root, _, _, cx| {
        let destinations = CopyDialog::destinations(&root.app.campaigns);
        if let Some(dialog) = &mut root.app.copy_dialog {
            dialog.create(&destinations, &root.app.campaigns);
        }
        follow(root);
        cx.notify();
    }));
    frame.action(create).action(cancel).into_any_element()
}

/// What the dialog asked of the app (a refresh after a copy), as egui does after it draws.
fn follow(root: &mut SplatRoot) {
    let ctx = root.ctx.clone();
    root.app.follow_copy_dialog(&ctx, true);
}
