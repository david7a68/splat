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

#[test]
fn library_browse() {
    let mut shell = Shell::new("library-browse", Seed::Empty);
    library(&mut shell);
    assert_eq!(shell.read(|root| root.app.page), Page::Library);
    shell.capture("library-browse__01-first-launch__1200x800");
    shell.click("id:library-search");
    shell.type_text("dragon");
    shell.capture("library-browse__02-search-dragon__1200x800");
    assert_eq!(
        shell.read(|root| root.app.library.search_text().to_owned()),
        "dragon"
    );
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
    // Reload and lose my draft: the file wins, and Discard's Undo stands.
    shell.click("Reload and lose my draft");
    loaded(&mut shell);
    assert!(!shell.read(|root| root.app.library.disk_changed()));
    assert!(
        shell
            .read(|root| root.app.library.draft_text().to_owned())
            .contains("Edited elsewhere.")
    );
    shell.capture("library-author__07-library-draft-discarded__1200x800");
    assert!(
        shell.bounds("Undo discard of Ash Exposure").is_some(),
        "{:?}",
        crate::gui::kit::probe::names()
            .into_iter()
            .filter(|name| name.contains("Undo") || name.contains("footer"))
            .collect::<Vec<_>>()
    );
    shell.click("Undo discard of Ash Exposure");
    assert!(
        shell
            .read(|root| root.app.library.draft_text().to_owned())
            .contains(SECOND_WORDS.trim()),
        "Undo gives the draft back"
    );
    shell.click("Discard");
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

const DRAFT_WORDS: &str = " Rangers of the ash line are exempt.";
const SECOND_WORDS: &str = " Rangers carry spare cloth.";

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
    shell.click("id:library-scope");
    shell.capture("library-scope__01-scope-menu__1200x800");
    shell.click("Everything");
    assert_eq!(
        shell.read(|root| root.app.library.scope()),
        crate::library::Scope::All
    );
    shell.capture("library-scope__02-scope-everything__1200x800");
    scope(&mut shell, "Ashen Marches");
    shell.click("About Ashen Marches");
    assert_eq!(
        shell.read(|root| root.app.library.open_name().map(str::to_owned)),
        Some("Ashen Marches".to_owned()),
        "About opens the package’s manifest"
    );
    shell.capture("library-scope__04-package-about-title__1200x800");
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
    // The header counts the file; its entries stay.
    assert!(shell.bounds("1 file needs fixing").is_some());
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

#[test]
fn library_to_encounter_copy() {
    let mut shell = Shell::new("library-to-encounter", Seed::Campaign);
    library(&mut shell);
    scope(&mut shell, "Ashen Marches");
    shell.click("Ash Wraith · Creature · CR 1");
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
    shell.capture("library-to-encounter__02-library-entry__1200x800");
    shell.click("Create copy");
    assert!(
        shell.read(|root| root
            .app
            .copy_dialog
            .as_ref()
            .is_some_and(|dialog| dialog.completed().is_some())),
        "Create copy writes the copy"
    );
    shell.capture("library-to-encounter__03-copied__1200x800");
    shell.click("Close");
    assert!(shell.read(|root| root.app.copy_dialog.is_none()));
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
    shell.click("Unregister…");
    assert!(shell.read(|root| root.app.library.unregister_open()));
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
