use super::*;
use crate::state_context::StateContext;

#[cfg(feature = "test-support")]
pub fn copy_registration_and_undo_keep_campaign_choices_and_later_edits(
    temp: &tempfile::TempDir,
    campaigns: &mut Campaigns,
) {
    let original = campaigns.modules()[0].clone();
    let campaign = campaigns.active_campaign().unwrap();
    let campaign_before = CampaignDocument::load(&campaign).unwrap().text;
    let copied = temp.path().join("copied");
    fs::create_dir(&copied).unwrap();
    for entry in content::read_directory(&original.path).unwrap().0 {
        let path = copied.join(&entry.path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::copy(original.path.join(&entry.path), path).unwrap();
    }
    for used in [false, true] {
        let mut library = Library {
            copied_package: Some(local_module(&copied).unwrap()),
            copy_name: "Reaches".into(),
            copy_not_used: !used,
            ..Default::default()
        };
        library.register_copy(campaigns).unwrap();
        let owner = library.shown_registration(campaigns).unwrap();
        let id = owner.package;
        let document = CampaignDocument::load(&campaign).unwrap();
        assert_eq!(
            document
                .selections
                .iter()
                .any(|selection| selection.package.id == Some(id)),
            used
        );
        assert!(
            document
                .selections
                .iter()
                .any(|selection| selection.package.id == Some(original.id))
        );
        let path = copied.join("Module.md");
        let manifest = fs::read_to_string(&path).unwrap();
        fs::write(&path, format!("{manifest}\nLater edit.\n")).unwrap();
        assert!(library.undo_registration_of(&owner, campaigns).is_err());
        assert_eq!(
            CampaignDocument::load(&campaign).unwrap().text,
            document.text
        );
        fs::write(path, manifest).unwrap();
        library.undo_registration_of(&owner, campaigns).unwrap();
        assert_eq!(
            CampaignDocument::load(&campaign).unwrap().text,
            campaign_before
        );
        assert_eq!(campaigns.modules(), std::slice::from_ref(&original));
    }
}

#[cfg(feature = "test-support")]
pub fn iter14_e_changed_files_end_every_undo_result(
    temp: &tempfile::TempDir,
    campaigns: &mut Campaigns,
    mut route: impl FnMut(&mut Campaigns, Library, &StateContext) -> Library,
    mut frame: impl FnMut(&mut Library, &mut Campaigns),
) {
    use crate::content_workspace::ContentEdits;
    let original = campaigns.modules()[0].clone();
    let campaign = campaigns.active_campaign().unwrap();
    let root = original.path.clone();
    let paths: Vec<String> = content::read_directory(&root)
        .unwrap()
        .0
        .into_iter()
        .filter(|entry| entry.kind != "package")
        .map(|entry| entry.path)
        .take(2)
        .collect();
    assert_eq!(paths.len(), 2, "fixture: two package entries");
    let outside = b"---\nsplat_kind: [\n---\nAn outside edit.\n".to_vec();
    // Writes `after` over each file and returns the edits, as an action would.
    let edit = |tag: &str| {
        let files = paths
            .iter()
            .map(|path| {
                let before = fs::read_to_string(root.join(path)).unwrap();
                let after = format!("{before}\n{tag}\n");
                (root.clone(), path.clone(), before, after)
            })
            .collect();
        ContentEdits::write(files).unwrap()
    };
    // Changes one file outside Splat, reloads, and returns the raw bytes left on disk.
    let change_outside = |library: &mut Library| {
        let target = root.join(&paths[1]);
        let kept = fs::read(&target).unwrap();
        fs::write(&target, &outside).unwrap();
        let first = fs::read(root.join(&paths[0])).unwrap();
        library.forget_changed_undos();
        assert_eq!(
            fs::read(&target).unwrap(),
            outside,
            "the reload wrote nothing"
        );
        assert_eq!(fs::read(root.join(&paths[0])).unwrap(), first);
        fs::write(&target, kept).unwrap();
    };

    // `Undo separate entries`: the batch.
    let mut library = Library::default();
    library.kept.set_identities((
        edit("batch"),
        crate::sentence!("2 files are now separate entries."),
    ));
    library.forget_changed_undos();
    assert!(
        library.kept.identities().is_some() && library.kept.marks() == 1,
        "unchanged files keep Undo, and its mark"
    );
    change_outside(&mut library);
    // M-L0.4a: the mark (the footer order) ends with its data.
    assert!(library.kept.identities().is_none() && library.kept.marks() == 0);
    // A deleted file of the batch also ends the result.
    library.kept.set_identities((
        edit("deleted"),
        crate::sentence!("2 files are now separate entries."),
    ));
    library.forget_changed_undos();
    assert!(
        library.kept.identities().is_some(),
        "fixture: Undo before the delete"
    );
    let target = root.join(&paths[1]);
    let kept = fs::read(&target).unwrap();
    let first = fs::read(root.join(&paths[0])).unwrap();
    fs::remove_file(&target).unwrap();
    library.forget_changed_undos();
    assert!(library.kept.identities().is_none() && library.kept.marks() == 0);
    assert!(!target.exists(), "the reload wrote nothing");
    assert_eq!(fs::read(root.join(&paths[0])).unwrap(), first);
    fs::write(&target, kept).unwrap();

    // `Undo separate entry`: one file.
    let text = fs::read_to_string(root.join(&paths[1])).unwrap();
    library.kept.set_identity(IdentityUndo {
        root: root.clone(),
        path: paths[1].clone(),
        before: "before".into(),
        after: text,
    });
    library.forget_changed_undos();
    assert!(library.kept.identity().is_some() && library.kept.marks() == 1);
    change_outside(&mut library);
    assert!(library.kept.identity().is_none() && library.kept.marks() == 0);

    // `Undo copy`: the changed file is the copy.
    library.kept.push_copy(DependencyUndo {
        root: root.clone(),
        path: paths[0].clone(),
        before: "before".into(),
        after: fs::read_to_string(root.join(&paths[0])).unwrap(),
        copied_path: paths[1].clone(),
        copied_text: fs::read_to_string(root.join(&paths[1])).unwrap(),
        name: "Copied".into(),
        package: "Package".into(),
    });
    library.forget_changed_undos();
    assert!(!library.kept.copies().is_empty() && library.kept.marks() == 1);
    change_outside(&mut library);
    assert!(library.kept.copies().is_empty() && library.kept.marks() == 0);

    // `Undo link update`.
    library.link_edits.record(LinkResult {
        owner: Uuid::nil(),
        source: OfferSource {
            root: root.clone(),
            path: paths[0].clone(),
            text: String::new(),
        },
        edits: edit("links"),
        old: "Cinder Wolf".into(),
        new: "Ash Wolf".into(),
    });
    library.forget_changed_undos();
    assert!(!library.link_edits.is_empty());
    change_outside(&mut library);
    assert!(library.link_edits.is_empty());

    // `Undo package choice`: only the package choices count.
    let mut document = CampaignDocument::load(&campaign).unwrap();
    library.kept.set_package_use(SelectionUndo {
        campaign: campaign.clone(),
        before: Vec::new(),
        after: document.selections.clone(),
    });
    let campaign_file = campaign.path.join(&document.path);
    let edited = format!("{}\nA later note.\n", document.text);
    fs::write(&campaign_file, &edited).unwrap();
    library.forget_changed_undos();
    assert!(
        library.kept.package_use().is_some() && library.kept.marks() == 1,
        "other campaign edits keep Undo"
    );
    let saved = fs::read(&campaign_file).unwrap();
    document = CampaignDocument::load(&campaign).unwrap();
    document.selections.clear();
    document.save().unwrap();
    let cleared = fs::read(&campaign_file).unwrap();
    library.forget_changed_undos();
    assert!(library.kept.package_use().is_none() && library.kept.marks() == 0);
    assert_eq!(fs::read(&campaign_file).unwrap(), cleared);
    fs::write(&campaign_file, saved).unwrap();
    // M-L0.4b, proof 5 (note §4): a write that fails keeps `Undo package choice`, with the
    // error, so the DM can try again; with the file writable, the next Undo puts the choices
    // back. Choices changed since end it, with the words that say so.
    let choices = CampaignDocument::load(&campaign).unwrap().selections;
    assert!(!choices.is_empty(), "fixture: the campaign has choices");
    let with_choices = fs::read(&campaign_file).unwrap();
    let choice_undo = || SelectionUndo {
        campaign: campaign.clone(),
        before: Vec::new(),
        after: choices.clone(),
    };
    library.kept.set_package_use(choice_undo());
    let writable = fs::metadata(&campaign_file).unwrap().permissions();
    let mut read_only = writable.clone();
    read_only.set_readonly(true);
    fs::set_permissions(&campaign_file, read_only).unwrap();
    library.error = None;
    library.undo_package_use();
    assert!(
        library.error.is_some()
            && library.kept.package_use().is_some()
            && library.kept.marks() == 1,
        "a failed write keeps Undo package choice: {:?}",
        library.error
    );
    fs::set_permissions(&campaign_file, writable.clone()).unwrap();
    // Astra on 3a2f6d42: a file that cannot be READ keeps it too, and so does a reload
    // while it still cannot be read (`forget_changed_undos`): only a read that proves the
    // choices differ ends it. Once the file reads again, a reload keeps it (its choices are
    // the same) and its Undo works.
    let mut unreadable = writable.clone();
    std::os::unix::fs::PermissionsExt::set_mode(&mut unreadable, 0o000);
    fs::set_permissions(&campaign_file, unreadable).unwrap();
    // While it cannot be read, a new entry made in the package cannot be chosen for the
    // campaign: the app's own words first, checked, then the system's cause.
    let not_chosen = library
        .choose_new_entry(&Owner::Package(original.clone()), Uuid::new_v4(), campaigns)
        .unwrap_err();
    assert!(
        matches!(&not_chosen, PageError::Because { own, .. } if *own == ENTRY_NOT_CHOSEN),
        "the app's words are checked: {not_chosen:?}"
    );
    assert!(
        design::outcome::Fault::cause(&LibraryFault::from(&not_chosen))
            .as_str()
            .starts_with("Entry created. Splat could not use it in this campaign. "),
        "{not_chosen:?}"
    );
    library.error = None;
    library.undo_package_use();
    assert!(
        library.error.is_some() && library.kept.package_use().is_some(),
        "a failed read keeps Undo package choice: {:?}",
        library.error
    );
    library.forget_changed_undos();
    assert!(
        library.kept.package_use().is_some() && library.kept.marks() == 1,
        "a reload while the file cannot be read keeps Undo package choice"
    );
    fs::set_permissions(&campaign_file, writable).unwrap();
    library.forget_changed_undos();
    assert!(
        library.kept.package_use().is_some(),
        "a reload once it reads keeps it: the choices are the ones it wrote"
    );
    library.undo_package_use();
    assert!(
        library.kept.package_use().is_none()
            && library.kept.marks() == 0
            && CampaignDocument::load(&campaign)
                .unwrap()
                .selections
                .is_empty(),
        "the next Undo puts the choices back"
    );
    library.error = None;
    library.kept.set_package_use(choice_undo());
    library.undo_package_use();
    assert!(
        library.kept.package_use().is_none()
            && library.kept.marks() == 0
            && library.error == Some(PageError::Own(CHOICES_CHANGED.clone())),
        "choices changed since end it, in the app's own checked words: {:?}",
        library.error
    );
    library.error = None;
    fs::write(&campaign_file, with_choices).unwrap();

    // `Undo package registration`: a copied file changed. Each registration result lives in
    // its store entry, with all its Undo needs (review of PR 367, points 1 to 4).
    let copy_folder = |name: &str| {
        let copied = temp.path().join(name);
        fs::create_dir(&copied).unwrap();
        for entry in content::read_directory(&original.path).unwrap().0 {
            let path = copied.join(&entry.path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::copy(original.path.join(&entry.path), path).unwrap();
        }
        copied
    };
    let copied = copy_folder("copied");
    let scope = Some(campaign.id);
    let register = |library: &mut Library, campaigns: &mut Campaigns, folder: &Path, used: bool| {
        library.copied_package = Some(local_module(folder).unwrap());
        library.copy_name = "Reaches".into();
        library.copy_not_used = !used;
        library.register_copy(campaigns).unwrap();
        library.shown_registration(campaigns).unwrap()
    };
    // A reload: every write makes one, and so does `Reload from disk`.
    let reload = |library: &mut Library, campaigns: &Campaigns| {
        library.forget_changed_undos();
        library.reconcile_registrations(campaigns);
    };
    let words = |library: &Library, owner: &RegistrationOwner| {
        library
            .registrations
            .get(owner)
            .map(|shown| shown.value().words().as_str().to_owned())
    };
    let blocked = |library: &Library, owner: &RegistrationOwner| {
        library
            .registrations
            .get(owner)
            .is_some_and(|shown| shown.value().blocked)
    };
    let count = |library: &Library| library.registrations.newest_first(&scope).count();
    let copy_chosen = |package: Uuid| {
        CampaignDocument::load(&campaign)
            .unwrap()
            .selections
            .iter()
            .any(|selection| selection.package.id == Some(package))
    };
    let mut library = Library::default();
    let a = register(&mut library, campaigns, &copied, true);
    // Round 23 AD1: the result says what the registration chose, not the campaign's use now.
    assert_eq!(
        words(&library, &a).as_deref(),
        Some("Reaches is registered, with every entry chosen for this campaign."),
        "iter-23-L-AD1-chosen"
    );
    reload(&mut library, campaigns);
    assert!(!blocked(&library, &a));
    let manifest = copied.join("Module.md");
    let written = fs::read(&manifest).unwrap();
    fs::write(&manifest, &outside).unwrap();
    reload(&mut library, campaigns);
    assert!(blocked(&library, &a) && words(&library, &a).is_some());
    assert_eq!(fs::read(&manifest).unwrap(), outside);
    fs::write(&manifest, &written).unwrap();
    reload(&mut library, campaigns);
    assert!(!blocked(&library, &a));
    // Another edit of the campaign file keeps Undo: Undo takes out only the copy's choices,
    // so the edit stays (it used to block Undo, which restored the whole file).
    let campaign_file = campaign
        .path
        .join(&CampaignDocument::load(&campaign).unwrap().path);
    let mut edited = fs::read_to_string(&campaign_file).unwrap();
    edited.push_str("\nA later note.\n");
    fs::write(&campaign_file, &edited).unwrap();
    reload(&mut library, campaigns);
    assert!(!blocked(&library, &a));
    assert_eq!(
        words(&library, &a).as_deref(),
        Some("Reaches is registered, with every entry chosen for this campaign."),
        "an edit that leaves the choices of the copy keeps the clause"
    );

    // R11 (row 127, G147): result A (this registration) and an unrelated result B (a link
    // update of another entry). The copy's use changes: A's clause ends at the reload, its
    // Undo stays, and B stays.
    let b = || LinkResult {
        owner: Uuid::nil(),
        source: OfferSource {
            root: root.clone(),
            path: paths[0].clone(),
            text: String::new(),
        },
        edits: edit("links-b"),
        old: "Cinder Wolf".into(),
        new: "Ash Wolf".into(),
    };
    library.link_edits.record(b());
    library.press_line(LibraryPress::UseCopy, &a, campaigns);
    assert!(library.error.is_none(), "{:?}", library.error);
    reload(&mut library, campaigns);
    assert_eq!(
        words(&library, &a).as_deref(),
        Some("Reaches is registered."),
        "R11-clause-ends"
    );
    assert!(!blocked(&library, &a), "R11-undo-stays");
    assert!(!library.link_edits.is_empty(), "R11-b-stays");
    let source_files = library
        .rosters
        .get(&a)
        .unwrap()
        .value()
        .edits
        .inverse_files();

    // Point 1: A's source change, then copy C registered and its registration undone: A's
    // source change stays on disk and with A.
    let c_folder = copy_folder("copied-c");
    let c = register(&mut library, campaigns, &c_folder, true);
    assert_ne!(a, c);
    assert_eq!(count(&library), 2);
    library.undo_registration_of(&c, campaigns).unwrap();
    reload(&mut library, campaigns);
    assert!(
        library
            .rosters
            .get(&a)
            .is_some_and(|source| source.value().edits.on_disk()),
        "point-1-undo-c-keeps-a-source-change"
    );
    for (root, path, written, _) in &source_files {
        assert!(
            crate::content_workspace::holds(root, path, written),
            "point-1: {path} keeps the source change of A"
        );
    }
    assert_eq!(library.shown_registration(campaigns), Some(a));

    // Undo of the use change does not bring the clause back: it ended.
    library.press_line(LibraryPress::UndoRosters, &a, campaigns);
    reload(&mut library, campaigns);
    assert!(library.rosters.get(&a).is_none());
    assert_eq!(
        words(&library, &a).as_deref(),
        Some("Reaches is registered.")
    );
    // Undo of A: its words go with its data (the copy unregistered, its choices out of the
    // campaign), B stays, and the campaign file keeps its other edit.
    let modules = campaigns.modules().len();
    library.press_line(LibraryPress::UndoRegistration, &a, campaigns);
    assert!(library.error.is_none(), "{:?}", library.error);
    assert_eq!(words(&library, &a), None, "R11-undo-a-words");
    assert_eq!(campaigns.modules().len(), modules - 1, "R11-undo-a-data");
    assert!(!copy_chosen(a.package), "R11-undo-a-choice");
    assert!(
        fs::read_to_string(&campaign_file)
            .unwrap()
            .contains("A later note."),
        "Undo keeps the other edit of the campaign file"
    );
    assert!(!library.link_edits.is_empty(), "R11-undo-a-b-stays");

    // Point 2: an unchosen copy's clause ends at the reload after each use change: through
    // Lookup's or Library's choice of one entry, the Campaign page's choice, an outside edit.
    let entry_of = |package: &Module| {
        content::read_directory(&package.path)
            .unwrap()
            .0
            .into_iter()
            .find(|entry| entry.kind != "package")
            .unwrap()
            .id
    };
    // Point 3 (second pass): the encounter search's event, while an unrelated Library draft
    // is unsaved, so no reload can run: the clause ends in the same pass as the write. Point 2 (second pass): an outside edit that names the copy by
    // its key.
    let original_path = &paths[0];
    const SRD: Uuid = uuid::uuid!("71871979-b5d2-59a8-81c8-2081b92bb2fe");
    const SRD_TASK: &str =
        "- [x] System Reference Document 5.2.1 ^71871979-b5d2-59a8-81c8-2081b92bb2fe\n";
    const SRD_ONLY: &str = "  - Only: [[spell/aid]]\n";
    // Another package's choice and indented prose, written under the copy's task.
    let b_id = Uuid::new_v4();
    let nested = format!("  - [x] Package B ^{b_id}\n  Indented prose stays.\n");
    // An unclosed fence inside the copy's list item: B's task ends the item, so the reader
    // reads B. Without the copy's line, the fence would open at the top level and hold B.
    let fenced_item =
        |name: &str, id: Uuid| format!("\n- [x] {name} ^{id}\n  ```\n- [x] Package B ^{b_id}\n");
    let errands = |name: &str| {
        format!(
            "\n## Errands\n\n- [ ] {name}\n  Buy rope.\n\n```markdown\n## Packages\n- [ ] {name}\n  Buy rope.\n```\n"
        )
    };
    for path in [
        "encounter search with a draft",
        "campaign page",
        "outside edit",
        "outside name link",
        "outside id, trailing spaces",
        "outside nested choice",
        "outside nested four spaces",
        "outside fence in the copy's item",
        "outside other package's hand-written Only",
        "outside indented task",
    ] {
        // The reader does not read a task line whose id has spaces after it (a fault on
        // that line), but Undo still takes that line out: it names the copy's identity.
        let read_as_choice = path != "outside id, trailing spaces";
        let d = register(&mut library, campaigns, &copied, false);
        assert_eq!(
            words(&library, &d).as_deref(),
            Some("Reaches is registered, with no entries chosen for this campaign."),
            "{path}"
        );
        let module = campaigns
            .modules()
            .iter()
            .find(|module| module.id == d.package)
            .unwrap()
            .clone();
        let choices_before = CampaignDocument::load(&campaign).unwrap().selections;
        let choose_by_hand = |reference: crate::content_workspace::EntryRef| {
            let document = CampaignDocument::load(&campaign).unwrap();
            let mut selections = document.selections.clone();
            selections.push(crate::content_workspace::PackageSelection {
                name: Some(module.name.clone()),
                package: reference,
                selection: crate::content_workspace::Selection::All,
            });
            let text = document.text_with(&selections).unwrap();
            fs::write(&campaign_file, text).unwrap();
        };
        match path {
            "encounter search with a draft" => {
                let loaded = load_all(campaigns.modules(), Some(&campaign)).unwrap();
                let unrelated = loaded
                    .notes
                    .iter()
                    .position(|note| {
                        note.owner.id() == Some(original.id) && note.path == *original_path
                    })
                    .unwrap();
                let _ = library.draft.replace(format!(
                    "{}\nUnsaved words.\n",
                    loaded.notes[unrelated].text.as_str()
                ));
                library.loaded = Some(loaded);
                library.selected = Some(unrelated);
                assert!(library.dirty(), "fixture: an unrelated draft is unsaved");
                // The encounter search's `Use it` (src/encounters.rs, `use_package_entry`):
                // the one producer of this event that no Library draft locks (Lookup's
                // `Open in Library` and Library's `Choose for this campaign` wait for the
                // draft). Its act is this insert; `route` is the app's own consumer.
                let ctx = StateContext::new();
                ctx.set(
                    crate::requests::USE_PACKAGE_ENTRY,
                    (module.id, entry_of(&module)),
                );
                library = route(campaigns, library, &ctx);
                assert!(
                    crate::content_workspace::CampaignDocument::load(&campaign)
                        .unwrap()
                        .selections
                        .iter()
                        .any(|selection| selection.package.id == Some(module.id)),
                    "fixture: the event wrote the choice"
                );
                assert!(
                    library.dirty() && library.pending.is_none(),
                    "no reload ran"
                );
            }
            "campaign page" => {
                let mut document = CampaignDocument::load(&campaign).unwrap();
                document
                    .selections
                    .push(crate::content_workspace::PackageSelection {
                        name: Some(module.name.clone()),
                        package: crate::content_workspace::EntryRef::id(module.id),
                        selection: crate::content_workspace::Selection::All,
                    });
                document.save().unwrap();
            }
            "outside edit" => {
                choose_by_hand(crate::content_workspace::EntryRef::id(module.id));
            }
            "outside nested choice"
            | "outside nested four spaces"
            | "outside fence in the copy's item"
            | "outside indented task" => {
                let task = if path == "outside nested choice" {
                    format!("- [x] {} ^{}\n{nested}", module.name, module.id)
                } else if path == "outside fence in the copy's item" {
                    fenced_item(&module.name, module.id)
                } else if path == "outside nested four spaces" {
                    format!(
                        "\n- [x] {} ^{}\n    - [x] Package B ^{b_id}\n",
                        module.name, module.id
                    )
                } else {
                    format!("  - [x] {} ^{}\n", module.name, module.id)
                };
                let text = fs::read_to_string(&campaign_file).unwrap();
                fs::write(
                    &campaign_file,
                    text.replacen("## Packages\n", &format!("## Packages\n{task}"), 1),
                )
                .unwrap();
                let chosen = CampaignDocument::load(&campaign).unwrap().selections;
                assert!(
                    chosen.iter().any(|s| s.package.id == Some(module.id))
                        && (path == "outside indented task"
                            || chosen.iter().any(|s| s.package.id == Some(b_id))),
                    "fixture: the reader reads the choices ({path})"
                );
            }
            "outside other package's hand-written Only" => {
                let text = fs::read_to_string(&campaign_file).unwrap();
                assert!(text.contains(SRD_TASK), "fixture: the SRD's task\n{text}");
                let text = text.replacen(SRD_TASK, &format!("{SRD_TASK}{SRD_ONLY}"), 1);
                let text = text.replacen(
                    "## Packages\n",
                    &format!("## Packages\n- [x] {} ^{}\n", module.name, module.id),
                    1,
                );
                fs::write(&campaign_file, text).unwrap();
            }
            "outside id, trailing spaces" => {
                choose_by_hand(crate::content_workspace::EntryRef::id(module.id));
                let mark = format!(" ^{}\n", module.id);
                let text = fs::read_to_string(&campaign_file).unwrap();
                assert!(text.contains(&mark), "fixture: the id-bearing line\n{text}");
                fs::write(
                    &campaign_file,
                    text.replacen(&mark, &format!(" ^{}  \n", module.id), 1),
                )
                .unwrap();
            }
            _ => {
                // A campaign file's package list names a package by its id or, written by
                // hand, by its name alone (a link). The key form is not in the file's list;
                // the one reading of a choice resolves it too.
                let manifests = package_manifests(campaigns);
                let key = manifests
                    .iter()
                    .find(|manifest| manifest.id == module.id)
                    .and_then(|manifest| manifest.key.clone())
                    .expect("fixture: the manifest of the copy has a key");
                assert_eq!(
                    chosen_package(
                        &crate::content_workspace::EntryRef {
                            id: None,
                            key: Some(key),
                            link: None,
                        },
                        &manifests,
                    ),
                    Some(module.id),
                    "point-2-key-form"
                );
                let text = fs::read_to_string(&campaign_file).unwrap();
                let text = text.replacen(
                    "## Packages\n",
                    &format!("## Packages\n- [x] {}\n", module.name),
                    1,
                );
                // An unrelated checklist item with the same name, under another heading, and
                // a fenced example that holds a `## Packages` list of its own.
                let text = format!("{text}{}", errands(&module.name));
                fs::write(&campaign_file, text).unwrap();
                assert!(
                    CampaignDocument::load(&campaign)
                        .unwrap()
                        .selections
                        .iter()
                        .any(|selection| selection.package.id.is_none()
                            && selection.package.link.as_deref() == Some(module.name.as_str())),
                    "fixture: the file names the copy by its name\n{}\n{:?}",
                    fs::read_to_string(&campaign_file).unwrap(),
                    CampaignDocument::load(&campaign).unwrap().selections
                );
            }
        }
        if path != "encounter search with a draft" {
            reload(&mut library, campaigns);
        }
        if path == "outside other package's hand-written Only" {
            // Undo takes out the copy's choice and touches no other line: the SRD's
            // hand-written `Only:` line stays byte for byte, its choice stays, and the guard
            // has nothing to refuse.
            library.press_line(LibraryPress::UndoRegistration, &d, campaigns);
            assert_eq!(library.error, None, "srd-only-undo-succeeds");
            let text = fs::read_to_string(&campaign_file).unwrap();
            assert!(
                text.contains(&format!("{SRD_TASK}{SRD_ONLY}"))
                    && !text.contains(&d.package.to_string()),
                "srd-only-line-byte-equal\n{text}"
            );
            let srd = CampaignDocument::load(&campaign)
                .unwrap()
                .selections
                .into_iter()
                .find(|s| s.package.id == Some(SRD))
                .map(|s| s.selection);
            assert!(
                matches!(
                    &srd,
                    Some(crate::content_workspace::Selection::Selected { entries })
                        if entries.iter().any(|e| e.link.as_deref() == Some("[[spell/aid]]"))
                ),
                "srd-aid-choice-stays: {srd:?}"
            );
            fs::write(&campaign_file, text.replacen(SRD_ONLY, "", 1)).unwrap();
            reload(&mut library, campaigns);
            continue;
        }
        if path == "outside fence in the copy's item" {
            // Taking the copy's line out would put B (and every later line) inside a code
            // block, so B's choice would be lost: the read-back guard refuses the Undo, and
            // nothing is written. The registration and its result stay; the reason shows.
            let before_undo = fs::read(&campaign_file).unwrap();
            library.press_line(LibraryPress::UndoRegistration, &d, campaigns);
            assert_eq!(
                library.error.as_ref().map(PageError::words),
                Some(PACKAGE_LIST_UNSAFE),
                "guard-refused\nbefore:\n{}\nafter:\n{}\nselections {:?}",
                String::from_utf8_lossy(&before_undo),
                fs::read_to_string(&campaign_file).unwrap(),
                CampaignDocument::load(&campaign).map(|d| d.selections)
            );
            assert_eq!(
                fs::read(&campaign_file).unwrap(),
                before_undo,
                "guard-nothing-written"
            );
            assert!(
                CampaignDocument::load(&campaign)
                    .unwrap()
                    .selections
                    .iter()
                    .any(|s| s.package.id == Some(b_id)),
                "guard-b-stays"
            );
            assert!(words(&library, &d).is_some(), "guard-result-stays");
            assert!(
                campaigns.modules().iter().any(|m| m.id == d.package),
                "guard-registration-stays"
            );
            // The DM takes the copy out by hand; then Undo works.
            library.error = None;
            let text = String::from_utf8(before_undo).unwrap();
            fs::write(
                &campaign_file,
                text.replacen(&fenced_item(&module.name, module.id), "", 1),
            )
            .unwrap();
            reload(&mut library, campaigns);
            library.undo_registration_of(&d, campaigns).unwrap();
            reload(&mut library, campaigns);
            continue;
        }
        if read_as_choice {
            assert_eq!(
                words(&library, &d).as_deref(),
                Some("Reaches is registered."),
                "point-2-{path}"
            );
            // Undo of the registration takes the later choice out with it, in whatever
            // form it names the copy; the other packages' choices stay, and the choice's
            // own result ends with its data.
            library.kept.set_package_use(SelectionUndo {
                campaign: campaign.clone(),
                before: choices_before.clone(),
                after: CampaignDocument::load(&campaign).unwrap().selections,
            });
        }
        library.undo_registration_of(&d, campaigns).unwrap();
        let after: Vec<_> = CampaignDocument::load(&campaign)
            .unwrap()
            .selections
            .into_iter()
            .filter(|s| s.package.id != Some(b_id))
            .collect();
        assert_eq!(after, choices_before, "point-3-{path}");
        if path == "outside nested four spaces" {
            // With this reader B stays a choice after the copy's line goes (the guard
            // checks it): B's line stays byte for byte.
            let text = fs::read_to_string(&campaign_file).unwrap();
            let b_line = format!("    - [x] Package B ^{b_id}\n");
            assert!(
                text.contains(&b_line)
                    && CampaignDocument::load(&campaign)
                        .unwrap()
                        .selections
                        .iter()
                        .any(|s| s.package.id == Some(b_id)),
                "point-3-four-spaces-b-stays\n{text}"
            );
            fs::write(&campaign_file, text.replacen(&b_line, "", 1)).unwrap();
        }
        if path == "outside nested choice" {
            // B's choice and the indented prose stay, byte for byte; only the copy's task
            // line went.
            let text = fs::read_to_string(&campaign_file).unwrap();
            assert!(
                text.contains(&format!("## Packages\n{nested}"))
                    && CampaignDocument::load(&campaign)
                        .unwrap()
                        .selections
                        .iter()
                        .any(|s| s.package.id == Some(b_id)),
                "point-3-nested-b-stays\n{text}"
            );
            fs::write(&campaign_file, text.replacen(&nested, "", 1)).unwrap();
        }
        if path == "outside name link" {
            let text = fs::read_to_string(&campaign_file).unwrap();
            // The other heading's item and the fenced example stay, byte for byte.
            assert!(
                text.ends_with(&errands(&module.name)),
                "point-2-unrelated-item-stays\n{text}"
            );
            let rest = text.split("\n## Errands").next().unwrap().to_owned() + "\n";
            fs::write(&campaign_file, rest).unwrap();
        }
        let text = fs::read_to_string(&campaign_file).unwrap();
        assert!(
            !text.contains(&d.package.to_string())
                && !text.contains(&format!("] {}\n", module.name)),
            "point-3-text-{path}: the file still names the copy\n{text}"
        );
        assert!(
            library.kept.package_use().is_none() && library.kept.marks() == 0,
            "point-3-result-{path}"
        );
        library.loaded = None;
        library.selected = None;
        library.draft.clear();
        reload(&mut library, campaigns);
    }

    // Point 1 (second pass): another campaign chose the copy, so the registration's Undo
    // waits with that campaign's name and is refused; when it stops using the copy, Undo
    // works again.
    let other = Campaign {
        id: Uuid::new_v4(),
        name: "Other".into(),
        path: temp.path().join("other"),
        dndbeyond_campaign_id: None,
        dndbeyond_campaign_name: None,
    };
    fs::create_dir(&other.path).unwrap();
    CampaignDocument::new(&other).unwrap().save().unwrap();
    campaigns.open_for_test(&other);
    campaigns.open_for_test(&campaign);
    let h = register(&mut library, campaigns, &copied, false);
    let h_module = campaigns
        .modules()
        .iter()
        .find(|module| module.id == h.package)
        .unwrap()
        .clone();
    crate::content_workspace::use_in_campaign(&other, h_module.id, &h_module.name, None).unwrap();
    reload(&mut library, campaigns);
    let lock = library
        .registrations
        .get(&h)
        .and_then(|shown| shown.value().lock());
    assert!(
        lock.as_ref()
            .is_some_and(|lock| lock.as_str().starts_with("Other uses this package.")),
        "point-1-lock: {lock:?}"
    );
    let refused = library.undo_registration_of(&h, campaigns);
    assert!(
        refused
            .as_ref()
            .is_err_and(|reason| reason.contains("Other")),
        "point-1-refused: {refused:?}"
    );
    assert!(
        words(&library, &h).is_some(),
        "the refused Undo keeps its result"
    );
    let mut document = CampaignDocument::load(&other).unwrap();
    document.selections.clear();
    document.save().unwrap();
    reload(&mut library, campaigns);
    assert_eq!(
        library
            .registrations
            .get(&h)
            .and_then(|shown| shown.value().lock()),
        None,
        "point-1-free"
    );
    library.undo_registration_of(&h, campaigns).unwrap();
    reload(&mut library, campaigns);

    // Point 4: the result is the campaign's. In another campaign it does not show; back in
    // its campaign it does. An unregister ends it at the reload, with its source change.
    let e = register(&mut library, campaigns, &copied, false);
    campaigns.open_for_test(&other);
    assert_eq!(
        library.shown_registration(campaigns),
        None,
        "point-4-switch"
    );
    campaigns.open_for_test(&campaign);
    assert_eq!(library.shown_registration(campaigns), Some(e));
    library.press_line(LibraryPress::UseCopy, &e, campaigns);
    assert!(library.rosters.get(&e).is_some());
    campaigns.unregister_module(e.package).unwrap();
    reload(&mut library, campaigns);
    assert_eq!(words(&library, &e), None, "point-4-unregister");
    assert!(
        library.rosters.get(&e).is_none(),
        "point-4-unregister-source"
    );

    // One result per copied folder in a campaign: registered again (an unregister that no
    // reload has seen), the older result ends by the store's owner event; Undo reverses
    // only the newest registration's own files.
    let f = register(&mut library, campaigns, &copied, false);
    let after_first = fs::read(&manifest).unwrap();
    campaigns.unregister_module(f.package).unwrap();
    let g = register(&mut library, campaigns, &copied, false);
    assert_eq!(count(&library), 1, "one-result-per-folder");
    assert_eq!(words(&library, &f), None);
    library.undo_registration_of(&g, campaigns).unwrap();
    assert_eq!(
        fs::read(&manifest).unwrap(),
        after_first,
        "undo-newest-only"
    );
    assert_eq!(count(&library), 0);

    // The check runs at events, never per frame: with a registration and its source change
    // kept, idle frames of the page after the load leave its count as it was.
    let i = register(&mut library, campaigns, &copied, false);
    library.press_line(LibraryPress::UseCopy, &i, campaigns);
    assert!(
        library.rosters.get(&i).is_some(),
        "fixture: the work's state is there"
    );
    for _ in 0..400 {
        frame(&mut library, campaigns);
        if library.loaded.is_some() && library.pending.is_none() && !library.stale {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(
        library.loaded.is_some() && library.pending.is_none(),
        "the load settled"
    );
    frame(&mut library, campaigns);
    let settled = library.reconciles;
    for _ in 0..20 {
        frame(&mut library, campaigns);
    }
    assert_eq!(
        library.reconciles, settled,
        "idle-frames-do-no-registration-work"
    );
}

#[cfg(feature = "test-support")]
impl Library {
    /// The registration's Undo, pressed on its line: by its entry's ticket.
    pub(super) fn undo_registration_of(
        &mut self,
        owner: &RegistrationOwner,
        campaigns: &mut Campaigns,
    ) -> Result<(), String> {
        let ticket = self
            .registrations
            .get(owner)
            .ok_or("No package registration to undo.")?
            .ticket_for_caller_test();
        self.undo_registration_by(ticket, campaigns)
    }
    /// A press on a registration's line, or on its source change's (`UndoRosters`), as the
    /// typed footer gives it: by its entry's ticket. Nothing when the entry is gone.
    pub(super) fn press_line(
        &mut self,
        id: LibraryPress,
        owner: &RegistrationOwner,
        campaigns: &mut Campaigns,
    ) {
        let ticket = if id == LibraryPress::UndoRosters {
            self.rosters
                .get(owner)
                .map(|shown| shown.ticket_for_caller_test())
        } else {
            self.registrations
                .get(owner)
                .map(|shown| shown.ticket_for_caller_test())
        };
        if let Some(ticket) = ticket {
            self.apply_footer_press(
                Press {
                    id,
                    ticket: Some(ticket),
                },
                campaigns,
            );
        }
    }
}
