//! Links in a Markdown paragraph are each their own control, in World and in the Library, and
//! short links side by side keep their own press targets.
use super::*;
use eframe::egui::accesskit::Role;
use std::fs;

const NOTE: &str = "Bell Tower";
const FIRST: &str = "the ferry master (opens Wren)";
const SECOND: &str = "the bell keeper (opens Oskar)";
/// Words that only the second link's note has.
const SECOND_TEXT: &str = "Oskar keeps the bell ropes dry.";

fn root(d: &Driver<'_>) -> std::path::PathBuf {
    d.root.join("campaigns/emberfall")
}

fn links_in_view(d: &Driver<'_>) -> Vec<String> {
    d.nodes()
        .into_iter()
        .filter(|node| node.role == Role::Link && node.in_view())
        .map(|node| node.name)
        .collect()
}

/// Tab reaches the first link, then the second, and Enter follows the second.
fn keyboard_follows_the_second_link(d: &mut Driver<'_>, place: &str, capture: &str) {
    let links = links_in_view(d);
    d.observe(
        &format!("iter-16-A-AD1-{place}-names"),
        links.iter().filter(|name| *name == FIRST).count() == 1
            && links.iter().filter(|name| *name == SECOND).count() == 1
            && !links.iter().any(|name| name.contains(" or ")),
        format!("{place}: each link in the paragraph has its own name: {links:?}"),
    );
    let first = d.tab_to(FIRST);
    d.press(Key::Tab);
    let second = d.focused_name();
    d.observe(
        &format!("iter-16-A-AD1-{place}-order"),
        first && second.as_deref() == Some(SECOND),
        format!("{place}: Tab reaches {FIRST:?} and then {second:?}"),
    );
    d.state(capture, Sizes::All);
    let before = d.visible_text(SECOND_TEXT);
    d.press(Key::Enter);
    d.settle();
    d.state(&format!("{capture}-followed"), Sizes::Default);
    d.observe(
        &format!("iter-16-A-AD1-{place}-enter"),
        !before && d.visible_text(SECOND_TEXT),
        format!("{place}: Enter on the second link opens Oskar, not Wren"),
    );
}

pub fn run(d: &mut Driver<'_>) {
    fs::create_dir_all(root(d).join("People")).unwrap();
    // A Splat note, so the Library lists it too, and two realistic two-letter names that sit
    // side by side in Problems.
    for (path, name, kind, body) in [
        (
            format!("{NOTE}.md"),
            NOTE,
            "place",
            "Ask [[Wren|the ferry master]] or [[Oskar|the bell keeper]] about the bells.",
        ),
        (
            "People/Wren.md".into(),
            "Wren",
            "npc",
            "Wren runs the ferry at dusk.",
        ),
        (
            "People/Oskar.md".into(),
            "Oskar",
            "npc",
            "Oskar keeps the bell ropes dry.",
        ),
        (
            "People/Jo.md".into(),
            "Jo",
            "npc",
            "A ferry hand on the inner sea.",
        ),
        (
            "People/Li.md".into(),
            "Li",
            "npc",
            "A ferry hand on the inner sea.",
        ),
    ] {
        let id = splat::character::stable_id(&path);
        let text = format!(
            "---\nsplat_schema: 1\nsplat_id: {id}\nsplat_kind: world\nsplat_name: {name}\n\
             splat_world:\n  type: {kind}\n---\n\n# {name}\n\n{body}\n"
        );
        let text = splat::content::formats::convert("World.md", &text).unwrap();
        fs::write(root(d).join(path), text).unwrap();
    }
    d.app_mut().lookup.refresh();
    d.settle();

    // SD1: two short links side by side never share a press target.
    d.click(NAV_WORLD);
    d.click_containing("Problems · ");
    let pair = |d: &Driver<'_>| {
        let nodes = d.nodes();
        let link = |name: &str| {
            nodes
                .iter()
                .find(|node| node.name == name && node.role == Role::Link && node.in_view())
                .map(|node| node.rect)
        };
        link("Jo").zip(link("Li"))
    };
    d.observe_both(
        "iter-16-A-SD1-side-by-side",
        "Jo and Li are links on one line, with no other link between them",
        |d| {
            pair(d).is_some_and(|(jo, li)| {
                let between = d.nodes().iter().any(|node| {
                    node.role == Role::Link
                        && node.in_view()
                        && node.rect.center().x > jo.center().x
                        && node.rect.center().x < li.center().x
                        && (node.rect.center().y - jo.center().y).abs() < 1.0
                });
                (jo.center().y - li.center().y).abs() < 1.0 && jo.left() < li.left() && !between
            })
        },
    );
    d.observe_both(
        "iter-16-A-SD1-no-overlap",
        "Jo's press target ends before Li's begins",
        |d| pair(d).is_some_and(|(jo, li)| jo.right() <= li.left()),
    );
    d.observe_both(
        "iter-16-A-SD1-size",
        "Jo and Li each have a press target at least XL wide and tall",
        |d| {
            pair(d).is_some_and(|(jo, li)| {
                [jo, li].iter().all(|rect| {
                    rect.width() >= crate::design::XL && rect.height() >= crate::design::XL
                })
            })
        },
    );
    d.state("short-links-side-by-side", Sizes::All);

    // AD1 in World.
    d.click_starting(&format!("{NOTE} · "));
    d.expect(FIRST);
    keyboard_follows_the_second_link(d, "world", "world-second-link-focused");

    // AD1 in the Library.
    go_library(d);
    search_library(d, NOTE);
    d.click_containing(&format!("{NOTE} · "));
    d.expect(FIRST);
    keyboard_follows_the_second_link(d, "library", "library-second-link-focused");
}
