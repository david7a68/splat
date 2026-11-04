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

fn stat_note(kind: &str, data: &str, table: &str) -> (String, String) {
    (
        note(kind, data, "# Rook\n\nKeep my prose.\n"),
        note(
            kind,
            "tags: [dnd]",
            &format!("# Rook\n\n{table}\nKeep my prose.\n"),
        )
        .replace("splat_schema: 1", "splat_schema: 2")
        .replace("tags: [dnd]\n", ""),
    )
}
const CREATURE_TABLE: &str = "## Stat block\n\n%% splat omit=splat_creature/armor_class_text %%\n\n| Stat | Value |\n| --- | --- |\n| Size | medium |\n| Type | beast |\n| Alignment | Unaligned |\n| Armor Class | 13 |\n| Hit Points | 22 (4d8 + 4) |\n| Initiative | +2 (12) |\n| Speed | 40 ft. |\n| Challenge | 1 (XP 200; PB +2) |\n| Senses | Darkvision 60 ft. |\n\n| Ability | Score | Mod | Save |\n| --- | --- | --- | --- |\n| Str | 12 | +1 | |\n| Dex | 15 | +2 | +4 |\n| Con | 13 | +1 | |\n| Int | 3 | −4 | |\n| Wis | 12 | +1 | |\n| Cha | 6 | −2 | |\n";
fn creature_pair() -> (String, String) {
    stat_note(
        "creature",
        "splat_creature: {size: medium, type: beast, alignment: Unaligned, hit_points_text: '22 (4d8 + 4)', movement_text: '40 ft.', cr_text: '1 (XP 200; PB +2)', senses_text: 'Darkvision 60 ft.'}\nsplat_stats: {max_hp: {value: 22}, armor_class: {value: 13}, initiative: {value: 2}, speed: {value: 40, unit: ft}, strength: {value: 12}, dexterity: {value: 15}, save_dexterity: {value: 4}, constitution: {value: 13}, intelligence: {value: 3}, wisdom: {value: 12}, charisma: {value: 6}}",
        CREATURE_TABLE,
    )
}

#[test]
fn fmt_f2_creature_spell_item_independent_golden_pairs() {
    let pairs = [
        creature_pair(),
        stat_note(
            "creature",
            "splat_creature: {size: medium, type: humanoid}\nsplat_stats: {initiative: {value: 0}, speed: {value: 30, unit: ft}, strength: {value: 10}}",
            "## Stat block\n\n%% splat omit=splat_creature/movement_text %%\n\n| Stat | Value |\n|---|---|\n| Size | medium |\n| Type | humanoid |\n| Initiative | +0 |\n| Speed | 30 ft. |\n\n| Ability | Score | Mod | Save |\n|---|---|---|---|\n| Str | 10 | +0 | |\n",
        ),
        stat_note(
            "spell",
            "splat_spell: {level: 1, school: evocation, classes: [wizard, sorcerer], casting_time: Action, range_text: Self, components: [verbal, somatic], duration: 'Concentration, up to 1 minute', concentration: true, ritual: false}",
            "## Spell\n\n| Field | Value |\n|---|---|\n| Level | 1 |\n| School | evocation |\n| Classes | Wizard, Sorcerer |\n| Casting Time | Action |\n| Range | Self |\n| Components | V, S |\n| Duration | Concentration, up to 1 minute |\n",
        ),
        stat_note(
            "item",
            "splat_item: {category: wondrous_item, rarity: uncommon, equipment: {attunement: required}}",
            "## Item\n\n| Field | Value |\n|---|---|\n| Type | Wondrous Item |\n| Rarity | uncommon |\n| Attunement | required |\n",
        ),
    ];
    for (old, new) in pairs {
        assert_eq!(model(&old), model(&new));
        let converted = formats::convert("Rook.md", &old).unwrap();
        assert_eq!(model(&new), model(&converted));
        assert!(
            Entry::parse("Rook.md", &converted)
                .unwrap()
                .flags
                .is_empty()
        );
        assert_eq!(converted, formats::convert("Rook.md", &converted).unwrap());
        assert!(converted.contains("Keep my prose."));
    }
}

#[test]
fn fmt_f2_book_conversions_keep_mechanics_without_second_stat_source() {
    for path in [
        "creatures/wolf.md",
        "creatures/aboleth.md",
        "spells/bless.md",
        "items/bag_of_holding.md",
    ] {
        let old = super::definitions::source_file(&format!("definitions/{path}")).unwrap();
        let new = formats::convert(path, old).unwrap();
        assert_eq!(model(old), model(&new), "{path}");
        assert!(!new.contains("path=splat_stats "));
        assert!(!new.contains("path=splat_spell "));
        assert!(!new.contains("path=splat_item "));
        assert_eq!(new, formats::convert(path, &new).unwrap());
    }
}

#[test]
fn fmt_f2_one_cell_edit_keeps_formula_crlf_comments_and_unknown_rows() {
    let (_, new) = creature_pair();
    let new = new
        .replace("| Hit Points | 22", "| **HP** | 22")
        .replace(
            "| Senses |",
            "| Custom | [[Note|Name]] and \\| pipe |\n| Senses |",
        )
        .replace("Keep my prose.", "%% keep %%\nKeep my prose.")
        .replace(
            "## Stat block",
            "### Stat block\n\n| Stat | Value |\n| --- | --- |\n| HP | 999 |\n\n## Stat block",
        )
        .replace('\n', "\r\n");
    let mut entry = Entry::parse("Rook.md", &new).unwrap();
    entry.fields.get_mut("splat_stats").unwrap()["max_hp"]["value"] = 23.into();
    let edited = formats::patch_model(
        &new,
        &[("splat_stats", Some(entry.fields["splat_stats"].clone()))],
    )
    .unwrap();
    assert_eq!(edited, new.replace("| **HP** | 22", "| **HP** | 23"));
    let mut entry = Entry::parse("Rook.md", &edited).unwrap();
    entry.fields.get_mut("splat_stats").unwrap()["dexterity"]["value"] = 16.into();
    let score = formats::patch_model(
        &edited,
        &[("splat_stats", Some(entry.fields["splat_stats"].clone()))],
    )
    .unwrap();
    assert_eq!(
        score,
        edited.replace("| Dex | 15 | +2 | +4 |", "| Dex | 16 | +3 | +4 |")
    );
    let malicious = "22 | extra\n| HP | 999";
    assert!(replace_cell(&new, "Stat block", &["Stat", "Value"], "HP", 1, malicious).is_err());
}

#[test]
fn fmt_f2_clearing_optional_text_changes_only_its_cell() {
    let (_, new) = creature_pair();
    let mut entry = Entry::parse("Rook.md", &new).unwrap();
    entry
        .fields
        .get_mut("splat_creature")
        .unwrap()
        .as_mapping_mut()
        .unwrap()
        .remove("senses_text");
    let edited = formats::patch_model(
        &new,
        &[(
            "splat_creature",
            Some(entry.fields["splat_creature"].clone()),
        )],
    )
    .unwrap();
    assert_eq!(
        edited,
        new.replace("| Senses | Darkvision 60 ft. |", "| Senses |  |")
    );
}

#[test]
fn fmt_f2_edit_uses_the_first_alias_row_and_keeps_duplicate_bytes() {
    let (_, new) = creature_pair();
    let new = new.replace(
        "| Hit Points | 22 (4d8 + 4) |",
        "| HP | 22 (4d8 + 4) |\n| Hit Points | 999 |",
    );
    let mut entry = Entry::parse("Rook.md", &new).unwrap();
    entry.fields.get_mut("splat_stats").unwrap()["max_hp"]["value"] = 23.into();
    let edited = formats::patch_model(
        &new,
        &[("splat_stats", Some(entry.fields["splat_stats"].clone()))],
    )
    .unwrap();
    assert_eq!(edited, new.replace("| HP | 22", "| HP | 23"));
}

#[test]
fn fmt_f2_malformed_cells_stay_first_alias_wins_and_old_yaml_is_ignored() {
    let (_, new) = creature_pair();
    let bad = new
        .replace(
            "| Hit Points | 22 (4d8 + 4) |",
            "| Hit Points | about twenty |\n| HP | 999 |",
        )
        .replace("| Dex | 15 | +2 | +4 |", "| Dex | 15 | +9 | nope |")
        .replace("+2 (12)", "+2 (99)")
        .replace(
            "splat_kind: creature",
            "splat_kind: creature\nsplat_stats: {max_hp: {value: 999}}",
        );
    let entry = Entry::parse("Rook.md", &bad).unwrap();
    let reading = super::stat_blocks::reading_body(&entry, &entry.body);
    assert!(reading.contains("about twenty"));
    assert!(!reading.contains("%% splat omit="));
    assert!(entry.fields["splat_stats"].get("max_hp").is_none());
    assert_eq!(
        entry.fields["splat_stats"]["dexterity"]["value"],
        Value::from(15)
    );
    assert!(entry.fields["splat_stats"].get("save_dexterity").is_none());
    for words in [
        "Hit Points needs a number first",
        "Duplicate Hit Points",
        "Mod differs",
        "Save needs",
        "passive value",
        "Old data ignored",
    ] {
        assert!(
            entry.flags.iter().any(|f| f.message.contains(words)),
            "{words}: {:?}",
            entry.flags
        );
    }
    assert_eq!(
        bad.lines().nth(
            entry
                .flags
                .iter()
                .find(|f| f.message.contains("Hit Points needs"))
                .unwrap()
                .line
                - 1
        ),
        Some("| Hit Points | about twenty |")
    );
}

#[test]
fn fmt_f2_missing_rows_insert_in_order_and_unrelated_flags_do_not_block_edit() {
    let (_, new) = creature_pair();
    let missing = new
        .replace("| Armor Class | 13 |\n", "")
        .replace("| Dex | 15 | +2 |", "| Dex | 15 | +9 |");
    let mut entry = Entry::parse("Rook.md", &missing).unwrap();
    entry
        .fields
        .get_mut("splat_stats")
        .unwrap()
        .as_mapping_mut()
        .unwrap()
        .insert(
            "armor_class".into(),
            serde_yaml_ng::from_str("{value: 14}").unwrap(),
        );
    let changed = formats::patch_model(
        &missing,
        &[("splat_stats", Some(entry.fields["splat_stats"].clone()))],
    )
    .unwrap();
    assert_eq!(
        changed,
        missing.replace("| Hit Points |", "| Armor Class | 14 |\n| Hit Points |")
    );
}

#[test]
fn fmt_f2_every_book_stat_block_converts_or_refuses_conflicting_legacy_values() {
    let mut errors = Vec::new();
    for entry in super::definitions::entries()
        .unwrap()
        .iter()
        .filter(|e| ["creature", "spell", "item"].contains(&e.kind.as_str()))
    {
        let old = super::definitions::source_file(&entry.path).unwrap();
        match formats::convert(&entry.path, old) {
            Ok(new) => assert_eq!(model(old), model(&new), "{}", entry.path),
            Err(error) => {
                let review = [
                    "animated_object",
                    "avatar_of_death",
                    "draconic_spirit",
                    "giant_insect",
                    "otherworldly_steed",
                    "werebear",
                    "wereboar",
                    "weretiger",
                    "werewolf",
                ];
                let name = std::path::Path::new(&entry.path)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap();
                if !review.contains(&name) {
                    errors.push(format!("{}: {error}", entry.path));
                }
                assert!(
                    error.contains("unsupported")
                        || error == "The printed stats and saved stats disagree."
                );
            }
        }
    }
    assert!(
        errors.is_empty(),
        "{} failures: {}",
        errors.len(),
        errors.join("\n")
    );
}

#[test]
fn fmt_f2_native_blank_hp_and_missing_ability_table_can_be_set() {
    let text = note(
        "creature",
        "",
        "# Rook\n\n## Stat block\n\n| Stat | Value |\n|---|---|\n| Hit Points | |\n\nKeep prose.\n",
    )
    .replace("splat_schema: 1", "splat_schema: 2");
    let stats = serde_yaml_ng::from_str("{max_hp: {value: 22}, strength: {value: 12}}").unwrap();
    let after = formats::patch_model(&text, &[("splat_stats", Some(stats))]).unwrap();
    let entry = Entry::parse("Rook.md", &after).unwrap();
    assert!(entry.flags.is_empty(), "{:?}", entry.flags);
    assert_eq!(
        entry.fields["splat_stats"]["max_hp"]["value"],
        Value::from(22)
    );
    assert_eq!(
        entry.fields["splat_stats"]["strength"]["value"],
        Value::from(12)
    );
    assert!(after.contains("| Ability | Score | Mod | Save |"));
    assert!(after.contains("Keep prose."));
}

#[test]
fn fmt_f2_spell_item_edits_preserve_other_cells_and_extras() {
    for (kind, data, root, key, next, before, after) in [
        (
            "spell",
            "splat_spell: {level: 1, school: evocation, components: [verbal], duration: Instantaneous, concentration: false}",
            "splat_spell",
            "level",
            Value::from(2),
            "| Level | 1 |",
            "| Level | 2 |",
        ),
        (
            "item",
            "splat_item: {category: wondrous_item, rarity: uncommon, mass: {value: 5, unit: lb}}",
            "splat_item",
            "rarity",
            Value::from("rare"),
            "| Rarity | uncommon |",
            "| Rarity | rare |",
        ),
    ] {
        let old = note(kind, data, "# Rook\n\nKeep prose.\n");
        let text = formats::convert("Rook.md", &old).unwrap();
        let mut entry = Entry::parse("Rook.md", &text).unwrap();
        entry.fields.get_mut(root).unwrap()[key] = next;
        let edited =
            formats::patch_model(&text, &[(root, Some(entry.fields[root].clone()))]).unwrap();
        assert_eq!(edited, text.replace(before, after));
    }
}
