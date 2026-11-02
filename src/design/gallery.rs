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
    ui.label(Type::Heading.rich(ui, title).color(colors().text));
    rule(ui);
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
        ui.horizontal(|ui| {
            let _ = action(label).icon(icons::ADD).show(ui);
            let _ = action(label).show(ui);
            let _ = action(label).density(Density::Read).show(ui);
            let _ = action(label)
                .blocked(Some("Blocked for a stated reason."))
                .show(ui);
        });
    }
    ui.horizontal(|ui| {
        let _ = icon_button(ui, icons::CLOSE, "Close");
        let _ = link(ui, "A link");
        let mut choice = 1;
        segmented(ui, &mut choice, &[(0, "Read"), (1, "Edit"), (2, "Log")]);
    });
    design_group(ui, "Fields");
    let mut text = "Bridge Ambush".to_owned();
    let mut empty = String::new();
    let mut amount = "7".to_owned();
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_max_width(240.0);
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
        });
    });
    design_group(ui, "Rows and status");
    let _ = two_line_row(ui, 40.0, false, "A list row", "Second line");
    let _ = two_line_row(ui, 40.0, true, "A selected row", "Second line");
    ui.horizontal_wrapped(|ui| {
        for (icon, word, color) in [
            (icons::OK, "Synced", c.sync.matching),
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
        style::subtitle(ui, "Spell · Level 3 · Built-in");
        style::identity(ui, "Medium humanoid, unaligned");
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
        });
}
