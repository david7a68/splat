use super::*;

#[cfg(feature = "test-support")]
pub fn package_copy_keeps_original_entries_encounters_and_byte_exact_undo(
    temp: &tempfile::TempDir,
    original: &Module,
    campaign: Campaign,
) {
    let copied = temp.path().join("copy");
    fs::create_dir(&copied).unwrap();
    let (entries, errors) = content::read_directory(&original.path).unwrap();
    assert!(errors.is_empty());
    let before: Vec<_> = entries
        .iter()
        .map(|e| {
            let text = fs::read_to_string(original.path.join(&e.path)).unwrap();
            let to = copied.join(&e.path);
            fs::create_dir_all(to.parent().unwrap()).unwrap();
            fs::write(to, &text).unwrap();
            (e.path.clone(), text)
        })
        .collect();
    let copy = PackageCopy::write(&copied, "New Marches").unwrap();
    let modules = [original.clone(), copy.module.clone()];
    let catalogue = Catalogue::load(&modules, Some(&campaign)).unwrap();
    assert!(catalogue.errors.is_empty(), "{:?}", catalogue.errors);
    for entry in &entries {
        assert_eq!(catalogue.entries[&entry.id].owner, original.id);
        assert_eq!(
            fs::read_to_string(original.path.join(&entry.path)).unwrap(),
            before.iter().find(|(p, _)| p == &entry.path).unwrap().1
        );
    }
    let (copies, _) = content::read_directory(&copied).unwrap();
    assert_eq!(copies.len(), entries.len());
    for entry in &copies {
        assert!(
            !entries
                .iter()
                .any(|old| old.id == entry.id || old.key == entry.key)
        );
        assert_eq!(catalogue.entries[&entry.id].owner, copy.module.id);
        for reference in content::required_references(entry).unwrap() {
            assert!(catalogue.resolve(&reference).is_ok());
        }
    }
    for name in ["Bridge Ambush", "Shrine of Cinders"] {
        let encounter = catalogue
            .entries
            .values()
            .find(|e| e.entry.name == name)
            .unwrap();
        let refs = content::required_references(&encounter.entry).unwrap();
        assert!(!refs.is_empty());
        for reference in refs {
            assert!(catalogue.resolve(&reference).is_ok());
        }
    }
    let (rosters, _) = content::read_directory(&campaign.path).unwrap();
    let rosters: Vec<_> = rosters
        .into_iter()
        .filter(|entry| matches!(entry.kind.as_str(), "encounter" | "encounter_group"))
        .collect();
    let originals: Vec<_> = rosters
        .iter()
        .map(|entry| fs::read_to_string(campaign.path.join(&entry.path)).unwrap())
        .collect();
    let edits = copy.use_in_rosters(&campaign).unwrap();
    assert!(!edits.files.is_empty());
    for (entry, before) in rosters.iter().zip(&originals) {
        let mut expected = before.clone();
        for old in &entries {
            let replacement = copies.iter().find(|new| new.path == old.path).unwrap();
            expected = redirect_reference(&expected, old, replacement.id).unwrap();
        }
        let after = fs::read_to_string(campaign.path.join(&entry.path)).unwrap();
        assert_eq!(
            after, expected,
            "Only references change; counts, names, changes and prose stay: {}",
            entry.path
        );
    }
    edits.undo().unwrap();
    for (entry, before) in rosters.iter().zip(&originals) {
        assert_eq!(
            fs::read_to_string(campaign.path.join(&entry.path)).unwrap(),
            *before
        );
    }
    copy.undo().unwrap();
    for (path, text) in &before {
        assert_eq!(fs::read(copied.join(path)).unwrap(), text.as_bytes());
    }
    let copy = PackageCopy::write(&copied, "New Marches").unwrap();
    fs::write(copied.join("Module.md"), "A later edit").unwrap();
    assert!(copy.undo().is_err());
    for (path, _, after) in &copy.files {
        if path != "Module.md" {
            assert_eq!(fs::read_to_string(copied.join(path)).unwrap(), *after);
        }
    }
}
