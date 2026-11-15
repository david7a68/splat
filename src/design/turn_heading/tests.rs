//! The turn heading proves its rules here, once: R22's key release, one complete candidate
//! inside its room that never hides the name, and one choice per change.
use eframe::egui::{self, Rect, pos2, vec2};

use super::*;
use crate::design::{RUN_COMPACT, RUN_LARGE, RUN_LIST_KEEPS, RUN_REGULAR, theme};

type Nodes = Vec<(egui::accesskit::NodeId, egui::accesskit::Node)>;

/// A text the frame painted: its words, its bounds, its clip, and whether egui cut it.
struct Painted {
    text: String,
    rect: Rect,
    clip: Rect,
    elided: bool,
}

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    theme::install(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// One frame whose heading has exactly `width`, drawing `draw` there.
fn frame(
    ctx: &egui::Context,
    width: f32,
    mut draw: impl FnMut(&mut Ui) -> TurnPresses,
) -> (Nodes, Vec<Painted>) {
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            pos2(0.0, 0.0),
            vec2(width + 40.0, 400.0),
        )),
        ..Default::default()
    };
    let output = ctx.run(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui(vec2(width, 300.0), |ui| {
                ui.set_max_width(width);
                let _ = draw(ui);
            });
        });
    });
    let mut painted = Vec::new();
    fn walk(shape: &egui::Shape, clip: Rect, out: &mut Vec<Painted>) {
        match shape {
            egui::Shape::Text(text) => out.push(Painted {
                text: text.galley.job.text.clone(),
                rect: text.visual_bounding_rect(),
                clip,
                elided: text.galley.elided,
            }),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, clip, out)),
            _ => {}
        }
    }
    for clipped in &output.shapes {
        walk(&clipped.shape, clipped.clip_rect, &mut painted);
    }
    let nodes = output
        .platform_output
        .accesskit_update
        .map(|update| update.nodes)
        .unwrap_or_default();
    (nodes, painted)
}

fn bounds(nodes: &Nodes, name: &str) -> Rect {
    let found: Vec<_> = nodes
        .iter()
        .filter(|(_, node)| node.label() == Some(name))
        .collect();
    assert_eq!(found.len(), 1, "one node named {name:?}");
    let b = found[0].1.bounds().expect("bounds");
    Rect::from_min_max(
        pos2(b.x0 as f32, b.y0 as f32),
        pos2(b.x1 as f32, b.y1 as f32),
    )
}

const LONG: &str = "Seraphina Thornwood-Ashgrave of the Ninth Bell";
/// Longer than any numbered creature name of the visual scenarios, with a copy number.
const LONG_NUMBERED: &str = "Seraphina Thornwood-Ashgrave of the Ninth Bell 12";

fn whole(name: &String) -> TurnName<'_> {
    TurnName::whole(Some(name))
}

/// The part's own measures: `Next turn` with and without its letter, and the least name font
/// at `scale`.
fn measures(ctx: &egui::Context, scale: RunScale) -> (f32, f32, FontId) {
    let mut out = (0.0, 0.0, FontId::default());
    let _ = frame(ctx, 600.0, |ui| {
        out = (
            next_turn(Letter::Shown, None).natural_width(ui).ceil(),
            next_turn(Letter::Released, None).natural_width(ui).ceil(),
            Type::Name.sized(ui, (scale.name + HAIRLINE_WIDTH).min(scale.hp)),
        );
        TurnPresses::default()
    });
    out
}

/// R22 (root `run-heading-fit`) and the lead's ruling 1, from the run list's kept width
/// (454 px) up, at the three run scales, for a three-digit round:
/// - a shown `N` stands only beside a whole name; a cut name beside a keyless `Next turn` as
///   narrow as its keyless measure;
/// - a candidate always fits there, and the painted name is not cut by egui, is inside its
///   clip, left of `Add`, starts with a letter of the name and keeps its copy mark;
/// - a name is cut only where its whole form at the least size does not fit the name's room
///   (an exact measure, so a size estimate never skips a fit);
/// - some width shows the whole name only because the letter gave up its room, and some width
///   gains no whole word from the release and still paints the keyless control.
#[test]
fn the_key_letter_gives_up_its_room_before_the_name_is_cut() {
    let ctx = context();
    let mut released_whole = 0;
    let mut released_cut = 0;
    for scale in [RUN_COMPACT, RUN_REGULAR, RUN_LARGE] {
        let (keyed, keyless, least) = measures(&ctx, scale);
        for name in ["Iselle Marrowwind", LONG, LONG_NUMBERED].map(str::to_owned) {
            let numbered = name == LONG_NUMBERED;
            for keys in [TurnKeys::Work, TurnKeys::Elsewhere] {
                let draw = |ui: &mut Ui, kept: &mut KeptTurnHeading| {
                    TurnHeading::new("Ember Road", 123, whole(&name), scale, false, None, keys)
                        .show(ui, kept)
                };
                for step in 0..=60 {
                    let width = RUN_LIST_KEEPS + 8.0 * step as f32;
                    let mut kept = KeptTurnHeading::new();
                    let (nodes, painted) = frame(&ctx, width, |ui| draw(ui, &mut kept));
                    let at = format!("{width} px, {name:?}, {keys:?}, hp {}", scale.hp);
                    assert!(
                        kept.fits(),
                        "{at}: a candidate fits at the run list's width"
                    );
                    let (shown, letter) = kept.shown().expect("a kept heading");
                    let shown = shown.to_owned();
                    let is_whole = shown == name || shown == format!("{name}’s turn");
                    assert!(
                        !letter || (is_whole && keys == TurnKeys::Work),
                        "{at}: N shows beside {shown:?}"
                    );
                    let next = bounds(&nodes, "Next turn");
                    if !letter {
                        assert!(
                            (next.width() - keyless).abs() < 1.0,
                            "{at}: a keyless Next turn is {} wide, not {keyless}",
                            next.width()
                        );
                    }
                    let add = bounds(&nodes, "Add to fight");
                    let text = painted
                        .iter()
                        .find(|text| text.text == shown)
                        .unwrap_or_else(|| panic!("{at}: {shown:?} is painted"));
                    assert!(
                        !text.elided
                            && text.clip.expand(0.5).contains_rect(text.rect)
                            && text.rect.right() <= add.left(),
                        "{at}: {shown:?} at {:?} is whole, inside {:?} and left of Add {add:?}",
                        text.rect,
                        text.clip
                    );
                    assert!(
                        shown.chars().next().is_some_and(char::is_alphanumeric)
                            && (!numbered || shown.trim_end_matches("’s turn").ends_with(" 12")),
                        "{at}: the name keeps its first letter and its copy mark: {shown:?}"
                    );
                    if !is_whole {
                        // The name's room as painted: from after the round to before `Add`.
                        let round = painted
                            .iter()
                            .find(|text| text.text == "Round 123")
                            .expect("the round");
                        let room = add.left() - S - HAIRLINE_WIDTH - (round.rect.right() + S);
                        let mut full = 0.0;
                        let _ = frame(&ctx, 600.0, |ui| {
                            full = number_width(ui, &name, &least);
                            TurnPresses::default()
                        });
                        assert!(
                            full > room - 1.0,
                            "{at}: {shown:?} is cut, but the whole name ({full}) fits {room}"
                        );
                    }
                    if keys == TurnKeys::Work && !letter {
                        // The old fixed slot reserved the keyed width: the name had that much
                        // less room. Would it have shown the same words?
                        let mut old = KeptTurnHeading::new();
                        let _ = frame(&ctx, width - (keyed - keyless), |ui| draw(ui, &mut old));
                        let (before, _) = old.shown().expect("a kept heading");
                        if is_whole && before != shown {
                            released_whole += 1;
                        } else if !is_whole && before == shown {
                            released_cut += 1;
                        }
                    }
                }
            }
        }
    }
    assert!(
        released_whole > 0,
        "some width shows the whole name only because the key letter gave up its room"
    );
    assert!(
        released_cut > 0,
        "some width gains no whole word from the release and still paints the keyless control"
    );
}

/// Ruling 1 below the run list's width: where not even one letter and the copy mark fit, no
/// candidate fits; the part says so and paints that least form, never a bare `…`.
#[test]
fn too_narrow_a_heading_fails_with_its_least_form() {
    let ctx = context();
    let name = LONG_NUMBERED.to_owned();
    let mut kept = KeptTurnHeading::new();
    let _ = frame(&ctx, 200.0, |ui| {
        TurnHeading::new(
            "Ember Road",
            123,
            whole(&name),
            RUN_LARGE,
            false,
            None,
            TurnKeys::Work,
        )
        .show(ui, &mut kept)
    });
    assert!(!kept.fits());
    assert_eq!(kept.shown().map(|(words, _)| words), Some("S… 12"));
}

/// The kept result: one choice per change. An unchanged frame builds no string and measures
/// no candidate; a change of the width, the name, the keys, the round or the theme's fonts
/// (same `FontId`, new data: E1) chooses again. Two headings with their own kept results do not
/// share or reset each other's (consumer spike: two owned line instances).
#[test]
fn one_choice_per_change_and_two_owners_keep_their_own() {
    let ctx = context();
    let mut a = KeptTurnHeading::new();
    let mut b = KeptTurnHeading::new();
    let long = LONG.to_owned();
    let both = |ctx: &egui::Context,
                width: f32,
                a: &mut KeptTurnHeading,
                b: &mut KeptTurnHeading,
                name: &str,
                keys: TurnKeys,
                round: u64| {
        let name = name.to_owned();
        let _ = frame(ctx, width, |ui| {
            let _ = TurnHeading::new(
                "Ember Road",
                round,
                whole(&name),
                RUN_REGULAR,
                false,
                None,
                keys,
            )
            .show(ui, a);
            TurnHeading::new(
                "Road",
                1,
                whole(&long),
                RUN_REGULAR,
                false,
                None,
                TurnKeys::Work,
            )
            .show(ui, b)
        });
    };
    for _ in 0..3 {
        both(
            &ctx,
            900.0,
            &mut a,
            &mut b,
            "Iselle Marrowwind",
            TurnKeys::Work,
            2,
        );
    }
    assert_eq!(
        (a.scans(), b.scans()),
        (1, 1),
        "unchanged frames choose nothing"
    );
    both(
        &ctx,
        901.0,
        &mut a,
        &mut b,
        "Iselle Marrowwind",
        TurnKeys::Work,
        2,
    );
    assert_eq!((a.scans(), b.scans()), (2, 2), "a new width chooses again");
    both(
        &ctx,
        901.0,
        &mut a,
        &mut b,
        "Brannoch Vale",
        TurnKeys::Work,
        2,
    );
    assert_eq!(
        (a.scans(), b.scans()),
        (3, 2),
        "a new name is A's change only"
    );
    both(
        &ctx,
        901.0,
        &mut a,
        &mut b,
        "Brannoch Vale",
        TurnKeys::Elsewhere,
        2,
    );
    assert_eq!(a.scans(), 4, "Lookup chooses again");
    both(
        &ctx,
        901.0,
        &mut a,
        &mut b,
        "Brannoch Vale",
        TurnKeys::Elsewhere,
        3,
    );
    assert_eq!((a.scans(), b.scans()), (5, 2), "a new round chooses again");
    // E1: the theme installs its fonts again; every `FontId` is the same.
    theme::install(&ctx);
    both(
        &ctx,
        901.0,
        &mut a,
        &mut b,
        "Brannoch Vale",
        TurnKeys::Elsewhere,
        3,
    );
    assert_eq!(
        (a.scans(), b.scans()),
        (6, 3),
        "new font data chooses again"
    );
    both(
        &ctx,
        901.0,
        &mut a,
        &mut b,
        "Brannoch Vale",
        TurnKeys::Elsewhere,
        3,
    );
    assert_eq!((a.scans(), b.scans()), (6, 3), "and only once");
}

/// The selector alone (not evidence for the plan row, whose recipe is a later pull request):
/// an order whose later candidate is wider than an earlier one. It takes the first that fits,
/// measures each legal candidate at most once in order, and never skips ahead by width.
#[test]
fn the_selector_takes_the_first_fit_in_order() {
    const ORDER: [u32; 5] = [70, 60, 66, 40, 12];
    for room in [80, 65, 61, 45, 10] {
        let mut measured = Vec::new();
        let chosen = select::first_fit(
            &ORDER,
            |_| true,
            |width| {
                measured.push(width);
                (width <= room).then_some(())
            },
        );
        let first = ORDER.iter().position(|width| *width <= room);
        assert_eq!(
            chosen.map(|(width, ())| width),
            first.map(|at| ORDER[at]),
            "room {room}"
        );
        assert_eq!(
            measured,
            ORDER[..first.map_or(ORDER.len(), |at| at + 1)].to_vec(),
            "room {room}: each candidate measured once, in order"
        );
    }
    let skip = select::first_fit(
        &ORDER,
        |width| width != 60,
        |width| (width <= 65).then_some(()),
    );
    assert_eq!(skip.map(|(width, ())| width), Some(40));
}
