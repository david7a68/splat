//! The turn heading proves its rules here, once: R22's key release, one complete candidate
//! inside its room, and one choice per change.
use eframe::egui::{self, Rect, pos2, vec2};

use super::*;
use crate::design::{RUN_REGULAR, theme};

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

/// One frame at `width`, drawing each heading on its own line with its own kept result.
fn frame(
    ctx: &egui::Context,
    width: f32,
    mut draw: impl FnMut(&mut Ui) -> TurnPresses,
) -> (Nodes, Vec<Painted>) {
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(width, 400.0))),
        ..Default::default()
    };
    let output = ctx.run(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = draw(ui);
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

fn heading(name: &str, keys: TurnKeys) -> TurnHeading<'_> {
    TurnHeading::new("Ember Road", 2, name, RUN_REGULAR, false, None, keys)
}

/// The keyless width of `Next turn`, as the part measures it.
fn keyless(ctx: &egui::Context) -> f32 {
    let mut width = 0.0;
    let _ = frame(ctx, 600.0, |ui| {
        width = next_turn(Letter::Released, None).natural_width(ui).ceil();
        TurnPresses::default()
    });
    width
}

/// R22 (root `run-heading-fit`): the hidden key letter gives up its room before the name is
/// cut. At every width: a shown `N` stands only beside a whole name; a cut name stands beside a
/// keyless `Next turn` as narrow as its keyless measure; the name's painted words are one
/// complete candidate, not cut by egui, inside its clip and left of `Add`. Some width chooses
/// the whole name only because the letter gave up its room (the old fixed slot cut it there),
/// and some width gains no whole word from the release and still paints the narrow control.
#[test]
fn the_key_letter_gives_up_its_room_before_the_name_is_cut() {
    let ctx = context();
    let keyless = keyless(&ctx);
    let names = ["Iselle Marrowwind", LONG];
    let mut released_whole = 0;
    let mut released_cut = 0;
    for name in names {
        for keys in [TurnKeys::Work, TurnKeys::Elsewhere] {
            for step in 0..=95 {
                let width = 380.0 + 10.0 * step as f32;
                let mut kept = KeptTurnHeading::default();
                let (nodes, painted) =
                    frame(&ctx, width, |ui| heading(name, keys).show(ui, &mut kept));
                let (shown, letter) = kept.shown().expect("a kept heading");
                let shown = shown.to_owned();
                let whole = shown == name || shown == format!("{name}’s turn");
                assert!(
                    !letter || (whole && keys == TurnKeys::Work),
                    "{width}: N shows beside {shown:?} ({keys:?})"
                );
                let next = bounds(&nodes, "Next turn");
                if !letter {
                    assert!(
                        (next.width() - keyless).abs() < 1.0,
                        "{width}: a keyless Next turn is {} wide, not {keyless}",
                        next.width()
                    );
                }
                let add = bounds(&nodes, "Add to fight");
                let text = painted
                    .iter()
                    .find(|text| text.text == shown)
                    .unwrap_or_else(|| panic!("{width}: {shown:?} is painted"));
                assert!(
                    !text.elided
                        && text.clip.expand(0.5).contains_rect(text.rect)
                        && text.rect.right() <= add.left(),
                    "{width}: {shown:?} at {:?} is whole, inside {:?} and left of Add {add:?}",
                    text.rect,
                    text.clip
                );
                assert!(
                    shown.trim_matches('…').chars().any(char::is_alphanumeric),
                    "{width}: the name paints a letter: {shown:?}"
                );
                if keys == TurnKeys::Work && !letter {
                    // Would the old fixed slot have kept the same words? It reserved the
                    // keyed width, so the name had that much less room.
                    let mut old = KeptTurnHeading::default();
                    let keyed = {
                        let mut width_with_key = 0.0;
                        let _ = frame(&ctx, 600.0, |ui| {
                            width_with_key =
                                next_turn(Letter::Shown, None).natural_width(ui).ceil();
                            TurnPresses::default()
                        });
                        width_with_key
                    };
                    let _ = frame(&ctx, width - (keyed - keyless), |ui| {
                        heading(name, keys).show(ui, &mut old)
                    });
                    let (before, _) = old.shown().expect("a kept heading");
                    if whole && before != shown {
                        released_whole += 1;
                    } else if !whole && before == shown {
                        released_cut += 1;
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

/// The kept result: one choice per change. An unchanged frame builds no string and measures
/// no candidate; a change of the width, the name, the keys or the round chooses again. Two
/// headings with their own kept results do not share or reset each other's (consumer spike:
/// two owned line instances).
#[test]
fn one_choice_per_change_and_two_owners_keep_their_own() {
    let ctx = context();
    let mut a = KeptTurnHeading::default();
    let mut b = KeptTurnHeading::default();
    let both = |ctx: &egui::Context,
                width: f32,
                a: &mut KeptTurnHeading,
                b: &mut KeptTurnHeading,
                name: &str,
                keys: TurnKeys,
                round: u64| {
        let _ = frame(ctx, width, |ui| {
            let _ = TurnHeading::new("Ember Road", round, name, RUN_REGULAR, false, None, keys)
                .show(ui, a);
            TurnHeading::new("Road", 1, LONG, RUN_REGULAR, false, None, TurnKeys::Work).show(ui, b)
        });
    };
    for _ in 0..3 {
        both(
            &ctx,
            960.0,
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
        961.0,
        &mut a,
        &mut b,
        "Iselle Marrowwind",
        TurnKeys::Work,
        2,
    );
    assert_eq!((a.scans(), b.scans()), (2, 2), "a new width chooses again");
    both(
        &ctx,
        961.0,
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
        961.0,
        &mut a,
        &mut b,
        "Brannoch Vale",
        TurnKeys::Elsewhere,
        2,
    );
    assert_eq!(a.scans(), 4, "Lookup chooses again");
    both(
        &ctx,
        961.0,
        &mut a,
        &mut b,
        "Brannoch Vale",
        TurnKeys::Elsewhere,
        3,
    );
    assert_eq!((a.scans(), b.scans()), (5, 2), "a new round chooses again");
    assert_eq!(b.shown().map(|(_, letter)| letter).is_some(), true);
}

/// Consumer spike: the plan row's mixed-XP candidates (ruling Q-s) through the same private
/// selector. A known Cinder Wolf beside an unknown Shade: dropping the Shade from the summary
/// adds its name to the missing-XP reason (R31), so a later candidate is wider than an earlier
/// one. The selector takes the first that fits, measures each legal candidate at most once in
/// order, and never skips ahead by width.
#[test]
fn consumer_spike_the_plan_rows_mixed_xp_candidates() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Plan {
        Whole,
        NoParty,
        NoShade,
        NoWolf,
        RatingOnly,
    }
    const PLAN: [Plan; 5] = [
        Plan::Whole,
        Plan::NoParty,
        Plan::NoShade,
        Plan::NoWolf,
        Plan::RatingOnly,
    ];
    let words = |plan: Plan| match plan {
        Plan::Whole => "2 Cinder Wolf, Shade · Hard, no XP for Shade · 4 players · Day 2",
        Plan::NoParty => "2 Cinder Wolf, Shade · Hard, no XP for Shade · Day 2",
        // R31: the Shade is no longer shown, so the reason names it in full.
        Plan::NoShade => "2 Cinder Wolf · Hard, no XP for Shade the Unbound · Day 2",
        Plan::NoWolf => "Shade · Hard, no XP for Shade · Day 2",
        Plan::RatingOnly => "Hard · Day 2",
    };
    for room in [70, 55, 52, 45, 10] {
        let mut measured = Vec::new();
        let chosen = select::first_fit(
            &PLAN,
            |_| true,
            |plan| {
                measured.push(plan);
                (words(plan).chars().count() <= room).then_some(words(plan))
            },
        );
        let first = PLAN
            .iter()
            .position(|plan| words(*plan).chars().count() <= room);
        assert_eq!(
            chosen.map(|(plan, _)| plan),
            first.map(|at| PLAN[at]),
            "room {room}"
        );
        assert_eq!(
            measured,
            PLAN[..first.map_or(PLAN.len(), |at| at + 1)].to_vec(),
            "room {room}: each candidate measured once, in order"
        );
    }
    // At 55 the wider NoShade does not fit, though the narrower NoWolf after it does.
    let chosen = select::first_fit(
        &PLAN,
        |_| true,
        |plan| (words(plan).chars().count() <= 55).then_some(()),
    );
    assert_eq!(chosen.map(|(plan, ())| plan), Some(Plan::NoParty));
    let skip_party = select::first_fit(
        &PLAN,
        |plan| plan != Plan::NoParty,
        |plan| (words(plan).chars().count() <= 55).then_some(()),
    );
    assert_eq!(skip_party.map(|(plan, ())| plan), Some(Plan::NoWolf));
}
