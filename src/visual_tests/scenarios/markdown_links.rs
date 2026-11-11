//! Links in a Markdown paragraph are each their own control, in World and in the Library, and
//! short links side by side keep their own press targets, in Problems and in prose.
use super::*;
use eframe::egui::accesskit::Role;
use std::fs;

const NOTE: &str = "Bell Tower";
const FIRST: &str = "the ferry master (opens Wren)";
const SECOND: &str = "the bell keeper (opens Oskar)";
/// Words that only the second link's note has.
const SECOND_TEXT: &str = "Oskar keeps the bell ropes dry.";
/// A note whose prose starts with two short links side by side.
const HANDS: &str = "Ferry Hands";
const JO: &str = "Jo (opens Jo)";
const LI: &str = "Li (opens Li)";

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

/// Write World notes into the campaign and wait for Lookup to read them.
fn write_notes<const N: usize>(d: &mut Driver<'_>, notes: [(String, &str, &str, &str); N]) {
    for (path, name, kind, body) in notes {
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
}

/// The one focus ring, on Jo at the start of the line: its inner edge is outside each of Jo's
/// glyphs, and the whole ring is inside the clip of the pane it is painted in.
fn ring_clear_of_letters(d: &mut Driver<'_>, size: (f32, f32)) -> (bool, String) {
    d.at_size(size, |d| {
        let rings = d.focus_rings();
        let mut glyphs = Vec::new();
        d.each_painted_text(|text, _| {
            if text.galley.job.text.contains("work the inner ferry") {
                let row = &text.galley.rows[0];
                for glyph in row.glyphs.iter().take(2) {
                    let left = text.pos.x + row.pos.x + glyph.pos.x;
                    glyphs.push((glyph.chr, left, left + glyph.advance_width));
                }
            }
        });
        let detail = format!("rings {rings:?}, glyphs {glyphs:?}");
        let [(ring, clip)] = rings[..] else {
            return (false, detail);
        };
        let inner = ring.shrink(crate::design::FOCUS_WIDTH);
        let off_letters = glyphs.len() == 2
            && glyphs.iter().map(|(chr, _, _)| *chr).eq(['J', 'o'])
            && glyphs.iter().all(|(_, left, right)| {
                *left >= inner.left() - 0.01 && *right <= inner.right() + 0.01
            });
        (off_letters && clip.contains_rect(ring), detail)
    })
}

/// Two links in view, at a window size: each target is at least XL square, the first ends
/// before the second begins, and they share a line.
fn pair_holds_xl(d: &mut Driver<'_>, first: &str, second: &str, size: (f32, f32)) -> bool {
    d.at_size(size, |d| {
        let nodes = d.nodes();
        let link = |name: &str| {
            nodes
                .iter()
                .find(|node| node.name == name && node.role == Role::Link && node.in_view())
                .map(|node| node.rect)
        };
        link(first).zip(link(second)).is_some_and(|(a, b)| {
            (a.center().y - b.center().y).abs() < 1.0
                && a.right() <= b.left()
                && [a, b].iter().all(|rect| {
                    rect.width() >= crate::design::XL && rect.height() >= crate::design::XL
                })
        })
    })
}

pub fn run(d: &mut Driver<'_>) {
    fs::create_dir_all(root(d).join("People")).unwrap();
    // A Splat note, so the Library lists it too, and two realistic two-letter names that sit
    // side by side in Problems.
    write_notes(
        d,
        [
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
        ],
    );

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
    let large = pair_holds_xl(d, "Jo", "Li", LARGE);
    d.observe(
        "iter-17-A-AD1-problems-1920",
        large,
        "At 1920, Jo and Li in Problems each have an XL target, and Jo's ends before Li's begins",
    );
    d.state("short-links-side-by-side", Sizes::All);

    // A note that links Jo and Li takes them out of Problems, so it comes after those checks.
    write_notes(
        d,
        [(
            format!("{HANDS}.md"),
            HANDS,
            "place",
            "[[Jo]][[Li]] work the inner ferry for [[Wren]].",
        )],
    );
    // AD1 in prose: two short links side by side each reserve an XL target in the paragraph.
    d.click_starting(&format!("{HANDS} · "));
    d.expect(JO);
    for (size, at) in [(SMALL, "960"), (LARGE, "1920")] {
        let held = pair_holds_xl(d, JO, LI, size);
        d.observe(
            &format!("iter-17-A-AD1-prose-{at}"),
            held,
            format!("At {at}, Jo and Li in the paragraph each have an XL target, and Jo's ends before Li's begins"),
        );
    }
    // The first link starts the line, and its focus ring stays inside the paragraph.
    let focused = d.tab_to(JO);
    d.observe(
        "iter-17-A-AD1-prose-tab",
        focused,
        "Tab reaches Jo, the first of the side-by-side links",
    );
    for (size, at) in [(SMALL, "960"), (LARGE, "1920")] {
        let (clear, detail) = ring_clear_of_letters(d, size);
        d.observe(
            &format!("iter-17-A-AD1-ring-{at}"),
            clear,
            format!(
                "At {at}, Jo's focus ring covers no letter and stays inside the reader: {detail}"
            ),
        );
    }
    d.state("prose-short-links-first-focused", Sizes::All);

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
