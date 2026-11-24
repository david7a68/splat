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
    assert!(shell.read(|root| root.app.library.dirty()), "typing makes a draft");
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
    assert!(std::fs::read_to_string(&file).unwrap().contains(DRAFT_WORDS.trim()));
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
    assert!(!shell.read(|root| root.app.library.forms().0), "Escape closes the form");
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
