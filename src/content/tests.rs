use super::*;

fn note(key: &str, kind: &str, extra: &str, body: &str) -> String {
    format!(
        "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: {kind}\nsplat_key: 'test:{key}'\nsplat_name: Example\n{extra}\n---\n{body}",
        Uuid::new_v4()
    )
}

fn parse(key: &str, kind: &str, extra: &str, body: &str) -> Entry {
    Entry::parse(key, &note(key, kind, extra, body)).unwrap()
}

fn table(marker: &str, rows: &str) -> String {
    format!(
        "<!-- splat:inventory{marker} -->\n\n| Key | Item | Quantity | Equipment |\n| --- | --- | --- | --- |\n{rows}\n"
    )
}

#[test]
fn reads_all_seventeen_design_notes_together() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/design/examples");
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
        note("bad", "item", "splat_item: {}", "").replace("splat_schema: 1", "splat_schema: 2");
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
            &format!("splat_item: {{}}\nsplat_origin: {reference}"),
            "",
        );
        assert!(!validate(&[entry]).errors.is_empty());
    }
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
            .is_err()
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
