//! The kit gallery (tests only): every kit part on one page, over the shell, so a screenshot shows
//! page agents how each looks and the tests can drive the parts that keep state (a text field, a
//! menu, a dialog). `Shell::update(|root, ..| root.gallery = Some(Gallery::default()))` shows it.
use super::{SplatRoot, icons, kit, theme};
use gpui::{AnyElement, Context, IntoElement, ParentElement, Styled, Window, div, px};
use theme::{L, Type, colors};

/// What the gallery's parts edit.
#[derive(Default)]
pub(crate) struct Gallery {
    pub name: String,
    pub notes: String,
    pub remember: bool,
    pub sync: bool,
    pub tab: usize,
    pub kind: Option<usize>,
    pub dialog: bool,
    pub pressed: Vec<&'static str>,
}

fn press(
    word: &'static str,
) -> impl Fn(&mut SplatRoot, &gpui::ClickEvent, &mut Window, &mut Context<SplatRoot>) {
    move |root, _, _, cx| {
        if let Some(gallery) = &mut root.gallery {
            gallery.pressed.push(word);
        }
        cx.notify();
    }
}

/// Draw the gallery.
pub(crate) fn render(
    root: &mut SplatRoot,
    _: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let Some(gallery) = &root.gallery else {
        return div().into_any_element();
    };
    let c = colors();
    let row = || {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(theme::S)
    };
    let buttons = row()
        .child(
            kit::Button::primary("g-primary", "Start fight")
                .on_click(cx.listener(press("primary"))),
        )
        .child(kit::Button::secondary("g-secondary", "Edit details").icon(icons::EDIT))
        .child(kit::Button::quiet("g-quiet", "Reload from disk").icon(icons::REFRESH))
        .child(kit::Button::destructive("g-destructive", "Remove"))
        .child(kit::Button::secondary("g-key", "Save").key("Ctrl+S"))
        .child(
            kit::Button::primary("g-blocked", "Connect")
                .blocked(Some("Paste your sign-in cookie first.")),
        )
        .child(kit::IconButton::new(
            "g-more",
            icons::EXPAND,
            "More actions",
        ));
    let fields = row()
        .child(
            div().w(px(280.0)).child(kit::field(
                "Campaign name",
                Some("The name shows in the rail.".into()),
                None,
                kit::TextField::new("g-name", gallery.name.clone())
                    .placeholder("Campaign name")
                    .on_change(cx.listener(|root, words: &str, _, cx| {
                        if let Some(gallery) = &mut root.gallery {
                            gallery.name = words.to_owned();
                        }
                        cx.notify();
                    })),
            )),
        )
        .child(div().w(px(280.0)).child(kit::field(
            "Notes",
            None,
            Some("The note could not be saved.".into()),
            kit::TextEditor::new("g-notes", gallery.notes.clone()).rows(3),
        )))
        .child(
            div().w(px(220.0)).child(kit::field(
                "Kind",
                None,
                None,
                kit::Dropdown::new(
                    "g-kind",
                    "Kind",
                    ["Creature", "Spell", "Item"],
                    gallery.kind,
                )
                .placeholder("Choose a kind")
                .on_select(cx.listener(|root, index: &usize, _, cx| {
                    if let Some(gallery) = &mut root.gallery {
                        gallery.kind = Some(*index);
                    }
                    cx.notify();
                })),
            )),
        );
    let choices = row()
        .child(
            kit::Segmented::new(
                "g-tabs",
                ["Packages", "Rules and runs", "Link and folder"],
                gallery.tab,
            )
            .on_select(cx.listener(|root, index: &usize, _, cx| {
                if let Some(gallery) = &mut root.gallery {
                    gallery.tab = *index;
                }
                cx.notify();
            })),
        )
        .child(
            kit::Checkbox::new("g-remember", "Remember my sign-in", gallery.remember).on_click(
                cx.listener(|root, _, _, cx| {
                    if let Some(gallery) = &mut root.gallery {
                        gallery.remember = !gallery.remember;
                    }
                    cx.notify();
                }),
            ),
        )
        .child(
            kit::Switch::new("g-sync", "Combat sync", gallery.sync).on_click(cx.listener(
                |root, _, _, cx| {
                    if let Some(gallery) = &mut root.gallery {
                        gallery.sync = !gallery.sync;
                    }
                    cx.notify();
                },
            )),
        )
        .child(
            kit::MenuButton::new(
                "g-menu",
                kit::Button::quiet("g-menu-button", "Packages").icon(icons::DOWN),
            )
            .item(
                kit::MenuItem::new("Rename")
                    .icon(icons::EDIT)
                    .on_click(cx.listener(press("rename"))),
            )
            .item(kit::MenuItem::new("Move up").blocked(Some("Already first.")))
            .item(
                kit::MenuItem::new("Remove")
                    .icon(icons::REMOVE)
                    .destructive(),
            ),
        );
    let messages = div()
        .flex()
        .flex_col()
        .gap(theme::S)
        .child(
            kit::Message::new(kit::Tone::Success, "Saved “Bridge Ambush”.")
                .action(kit::Button::quiet("g-undo", "Undo")),
        )
        .child(
            kit::Message::new(
                kit::Tone::Warning,
                "D&D Beyond has not confirmed changes to Mira.",
            )
            .action(kit::Button::secondary("g-keep", "Keep running"))
            .destructive(kit::Button::destructive("g-lose", "Close and lose changes")),
        )
        .child(kit::Message::new(
            kit::Tone::Error,
            "The timeline could not be read.",
        ))
        .child(kit::Message::new(kit::Tone::Info, "Sync is off.").small())
        .child(
            row()
                .child(kit::status(&kit::Status::new(
                    icons::DRAFT,
                    "Library draft",
                    c.document.unsaved,
                )))
                .child(kit::status_link(
                    "g-status",
                    kit::Status::new(icons::WARN, "Session folder not found", c.result.warning),
                )),
        );
    let lists = div()
        .w(px(320.0))
        .flex()
        .flex_col()
        .child(kit::label("Campaigns", 3))
        .child(kit::two_line_row(
            "g-row-1",
            "The Emberfall Marches",
            "Linked to D&D Beyond",
            true,
        ))
        .child(kit::two_line_row(
            "g-row-2",
            "Quiet Harbor",
            "Not linked",
            false,
        ))
        .child(kit::Row::new("g-row-3", "Hollow Keep").child(kit::text(
            Type::Body,
            c.text,
            "Hollow Keep",
        )));
    let markdown = div().w(px(420.0)).child(kit::markdown(
        "g-markdown",
        "## Ash Exposure\n\nA creature **without** a face covering makes a *DC 12* save, or gains [Exhaustion](exhaustion.md).\n\n- one level\n- `1d4` hours\n\n| Level | Effect |\n|---|---|\n| 1 | −2 to d20 tests |\n",
    ));
    let mut page = div()
        .flex()
        .flex_col()
        .gap(L)
        .child(kit::page_header(
            "Kit gallery",
            Some("every part".into()),
            vec![
                kit::Button::secondary("g-open-dialog", "Open dialog")
                    .on_click(cx.listener(|root, _, _, cx| {
                        if let Some(gallery) = &mut root.gallery {
                            gallery.dialog = true;
                        }
                        cx.notify();
                    }))
                    .into_any_element(),
            ],
        ))
        .child(buttons)
        .child(fields)
        .child(choices)
        .child(
            div()
                .flex()
                .flex_row()
                .gap(theme::XL)
                .child(messages)
                .child(lists)
                .child(markdown),
        )
        .child(kit::empty_state(
            icons::CAMPAIGN,
            "Nothing planned yet",
            "Plan a fight to see it here.",
            None::<AnyElement>,
        ));
    if gallery.dialog {
        let close = cx.listener(|root, _, _, cx| {
            if let Some(gallery) = &mut root.gallery {
                gallery.dialog = false;
            }
            cx.notify();
        });
        page = page.child(
            kit::Dialog::new("g-dialog", "Unsaved changes")
                .child(kit::text(
                    Type::Body,
                    c.text,
                    "“Ash Exposure” in the Ashen Marches package has unsaved changes.",
                ))
                .child(kit::text(
                    Type::Small,
                    c.muted,
                    "Save and close, keep editing, or close and lose these changes.",
                ))
                .destructive(kit::Button::destructive("g-discard", "Discard and close"))
                .action(kit::Button::secondary("g-keep-editing", "Keep editing").on_click(close))
                .action(kit::Button::primary("g-save", "Save and close"))
                .on_escape({
                    let root = cx.entity().downgrade();
                    move |_, cx| {
                        let _ = root.update(cx, |root, cx| {
                            if let Some(gallery) = &mut root.gallery {
                                gallery.dialog = false;
                            }
                            cx.notify();
                        });
                    }
                }),
        );
    }
    kit::scroll_column("g-scroll", page).into_any_element()
}
