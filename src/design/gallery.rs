//! Every token and component, in each state, for the `gallery` visual scenario.
use super::{reader as style, *};
use eframe::egui::{self, Color32};

fn swatch(ui: &mut egui::Ui, name: &str, color: Color32) {
    // One allocation for both, so the parent's wrapping cannot part a swatch from its name.
    const SWATCH: egui::Vec2 = egui::vec2(28.0, 16.0);
    let muted = colors().muted;
    let galley = ui
        .painter()
        .layout_no_wrap(name.to_owned(), Type::Small.font(ui), muted);
    let width = SWATCH.x + XS + galley.size().x + S;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, SWATCH.y), egui::Sense::hover());
    let block = egui::Rect::from_min_size(rect.min, SWATCH);
    ui.painter().rect(
        block,
        2.0,
        color,
        egui::Stroke::new(HAIRLINE_WIDTH, colors().rule),
        egui::StrokeKind::Inside,
    );
    let text = egui::pos2(block.right() + XS, rect.center().y - galley.size().y / 2.0);
    ui.painter().galley(text, galley, muted);
}

fn swatches(ui: &mut egui::Ui, title: &str, list: &[(&str, Color32)]) {
    design_group(ui, title);
    ui.horizontal_wrapped(|ui| {
        for (name, color) in list {
            swatch(ui, name, *color);
        }
    });
}

fn design_group(ui: &mut egui::Ui, title: &str) {
    ui.add_space(XS);
    group_heading(ui, title, 0);
}

pub fn tokens(ui: &mut egui::Ui) {
    let c = colors();
    swatches(
        ui,
        "Surfaces and lines",
        &[
            ("page", c.page),
            ("rail", c.rail),
            ("raised", c.raised),
            ("well", c.well),
            ("hover", c.hover),
            ("selected", c.selected),
            ("current_row", c.current_row),
            ("hairline", c.hairline),
            ("rule", c.rule),
            ("field_border", c.field_border),
        ],
    );
    swatches(
        ui,
        "Text and accent",
        &[
            ("text", c.text),
            ("muted", c.muted),
            ("faint", c.faint),
            ("disabled", c.disabled),
            ("accent", c.accent),
            ("accent_fill", c.accent_fill),
            ("focus", c.focus),
            ("destructive", c.destructive),
            ("recording", c.recording),
        ],
    );
    swatches(
        ui,
        "HP and participant",
        &[
            ("healthy", c.hp.healthy),
            ("damaged", c.hp.damaged),
            ("bloodied", c.hp.bloodied),
            ("zero", c.hp.zero),
            ("temp", c.hp.temp),
            ("unknown", c.hp.unknown),
            ("track", c.hp.track),
            ("absent", c.participant.absent),
            ("concentrating", c.participant.concentrating),
            ("condition", c.participant.condition),
            ("changed", c.participant.changed),
        ],
    );
    swatches(
        ui,
        "Side, sync, connection, document, result",
        &[
            ("party", c.side.party),
            ("enemies", c.side.enemies),
            ("other", c.side.other),
            ("matching", c.sync.matching),
            ("pending", c.sync.pending),
            ("conflict", c.sync.conflict),
            ("error", c.sync.error),
            ("connected", c.connection.connected),
            ("offline", c.connection.offline),
            ("unsaved", c.document.unsaved),
            ("saved", c.document.saved),
            ("invalid", c.document.invalid),
            ("external", c.document.external),
            ("warning", c.result.warning),
            ("success", c.result.success),
        ],
    );
    design_group(ui, "Type scale");
    for role in Type::ALL {
        let sample = match role {
            Type::Label => "LABEL 0123456789",
            Type::Numeral | Type::NumeralLarge => "0123456789 26/33 +5",
            _ => "Sphinx of black quartz × 2 – ½ → 0Oo1lI",
        };
        ui.horizontal(|ui| {
            ui.add_sized(
                [86.0, role.line()],
                egui::Label::new(Type::Small.rich(ui, role.name()).color(colors().muted)),
            );
            ui.label(role.rich(ui, sample).color(colors().text));
        });
    }
}

pub fn components(ui: &mut egui::Ui) {
    let c = colors();
    page_header(ui, "Encounters", Some("2 ready · 1 in progress"), |ui| {
        let _ = Action::secondary("New encounter").icon(icons::ADD).show(ui);
        let _ = Action::quiet("Reload from disk")
            .icon(icons::REFRESH)
            .show(ui);
    });
    design_group(ui, "Buttons: enabled, blocked");
    for (kind, label) in [
        (0, "Primary"),
        (1, "Secondary"),
        (2, "Quiet"),
        (3, "Destructive"),
    ] {
        let action = |label| match kind {
            0 => Action::primary(label),
            1 => Action::secondary(label),
            2 => Action::quiet(label),
            _ => Action::destructive(label),
        };
        ui.horizontal_top(|ui| {
            for (state, sample) in [
                ("With icon", 0),
                ("Enabled", 1),
                ("Read density", 2),
                ("Blocked", 3),
            ] {
                ui.vertical(|ui| {
                    ui.set_max_width(if sample == 3 { 260.0 } else { 140.0 });
                    ui.label(Type::Small.rich(ui, state).color(c.muted));
                    match sample {
                        0 => {
                            let _ = action(label).icon(icons::ADD).show(ui);
                        }
                        1 => {
                            let _ = action(label).show(ui);
                        }
                        2 => {
                            let _ = action(label).density(Density::Read).show(ui);
                        }
                        _ => {
                            ui.horizontal(|ui| {
                                blocked_reason(ui, Some("Wait for the save."));
                                let _ = action(label).blocked(Some("Wait for the save.")).show(ui);
                            });
                        }
                    }
                });
            }
        });
    }
    ui.horizontal(|ui| {
        let _ = icon_button(ui, icons::CLOSE, "Close");
        let _ = link(ui, "A link");
        let _ = small_link(ui, "A small link");
        let mut choice = 1;
        segmented(ui, &mut choice, &[(0, "Read"), (1, "Edit"), (2, "Log")]);
        let _ = split_action(ui, Action::secondary("Split"), "Other ways", |ui| {
            let _ = ui.selectable_label(true, "The default");
            let _ = ui.selectable_label(false, "Another way");
        });
    });
    ui.horizontal(|ui| {
        let _ = Action::quiet("Absent")
            .icon(icons::ABSENT)
            .selected(true)
            .show(ui);
    });
    design_group(ui, "Fields");
    let mut text = "Bridge Ambush".to_owned();
    let mut empty = String::new();
    let mut amount = "7".to_owned();
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_max_width(240.0);
            ui.label(Type::Small.rich(ui, "Under a group heading").color(c.muted));
            small_text_field(ui, "Campaign name", &mut text, None, None);
            ui.label(Type::Small.rich(ui, "Alone on a form").color(c.muted));
            text_field(ui, "Field", &mut text, Some("Help text."), None);
            text_field(
                ui,
                "Field with error",
                &mut empty,
                None,
                Some("State the fix."),
            );
        });
        ui.vertical(|ui| {
            search_field(ui, "Search", &mut empty, "Search…", 200.0);
            number_field(ui, "Amount", &mut amount, "Amount", 64.0, 30.0);
            let mut quantity = 3;
            stepper(ui, "Quantity", &mut quantity, 0..=99);
        });
    });
    design_group(ui, "Rows and status");
    let _ = two_line_row(ui, 40.0, false, "A list row", "Second line");
    let _ = two_line_row(ui, 40.0, true, "A selected row", "Second line");
    ui.horizontal_wrapped(|ui| {
        for (icon, word, color) in [
            (icons::OK, "Matching", c.sync.matching),
            (icons::REFRESH, "Pending", c.sync.pending),
            (icons::WARN, "Conflict", c.sync.conflict),
            (icons::ERROR, "Error", c.sync.error),
            (icons::READ_ONLY, "Read only", c.sync.read_only),
            (icons::OFFLINE, "Offline", c.connection.offline),
            (icons::DRAFT, "Unsaved", c.document.unsaved),
        ] {
            status(ui, Status { icon, word, color });
        }
    });
    ui.scope(|ui| {
        ui.set_max_width(ui.available_width().min(LIST_MEASURE));
        group_heading(ui, "In progress", 1);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(Type::Name.rich(ui, "Bridge Ambush"));
                let _ = status_link(
                    ui,
                    Status {
                        icon: icons::RUN,
                        word: "Round 2 · Paused",
                        color: c.muted,
                    },
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                row_menu(ui, "Paused Bridge Ambush", |ui| {
                    let _ = Action::quiet("Start another fight").show(ui);
                });
                let _ = Action::quiet("Edit").show(ui);
                let _ = Action::secondary("Resume").show(ui);
            });
        });
        group_heading(ui, "Ready to run", 2);
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(Type::Name.rich(ui, "Bridge Ambush"));
                ui.label(
                    Type::Small
                        .rich(ui, "2 wolves · Captain Varra")
                        .color(c.muted),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                row_menu(ui, "Bridge Ambush", |ui| {
                    let _ = ui.button("Remove…");
                });
                let _ = Action::quiet("Edit").show(ui);
                let _ = Action::secondary("Start fight").show(ui);
            });
        });
    });
    design_group(ui, "One cause, one sentence");
    ui.horizontal(|ui| {
        status(
            ui,
            Status {
                icon: icons::DRAFT,
                word: "Unsaved",
                color: c.document.unsaved,
            },
        );
        blocked_reason(ui, Some("Save or discard to change packages."));
    });
    ui.horizontal(|ui| {
        let _ = Action::secondary("Packages")
            .blocked(Some("Save or discard to change packages."))
            .show(ui);
        ui.add_space(L);
        blocked_reason(ui, Some("Paste your session cookie first."));
        let _ = Action::secondary("Connect")
            .blocked(Some("Paste your session cookie first."))
            .show(ui);
    });
    design_group(ui, "Save rows");
    ui.push_id("clean_save_row", |ui| {
        ui.horizontal(|ui| {
            let _ = save_row_state(
                ui,
                Status {
                    icon: icons::OK,
                    word: "No changes",
                    color: c.muted,
                },
                false,
                None,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let _ = Action::secondary("Save")
                    .blocked(Some("No changes to save."))
                    .show(ui);
            });
        });
    });
    ui.push_id("dirty_save_row", |ui| {
        ui.horizontal(|ui| {
            let _ = save_row_state(
                ui,
                Status {
                    icon: icons::DRAFT,
                    word: "Unsaved",
                    color: c.document.unsaved,
                },
                true,
                None,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let _ = Action::primary("Save").show(ui);
            });
        });
    });
    let _ = draft_discarded(ui, "Fireball");
    use crate::timeline::Kind;
    hanging_add_strip(
        ui,
        |ui| {
            for (word, icon) in [("Fight", icons::ENCOUNTERS), ("Group", icons::GROUP)] {
                let _ = Action::quiet(word)
                    .icon(icon)
                    .density(Density::Read)
                    .show(ui);
            }
        },
        [
            Kind::Day,
            Kind::LongRest,
            Kind::ShortRest,
            Kind::Travel,
            Kind::Event,
        ]
        .iter()
        .map(|kind| {
            Action::quiet(kind.word())
                .icon(kind.icon())
                .density(Density::Read)
        })
        .collect(),
        &[1],
    );
    // The submenu keeps inside the gallery's pane, as the encounter editor's does.
    let inside = ui.max_rect().intersect(ui.clip_rect());
    let menu = Action::quiet("Group actions").show(ui);
    egui::Popup::menu(&menu).show(|ui| {
        menu_submenu(ui, "Move to…", inside, |ui| {
            let _ = Action::quiet("Bridge Ambush")
                .density(Density::Read)
                .show(ui);
        });
        let _ = Action::quiet("Make available to any encounter")
            .density(Density::Read)
            .show(ui);
        let _ = Action::quiet("Delete group…")
            .density(Density::Read)
            .show(ui);
    });
    design_group(ui, "Paused runs");
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(RAIL_WIDTH);
            let _ = paused_run_nav(ui, 1, "Bridge Ambush", "Round 2");
            let _ = paused_run_nav(ui, 2, "", "");
        });
    });
    design_group(ui, "HP");
    for (hp, max, temp) in [
        (Some(33), Some(33), 0),
        (Some(26), Some(33), 5),
        (Some(11), Some(41), 0),
        (Some(0), Some(62), 0),
        (None, None, 0),
    ] {
        let (rect, _) = ui.allocate_exact_size(egui::vec2(200.0, 26.0), egui::Sense::hover());
        hp_readout(
            ui,
            ui.painter(),
            rect.left() + 90.0,
            rect.top() + 12.0,
            hp,
            max,
            &HpFonts::standard(ui),
        );
        hp_bar(
            ui.painter(),
            egui::Rect::from_min_size(
                rect.left_top() + egui::vec2(100.0, 8.0),
                egui::vec2(90.0, 3.0),
            ),
            hp,
            max,
            temp,
        );
        if temp > 0 {
            ui.painter().text(
                rect.left_top() + egui::vec2(200.0, 12.0),
                egui::Align2::LEFT_CENTER,
                temp_hp_text(temp),
                Type::Numeral.font(ui),
                colors().hp.temp,
            );
        }
    }
    ui.horizontal(|ui| {
        stat_tile(ui, "Armor class", "15");
        stat_tile(ui, "Initiative", "+2");
    });
    design_group(ui, "Empty state");
    empty_state(
        ui,
        icons::CAMPAIGN,
        "Nothing here yet",
        "What this place is, and the one next step.",
        |ui| {
            let _ = Action::primary("Next step").show(ui);
        },
    );
}

pub fn messages(ui: &mut egui::Ui) {
    design_group(ui, "Inline messages");
    message(ui, Tone::Info, "Info: nothing is wrong.");
    message(ui, Tone::Success, "Saved.");
    message(ui, Tone::Warning, "This changed on disk.");
    message(ui, Tone::Error, "This could not be saved. State the fix.");
    ui.scope(|ui| {
        ui.set_max_width(READER_MEASURE / 2.0);
        let _ = message_with_action(
            ui,
            Tone::Info,
            "Party HP is full after the long rest.",
            "Use the saved HP",
        );
    });
    design_group(ui, "Sections");
    let _ = section(ui, "A section", |ui| {
        ui.label("Its contents.");
    });
    let _ = quiet_section(ui, "Technical details", |ui| {
        ui.label("Its contents.");
    });
    design_group(ui, "Mono, for authoring");
    ui.label(Type::Mono.rich(ui, "splat_kind: creature"));
    design_group(ui, "Reader");
    style::measure(ui, |ui| {
        style::title(ui, "Entry title");
        style::subtitle(ui, "Spell · Level 3 · D&D 2024");
        style::identity(ui, "Level 3 Evocation (Sorcerer, Wizard)");
        style::facts(
            ui,
            &[
                ("Casting Time".to_owned(), "Action".to_owned()),
                ("Range".to_owned(), "150 feet".to_owned()),
            ],
        );
        style::section_heading(ui, "Section heading");
        style::markdown(
            ui,
            "| Ability | Score | Modifier |\n| --- | --- | --- |\n| Strength | 18 | +4 |\n| Dexterity | 10 | +0 |\n",
        );
        style::markdown(
            ui,
            "Rules prose in the serif, at a comfortable measure. **A named action.** It reads as a page, with _emphasis_ and 2d6 + 3 damage × ½ → done.",
        );
    });
}

#[derive(Clone, Copy, Default, PartialEq)]
#[repr(u8)]
pub enum Sheet {
    #[default]
    Tokens,
    Components,
    Messages,
    Markdown,
}

/// One note where it is typed and where it is read, to judge the two together.
const NOTE: &str = "# Session 4: The Old Mill

2026-11-02, The Emberfall Marches

## Plan for tonight

*How does the session begin? Start close to the action.*

## What happened

We reached *the old mill* at dusk. **Oren** lit the lamps, and ***nobody*** spoke. The road behind us was covered in ash, and it stayed that way until the wolves came down from the ridge.

- Roll `1d20 + 5` for the bridge
- [[Timeline#^t-1a2b3c4d5e|Bridge Ambush]] left two wolves
- ~~Kestrel has the key~~ The key is lost
- 

> The miller said the road was covered in ash.

### Later that night

See [the map](https://example.com) for the road north.

```
Reward: 40 gp
```
";

pub fn markdown(ui: &mut egui::Ui) {
    ui.columns(2, |columns| {
        design_group(&mut columns[0], "Write");
        let mut note = NOTE;
        let mut layouter = crate::markdown_source::layouter;
        let output = egui::TextEdit::multiline(&mut note)
            .margin(egui::vec2(S, S))
            .layouter(&mut layouter)
            .desired_width(f32::INFINITY)
            .show(&mut columns[0]);
        crate::markdown_source::paint_section_rules(&columns[0], &output);
        named(&columns[0], &output.response, "Sample note");
        design_group(&mut columns[1], "Read");
        // The Reader gets a reference as its words, from the timeline.
        let read = NOTE.replace("[[Timeline#^t-1a2b3c4d5e|Bridge Ambush]]", "Bridge Ambush");
        crate::markdown_view::show_reader(&mut columns[1], &read);
    });
}

pub fn show(ui: &mut egui::Ui) {
    let id = egui::Id::new("gallery_sheet");
    let mut sheet = ui
        .data(|data| data.get_temp::<Sheet>(id))
        .unwrap_or_default();
    segmented(
        ui,
        &mut sheet,
        &[
            (Sheet::Tokens, "Tokens"),
            (Sheet::Components, "Components"),
            (Sheet::Messages, "Messages"),
            (Sheet::Markdown, "Markdown"),
        ],
    );
    ui.data_mut(|data| data.insert_temp(id, sheet));
    egui::ScrollArea::vertical()
        .id_salt(("gallery", sheet as u8))
        .auto_shrink([false, false])
        .show(ui, |ui| match sheet {
            Sheet::Tokens => tokens(ui),
            Sheet::Components => components(ui),
            Sheet::Messages => messages(ui),
            Sheet::Markdown => markdown(ui),
        });
}
