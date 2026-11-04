use super::{Entry, legacy};

#[test]
fn all_old_note_kinds_are_flagged_and_not_loaded() {
    let root = tempfile::tempdir().unwrap();
    let kinds = [
        "package",
        "campaign",
        "creature",
        "item",
        "spell",
        "effect",
        "container",
        "encounter",
        "encounter_group",
        "world",
        "sequence",
        "rule",
        "rule_group",
        "strategy",
        "seed",
        "reference",
    ];
    for kind in kinds {
        let text = format!(
            "---\nsplat_schema: 1\nsplat_id: {}\nsplat_kind: {kind}\nsplat_name: Old\n---\n",
            uuid::Uuid::new_v4()
        );
        assert_eq!(Entry::parse(kind, &text).unwrap_err(), legacy::OLD_LAYOUT);
        std::fs::write(root.path().join(format!("{kind}.md")), text).unwrap();
    }
    let character = include_str!("../../tests/fixtures/conversion/character.old.md");
    assert_eq!(
        Entry::parse("Character.md", character).unwrap_err(),
        legacy::OLD_LAYOUT
    );
    std::fs::write(root.path().join("Character.md"), character).unwrap();
    let (entries, errors) = super::read_directory(root.path()).unwrap();
    assert!(entries.is_empty());
    assert_eq!(errors.len(), kinds.len() + 1);
    assert!(
        errors
            .iter()
            .all(|e| e.path.ends_with(".md") && e.message == legacy::OLD_LAYOUT)
    );
}

#[test]
fn every_builtin_note_is_schema_two_without_flags() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("builtin");
    let (entries, errors) = super::read_directory(&root).unwrap();
    assert!(errors.is_empty(), "{errors:?}");
    assert_eq!(entries.len(), 3550);
    let flags: Vec<_> = entries
        .iter()
        .filter(|e| !e.flags.is_empty())
        .map(|e| format!("{}: {:?}", e.path, e.flags))
        .collect();
    assert!(flags.is_empty(), "{}", flags.join("\n"));
    assert!(
        entries
            .iter()
            .all(|e| e.fields["splat_schema"].as_u64() == Some(2))
    );
}

#[test]
fn conversion_keeps_models_and_exact_output_for_every_note_kind() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/conversion");
    for file in std::fs::read_dir(&root).unwrap() {
        let path = file.unwrap().path();
        if !path.to_string_lossy().ends_with(".old.md") {
            continue;
        }
        let old = std::fs::read_to_string(&path).unwrap();
        let expected = std::fs::read_to_string(
            path.with_file_name(
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .replace(".old.md", ".new.md"),
            ),
        )
        .unwrap();
        let new = super::formats::convert("Fixture.md", &old).unwrap();
        assert_eq!(new, expected, "{}", path.display());
        let before = legacy::parse("Fixture.md", &old).unwrap();
        let after = Entry::parse("Fixture.md", &new).unwrap();
        let mut fields = (*before.fields).clone();
        fields.insert("splat_schema".into(), 2.into());
        if ["campaign", "encounter", "encounter_group"].contains(&after.kind.as_str()) {
            assert_eq!(
                super::list_formats::canonical(&fields),
                super::list_formats::canonical(&after.fields),
                "{}",
                path.display()
            );
        } else {
            assert_eq!(fields, *after.fields, "{}", path.display());
        }
        assert_eq!(
            serde_json::to_value(before.inventory).unwrap(),
            serde_json::to_value(after.inventory).unwrap()
        );
    }
}
