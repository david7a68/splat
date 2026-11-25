//! Screenshots and interaction tests of the Library page. Captures are named after the
//! scenarios' steps (`<scenario>__<step>__<w>x<h>`), so they pair with the references.
use crate::Page;
use crate::fixtures::Seed;
use crate::gui::harness::Shell;
use gpui::{px, size};

/// Open the Library and settle its load.
fn library(shell: &mut Shell) {
    shell.click("Library");
    loaded(shell);
}

/// Wait for a load (the first, or a reload after an act) to finish. It runs on a thread; its
/// result arrives at a later frame.
fn loaded(shell: &mut Shell) {
    for _ in 0..200 {
        let done = shell.read(|root| root.app.library.loaded() && !root.app.library.loading());
        if done {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.settle();
    assert!(
        shell.read(|root| root.app.library.loaded() && !root.app.library.loading()),
        "the Library loads"
    );
}

/// Choose a scope in the scope drop-down by its words.
fn scope(shell: &mut Shell, words: &str) {
    shell.click("id:library-scope");
    shell.click(words);
}

/// Search the Library for `words`, typed in one input (`steps::search_library`): Clear first
/// when the field has words.
fn search(shell: &mut Shell, words: &str) {
    if shell.bounds("id:library-search-clear").is_some() {
        shell.click("id:library-search-clear");
    }
    shell.click("id:library-search");
    shell.insert_text(words);
    assert_eq!(
        shell.read(|root| root.app.library.search_text().to_owned()),
        words
    );
}

/// Open the list row whose words start with `start`.
fn open_row(shell: &mut Shell, start: &str) {
    let row = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with(start))
        .unwrap_or_else(|| panic!("no row starts with {start:?}"));
    shell.click(&row);
}

fn open_name(shell: &mut Shell) -> Option<String> {
    shell.read(|root| root.app.library.open_name().map(str::to_owned))
}

#[test]
fn library_browse() {
    let mut shell = Shell::new("library-browse", Seed::Empty);
    library(&mut shell);
    assert_eq!(shell.read(|root| root.app.page), Page::Library);
    shell.capture("library-browse__01-first-launch__1200x800");
    // A search opens its first result, which is the highlighted row.
    search(&mut shell, "dragon");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Dragon Orb"));
    shell.capture("library-browse__02-search-dragon__1200x800");
    open_row(&mut shell, "Adult Black Dragon · ");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Adult Black Dragon"));
    shell.capture("library-browse__03-creature-stat-block__1200x800");
    search(&mut shell, "fireball");
    open_row(&mut shell, "Fireball · ");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Fireball"));
    shell.capture("library-browse__04-spell__1200x800");
    search(&mut shell, "wizard");
    open_row(&mut shell, "Wizard · Class");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Wizard"));
    shell.capture("library-browse__05-class-traits-and-features__1200x800");
    // A built-in entry's Markdown source reads, but takes no typing.
    shell.click("Markdown source");
    assert!(shell.read(|root| root.app.library.source_open()));
    // The words and the caret: typing would move the caret even when the words go back.
    let words = |shell: &mut Shell| {
        shell.update(|root, _, cx| {
            root.pages
                .library
                .editor
                .as_ref()
                .map(|(_, handle)| (handle.words(cx).to_string(), handle.caret(cx)))
        })
    };
    shell.click("Markdown editor");
    let before = words(&mut shell).expect("the editor");
    shell.type_text("typed");
    assert_eq!(
        words(&mut shell).expect("the editor"),
        before,
        "the built-in source takes no typing"
    );
    assert!(!shell.read(|root| root.app.library.dirty()));
    let mut shell = Shell::with_size("library-browse", Seed::Empty, size(px(960.0), px(640.0)));
    library(&mut shell);
    search(&mut shell, "dragon");
    open_row(&mut shell, "Adult Black Dragon · ");
    shell.capture("library-browse__03-creature-stat-block__960x640");
    // Round 25 FF5: a duplicate row's folder ends its second line, whole; the words before it
    // shorten (`list::folder_last`). A line with no folder is one part.
    assert_eq!(
        super::list::folder_last(
            "Same entry in two files · Creature · CR 1 · Ashen Marches · ashen-marches-copy/"
        ),
        (
            "Same entry in two files · Creature · CR 1 · Ashen Marches",
            Some(" · ashen-marches-copy/")
        )
    );
    assert_eq!(
        super::list::folder_last("Creature · CR 1 · Ashen Marches"),
        ("Creature · CR 1 · Ashen Marches", None)
    );
}

#[test]
fn library_author() {
    let mut shell = Shell::new("library-author", Seed::Campaign);
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    shell.click("Ash Exposure · Rule");
    assert_eq!(
        shell.read(|root| root.app.library.open_name().map(str::to_owned)),
        Some("Ash Exposure".to_owned())
    );
    shell.capture("library-author__01-local-package__1200x800");
    shell.click("Edit Markdown");
    assert!(shell.read(|root| root.app.library.source_open()));
    shell.capture("library-author__02-edit-markdown__1200x800");
    // Type at the end of the draft.
    shell.click("Markdown editor");
    shell.press("ctrl-end");
    shell.type_text(DRAFT_WORDS);
    assert!(
        shell.read(|root| root.app.library.dirty()),
        "typing makes a draft"
    );
    assert!(shell.bounds("Discard").is_some(), "a draft offers Discard");
    shell.capture("library-author__03-dirty__1200x800");
    // Ctrl+S saves the draft to its file.
    shell.press("ctrl-s");
    loaded(&mut shell);
    assert!(!shell.read(|root| root.app.library.dirty()), "Ctrl+S saves");
    let file = shell.read(|root| {
        root.app
            .campaigns
            .modules()
            .iter()
            .find(|module| module.name == "Ashen Marches")
            .map(|module| module.path.join("Rules/Ash Exposure.md"))
            .expect("the package")
    });
    assert!(
        std::fs::read_to_string(&file)
            .unwrap()
            .contains(DRAFT_WORDS.trim())
    );
    shell.capture("library-author__05-saved-with-ctrl-s__1200x800");
    // A change on disk under a draft: Save stops, and the choices show.
    shell.type_text(SECOND_WORDS);
    let edited = std::fs::read_to_string(&file).unwrap() + "\nEdited elsewhere.\n";
    std::fs::write(&file, edited).unwrap();
    shell.click("id:library-save");
    assert!(shell.read(|root| root.app.library.disk_changed()));
    assert!(shell.bounds("Overwrite with my draft").is_some());
    shell.capture("library-author__06-save-conflict__1200x800");
    // Reload and lose my draft: the file wins, and Discard's Undo gives the draft back.
    shell.click("Reload and lose my draft");
    loaded(&mut shell);
    assert!(!shell.read(|root| root.app.library.disk_changed()));
    let draft = |shell: &mut Shell| shell.read(|root| root.app.library.draft_text().to_owned());
    assert!(draft(&mut shell).contains("Edited elsewhere."));
    shell.click("Undo discard of Ash Exposure");
    assert!(
        draft(&mut shell).contains(SECOND_WORDS.trim()),
        "Undo gives the draft back"
    );
    // After the reload, the same draft saves.
    shell.click("id:library-save");
    loaded(&mut shell);
    assert!(
        std::fs::read_to_string(&file)
            .unwrap()
            .contains(SECOND_WORDS.trim())
    );
    // Overwrite puts the draft on disk in place of the other change.
    shell.click("Markdown editor");
    shell.press("ctrl-end");
    shell.type_text(THIRD_WORDS);
    let edited = std::fs::read_to_string(&file).unwrap() + "\nEdited again.\n";
    std::fs::write(&file, edited).unwrap();
    shell.click("id:library-save");
    shell.click("Overwrite with my draft");
    loaded(&mut shell);
    let text = std::fs::read_to_string(&file).unwrap();
    assert!(
        text.contains(THIRD_WORDS.trim()) && !text.contains("Edited again."),
        "Overwrite writes the draft: {text}"
    );
    // A new draft, discarded: the file's words come back, with Undo.
    shell.click("Markdown editor");
    shell.press("ctrl-end");
    shell.type_text(THIRD_WORDS);
    shell.click("Discard");
    assert!(!shell.read(|root| root.app.library.dirty()));
    assert!(
        shell.bounds("Undo discard of Ash Exposure").is_some(),
        "{:?}",
        crate::gui::kit::probe::names()
            .into_iter()
            .filter(|name| name.contains("Undo") || name.contains("footer"))
            .collect::<Vec<_>>()
    );
    shell.capture("library-author__07-library-draft-discarded__1200x800");
    // Manage packages, from the Packages menu.
    shell.click("Packages");
    shell.capture("library-author__08-package-actions__1200x800");
    shell.click("Manage packages…");
    assert!(shell.read(|root| root.app.library.forms().0));
    shell.capture("library-author__09-manage-packages__1200x800");
    shell.press("escape");
    assert!(
        !shell.read(|root| root.app.library.forms().0),
        "Escape closes the form"
    );
    // New entry: a creature in the package, opened in the editor. The form opens with the
    // name field holding the keyboard.
    shell.click("id:library-new");
    shell.type_text("Cinder Hound");
    let form = |shell: &mut Shell| {
        shell.update(|root, _, _| {
            let app = &mut root.app;
            let view = app
                .library
                .new_entry_view(&app.campaigns)
                .expect("the form");
            (view.name, view.folder)
        })
    };
    assert_eq!(
        form(&mut shell).0,
        "Cinder Hound",
        "the name field takes the keyboard when the form opens"
    );
    // The folder's suggestions: the package's folders, read when the menu opens; a choice
    // fills the field, and `Main folder` empties it.
    // The menu lists the folders whose names hold the field's words, so the field is emptied
    // first (it starts at the kind's folder).
    std::fs::create_dir_all(package_folder(&mut shell).join("Beasts/Ash")).unwrap();
    // An editable combo (egui's `folder_field`): its chevron is inside the field's frame.
    let field = shell
        .bounds("id:library-new-folder")
        .expect("the Folder field");
    let chevron = shell
        .bounds("Suggestions for Folder")
        .expect("the Folder's chevron");
    assert!(
        field.contains(&chevron.center()) && chevron.right() <= field.right(),
        "the chevron {chevron:?} is inside the Folder field {field:?}"
    );
    shell.click("id:library-new-folder");
    shell.press("ctrl-a backspace");
    shell.click("Suggestions for Folder");
    assert!(
        shell.bounds("Beasts/Ash").is_some(),
        "a folder made outside Splat is suggested"
    );
    shell.press("escape");
    shell.click("id:library-new-folder");
    shell.type_text("ash");
    shell.click("Suggestions for Folder");
    assert!(
        shell.bounds("Beasts/Ash").is_some()
            && shell
                .bounds("id:library-new-folder-menu-Creatures")
                .is_none(),
        "the field's words choose the folders shown"
    );
    shell.capture("library-author__10b-folder-suggestions__1200x800");
    shell.click("Beasts/Ash");
    assert_eq!(form(&mut shell).1, "Beasts/Ash");
    shell.click("Suggestions for Folder");
    shell.click("Main folder");
    assert_eq!(form(&mut shell).1, "");
    shell.click("Suggestions for Folder");
    shell.click("Creatures");
    shell.capture("library-author__10-new-entry-form__1200x800");
    shell.click("Create entry");
    loaded(&mut shell);
    assert_eq!(
        shell.read(|root| root.app.library.open_name().map(str::to_owned)),
        Some("Cinder Hound".to_owned())
    );
    assert!(shell.read(|root| root.app.library.source_open()));
    shell.capture("library-author__11-new-entry-created__1200x800");
}

const DRAFT_WORDS: &str = "Rangers of the ash line are exempt. ";
const SECOND_WORDS: &str = "Rangers carry spare cloth. ";
const THIRD_WORDS: &str = "Cloth wears out in a week. ";

#[test]
fn library_author_small() {
    let mut shell = Shell::with_size("library-author", Seed::Campaign, size(px(960.0), px(640.0)));
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    shell.click("Ash Exposure · Rule");
    shell.capture("library-author__01-local-package__960x640");
    shell.click("Edit Markdown");
    shell.capture("library-author__02-edit-markdown__960x640");
}

/// The fixture package's folder.
fn package_folder(shell: &mut Shell) -> std::path::PathBuf {
    shell.read(|root| {
        root.app
            .campaigns
            .modules()
            .iter()
            .find(|module| module.name == "Ashen Marches")
            .map(|module| module.path.clone())
            .expect("the package")
    })
}

/// Reload from disk, from the Packages menu.
fn reload(shell: &mut Shell) {
    shell.click("Packages");
    shell.click("Reload from disk");
    loaded(shell);
}

#[test]
fn library_scope() {
    let mut shell = Shell::new("library-scope", Seed::Campaign);
    library(&mut shell);
    search(&mut shell, "fireball");
    // The built-in Fireball, by its row's whole name (egui's `click`): the campaign's own
    // Fireball's row starts with the same words.
    shell.click("Fireball · Spell · Level 3");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Fireball"));
    assert!(
        shell.bounds("Show file").is_none(),
        "the built-in Fireball is open, which has no file"
    );
    shell.click("id:library-scope");
    shell.capture("library-scope__01-scope-menu__1200x800");
    shell.click("Everything");
    assert_eq!(
        shell.read(|root| root.app.library.scope()),
        crate::library::Scope::All
    );
    // A scope change keeps the words and the open spell.
    assert_eq!(
        shell.read(|root| root.app.library.search_text().to_owned()),
        "fireball"
    );
    assert_eq!(open_name(&mut shell).as_deref(), Some("Fireball"));
    assert!(
        shell.bounds("Show file").is_none(),
        "the built-in Fireball stays open"
    );
    shell.capture("library-scope__02-scope-everything__1200x800");
    open_row(&mut shell, "Fireball · Spell · Level 3 · Campaign");
    assert!(
        shell.bounds("Show file").is_some(),
        "the campaign's Fireball is open"
    );
    assert!(
        shell.bounds("id:library-back").is_some(),
        "Back returns to the built-in Fireball"
    );
    shell.capture("library-scope__03-campaign-fireball__1200x800");
    scope(&mut shell, "Ashen Marches");
    shell.click("About Ashen Marches");
    assert_eq!(
        open_name(&mut shell),
        Some("Ashen Marches".to_owned()),
        "About opens the package’s manifest"
    );
    // The empty search names its scope, and Search Everything keeps the words.
    assert!(
        shell.bounds("id:library-search-everything").is_some(),
        "the package's empty search offers Search Everything: {:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("library-scope__04-package-about-title__1200x800");
    shell.click("id:library-search-everything");
    assert_eq!(
        shell.read(|root| root.app.library.scope()),
        crate::library::Scope::All
    );
    assert_eq!(
        shell.read(|root| root.app.library.search_text().to_owned()),
        "fireball"
    );
    shell.click("id:library-search-clear");
    scope(&mut shell, "Ashen Marches");
    shell.click("About Ashen Marches");
    // A section row ends About and lists its entries.
    shell.click("Creatures");
    assert_eq!(
        shell.read(|root| root.app.library.section()),
        crate::library::model::Section::Creatures
    );
    // The section's first entry opens in About's place.
    assert_ne!(
        shell.read(|root| root.app.library.open_name().map(str::to_owned)),
        Some("Ashen Marches".to_owned())
    );
}

#[test]
fn library_file_fault() {
    let mut shell = Shell::new("library-file-fault", Seed::Campaign);
    let path = package_folder(&mut shell).join("Creatures/Cinder Wolf.md");
    let text = std::fs::read(&path).unwrap();
    let mut not_text = text.clone();
    let at = text.windows(4).position(|w| w == b"Bite").unwrap();
    not_text[at] = 0xff;
    std::fs::write(&path, &not_text).unwrap();
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    search(&mut shell, "Cinder Wolf");
    let row = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with("Creatures/Cinder Wolf.md · "))
        .expect("the unreadable file's row, named by its path");
    shell.click(&row);
    // The reader names the fault, with Try again; Edit Markdown waits.
    assert!(
        shell.bounds("Try again").is_some(),
        "the fault offers Try again"
    );
    shell.click("Edit Markdown");
    assert!(
        !shell.read(|root| root.app.library.source_open()),
        "a file that cannot be read has no Markdown to edit"
    );
    shell.capture("library-file-fault__01-package-file-not-text__1200x800");
    // The file reads again outside Splat; Try again reads it.
    std::fs::write(&path, &text).unwrap();
    shell.click("Try again");
    loaded(&mut shell);
    assert!(shell.bounds("Try again").is_none(), "the entry reads again");
    assert_eq!(
        shell.read(|root| root.app.library.open_name().map(str::to_owned)),
        Some("Cinder Wolf".to_owned())
    );
}

#[test]
fn package_manifest_broken() {
    let mut shell = Shell::new("package-manifest-broken", Seed::Campaign);
    let manifest = package_folder(&mut shell).join("Module.md");
    let text = std::fs::read_to_string(&manifest).unwrap();
    std::fs::write(
        &manifest,
        text.replace("splat_kind: package", "splat_kind: [package"),
    )
    .unwrap();
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    search(&mut shell, "Cinder Wolf");
    open_row(&mut shell, "Cinder Wolf · ");
    // The header counts the file; its entries stay and open.
    assert!(shell.bounds("1 file needs fixing").is_some());
    assert_eq!(open_name(&mut shell).as_deref(), Some("Cinder Wolf"));
    shell.capture("package-manifest-broken__01-manifest-invalid-library-keeps-entries__1200x800");
    shell.click("1 file needs fixing");
    shell.capture("package-manifest-broken__01b-issues-menu__1200x800");
    shell.press("escape");
    // Fixed outside Splat: Reload from disk reads it, and the count goes.
    std::fs::write(&manifest, text).unwrap();
    reload(&mut shell);
    assert!(
        shell.bounds("1 file needs fixing").is_none(),
        "the fault ends with the reload"
    );
}

/// Scenario `library-to-encounter`: an encounter draft open, a creature copied from the Library
/// into the campaign, and the copy added to the draft.
#[test]
fn library_to_encounter_copy() {
    let mut shell = Shell::new("library-to-encounter", Seed::Campaign);
    shell.update(|root, _, cx| root.show_page(Page::Encounters, cx));
    for _ in 0..10 {
        if !shell.read(|root| root.app.encounters.busy()) {
            break;
        }
        shell.update(|root, _, _| root.app.encounters.wait_for_load());
    }
    shell.click("New encounter");
    shell.type_text("Deep Ford");
    assert_eq!(
        shell.read(|root| root.app.encounters.draft_name()),
        Some("Deep Ford".to_owned())
    );
    shell.capture("library-to-encounter__01-draft-open__1200x800");
    library(&mut shell);
    search(&mut shell, "aboleth");
    open_row(&mut shell, "Aboleth · ");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Aboleth"));
    shell.capture("library-to-encounter__02-library-entry__1200x800");
    shell.click("Copy to…");
    assert!(
        shell.read(|root| root.app.copy_dialog.is_some()),
        "Copy to… opens the dialog"
    );
    // The catalogue loads on a thread.
    for _ in 0..100 {
        if shell.read(|root| {
            root.app
                .copy_dialog
                .as_ref()
                .is_some_and(|d| d.plan().is_some())
        }) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    // The dialog opens with the copy's name selected: typing replaces it.
    shell.type_text("Deep Aboleth");
    assert_eq!(
        shell.read(|root| root
            .app
            .copy_dialog
            .as_ref()
            .map(|dialog| dialog.name().to_owned())),
        Some("Deep Aboleth".to_owned()),
        "the name field holds the keyboard, its words selected"
    );
    shell.press("ctrl-a");
    shell.type_text("Aboleth");
    shell.click("Create copy");
    assert!(
        shell.read(|root| root
            .app
            .copy_dialog
            .as_ref()
            .is_some_and(|dialog| dialog.completed().is_some())),
        "Create copy writes the copy"
    );
    // The Library reads the copy in; the list behind the dialog shows both Aboleths.
    loaded(&mut shell);
    assert!(
        shell.bounds("Add to Deep Ford").is_some(),
        "the copy can go to the open draft: {:?}",
        crate::gui::kit::probe::names()
    );
    shell.capture("library-to-encounter__03-copied__1200x800");
    let before = shell.read(|root| {
        root.app
            .encounters
            .roster_names()
            .map_or(0, |names| names.len())
    });
    shell.click("Add to Deep Ford");
    assert!(shell.read(|root| root.app.copy_dialog.is_none()));
    shell.update(|root, _, cx| root.show_page(Page::Encounters, cx));
    shell.settle();
    assert_eq!(
        shell.read(|root| root
            .app
            .encounters
            .roster_names()
            .map_or(0, |names| names.len())),
        before + 1,
        "the copy joins the draft"
    );
    shell.capture("library-to-encounter__04-copy-in-open-draft-results__1200x800");
}

#[test]
fn remove_package_in_use() {
    let mut shell = Shell::new("remove-package-in-use", Seed::Campaign);
    let more = shell.read(|root| {
        format!(
            "id:library-package-more-{}",
            root.app.campaigns.modules()[0].id
        )
    });
    library(&mut shell);
    shell.click("Packages");
    shell.click("Manage packages…");
    shell.click(&more);
    let reads = || crate::library::model::UNREGISTER_READS.with(std::cell::Cell::get);
    let before = reads();
    shell.click("Unregister…");
    assert!(shell.read(|root| root.app.library.unregister_open()));
    // The packages are read once as the dialog opens, not on each frame it shows.
    shell.frame_cost(5);
    assert_eq!(reads() - before, 1, "the dialog read the packages once");
    shell.capture("remove-package-in-use__01-package-in-use-warning__1200x800");
    // Escape keeps the package.
    shell.press("escape");
    assert!(!shell.read(|root| root.app.library.unregister_open()));
    shell.click(&more);
    shell.click("Unregister…");
    shell.click("id:library-unregister-confirm");
    loaded(&mut shell);
    assert!(shell.read(|root| root.app.campaigns.modules().is_empty()));
    shell.capture("remove-package-in-use__02-package-unregistered-with-undo__1200x800");
    shell.click("Undo unregister Ashen Marches");
    loaded(&mut shell);
    assert_eq!(
        shell.read(|root| root.app.campaigns.modules().len()),
        1,
        "Undo registers it again"
    );
}

/// Up and Down move the keyboard from row to row of the entry list, Enter
/// opens the row that holds it, and Alt+Left and Alt+Right go Back and Forward between the
/// entries opened.
#[test]
fn library_keys() {
    let mut shell = Shell::new("library-keys", Seed::Campaign);
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    let name = |shell: &mut Shell, row: usize| {
        shell
            .read(move |root| root.app.library.entry_row(row, None).map(|row| row.name))
            .expect("the row")
    };
    let open =
        |shell: &mut Shell| shell.read(|root| root.app.library.open_name().map(str::to_owned));
    let (first, second) = (name(&mut shell, 0), name(&mut shell, 1));
    // No entry open: the first Down starts at the first row, the second goes on to the next.
    shell.press("down");
    shell.press("down");
    shell.press("enter");
    assert_eq!(
        open(&mut shell),
        Some(second.clone()),
        "Enter opens the row Down reached"
    );
    shell.capture("library-keys__01-down-and-enter__1200x800");
    shell.press("up");
    shell.press("enter");
    assert_eq!(open(&mut shell), Some(first.clone()), "Up goes back a row");
    // The list's opens leave a trail: Back returns, Forward goes again.
    shell.press("alt-left");
    assert_eq!(open(&mut shell), Some(second.clone()), "Alt+Left goes Back");
    shell.press("alt-right");
    assert_eq!(open(&mut shell), Some(first), "Alt+Right goes Forward");
}

/// The list heading's count groups its thousands (`design::count`): a search that matches over a thousand entries says `SEARCH RESULTS 1,234`.
#[test]
fn library_counts_group_thousands() {
    let mut shell = Shell::new("library-counts", Seed::Empty);
    library(&mut shell);
    shell.click("id:library-search");
    shell.type_text("e");
    let heading = shell.update(|root, _, _| {
        let app = &mut root.app;
        app.library
            .list_view(&app.campaigns, None, false, 900.0)
            .and_then(|list| list.heading)
    });
    let (title, count) = heading.expect("the search results heading");
    assert!(
        count >= 1000,
        "the search matches over a thousand entries: {count}"
    );
    let grouped = crate::design::count(count, "", "");
    assert!(grouped.contains(','), "{grouped}");
    assert!(
        shell.bounds(&format!("{title} {grouped}")).is_some(),
        "the heading's count {grouped} is shown grouped"
    );
    shell.capture("library-browse__03-search-results-count__1200x800");
}

/// A file changed on disk under a draft: the comparison aligns the lines both texts share (a
/// line only one side has stands beside an empty row), counts only the lines that differ, and
/// names the heading the change is under.
#[test]
fn library_comparison_matches_lines() {
    let mut shell = Shell::new("library-comparison", Seed::Campaign);
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    shell.click("Ash Exposure · Rule");
    shell.click("Edit Markdown");
    let file = package_folder(&mut shell).join("Rules/Ash Exposure.md");
    let text = std::fs::read_to_string(&file).unwrap();
    // The draft adds a line at the end; the file gains one under the title heading.
    shell.update(|root, _, _| {
        let words = format!("{}Draft line.\n", root.app.library.draft_text());
        root.app.library.edit_draft(&words);
    });
    let (head, body) = text.split_once("\n# ").expect("a heading");
    let (heading, rest) = body.split_once('\n').expect("a body under the heading");
    std::fs::write(
        &file,
        format!("{head}\n# {heading}\nA line on disk.\n{rest}"),
    )
    .unwrap();
    shell.click("id:library-save");
    assert!(shell.read(|root| root.app.library.disk_changed()));
    let lines = shell
        .read(|root| root.pages.library.comparison.clone())
        .expect("the comparison")
        .1;
    assert_eq!(
        lines.changed, 2,
        "one line each side, not every line after the first change"
    );
    assert!(
        lines.rows.iter().any(|row| row[0].is_none())
            && lines.rows.iter().any(|row| row[1].is_none()),
        "a line only one side has stands beside an empty row"
    );
    let summary = lines.summary(None);
    assert!(
        summary.starts_with("2 lines differ · Under the heading"),
        "the summary counts the lines that differ and names their heading: {summary}"
    );
    assert!(shell.bounds(&summary).is_some(), "the summary is shown");
    shell.capture("library-comparison__01-aligned__1200x800");
}

/// A note whose properties do not parse: `Fix in Edit Markdown` opens the editor, which takes
/// the keyboard with the caret at the start of the broken line (`focus_repair`).
#[test]
fn library_repair_puts_the_caret_on_the_line() {
    let mut shell = Shell::new("library-repair-caret", Seed::Campaign);
    const BROKEN: &str = "---\nsplat_schema: 2\nsplat_id: [\n---\n# Broken\n";
    std::fs::write(package_folder(&mut shell).join("Rules/Broken.md"), BROKEN).unwrap();
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    let row = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with("Rules/Broken.md · ") || name.starts_with("Broken.md · "))
        .expect("the broken note's row");
    shell.click(&row);
    shell.click("Fix in Edit Markdown");
    assert!(shell.read(|root| root.app.library.source_open()));
    let (focused, selection, line) = shell.update(|root, window, cx| {
        let (_, handle) = root
            .pages
            .library
            .editor
            .clone()
            .expect("the editor's state");
        let app = &mut root.app;
        let subtitle = app.library.open_subtitle();
        let line = app
            .library
            .reader_view(&app.campaigns, None, &subtitle)
            .and_then(|(view, _)| view.repair_line);
        (handle.focused(window, cx), handle.selection(cx), line)
    });
    let line = line.expect("the broken line");
    let at: usize = BROKEN
        .lines()
        .take(line - 1)
        .map(|line| line.len() + 1)
        .sum();
    assert!(focused, "the editor takes the keyboard");
    assert_eq!(selection, at..at, "the caret starts line {line}");
    shell.capture("library-repair-caret__01-caret-on-broken-line__1200x800");
}

/// Search the Library for `name` and open the found entry whose row starts with it.
fn open_found(shell: &mut Shell, name: &str) {
    shell.click("id:library-search");
    shell.press("ctrl-a");
    shell.type_text(name);
    let row = crate::gui::kit::probe::names()
        .into_iter()
        .find(|row| row.starts_with(&format!("{name} · ")))
        .unwrap_or_else(|| panic!("no row for {name}"));
    shell.click(&row);
    assert_eq!(
        shell.read(|root| root.app.library.open_name().map(str::to_owned)),
        Some(name.to_owned())
    );
}

/// The Library's Read is the entry reader: a campaign
/// creature's stat block with its rolling ability table and its action tile, a flagged HP cell,
/// a flagged spell cell and an item's facts (scenario `stat-formats`).
#[test]
fn stat_formats() {
    let mut shell = Shell::with_size("stat-formats", Seed::Formats, size(px(960.0), px(640.0)));
    library(&mut shell);
    open_found(&mut shell, "Slate Wolf");
    shell.capture("stat-formats__01-schema-2-creature-library__960x640");
    let mut shell = Shell::new("stat-formats", Seed::Formats);
    library(&mut shell);
    open_found(&mut shell, "Slate Wolf");
    // The stat block: Dexterity's modifier rolls its check, Bite is a tile, and the closed end
    // sections follow.
    let dexterity = splat::content::dice_text::RollLink::d20(
        splat::content::dice_text::D20::Check,
        2,
        "Dexterity",
    );
    let modifier = format!("{} +2", crate::rolls::link_label(&dexterity));
    assert!(
        shell.bounds(&modifier).is_some(),
        "the ability table's modifier rolls: {:?}",
        crate::gui::kit::probe::names()
    );
    assert!(shell.bounds("Bite").is_some(), "Bite is an action tile");
    assert!(shell.bounds("Source and identifiers").is_some());
    shell.capture("stat-formats__01-schema-2-creature-library__1200x800");
    // A press on the modifier rolls it for this creature, as the Reader's dice do.
    shell.click(&modifier);
    assert!(
        shell
            .read(|root| crate::rolls::plain::preview_words(&root.ctx, "Slate Wolf", &[&dexterity]))
            .is_some(),
        "the check was rolled for Slate Wolf"
    );
    open_found(&mut shell, "Flagged Wolf");
    shell.capture("stat-formats__02-flagged-hp-cell__1200x800");
    open_found(&mut shell, "Flagged Spark");
    shell.capture("stat-formats__03-flagged-spell-cell__1200x800");
    open_found(&mut shell, "Bound Charm");
    shell.capture("stat-formats__04-item-facts__1200x800");
}

/// Turn the mouse wheel by `dy` over the target `name` (a row a scrolled menu hides).
fn scroll_over(shell: &mut Shell, name: &str, dy: gpui::Pixels) {
    let at = shell
        .bounds(name)
        .unwrap_or_else(|| panic!("No target named {name:?}"))
        .center();
    shell.scroll(
        at,
        gpui::ScrollDelta::Pixels(gpui::point(px(0.0), dy)),
        gpui::Modifiers::default(),
    );
}

/// The heading over the list (`Matches`; none over a section on a wide page) and how many rows
/// the list has, as it draws them now.
fn list_heading(shell: &mut Shell) -> (Option<&'static str>, usize) {
    shell.update(|root, _, _| {
        let app = &mut root.app;
        let list = app
            .library
            .list_view(&app.campaigns, None, false, 1200.0)
            .expect("the Library is loaded");
        (list.heading.map(|(title, _)| title), list.rows)
    })
}

/// The creature filter: the Filter menu's facts open their values, each
/// chosen value is a chip that removes it, the challenge's lowest and highest rows, and Clear
/// filters; the list shows only the creatures that match.
#[test]
fn creature_filter() {
    let mut shell = Shell::new("creature-filter", Seed::Empty);
    library(&mut shell);
    shell.click("Creatures");
    assert_eq!(list_heading(&mut shell), (None, 336));
    shell.click("Filter creatures");
    shell.click("Habitat");
    // Its values run past the menu's height: the menu scrolls to Underdark.
    scroll_over(&mut shell, "Filter Habitat: Any", px(-400.0));
    shell.click("Filter Habitat: Underdark");
    // A fact's row counts its chosen values; a second press closes it.
    scroll_over(&mut shell, "Filter Habitat: Underdark", px(400.0));
    shell.click("Habitat (1)");
    assert!(shell.bounds("Filter creatures").is_some());
    assert!(shell.bounds("Filter Habitat: Underdark").is_none());
    shell.click("Type");
    shell.click("Filter Type: Aberration");
    shell.capture("creature-filter__00-menu-open__1200x800");
    // Filter again closes the menu.
    shell.click("Filter creatures");
    assert!(
        shell.bounds("Filter Habitat: Underdark").is_none(),
        "the menu closed"
    );
    assert!(shell.bounds("Remove filter Habitat: Underdark").is_some());
    assert!(shell.bounds("Remove filter Type: Aberration").is_some());
    assert!(shell.bounds("Clear filters").is_some(), "two values chosen");
    assert_eq!(list_heading(&mut shell), (Some("Matches"), 9));
    assert!(shell.bounds("Aboleth · Creature · CR 10").is_some());
    shell.capture("creature-filter__01-underdark-aberrations__1200x800");

    // Removing the Type chip keeps the habitat; Clear filters goes away with one value left.
    shell.click("Remove filter Type: Aberration");
    assert!(shell.bounds("Remove filter Type: Aberration").is_none());
    assert!(shell.bounds("Clear filters").is_none(), "one value chosen");
    let underdark = list_heading(&mut shell).1;
    assert!(
        underdark > 9,
        "more Underdark creatures than aberrations: {underdark}"
    );
    shell.click("Remove filter Habitat: Underdark");
    assert_eq!(list_heading(&mut shell), (None, 336));

    // The challenge: a lowest bound is a chip of its own; a second press removes it.
    shell.click("Filter creatures");
    shell.click("Challenge");
    shell.click("Lowest CR 17");
    assert!(shell.bounds("Remove filter CR 17 or more").is_some());
    shell.click("Lowest CR 17");
    assert!(shell.bounds("Remove filter CR 17 or more").is_none());
    shell.click("Lowest CR 17");
    shell.click("Highest CR 20");
    assert!(shell.bounds("Remove filter CR 17 to 20").is_some());
    let ranged = list_heading(&mut shell);
    assert_eq!(ranged.0, Some("Matches"));
    assert!(ranged.1 > 0 && ranged.1 < 336, "CR 17 to 20: {ranged:?}");
    // A type with the range makes two.
    shell.click("Challenge (1)");
    shell.click("Type");
    shell.click("Filter Type: Dragon");
    shell.click("Filter creatures");
    assert!(shell.bounds("Remove filter Type: Dragon").is_some());
    shell.click("Clear filters");
    assert!(shell.read(|root| root.app.library.filter().is_empty()));
    assert_eq!(list_heading(&mut shell), (None, 336));

    // Search terms find by facts; the bar stays under the search.
    shell.click("id:library-search");
    shell.type_text("immune:fire cr:17+");
    assert_eq!(list_heading(&mut shell), (Some("Search results"), 8));
    assert!(shell.bounds("Filter creatures").is_some());
    shell.capture("creature-filter__02-search-terms__1200x800");
}

/// Scenario `reader-tables`, step 01: a package rule whose table is wider than the reader, read
/// in the Library at each size. Where it does not fit, the table scrolls sideways and says so.
#[test]
fn reader_tables_in_library() {
    const TABLE: &str = "| Level | Proficiency Bonus | Features | Cantrips | Prepared Spells | 1st | 2nd | 3rd | 4th | 5th | 6th | 7th | 8th | 9th |\n\
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n\
| 5 | +3 | Memorize Spell | 4 | 9 | 4 | 3 | 2 | 0 | 0 | 0 | 0 | 0 | Ninth-slot |\n\n\
The table leaves this note readable.\n\n\
Read the row for your class level. A slot is spent when you cast a spell of that level or higher, and every slot comes back when you finish a Long Rest, so plan the hardest fights of the day around the slots you still have.\n";
    for (w, h, scrolls) in [
        (1200.0, 800.0, true),
        (960.0, 640.0, true),
        (1920.0, 1080.0, false),
    ] {
        let mut shell = Shell::with_size("reader-tables", Seed::Campaign, size(px(w), px(h)));
        let folder = package_folder(&mut shell);
        let note = splat::content::formats::fixture(
            "Rule.md",
            &format!(
                "splat_id: {}\nsplat_kind: rule\nsplat_key: ashen_marches:rule/spell_progression\nsplat_name: Spell Progression\nsplat_rule: {{}}\n",
                splat::character::stable_id("spell-progression")
            ),
            &format!("\n# Spell Progression\n\n{TABLE}"),
        )
        .unwrap();
        std::fs::write(folder.join("Rules/Spell Progression.md"), note).unwrap();
        library(&mut shell);
        scope(&mut shell, "Ashen Marches");
        open_row(&mut shell, "Spell Progression · ");
        assert_eq!(open_name(&mut shell).as_deref(), Some("Spell Progression"));
        let name = format!("reader-tables__01-wide-table-in-library__{w}x{h}");
        assert_eq!(
            shell
                .bounds("More columns: scroll sideways or use Shift+wheel.")
                .is_some(),
            scrolls,
            "{name}: a table scrolls sideways only where it does not fit"
        );
        shell.capture(&name);
    }
}

/// Print the harness names that start with `start` (for a failed step's message).
fn names_starting(start: &str) -> Vec<String> {
    crate::gui::kit::probe::names()
        .into_iter()
        .filter(|name| name.starts_with(start))
        .collect()
}

/// Scenario `package-about`: the package's About page with its choice, Choose all and its Undo.
#[test]
fn package_about() {
    let mut shell = Shell::new("package-about", Seed::Campaign);
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    shell.click("About Ashen Marches");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Ashen Marches"));
    shell.capture("package-about__01-package-about__1200x800");
    shell.click("Choose all");
    loaded(&mut shell);
    assert!(
        shell.bounds("Undo package choice").is_some(),
        "Choose all offers its Undo: {:?}",
        names_starting("Undo")
    );
    shell.capture("package-about__02-package-every-entry-used__1200x800");
    shell.click("Undo package choice");
    loaded(&mut shell);
    assert!(shell.bounds("Undo package choice").is_none());
}

/// Scenario `package-rename`, steps 01 to 03: an entry's reading links a spell; the spell is
/// renamed outside Splat; its heading no longer says its name, and Use fixes it with an Undo.
#[test]
fn package_rename() {
    let mut shell = Shell::new("package-rename", Seed::Campaign);
    let package = package_folder(&mut shell);
    let (entries, _) = splat::content::read_directory(&package).unwrap();
    let target = entries.iter().find(|e| e.name == "Ashfall").unwrap();
    let path = package.join("Rules/Ash Exposure.md");
    let original = std::fs::read_to_string(&path).unwrap() + "\nSee [[Cinder Wolf]].\n";
    let reading = serde_yaml_ng::to_value(serde_json::json!({
        "title": "Ash Exposure",
        "summary": "The watch knows this spell.",
        "related": { target.id.to_string(): "Read the watch spell" }
    }))
    .unwrap();
    std::fs::write(
        &path,
        crate::content_workspace::patch_fields(&original, &[("splat_reading", Some(reading))])
            .unwrap(),
    )
    .unwrap();
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    open_row(&mut shell, "Ash Exposure · ");
    assert!(
        shell.bounds("Read the watch spell").is_some(),
        "the reading's link shows: {:?}",
        names_starting("Read")
    );
    shell.capture("package-rename__01-before-linked-entry-rename__1200x800");
    let old_path = package.join(&target.path);
    let renamed = crate::content_workspace::patch_fields(
        &std::fs::read_to_string(&old_path).unwrap(),
        &[("splat_name", Some("Watch Ashfall".into()))],
    )
    .unwrap();
    std::fs::write(package.join("Spells/Watch Ashfall.md"), renamed).unwrap();
    std::fs::remove_file(old_path).unwrap();
    reload(&mut shell);
    shell.click("Read the watch spell");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Watch Ashfall"));
    assert!(
        shell.bounds("Use Watch Ashfall").is_some(),
        "the heading's fix shows: {:?}",
        names_starting("Use")
    );
    shell.capture("package-rename__02-linked-entry-renamed-on-disk__1200x800");
    shell.click("Use Watch Ashfall");
    loaded(&mut shell);
    assert!(
        shell.bounds("Undo heading change").is_some(),
        "{:?}",
        names_starting("Undo")
    );
    assert!(
        std::fs::read_to_string(package.join("Spells/Watch Ashfall.md"))
            .unwrap()
            .contains("# Watch Ashfall"),
        "the heading says the new name"
    );
    shell.capture("package-rename__03-renamed-entry-heading-fixed__1200x800");
}

/// Scenario `package-delete-entry`, steps 01 and 02: an entry that links a package creature; the
/// creature's file is deleted outside Splat, and the encounter that uses it says so.
#[test]
fn package_delete_entry() {
    let mut shell = Shell::new("package-delete-entry", Seed::Campaign);
    let package = package_folder(&mut shell);
    let rules = package.join("Rules/Ash Exposure.md");
    let text = std::fs::read_to_string(&rules).unwrap();
    std::fs::write(&rules, format!("{text}\n\n[[Cinder Wolf]]\n")).unwrap();
    library(&mut shell);
    search(&mut shell, "Ash Exposure");
    open_row(&mut shell, "Ash Exposure · ");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Ash Exposure"));
    shell.capture("package-delete-entry__01-package-entry-before-delete__1200x800");
    std::fs::remove_file(package.join("Creatures/Cinder Wolf.md")).unwrap();
    // The DM deleted the file in a file manager and comes back: the window's focus starts the
    // files' check (egui's `focus_window(false)`, `focus_window(true)`).
    shell.focus_window_again();
    shell.click("id:nav-Encounters");
    for _ in 0..100 {
        if shell.read(|root| {
            !root.app.lookup.busy() && root.app.encounters.loaded() && !root.app.encounters.busy()
        }) {
            break;
        }
        shell.update(|root, _, _| {
            root.app.lookup.wait_for_load();
            root.app.encounters.wait_for_load();
        });
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    // Bridge Ambush is the paused run: its row's Edit (egui: the Edit beside "Bridge Ambush")
    // opens the encounter in its editor, where the deleted creature's rows say so.
    let edit = crate::gui::kit::probe::names()
        .into_iter()
        .find(|name| name.starts_with("id:edit:") && name.contains("Encounter Runs"))
        .expect("the paused run's Edit");
    shell.click(&edit);
    assert!(shell.read(|root| root.app.encounters.editor_open()));
    let (rows, difficulty) = shell.update(|root, _, _| {
        let ctx = root.ctx.clone();
        let roster = root
            .app
            .encounters
            .roster_view(&ctx, &root.app.party, true)
            .expect("the roster");
        let rows: Vec<_> = roster
            .rows
            .iter()
            .filter(|row| row.name.contains("Cinder Wolf"))
            .map(|row| (row.missing.is_some(), row.edit_word))
            .collect();
        let difficulty: Vec<_> = roster.difficulty.into_iter().map(|(w, _)| w).collect();
        (rows, difficulty.join(" "))
    });
    assert_eq!(
        rows,
        vec![(true, "Replace…"); 2],
        "both Cinder Wolf rows say the creature is missing and offer Replace…"
    );
    assert!(
        difficulty.contains("XP not known: Cinder Wolf"),
        "the roster's difficulty names the deleted creature: {difficulty}"
    );
    shell.capture("package-delete-entry__02-deleted-package-entry-in-encounter__1200x800");
}

/// Scenario `package-duplicate-id`, steps 01 and 02: a copied creature file keeps its original's
/// identity; both rows say so, and the original opens in the editor.
#[test]
fn package_duplicate_id() {
    let mut shell = Shell::new("package-duplicate-id", Seed::Campaign);
    let package = package_folder(&mut shell);
    let before = std::fs::read_to_string(package.join("Creatures/Cinder Wolf.md")).unwrap();
    let copy = before
        .replace("splat_name: Cinder Wolf", "splat_name: Cinder Wolf copy")
        .replace("max_hp: {value: 22}", "max_hp: {value: 44}")
        .replace("HP 22 (", "HP 44 (")
        .replace("| Hit Points | 22 (", "| Hit Points | 44 (");
    std::fs::write(package.join("Creatures/Cinder Wolf copy.md"), copy).unwrap();
    library(&mut shell);
    search(&mut shell, "Cinder Wolf");
    let rows = names_starting("Cinder Wolf");
    assert!(
        rows.iter()
            .filter(|row| row.contains("Same entry in two files"))
            .count()
            >= 2,
        "both rows name the duplicate: {rows:?}"
    );
    shell.capture("package-duplicate-id__01-duplicate-package-library-list__1200x800");
    open_row(&mut shell, "Cinder Wolf · ");
    shell.click("Edit Markdown");
    assert!(shell.read(|root| root.app.library.source_open()));
    shell.capture("package-duplicate-id__02-duplicate-original-editor__1200x800");
}

/// The open campaign's folder.
fn campaign_folder(shell: &mut Shell) -> std::path::PathBuf {
    shell.read(|root| {
        root.app
            .campaigns
            .active_campaign()
            .expect("a campaign")
            .path
    })
}

/// Scenario `package-file-changed`, step 01: a package creature read before its file changes.
#[test]
fn package_file_changed() {
    let mut shell = Shell::new("package-file-changed", Seed::Campaign);
    library(&mut shell);
    search(&mut shell, "Cinder Wolf");
    open_row(&mut shell, "Cinder Wolf · ");
    assert_eq!(open_name(&mut shell).as_deref(), Some("Cinder Wolf"));
    shell.capture("package-file-changed__01-package-before-disk-edit__1200x800");
}

/// Scenario `library-navigation`, step 01: a package rule whose reading links a campaign note,
/// with a wiki link to a spell and one to a place that does not exist.
#[test]
fn library_navigation() {
    let mut shell = Shell::new("library-navigation", Seed::Campaign);
    let path = package_folder(&mut shell).join("Rules/Ash Exposure.md");
    let text = std::fs::read_to_string(&path).unwrap();
    let target_path = "Notes/Captain Varra.md";
    let target_text =
        std::fs::read_to_string(campaign_folder(&mut shell).join(target_path)).unwrap();
    let target = splat::content::Entry::parse(target_path, &target_text).unwrap();
    let reading = serde_yaml_ng::to_value(serde_json::json!({
        "title": "Ash Exposure",
        "summary": "Captain Varra knows the road through the ash fields.",
        "related": {target.id.to_string(): "Read Captain Varra"}
    }))
    .unwrap();
    let text =
        crate::content_workspace::patch_fields(&text, &[("splat_reading", Some(reading))]).unwrap();
    let text = format!("{text}\n\n[[Ashfall|Wiki watch spell]]\n\n[[Missing place]]\n");
    std::fs::write(&path, text).unwrap();
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    open_row(&mut shell, "Ash Exposure · ");
    assert!(
        shell.bounds("Read Captain Varra").is_some(),
        "the reading links the campaign note: {:?}",
        names_starting("Read")
    );
    shell.capture("library-navigation__01-homebrew-reference__1200x800");
}

/// Scenario `share-to-package`, step 01: a campaign rule that needs a campaign note, copied to
/// the package: the dialog copies both together.
#[test]
fn share_to_package() {
    let mut shell = Shell::new("share-to-package", Seed::Campaign);
    let root = campaign_folder(&mut shell);
    let (entries, _) = splat::content::read_directory(&root).unwrap();
    let varra = entries
        .iter()
        .find(|entry| entry.name == "Captain Varra")
        .unwrap();
    let text = crate::library::note_text("Watch Orders", "rule", None);
    let text = splat::content::formats::patch_model(
        &text,
        &[(
            "splat_requires",
            Some(
                serde_yaml_ng::to_value(vec![crate::content_workspace::EntryRef::id(varra.id)])
                    .unwrap(),
            ),
        )],
    )
    .unwrap();
    std::fs::write(root.join("Watch Orders.md"), text).unwrap();
    library(&mut shell);
    scope(&mut shell, "Campaign entries");
    search(&mut shell, "Watch Orders");
    open_row(&mut shell, "Watch Orders · ");
    shell.click("Copy to…");
    wait_for_copy_plan(&mut shell);
    shell.click("Destination");
    shell.click("Package · Ashen Marches");
    wait_for_copy_plan(&mut shell);
    assert!(
        shell.bounds("Copy both").is_some(),
        "the dependency is copied with it: {:?}",
        names_starting("C")
    );
    shell.capture("share-to-package__01-private-dependency-shared-together__1200x800");
}

/// Wait while the copy dialog works out its plan (on a thread).
fn wait_for_copy_plan(shell: &mut Shell) {
    for _ in 0..100 {
        if shell.read(|root| {
            root.app
                .copy_dialog
                .as_ref()
                .is_some_and(|d| d.plan().is_some())
        }) {
            break;
        }
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

/// Scenario `book-to-package`, step 01: a built-in creature copied into the package, named and
/// foldered.
#[test]
fn book_to_package() {
    let mut shell = Shell::new("book-to-package", Seed::Campaign);
    library(&mut shell);
    search(&mut shell, "Aboleth");
    open_row(&mut shell, "Aboleth · ");
    shell.click("Copy to…");
    wait_for_copy_plan(&mut shell);
    shell.click("Destination");
    shell.click("Package · Ashen Marches");
    wait_for_copy_plan(&mut shell);
    shell.click("Copy name");
    shell.press("ctrl-a");
    shell.type_text("Watch Aboleth");
    shell.click("Folder");
    shell.press("ctrl-a");
    shell.type_text("Creatures");
    wait_for_copy_plan(&mut shell);
    assert_eq!(
        shell.read(|root| root
            .app
            .copy_dialog
            .as_ref()
            .map(|dialog| (dialog.name().to_owned(), dialog.folder().to_owned()))),
        Some(("Watch Aboleth".to_owned(), "Creatures".to_owned()))
    );
    shell.capture("book-to-package__01-book-copy-to-package__1200x800");
}

/// Scenario `package-folder-copied`, step 01: a copy of the package's folder, offered as a new
/// package, is named as a copy and waits.
#[test]
fn package_folder_copied() {
    fn copy_folder(from: &std::path::Path, to: &std::path::Path) {
        std::fs::create_dir_all(to).unwrap();
        for file in std::fs::read_dir(from).unwrap() {
            let file = file.unwrap();
            let target = to.join(file.file_name());
            if file.path().is_dir() {
                copy_folder(&file.path(), &target);
            } else {
                std::fs::copy(file.path(), target).unwrap();
            }
        }
    }
    let mut shell = Shell::new("package-folder-copied", Seed::Campaign);
    let original = package_folder(&mut shell);
    let copied = original.with_file_name("ashen-marches-copy");
    copy_folder(&original, &copied);
    library(&mut shell);
    shell.click("Packages");
    shell.click("Manage packages…");
    shell.click("Package folder");
    shell.insert_text(&copied.to_string_lossy());
    shell.click("Add folder");
    assert_eq!(
        shell.read(|root| root.app.campaigns.modules().len()),
        1,
        "the copy waits"
    );
    shell.capture("package-folder-copied__01-copied-package-names-duplicate-identity__1200x800");
}

/// Scenario `package-to-table`, steps 01 and 02: a new package made from the Library while an
/// encounter draft is open, then a new entry in it.
#[test]
fn package_to_table() {
    let mut shell = Shell::new("package-to-table", Seed::Campaign);
    shell.click("id:nav-Encounters");
    for _ in 0..100 {
        if shell.read(|root| root.app.encounters.loaded() && !root.app.encounters.busy()) {
            break;
        }
        shell.update(|root, _, _| root.app.encounters.wait_for_load());
        shell.wait(std::time::Duration::from_millis(50));
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    shell.click("New encounter");
    assert!(shell.read(|root| root.app.encounters.editor_open()));
    library(&mut shell);
    shell.click("Packages");
    shell.click("Manage packages…");
    assert!(shell.read(|root| root.app.library.forms().0));
    shell.type_text("Copper Coast");
    shell.capture("package-to-table__01-create-package-location__1200x800");
    shell.click("Create package");
    loaded(&mut shell);
    assert!(
        shell.read(|root| root
            .app
            .campaigns
            .modules()
            .iter()
            .any(|module| module.name == "Copper Coast")),
        "the package is registered"
    );
    shell.type_text("Copper Drake");
    shell.capture("package-to-table__02-new-entry-used-by-default__1200x800");
}
