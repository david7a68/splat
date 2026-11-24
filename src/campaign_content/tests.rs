use super::*;
use splat::character::campaign_package::Move;

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
        let content = CampaignContent {
            snapshot: Some(load_snapshot(&active, &[module]).1.unwrap()),
            ..Default::default()
        };
        // Round 26 FD1: the page's footer says the warnings; the package list's rows follow.
        let mut messages = design::outcome::Messages::new();
        let unavailable = content.package_warnings_of(&campaigns).unavailable;
        content.package_messages(&mut messages, &campaigns, &unavailable);
        let lines = messages.into_plain();
        let rows = content.package_rows(&campaigns);
        let rendered = lines
            .iter()
            .flat_map(|line| {
                std::iter::once(line.words.clone())
                    .chain(line.help.iter().cloned())
                    .chain(line.choices.iter().map(|choice| choice.words.clone()))
            })
            .chain(rows.iter().flat_map(|row| {
                [
                    row.name.clone(),
                    row.detail.clone(),
                    row.mark
                        .map(|(_, words)| words.to_owned())
                        .unwrap_or_default(),
                    row.note.clone().unwrap_or_default(),
                ]
            }))
            .collect::<Vec<_>>()
            .join("\n");
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
    // A load error that starts with the note's path.
    assert_eq!(
        words(ContentFault::Error("notes/a.md: Missing YAML frontmatter")),
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
    // Reading the page starts nothing. Only the DM's click applies a change.
    let _ = content.issues(&active);
    let _ = content.package_rows(&campaigns);
    let _ = content.footer_lines(&campaigns, &active, &|_| false);
    assert!(content.reconcile.pending.is_none());
}

#[test]
fn closure_follows_a_selection_change() {
    let temp = tempfile::tempdir().unwrap();
    let active = campaign(temp.path());
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
    // The page's save row reads the closure again when the selections changed (`save_bar`).
    let frame = |content: &mut CampaignContent| {
        let _ = content.save_bar();
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
    let ctx = StateContext::new();
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
        // The page's footer as the GPUI page builds it, and a press of an Undo in it.
        let undo_of = |content: &CampaignContent, label: &str| {
            content
                .footer_lines(&campaigns, &first, &|_| false)
                .iter()
                .find(|line| line.undo.as_ref().is_some_and(|undo| undo.label == label))
                .and_then(|line| line.press_undo())
        };
        assert!(undo_of(&content, "Undo discard of other.md").is_some());
        let press = undo_of(&content, "Undo discard of note.md").expect("A's Undo");
        content.footer_press(&ctx, &campaigns, &first, press);
        assert_eq!(
            repair_discards(&content, first.id),
            ["other.md"],
            "Undo of A ends A only"
        );
        assert!(
            undo_of(&content, "Undo discard of note.md").is_none()
                && undo_of(&content, "Undo discard of other.md").is_some(),
            "B keeps its result and its Undo"
        );
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
    use design::outcome::plain::{ChoiceForm, PlainLine};
    struct Page {
        content: CampaignContent,
        campaigns: Campaigns,
        /// The Campaign page is not drawn (another page is open); its `tick` still runs.
        hidden: bool,
        /// How often the app read Lookup and Encounters again for the page's written choices
        /// (`choices_written`, taken by `Splat::route`).
        refreshed: usize,
        ctx: StateContext,
    }
    impl Page {
        /// One frame as the GPUI app runs it: `tick` on any page, the Packages tab's reads
        /// while it is shown, then the app's own steps after the page (`lib.rs`).
        fn frame(&mut self) {
            self.content.tick(&self.ctx, &self.campaigns);
            if !self.hidden {
                let _ = self.content.prepare(&self.ctx, &self.campaigns);
                let _ = self.content.save_bar();
            }
            if let Some(module) = self.content.found_package.take() {
                let result = self.campaigns.restore_module(module);
                self.content.package_restored(result);
            }
            if let Some(id) = self.content.forget_package.take() {
                let result = self.campaigns.unregister_module(id);
                self.content.package_restored(result);
            }
            if std::mem::take(&mut self.content.choices_written) {
                self.refreshed += 1;
            }
        }
        /// The page's footer, in its order.
        fn lines(&self) -> Vec<PlainLine<ContentPress>> {
            let active = self.campaigns.active_campaign().unwrap();
            self.content
                .footer_lines(&self.campaigns, &active, &|_| false)
        }
        /// Where the first line whose words contain `words` stands in the footer.
        fn message(&self, words: &str) -> Option<usize> {
            self.lines()
                .iter()
                .position(|line| line.words.contains(words))
        }
        /// Where the line with the Undo `label` stands.
        fn undo(&self, label: &str) -> Option<usize> {
            self.lines()
                .iter()
                .position(|line| line.undo.as_ref().is_some_and(|undo| undo.label == label))
        }
        fn press(&mut self, press: design::outcome::Press<ContentPress>) {
            let active = self.campaigns.active_campaign().unwrap();
            self.content
                .footer_press(&self.ctx, &self.campaigns, &active, press);
        }
        fn press_undo(&mut self, label: &str) {
            let at = self.undo(label).unwrap_or_else(|| panic!("no {label}"));
            let press = self.lines()[at].press_undo().expect("the Undo is open");
            self.press(press);
        }
        /// The first line's choice `words` (Watch's `Find the folder…`: the list's order).
        fn press_choice(&mut self, words: &str) {
            let press = self
                .lines()
                .iter()
                .find_map(|line| {
                    let at = line
                        .choices
                        .iter()
                        .position(|choice| choice.words == words)?;
                    line.press_choice(at)
                })
                .unwrap_or_else(|| panic!("no choice {words}"));
            self.press(press);
        }
        /// Waits on the loader itself, not on the clock: its answer (or its end) is received
        /// here and handed back for the page to take in its next frame.
        fn settle(&mut self) {
            for _ in 0..50 {
                self.frame();
                self.frame();
                if let Some(rx) = self.content.pending.take() {
                    let (tx, again) = mpsc::channel();
                    if let Ok(loaded) = rx.recv() {
                        tx.send(loaded).unwrap();
                    }
                    self.content.pending = Some(again);
                } else if !self.content.refresh_pending {
                    self.frame();
                    return;
                }
            }
            panic!("the page did not load");
        }
        fn answer(&self) -> Option<String> {
            self.content
                .answer
                .as_ref()
                .map(|answer| answer.words.clone())
        }
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
    let mut page = Page {
        content: CampaignContent::default(),
        campaigns,
        hidden: false,
        refreshed: 0,
        ctx: StateContext::new(),
    };
    let moved = |page: &mut Page, from: &str, to: &str| {
        fs::rename(packages.join(from), packages.join(to)).unwrap();
        page.content.refresh_destination(&packages.join(to));
        page.settle();
    };
    let pick = |page: &mut Page, folder: &str| {
        crate::hermetic::closed::next_folder(packages.join(folder));
        page.press_choice("Find the folder…");
        page.settle();
    };
    let registered = |page: &Page, id: Uuid| {
        page.campaigns
            .modules()
            .iter()
            .find(|module| module.id == id)
            .map(|module| module.path.clone())
    };
    page.settle();
    // Ruling R21 (G45): the repairs, then the links, then `Remove from this campaign` last, in
    // the lose form (the kit draws it `L` after the others).
    moved(&mut page, "watch", "watch-moved");
    let order = [
        "Use this folder",
        "Find the folder…",
        "Technical details",
        "Remove from this campaign",
    ];
    let line = page
        .lines()
        .into_iter()
        .find(|line| line.choices.iter().any(|c| c.words == "Find the folder…"))
        .expect("the moved folder's message");
    let words: Vec<&str> = line
        .choices
        .iter()
        .map(|choice| choice.words.as_str())
        .filter(|words| order.contains(words))
        .collect();
    assert_eq!(
        words, order,
        "the moved-folder choices are not in the order repairs, links, Remove"
    );
    assert_eq!(
        line.choices.last().map(|choice| choice.form),
        Some(ChoiceForm::Lose)
    );
    // Ruling Q-x (G149): A = the refusal of a wrong pick for Watch, B = Toll's package fault.
    // A is the answer to the last act, so it is first; B stays; A ends when its cause ends.
    moved(&mut page, "toll", "toll-a");
    pick(&mut page, "toll-a");
    let refusal =
        "The folder toll-a/ holds a different package. Choose the folder that holds Watch.";
    let shown = page.message(refusal).expect("the refusal");
    let fault = page
        .message("Toll moved from ")
        .expect("Toll's fault stays beside the refusal");
    let watch_fault = page
        .message("Watch moved from ")
        .expect("Watch's fault stays");
    assert!(
        shown < fault && shown < watch_fault,
        "the refusal {shown} is not first, above the package faults {watch_fault}, {fault}"
    );
    // Watch comes back to its folder outside Splat, with no press: the refusal's cause ends,
    // and the refusal with it. Toll's fault stays.
    moved(&mut page, "watch-moved", "watch");
    assert!(page.message(refusal).is_none());
    assert!(page.message("Toll moved from ").is_some());
    moved(&mut page, "watch", "watch-a");
    // The newest failed answer first, whatever its kind: a kept result whose Undo will fail
    // (the package choices it would restore are not the file's), then a refusal (first,
    // above that older result), then the failed Undo: its error is first and the refusal
    // is gone.
    {
        let content = &mut page.content;
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
    page.frame();
    pick(&mut page, "toll-a");
    let kept = page.undo("Undo remove Gone").expect("the kept result");
    let shown = page.message(refusal).expect("the refusal");
    assert!(
        shown < kept,
        "the refusal {shown} is not above the older kept result {kept}"
    );
    page.press_undo("Undo remove Gone");
    page.settle();
    let failed = "Campaign choices changed. Keep them and edit the package choices here.";
    let error = page.message(failed).expect("the failed Undo says why");
    let kept = page
        .undo("Undo remove Gone")
        .expect("the kept result stays");
    assert!(
        error < kept && page.message(refusal).is_none(),
        "the failed Undo's error {error} is not first above {kept}, or the refusal stays"
    );
    // The campaign file, as the page last read it; a line added outside Splat makes the next
    // write of it fail, and the file put back lets the same act succeed.
    let file = {
        let document = &page.content.snapshot.as_ref().unwrap().document;
        document.campaign.path.join(&document.path)
    };
    let text = fs::read_to_string(&file).unwrap();
    let outside = format!("{text}\nA line written outside Splat.\n");
    // A refusal, then a failed `Save selections`: the save's failure replaces the refusal and
    // leads the footer; its successful retry ends it.
    pick(&mut page, "toll-a");
    assert!(page.message(refusal).is_some());
    page.content.snapshot.as_mut().unwrap().document.selections[1].name =
        Some("Toll of the old road".into());
    fs::write(&file, &outside).unwrap();
    page.frame();
    let refreshed = page.refreshed;
    page.content.save_now();
    page.settle();
    assert_eq!(page.refreshed, refreshed, "a failed save wrote no choices");
    let failed = page.answer().expect("the failed save is the newest answer");
    let error = page.message(&failed).expect("the failed save says why");
    let kept = page.undo("Undo remove Gone").expect("the kept result");
    assert!(
        page.message(refusal).is_none() && error < kept,
        "the failed save {error} does not lead above {kept}, or the refusal stays"
    );
    fs::write(&file, &text).unwrap();
    page.content.save_now();
    page.settle();
    assert!(
        page.answer().is_none() && page.message(&failed).is_none(),
        "the successful save did not end its failed try"
    );
    assert_eq!(page.refreshed, refreshed + 1, "the save refreshed");
    // A removal's Undo that fails (the file changed outside Splat), then its successful retry
    // (the file put back): the success ends the failure it stored.
    let text = fs::read_to_string(&file).unwrap();
    {
        let content = &mut page.content;
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
    page.frame();
    let refreshed = page.refreshed;
    page.press_undo("Undo remove Gone");
    page.settle();
    let failed = page.answer().expect("the failed Undo is the newest answer");
    assert!(page.message(&failed).is_some());
    assert_eq!(page.refreshed, refreshed, "a failed Undo wrote no choices");
    fs::write(&file, &text).unwrap();
    page.press_undo("Undo remove Gone");
    page.settle();
    assert_eq!(page.refreshed, refreshed + 1, "the Undo refreshed");
    assert!(
        page.answer().is_none()
            && page.message(&failed).is_none()
            && page.undo("Undo remove Gone").is_none(),
        "the successful Undo did not end its failed try"
    );
    // A refusal, and a round trip to another campaign while the page is hidden: the change
    // of campaign ends the stored answer, not only its words on screen.
    pick(&mut page, "toll-a");
    assert!(page.message(refusal).is_some());
    page.hidden = true;
    page.campaigns.open_for_test(&second);
    page.frame();
    page.frame();
    page.campaigns.open_for_test(&first);
    page.frame();
    page.frame();
    assert!(
        page.content.answer.is_none(),
        "a refused pick of the first campaign outlived a change of campaign"
    );
    page.hidden = false;
    page.settle();
    assert!(page.message(refusal).is_none());
    // A newer act answers: the refusal of a second wrong pick ends when Watch is found.
    pick(&mut page, "toll-a");
    assert!(page.message(refusal).is_some());
    pick(&mut page, "watch-a");
    assert!(page.message(refusal).is_none());
    assert!(page.message("Toll moved from ").is_some());
    // A finds Watch in watch-a/ and Toll in toll-a/.
    pick(&mut page, "toll-a");
    assert!(
        page.undo("Undo recover Watch in packages/watch-a/")
            .is_some()
    );
    assert!(page.undo("Undo recover Toll in packages/toll-a/").is_some());
    // B: Watch moved again, and B finds it in watch-b/.
    page.campaigns.open_for_test(&second);
    page.settle();
    moved(&mut page, "watch-a", "watch-b");
    pick(&mut page, "watch-b");
    assert!(
        page.undo("Undo recover Watch in packages/watch-b/")
            .is_some()
    );
    // Back in A: no line names B's folder, and A's Watch result ended with B's recovery.
    page.campaigns.open_for_test(&first);
    page.settle();
    for gone in [
        "Undo recover Watch in packages/watch-b/",
        "Undo recover Watch in packages/watch-a/",
    ] {
        assert!(
            page.undo(gone).is_none(),
            "A shows {gone}: an Undo that is not A's to act on"
        );
    }
    // A's Toll line names A's folder, and its Undo acts on A's data: Toll's registration
    // goes back to toll/; Watch stays where B found it.
    page.press_undo("Undo recover Toll in packages/toll-a/");
    page.settle();
    assert_eq!(registered(&page, toll.id), Some(toll.path.clone()));
    assert_eq!(registered(&page, watch.id), Some(packages.join("watch-b")));
    assert!(page.undo("Undo recover Toll in packages/toll-a/").is_none());
    // B keeps its result and its Undo.
    page.campaigns.open_for_test(&second);
    page.settle();
    assert!(
        page.undo("Undo recover Watch in packages/watch-b/")
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
