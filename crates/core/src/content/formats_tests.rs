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
    let mut fields = (*super::legacy::parse("Rook.md", text).unwrap().fields).clone();
    fields.remove("splat_schema");
    fields
}

#[test]
fn converted_optional_spell_rows_do_not_repeat_in_reader_prose() {
    let old = note(
        "spell",
        "splat_spell: {level: 1, school: evocation, ritual: false, concentration: false, range: {feet: 30}, range_text: 'Within sight'}",
        "# Rook\n\nKeep this spell text.\n",
    );
    let converted = formats::convert("Rook.md", &old).unwrap();
    let entry = Entry::parse("Rook.md", &converted).unwrap();
    assert!(entry.body.contains("| Ritual | No |"));
    assert!(entry.body.contains("| Concentration | No |"));
    assert!(entry.body.contains("| Range Text | Within sight |"));
    let reading = super::stat_blocks::reading_body(&entry, &entry.body);
    assert!(reading.contains("Keep this spell text."));
    for row in ["| Ritual |", "| Concentration |", "| Range Text |"] {
        assert!(!reading.contains(row), "{reading}");
    }
}

#[test]
fn fmt_f1_world_conversion_and_obsidian_properties_rewrite() {
    let old = note(
        "world",
        "splat_world: {type: npc, status: alive, creature: {key: 'srd:creature/tough'}, in: '[[Undercroft]]', aliases: [The Veteran]}",
        "# Rook\n",
    );
    let new = old.replace("splat_schema: 1", "splat_schema: 2").replace("splat_world: {type: npc, status: alive, creature: {key: 'srd:creature/tough'}, in: '[[Undercroft]]', aliases: [The Veteran]}", "splat_type: npc\nsplat_status: alive\nsplat_stat_block: srd:creature/tough\nsplat_in: '[[Undercroft]]'\naliases: [The Veteran]");
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
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
fn fmt_f1_provenance_mechanics_and_package_conversions() {
    let old = note(
        "rule",
        "splat_rule: {}\nsplat_origin: {key: 'srd:rules/test'}\nsplat_sources: [{title: Book, locator: 'p. 3'}]\nsplat_requires: [{key: 'srd:rules/other'}]",
        "# Rook\n",
    );
    let new = note("rule", "splat_origin: srd:rules/test\nsplat_sources: ['Book, p. 3']\nsplat_requires: [srd:rules/other]", "# Rook\n").replace("splat_schema: 1", "splat_schema: 2");
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
    let old = note(
        "package",
        "splat_namespace: rook\nsplat_reference: {pages: [1, 2, 3]}\nsplat_import_routes: {items: 'pack:items'}",
        "# Rook\n",
    );
    let new = note("package", "splat_namespace: rook\nsplat_pages: 1-3", "# Rook\n\n## Splat data\n\n<!-- splat:yaml path=splat_import_routes -->\n\n```yaml\nitems: pack:items\n```\n").replace("splat_schema: 1", "splat_schema: 2");
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
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
            super::legacy::parse("Rook.md", &old).unwrap().fields["splat_schema"],
            Value::from(1)
        );
    }
}

#[test]
fn fmt_f1_machine_kinds_and_character_have_independent_conversions() {
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
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
    assert_eq!(
        model(&new),
        model(&formats::convert("Rook.md", &old).unwrap())
    );
}

#[test]
fn fmt_f1_bundled_files_parse_within_budget() {
    let started = std::time::Instant::now();
    let (entries, errors) = super::read_directory(std::path::Path::new(concat!(
        concat!(env!("CARGO_MANIFEST_DIR"), "/../.."),
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
    let entry = super::legacy::parse("Rook.md", &text).unwrap();
    assert_eq!(entry.flags.len(), 1);
    assert_eq!(entry.flags[0].line, 7);
    assert!(entry.fields["splat_world"].get("status").is_none());
    assert_eq!(
        entry.fields["splat_world"]["aliases"][0],
        Value::from("Rook")
    );
    assert!(text.contains("splat_status: [alive]"));
    // A type is the DM's own word, and only a word.
    let own_type = text.replace("splat_type: npc", "splat_type: Company");
    let entry = super::legacy::parse("Rook.md", &own_type).unwrap();
    assert_eq!(entry.flags.len(), 1);
    assert_eq!(entry.fields["splat_world"]["type"], Value::from("company"));
    let bad_type = text.replace("splat_type: npc", "splat_type: [dragon]");
    let entry = super::legacy::parse("Rook.md", &bad_type).unwrap();
    assert_eq!(entry.flags.len(), 2);
    assert!(entry.fields["splat_world"].get("type").is_none());
}

#[test]
fn world_relations_and_dates_read_from_flat_properties_and_stay_on_a_patch() {
    let text = note(
        "world",
        "splat_type: event\nsplat_part_of:\n- '[[Verori]]'\n- Kaun\nsplat_when: 120 years ago\nsplat_until: 90 years ago",
        "# The Revolt\n",
    )
    .replace("splat_schema: 1", "splat_schema: 2");
    let entry = Entry::parse("The Revolt.md", &text).unwrap();
    assert!(entry.flags.is_empty(), "{:?}", entry.flags);
    let world = &entry.fields["splat_world"];
    assert_eq!(world["part_of"][0], Value::from("[[Verori]]"));
    assert_eq!(world["part_of"][1], Value::from("Kaun"));
    assert_eq!(world["when"], Value::from("120 years ago"));
    assert_eq!(world["until"], Value::from("90 years ago"));
    // A hand-written link without quotes reads as the same link.
    let bare = text.replace("- '[[Verori]]'", "- [[Verori]]");
    let entry = Entry::parse("The Revolt.md", &bare).unwrap();
    assert!(entry.flags.is_empty(), "{:?}", entry.flags);
    assert_eq!(
        entry.fields["splat_world"]["part_of"][0],
        Value::from("[[Verori]]")
    );
    // One group reads as a list of one, and a number is not a date.
    let one = text
        .replace(
            "splat_part_of:\n- '[[Verori]]'\n- Kaun",
            "splat_part_of: Kaun",
        )
        .replace("splat_when: 120 years ago", "splat_when: 120");
    let entry = Entry::parse("The Revolt.md", &one).unwrap();
    assert_eq!(
        entry.fields["splat_world"]["part_of"][0],
        Value::from("Kaun")
    );
    assert_eq!(entry.flags.len(), 1);
    assert!(entry.flags[0].message.starts_with("When \"120\" not read"));
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
    assert_eq!(super::legacy::parse("Rook.md", &after).unwrap().flags, []);
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
    let target = super::legacy::parse(
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
    let parsed = super::legacy::parse("Rook.md", &after).unwrap();
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
fn fmt_f2_creature_spell_item_independent_conversions() {
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
        assert_eq!(
            model(&formats::convert("Rook.md", &old).unwrap()),
            model(&new)
        );
        let converted = formats::convert("Rook.md", &old).unwrap();
        assert_eq!(model(&new), model(&converted));
        assert!(
            super::legacy::parse("Rook.md", &converted)
                .unwrap()
                .flags
                .is_empty()
        );
        assert_eq!(converted, formats::convert("Rook.md", &converted).unwrap());
        assert!(converted.contains("Keep my prose."));
    }
}

#[test]
fn fmt_f2_printed_identity_fills_missing_properties_and_writes_reader_case() {
    let old = note(
        "creature",
        "splat_creature: {}\nsplat_stats: {max_hp: {value: 22}}",
        "# Rook\n\n## Source text\n\nMedium beast, Unaligned\\\nHP 22\n",
    );
    let (_, golden) = stat_note(
        "creature",
        "",
        "## Stat block\n\n%% splat omit=splat_creature/hit_points_text %%\n\n| Stat | Value |\n| --- | --- |\n| Size | Medium |\n| Type | Beast |\n| Alignment | Unaligned |\n| Hit Points | 22 |\n",
    );
    assert_eq!(model(&old), model(&golden));
    let converted = formats::convert("Rook.md", &old).unwrap();
    assert_eq!(model(&converted), model(&golden));
    for row in [
        "| Size | Medium |",
        "| Type | Beast |",
        "| Alignment | Unaligned |",
    ] {
        assert!(converted.contains(row), "{row}");
    }
    assert_eq!(converted, formats::convert("Rook.md", &converted).unwrap());
    assert_eq!(
        model(&converted),
        model(
            &converted
                .replace("| Size | Medium |", "| Size | mEDiUm |")
                .replace("| Type | Beast |", "| Type | bEaSt |")
        )
    );
    let no_alignment = old.replace(", Unaligned", "");
    assert!(
        !formats::convert("Rook.md", &no_alignment)
            .unwrap()
            .contains("| Alignment |")
    );
    let explicit = old.replace(
        "splat_creature: {}",
        "splat_creature: {size: small, type: fey, alignment: Neutral}",
    );
    let explicit = formats::convert("Rook.md", &explicit).unwrap();
    assert!(explicit.contains("| Size | Small |"));
    assert!(explicit.contains("| Type | Fey |"));
    assert!(explicit.contains("| Alignment | Neutral |"));
    let tagged = old.replace(
        "Medium beast, Unaligned",
        "Medium humanoid (human, elf), Neutral",
    );
    let tagged = formats::convert("Rook.md", &tagged).unwrap();
    assert!(tagged.contains("| Type | Humanoid (human, elf) |"));
    assert!(tagged.contains("| Alignment | Neutral |"));
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
    let mut entry = super::legacy::parse("Rook.md", &new).unwrap();
    entry.fields.get_mut("splat_stats").unwrap()["max_hp"]["value"] = 23.into();
    let edited = formats::patch_model(
        &new,
        &[("splat_stats", Some(entry.fields["splat_stats"].clone()))],
    )
    .unwrap();
    assert_eq!(edited, new.replace("| **HP** | 22", "| **HP** | 23"));
    let mut entry = super::legacy::parse("Rook.md", &edited).unwrap();
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
    let mut entry = super::legacy::parse("Rook.md", &new).unwrap();
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
    let mut entry = super::legacy::parse("Rook.md", &new).unwrap();
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
    let entry = super::legacy::parse("Rook.md", &bad).unwrap();
    let reading = super::stat_blocks::reading_body(&entry, &entry.body);
    assert!(!reading.contains("about twenty"));
    assert_eq!(
        super::stat_blocks::invalid_vital(&entry, "max_hp").as_deref(),
        Some("about twenty")
    );
    assert!(bad.contains("| Hit Points | about twenty |"));
    assert!(!reading.contains("%% splat omit="));
    assert!(entry.fields["splat_stats"].get("max_hp").is_none());
    assert_eq!(
        entry.fields["splat_stats"]["dexterity"]["value"],
        Value::from(15)
    );
    assert!(entry.fields["splat_stats"].get("save_dexterity").is_none());
    for words in [
        "Hit Points \"about twenty\" not read (line 21). Start with a number, such as 22 (4d8 + 4).",
        "Duplicate Hit Points",
        "Save needs",
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
                .find(|f| f.message.contains("Hit Points \"about twenty\" not read"))
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
    let mut entry = super::legacy::parse("Rook.md", &missing).unwrap();
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
        missing
            .replace("| Hit Points |", "| Armor Class | 14 |\n| Hit Points |")
            .replace("| Dex | 15 | +9 |", "| Dex | 15 | +2 |")
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
                        || error == "The printed stats and saved stats disagree.",
                    "{}: {error}",
                    entry.path
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
    let entry = super::legacy::parse("Rook.md", &after).unwrap();
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
            "| Rarity | Uncommon |",
            "| Rarity | Rare |",
        ),
    ] {
        let old = note(kind, data, "# Rook\n\nKeep prose.\n");
        let text = formats::convert("Rook.md", &old).unwrap();
        let mut entry = super::legacy::parse("Rook.md", &text).unwrap();
        entry.fields.get_mut(root).unwrap()[key] = next;
        let edited =
            formats::patch_model(&text, &[(root, Some(entry.fields[root].clone()))]).unwrap();
        assert_eq!(edited, text.replace(before, after));
    }
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
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
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
    let entry = super::legacy::parse("Campaign.md", &bad).unwrap();
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
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
    let converted = formats::convert("Group.md", &old).unwrap();
    assert_eq!(model(&old), model(&converted));
    assert_eq!(converted, formats::convert("Group.md", &converted).unwrap());
    let mut group = model(&new)["splat_encounter_group"].clone();
    group["members"][0]["overrides"]["max_hp"]["add"] = 10.into();
    let edited = formats::patch_model(&new, &[("splat_encounter_group", Some(group))]).unwrap();
    assert_eq!(edited, new.replace("HP +5", "HP +10"));
    let bad = format!("{new}| 1 | [Goblin](splat:{key}) | | HP lots | Keep this |\n");
    let entry = super::legacy::parse("Group.md", &bad).unwrap();
    assert_eq!(entry.flags.len(), 1);
    assert_eq!(
        entry.flags[0].message,
        "the change \"HP lots\" was not read. Use a number, such as HP +5."
    );
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
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
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
    let entry = super::legacy::parse("Encounter.md", &bad).unwrap();
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
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
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
            &format!("Campaign initiative %% splat rule=dnd2024:rules/combat_initiative id={id}"),
            &format!(
                "Campaign initiative %% splat rule=dnd2024:rules/combat_initiative id={other}"
            )
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

#[test]
fn i19_challenge_derives_missing_xp_and_pb_and_flags_explicit_mismatches() {
    let (_, new) = creature_pair();
    for (raw, expected, warning) in [
        ("3", "3 (XP 700; PB +2)", None),
        ("1/8", "1/8 (XP 25; PB +2)", None),
        ("5", "5 (XP 1800; PB +3)", None),
        (
            "3 (XP 200; PB +2)",
            "3 (XP 200; PB +2)",
            Some("CR 3 is XP 700; XP 200 is used"),
        ),
        (
            "5 (XP 1800; PB +2)",
            "5 (XP 1800; PB +2)",
            Some("CR 5 is PB +3; PB +2 is used"),
        ),
        (
            "3 (XP 200 in lair; PB +2)",
            "3 (XP 200 in lair; PB +2)",
            None,
        ),
    ] {
        let text = new.replace("1 (XP 200; PB +2)", raw);
        let entry = super::legacy::parse("Rook.md", &text).unwrap();
        assert_eq!(
            entry.fields["splat_creature"]["cr_text"].as_str(),
            Some(expected)
        );
        if let Some(warning) = warning {
            assert!(
                entry.flags.iter().any(|flag| flag.message == warning
                    && text
                        .lines()
                        .nth(flag.line - 1)
                        .unwrap()
                        .contains("Challenge")),
                "{:?}",
                entry.flags
            );
        } else {
            assert!(entry.flags.is_empty(), "{:?}", entry.flags);
        }
        let saved = super::stat_blocks::render(&entry.fields, "creature");
        assert!(saved.contains(&format!("| Challenge | {expected} |")));
    }
}

#[test]
fn i19_conversion_folds_printed_stats_and_keeps_book_text() {
    let old = note(
        "creature",
        "splat_creature: {size: medium, type: beast, cr_text: '1 (XP 200; PB +2)'}\nsplat_stats: {max_hp: {value: 22}, armor_class: {value: 13}}",
        "# Rook\n\n## Source text\n\nMedium beast, Unaligned\\\nAC 13  Initiative +2 (12)\\\nHP 22 (4d8 + 4)\\\nSpeed 30 ft.\\\nCR 1 (XP 200; PB +2)\\\n\n### Traits\n\nKeen Hearing. Keep this rule.\n\n### Actions\n\nBite. Keep this attack.\n",
    );
    let converted = formats::convert("Rook.md", &old).unwrap();
    assert!(converted.contains("> [!quote]- Printed stat block (not read; edit the table above)"));
    for line in [
        "Medium beast, Unaligned",
        "AC 13  Initiative +2 (12)",
        "HP 22 (4d8 + 4)",
        "Speed 30 ft.",
        "CR 1 (XP 200; PB +2)",
    ] {
        assert!(converted.contains(&format!("> {line}")), "{line}");
    }
    assert!(converted.contains("Keen Hearing. Keep this rule."));
    assert!(converted.contains("Bite. Keep this attack."));
    let entry = super::legacy::parse("Rook.md", &converted).unwrap();
    let reading = super::stat_blocks::reading_body(&entry, &entry.body);
    assert!(!reading.contains("HP 22"));
    assert!(!reading.contains("[!quote]"));
    assert!(reading.contains("Keen Hearing. Keep this rule."));
    assert_eq!(converted, formats::convert("Rook.md", &converted).unwrap());
}

#[test]
fn i19_derived_cells_and_speed_spellings_read_without_flags_and_write_back() {
    let (_, new) = creature_pair();
    for speed in ["30 ft", "30 ft.", "30 feet", "30ft."] {
        let text = new
            .replace("40 ft.", speed)
            .replace("| Dex | 15 | +2 | +4 |", "| Dex | 16 | | +4 |")
            .replace("+2 (12)", "+3 (12)")
            .replace("1 (XP 200; PB +2)", "3");
        let entry = super::legacy::parse("Rook.md", &text).unwrap();
        assert!(entry.flags.is_empty(), "{:?}", entry.flags);
        assert_eq!(
            entry.fields["splat_stats"]["speed"]["value"],
            Value::from(30)
        );
        let saved = formats::patch_model(
            &text,
            &[("splat_stats", Some(entry.fields["splat_stats"].clone()))],
        )
        .unwrap();
        for row in [
            "| Dex | 16 | +3| +4 |",
            "| Initiative | +3 (13) |",
            "| Speed | 30 ft. |",
            "| Challenge | 3 (XP 700; PB +2) |",
        ] {
            assert!(saved.contains(row), "{row}: {saved}");
        }
    }
}

#[test]
fn i19_conversion_writes_human_words_and_puts_hints_after_tables() {
    for (kind, fields, rows) in [
        (
            "spell",
            "splat_spell: {level: 0, school: evocation, classes: [wizard, sorcerer], casting_time: {amount: 1, unit: action}}",
            vec![
                "| Level | Cantrip |",
                "| School | Evocation |",
                "| Classes | Wizard, Sorcerer |",
            ],
        ),
        (
            "item",
            "splat_item: {category: wondrous_item, rarity: uncommon, equipment: {attunement: none}}",
            vec![
                "| Type | Wondrous Item |",
                "| Rarity | Uncommon |",
                "| Attunement | No |",
            ],
        ),
    ] {
        let text = formats::convert("Rook.md", &note(kind, fields, "# Rook\n")).unwrap();
        for row in rows {
            assert!(text.contains(row), "{row}: {text}");
        }
        if let Some(hint) = text.find("%% splat") {
            assert!(hint > text.rfind("|\n").unwrap());
        }
    }
    let health = note(
        "creature",
        "splat_creature: {}",
        "# Rook\n\n## Health\n\n<!-- splat:map path=splat_stats -->\n\n| Key | value |\n| --- | --- |\n| max_hp | 22 |\n",
    );
    assert!(
        !formats::convert("Rook.md", &health)
            .unwrap()
            .contains("## Health\n")
    );
}

#[test]
fn i19_conversion_keeps_explicit_challenge_mismatch_and_flags_it() {
    let text = note(
        "creature",
        "splat_creature: {cr_text: '3 (XP 200; PB +2)'}\nsplat_stats: {}",
        "# Rook\n",
    );
    let converted = formats::convert("Rook.md", &text).unwrap();
    let entry = super::legacy::parse("Rook.md", &converted).unwrap();
    assert_eq!(model(&text), model(&converted));
    assert!(
        entry
            .flags
            .iter()
            .any(|flag| flag.message == "CR 3 is XP 700; XP 200 is used")
    );
}

#[test]
fn i19_x5_visible_rule_choice_wins_and_save_updates_its_id() {
    let old_id = "00000000-0000-0000-0000-000000000002";
    let target = super::legacy::parse(
        "Rules/Individual.md",
        &note("rule", "splat_rule: {}", "")
            .replace("splat_name: Rook", "splat_name: Individual initiative"),
    )
    .unwrap();
    for visible in [
        "Individual initiative",
        "[[Rules/Individual|Individual initiative]]",
    ] {
        let source = note("campaign", "splat_namespace: rook", &format!("## Rule choices\n\n| Rule | Choice |\n| --- | --- |\n| Combat initiative | {visible} %% splat rule=dnd2024:rules/combat_initiative id={old_id} %% |\n")).replace("splat_schema: 1", "splat_schema: 2");
        let mut entry = super::legacy::parse("Campaign.md", &source).unwrap();
        super::list_formats::resolve(&mut entry, std::slice::from_ref(&target), &source);
        assert_eq!(
            entry.fields["splat_rule_choices"]["dnd2024:rules/combat_initiative"]["id"],
            Value::from(target.id.to_string())
        );
        let saved = formats::convert_resolved("Campaign.md", &source, [&target]).unwrap();
        assert!(saved.contains(&format!("id={} %%", target.id)));
        assert!(!saved.contains(&format!("| {old_id} %%")));
    }
}

#[test]
fn i19_x5_unknown_rule_choice_is_flagged_and_not_used() {
    let source = note("campaign", "splat_namespace: rook", "## Rule choices\n\n| Rule | Choice |\n| --- | --- |\n| Combat initiative | Imaginary initiative %% splat rule=dnd2024:rules/combat_initiative id=00000000-0000-0000-0000-000000000002 %% |\n").replace("splat_schema: 1", "splat_schema: 2");
    let mut entry = super::legacy::parse("Campaign.md", &source).unwrap();
    super::list_formats::resolve(&mut entry, &[], &source);
    assert!(
        entry.fields["splat_rule_choices"]
            .as_mapping()
            .unwrap()
            .is_empty()
    );
    assert_eq!(entry.flags.len(), 1);
    assert!(entry.flags[0].message.contains("Imaginary initiative"));
    assert_eq!(
        source
            .lines()
            .nth(entry.flags[0].line - 1)
            .unwrap()
            .chars()
            .next(),
        Some('|')
    );
}

#[test]
fn i20_x29_renamed_rule_uses_id_and_writes_current_name() {
    let target = super::legacy::parse(
        "Rules/Side.md",
        &note("rule", "splat_rule: {}", "")
            .replace("splat_name: Rook", "splat_name: Side initiative"),
    )
    .unwrap();
    let source = note("campaign", "splat_namespace: rook", &format!(
        "## Rule choices\n\n| Rule | Choice |\n| --- | --- |\n| Combat initiative | Campaign initiative %% splat rule=dnd2024:rules/combat_initiative id={} %% |\n", target.id))
        .replace("splat_schema: 1", "splat_schema: 2");
    let mut entry = super::legacy::parse("Campaign.md", &source).unwrap();
    super::list_formats::resolve(&mut entry, std::slice::from_ref(&target), &source);
    assert!(entry.flags.is_empty(), "{:?}", entry.flags);
    assert_eq!(
        entry.fields["splat_rule_choices"]["dnd2024:rules/combat_initiative"]["id"],
        Value::from(target.id.to_string())
    );
    let saved = formats::convert_resolved("Campaign.md", &source, [&target]).unwrap();
    assert!(saved.contains("| Combat initiative | Side initiative %%"));
}

#[test]
fn i20_x31_package_prose_is_kept_and_bad_list_items_are_flagged() {
    let source = note(
        "campaign",
        "splat_namespace: rook",
        "## Packages\n\nWe use the 2024 rules.\n\n- Not a package task\n",
    )
    .replace("splat_schema: 1", "splat_schema: 2");
    let entry = super::legacy::parse("Campaign.md", &source).unwrap();
    assert_eq!(entry.flags.len(), 1);
    assert!(
        source
            .lines()
            .nth(entry.flags[0].line - 1)
            .unwrap()
            .starts_with("- Not")
    );
    assert_eq!(formats::convert("Campaign.md", &source).unwrap(), source);
}

#[test]
fn i19_world_properties_accept_case_and_explain_bad_values() {
    for (typed, stored) in [("NPC", "npc"), ("Location", "location")] {
        let source = note(
            "world",
            &format!("splat_type: {typed}\nsplat_status: [alive]"),
            "",
        )
        .replace("splat_schema: 1", "splat_schema: 2");
        let entry = super::legacy::parse("World.md", &source).unwrap();
        assert_eq!(entry.fields["splat_world"]["type"], Value::from(stored));
        assert_eq!(entry.flags.len(), 1);
        assert!(
            entry.flags[0]
                .message
                .contains("Status \"[alive]\" not read (line 7). Use a short word, such as alive.")
        );
    }
}

#[test]
fn i19_readable_stat_block_and_vault_requirement_survive_save() {
    let creature = super::legacy::parse(
        "Creatures/captain.md",
        &note(
            "creature",
            "splat_key: dnd2024:creatures/bandit_captain\nsplat_creature: {}",
            "",
        )
        .replace("splat_name: Rook", "splat_name: Bandit Captain"),
    )
    .unwrap();
    let required = super::legacy::parse(
        "Rules/Other.md",
        &note("rule", "splat_rule: {}\nsplat_key: srd:rules/other", "")
            .replace("splat_name: Rook", "splat_name: Other")
            .replace(
                "00000000-0000-0000-0000-000000000001",
                "00000000-0000-0000-0000-000000000002",
            ),
    )
    .unwrap();
    let old = note(
        "world",
        &format!(
            "splat_world: {{type: npc, creature: {{key: dnd2024:creatures/bandit_captain}}}}\nsplat_requires: [{{id: {}}}]",
            required.id
        ),
        "",
    );
    let saved = formats::convert_resolved("World.md", &old, [&creature, &required]).unwrap();
    assert!(saved.contains("splat_stat_block: Bandit Captain"));
    assert!(saved.contains("[[Rules/Other|Other]]"));
    assert_eq!(
        saved,
        formats::convert_resolved("World.md", &saved, [&creature, &required]).unwrap()
    );
    let mut duplicate = creature.clone();
    duplicate.id = uuid::Uuid::new_v4();
    duplicate.key = Some("other:creatures/captain".into());
    let saved =
        formats::convert_resolved("World.md", &old, [&creature, &duplicate, &required]).unwrap();
    assert!(saved.contains("splat_stat_block: dnd2024:creatures/bandit_captain"));
}

#[test]
fn i19_package_and_character_data_are_folded_and_editable() {
    for (kind, data) in [
        (
            "package",
            "splat_namespace: rook\nsplat_import_routes: {items: pack:items}",
        ),
        (
            "creature",
            "splat_creature: {}\nsplat_character: {level: 1}",
        ),
    ] {
        let data = if kind == "creature" {
            format!(
                "splat_creature: {{}}\nsplat_character: {}",
                serde_json::to_string(&super::character::Sheet {
                    level: 1,
                    ..Default::default()
                })
                .unwrap()
            )
        } else {
            data.into()
        };
        let old = note(kind, &data, "# Rook\n\nKeep my prose.\n");
        let folded = formats::convert("Note.md", &old).unwrap();
        assert!(folded.contains("> [!note]- Splat data"));
        assert!(!folded.contains("## Splat data"));
        assert_eq!(model(&old), model(&folded));
        let key = if kind == "package" {
            "splat_import_routes"
        } else {
            "splat_character"
        };
        let mut value = model(&folded)[key].clone();
        let (field, revised) = if kind == "package" {
            ("items", Value::from("pack:revised"))
        } else {
            ("level", Value::from(2))
        };
        value[field] = revised.clone();
        let edited = formats::patch_model(&folded, &[(key, Some(value))]).unwrap();
        assert_eq!(model(&edited)[key][field], revised);
        assert!(edited.contains("Keep my prose."));
        assert_eq!(edited, formats::convert("Note.md", &edited).unwrap());
    }
}

#[test]
fn i19_package_tasks_and_only_links_accept_obsidian_lists_without_selecting_all() {
    let id = "00000000-0000-0000-0000-000000000002";
    for bullet in ["-", "*", "+"] {
        let source = note("campaign", "splat_namespace: rook", &format!("## Packages\n\n{bullet} [x] Core ^{id}\n\t+ Only: [[Creatures/Rook, elder|Rook, elder]], [Goblin, scout](splat:dnd2024:creatures/goblin)\n")).replace("splat_schema: 1", "splat_schema: 2");
        let entry = super::legacy::parse("Campaign.md", &source).unwrap();
        assert!(entry.flags.is_empty(), "{:?}", entry.flags);
        let selection = &entry.fields["splat_packages"][0]["selection"];
        assert_eq!(selection["mode"], Value::from("selected"));
        assert_eq!(selection["entries"].as_sequence().unwrap().len(), 2);
        let edited = formats::patch_model(
            &source,
            &[("splat_packages", Some(Value::Sequence(Vec::new())))],
        )
        .unwrap();
        assert_eq!(edited, source.replace("[x]", "[ ]"));
        let bad = source.replace("\t+ Only:", "- Unrecognised:");
        assert_eq!(
            super::legacy::parse("Campaign.md", &bad)
                .unwrap()
                .flags
                .len(),
            1
        );
        let bad = source.replace("[[Creatures/Rook, elder|Rook, elder]]", "[[Unclosed");
        let entry = super::legacy::parse("Campaign.md", &bad).unwrap();
        assert_eq!(entry.flags.len(), 1);
        assert_eq!(
            entry.fields["splat_packages"][0]["selection"]["mode"],
            Value::from("selected")
        );
        assert!(
            entry.fields["splat_packages"][0]["selection"]["entries"]
                .as_sequence()
                .unwrap()
                .is_empty()
        );
    }
}

#[test]
fn i19_x3_copied_roster_row_becomes_a_new_saved_slot() {
    let row = "| enemies | [Goblin](splat:dnd2024:creatures/goblin_warrior) | | HP +5 | %% splat slot=actor_one %% |\n";
    let text = note("encounter", "", &format!("## Roster\n\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n{row}{row}")).replace("splat_schema: 1", "splat_schema: 2");
    let entry = super::legacy::parse("Encounter.md", &text).unwrap();
    assert!(entry.flags.is_empty(), "{:?}", entry.flags);
    let setup = &entry.fields["splat_encounter"];
    assert_eq!(setup["roster"].as_sequence().unwrap().len(), 2);
    assert_ne!(setup["roster"][0]["key"], setup["roster"][1]["key"]);
    let saved = formats::patch_model(&text, &[("splat_encounter", Some(setup.clone()))]).unwrap();
    assert_eq!(saved.matches("slot=actor_one ").count(), 1);
    assert_eq!(model(&saved)["splat_encounter"], *setup);
    assert_eq!(
        saved,
        formats::patch_model(&saved, &[("splat_encounter", Some(setup.clone()))]).unwrap()
    );
}

#[test]
fn i19_x4_unique_creature_names_read_and_save_as_named_links() {
    let target = super::legacy::parse(
        "goblin_warrior.md",
        &note(
            "creature",
            "splat_key: dnd2024:creatures/goblin_warrior\nsplat_creature: {}",
            "",
        )
        .replace("splat_name: Rook", "splat_name: Goblin Warrior"),
    )
    .unwrap();
    for name in ["Goblin Warrior", "[[Goblin Warrior]]"] {
        for (kind, heading, first, columns) in [
            (
                "encounter_group",
                "Members",
                "2",
                super::list_formats::GROUP,
            ),
            (
                "encounter",
                "Roster",
                "enemies",
                super::list_formats::ROSTER,
            ),
        ] {
            let text = note(kind, "splat_side: enemies", &format!("## {heading}\n\n| {} |\n| --- | --- | --- | --- | --- |\n| {first} | {name} | | | |\n", columns.join(" | "))).replace("splat_schema: 1", "splat_schema: 2");
            let mut entry = super::legacy::parse("List.md", &text).unwrap();
            super::list_formats::resolve(&mut entry, std::slice::from_ref(&target), &text);
            assert!(entry.flags.is_empty(), "{name}: {:?}", entry.flags);
            let key = if kind == "encounter" {
                "splat_encounter"
            } else {
                "splat_encounter_group"
            };
            let rows = if kind == "encounter" {
                "roster"
            } else {
                "members"
            };
            assert_eq!(
                entry.fields[key][rows][0]["creature"]["key"],
                Value::from("dnd2024:creatures/goblin_warrior")
            );
            let saved = formats::convert_resolved("List.md", &text, [&target]).unwrap();
            assert!(
                saved.contains("[Goblin Warrior](splat:dnd2024:creatures/goblin_warrior)"),
                "{saved}"
            );
            assert_eq!(
                super::legacy::parse("List.md", &saved).unwrap().fields[key][rows][0]["creature"]["id"],
                Value::from(target.id.to_string())
            );
            let mut replacement = target.clone();
            replacement.id = uuid::Uuid::new_v4();
            replacement.key = Some("dnd2024:creatures/scout".into());
            let changed = saved.replace("creatures/goblin_warrior", "creatures/scout");
            let mut changed_entry = super::legacy::parse("List.md", &changed).unwrap();
            super::list_formats::resolve(
                &mut changed_entry,
                &[target.clone(), replacement.clone()],
                &changed,
            );
            assert_eq!(
                changed_entry.fields[key][rows][0]["creature"]["id"],
                Value::from(replacement.id.to_string())
            );
            let mut duplicate = target.clone();
            duplicate.id = uuid::Uuid::new_v4();
            let mut ambiguous = super::legacy::parse("List.md", &text).unwrap();
            super::list_formats::resolve(&mut ambiguous, &[target.clone(), duplicate], &text);
            assert!(
                ambiguous
                    .flags
                    .iter()
                    .any(|flag| flag.message.contains("ambiguous"))
            );
        }
    }
}

#[test]
fn i19_x16_starting_hp_lives_in_the_changes_cell() {
    let old = note(
        "encounter",
        "splat_encounter: {roster: [{key: actor_guard, creature: {key: 'dnd2024:creatures/guard'}, side: enemies, controller: {kind: manual, role: gm}, initial: {hp: 12, temporary_hp: 3}, overrides: {}}], sides: {party: {hostile_to: [enemies]}, enemies: {hostile_to: [party]}}, assumptions: [], completion: {mode: manual}, limits: {rounds: 20, resolution_events: 10000}}",
        "",
    );
    let saved = formats::convert("Encounter.md", &old).unwrap();
    assert!(saved.contains("Start HP 12"), "{saved}");
    assert!(!saved.contains("hp: 12"));
    assert_eq!(model(&old), model(&saved));
    let edited = saved.replace("Start HP 12", "Start HP 8");
    assert_eq!(
        model(&edited)["splat_encounter"]["roster"][0]["initial"]["hp"],
        Value::from(8)
    );
    let cleared = saved.replace("Start HP 12", "");
    assert!(
        model(&cleared)["splat_encounter"]["roster"][0]["initial"]
            .get("hp")
            .is_none()
    );
}

#[test]
fn i19_x9_saved_links_use_catalogue_names_and_keep_custom_aliases() {
    let target = super::legacy::parse(
        "goblin_warrior.md",
        &note(
            "creature",
            "splat_key: dnd2024:creatures/goblin_warrior\nsplat_creature: {}",
            "",
        )
        .replace("splat_name: Rook", "splat_name: Goblin Warrior"),
    )
    .unwrap();
    let text = note("encounter_group", "splat_side: enemies", "## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 2 | [goblin warrior](splat:dnd2024:creatures/goblin_warrior) | | | |\n").replace("splat_schema: 1", "splat_schema: 2");
    let saved = formats::convert_resolved("Group.md", &text, [&target]).unwrap();
    assert_eq!(saved, text.replace("[goblin warrior]", "[Goblin Warrior]"));
    let alias = text.replace("[goblin warrior]", "[Gate guards]");
    assert_eq!(
        formats::convert_resolved("Group.md", &alias, [&target]).unwrap(),
        alias
    );
    let campaign = note(
        "campaign",
        "splat_namespace: rook",
        "## Packages\n\n  - Only: [goblin warrior](splat:dnd2024:creatures/goblin_warrior)\n",
    )
    .replace("splat_schema: 1", "splat_schema: 2");
    assert_eq!(
        formats::convert_resolved("Campaign.md", &campaign, [&target]).unwrap(),
        campaign.replace("[goblin warrior]", "[Goblin Warrior]")
    );
}

#[test]
fn i20_x20_wiki_links_keep_five_obsidian_table_columns_and_read_both_forms() {
    let old = note(
        "encounter_group",
        "splat_encounter_group: {side: enemies, members: [{creature: {id: '00000000-0000-0000-0000-000000000002', link: '[[Characters/Rook|Rook]]'}, count: 1, overrides: {note: 'Before [[Gate|door]] after'}}]}",
        "",
    );
    let saved = formats::convert("Group.md", &old).unwrap();
    let row = saved
        .lines()
        .find(|line| line.starts_with("| 1 |"))
        .unwrap();
    let separators = row
        .char_indices()
        .filter(|(at, ch)| *ch == '|' && (*at == 0 || row.as_bytes()[at - 1] != b'\\'))
        .count();
    assert_eq!(separators, 6, "Obsidian must keep five cells: {row}");
    assert!(saved.contains(r"[[Characters/Rook\|Rook]]"));
    assert_eq!(model(&saved), model(&old));
    assert_eq!(model(&saved.replace(r"\|", "|")), model(&old));
    let target = super::legacy::parse(
        "Characters/Rook.md",
        &note("creature", "splat_creature: {}", "").replace(
            "00000000-0000-0000-0000-000000000001",
            "00000000-0000-0000-0000-000000000002",
        ),
    )
    .unwrap();
    let written = super::list_formats::write_references(&saved, [&target]).unwrap();
    assert!(written.contains(r"[[Characters/Rook\|Rook]]"));
}

#[test]
fn i20_x21_note_keeps_text_on_both_sides_of_machine_comments() {
    for note_cell in [
        "%% splat slot=actor_one %% Watch the door",
        "Watch %% splat slot=actor_one %% the door",
        "Watch the door %% splat slot=actor_one %%",
    ] {
        let text = note("encounter", "", &format!("## Roster\n\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| enemies | [[Rook]] | | | {note_cell} |\n")).replace("splat_schema: 1", "splat_schema: 2");
        let entry = super::legacy::parse("Fight.md", &text).unwrap();
        assert_eq!(
            entry.fields["splat_encounter"]["roster"][0]["overrides"]["note"],
            Value::from("Watch the door")
        );
        assert_eq!(
            entry.fields["splat_encounter"]["roster"][0]["key"],
            Value::from("actor_one")
        );
    }
}

#[test]
fn i20_x22_visible_vault_link_wins_over_backup_id_in_roster_group_and_only() {
    let first_id = "00000000-0000-0000-0000-000000000002";
    let second_id = "00000000-0000-0000-0000-000000000003";
    let targets: Vec<_> = [("First", first_id), ("Second", second_id)]
        .into_iter()
        .map(|(name, id)| {
            super::legacy::parse(
                &format!("Characters/{name}.md"),
                &note("creature", "splat_creature: {}", "")
                    .replace("splat_name: Rook", &format!("splat_name: {name}"))
                    .replace("00000000-0000-0000-0000-000000000001", id),
            )
            .unwrap()
        })
        .collect();
    for kind in ["encounter_group", "encounter", "campaign"] {
        let link = format!(r"[[Characters/Second\|Second]] %% splat id={first_id} %%");
        let body = if kind == "campaign" {
            format!(
                "## Packages\n\n- [x] Friends ^00000000-0000-0000-0000-000000000004\n  - Only: {link}\n"
            )
        } else {
            format!(
                "## {}\n\n| {} | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| {} | {link} | | | %% splat slot=actor_one %% |\n",
                if kind == "encounter" {
                    "Roster"
                } else {
                    "Members"
                },
                if kind == "encounter" { "Side" } else { "Count" },
                if kind == "encounter" { "party" } else { "1" }
            )
        };
        let text = note(
            kind,
            if kind == "campaign" {
                "splat_namespace: friends"
            } else if kind == "encounter_group" {
                "splat_side: enemies"
            } else {
                ""
            },
            &body,
        )
        .replace("splat_schema: 1", "splat_schema: 2");
        let reference = |entry: &Entry| {
            if kind == "campaign" {
                entry.fields["splat_packages"][0]["selection"]["entries"][0].clone()
            } else {
                entry.fields[if kind == "encounter" {
                    "splat_encounter"
                } else {
                    "splat_encounter_group"
                }][if kind == "encounter" {
                    "roster"
                } else {
                    "members"
                }][0]["creature"]
                    .clone()
            }
        };
        let mut entry = super::legacy::parse("List.md", &text).unwrap();
        super::list_formats::resolve(&mut entry, &targets, &text);
        assert_eq!(reference(&entry)["id"], Value::from(second_id), "{kind}");
        let saved = formats::convert_resolved("List.md", &text, targets.iter()).unwrap();
        assert!(saved.contains(&format!("id={second_id}")), "{saved}");
        let missing = text.replace("Characters/Second", "Characters/Moved");
        let mut entry = super::legacy::parse("List.md", &missing).unwrap();
        super::list_formats::resolve(&mut entry, &targets, &missing);
        assert_eq!(reference(&entry)["id"], Value::from(first_id));
    }
}

#[test]
fn i20_x27_copied_party_row_is_flagged_and_monster_copy_keeps_only_visible_cells() {
    let old = note(
        "encounter",
        "splat_encounter: {roster: [{key: actor_party, party_slot: lead, side: party, controller: {kind: manual, role: player}, overrides: {}}], sides: {party: {hostile_to: [enemies]}, enemies: {hostile_to: [party]}}, assumptions: [], completion: {mode: manual}, limits: {rounds: 20, resolution_events: 10000}}",
        "",
    );
    let saved = formats::convert("Fight.md", &old).unwrap();
    let row = saved
        .lines()
        .find(|line| line.starts_with("| party |"))
        .unwrap();
    let copied = saved.replace(row, &format!("{row}\n{row}"));
    let entry = super::legacy::parse("Fight.md", &copied).unwrap();
    assert!(
        entry
            .flags
            .iter()
            .any(|flag| flag.message == "This party member is already in the roster")
    );
    assert_eq!(
        entry.fields["splat_encounter"]["roster"]
            .as_sequence()
            .unwrap()
            .len(),
        1
    );
    let monster = old.replace(
        "party_slot: lead",
        "creature: {key: 'srd:creature/guard'}, initial: {temp_hp: 9}",
    );
    let saved = formats::convert("Fight.md", &monster).unwrap();
    let row = saved
        .lines()
        .find(|line| line.starts_with("| party |"))
        .unwrap();
    let copied = saved.replace(row, &format!("{row}\n{row}"));
    let entry = super::legacy::parse("Fight.md", &copied).unwrap();
    assert!(entry.flags.is_empty());
    assert_eq!(
        entry.fields["splat_encounter"]["roster"][0]["initial"]["temp_hp"],
        Value::from(9)
    );
    assert!(entry.fields["splat_encounter"]["roster"][1]["initial"].is_null());
}

#[test]
fn i20_x28_member_side_moves_with_its_visible_row() {
    let old = note(
        "encounter_group",
        "splat_encounter_group: {side: enemies, members: [{creature: {key: 'srd:creature/guard'}, count: 1, overrides: {name: Scout, side: rivals}}, {creature: {key: 'srd:creature/guard'}, count: 2}]}",
        "",
    );
    let saved = formats::convert("Group.md", &old).unwrap();
    assert!(saved.contains("Side rivals"));
    assert!(!saved.contains("members_data"));
    let first = saved
        .lines()
        .find(|line| line.starts_with("| 1 |"))
        .unwrap();
    let second = saved
        .lines()
        .find(|line| line.starts_with("| 2 |"))
        .unwrap();
    let moved = saved.replace(&format!("{first}\n{second}"), &format!("{second}\n{first}"));
    let entry = super::legacy::parse("Group.md", &moved).unwrap();
    let members = &entry.fields["splat_encounter_group"]["members"];
    assert!(members[0]["overrides"]["side"].is_null());
    assert_eq!(members[1]["overrides"]["side"], Value::from("rivals"));
    let deleted = saved.replace(&format!("{first}\n"), "");
    let entry = super::legacy::parse("Group.md", &deleted).unwrap();
    assert!(entry.fields["splat_encounter_group"]["members"][0]["overrides"]["side"].is_null());
}

#[test]
fn i20_x30_changes_accept_speed_units_and_explain_group_start_hp() {
    for speed in ["Speed 40 ft.", "Speed 40 ft", "Speed 40"] {
        let changes = super::list_formats::changes(speed, "", "").unwrap();
        assert_eq!(changes["speed"]["set"], Value::from(40));
        assert_eq!(
            super::list_formats::show_changes(&Value::Mapping(changes)),
            "Speed 40 ft."
        );
    }
    assert_eq!(
        super::list_formats::changes("Start HP 12", "", "").unwrap_err(),
        "Start HP is set on an encounter's roster"
    );
}

#[test]
fn i20_x22_named_package_alias_also_refreshes_the_backup_id() {
    let id = "00000000-0000-0000-0000-000000000003";
    let target = super::legacy::parse(
        "Guard.md",
        &note(
            "creature",
            "splat_key: core:creatures/guard\nsplat_creature: {}",
            "",
        )
        .replace("00000000-0000-0000-0000-000000000001", id),
    )
    .unwrap();
    let text = note("encounter_group", "splat_side: enemies", "## Members\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 1 | [Door watch](splat:core:creatures/guard) %% splat id=00000000-0000-0000-0000-000000000002 %% | | | |\n").replace("splat_schema: 1", "splat_schema: 2");
    let saved = formats::convert_resolved("Group.md", &text, [&target]).unwrap();
    assert!(
        saved.contains(&format!(
            "[Door watch](splat:core:creatures/guard) %% splat id={id} %%"
        )),
        "{saved}"
    );
}

#[test]
fn i20_x25_plain_vitals_need_no_prose_presence_hints() {
    let old = note(
        "creature",
        "splat_creature: {size: medium, type: beast}\nsplat_stats: {armor_class: {value: 13}, max_hp: {value: 22}, speed: {value: 40, unit: ft}}",
        "# Rook\n",
    );
    let new = formats::convert("Rook.md", &old).unwrap();
    for path in [
        "splat_creature/armor_class_text",
        "splat_creature/hit_points_text",
        "splat_creature/movement_text",
    ] {
        assert!(!new.contains(path), "{new}");
    }
    assert_eq!(
        model(&formats::convert("Rook.md", &old).unwrap()),
        model(&new)
    );
    assert!(new.contains("| Speed | 40 |"));
    let formula = new.replace("| Hit Points | 22 |", "| Hit Points | 22 (4d8 + 4) |");
    assert_eq!(
        model(&formula)["splat_creature"]["hit_points_text"],
        Value::from("22 (4d8 + 4)")
    );
    let conflict = old.replace("type: beast", "type: beast, hit_points_text: '30'");
    assert_eq!(
        model(&conflict)["splat_creature"]["hit_points_text"],
        Value::from("30")
    );
    assert!(
        formats::convert("Rook.md", &conflict).is_err(),
        "A conflicting legacy value needs review"
    );
}

#[test]
fn i20_x26_all_machine_fences_are_folded_and_empty_defaults_are_omitted() {
    for (kind, fields) in [
        (
            "creature",
            "splat_creature: {size: medium, type: beast}\nsplat_rule: {parameters: {a: b}}",
        ),
        ("rule", "splat_rule: {parameters: {a: b}}"),
        ("effect", "splat_effect: {duration: 3}"),
        (
            "encounter",
            "splat_encounter: {roster: [], sides: {party: {hostile_to: [enemies]}, enemies: {hostile_to: [party]}}, assumptions: [], completion: {mode: side_defeated, side: enemies}, limits: {rounds: 30, resolution_events: 10000}}\nsplat_rule: {parameters: {a: b}}",
        ),
    ] {
        let old = note(kind, fields, "# Rook\n");
        let new = formats::convert("Rook.md", &old).unwrap_or_else(|e| panic!("{kind}: {e}"));
        assert!(
            new.contains("> [!note]- Splat data (edit in Splat)"),
            "{new}"
        );
        assert!(!new.contains("## Splat data"));
        assert_eq!(
            model(&formats::convert("Rook.md", &old).unwrap()),
            model(&new)
        );
        assert_eq!(new, formats::convert("Rook.md", &new).unwrap());
        if kind == "encounter" {
            let mut encounter = model(&new)["splat_encounter"].clone();
            encounter["limits"]["rounds"] = Value::from(40);
            encounter["completion"]["side"] = Value::from("party");
            let revised =
                formats::patch_model(&new, &[("splat_encounter", Some(encounter.clone()))])
                    .unwrap();
            assert_eq!(model(&revised)["splat_encounter"], encounter);
            assert!(revised.contains("> [!note]- Splat data (edit in Splat)"));
        }
    }
    for kind in ["rule", "effect", "rule_group"] {
        let old = note(kind, &format!("splat_{kind}: {{}}"), "# Rook\n");
        let new = formats::convert("Rook.md", &old).unwrap();
        assert!(!new.contains("```"), "{new}");
        assert_eq!(
            model(&formats::convert("Rook.md", &old).unwrap()),
            model(&new)
        );
    }
}

#[test]
fn world_conversion_keeps_both_reference_identity_and_readable_link() {
    let old = note(
        "world",
        "splat_world: {type: npc, creature: {id: '00000000-0000-0000-0000-000000000002', link: '[[Watchkeeper Guard]]'}}",
        "# Rook\n",
    );
    let new = formats::convert("Rook.md", &old).unwrap();
    assert!(
        new.contains("[[Watchkeeper Guard]] %% splat id=00000000-0000-0000-0000-000000000002 %%")
    );
    assert_eq!(model(&old), model(&new));
    assert!(Entry::parse("Rook.md", &new).unwrap().flags.is_empty());
}

#[test]
fn vault_conversion_saved_creature_values_are_authoritative() {
    let old = note(
        "creature",
        "splat_creature: {size: 'medium or small', type: 'humanoid (scout)', hit_points_text: '19 (3d8 + 6)', movement_text: '30 ft., fly 60 ft.', cr_text: '2 (XP 450; PB +2)'}\nsplat_stats: {max_hp: {value: 19}, speed: {value: 60}}",
        "# Rook\n\n## Source text\n\nLarge beast, Unaligned\\\nHP 99 (18d10)\\\nSpeed 20 ft.\\\n\n### Actions\n\nA synthetic attack.\n",
    );
    let converted = formats::convert("Rook.md", &old).unwrap();
    assert_eq!(model(&old), model(&converted));
    assert!(converted.contains("> [!quote]- Printed stat block (not read; edit the table above)\n> Large beast, Unaligned\\\n> HP 99 (18d10)\\\n> Speed 20 ft.\\\n"));
}

#[test]
fn vault_conversion_spell_and_item_optional_values_round_trip() {
    for (kind, data) in [
        (
            "spell",
            "splat_spell: {concentration: false, ritual: false, casting_time: Action, range_text: '120 feet', duration: Instantaneous}",
        ),
        (
            "spell",
            "splat_spell: {concentration: true, ritual: true, casting_time: Action, range_text: Sight, duration: 'up to 1 minute'}",
        ),
        (
            "item",
            "splat_item: {category: wondrous_item, equipment: {attunement: ''}}",
        ),
        (
            "item",
            "splat_item: {category: wondrous_item, rarity: unknown}",
        ),
        (
            "spell",
            "splat_spell: {range: {feet: 60}, range_text: 'special range'}",
        ),
    ] {
        let old = note(kind, data, "# Rook\n");
        let converted = formats::convert("Rook.md", &old).unwrap();
        assert_eq!(model(&old), model(&converted));
    }
}

#[test]
fn vault_conversion_missing_reference_keeps_identity() {
    let id = "00000000-0000-0000-0000-000000000099";
    let old = note(
        "encounter_group",
        &format!(
            "splat_encounter_group: {{side: enemies, members: [{{creature: {{id: {id}}}, count: 1}}]}}"
        ),
        "# Rook\n",
    );
    let converted = formats::convert_resolved("Rook.md", &old, []).unwrap();
    assert_eq!(model(&old), model(&converted));
    assert!(converted.contains(id));
    formats::verify_conversion("Rook.md", &old, &converted, &[]).unwrap();
    assert!(
        formats::verify_conversion(
            "Rook.md",
            &old,
            &converted.replace("splat_name: Rook", "splat_name: Changed"),
            &[]
        )
        .is_err()
    );
}

#[test]
fn vault_conversion_composite_type_with_independent_tags_round_trips() {
    let old = note(
        "creature",
        "splat_creature: {type: 'humanoid (Scout)', tags_text: '(Ranger)'}",
        "# Rook\n",
    );
    let converted = formats::convert("Rook.md", &old).unwrap();
    assert_eq!(model(&old), model(&converted));
    assert!(converted.contains("| Type | humanoid (Scout) |"));
    assert!(converted.contains("| Type Tags | (Ranger) |"));
}

#[test]
fn vault_conversion_requirement_does_not_choose_an_ambiguous_package_path() {
    let target = super::legacy::parse(
        "Package.md",
        &note(
            "package",
            "splat_namespace: rook\nsplat_key: rook:package/core",
            "# Rook\n",
        ),
    )
    .unwrap();
    let other = super::legacy::parse(
        "Package.md",
        &note("package", "splat_namespace: other", "# Other\n")
            .replace("000000000001", "000000000002"),
    )
    .unwrap();
    let old = note(
        "package",
        "splat_namespace: third\nsplat_requires: [{key: 'rook:package/core'}]",
        "# Rook\n",
    );
    let entries = [target, other];
    let converted = formats::convert_resolved("Third.md", &old, &entries).unwrap();
    formats::verify_conversion("Third.md", &old, &converted, &entries).unwrap();
}

#[test]
fn loot_is_the_last_term_of_a_changes_cell_in_a_roster_and_a_group() {
    let key = "dnd2024:creatures/goblin_warrior";
    let roster = note("encounter", "", &format!("# Rook\n\n## Roster\n\n| Side | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| enemies | [Goblin](splat:{key}) | | HP +5, Loot Iron key, rusty; Roll gems | %% splat slot=actor_one %% |\n")).replace("splat_schema: 1", "splat_schema: 2");
    let slot = &model(&roster)["splat_encounter"]["roster"][0];
    assert_eq!(slot["overrides"]["loot"], "Iron key, rusty; Roll gems");
    assert_eq!(slot["overrides"]["max_hp"]["add"], 5);
    // A starting HP goes before the loot, which keeps its commas.
    let mut encounter = model(&roster)["splat_encounter"].clone();
    encounter["roster"][0]["initial"] = serde_yaml_ng::from_str("{hp: 3}").unwrap();
    let edited = formats::patch_model(&roster, &[("splat_encounter", Some(encounter))]).unwrap();
    assert!(
        edited.contains("| HP +5, Start HP 3, Loot Iron key, rusty; Roll gems |"),
        "{edited}"
    );
    assert_eq!(
        model(&edited)["splat_encounter"]["roster"][0]["overrides"]["loot"],
        "Iron key, rusty; Roll gems"
    );
    // A group member's loot is its own, and taking it out leaves the other changes.
    let group = note("encounter_group", "splat_side: enemies", &format!("# Rook\n\n| Count | Creature | Name | Changes | Note |\n| --- | --- | --- | --- | --- |\n| 2 | [Goblin](splat:{key}) | | Loot 3 sp | |\n")).replace("splat_schema: 1", "splat_schema: 2");
    let mut members = model(&group)["splat_encounter_group"].clone();
    assert_eq!(members["members"][0]["overrides"]["loot"], "3 sp");
    members["members"][0]["overrides"] = serde_yaml_ng::from_str("{max_hp: {add: 2}}").unwrap();
    let edited = formats::patch_model(&group, &[("splat_encounter_group", Some(members))]).unwrap();
    assert!(
        edited.contains("| 2 | [Goblin](splat:dnd2024:creatures/goblin_warrior) | | HP +2 | |"),
        "{edited}"
    );
    let empty = group.replace("Loot 3 sp", "HP +1, Loot");
    assert!(
        super::legacy::parse("Rook.md", &empty).unwrap().flags[0]
            .message
            .contains("Loot needs")
    );
}

#[test]
fn fmt_f2_hit_points_hint_shows_the_shape_and_other_numbers_keep_theirs() {
    let (_, new) = creature_pair();
    let bad = new
        .replace("| Hit Points | 22 (4d8 + 4) |", "| HP | about twenty |")
        .replace("| Armor Class | 13 |", "| Armor Class | about fifteen |");
    let entry = super::legacy::parse("Rook.md", &bad).unwrap();
    let messages: Vec<&str> = entry.flags.iter().map(|f| f.message.as_str()).collect();
    // The alias row still names the stat by its full name, with the example.
    assert!(
        messages
            .iter()
            .any(|m| m.starts_with("Hit Points \"about twenty\" not read")
                && m.ends_with(". Start with a number, such as 22 (4d8 + 4).")),
        "{messages:?}"
    );
    // Only Hit Points changed: Armor Class keeps the short hint.
    assert!(
        messages
            .iter()
            .any(|m| m.starts_with("Armor Class \"about fifteen\" not read")
                && m.ends_with(". Use a number first.")),
        "{messages:?}"
    );
}
