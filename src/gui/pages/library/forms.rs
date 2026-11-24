//! The Library's forms, in the reader's place: New entry (`Library::new_entry`) and Manage
//! packages (`Library::package_form`); and the unregister confirmation (`unregister_dialog`).
use crate::gui::theme::{Density, FORM_WIDTH, L, S, Type, XS, colors};
use crate::gui::{SplatRoot, kit};
use gpui::{
    AnyElement, Context, ElementId, InteractiveElement, IntoElement, ParentElement, SharedString,
    Styled, Window, div,
};

/// `Choose for this campaign`, its help, then `L`.
fn use_box(
    id: &'static str,
    checked: bool,
    help: &'static str,
    toggle: impl Fn(&gpui::ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(XS)
        .pb(L)
        .child(kit::Checkbox::new(id, "Choose for this campaign", checked).on_click(toggle))
        .child(kit::help(help))
}

/// The line under an inline form: quiet Cancel (or Clear) at its start, then the reason or the
/// preview of what the commit will make.
fn kept_line(cancel: Option<AnyElement>, words: Option<String>) -> impl IntoElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .min_h(Density::Read.control())
        .children(cancel)
        .children(words.map(|words| kit::text(Type::Small, c.muted, words)))
}

/// The New entry form.
pub(super) fn new_entry(
    root: &mut SplatRoot,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let view = {
        let app = &mut root.app;
        app.library.new_entry_view(&app.campaigns)
    };
    let Some(view) = view else {
        return div().into_any_element();
    };
    let _ = root.app.library.take_focus_new_name();
    let kinds: Vec<String> = view.kinds.iter().map(|(kind, _)| kind.clone()).collect();
    let kind_at = kinds.iter().position(|kind| *kind == view.kind);
    let create = kit::Button::primary("library-create-entry", "Create entry")
        .blocked(view.blocked)
        .on_click(cx.listener(|root, _, _, cx| {
            let app = &mut root.app;
            app.library.create_entry(&app.campaigns);
            cx.notify();
        }));
    let cancel = kit::Button::quiet("library-new-cancel", "Cancel")
        .density(Density::Read)
        .line_start()
        .on_click(cx.listener(|root, _, _, cx| {
            root.app.library.close_new_entry();
            cx.notify();
        }));
    let words = view.blocked.map(str::to_owned).or(view
        .preview
        .as_ref()
        .map(|(_, relative, destination)| format!("Will create {destination}/{relative}")));
    div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(FORM_WIDTH)
        .child(kit::heading(
            kit::Heading::Group {
                title: "New entry",
                count: 0,
            },
            true,
        ))
        .child(kit::text(
            Type::Small,
            c.muted,
            "Creates the entry, then opens it in the editor.",
        ))
        .child(kit::gap(S))
        .child(kit::field(
            "Entry name",
            None,
            None,
            kit::TextField::new("library-new-name", view.name.clone())
                .name("Entry name")
                .on_change(cx.listener(|root, words: &str, _, cx| {
                    root.app.library.set_new_name(words);
                    cx.notify();
                }))
                .on_enter(super::act(cx, |root, _, cx| {
                    let app = &mut root.app;
                    app.library.create_entry(&app.campaigns);
                    cx.notify();
                })),
        ))
        .child(kit::field(
            "Kind",
            None,
            None,
            kit::Dropdown::new(
                "library-new-kind",
                "Kind of entry",
                view.kinds.iter().map(|(_, words)| words.clone()),
                kind_at,
            )
            .on_select(cx.listener(move |root, at: &usize, _, cx| {
                if let Some(kind) = kinds.get(*at) {
                    root.app.library.set_new_kind(kind);
                    cx.notify();
                }
            })),
        ))
        .child(kit::field(
            "Destination",
            None,
            None,
            kit::Dropdown::new(
                "library-new-place",
                "Destination",
                view.places.clone(),
                Some(view.place),
            )
            .on_select(cx.listener(|root, at: &usize, _, cx| {
                let app = &mut root.app;
                app.library.set_new_place(*at, &app.campaigns);
                cx.notify();
            })),
        ))
        .children(view.use_box.map(|checked| {
            use_box(
                "library-new-used",
                checked,
                "Adds this entry to the campaign’s package choices.",
                cx.listener(|root, _, _, cx| {
                    root.app.library.toggle_new_used();
                    cx.notify();
                }),
            )
        }))
        .child(
            div()
                .flex()
                .flex_row()
                .items_end()
                .gap(S)
                .child(
                    div().flex_1().min_w_0().child(
                        kit::field(
                            "Folder",
                            None,
                            None,
                            kit::TextField::new("library-new-folder", view.folder.clone())
                                .name("Folder")
                                .on_change(cx.listener(|root, words: &str, _, cx| {
                                    root.app.library.set_new_folder(words);
                                    cx.notify();
                                })),
                        )
                        .pb(gpui::px(0.0)),
                    ),
                )
                .child(create),
        )
        .child(kit_line_gap())
        .child(kept_line(Some(cancel.into_any_element()), words))
        .into_any_element()
}

fn kit_line_gap() -> impl IntoElement {
    kit::gap(XS)
}

/// Manage packages: add a package folder (or register a copy), make a new package, and the
/// registered packages with their menus.
pub(super) fn package_form(
    root: &mut SplatRoot,
    _window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let view = {
        let app = &mut root.app;
        app.library.package_form_view(&app.campaigns)
    };
    let _ = root.app.library.take_focus_package_name();
    let mut form = div()
        .flex()
        .flex_col()
        .w_full()
        .max_w(FORM_WIDTH)
        .child(kit::gap(L))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .child(
                    div()
                        .flex_1()
                        .child(kit::text(Type::Heading, c.text, "Manage packages")),
                )
                .child(
                    kit::Button::quiet("library-packages-close", "Close")
                        .name("Close the package form")
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.library.close_package_form();
                            cx.notify();
                        })),
                ),
        )
        .child(kit::heading(
            kit::Heading::Group {
                title: "Add a package folder",
                count: 0,
            },
            false,
        ))
        .child(kit::text(Type::Small, c.muted, "Package folder"))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .pt(XS)
                .child(
                    div().flex_1().min_w_0().child(
                        kit::TextField::new("library-package-folder", view.path.clone())
                            .name("Package folder")
                            .on_change(cx.listener(|root, words: &str, _, cx| {
                                root.app.library.set_package_path(words);
                                cx.notify();
                            })),
                    ),
                )
                .child(
                    kit::Button::secondary("library-package-browse", "Browse…")
                        .blocked(view.browse_blocked.clone())
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.library.browse_package_path();
                            cx.notify();
                        })),
                )
                .child(
                    kit::Button::secondary("library-add-folder", "Add folder")
                        .blocked(view.add_blocked.clone())
                        .on_click(cx.listener(|root, _, _, cx| {
                            let app = &mut root.app;
                            app.library.add_package_folder(&mut app.campaigns);
                            cx.notify();
                        })),
                ),
        )
        .child(kept_line(None, view.add_blocked.clone()));
    if let Some(copy) = &view.copy {
        let warning = kit::Message::new(kit::Tone::Warning, copy.warning.clone());
        if copy.naming {
            form = form.child(warning).child(kit::heading(
                kit::Heading::Group {
                    title: "Register a copy",
                    count: 0,
                },
                false,
            ));
            if let Some(chosen) = copy.chosen {
                form = form.child(use_box(
                    "library-copy-used",
                    chosen,
                    copy.help,
                    cx.listener(|root, _, _, cx| {
                        root.app.library.toggle_copy_used();
                        cx.notify();
                    }),
                ));
            }
            let register = if copy.register_primary {
                kit::Button::primary("library-register", "Register")
            } else {
                kit::Button::secondary("library-register", "Register")
            }
            .blocked(copy.register_blocked.clone())
            .on_click(cx.listener(|root, _, _, cx| {
                let app = &mut root.app;
                app.library.register_copied_package(&mut app.campaigns);
                cx.notify();
            }));
            form = form
                .child(kit::text(Type::Small, c.muted, "New package name"))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(S)
                        .pt(XS)
                        .child(
                            div().flex_1().min_w_0().child(
                                kit::TextField::new("library-copy-name", copy.name.clone())
                                    .name("New package name")
                                    .on_change(cx.listener(|root, words: &str, _, cx| {
                                        root.app.library.set_copy_name(words);
                                        cx.notify();
                                    })),
                            ),
                        )
                        .child(register),
                )
                .child(kept_line(
                    Some(
                        kit::Button::quiet("library-copy-cancel", "Cancel")
                            .density(Density::Read)
                            .line_start()
                            .on_click(cx.listener(|root, _, _, cx| {
                                root.app.library.cancel_copy_name();
                                cx.notify();
                            }))
                            .into_any_element(),
                    ),
                    copy.register_blocked.clone(),
                ));
        } else {
            form = form.child(
                warning.action(
                    kit::Button::secondary("library-copy-repair", "Register as a new package")
                        .density(Density::Read)
                        .blocked(copy.repair_blocked.clone())
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.library.open_copy_name();
                            cx.notify();
                        })),
                ),
            );
        }
        form = form
            .child(kit::gap(S))
            .child(kit::text(Type::Small, c.muted, "Technical details"))
            .child(kit::text(Type::Small, c.faint, copy.id.clone()));
    }
    if view.copy.as_ref().is_none_or(|copy| !copy.naming) {
        let create = if view.create_primary {
            kit::Button::primary("library-create-package", "Create package")
        } else {
            kit::Button::secondary("library-create-package", "Create package")
        }
        .blocked(view.create_blocked.clone())
        .on_click(cx.listener(|root, _, _, cx| {
            let app = &mut root.app;
            app.library.create_package(&mut app.campaigns);
            cx.notify();
        }));
        form = form
            .child(kit::heading(
                kit::Heading::Group {
                    title: "New package",
                    count: 0,
                },
                false,
            ))
            .child(kit::field(
                "Name",
                None,
                None,
                kit::TextField::new("library-package-name", view.name.clone())
                    .name("Name")
                    .on_change(cx.listener(|root, words: &str, _, cx| {
                        root.app.library.set_package_name(words);
                        cx.notify();
                    })),
            ))
            .children(view.chosen.map(|chosen| {
                use_box(
                    "library-package-used",
                    chosen,
                    "Uses every entry in this package, including future entries.",
                    cx.listener(|root, _, _, cx| {
                        root.app.library.toggle_package_used();
                        cx.notify();
                    }),
                )
            }))
            .child(kit::text(Type::Small, c.muted, "Folder"))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(S)
                    .pt(XS)
                    .child(
                        div().flex_1().min_w_0().child(
                            kit::TextField::new("library-package-parent", view.folder.clone())
                                .name("Folder")
                                .on_change(cx.listener(|root, words: &str, _, cx| {
                                    let app = &mut root.app;
                                    app.library.set_package_folder(words, &app.campaigns);
                                    cx.notify();
                                })),
                        ),
                    )
                    .child(
                        kit::Button::secondary("library-package-parent-browse", "Browse…")
                            .on_click(cx.listener(|root, _, _, cx| {
                                root.app.library.browse_package_folder();
                                cx.notify();
                            })),
                    )
                    .child(create),
            )
            .child(kept_line(
                view.clear.then(|| {
                    kit::Button::quiet("library-package-clear", "Clear")
                        .density(Density::Read)
                        .line_start()
                        .on_click(cx.listener(|root, _, _, cx| {
                            root.app.library.clear_package();
                            cx.notify();
                        }))
                        .into_any_element()
                }),
                view.create_blocked.clone().or(view.preview.clone()),
            ));
    }
    form = form
        .child(kit::heading(
            kit::Heading::Group {
                title: "Registered packages",
                count: 0,
            },
            false,
        ))
        .child(kit::rule());
    if view.registered.is_empty() {
        form = form.child(kit::text(Type::Body, c.muted, "None yet."));
    }
    for (id, name, path) in &view.registered {
        let id = *id;
        let name: SharedString = name.clone().into();
        form = form
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(S)
                    .py(XS)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .min_w_0()
                            .child(kit::text(Type::Strong, c.text, name.clone()))
                            .child(kit::text(Type::Small, c.muted, path.clone())),
                    )
                    .child(
                        kit::MenuButton::new(
                            ElementId::Name(format!("library-package-menu-{id}").into()),
                            kit::Button::quiet(
                                ElementId::Name(format!("library-package-more-{id}").into()),
                                "",
                            )
                            .icon(crate::gui::icons::EXPAND)
                            .name(format!("More actions for {name}")),
                        )
                        .item(
                            kit::MenuItem::new("Unregister…")
                                .destructive()
                                .blocked(view.unregister_blocked.clone())
                                .on_click(cx.listener(move |root, _, _, cx| {
                                    let app = &mut root.app;
                                    app.library.ask_unregister(id, &app.campaigns);
                                    cx.notify();
                                })),
                        ),
                    ),
            )
            .child(kit::rule());
    }
    div()
        .id("library-package-form")
        .child(form)
        .into_any_element()
}

/// Unregister <package>?: what leaves, who uses it, where the files stay.
pub(super) fn unregister_dialog(
    root: &mut SplatRoot,
    cx: &mut Context<SplatRoot>,
) -> Option<AnyElement> {
    let view = root.app.library.unregister_view(&root.app.campaigns)?;
    let c = colors();
    Some(
        kit::Dialog::new("library-unregister", view.title)
            .child(kit::text(Type::Body, c.text, view.words))
            .children(view.users.map(|users| kit::text(Type::Body, c.text, users)))
            .child(kit::help(view.folder))
            .destructive(
                kit::Button::destructive("library-unregister-confirm", "Unregister").on_click(
                    cx.listener(|root, _, _, cx| {
                        let app = &mut root.app;
                        app.library.confirm_unregister(&mut app.campaigns);
                        cx.notify();
                    }),
                ),
            )
            .action(
                kit::Button::secondary("library-unregister-keep", "Keep the package").on_click(
                    cx.listener(|root, _, _, cx| {
                        root.app.library.keep_package();
                        cx.notify();
                    }),
                ),
            )
            .on_escape(super::act(cx, |root, _, cx| {
                root.app.library.keep_package();
                cx.notify();
            }))
            .into_any_element(),
    )
}
