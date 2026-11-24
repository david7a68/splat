use super::*;

#[test]
fn folder_recovery_requires_exactly_one_matching_identity() {
    let temp = tempfile::tempdir().unwrap();
    let old = temp.path().join("original");
    fs::create_dir(&old).unwrap();
    let text = crate::library::note_text("Test", "package", Some("test"));
    fs::write(old.join("Package.md"), &text).unwrap();
    let module = crate::library::local_module(&old).unwrap();
    assert!(relocated_package(&module, &[]).is_none());
    let moved = temp.path().join("moved");
    fs::rename(&old, &moved).unwrap();
    assert_eq!(relocated_package(&module, &[]).unwrap().path, moved);
    let copy = temp.path().join("copy");
    fs::create_dir(&copy).unwrap();
    fs::write(copy.join("Package.md"), text).unwrap();
    assert!(relocated_package(&module, &[]).is_none());
    crate::hermetic::closed::next_folder(moved.clone());
    assert_eq!(
        crate::hermetic::pick_folder_from(Some(temp.path())),
        Some(moved)
    );
    assert!(crate::hermetic::pick_folder().is_none());
}

fn campaign(root: &std::path::Path) -> Campaign {
    let campaign = Campaign {
        id: uuid::Uuid::new_v4(),
        name: "Test campaign".into(),
        path: root.into(),
        dndbeyond_campaign_id: None,
        dndbeyond_campaign_name: None,
    };
    if campaign.path.is_dir() && !campaign.path.join("Campaign.md").exists() {
        crate::content_workspace::CampaignDocument::new(&campaign)
            .unwrap()
            .save()
            .unwrap();
    }
    campaign
}

/// What `pick` reads from each kept result of `campaign`, newest first.
fn kinds_and<T>(
    content: &CampaignContent,
    campaign: Uuid,
    pick: impl Fn(&Payload) -> Option<T>,
) -> Vec<T> {
    content
        .results
        .newest_first(&campaign)
        .filter_map(|shown| pick(&shown.value().payload))
        .collect()
}

#[cfg(test)]
fn has(content: &CampaignContent, campaign: Uuid, kind: PageResult) -> bool {
    !kinds_and(content, campaign, |payload| {
        (payload.kind() == kind).then_some(())
    })
    .is_empty()
}

#[cfg(test)]
fn repair_discards(content: &CampaignContent, campaign: Uuid) -> Vec<String> {
    kinds_and(content, campaign, |payload| match payload {
        Payload::RepairDiscard { path, .. } => Some(path.clone()),
        _ => None,
    })
}

fn finished(content: &mut CampaignContent, checked: Checked) {
    let active = content.reconcile.campaign;
    finished_in(content, checked, active);
}

/// The check completes while `active` is the open campaign.
fn finished_in(content: &mut CampaignContent, checked: Checked, active: Option<Uuid>) {
    let (tx, rx) = mpsc::channel();
    content.reconcile.pending = Some(rx);
    tx.send(checked).unwrap();
    content.finish_check(active);
}

#[test]
fn iter8_e_campaign_deleted_manifest_action_does_not_guess_a_filename() {
    for filename in ["Package.md", "Module.md"] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("campaign");
        fs::create_dir(&root).unwrap();
        let active = campaign(&root);
        let package = temp.path().join("package");
        fs::create_dir(&package).unwrap();
        fs::write(
            package.join("Package.md"),
            crate::library::note_text("Test", "package", Some("test")),
        )
        .unwrap();
        let module = crate::library::local_module(&package).unwrap();
        if filename == "Module.md" {
            fs::rename(module.path.join("Package.md"), module.path.join(filename)).unwrap();
        }
        let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
        campaigns.register_module(module.clone()).unwrap();
        fs::remove_file(module.path.join(filename)).unwrap();
        let mut content = CampaignContent {
            snapshot: Some(load_snapshot(&active, &[module]).1.unwrap()),
            ..Default::default()
        };
        let ctx = egui::Context::default();
        let mut rendered = String::new();
        for _ in 0..2 {
            let output = ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1200.0, 2400.0),
                    )),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        // Round 26 FD1: the page's footer draws the warnings.
                        let frame = design::outcome::PaneFrame::new(ui);
                        let mut messages =
                            design::outcome::Messages::new(&frame, "warnings", 800.0);
                        let unavailable = content.package_warnings_of(&campaigns).unavailable;
                        content.package_messages(&mut messages, &campaigns, &unavailable);
                        // The footer, then the package list beside it (a Content pane
                        // refuses the list's own scroll column).
                        let _ = messages.pane(ui, design::Body::Content, |_| ());
                        content.packages_content(ui, &campaigns, &active);
                    });
                },
            );
            rendered = output
                .shapes
                .iter()
                .filter_map(|shape| match &shape.shape {
                    egui::Shape::Text(text) => Some(text.galley.job.text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("\n");
        }
        assert!(rendered.contains("Write the manifest again"), "{rendered}");
        assert!(
            rendered.contains("The package manifest was deleted."),
            "{rendered}"
        );
        assert!(!rendered.contains("Missing YAML frontmatter"), "{rendered}");
        for guess in [
            "Package.md was deleted",
            "Module.md was deleted",
            "Write Package.md again",
            "Write Module.md again",
        ] {
            assert!(!rendered.contains(guess), "{rendered}");
        }
    }
}

/// Wave 2 d (C) review: outside words that start with a path or a file name keep its
/// spelling (`Sentence::outside` makes a first letter a capital): checked words come first.
#[test]
fn outside_words_keep_a_path_s_spelling() {
    use design::outcome::Fault;
    let words = |fault: ContentFault<'_>| fault.cause().as_str().to_owned();
    // `ManifestWrite::undo`'s refusal, as a failed Undo puts it in the page's errors.
    let undo = undo_failed("package.md changed on disk. Reload before undoing.");
    assert_eq!(
        words(ContentFault::Error(&undo)),
        "Undo did not work: package.md changed on disk. Reload before undoing."
    );
    // A content layer's flag or a load error that starts with the note's path.
    assert_eq!(
        words(ContentFault::Flag("notes/a.md: Missing YAML frontmatter")),
        "Problem: notes/a.md: Missing YAML frontmatter."
    );
    assert_eq!(
        words(ContentFault::Error("No such file (os error 2)")),
        "No such file (os error 2)."
    );
}

#[test]
fn package_counts_share_one_form() {
    // Round 24 FF2: `<All, None or N of M> chosen`, then the entries.
    assert_eq!(
        package_count(2065, 2065, false),
        "All chosen · 2,065 entries"
    );
    assert_eq!(package_count(0, 6, false), "None chosen · 6 entries");
    // Round 27 FA6: a needed entry is ticked, so it counts as chosen. Four chosen and one
    // needed of six read `5 of 6 chosen`; five chosen and the one needed read `All chosen`.
    assert_eq!(package_count(4 + 1, 6, true), "5 of 6 chosen");
    assert_eq!(package_count(5 + 1, 6, false), "All chosen · 6 entries");
    // An off package whose three entries others need: three of its boxes are ticked.
    assert_eq!(package_count(3, 6, true), "3 of 6 chosen");
    // Every entry needed, none chosen: the box is not fully ticked, so not `All chosen`.
    assert_eq!(package_count(6, 6, true), "6 of 6 chosen");
    assert_eq!(package_count(0, 0, false), "None chosen");
}

#[test]
fn an_unclosed_list_names_and_marks_its_opening_line() {
    let text = "---\nsplat_schema: 2\nsplat_kind: [\n---\n";
    let (problem, line) = repair_problem("Notes/Broken.md", text);
    assert_eq!(line, Some(3));
    assert_eq!(
        problem.as_deref(),
        Some("Line 3 opens a list with [ that never closes.")
    );
    // Round 23 FA3: the save row asks Save's own question, the one World's repair asks. A
    // draft Save refuses names its cause; one Save writes has none.
    let active = campaign(std::path::Path::new("/nonexistent/splat-campaign"));
    let refusal = |draft: &str| Repair::new("Notes/Broken.md".into(), draft.into(), &active);
    assert_eq!(
        refusal(text).refusal.as_deref(),
        Some("Line 3 opens a list with [ that never closes.")
    );
    assert_eq!(refusal(text).error_line, Some(3));
    let rule = crate::library::note_text("Broken", "rule", None);
    assert_eq!(refusal(&rule).refusal, None);
    // A draft that parses but takes an identity settings own is still refused.
    let ownership = Some("Keep campaign ownership consistent with settings.".to_owned());
    let package = crate::library::note_text("Broken", "package", Some("broken"));
    assert_eq!(refusal(&package).refusal, ownership);
    let other = crate::library::note_text("Broken", "campaign", Some("broken"));
    assert_eq!(refusal(&other).refusal, ownership);
    let own = other.replace(
        &splat::content::Entry::parse("Campaign.md", &other)
            .unwrap()
            .id
            .to_string(),
        &active.id.to_string(),
    );
    assert_eq!(refusal(&own).refusal, None);
    // Round 23 review: the check runs when the draft changes, and the change ends this
    // repair's failed Save.
    let mut repair = refusal(&rule);
    repair.failure = Some("This file is read only.".into());
    let mut checks = 0;
    let _ = repair.draft.replace(package.clone());
    repair.draft_changed(&active, &mut checks);
    assert_eq!(
        (repair.refusal, repair.failure, checks),
        (ownership.clone(), None, 1)
    );
    // Round 24 SA1, FA1: Save asks again before it writes. A refusal found then (here the
    // note now belongs to another campaign) is Invalid with its cause, not `Not saved.`.
    let mut repair = refusal(&own);
    assert_eq!(repair.refusal, None);
    let elsewhere = Campaign {
        id: uuid::Uuid::new_v4(),
        ..active.clone()
    };
    assert!(!repair.save(&elsewhere));
    assert_eq!((repair.refusal, repair.failure), (ownership, None));
    // A write that fails on the disk is the failed Save, and only it.
    let mut repair = refusal(&rule);
    assert!(!repair.save(&active));
    assert_eq!(repair.refusal, None);
    assert!(repair.failure.is_some(), "{:?}", repair.failure);
}

#[test]
fn a_check_reaches_the_sidebar_and_the_review_list_and_changes_nothing() {
    let temp = tempfile::tempdir().unwrap();
    let active = campaign(temp.path());
    let campaigns = Campaigns::at(temp.path().join("settings.json"));
    let mut content = CampaignContent::default();
    assert_eq!(content.review_detail(), None);
    let proposal = |key: &str, blocked: Option<&str>| Proposal {
        key: key.into(),
        name: "Witch Bolt".into(),
        kind: "spell".into(),
        change: Move::ToPackage("Player's Handbook".into()),
        blocked: blocked.map(str::to_owned),
    };
    finished(
        &mut content,
        Checked {
            applied: None,
            proposals: Ok(vec![
                proposal("a", None),
                proposal("b", Some("It has personal notes.")),
            ]),
        },
    );
    assert_eq!(content.review_detail().as_deref(), Some("2 to review"));
    assert!(content.reconcile.pending.is_none());
    assert!(!content.refresh_pending);
    // Showing the list starts nothing. Only the DM's click applies a change.
    let _ = egui::Context::default().run(Default::default(), |ctx| {
        egui::CentralPanel::default()
            .show(ctx, |ui| content.reconcile_section(ui, &campaigns, &active));
    });
    assert!(content.reconcile.pending.is_none());
}

#[test]
fn closure_follows_a_selection_change() {
    let temp = tempfile::tempdir().unwrap();
    let active = campaign(temp.path());
    let campaigns = Campaigns::at(temp.path().join("settings.json"));
    let catalogue = Catalogue::load(&[], None).unwrap();
    let document = CampaignDocument::load(&active).unwrap();
    let closure = catalogue.closure(&document.selections, active.id);
    let mut content = CampaignContent {
        active: Some(active.clone()),
        snapshot: Some(Snapshot {
            closure: (document.selections.clone(), closure),
            document,
            subtitles: crate::library::catalogue_subtitles(&catalogue),
            catalogue,
            issues: None,
        }),
        ..Default::default()
    };
    let ctx = egui::Context::default();
    let frame = |content: &mut CampaignContent| {
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                content.selection_bar(ui, &campaigns, &active);
                content.packages(ui, &campaigns, &active);
            });
        });
    };
    let kept = |content: &CampaignContent| content.snapshot.as_ref().unwrap().closure.0.as_ptr();
    let before = kept(&content);
    frame(&mut content);
    assert_eq!(
        kept(&content),
        before,
        "the same selections keep the closure"
    );
    let direct =
        |content: &CampaignContent| content.snapshot.as_ref().unwrap().closure.1.direct.len();
    assert!(
        direct(&content) > 0,
        "a new campaign uses the built-in package"
    );
    let snapshot = content.snapshot.as_mut().unwrap();
    snapshot.document.selections.clear();
    frame(&mut content);
    assert_eq!(direct(&content), 0);
}

#[test]
fn undo_discard_stays_in_its_campaign() {
    let temp = tempfile::tempdir().unwrap();
    let (first, second) = (
        campaign(&temp.path().join("a")),
        campaign(&temp.path().join("b")),
    );
    let campaigns = Campaigns::at(temp.path().join("settings.json"));
    let ctx = egui::Context::default();
    let mut content = CampaignContent {
        active: Some(first.clone()),
        ..Default::default()
    };
    let repair_discard = |stamp, path: &str, text: &str| Payload::RepairDiscard {
        stamp,
        path: path.into(),
        text: text.into(),
    };
    new_result!(
        content,
        Payload::Discarded {
            selections: Vec::new(),
            baseline: None,
        }
    );
    // Wave 2 d (C): each discard is a kept result with its own Undo, in the store's order.
    // Round 26: two repairs' discards, each with its own Undo. A (note.md), then B
    // (other.md); Undo of A in the real footer restores A only: B's result and its Undo stay.
    new_result!(content, repair_discard(0, "note.md", "words"));
    new_result!(content, repair_discard(1, "other.md", "more"));
    {
        use egui_kittest::kittest::Queryable;
        let active = first.clone();
        let campaigns = Campaigns::at(temp.path().join("settings.json"));
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(960.0, 640.0))
            .build_ui_state(
                move |ui, content: &mut CampaignContent| {
                    let frame = design::outcome::PaneFrame::new(ui);
                    let open = |path: &str| {
                        ui.data(|data| {
                            data.get_temp::<bool>(egui::Id::new(("repair_open", path)))
                                .unwrap_or(false)
                        })
                    };
                    let messages =
                        content.page_footer_with(&frame, &open, &campaigns, &active, 600.0);
                    let paned = messages.pane(ui, design::Body::Content, |_| ());
                    content.press_result(ui.ctx(), &campaigns, paned.pressed);
                },
                content,
            );
        harness.run_steps(2);
        assert!(harness.query_by_label("Undo discard of other.md").is_some());
        harness.get_by_label("Undo discard of note.md").click();
        harness.run_steps(3);
        assert_eq!(
            repair_discards(harness.state(), first.id),
            ["other.md"],
            "Undo of A ends A only"
        );
        assert!(
            harness.query_by_label("Undo discard of note.md").is_none()
                && harness.query_by_label("Undo discard of other.md").is_some(),
            "B keeps its result and its Undo"
        );
        content = std::mem::take(harness.state_mut());
    }
    new_result!(content, repair_discard(2, "note.md", "words"));
    content.reload(&ctx, &campaigns, first.clone());
    assert!(
        has(&content, first.id, PageResult::Discarded),
        "a refresh keeps it"
    );
    assert_eq!(
        repair_discards(&content, first.id).len(),
        2,
        "a refresh keeps both"
    );
    content.reload(&ctx, &campaigns, second);
    assert!(!has(&content, first.id, PageResult::Discarded));
    assert!(repair_discards(&content, first.id).is_empty());

    // Round 23 FA8: the discard belongs to Campaign.md as it was read. The same file keeps
    // it; a changed or missing file ends it.
    let folder = temp.path().join("c");
    fs::create_dir(&folder).unwrap();
    let active = campaign(&folder);
    let document = CampaignDocument::load(&active).unwrap();
    content.active = Some(active.clone());
    let discarded = || Payload::Discarded {
        selections: Vec::new(),
        baseline: document.baseline.clone(),
    };
    new_result!(content, discarded());
    content
        .forget_discard_of_changed_file(active.id, Ok(&CampaignDocument::load(&active).unwrap()));
    assert!(
        has(&content, active.id, PageResult::Discarded),
        "the same file keeps it"
    );
    let file = active.path.join(&document.path);
    fs::write(
        &file,
        format!("{}\nA later note.\n", fs::read_to_string(&file).unwrap()),
    )
    .unwrap();
    content
        .forget_discard_of_changed_file(active.id, Ok(&CampaignDocument::load(&active).unwrap()));
    assert!(
        !has(&content, active.id, PageResult::Discarded),
        "a changed file ends it"
    );
    new_result!(content, discarded());
    fs::remove_file(&file).unwrap();
    let missing = CampaignDocument::load(&active).map(|_| ()).unwrap_err();
    content.forget_discard_of_changed_file(active.id, Err(&missing));
    assert!(
        !has(&content, active.id, PageResult::Discarded),
        "a missing file ends it"
    );
}
/// Wave 2 d (C) review: each recovery's Undo owns its own folder and registration. A finds
/// P and Q; B finds P again; back in A, A's P result is gone (its Undo would put back the
/// registration B's replaced), A's Q line names A's folder and its Undo acts on A's data, and
/// B keeps its result.
#[test]
fn a_recovery_undo_keeps_its_own_campaign_and_folder() {
    use crate::content_workspace::{EntryRef, PackageSelection, Selection};
    use egui_kittest::kittest::Queryable;
    struct Page {
        content: CampaignContent,
        campaigns: Campaigns,
        /// The Campaign page is not drawn (another page is open); its `tick` still runs.
        hidden: bool,
        /// The app's modal test (`crate::modal_open`), which Ctrl+S waits for.
        modal: bool,
        /// How often the app read Lookup and Encounters again for the page's written choices
        /// (`choices_written`, taken by `Splat::route`).
        refreshed: usize,
    }
    crate::hermetic::closed::clear_next_folder();
    let temp = tempfile::tempdir().unwrap();
    let packages = temp.path().join("packages");
    let package = |folder: &str, name: &str| {
        let path = packages.join(folder);
        fs::create_dir_all(&path).unwrap();
        fs::write(
            path.join("Package.md"),
            crate::library::note_text(name, "package", Some(&name.to_lowercase())),
        )
        .unwrap();
        crate::library::local_module(&path).unwrap()
    };
    let (watch, toll) = (package("watch", "Watch"), package("toll", "Toll"));
    let mut campaigns = Campaigns::at(temp.path().join("settings.json"));
    campaigns.register_module(watch.clone()).unwrap();
    campaigns.register_module(toll.clone()).unwrap();
    let open = |folder: &str| {
        let root = temp.path().join(folder);
        fs::create_dir(&root).unwrap();
        let active = campaign(&root);
        let mut document = CampaignDocument::load(&active).unwrap();
        for module in [&watch, &toll] {
            document.selections.push(PackageSelection {
                name: Some(module.name.clone()),
                package: EntryRef::id(module.id),
                selection: Selection::All,
            });
        }
        document.save().unwrap();
        active
    };
    let (first, second) = (open("a"), open("b"));
    campaigns.open_for_test(&second);
    campaigns.open_for_test(&first);
    let mut harness = egui_kittest::Harness::builder()
        .with_size(egui::vec2(1200.0, 900.0))
        .build_ui_state(
            |ui, page: &mut Page| {
                page.content.tick(ui.ctx(), &page.campaigns);
                if !page.hidden {
                    page.content.save_shortcut(ui.ctx(), page.modal);
                    page.content.show(ui, &page.campaigns);
                }
                // The app's own steps after the page (`lib.rs`).
                if let Some(module) = page.content.found_package.take() {
                    let result = page.campaigns.restore_module(module);
                    page.content.package_restored(result);
                }
                if let Some(id) = page.content.forget_package.take() {
                    let result = page.campaigns.unregister_module(id);
                    page.content.package_restored(result);
                }
                if std::mem::take(&mut page.content.choices_written) {
                    page.refreshed += 1;
                }
            },
            Page {
                content: CampaignContent::default(),
                campaigns,
                hidden: false,
                modal: false,
                refreshed: 0,
            },
        );
    // Waits on the loader itself, not on the clock: its answer (or its end) is received here
    // and handed back for the page to take in its next frame.
    let settle = |harness: &mut egui_kittest::Harness<'_, Page>| {
        for _ in 0..50 {
            harness.run_steps(2);
            let content = &mut harness.state_mut().content;
            if let Some(rx) = content.pending.take() {
                let (tx, again) = mpsc::channel();
                if let Ok(loaded) = rx.recv() {
                    tx.send(loaded).unwrap();
                }
                content.pending = Some(again);
            } else if !content.refresh_pending {
                harness.run_steps(2);
                return;
            }
        }
        panic!("the page did not load");
    };
    let find = |harness: &mut egui_kittest::Harness<'_, Page>, from: &str, to: &str| {
        let to = packages.join(to);
        fs::rename(packages.join(from), &to).unwrap();
        harness.state_mut().content.refresh_destination(&to);
        settle(harness);
        crate::hermetic::closed::next_folder(to);
        harness.get_by_label("Find the folder…").click();
        settle(harness);
    };
    let registered = |harness: &egui_kittest::Harness<'_, Page>, id: Uuid| {
        harness
            .state()
            .campaigns
            .modules()
            .iter()
            .find(|module| module.id == id)
            .map(|module| module.path.clone())
    };
    settle(&mut harness);
    // Issue #513: Ctrl+S, taken before the page draws and saved after the package list. The
    // built-in package unticked (Space on its focused box) and Ctrl+S in ONE input frame: the
    // save is without it, and the app reads Lookup and Encounters again (#509). Behind a modal
    // the key saves nothing (first, while the list can still be saved; Discard ends its draft).
    // A new campaign's first choice, the built-in package (the fixture adds Watch and Toll).
    let builtin = CampaignDocument::load(&first).unwrap().selections[0]
        .package
        .id
        .unwrap();
    let chosen_on_disk = |id: Uuid| {
        CampaignDocument::load(&first)
            .unwrap()
            .selections
            .iter()
            .any(|selection| selection.package.id == Some(id))
    };
    let key = |key, modifiers| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    };
    let one_frame = |harness: &mut egui_kittest::Harness<'_, Page>, events: &[egui::Event]| {
        let frame = harness.ctx.cumulative_frame_nr();
        harness.input_mut().events.extend(events.iter().cloned());
        harness.step();
        assert_eq!(
            harness.ctx.cumulative_frame_nr() - frame,
            1,
            "one input frame"
        );
    };
    let space = key(egui::Key::Space, egui::Modifiers::NONE);
    let ctrl_s = key(egui::Key::S, egui::Modifiers::COMMAND);
    assert!(builtin != watch.id && builtin != toll.id && chosen_on_disk(watch.id));
    let focus_builtin = |harness: &mut egui_kittest::Harness<'_, Page>| {
        harness.get_by_label("Use D&D 2024 · SRD 5.2.1").focus();
        harness.run_steps(2);
    };
    focus_builtin(&mut harness);
    harness.state_mut().modal = true;
    one_frame(&mut harness, &[space.clone(), ctrl_s.clone()]);
    harness.state_mut().modal = false;
    assert!(
        chosen_on_disk(builtin)
            && harness.state().content.selections_changed()
            && harness.state().refreshed == 0,
        "behind a modal Ctrl+S saved"
    );
    harness.run_steps(2);
    harness.get_by_label("Discard").click();
    harness.run_steps(2);
    assert!(!harness.state().content.selections_changed());
    focus_builtin(&mut harness);
    one_frame(&mut harness, &[space, ctrl_s]);
    assert!(
        !chosen_on_disk(builtin)
            && chosen_on_disk(watch.id)
            && chosen_on_disk(toll.id)
            && !harness.state().content.selections_changed()
            && harness.state().refreshed == 1,
        "Ctrl+S in the frame of the untick saves it: built-in on disk {}, draft left {}, \
         refreshed {}",
        chosen_on_disk(builtin),
        harness.state().content.selections_changed(),
        harness.state().refreshed
    );
    // Ruling R21 (G45): the repairs, then the links, then `Remove from this campaign` last
    // with exactly `L` before it. A reading-order test on the message's choices.
    fs::rename(packages.join("watch"), packages.join("watch-moved")).unwrap();
    harness
        .state_mut()
        .content
        .refresh_destination(&packages.join("watch-moved"));
    settle(&mut harness);
    // `L` after the choice before it on a shared line; else a line of its own at the words,
    // `M` below the others (`docs/design-system.md`, the destructive choice rule).
    let check = |harness: &egui_kittest::Harness<'_, Page>| {
        let order = [
            "Use this folder",
            "Find the folder…",
            "Technical details",
            "Remove from this campaign",
        ]
        .map(|name| harness.get_by_label(name).rect());
        assert!(
            order
                .windows(2)
                .all(|pair| (pair[0].center().y - pair[1].center().y).abs() < 1.0
                    && pair[0].right() < pair[1].left()
                    || pair[0].bottom() <= pair[1].top()),
            "the moved-folder choices are not in the order repairs, links, Remove: {order:?}"
        );
        let (before, remove) = (order[2], order[3]);
        let shares = (before.center().y - remove.center().y).abs() < 1.0;
        let gap = remove.left() - before.right();
        let above = remove.top()
            - order[..3]
                .iter()
                .map(|rect| rect.bottom())
                .fold(f32::MIN, f32::max);
        assert!(
            if shares {
                (gap - design::L).abs() < 0.5
            } else {
                (remove.left() - order[0].left()).abs() < 0.5 && (above - design::M).abs() < 0.5
            },
            "`Remove from this campaign` has not its room: shares a line {shares}, gap {gap}, \
             above {above}"
        );
        shares
    };
    assert!(
        check(&harness),
        "at 1200 px Remove shares the safe choices' line"
    );
    // Narrow: Remove wraps to a line of its own.
    harness.set_size(egui::vec2(500.0, 900.0));
    harness.run_steps(2);
    assert!(!check(&harness), "at 500 px Remove wraps");
    harness.set_size(egui::vec2(1200.0, 900.0));
    harness.run_steps(2);
    let moved = |harness: &mut egui_kittest::Harness<'_, Page>, from: &str, to: &str| {
        fs::rename(packages.join(from), packages.join(to)).unwrap();
        harness
            .state_mut()
            .content
            .refresh_destination(&packages.join(to));
        settle(harness);
    };
    // The first package's `Find the folder…` (Watch's: the list's order).
    let pick = |harness: &mut egui_kittest::Harness<'_, Page>, folder: &str| {
        crate::hermetic::closed::next_folder(packages.join(folder));
        harness
            .get_all_by_label("Find the folder…")
            .next()
            .unwrap()
            .click();
        settle(harness);
    };
    // Ruling Q-x (G149): A = the refusal of a wrong pick for Watch, B = Toll's package fault.
    // A is the answer to the last act, so it is first; B stays; A ends when its cause ends.
    moved(&mut harness, "toll", "toll-a");
    pick(&mut harness, "toll-a");
    let refusal =
        "The folder toll-a/ holds a different package. Choose the folder that holds Watch.";
    // The footer and the message carry the words too: the smallest node is the words' own.
    let message = |harness: &egui_kittest::Harness<'_, Page>, words: &str| {
        harness
            .query_all_by_label_contains(words)
            .map(|node| node.rect())
            .min_by(|a, b| a.area().total_cmp(&b.area()))
    };
    let toll_fault =
        |harness: &egui_kittest::Harness<'_, Page>| message(harness, "Toll moved from ");
    let shown = harness.get_by_label(refusal).rect();
    let fault = toll_fault(&harness).expect("Toll's fault stays beside the refusal");
    let watch_fault = message(&harness, "Watch moved from ").expect("Watch's fault stays");
    assert!(
        shown.bottom() <= fault.top() && shown.bottom() <= watch_fault.top(),
        "the refusal {shown:?} is not first, above the package faults {watch_fault:?}, {fault:?}"
    );
    // Watch comes back to its folder outside Splat, with no press: the refusal's cause ends,
    // and the refusal with it. Toll's fault stays.
    moved(&mut harness, "watch-moved", "watch");
    assert!(harness.query_by_label(refusal).is_none());
    assert!(toll_fault(&harness).is_some());
    moved(&mut harness, "watch", "watch-a");
    // The newest failed answer first, whatever its kind: a kept result whose Undo will fail
    // (the package choices it would restore are not the file's), then a refusal (first,
    // above that older result), then the failed Undo: its error is first and the refusal
    // is gone.
    {
        let content = &mut harness.state_mut().content;
        new_result!(
            content,
            Payload::Removed(RemovedPackage {
                name: "Gone".into(),
                words: removal_words("Gone", &[]).0,
                warning: None,
                before: Vec::new(),
                after: Vec::new(),
                users: Vec::new(),
            })
        );
    }
    harness.run_steps(2);
    pick(&mut harness, "toll-a");
    let kept = harness.get_by_label("Undo remove Gone").rect();
    let shown = harness.get_by_label(refusal).rect();
    assert!(
        shown.bottom() <= kept.top(),
        "the refusal {shown:?} is not above the older kept result {kept:?}"
    );
    harness.get_by_label("Undo remove Gone").click();
    settle(&mut harness);
    let failed = "Campaign choices changed. Keep them and edit the package choices here.";
    let error = message(&harness, failed).expect("the failed Undo says why");
    let kept = harness.get_by_label("Undo remove Gone").rect();
    assert!(
        error.bottom() <= kept.top() && harness.query_by_label(refusal).is_none(),
        "the failed Undo's error {error:?} is not first above {kept:?}, or the refusal stays"
    );
    // The campaign file, as the page last read it; a line added outside Splat makes the next
    // write of it fail, and the file put back lets the same act succeed.
    let file = {
        let document = &harness.state().content.snapshot.as_ref().unwrap().document;
        document.campaign.path.join(&document.path)
    };
    let text = fs::read_to_string(&file).unwrap();
    let outside = format!("{text}\nA line written outside Splat.\n");
    let answer = |harness: &egui_kittest::Harness<'_, Page>| {
        harness
            .state()
            .content
            .answer
            .as_ref()
            .map(|answer| answer.words.clone())
    };
    // A refusal, then a failed `Save selections`: the save's failure replaces the refusal and
    // leads the footer; its successful retry ends it.
    pick(&mut harness, "toll-a");
    assert!(harness.query_by_label(refusal).is_some());
    harness
        .state_mut()
        .content
        .snapshot
        .as_mut()
        .unwrap()
        .document
        .selections[1]
        .name = Some("Toll of the old road".into());
    fs::write(&file, &outside).unwrap();
    harness.run_steps(2);
    let refreshed = harness.state().refreshed;
    harness.get_by_label("Save selections").click();
    settle(&mut harness);
    assert_eq!(
        harness.state().refreshed,
        refreshed,
        "a failed save wrote no choices"
    );
    let failed = answer(&harness).expect("the failed save is the newest answer");
    let error = message(&harness, &failed).expect("the failed save says why");
    let kept = harness.get_by_label("Undo remove Gone").rect();
    assert!(
        harness.query_by_label(refusal).is_none() && error.bottom() <= kept.top(),
        "the failed save {error:?} does not lead above {kept:?}, or the refusal stays"
    );
    fs::write(&file, &text).unwrap();
    harness.get_by_label("Save selections").click();
    settle(&mut harness);
    assert!(
        answer(&harness).is_none() && message(&harness, &failed).is_none(),
        "the successful save did not end its failed try"
    );
    assert_eq!(
        harness.state().refreshed,
        refreshed + 1,
        "the save refreshed"
    );
    // A removal's Undo that fails (the file changed outside Splat), then its successful retry
    // (the file put back): the success ends the failure it stored.
    let text = fs::read_to_string(&file).unwrap();
    {
        let content = &mut harness.state_mut().content;
        let selections = content
            .snapshot
            .as_ref()
            .unwrap()
            .document
            .selections
            .clone();
        new_result!(
            content,
            Payload::Removed(RemovedPackage {
                name: "Gone".into(),
                words: removal_words("Gone", &[]).0,
                warning: None,
                before: selections.clone(),
                after: selections,
                users: Vec::new(),
            })
        );
    }
    fs::write(&file, format!("{text}\nA line written outside Splat.\n")).unwrap();
    harness.run_steps(2);
    let refreshed = harness.state().refreshed;
    harness.get_by_label("Undo remove Gone").click();
    settle(&mut harness);
    let failed = answer(&harness).expect("the failed Undo is the newest answer");
    assert!(message(&harness, &failed).is_some());
    assert_eq!(
        harness.state().refreshed,
        refreshed,
        "a failed Undo wrote no choices"
    );
    fs::write(&file, &text).unwrap();
    harness.get_by_label("Undo remove Gone").click();
    settle(&mut harness);
    assert_eq!(
        harness.state().refreshed,
        refreshed + 1,
        "the Undo refreshed"
    );
    assert!(
        answer(&harness).is_none()
            && message(&harness, &failed).is_none()
            && harness.query_by_label("Undo remove Gone").is_none(),
        "the successful Undo did not end its failed try"
    );
    // A refusal, and a round trip to another campaign while the page is hidden: the change
    // of campaign ends the stored answer, not only its words on screen.
    pick(&mut harness, "toll-a");
    assert!(harness.query_by_label(refusal).is_some());
    harness.state_mut().hidden = true;
    harness.state_mut().campaigns.open_for_test(&second);
    harness.run_steps(4);
    harness.state_mut().campaigns.open_for_test(&first);
    harness.run_steps(4);
    assert!(
        harness.state().content.answer.is_none(),
        "a refused pick of the first campaign outlived a change of campaign"
    );
    harness.state_mut().hidden = false;
    settle(&mut harness);
    assert!(harness.query_by_label(refusal).is_none());
    // A newer act answers: the refusal of a second wrong pick ends when Watch is found.
    pick(&mut harness, "toll-a");
    assert!(harness.query_by_label(refusal).is_some());
    pick(&mut harness, "watch-a");
    assert!(harness.query_by_label(refusal).is_none());
    assert!(toll_fault(&harness).is_some());
    // A finds Watch in watch-a/ and Toll in toll-a/.
    pick(&mut harness, "toll-a");
    assert!(
        harness
            .query_by_label("Undo recover Watch in packages/watch-a/")
            .is_some()
    );
    assert!(
        harness
            .query_by_label("Undo recover Toll in packages/toll-a/")
            .is_some()
    );
    // B: Watch moved again, and B finds it in watch-b/.
    harness.state_mut().campaigns.open_for_test(&second);
    settle(&mut harness);
    find(&mut harness, "watch-a", "watch-b");
    assert!(
        harness
            .query_by_label("Undo recover Watch in packages/watch-b/")
            .is_some()
    );
    // Back in A: no line names B's folder, and A's Watch result ended with B's recovery.
    harness.state_mut().campaigns.open_for_test(&first);
    settle(&mut harness);
    for gone in [
        "Undo recover Watch in packages/watch-b/",
        "Undo recover Watch in packages/watch-a/",
    ] {
        assert!(
            harness.query_by_label(gone).is_none(),
            "A shows {gone}: an Undo that is not A's to act on"
        );
    }
    // A's Toll line names A's folder, and its Undo acts on A's data: Toll's registration
    // goes back to toll/; Watch stays where B found it.
    harness
        .get_by_label("Undo recover Toll in packages/toll-a/")
        .click();
    settle(&mut harness);
    assert_eq!(registered(&harness, toll.id), Some(toll.path.clone()));
    assert_eq!(
        registered(&harness, watch.id),
        Some(packages.join("watch-b"))
    );
    assert!(
        harness
            .query_by_label("Undo recover Toll in packages/toll-a/")
            .is_none()
    );
    // B keeps its result and its Undo.
    harness.state_mut().campaigns.open_for_test(&second);
    settle(&mut harness);
    assert!(
        harness
            .query_by_label("Undo recover Watch in packages/watch-b/")
            .is_some()
    );
}

/// AD1: `Undo package removal` goes when the file's package choices changed after it.
#[test]
fn iter14_e_changed_choices_end_undo_package_removal() {
    use crate::content_workspace::{CampaignDocument, EntryRef, PackageSelection, Selection};
    let temp = tempfile::tempdir().unwrap();
    for folder in ["a", "b"] {
        fs::create_dir(temp.path().join(folder)).unwrap();
    }
    let active = campaign(&temp.path().join("a"));
    let other = campaign(&temp.path().join("b"));
    let selection = |name: &str| PackageSelection {
        name: Some(name.into()),
        package: EntryRef::id(uuid::Uuid::new_v4()),
        selection: Selection::All,
    };
    let mut document = CampaignDocument::load(&active).unwrap();
    let before = vec![selection("Gone")];
    document.selections = Vec::new();
    document.save().unwrap();
    let removed = || RemovedPackage {
        name: "Gone".into(),
        words: removal_words("Gone", &[]).0,
        warning: None,
        before: before.clone(),
        after: Vec::new(),
        users: Vec::new(),
    };
    let mut content = CampaignContent {
        active: Some(active.clone()),
        ..Default::default()
    };
    new_result!(content, Payload::Removed(removed()));
    // A reload of the file as the removal left it keeps Undo, and other edits keep it too.
    let file = active.path.join(&document.path);
    let text = format!("{}\nA later note.\n", fs::read_to_string(&file).unwrap());
    fs::write(&file, &text).unwrap();
    content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
    assert!(content.removal(active.id).is_some());
    // The file's package choices changed outside Splat: the result and its Undo go.
    let mut changed = CampaignDocument::load(&active).unwrap();
    changed.selections.push(selection("Added in Obsidian"));
    changed.save().unwrap();
    let bytes = fs::read(&file).unwrap();
    content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
    assert!(content.removal(active.id).is_none());
    assert_eq!(fs::read(&file).unwrap(), bytes, "the check writes nothing");
    // A removal from another campaign is not judged by this campaign's file.
    content.active = Some(other.clone());
    new_result!(content, Payload::Removed(removed()));
    content.forget_changed_removal(&CampaignDocument::load(&active).unwrap());
    assert!(content.removal(other.id).is_some());
}

#[test]
fn finding_a_package_rejects_another_packages_folder() {
    let dir = tempfile::tempdir().unwrap();
    let expected = dir.path().join("expected");
    let other = dir.path().join("other");
    fs::create_dir(&expected).unwrap();
    fs::create_dir(&other).unwrap();
    let text = crate::library::note_text("Ashen Marches", "package", Some("ashen"));
    let entry = write_note(&expected, "Package.md", None, &text).unwrap();
    let text = crate::library::note_text("Other package", "package", Some("other"));
    write_note(&other, "Package.md", None, &text).unwrap();
    let reference = EntryRef::id(entry.id);
    assert_eq!(
        find_package_folder(&reference, Some("Ashen Marches"), &expected)
            .unwrap()
            .name,
        "Ashen Marches"
    );
    // Round 23 FE3: the fault comes first and names the picked folder and the package.
    let folder = other.file_name().unwrap().to_string_lossy().into_owned();
    assert_eq!(
        find_package_folder(&reference, Some("Ashen Marches"), &other).unwrap_err(),
        format!(
            "The folder {folder}/ holds a different package. Choose the folder that holds Ashen Marches."
        )
    );
}
#[cfg(feature = "test-support")]
pub fn applied_changes_reload_the_catalogue_and_failures_are_shown(
    observe_switch: impl FnOnce(CampaignContent) -> CampaignContent,
) {
    let mut content = CampaignContent::default();
    finished(
        &mut content,
        Checked {
            applied: Some(Ok(2)),
            proposals: Ok(Vec::new()),
        },
    );
    assert!(content.refresh_pending);
    assert_eq!(content.review_detail(), None);
    let applied = kinds_and(&content, Uuid::nil(), |payload| match payload {
        Payload::Applied(count) => Some(*count),
        _ => None,
    });
    assert!(applied_words(applied[0]).as_str().starts_with("2 changes"));
    finished(
        &mut content,
        Checked {
            applied: Some(Err("A sheet has local edits.".into())),
            proposals: Ok(Vec::new()),
        },
    );
    // Ruling Q-x: the failed Apply is the newest answer, first in the footer.
    assert_eq!(
        content.answer.as_ref().map(|answer| answer.words.as_str()),
        Some("A sheet has local edits.")
    );
    // An Apply started in campaign A that completes after the DM opened campaign B belongs
    // to A: its failure and its proposals are dropped, never shown in B, and B's check is
    // due again.
    let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
    content.answer = None;
    content.reconcile.campaign = Some(a);
    finished_in(
        &mut content,
        Checked {
            applied: Some(Err("A sheet has local edits.".into())),
            proposals: Ok(Vec::new()),
        },
        Some(b),
    );
    assert!(
        content.answer.is_none(),
        "A's late failure leads B's footer: {:?}",
        content
            .answer
            .as_ref()
            .map(|answer| (answer.campaign, &answer.words))
    );
    assert_eq!(content.reconcile.campaign, None, "B's check is due");
    // A -> B -> A before the Apply completes: the change of campaign (run by `tick`, also
    // while the page is hidden) drops the pending completion; the old one never enters A's
    // new visit.
    let temp = tempfile::tempdir().unwrap();
    let (first, second) = (
        campaign(&temp.path().join("a")),
        campaign(&temp.path().join("b")),
    );
    content.active = Some(first.clone());
    content.reconcile.campaign = Some(first.id);
    let (tx, rx) = mpsc::channel();
    content.reconcile.pending = Some(rx);
    content.forget_discards_of_other_campaigns(Some(&second));
    content.forget_discards_of_other_campaigns(Some(&first));
    let _ = tx.send(Checked {
        applied: Some(Err("A sheet has local edits.".into())),
        proposals: Ok(Vec::new()),
    });
    content.finish_check(Some(first.id));
    assert!(
        content.answer.is_none() && content.reconcile.pending.is_none(),
        "an Apply from A's earlier visit entered A's new state"
    );
    // The page loaded A and is hidden; the DM opens B. Each tick runs the campaign change and
    // then starts a check when one is due (`check_due`, as `tick` does): N ticks start one
    // check for B, and it is never cancelled by the page's own, older campaign.
    content.active = Some(first.clone());
    let mut senders = Vec::new();
    let mut starts = 0;
    for _ in 0..10 {
        content.forget_discards_of_other_campaigns(Some(&second));
        if content.check_due(second.id) {
            let (tx, rx) = mpsc::channel::<Checked>();
            content.reconcile.campaign = Some(second.id);
            content.reconcile.pending = Some(rx);
            senders.push(tx);
            starts += 1;
        }
    }
    assert_eq!(
        starts, 1,
        "ticks on a hidden page started {starts} checks for B"
    );
    // B's check completes while the page holds A; the DM returns to A, whose own check then
    // fails: A never shows B's proposals (the check's stored data ends with its campaign).
    let witch = || Proposal {
        key: "witch-bolt".into(),
        name: "Witch Bolt".into(),
        kind: "spell".into(),
        change: Move::ToPackage("Player's Handbook".into()),
        blocked: None,
    };
    finished_in(
        &mut content,
        Checked {
            applied: None,
            proposals: Ok(vec![witch()]),
        },
        Some(second.id),
    );
    assert!(content.review_detail().is_some(), "B's proposals are B's");
    content.forget_discards_of_other_campaigns(Some(&first));
    assert!(
        content.reconcile.proposals.is_empty() && content.review_detail().is_none(),
        "back in A, B's proposals show while A's own check runs"
    );
    assert!(content.check_due(first.id));
    content.set_check_owner(Some(first.id));
    finished_in(
        &mut content,
        Checked {
            applied: None,
            proposals: Err("The campaign package check stopped.".into()),
        },
        Some(first.id),
    );
    assert!(
        content.reconcile.proposals.is_empty() && content.review_detail().is_none(),
        "A shows B's proposals after its own check failed"
    );
    // An Apply runs in A: the campaign switch is refused with its reason, a campaign change
    // does not cancel it, and its failure answers in A when it completes.
    let (tx, rx) = mpsc::channel();
    content.set_check_owner(Some(first.id));
    content.reconcile.pending = Some(rx);
    content.reconcile.applying = true;
    let mut content = observe_switch(content);
    content.forget_discards_of_other_campaigns(Some(&second));
    assert!(
        content.applying(),
        "a campaign change cancelled a running Apply"
    );
    tx.send(Checked {
        applied: Some(Err("A sheet has local edits.".into())),
        proposals: Ok(Vec::new()),
    })
    .unwrap();
    content.finish_check(Some(first.id));
    assert_eq!(
        content
            .answer
            .as_ref()
            .map(|answer| (answer.campaign, answer.words.as_str())),
        Some((first.id, "A sheet has local edits.")),
        "the Apply's failure answers in its own campaign"
    );
}

/// Review of PR 740 (P2): Campaign content's issues section stays as the DM left it when a
/// repair changes its count (one of several issues repaired), and Campaign.md flags that appear
/// open it.
#[test]
fn the_issues_section_keeps_its_state_through_a_repair() {
    use crate::design::test_harness::ThemedPage as _;
    use egui_kittest::kittest::{NodeT, Queryable};
    let shown = std::rc::Rc::new(std::cell::Cell::new(("3 content issues to repair", false)));
    let drawn = shown.clone();
    let mut harness = egui_kittest::Harness::builder()
        .with_size(egui::vec2(600.0, 300.0))
        .themed_ui(move |ui| {
            let (heading, flagged) = drawn.get();
            let _ = issues_section(ui, heading, flagged, |ui| ui.label("The issue lines"));
        });
    harness.run_steps(2);
    let header = |heading: &str| format!("{} {heading}", design::icons::WARN);
    let open = |harness: &egui_kittest::Harness<'_>, heading: &str| {
        harness
            .get_by_label(&header(heading))
            .accesskit_node()
            .data()
            .is_expanded()
    };
    assert_eq!(
        open(&harness, "3 content issues to repair"),
        Some(false),
        "closed at first: no Campaign.md flags"
    );
    harness
        .get_by_label(&header("3 content issues to repair"))
        .click();
    harness.run_steps(3);
    assert!(harness.query_by_label("The issue lines").is_some());
    // The DM repairs one of the three: the count changes, and the list stays open.
    shown.set(("2 content issues to repair", false));
    harness.run_steps(3);
    assert_eq!(
        open(&harness, "2 content issues to repair"),
        Some(true),
        "m-p0-issues-stay-open: a repair that changes the count keeps the list open"
    );
    assert!(harness.query_by_label("The issue lines").is_some());
    // The DM closes it; then Campaign.md flags appear (a reload), and the list opens for them.
    harness
        .get_by_label(&header("2 content issues to repair"))
        .click();
    harness.run_steps(3);
    assert!(harness.query_by_label("The issue lines").is_none());
    shown.set(("2 content issues to repair · 1 Campaign.md issue", true));
    harness.run_steps(3);
    assert_eq!(
        open(&harness, "2 content issues to repair · 1 Campaign.md issue"),
        Some(true),
        "m-p0-issues-flags-open: flags that appear open the list"
    );
}
