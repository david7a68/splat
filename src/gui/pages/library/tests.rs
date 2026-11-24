//! Screenshots and interaction tests of the Library page. Captures are named after the egui
//! scenarios' steps (`<scenario>__<step>__<w>x<h>`), so they pair with the references.
use crate::Page;
use crate::gui::harness::Shell;
use crate::visual_tests::fixture::Seed;
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
    // New entry: a creature in the package, opened in the editor.
    shell.click("id:library-new");
    shell.click("id:library-new-name");
    shell.type_text("Cinder Hound");
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
    // As in egui, the section's first entry opens in About's place.
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

/// The creature filter (egui `creature-filter`): the Filter menu's facts open their values, each
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
    shell.scroll("Filter Habitat: Any", px(-400.0));
    shell.click("Filter Habitat: Underdark");
    // A fact's row counts its chosen values; a second press closes it.
    shell.scroll("Filter Habitat: Underdark", px(400.0));
    shell.click("Habitat (1)");
    assert!(shell.bounds("Filter creatures").is_some());
    assert!(shell.bounds("Filter Habitat: Underdark").is_none());
    shell.click("Type");
    shell.click("Filter Type: Aberration");
    shell.capture("creature-filter__00-menu-open__1200x800");
    // Filter again closes the menu.
    shell.click("Filter creatures");
    assert!(shell.bounds("Filter Habitat: Underdark").is_none(), "the menu closed");
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
    assert!(underdark > 9, "more Underdark creatures than aberrations: {underdark}");
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
