use super::{
    Entry, Mapping, Value, formats,
    layout::{Document, replace_cell},
    patch_fields,
};

fn note(kind: &str, data: &str, body: &str) -> String {
    format!(
        "---\nsplat_schema: 1\nsplat_id: 00000000-0000-0000-0000-000000000001\nsplat_kind: {kind}\nsplat_name: Rook\n{data}\n---\n\n{body}"
    )
}

fn model(text: &str) -> Mapping {
    let mut fields = (*Entry::parse("Rook.md", text).unwrap().fields).clone();
    fields.remove("splat_schema");
    fields
}

#[test]
fn fmt_f1_world_golden_pair_and_obsidian_properties_rewrite() {
    let old = note(
        "world",
        "splat_world: {type: npc, status: alive, creature: {key: 'srd:creature/tough'}, in: '[[Undercroft]]', aliases: [The Veteran]}",
        "# Rook\n",
    );
    let new = old.replace("splat_schema: 1", "splat_schema: 2").replace("splat_world: {type: npc, status: alive, creature: {key: 'srd:creature/tough'}, in: '[[Undercroft]]', aliases: [The Veteran]}", "splat_type: npc\nsplat_status: alive\nsplat_stat_block: srd:creature/tough\nsplat_in: '[[Undercroft]]'\naliases: [The Veteran]");
    assert_eq!(model(&old), model(&new));
    let obsidian = new
        .replace(
            "splat_type: npc\nsplat_status: alive",
            "\"splat_status\": \"alive\"\n'splat_type': npc",
        )
        .replace("aliases: [The Veteran]", "aliases:\n  - The Veteran");
    assert_eq!(model(&new), model(&obsidian));
    let edited = patch_fields(&obsidian, &[("splat_status", Some("dead".into()))]).unwrap();
    assert_eq!(edited, obsidian.replace("\"alive\"", "dead"));
}

#[test]
fn fmt_f1_provenance_mechanics_and_package_golden_pairs() {
    let old = note(
        "rule",
        "splat_rule: {}\nsplat_origin: {key: 'srd:rules/test'}\nsplat_sources: [{title: Book, locator: 'p. 3'}]\nsplat_requires: [{key: 'srd:rules/other'}]",
        "# Rook\n",
    );
    let new = note("rule", "splat_origin: srd:rules/test\nsplat_sources: ['Book, p. 3']\nsplat_requires: [srd:rules/other]", "# Rook\n").replace("splat_schema: 1", "splat_schema: 2");
    assert_eq!(model(&old), model(&new));
    let old = note(
        "package",
        "splat_namespace: rook\nsplat_reference: {pages: [1, 2, 3]}\nsplat_import_routes: {items: 'pack:items'}",
        "# Rook\n",
    );
    let new = note("package", "splat_namespace: rook\nsplat_pages: 1-3", "# Rook\n\n## Splat data\n\n<!-- splat:yaml path=splat_import_routes -->\n\n```yaml\nitems: pack:items\n```\n").replace("splat_schema: 1", "splat_schema: 2");
    assert_eq!(model(&old), model(&new));
}

#[test]
fn fmt_f1_conversion_round_trip_and_idempotence_for_small_kinds() {
    for old in [
        note(
            "world",
            "splat_world: {type: npc, status: alive, aliases: [Rook]}",
            "# Rook\n\nPersonal prose.\n",
        ),
        note(
            "rule",
            "splat_rule: {}\nsplat_origin: {key: 'srd:rules/test'}\nsplat_sources: [{title: Book, locator: 'p. 3', url: 'https://example.com'}]\nsplat_requires: [{key: 'srd:rules/other'}]",
            "# Rule\n",
        ),
        note(
            "package",
            "splat_namespace: rook\nsplat_import_routes: {items: 'pack:items'}\nsplat_reference: {pages: [1, 2, 3]}",
            "# Package\n",
        ),
        note("effect", "splat_effect: {}", "# Effect\n"),
    ] {
        let converted = formats::convert("Rook.md", &old).unwrap();
        assert_eq!(model(&old), model(&converted));
        assert_eq!(converted, formats::convert("Rook.md", &converted).unwrap());
        assert_eq!(
            Entry::parse("Rook.md", &old).unwrap().fields["splat_schema"],
            Value::from(1)
        );
    }
}

#[test]
fn fmt_f1_machine_kinds_and_character_have_independent_golden_pairs() {
    for kind in ["effect", "sequence", "strategy", "seed", "rule_group"] {
        let key = format!("splat_{kind}");
        let old = note(kind, &format!("{key}: {{}}"), "# Rook\n");
        let new = note(
            kind,
            "",
            &format!(
                "# Rook\n\n## Splat data\n\n<!-- splat:yaml path={key} -->\n\n```yaml\n{{}}\n```\n"
            ),
        )
        .replace("splat_schema: 1", "splat_schema: 2");
        assert_eq!(model(&old), model(&new), "{kind}");
        let converted = formats::convert("Rook.md", &old).unwrap();
        assert_eq!(model(&new), model(&converted));
        assert_eq!(converted, formats::convert("Rook.md", &converted).unwrap());
    }
    let sheet = super::character::Sheet {
        level: 1,
        ..Default::default()
    };
    let fields = serde_yaml_ng::to_string(&Mapping::from_iter([
        ("splat_creature".into(), Value::Mapping(Mapping::new())),
        (
            "splat_character".into(),
            serde_yaml_ng::to_value(sheet).unwrap(),
        ),
    ]))
    .unwrap();
    let old = note("creature", &fields, "# Rook\n");
    let new = note("creature", "", &format!("# Rook\n\n## Splat data (from D&D Beyond; do not edit)\n\n<!-- splat:yaml path=splat_payload -->\n\n```yaml\n{fields}```\n")).replace("splat_schema: 1", "splat_schema: 2");
    assert_eq!(model(&old), model(&new));
    assert_eq!(
        model(&new),
        model(&formats::convert("Rook.md", &old).unwrap())
    );
}

#[test]
fn fmt_f1_bundled_files_parse_within_budget() {
    let started = std::time::Instant::now();
    let (entries, errors) = super::read_directory(std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/builtin"
    )))
    .unwrap();
    assert!(errors.is_empty(), "{errors:?}");
    assert!(entries.len() > 3500, "{} bundled notes", entries.len());
    assert!(
        started.elapsed() < std::time::Duration::from_secs(30),
        "{:?}",
        started.elapsed()
    );
}

#[test]
fn fmt_f1_flat_scalar_edit_preserves_position_comment_and_all_noise() {
    let old = note(
        "world",
        "# before\nsplat_world: {type: npc}\nsplat_status: alive # keep\ntags:\n- dnd\n# after",
        "# Rook\n\n%% keep %%\nCustom prose.  \n",
    )
    .replace('\n', "\r\n");
    let mut edited = old.clone();
    for status in ["dead", "missing", "here", "away", "alive"] {
        let before = edited.clone();
        let previous: Mapping =
            serde_yaml_ng::from_str(before.split("---\r\n").nth(1).unwrap()).unwrap();
        let value = previous["splat_status"].as_str().unwrap();
        edited = patch_fields(&before, &[("splat_status", Some(status.into()))]).unwrap();
        assert_eq!(
            edited,
            before.replace(
                &format!("splat_status: {value}"),
                &format!("splat_status: {status}")
            )
        );
    }
    assert_eq!(old, edited);
}

#[test]
fn fmt_f1_one_edit_one_cell_and_noise_survives() {
    let text = "# Wolf\r\n\r\n```md\r\n## Stat block\r\n| Stat | Value |\r\n|---|---|\r\n| HP | wrong |\r\n```\r\n\r\n## Stat block\r\n\r\nUnknown prose. %% keep %%\r\n\r\n| Stat   | Value |\r\n|:-------|------:|\r\n| **HP** | 22 (4d8 + 4)  |\r\n| Custom | [[Note|Alias]] and a \\| pipe |\r\n\r\n## Other\r\n\r\nKeep this.\r\n";
    let mut edited = text.to_string();
    for hp in ["23", "24", "25", "26", "22"] {
        let table = Document::new(&edited)
            .section("STAT BLOCK")
            .unwrap()
            .table(&["stat", "value"])
            .unwrap();
        let old = table.row("hp").unwrap()[1].value.clone();
        let value = format!("{hp} (4d8 + 4)");
        let after =
            replace_cell(&edited, "Stat block", &["Stat", "Value"], "hp", 1, &value).unwrap();
        assert_eq!(
            after,
            edited.replacen(&format!("| {old}  |"), &format!("| {value}  |"), 1)
        );
        edited = after;
    }
    assert_eq!(edited, text);
    assert!(
        replace_cell(
            text,
            "Stat block",
            &["Stat", "Value"],
            "HP",
            1,
            "4 | injected"
        )
        .is_err()
    );
}

#[test]
fn fmt_f1_malformed_rows_stay_with_line_flags_and_first_row_wins() {
    let text = "## Stat block\n\n| Stat | Value |\n|---|---|\n| HP | about twenty |\n| HP | 22 |\n| AC | 13 |\n| Speed | 30 | extra |\n";
    let table = Document::new(text)
        .section("Stat block")
        .unwrap()
        .table(&["Stat", "Value"])
        .unwrap();
    let mut flags = Vec::new();
    let parse = |v: &str| {
        v.parse::<i64>()
            .map_err(|_| "Hit Points needs a number first".into())
    };
    assert_eq!(table.read("HP", 1, &mut flags, parse), None);
    assert_eq!(table.read("AC", 1, &mut flags, parse), Some(13));
    assert_eq!(table.read("Speed", 1, &mut flags, parse), None);
    assert_eq!(flags.iter().map(|f| f.line).collect::<Vec<_>>(), [5, 6, 8]);
    assert_eq!(table.row("hp").unwrap()[1].value, "about twenty");
}

#[test]
fn fmt_f1_malformed_flat_property_stays_and_note_loads() {
    let text = note(
        "world",
        "splat_type: npc\nsplat_status: [alive]\naliases: [Rook]",
        "# Rook\n",
    )
    .replace("splat_schema: 1", "splat_schema: 2");
    let entry = Entry::parse("Rook.md", &text).unwrap();
    assert_eq!(entry.flags.len(), 1);
    assert_eq!(entry.flags[0].line, 7);
    assert!(entry.fields["splat_world"].get("status").is_none());
    assert_eq!(
        entry.fields["splat_world"]["aliases"][0],
        Value::from("Rook")
    );
    assert!(text.contains("splat_status: [alive]"));
    let bad_type = text.replace("splat_type: npc", "splat_type: dragon");
    let entry = Entry::parse("Rook.md", &bad_type).unwrap();
    assert_eq!(entry.flags.len(), 2);
    assert!(entry.fields["splat_world"].get("type").is_none());
}

#[test]
fn fmt_f1_model_edits_keep_flat_references_and_end_fences() {
    let old = note(
        "rule",
        "splat_rule: {enabled: true}\nsplat_requires: [{key: 'pack:rules/one'}]",
        "# Rook\n\n%% Keep %%\n",
    );
    let text = formats::convert("Rook.md", &old)
        .unwrap()
        .replace('\n', "\r\n");
    let required: Value = serde_yaml_ng::from_str("[{key: 'pack:rules/two'}]").unwrap();
    let rule: Value = serde_yaml_ng::from_str("{enabled: false}").unwrap();
    let after = formats::patch_model(
        &text,
        &[
            ("splat_requires", Some(required)),
            ("splat_rule", Some(rule)),
        ],
    )
    .unwrap();
    assert_eq!(
        after,
        text.replace("pack:rules/one", "pack:rules/two")
            .replace("enabled: true", "enabled: false")
    );
    assert_eq!(Entry::parse("Rook.md", &after).unwrap().flags, []);
    let quoted = old.replace("splat_rule:", "\"splat_rule\":");
    let converted = formats::convert("Rook.md", &quoted).unwrap();
    assert!(!converted.contains("\"splat_rule\":"));
    assert_eq!(model(&quoted), model(&converted));
    let empty = formats::convert("Rook.md", &note("rule", "splat_rule: {}", "# Rook\n")).unwrap();
    assert!(!empty.contains("path=splat_rule"));
    let updated = formats::patch_model(
        &empty,
        &[(
            "splat_rule",
            Some(serde_yaml_ng::from_str("{enabled: true}").unwrap()),
        )],
    )
    .unwrap();
    assert!(updated.contains("<!-- splat:yaml path=splat_rule -->"));
    assert!(
        !updated
            .split("\n---\n")
            .next()
            .unwrap()
            .contains("splat_rule:")
    );
}

#[test]
fn fmt_f1_world_conversion_resolves_ids_to_readable_links_once() {
    let target = Entry::parse(
        "Locations/Undercroft.md",
        &note("world", "splat_world: {type: location}", "# Undercroft\n")
            .replace("splat_name: Rook", "splat_name: Undercroft"),
    )
    .unwrap();
    let old = note(
        "world",
        &format!("splat_world: {{type: npc, in: {{id: {}}}}}", target.id),
        "# Rook\n",
    );
    let after = formats::convert_resolved("Rook.md", &old, [&target]).unwrap();
    assert!(after.contains("[[Locations/Undercroft|Undercroft]]"));
    assert_eq!(
        after,
        formats::convert_resolved("Rook.md", &after, [&target]).unwrap()
    );
    let parsed = Entry::parse("Rook.md", &after).unwrap();
    let raw = parsed.fields["splat_world"]["in"].as_str().unwrap();
    assert_eq!(formats::resolve_readable(raw, [&target]), Ok(target.id));
}

#[test]
fn fmt_f1_line_editor_checks_the_complete_model() {
    let text = "status: alive\nother: keep\n";
    let parse = |text: &str| serde_yaml_ng::from_str(text).map_err(|e| e.to_string());
    assert_eq!(
        super::layout::replace_value(text, 8..13, "dead", &["status"], "dead".into(), parse)
            .unwrap(),
        "status: dead\nother: keep\n"
    );
    assert!(
        super::layout::replace_value(
            text,
            8..13,
            "dead\nextra: bad",
            &["status"],
            "dead".into(),
            parse
        )
        .is_err()
    );
}

#[test]
fn fmt_f3_campaign_golden_toggle_noise_and_bad_rows() {
    let id = "00000000-0000-0000-0000-000000000002";
    let old = note(
        "campaign",
        &format!(
            "splat_namespace: rook\nsplat_run_storage: {{directory: Runs}}\nsplat_packages: [{{name: Core, package: {{id: {id}}}, selection: {{mode: all}}}}]\nsplat_rule_choices: {{'dnd2024:rules/combat_initiative': {{id: {id}}}}}"
        ),
        "# Rook\n\nDM prose.\n",
    );
    let new=note("campaign", "splat_namespace: rook\nsplat_run_folder: Runs",&format!("# Rook\n\nDM prose.\n\n## Packages\n\n- [x] Core ^{id}\n\n## Rule choices\n\n| Rule | Choice |\n| --- | --- |\n| Combat initiative | Side initiative %% splat rule=dnd2024:rules/combat_initiative id={id} %% |\n")).replace("splat_schema: 1","splat_schema: 2");
    assert_eq!(model(&old), model(&new));
    let converted = formats::convert("Campaign.md", &old).unwrap();
    assert_eq!(model(&old), model(&converted));
    assert!(converted.contains("DM prose."));
    assert_eq!(
        converted,
        formats::convert("Campaign.md", &converted).unwrap()
    );
    let off =
        formats::patch_model(&new, &[("splat_packages", Some(Value::Sequence(vec![])))]).unwrap();
    assert_eq!(off, new.replace("[x]", "[ ]"));
    let on = formats::patch_model(
        &off,
        &[(
            "splat_packages",
            Some(model(&old)["splat_packages"].clone()),
        )],
    )
    .unwrap();
    assert_eq!(on, new);
    let bad = new
        .replace("- [x] Core", "- [q] Core")
        .replace(&format!("id={id}"), "id=oops");
    let entry = Entry::parse("Campaign.md", &bad).unwrap();
    assert_eq!(entry.flags.len(), 2);
    assert!(
        entry.fields["splat_packages"]
            .as_sequence()
            .unwrap()
            .is_empty()
    );
    assert!(
        entry.fields["splat_rule_choices"]
            .as_mapping()
            .unwrap()
            .is_empty()
    );
    let renamed = formats::patch_model(&bad, &[("splat_name", Some("New name".into()))]).unwrap();
    assert_eq!(
        renamed,
        bad.replace("splat_name: Rook", "splat_name: New name")
    );
}

#[test]
fn fmt_f3_group_golden_one_cell_noise_and_bad_changes() {
    let key = "dnd2024:creatures/goblin_warrior";
    let old = note(
        "encounter_group",
        &format!(
            "splat_encounter_group: {{side: enemies, members: [{{creature: {{key: '{key}'}}, count: 2, overrides: {{name: Scout, max_hp: {{add: 5}}, note: Quiet}}}}]}}"
        ),
        "# Rook\n\nDM prose.\n",
    );
    let new=note("encounter_group","splat_side: enemies",&format!("# Rook\n\nDM prose.\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 2 | [Goblin](splat:{key}) | Scout | HP +5 | Quiet |\n")).replace("splat_schema: 1","splat_schema: 2");
    assert_eq!(model(&old), model(&new));
    let converted = formats::convert("Group.md", &old).unwrap();
    assert_eq!(model(&old), model(&converted));
    assert_eq!(converted, formats::convert("Group.md", &converted).unwrap());
    let mut group = model(&new)["splat_encounter_group"].clone();
    group["members"][0]["overrides"]["max_hp"]["add"] = 10.into();
    let edited = formats::patch_model(&new, &[("splat_encounter_group", Some(group))]).unwrap();
    assert_eq!(edited, new.replace("HP +5", "HP +10"));
    let bad = format!("{new}| 1 | [Goblin](splat:{key}) | | HP lots | Keep this |\n");
    let entry = Entry::parse("Group.md", &bad).unwrap();
    assert_eq!(entry.flags.len(), 1);
    assert!(entry.flags[0].message.contains("Changes"));
    assert_eq!(
        entry.fields["splat_encounter_group"]["members"]
            .as_sequence()
            .unwrap()
            .len(),
        1
    );
    let renamed = formats::patch_model(&bad, &[("splat_name", Some("New name".into()))]).unwrap();
    assert!(renamed.contains("HP lots | Keep this"));
}

#[test]
fn fmt_f3_roster_golden_one_cell_noise_and_bad_rows() {
    let key = "dnd2024:creatures/goblin_warrior";
    let old = note(
        "encounter",
        &format!(
            "splat_encounter: {{roster: [{{key: actor_one, creature: {{key: '{key}'}}, side: enemies, controller: {{kind: manual, role: gm}}, overrides: {{max_hp: {{add: 5}}}}}}], sides: {{party: {{hostile_to: [enemies]}}, enemies: {{hostile_to: [party]}}}}, assumptions: [], completion: {{mode: manual}}, limits: {{rounds: 20, resolution_events: 10000}}}}"
        ),
        "# Rook\n\nDM prose.\n",
    );
    let new=note("encounter","",&format!("# Rook\n\nDM prose.\n\n## Roster\n\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| enemies | [Goblin](splat:{key}) | | HP +5 | %% splat slot=actor_one %% |\n")).replace("splat_schema: 1","splat_schema: 2");
    assert_eq!(model(&old), model(&new));
    let converted = formats::convert("Encounter.md", &old).unwrap();
    assert_eq!(model(&old), model(&converted));
    assert_eq!(
        converted,
        formats::convert("Encounter.md", &converted).unwrap()
    );
    let mut encounter = model(&new)["splat_encounter"].clone();
    encounter["roster"][0]["overrides"]["max_hp"]["add"] = 10.into();
    let edited = formats::patch_model(&new, &[("splat_encounter", Some(encounter))]).unwrap();
    assert_eq!(edited, new.replace("HP +5", "HP +10"));
    let bad = format!(
        "{new}| enemies | [Goblin](splat:{key}) | | HP lots | %% splat slot=actor_two %% |\n"
    );
    let entry = Entry::parse("Encounter.md", &bad).unwrap();
    assert_eq!(entry.flags.len(), 1);
    assert_eq!(
        entry.fields["splat_encounter"]["roster"]
            .as_sequence()
            .unwrap()
            .len(),
        1
    );
    let renamed = formats::patch_model(&bad, &[("splat_name", Some("New name".into()))]).unwrap();
    assert!(renamed.contains("HP lots"));
}

#[test]
fn fmt_f3_roster_custom_sides_party_slots_and_controller_round_trip() {
    let old = note(
        "encounter",
        "splat_encounter: {roster: [{key: actor_party, party_slot: lead, side: Rivals, controller: {kind: manual, role: player}, initial: {hp: 12}, overrides: {}}], sides: {Rivals: {hostile_to: [party]}, party: {hostile_to: [Rivals]}}, assumptions: [], completion: {mode: manual}, limits: {rounds: 30, resolution_events: 10000}}",
        "DM prose.\n",
    );
    let new = formats::convert("Encounter.md", &old).unwrap();
    assert!(new.contains("splat_sides:"));
    assert!(new.contains("Rivals (player) |  |"));
    assert_eq!(model(&old), model(&new));
    let mut setup = model(&new)["splat_encounter"].clone();
    setup["roster"][0]["initial"]["hp"] = 13.into();
    let edited = formats::patch_model(&new, &[("splat_encounter", Some(setup.clone()))]).unwrap();
    assert_eq!(model(&edited)["splat_encounter"], setup);
    assert!(edited.contains("DM prose."));
}

#[test]
fn fmt_f3_table_append_without_final_newline_and_remove_preserve_noise() {
    let old = note(
        "encounter_group",
        "splat_encounter_group: {side: enemies, members: [{creature: {key: 'dnd2024:creatures/goblin_warrior'}}]}",
        "DM prose.\n",
    );
    let new = formats::convert("Group.md", &old).unwrap();
    let new = new.trim_end();
    let mut group = model(new)["splat_encounter_group"].clone();
    let mut member = group["members"][0].clone();
    member["count"] = 3.into();
    group["members"].as_sequence_mut().unwrap().push(member);
    let appended =
        formats::patch_model(new, &[("splat_encounter_group", Some(group.clone()))]).unwrap();
    assert_eq!(model(&appended)["splat_encounter_group"], group);
    group["members"].as_sequence_mut().unwrap().remove(0);
    let removed =
        formats::patch_model(&appended, &[("splat_encounter_group", Some(group.clone()))]).unwrap();
    assert_eq!(model(&removed)["splat_encounter_group"], group);
    assert!(removed.contains("DM prose."));
}

#[test]
fn fmt_f3_campaign_toggle_ignores_unrelated_tasks_and_rule_edit_is_one_cell() {
    let id = "00000000-0000-0000-0000-000000000002";
    let other = "00000000-0000-0000-0000-000000000003";
    let old = note(
        "campaign",
        &format!(
            "splat_namespace: rook\nsplat_packages: [{{name: Core, package: {{id: {id}}}, selection: {{mode: all}}}}]\nsplat_rule_choices: {{'dnd2024:rules/combat_initiative': {{id: {id}}}}}"
        ),
        &format!("## DM tasks\n\n- [x] Ask players ^{id}\n\nDM prose.\n"),
    );
    let new = formats::convert("Campaign.md", &old)
        .unwrap()
        .replace('\n', "\r\n");
    let edited = formats::patch_model(
        &new,
        &[("splat_packages", Some(Value::Sequence(Vec::new())))],
    )
    .unwrap();
    assert_eq!(
        edited,
        new.replace(&format!("- [x] Core ^{id}"), &format!("- [ ] Core ^{id}"))
    );
    let mut rules = model(&new)["splat_rule_choices"].clone();
    rules["dnd2024:rules/combat_initiative"]["id"] = other.into();
    let edited = formats::patch_model(&new, &[("splat_rule_choices", Some(rules))]).unwrap();
    assert_eq!(
        edited,
        new.replace(
            &format!("{id} %% splat rule=dnd2024:rules/combat_initiative id={id}"),
            &format!("{other} %% splat rule=dnd2024:rules/combat_initiative id={other}")
        )
    );
}

#[test]
fn fmt_f3_campaign_name_only_package_gets_id_on_save() {
    let id = "00000000-0000-0000-0000-000000000002";
    let old = note(
        "campaign",
        &format!(
            "splat_namespace: rook\nsplat_packages: [{{name: Core, package: {{id: {id}}}, selection: {{mode: all}}}}]"
        ),
        "DM prose.\n",
    );
    let new = formats::convert("Campaign.md", &old).unwrap();
    let external = new.replace(&format!("Core ^{id}"), "Core");
    let edited = formats::patch_model(
        &external,
        &[(
            "splat_packages",
            Some(model(&old)["splat_packages"].clone()),
        )],
    )
    .unwrap();
    assert_eq!(edited, new);
    let off = formats::patch_model(
        &external,
        &[("splat_packages", Some(Value::Sequence(Vec::new())))],
    )
    .unwrap();
    assert_eq!(off, external.replace("[x]", "[ ]"));
}
