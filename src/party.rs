//! Character viewer with optional combat synchronization.
use crate::{auth::Session, party_sync::Reader};
use eframe::egui;
use serde_json::Value;
use splat::character::{ABILITIES, Cache, EntitySnapshot, now};
use std::time::Duration;
#[derive(Default)]
pub struct Party {
    reader: Reader,
    page: usize,
    search: String,
    combat: crate::combat_sync::CombatSync,
    epoch: u64,
    content_entry: Option<(String, Result<splat::content::Entry, String>)>,
}
impl Party {
    pub fn sync(
        &mut self,
        ctx: &egui::Context,
        epoch: u64,
        campaign: Option<(uuid::Uuid, String, std::path::PathBuf)>,
        connected: bool,
        visible: bool,
    ) {
        self.epoch = epoch;
        if !visible || !connected {
            self.combat.stop();
        }
        self.reader.sync(ctx, epoch, campaign, connected, visible);
    }
    pub fn show(&mut self, ui: &mut egui::Ui, session: Option<Session>, linked: bool) {
        ui.visuals_mut().override_text_color = Some(egui::Color32::from_gray(205));
        ui.visuals_mut().weak_text_color = Some(egui::Color32::from_gray(165));
        ui.style_mut().spacing.scroll = egui::style::ScrollStyle::solid();
        self.reader.tick(session.clone(), ui.ctx());
        ui.horizontal(|ui| {
            ui.heading("Party");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(
                        session.is_some() && linked && !self.reader.loading(),
                        egui::Button::new("Refresh party"),
                    )
                    .clicked()
                    && let Some(session) = session.clone()
                {
                    self.reader.refresh(session, ui.ctx(), true);
                }
            });
        });
        if !linked {
            ui.label("Link this campaign to D&D Beyond in Settings to see its party.");
            return;
        }
        let cached = self
            .reader
            .selected
            .as_ref()
            .and_then(|id| self.reader.sheets.get(id));
        let checked = self.reader.selected.as_ref().is_some_and(|id| {
            self.reader.online_ids.contains(id) && !self.reader.errors.contains_key(id)
        }) && self.reader.error.is_none();
        let status = if session.is_none() {
            "Offline"
        } else if self.reader.loading() {
            "Refreshing"
        } else if !checked {
            "Connected · Cached sheet"
        } else if self.reader.stream_open {
            "Live updates"
        } else {
            "Connected · Checking for updates"
        };
        ui.horizontal_wrapped(|ui| {
            if self.reader.loading() {
                ui.spinner();
            }
            let age = cached
                .map(|record| {
                    format!(
                        " · Fetched {}",
                        fetch_age(now().saturating_sub(record.source.fetched_at))
                    )
                })
                .unwrap_or_default();
            ui.weak(format!("{status}{age}"))
                .on_hover_text(if session.is_none() {
                    "Showing saved sheets. Connect in Settings to get updates."
                } else {
                    "Sheets refresh automatically. Use Refresh party to request the latest data."
                });
        });
        if let Some(e) = &self.reader.error {
            ui.colored_label(egui::Color32::LIGHT_RED, e);
        }
        if let Some(e) = &self.reader.warning {
            ui.colored_label(egui::Color32::LIGHT_YELLOW, format!("Local storage: {e}"));
        }
        ui.ctx().request_repaint_after(Duration::from_secs(1));
        let mut selected = self.reader.selected.clone();
        ui.horizontal_wrapped(|ui| {
            for m in &self.reader.members {
                let record = self.reader.sheets.get(&m.id);
                let level = record.map(|r| r.character.level).or(m.level);
                let classes = record
                    .map(|r| {
                        r.character
                            .classes
                            .iter()
                            .filter_map(|c| c["name"].as_str())
                            .collect::<Vec<_>>()
                            .join(" / ")
                    })
                    .unwrap_or_default();
                let summary = [
                    level.map(|n| format!("Level {n}")).unwrap_or_default(),
                    classes,
                ]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join(" · ");
                let mut label = egui::text::LayoutJob::default();
                label.append(
                    &m.name,
                    0.0,
                    egui::TextFormat {
                        font_id: egui::FontId::proportional(18.0),
                        color: ui.visuals().strong_text_color(),
                        ..Default::default()
                    },
                );
                if !summary.is_empty() {
                    label.append(
                        &format!("\n{summary}"),
                        0.0,
                        egui::TextFormat {
                            font_id: egui::FontId::proportional(13.0),
                            color: ui.visuals().weak_text_color(),
                            ..Default::default()
                        },
                    );
                }
                if ui
                    .add(
                        egui::Button::new(label)
                            .selected(selected.as_ref() == Some(&m.id))
                            .min_size(egui::vec2(190.0, 58.0)),
                    )
                    .clicked()
                {
                    selected = Some(m.id.clone());
                    self.search.clear();
                }
            }
        });
        if selected != self.reader.selected
            && let Some(id) = selected
        {
            self.reader.select(id);
        }
        ui.separator();
        let Some(id) = &self.reader.selected else {
            if self.reader.loaded {
                ui.label("This campaign has no party members.");
            } else if !self.reader.loading() {
                ui.label("No local characters yet. Connect to load this party.");
            }
            return;
        };
        if let Some(error) = self.reader.errors.get(id) {
            ui.colored_label(egui::Color32::LIGHT_RED, error);
        }
        if let Some(record) = self.reader.sheets.get(id) {
            self.combat.tick(
                ui.ctx(),
                session.clone(),
                self.epoch,
                &record.source.campaign_id,
                id,
            );
        }
        if self.combat.updated {
            if let Some(record) = self.combat.record().cloned() {
                if let Some(scope) = self.reader.local_scope()
                    && let Err(error) = Cache::user().and_then(|cache| cache.save(scope, &record))
                {
                    self.reader.warning = Some(error);
                }
                if let Some(path) = self.reader.campaign_path()
                    && let Err(error) = record.save_campaign_copy(path)
                {
                    self.reader.warning = Some(error);
                }
                self.reader.sheets.insert(id.clone(), record);
            }
            self.combat.updated = false;
        }
        if session.is_some() {
            egui::CollapsingHeader::new("Combat sync")
                .id_salt("party_combat_sync")
                .show(ui, |ui| self.combat.show(ui));
        }
        let Some(record) = self.combat.record().or_else(|| self.reader.sheets.get(id)) else {
            if !self.reader.loading() {
                ui.label("No local copy is available for this character.");
            }
            return;
        };
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(&record.definition.splat_name)
                    .size(30.0)
                    .strong()
                    .color(ui.visuals().strong_text_color()),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.hyperlink_to(
                    "Open on D&D Beyond ↗",
                    format!("https://www.dndbeyond.com/characters/{id}"),
                );
            });
        });
        ui.label(
            egui::RichText::new(format!(
                "{} · {} · Level {}",
                record.character.species,
                record
                    .character
                    .classes
                    .iter()
                    .map(|c| text(&c["name"]))
                    .collect::<Vec<_>>()
                    .join(" / "),
                record.character.level
            ))
            .size(17.0),
        );
        ui.horizontal_wrapped(|ui| {
            for (i, name) in [
                "Overview",
                "Actions",
                "Spells",
                "Equipment",
                "Features",
                "Source & parity",
            ]
            .iter()
            .enumerate()
            {
                if ui
                    .add(egui::Button::new(*name).selected(self.page == i))
                    .clicked()
                {
                    self.page = i;
                    self.search.clear();
                }
            }
        });
        ui.separator();
        if (1..5).contains(&self.page) {
            ui.horizontal(|ui| {
                ui.label("Filter");
                ui.add(egui::TextEdit::singleline(&mut self.search).hint_text("Filter entries…"));
                if !self.search.is_empty() && ui.small_button("Clear").clicked() {
                    self.search.clear();
                }
            });
        }
        if self.page == 5
            && self
                .content_entry
                .as_ref()
                .is_none_or(|(hash, _)| hash != &record.source.content_hash)
        {
            self.content_entry = Some((record.source.content_hash.clone(), record.entry()));
        }
        egui::ScrollArea::vertical().scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible).id_salt((id,self.page)).auto_shrink([false,false]).show(ui,|ui|{
            if self.page==0{overview(ui,record);}else if self.page==5{
                ui.heading("Source & parity");ui.label("Displayed totals come directly from D&D Beyond’s computed-character service. Splat does not add modifiers to these totals.");
                for message in &record.coverage{ui.label(message);}
                if let Some((_, Ok(entry))) = &self.content_entry {
                    campaign_option_status(ui, entry);
                    ui.label(format!("Campaign content: {} inventory rows, {} action definitions.", entry.inventory.len(), entry.fields.get("splat_actions").and_then(serde_yaml_ng::Value::as_mapping).map_or(0, |v| v.len())));
                    if let Some(import) = entry.fields.get("splat_import") {
                        let conversion_count = import.get("gaps").and_then(serde_yaml_ng::Value::as_sequence).map_or(0, Vec::len);
                        let engine_count = import.get("engine_gaps").and_then(serde_yaml_ng::Value::as_sequence).map_or(0, Vec::len);
                        ui.label(format!("{conversion_count} conversion gaps; {engine_count} engine adapters still needed."));
                        ui.collapsing("Rule conversion gaps", |ui| {
                            if let Some(gaps) = import.get("gaps").and_then(serde_yaml_ng::Value::as_sequence) {
                                for gap in gaps { if let Some(reason) = gap.get("reason").and_then(serde_yaml_ng::Value::as_str) { ui.label(reason); } }
                            }
                        });
                        ui.collapsing("Required character or encounter inputs", |ui| {
                            if let Some(inputs) = import.get("required_inputs").and_then(serde_yaml_ng::Value::as_sequence) {
                                for input in inputs { if let Some(reason) = input.get("reason").and_then(serde_yaml_ng::Value::as_str) { ui.label(reason); } }
                            }
                        });
                    }
                }
                ui.label(format!("Local creature UUID: {}",record.definition.splat_id));ui.label(format!("Source character: {} · Campaign: {}",record.source.character_id,record.source.campaign_id));
                ui.label(format!("Computed response fetched at {} UTC epoch seconds; source details at {}.",record.source.fetched_at,record.source.raw_fetched_at));
                if let Some(modified)=&record.source.source_modified{ui.label(format!("Beyond source modification: {modified}"));}
                ui.label(format!("Preserved {} source fields and {} computed fields.",record.source.raw.as_object().map_or(0,|o|o.len()),record.source.computed.as_object().map_or(0,|o|o.len())));
                ui.label("A cache read or a successful fetch does not prove that the two Beyond services represent the same edit. The next check can reconcile delayed results.");
                if let Some(root) = self.reader.campaign_path() { ui.label(format!("Campaign copy: {}",record.campaign_path(root).display())); }
                if let Some(scope)=self.reader.local_scope() && let Ok(cache)=Cache::user() && let Ok(path)=cache.path(scope,&record.source.campaign_id,&record.source.character_id){ui.label(format!("Local copy: {}",path.display()));}
            }else{details(ui,record,self.page,&self.search);}
        });
    }
}
fn campaign_option_status(ui: &mut egui::Ui, entry: &splat::content::Entry) {
    use splat::content::character::{CampaignOptionState, Sheet};
    let Some(value) = entry.fields.get("splat_character") else {
        return;
    };
    let Ok(sheet) = serde_yaml_ng::from_value::<Sheet>(value.clone()) else {
        return;
    };
    ui.collapsing("Optional character rules", |ui| {
        for (key, name) in [("dark_bargain", "Dark Bargain"), ("character_threads", "Character Threads"), ("runestones", "Runestones")] {
            match sheet.campaign_options.get(key) {
                Some(CampaignOptionState::Inactive) => { ui.label(format!("{name}: inactive (not granted on the sheet).")); }
                Some(CampaignOptionState::Unselected) => { ui.label(format!("{name}: no effect selected. No effect imported.")); }
                Some(CampaignOptionState::Unknown) => { ui.label(format!("{name}: source selection data is unavailable.")); }
                Some(CampaignOptionState::Active { selections }) => {
                    for selected in selections {
                        let status = if selected.rule.is_some() { "converted" } else { "needs rule conversion" };
                        ui.label(format!("{name}: {} ({status}).", selected.name));
                    }
                }
                None => (),
            }
        }
        ui.weak("Selections follow the D&D Beyond sheet. Active choices can need further details. Simulation support is separate.");
    });
}
fn fetch_age(seconds: u64) -> String {
    match seconds {
        0..60 => "just now".into(),
        60..3600 => format!("{} min ago", seconds / 60),
        3600..86400 => format!("{} h ago", seconds / 3600),
        86400..172800 => "1 day ago".into(),
        _ => format!("{} days ago", seconds / 86400),
    }
}
fn text(v: &Value) -> &str {
    v.as_str().unwrap_or("—")
}
fn array(v: &Value) -> &[Value] {
    v.as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn section_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(12.0);
    ui.label(
        egui::RichText::new(title)
            .size(21.0)
            .strong()
            .color(ui.visuals().strong_text_color()),
    );
    ui.separator();
    ui.add_space(3.0);
}
fn subsection_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(6.0);
    ui.label(
        egui::RichText::new(title)
            .size(17.0)
            .strong()
            .color(ui.visuals().strong_text_color()),
    );
    ui.add_space(2.0);
}
fn metric(ui: &mut egui::Ui, label: &str, value: impl ToString) {
    egui::Frame::group(ui.style())
        .inner_margin(12.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.set_min_width(82.0);
                ui.weak(label);
                ui.label(egui::RichText::new(value.to_string()).size(23.0).strong());
            });
        });
}
fn val(record: &EntitySnapshot, key: &str) -> String {
    record
        .stat(key)
        .map(|n| n.to_string())
        .unwrap_or("—".into())
}
fn signed(record: &EntitySnapshot, key: &str) -> String {
    record
        .stat(key)
        .map(|n| format!("{n:+}"))
        .unwrap_or("—".into())
}
pub(crate) fn overview(ui: &mut egui::Ui, record: &EntitySnapshot) {
    section_heading(ui, "Combat");
    ui.horizontal_wrapped(|ui| {
        metric(
            ui,
            "Hit points",
            format!("{} / {}", record.initial.hp, val(record, "max_hp")),
        );
        metric(ui, "Armor class", val(record, "armor_class"));
        metric(ui, "Temp HP", record.initial.temporary_hp);
        metric(ui, "Proficiency", signed(record, "proficiency_bonus"));
        metric(ui, "Initiative", signed(record, "initiative"));
        metric(
            ui,
            "Inspiration",
            if record.initial.inspiration {
                "Yes"
            } else {
                "No"
            },
        );
    });
    section_heading(ui, "Ability scores");
    ui.horizontal_wrapped(|ui| {
        for ability in ABILITIES {
            metric(
                ui,
                &format!("{} · {}", ability[..3].to_uppercase(), val(record, ability)),
                signed(record, &format!("{ability}_modifier")),
            );
        }
    });
    section_heading(ui, "Awareness & movement");
    ui.horizontal_wrapped(|ui| {
        for (label, key) in [
            ("Passive perception", "passive_perception"),
            ("Passive insight", "passive_insight"),
            ("Passive investigation", "passive_investigation"),
        ] {
            metric(ui, label, val(record, key));
        }
        for (key, stat) in &record.definition.splat_stats {
            if let Some(mode) = key.strip_prefix("speed_") {
                metric(ui, mode, format!("{} ft", stat.value));
            }
        }
    });
    section_heading(ui, "Saving throws & skills");
    ui.columns(3, |cols| {
        subsection_heading(&mut cols[0], "Saving throws");
        egui::Grid::new("saves").show(&mut cols[0], |ui| {
            for ability in ABILITIES {
                ui.label(signed(record, &format!("{ability}_save")));
                let mut name = ability.to_string();
                name[..1].make_ascii_uppercase();
                ui.label(name);
                ui.end_row();
            }
        });
        for (column, col) in cols.iter_mut().enumerate().skip(1) {
            subsection_heading(col, "Skills");
            egui::Grid::new(("skills", column)).show(col, |ui| {
                for skill in record
                    .character
                    .skills
                    .iter()
                    .skip((column - 1) * 9)
                    .take(if column == 1 { 9 } else { usize::MAX })
                {
                    ui.label(signed(record, &skill.stat));
                    let flags = format!(
                        "{}{}{}",
                        if skill.expertise {
                            " (E)"
                        } else if skill.proficient {
                            " (P)"
                        } else if skill.half_proficient {
                            " (½)"
                        } else {
                            ""
                        },
                        if skill.advantage { " Adv" } else { "" },
                        if skill.disadvantage { " Dis" } else { "" }
                    );
                    ui.label(format!("{}{flags}", skill.name));
                    ui.end_row();
                }
            });
        }
    });
    ui.weak("P: proficient · E: expertise · ½: half proficiency · Adv/Dis: advantage/disadvantage");
    section_heading(ui, "Resources");
    if record.definition.splat_resources.is_empty() {
        ui.label("No limited-use pools returned.");
    }
    for (key, pool) in &record.definition.splat_resources {
        ui.label(format!(
            "{} · {} / {} remaining",
            pool.name,
            record
                .initial
                .resources
                .get(key)
                .map(|v| v.to_string())
                .unwrap_or("—".into()),
            pool.max
        ));
    }
    ui.add_space(10.0);
    ui.columns(2, |cols| {
        subsection_heading(&mut cols[0], "Proficiencies & languages");
        for group in &record.character.proficiencies {
            let values = text(&group["values"]);
            if !values.is_empty() {
                cols[0].label(format!("{}: {values}", text(&group["group"])));
            }
        }
        subsection_heading(&mut cols[1], "Defenses & senses");
        for (label, values) in [
            ("Resistance", &record.character.resistances),
            ("Immunity", &record.character.immunities),
            ("Vulnerability", &record.character.vulnerabilities),
        ] {
            for value in values {
                cols[1].label(format!("{label}: {}", text(&value["name"])));
            }
        }
        for sense in &record.character.senses {
            cols[1].label(format!(
                "{} · {}",
                text(&sense["name"]),
                text(&sense["distance"])
            ));
        }
        subsection_heading(&mut cols[1], "Death saves");
        cols[1].label(format!(
            "{} successes · {} failures",
            record.initial.death_saves.successes, record.initial.death_saves.failures
        ));
        subsection_heading(&mut cols[1], "Conditions");
        if record.initial.conditions.is_empty() {
            cols[1].label("None");
        }
        for condition in &record.initial.conditions {
            cols[1].label(
                condition["name"]
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| condition.to_string()),
            );
        }
    });
}
pub(crate) fn details(ui: &mut egui::Ui, record: &EntitySnapshot, page: usize, search: &str) {
    let d = &record.source.raw;
    let mut entries: Vec<(String, String, String, Option<u64>)> = Vec::new();
    let mut add = |v: &Value, source: &str| {
        let def = if v["definition"].is_object() {
            &v["definition"]
        } else {
            v
        };
        let Some(name) = def["name"].as_str() else {
            return;
        };
        let desc = def["description"]
            .as_str()
            .or_else(|| def["snippet"].as_str())
            .unwrap_or("No description supplied.");
        let source = match source {
            "race" => "Species",
            "class" => "Class",
            "feat" => "Feat",
            other => other,
        };
        let meta = match page {
            3 => format!(
                "Qty {} · {}{}",
                v["quantity"].as_u64().unwrap_or(1),
                if v["equipped"] == true {
                    "Equipped"
                } else {
                    "Carried"
                },
                if v["isAttuned"] == true {
                    " · Attuned"
                } else {
                    ""
                }
            ),
            2 => format!(
                "{} · {}{}",
                source,
                def["level"]
                    .as_u64()
                    .map(|n| if n == 0 {
                        "Cantrip".into()
                    } else {
                        format!("Level {n}")
                    })
                    .unwrap_or_default(),
                if v["prepared"] == true || v["alwaysPrepared"] == true {
                    " · Prepared"
                } else {
                    ""
                }
            ),
            _ => source.into(),
        };
        let mut extra = Vec::new();
        if let Some(dice) = def["damage"]["diceString"].as_str() {
            extra.push(format!("Printed damage: {dice}"));
        }
        if let Some(ac) = def["armorClass"].as_u64() {
            extra.push(format!("Armor value: {ac}"));
        }
        if page == 2 {
            if let Some(range) = def["range"]["rangeValue"].as_u64() {
                extra.push(if range == 0 {
                    format!(
                        "Range: {}",
                        def["range"]["origin"].as_str().unwrap_or("see description")
                    )
                } else {
                    format!("Range: {range} ft")
                });
            }
            if def["concentration"] == true {
                extra.push("Concentration".into());
            }
            if def["ritual"] == true {
                extra.push("Ritual".into());
            }
            if let Some(time) = def["activation"]["activationTime"].as_u64() {
                let unit = match def["activation"]["activationType"].as_u64() {
                    Some(1) => "action",
                    Some(3) => "bonus action",
                    Some(4) => "reaction",
                    Some(6) => "minute",
                    Some(7) => "hour",
                    _ => "(see description)",
                };
                extra.push(format!("Casting time: {time} {unit}"));
            }
        }
        let meta = if extra.is_empty() {
            meta
        } else {
            format!("{meta} · {}", extra.join(" · "))
        };
        entries.push((
            name.into(),
            meta,
            splat::character::description_text(desc),
            def["level"].as_u64(),
        ));
    };
    match page {
        1 => {
            section_heading(ui, "Attacks");
            for attack in &record.character.attacks {
                let value = &attack["damage"]["value"];
                let damage = if let Some(n) = value.as_i64() {
                    n.to_string()
                } else {
                    format!(
                        "{}{}",
                        text(&value["diceString"]),
                        value["fixedValue"]
                            .as_i64()
                            .filter(|n| *n != 0)
                            .map(|n| format!("{n:+}"))
                            .unwrap_or_default()
                    )
                };
                let hit = attack["toHit"]
                    .as_i64()
                    .map(|n| format!("{n:+} to hit"))
                    .unwrap_or_else(|| {
                        format!(
                            "Save DC {} {}",
                            attack["saveInfo"]["value"],
                            text(&attack["saveInfo"]["label"])
                        )
                    });
                if text(&attack["name"])
                    .to_lowercase()
                    .contains(&search.to_lowercase())
                {
                    ui.label(format!(
                        "{} · {hit} · {damage} {}",
                        text(&attack["name"]),
                        text(&attack["damage"]["type"])
                    ));
                }
            }
            ui.add_space(10.0);
            section_heading(ui, "Action descriptions");
            for (source, values) in d["actions"].as_object().into_iter().flat_map(|o| o.iter()) {
                for v in array(values) {
                    add(v, source);
                }
            }
            for v in array(&d["customActions"]) {
                add(v, "Custom action");
            }
        }
        2 => {
            section_heading(ui, "Spellcasting");
            ui.weak("Spell descriptions use their printed values. Upcasting and character-specific formulas are not evaluated.");
            for (label, key) in [
                ("Spell save DC", "saveDcs"),
                ("Spell attack", "spellAttacks"),
                ("Casting modifier", "modifiers"),
            ] {
                for value in array(&record.character.casting[key]) {
                    ui.label(format!(
                        "{label}: {} · {}",
                        value["value"],
                        array(&value["sources"])
                            .iter()
                            .filter_map(Value::as_str)
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
            for (key, pool) in &record.definition.splat_resources {
                if pool.kind.as_deref() != Some("spell_slot") {
                    continue;
                }
                ui.label(format!(
                    "{} · {} / {} remaining",
                    pool.name,
                    record
                        .initial
                        .resources
                        .get(key)
                        .map(|v| v.to_string())
                        .unwrap_or("—".into()),
                    pool.max
                ));
            }
            for (source, values) in d["spells"].as_object().into_iter().flat_map(|o| o.iter()) {
                for v in array(values) {
                    add(v, source);
                }
            }
            for class in array(&d["classSpells"]) {
                for v in array(&class["spells"]) {
                    add(v, "Class spell");
                }
            }
        }
        3 => {
            section_heading(ui, "Equipment & coins");
            for v in array(&d["inventory"]) {
                add(v, "");
            }
            for v in array(&d["customItems"]) {
                add(v, "Custom item");
            }
            ui.label(format!(
                "Coins · {} cp · {} sp · {} ep · {} gp · {} pp",
                d["currencies"]["cp"],
                d["currencies"]["sp"],
                d["currencies"]["ep"],
                d["currencies"]["gp"],
                d["currencies"]["pp"]
            ));
        }
        _ => {
            section_heading(ui, "Features & traits");
            for v in array(&d["race"]["racialTraits"]) {
                add(v, "Species");
            }
            for class in array(&d["classes"]) {
                for v in array(&class["classFeatures"]) {
                    if v["definition"]["requiredLevel"].as_u64().unwrap_or(0)
                        <= class["level"].as_u64().unwrap_or(0)
                    {
                        add(v, text(&class["definition"]["name"]));
                    }
                }
            }
            for v in array(&d["feats"]) {
                add(v, "Feat");
            }
        }
    }
    entries.sort_by(|a, b| {
        if page == 2 {
            a.3.cmp(&b.3).then_with(|| a.0.cmp(&b.0))
        } else {
            a.0.cmp(&b.0)
        }
    });
    if page != 3 && page != 1 {
        entries.dedup();
    }
    let search = search.to_lowercase();
    let filtered: Vec<_> = entries
        .iter()
        .filter(|(name, source, _, _)| format!("{name} {source}").to_lowercase().contains(&search))
        .collect();
    ui.weak(format!("{} entries", filtered.len()));
    if filtered.is_empty() {
        ui.label(if search.is_empty() {
            "No entries returned for this character."
        } else {
            "No entries match this filter."
        });
    }
    let mut last_level = None;
    for (i, (name, meta, description, level)) in filtered.into_iter().enumerate() {
        if page == 2 && (i == 0 || last_level != *level) {
            section_heading(
                ui,
                &match level {
                    Some(0) => "Cantrips".to_owned(),
                    Some(n) => format!("Level {n} spells"),
                    None => "Other spells".to_owned(),
                },
            );
            last_level = *level;
        }
        let response = egui::CollapsingHeader::new(
            egui::RichText::new(name)
                .size(17.0)
                .strong()
                .color(ui.visuals().strong_text_color()),
        )
        .id_salt((page, i, name))
        .show(ui, |ui| {
            ui.set_max_width(ui.available_width().min(760.0));
            ui.weak(meta);
            ui.add_space(6.0);
            for paragraph in description.split("\n\n") {
                ui.add(egui::Label::new(paragraph).wrap());
                ui.add_space(5.0);
            }
        });
        if response.body_returned.is_none() {
            ui.horizontal_wrapped(|ui| {
                ui.add_space(20.0);
                ui.weak(meta);
            });
        }
        ui.add_space(4.0);
    }
}
