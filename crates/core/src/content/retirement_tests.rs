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
    let (entries, errors) = super::read_directory(root.path()).unwrap();
    assert!(entries.is_empty());
    assert_eq!(errors.len(), kinds.len());
    assert!(
        errors
            .iter()
            .all(|e| e.path.ends_with(".md") && e.message == legacy::OLD_LAYOUT)
    );
}

#[test]
fn every_builtin_note_is_schema_two_without_flags() {
    let root = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).join("builtin");
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

/// Each fixture is a note model (`<kind>.fields.yaml` and `<kind>.body.md`) and the note
/// the writer must give for it (`<kind>.md`).
#[test]
fn writer_keeps_models_and_exact_output_for_every_note_kind() {
    let root = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
        .join("tests/fixtures/writer");
    let mut kinds = 0;
    for file in std::fs::read_dir(&root).unwrap() {
        let path = file.unwrap().path();
        if !path.to_string_lossy().ends_with(".fields.yaml") {
            continue;
        }
        kinds += 1;
        let read = |extension: &str| {
            std::fs::read_to_string(path.with_extension("").with_extension(extension)).unwrap()
        };
        let model: super::Mapping =
            serde_yaml_ng::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let (body, expected) = (&read("body.md"), read("md"));
        let new = super::formats::write("Fixture.md", &model, body).unwrap();
        assert_eq!(new, expected, "{}", path.display());
        let before = super::model::entry("Fixture.md", body, &model).unwrap();
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
    assert_eq!(
        kinds, 17,
        "a fixture for each note kind, and one for a character"
    );
}
