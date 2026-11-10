use super::*;

fn note(key: &str, kind: &str, extra: &str, body: &str) -> String {
    let text = format!(
        "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: {kind}\nsplat_key: 'test:{key}'\nsplat_name: Example\n{extra}\n---\n{body}",
        Uuid::new_v4()
    );
    crate::content::formats::convert("Fixture.md", &text)
        .unwrap_or_else(|_| text.replace("splat_schema: 1", "splat_schema: 2"))
}

fn parse(key: &str, kind: &str, extra: &str, body: &str) -> Entry {
    Entry::parse(key, &note(key, kind, extra, body)).unwrap()
}

#[test]
fn editing_cloned_fields_keeps_other_entries_and_serialization_independent() {
    let source = parse(
        "shared",
        "creature",
        "splat_creature: {}\ncustom:\n  hp: 12",
        "Body",
    );
    let original = serde_json::to_value(&source).unwrap();
    let mut edited = source.clone();
    let peer = source.clone();
    edited.fields["custom"]["hp"] = Value::from(99);
    edited.fields.remove("splat_name");
    assert_eq!(source.fields["custom"]["hp"].as_i64(), Some(12));
    assert_eq!(serde_json::to_value(&source).unwrap(), original);
    assert_eq!(serde_json::to_value(&peer).unwrap(), original);

    let mut expected = original;
    expected["fields"]["custom"]["hp"] = 99.into();
    expected["fields"]
        .as_object_mut()
        .unwrap()
        .remove("splat_name");
    let wire = serde_json::to_value(&edited).unwrap();
    assert_eq!(wire, expected);
    let mut restored: Entry = serde_json::from_value(wire).unwrap();
    restored.fields["custom"]["hp"] = Value::from(7);
    assert_eq!(edited.fields["custom"]["hp"].as_i64(), Some(99));
    assert_eq!(restored.fields["custom"]["hp"].as_i64(), Some(7));
}

fn table(marker: &str, rows: &str) -> String {
    format!(
        "<!-- splat:inventory{marker} -->\n\n| Key | Item | Quantity | Equipment |\n| --- | --- | --- | --- |\n{rows}\n"
    )
}

#[test]
fn damaged_frontmatter_with_splat_like_keys_is_reported() {
    let temp = tempfile::tempdir().unwrap();
    for (path, text) in [
        ("No fence.md", "splat_id: lost\nsplat_name: Wolf\n---\nBody"),
        (
            "No prefix.md",
            "---\nschema: 1\nid: lost\nkind: creature\nname: Wolf\n---\nBody",
        ),
    ] {
        std::fs::write(temp.path().join(path), text).unwrap();
    }
    let (entries, errors) = read_directory(temp.path()).unwrap();
    assert!(entries.is_empty());
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|error| error.path == "No fence.md"));
    assert!(errors.iter().any(|error| error.path == "No prefix.md"));
}

#[test]
fn directory_keeps_readable_entries_skips_hidden_files_and_names_unreadable_notes() {
    let temp = tempfile::tempdir().unwrap();
    let wolf = note("wolf", "creature", "splat_creature: {}", "Wolf");
    std::fs::write(temp.path().join("Wolf.md"), &wolf).unwrap();
    for hidden in [".trash", ".stversions"] {
        std::fs::create_dir(temp.path().join(hidden)).unwrap();
        std::fs::write(temp.path().join(hidden).join("Wolf.md"), &wolf).unwrap();
    }
    std::fs::write(temp.path().join("Latin.md"), [0xff, 0xfe]).unwrap();
    let (entries, errors) = read_directory(temp.path()).unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].path, "Latin.md");
    assert!(errors[0].message.contains("UTF-8"));
}

#[test]
fn reads_all_seventeen_design_notes_together() {
    let root =
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).join("docs/design/examples");
    let (entries, errors) = read_directory(&root).unwrap();
    assert!(errors.is_empty(), "{errors:?}");
    assert_eq!(entries.len(), 17);
    let report = validate(&entries);
    assert!(report.errors.is_empty(), "{:?}", report.errors);
    assert_eq!(report.executable_entries, 0);
    let warden = entries.iter().find(|e| e.name == "Test Warden").unwrap();
    assert_eq!(warden.inventory.len(), 7);
    let setup = entries.iter().find(|e| e.name == "Supply Pack").unwrap();
    assert_eq!(setup.inventory[1].container.as_deref(), Some("pouch"));
}

#[test]
fn preserves_unknown_properties_and_body_without_writes() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("note.md");
    let raw = format!(
        "\u{feff}{}",
        note(
            "item",
            "item",
            "splat_item: {}\ntags: [mine]\nsplat_future: {arbitrary: data}",
            "# Prose\n\n`splat:use x`\n"
        )
        .replace('\n', "\r\n")
    );
    std::fs::write(&path, &raw).unwrap();
    std::fs::write(
        temp.path().join("ordinary.md"),
        "```yaml\nsplat_schema: 1\n```\n",
    )
    .unwrap();
    let (entries, errors) = read_directory(temp.path()).unwrap();
    assert!(errors.is_empty());
    assert_eq!(entries.len(), 1);
    assert!(field(&entries[0].fields, "splat_future").is_some());
    assert_eq!(validate(&entries).warnings.len(), 1);
    assert!(entries[0].body.contains("`splat:use x`"));
    assert_eq!(std::fs::read_to_string(path).unwrap(), raw);
}

#[test]
fn parallel_directory_reads_preserve_order_errors_and_external_edits() {
    let temp = tempfile::tempdir().unwrap();
    let mut expected = Vec::new();
    let mut broken = Vec::new();
    for i in 0..64 {
        let path = format!("{i:02}.md");
        let raw = if i % 7 == 0 {
            broken.push(path.clone());
            "---\nsplat_schema: 999\n---\n".to_owned()
        } else {
            let raw = note(&format!("item_{i}"), "item", "splat_item: {}", "Original");
            expected.push(Entry::parse(&path, &raw).unwrap());
            raw
        };
        std::fs::write(temp.path().join(path), raw).unwrap();
    }
    std::fs::write(temp.path().join("ordinary.md"), "# Ordinary note").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(temp.path().join("01.md"), temp.path().join("link.md")).unwrap();
    for _ in 0..2 {
        let (entries, errors) = read_directory(temp.path()).unwrap();
        assert_eq!(
            serde_json::to_value(entries).unwrap(),
            serde_json::to_value(&expected).unwrap()
        );
        assert_eq!(
            errors.iter().map(|e| &e.path).collect::<Vec<_>>(),
            broken.iter().collect::<Vec<_>>()
        );
    }
    let changed = note(
        "replacement",
        "item",
        "splat_item: {}",
        "Changed externally",
    );
    std::fs::write(temp.path().join("01.md"), &changed).unwrap();
    let (entries, _) = read_directory(temp.path()).unwrap();
    assert_eq!(entries[0].key.as_deref(), Some("test:replacement"));
    assert_eq!(
        entries[0].body,
        Entry::parse("replacement", &changed).unwrap().body
    );
}

#[test]
fn rejects_duplicate_yaml_keys_at_any_depth_and_future_schema() {
    for extra in [
        "splat_item: {}\nsplat_name: Again",
        "splat_item: {mass: 1, mass: 2}",
        "splat_item: {}\nsplat_actions: {use: {}, use: {}}",
    ] {
        assert!(
            Entry::parse("bad", &note("bad", "item", extra, "")).is_err(),
            "Accepted {extra}"
        );
    }
    let raw =
        note("bad", "item", "splat_item: {}", "").replace("splat_schema: 2", "splat_schema: 3");
    assert!(
        Entry::parse("bad", &raw)
            .unwrap_err()
            .contains("Unsupported")
    );
}

#[test]
fn detects_duplicate_ids_keys_namespaces_and_bad_references() {
    let original = parse("a", "package", "splat_namespace: test", "");
    let mut duplicate = original.clone();
    duplicate.path = "second.md".into();
    let report = validate(&[original, duplicate]);
    for expected in ["Duplicate UUID", "Duplicate key", "Duplicate namespace"] {
        assert!(report.errors.iter().any(|e| e.message.contains(expected)));
    }
    for reference in [
        "{key: 'test:missing'}",
        "{id: 00000000-0000-0000-0000-000000000001, key: 'test:missing'}",
    ] {
        let entry = parse(
            "a",
            "item",
            &format!("splat_item: {{}}\nsplat_requires: [{reference}]"),
            "",
        );
        assert!(!entry.flags.is_empty() || !validate(&[entry]).errors.is_empty());
    }
}

#[test]
fn copy_origins_may_be_unavailable_but_must_be_valid_references() {
    let copy = parse(
        "copy",
        "item",
        "splat_item: {}\nsplat_origin: {key: 'test:removed'}",
        "",
    );
    assert!(validate(&[copy]).errors.is_empty());
    let bad = parse(
        "copy",
        "item",
        "splat_item: {}\nsplat_origin: {id: invalid, key: 'test:removed'}",
        "",
    );
    assert!(!bad.flags.is_empty() || !validate(&[bad]).errors.is_empty());
}

#[test]
fn rejects_invalid_inventory_authority_and_containment() {
    let a = table("", "| pack | test:pack | 1 | carried |");
    for body in [
        format!("{a}\n{a}"),
        table("", "| pack | test:pack | 0 | carried |"),
        table(" container=missing", "| child | test:pack | 1 | stored |"),
        table(" container=pack", "| pack | test:pack | 1 | stored |"),
        format!(
            "{}\n{}",
            table(" container=b", "| a | test:pack | 1 | stored |"),
            table(" container=a", "| b | test:pack | 1 | stored |")
        ),
        "<!-- splat:inventory -->\n\nIntervening prose\n\n".to_string() + &a,
    ] {
        assert!(
            Entry::parse("bad", &note("bad", "creature", "splat_creature: {}", &body)).is_err(),
            "Accepted {body}"
        );
    }
    let item = parse("pack", "item", "splat_item: {containment: {}}", "");
    let setup = parse("setup", "container", "splat_item: {key: 'test:pack'}", "");
    let body = format!(
        "{}\n{}",
        table("", "| pack | test:setup | 1 | carried |"),
        table(" container=pack", "| child | test:pack | 1 | stored |")
    );
    let actor = parse("actor", "creature", "splat_creature: {}", &body);
    assert!(
        validate(&[item, setup, actor])
            .errors
            .iter()
            .any(|e| e.message.contains("cannot merge"))
    );
}

#[test]
fn validates_stat_alternatives_and_pool_bounds() {
    for extra in [
        "splat_stats: {hp: {value: 10, derive: 10}}",
        "splat_stats: {hp: {}}",
        "splat_resources: {fire: {max: 4, initial: 5}}",
    ] {
        assert!(
            Entry::parse(
                "bad",
                &note(
                    "bad",
                    "creature",
                    &format!("splat_creature: {{}}\n{extra}"),
                    ""
                )
            )
            .map_or(true, |entry| !entry.flags.is_empty())
        );
    }
}

#[test]
fn verifies_separate_item_pools_and_rejects_unknown_or_overfull_pools() {
    let item = parse(
        "wand",
        "item",
        "splat_item: {}\nsplat_resources: {fire: {max: 4}, healing: {max: 2}}",
        "",
    );
    let body = table("", "| wand | test:wand | 1 | carried |");
    for (resources, valid) in [
        ("{fire: 4, healing: 1}", true),
        ("{fire: 5}", false),
        ("{unknown: 1}", false),
        ("{healing: -1}", false),
    ] {
        let actor = parse(
            "actor",
            "creature",
            &format!(
                "splat_creature: {{}}\nsplat_inventory_details: {{wand: {{resources: {resources}}}}}"
            ),
            &body,
        );
        let report = validate(&[item.clone(), actor]);
        assert_eq!(report.errors.is_empty(), valid, "{:?}", report.errors);
    }
}

#[test]
fn rejects_cross_setup_cycles_and_even_empty_tables_with_dual_authority() {
    let item = parse("pack", "item", "splat_item: {containment: {}}", "");
    let setup = parse(
        "setup",
        "container",
        "splat_item: {key: 'test:pack'}",
        &table("", "| pack | test:setup | 1 | stored |"),
    );
    assert!(
        validate(&[item.clone(), setup.clone()])
            .errors
            .iter()
            .any(|e| e.message.contains("cycle"))
    );
    let mut setup = setup;
    setup.inventory.clear();
    let actor = parse(
        "actor",
        "creature",
        "splat_creature: {}",
        &(table("", "| pack | test:setup | 1 | carried |") + "\n" + &table(" container=pack", "")),
    );
    assert!(
        validate(&[item, setup, actor])
            .errors
            .iter()
            .any(|e| e.message.contains("cannot merge"))
    );
}

#[test]
fn allows_missing_optional_dependencies_and_ignores_non_splat_metadata() {
    let entry = parse(
        "a",
        "item",
        "splat_item: {}\nsplat_optional: [{key: 'test:uninstalled'}]\nexternal_metadata: {id: not-a-uuid}",
        "",
    );
    assert!(validate(&[entry]).errors.is_empty());
}

#[test]
fn body_tables_and_blocks_reconstruct_data_independent_of_headings_and_order() {
    let body = r#"
# My layout
<!-- splat:yaml path=splat_rule/parameters/rows/1/effect -->

```yaml
on_hit: {damage: 8d6, type: fire}
```

<!-- splat:rows path=splat_rule/parameters/rows -->

| Row | level | label |
| --- | --- | --- |
| 1 | 2 | `"two\|three"` |
| 0 | 1 | `"true"` |

## Renamed
<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| mechanic | test:rule |
| implementation | test:adapter |

<!-- splat:abilities path=splat_stats -->

| Ability | Score | Save override |
| --- | --- | --- |
| Intelligence | 6 | +2 |
| Strength | 18 | |
"#;
    let entry = parse("body", "rule", "", body);
    let expected: Value = serde_yaml_ng::from_str(
        r#"
splat_rule:
  mechanic: test:rule
  implementation: test:adapter
  parameters:
    rows:
    - {level: 1, label: 'true'}
    - {level: 2, label: 'two|three', effect: {on_hit: {damage: 8d6, type: fire}}}
splat_stats:
  intelligence: {value: 6}
  save_intelligence: {value: 2}
  strength: {value: 18}
"#,
    )
    .unwrap();
    for (key, value) in expected.as_mapping().unwrap() {
        assert_eq!(entry.fields.get(key), Some(value));
    }
    assert!(
        crate::execution::model::compile(&entry)
            .unwrap_err()
            .contains("test:adapter")
    );
    assert_eq!(entry.body, body);
}

#[test]
fn body_data_rejects_conflicts_malformed_tables_and_dangling_markers() {
    for body in [
        "<!-- splat:yaml path=splat_item -->\n\n```yaml\n{}\n```",
        "<!-- splat:yaml path=splat_stats -->\n\n```yaml\nstrength: {value: 1, value: 2}\n```",
        "<!-- splat:yaml path=splat_stats -->\n\n```yaml\nstrength: {value: 1}\n",
        "<!-- splat:values path=splat_stats -->\n\nIntervening text\n\n| Field | Value |\n|---|---|\n| strength | 1 |",
        "<!-- splat:values path=splat_stats -->\n\n| Field | Value |\n|---|---|\n| strength | 1 | discarded |",
        "<!-- splat:values path=splat_stats -->\n\n| Field | Value |\n|---|---|\n| strength |",
        "<!-- splat:values path=splat_stats -->\n\n| Field | Value |\n|---|---|\n| strength | 1 |\n| strength | 2 |",
        "<!-- splat:rows path=splat_rule/parameters/rows -->\n\n| Row | value |\n|---|---|\n| 1 | 3 |",
        "<!-- splat:values path=splat_stats -->",
        "<!-- splat:values path=splat_name -->\n\n| Field | Value |\n|---|---|\n| label | changed |",
        "<!-- splat:unknown path=splat_stats -->\n",
    ] {
        assert!(
            Entry::parse("invalid", &note("invalid", "item", "splat_item: {}", body))
                .map_or(true, |entry| !entry.flags.is_empty()),
            "Accepted: {body}"
        );
    }
    let conflict = "<!-- splat:yaml path=splat_stats/strength -->\n\n```yaml\nvalue: 1\n```\n\n<!-- splat:values path=splat_stats/strength -->\n\n| Field | Value |\n|---|---|\n| value | 2 |\n";
    assert!(
        Entry::parse(
            "conflict",
            &note("conflict", "item", "splat_item: {}", conflict)
        )
        .is_err()
    );
}

#[test]
fn unmarked_examples_are_not_interpreted_as_data() {
    let body = "```markdown\n<!-- splat:yaml path=splat_item -->\n```\n\n```yaml\nsplat_item: {changed: true}\n```\n\n| Field | Value |\n|---|---|\n| ignored | true |\n";
    let entry = parse("example", "item", "splat_item: {}", body);
    assert!(
        field(&entry.fields, "splat_item")
            .unwrap()
            .as_mapping()
            .unwrap()
            .is_empty()
    );
}

fn mapped_sample() -> String {
    r#"<!-- splat:table
path: splat_rule/parameters/options
key: Variant
order: [arrows, bolts]
columns:
  Variant: {path: id, type: symbol}
  Weight: {path: mass, type: quantity}
  Price: {path: price, type: quantity}
  Spell: {path: spell, type: reference, optional: true}
  Healing: {path: heal/roll, type: dice, optional: true}
  Note: {path: note, type: string, optional: true}
-->

| Variant | Weight | Price | Spell | Healing | Note |
| --- | --- | --- | --- | --- | --- |
| Bolts | 1½ lb. | 1 GP | [Any display text](elsewhere.md "test:spells/heal") | 2d4+2 | false |
| Arrows | 1 lb | 4 CP | | | |

### This heading can change
<!-- splat:yaml path=splat_rule/parameters/options/@bolts/extra -->

```yaml
requires: held
```
"#
    .into()
}

#[test]
fn mapped_tables_preserve_nested_values_and_stable_rows() {
    let body = mapped_sample();
    let entry = parse("mapped", "rule", "", &body);
    let rule = field(&entry.fields, "splat_rule").unwrap();
    let expected: Value = serde_yaml_ng::from_str(
        r#"
parameters:
  options:
  - {id: arrows, mass: {value: 1, unit: lb}, price: {value: 4, unit: cp}}
  - id: bolts
    mass: {value: 1.5, unit: lb}
    price: {value: 1, unit: gp}
    spell: {key: 'test:spells/heal'}
    heal: {roll: {count: 2, sides: 4, add: 2}}
    note: 'false'
    extra: {requires: held}
"#,
    )
    .unwrap();
    assert_eq!(rule, &expected);
    // Move the block before its table; the key, not position, binds it.
    let (table, block) = body.split_once("### This heading").unwrap();
    let moved = format!("### This heading{block}\n{table}");
    assert_eq!(
        parse("moved", "rule", "", &moved)
            .fields
            .get(Value::String("splat_rule".into())),
        Some(rule)
    );
}

#[test]
fn mapped_tables_reject_conflicts_bad_codecs_and_unknown_rows() {
    let original = mapped_sample();
    for (from, to) in [
        ("order: [arrows, bolts]", "order: [arrows, arrows]"),
        ("order: [arrows, bolts]", "order: [arrows, missing]"),
        ("@bolts/extra", "@unknown/extra"),
        ("@bolts/extra", "@bolts/mass"),
        ("@bolts/extra", "1/extra"),
        ("1½ lb.", "bad½ lb"),
        ("1½ lb.", "1/0 lb"),
        ("2d4+2", "2d1+2"),
        ("2d4+2", "2d4+oops"),
        ("test:spells/heal", "not-an-identity"),
        ("type: symbol", "type: execute"),
        ("path: price", "path: mass/value"),
        ("| Arrows | 1 lb", "| Bolts | 1 lb"),
        ("| Arrows | 1 lb", "| Arrows | "),
        ("key: Variant", "key: Variant\nkey: Weight"),
    ] {
        let bad = original.replace(from, to);
        assert!(
            Entry::parse("bad", &note("bad", "rule", "", &bad)).is_err(),
            "Accepted {from} -> {to}"
        );
    }
}

#[test]
fn mapped_root_ranges_and_custom_compound_fields_are_lossless() {
    let body = r#"<!-- splat:table
path: splat_rule/parameters/results
key: Key
order: [first, second]
columns:
  Key: {type: key}
  Roll: {path: '', type: range, fields: [start, end]}
  Damage: {path: damage, type: dice, fields: [count, sides, modifier]}
-->

| Key | Roll | Damage |
| --- | --- | --- |
| first | 1–20 | 1d6-2 |
| second | 21 | 2d6 |
"#;
    let e = parse("range", "rule", "", body);
    assert_eq!(field(&e.fields,"splat_rule").unwrap(),&serde_yaml_ng::from_str::<Value>("parameters: {results: [{start: 1, end: 20, damage: {count: 1, sides: 6, modifier: -2}}, {start: 21, end: 21, damage: {count: 2, sides: 6, modifier: 0}}]}").unwrap());
    assert!(
        Entry::parse(
            "bad",
            &note("bad", "rule", "", &body.replace("1–20", "20–1"))
        )
        .is_err()
    );
}

#[test]
fn mapped_nullable_values_and_sections_preserve_distinctions() {
    let body = r#"<!-- splat:table
path: splat_rule/parameters/rows
key: Key
order: [first, second, third]
columns:
  Key: {type: key}
  Value: {path: value, type: number, optional: true, nullable: true}
  Details: {type: section, optional: true}
-->

| Key | Value | Details |
| --- | --- | --- |
| first | null | [Rules](#^stable "section:first") |
| second | 0.0 | |
| third | | [Rules](#^other "section:third") |

<!-- splat:yaml path=splat_rule/parameters/rows/@first/rule -->

```yaml
mode: immediate
```

<!-- splat:yaml path=splat_rule/parameters/rows/@third/rule -->

```yaml
mode: next_turn
```
"#;
    let entry = parse("nullable", "rule", "", body);
    assert_eq!(field(&entry.fields,"splat_rule").unwrap(),&serde_yaml_ng::from_str::<Value>("parameters: {rows: [{value: null, rule: {mode: immediate}}, {value: 0.0}, {rule: {mode: next_turn}}]}").unwrap());
    let actual = crate::content::contracts::specification(&entry).unwrap();
    assert_eq!(
        actual["splat_rule"]["parameters"]["rows"][1]["value"].to_string(),
        "0.0"
    );
    for replacement in ["NaN", "infinity", "bad½", "1,2"] {
        assert!(
            Entry::parse(
                "bad",
                &note("bad", "rule", "", &body.replace("0.0", replacement))
            )
            .is_err()
        );
    }
}

#[test]
fn nested_mapped_tables_use_stable_paths_at_each_level() {
    let body = r#"<!-- splat:table
path: splat_rule/parameters/groups/@winter/spells
key: Spell
order: [ice]
columns:
  Spell: {path: spell, type: reference}
-->

| Spell |
| --- |
| [Ice](ice.md "test:spells/ice") |

<!-- splat:table
path: splat_rule/parameters/groups
key: Group
order: [winter, summer]
columns:
  Group: {path: name, type: symbol}
-->

| Group |
| --- |
| Summer |
| Winter |
"#;
    let e = parse("nested", "rule", "", body);
    assert_eq!(field(&e.fields,"splat_rule").unwrap(),&serde_yaml_ng::from_str::<Value>("parameters: {groups: [{name: winter, spells: [{spell: {key: 'test:spells/ice'}}]}, {name: summer}]}").unwrap());
}

#[test]
fn a_file_replaced_during_a_load_is_not_a_failed_load() {
    let temp = tempfile::tempdir().unwrap();
    let gone = temp.path().join("gone.md.tmp");
    assert_eq!(unless_vanished(std::fs::read(&gone)), Ok(None));
    assert_eq!(
        unless_vanished(std::fs::read(temp.path().join("gone.md.tmp/inside"))),
        Ok(None)
    );
    // A directory read as a file is a real failure.
    assert!(unless_vanished(std::fs::read(temp.path())).is_err());
}

#[test]
fn iter8_e_campaign_plain_root_manifest_names_are_ignored() {
    let temp = tempfile::tempdir().unwrap();
    let campaign = note(
        "campaign",
        "campaign",
        "splat_namespace: campaign",
        "Campaign",
    );
    std::fs::write(temp.path().join("Campaign.md"), campaign).unwrap();
    for filename in ["Module.md", "Package.md"] {
        std::fs::write(temp.path().join(filename), "Notes on this adventure.").unwrap();
    }
    let folder = read_folder(temp.path()).unwrap();
    assert!(folder.errors.is_empty(), "{:?}", folder.errors);
    assert_eq!(folder.entries.len(), 1);
    assert_eq!(folder.entries[0].kind, "campaign");
}
