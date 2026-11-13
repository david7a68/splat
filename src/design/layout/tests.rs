//! The layout parts prove their rules here, once. Callers do not repeat these proofs.
use eframe::egui::{self, Rect, pos2, vec2};

use super::*;
use crate::design::theme;

type Nodes = Vec<(egui::accesskit::NodeId, egui::accesskit::Node)>;

/// A text the frame painted: its words, where, the clip it was painted in, and whether egui cut
/// it to an ellipsis.
struct Painted {
    text: String,
    rect: Rect,
    clip: Rect,
    elided: bool,
}

/// One frame at `size` with `events`, drawing `add` in a central panel.
fn frame_with(
    ctx: &egui::Context,
    size: egui::Vec2,
    events: Vec<egui::Event>,
    mut add: impl FnMut(&mut Ui),
) -> (Nodes, Vec<Painted>) {
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), size)),
        events,
        ..Default::default()
    };
    let output = ctx.run(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| add(ui));
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
            egui::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, clip, out);
                }
            }
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

fn frame(ctx: &egui::Context, size: egui::Vec2, add: impl FnMut(&mut Ui)) -> Nodes {
    frame_with(ctx, size, Vec::new(), add).0
}

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    theme::install(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// The bounds of the one node with this accessible name.
fn bounds(nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)], name: &str) -> Rect {
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

/// The words are painted whole: one text with exactly these words, not cut to an ellipsis, and
/// inside the clip it was painted in.
fn painted_whole(painted: &[Painted], words: &str) -> bool {
    painted.iter().any(|text| {
        text.text == words && !text.elided && text.clip.expand(0.5).contains_rect(text.rect)
    })
}

/// F14: the content's right edge is the same with and without the scroll bar, also when the
/// caller's style has floating bars that take no width, or solid bars wider than the lane.
#[test]
fn scroll_column_keeps_the_lane_with_and_without_the_bar() {
    let ctx = context();
    let edge = |rows: usize, style: usize| {
        let mut right = 0.0;
        let mut given = 0.0;
        let mut overflows = false;
        let mut inner = 0.0;
        frame(&ctx, vec2(600.0, 300.0), |ui| {
            match style {
                1 => ui.spacing_mut().scroll = egui::style::ScrollStyle::floating(),
                2 => ui.spacing_mut().scroll.bar_width = 24.0,
                _ => {}
            }
            given = ui.max_rect().right();
            let output = ScrollColumn::new(("lane", rows, style))
                .max_height(200.0)
                .show(ui, |ui| {
                    for row in 0..rows {
                        ui.label(format!("Row {row}"));
                    }
                    right = ui.max_rect().right();
                });
            overflows = output.content_size.y > output.inner_rect.height();
            inner = output.inner_rect.right();
        });
        (right, given, overflows, inner)
    };
    assert!(lane_width() > 0.0);
    for style in [0, 1, 2] {
        // Two frames each, so the bar's state settles.
        edge(1, style);
        let (short, given, short_bar, _) = edge(1, style);
        edge(80, style);
        let (tall, _, tall_bar, inner) = edge(80, style);
        // The bar starts where the area's inner rect ends: the content is clear of it.
        assert!(
            tall <= inner + 0.01,
            "the bar does not cover the content (style {style})"
        );
        assert!(
            !short_bar && tall_bar,
            "one column has a bar and one has none"
        );
        assert_eq!(short, tall, "the content ends at one edge (style {style})");
        assert_eq!(short, given - lane_width(), "the lane is kept");
    }
}

/// F8: a value that changes in place keeps the next control still.
#[test]
fn slot_and_status_slot_keep_the_next_control_still() {
    const SAVES: Slot = Slot::new(&["0 of 3", "3 of 3 saves"], SlotType::Small);
    let ctx = context();
    let after = |value: &str, state: usize| {
        let nodes = frame(&ctx, vec2(600.0, 300.0), |ui| {
            ui.horizontal(|ui| {
                SAVES.show(ui, value, colors().text);
                Action::quiet("Success").show(ui);
            });
            ui.horizontal(|ui| {
                status_slot(ui, &[0, 1], state, |state| Status {
                    icon: "*",
                    word: ["On", "Not connected at all"][state],
                    color: colors().muted,
                });
                Action::quiet("Account").show(ui);
            });
        });
        (bounds(&nodes, "Success"), bounds(&nodes, "Account"))
    };
    let narrow = after("0 of 3", 0);
    let wide = after("3 of 3 saves", 1);
    assert_eq!(narrow, wide);
    // A value wider than every example is cut: it never pushes.
    assert_eq!(after("3 of 3 saves and many more words", 1), narrow);
}

/// The ruling's order, by measure, at each boundary: on the title line, else one row under the
/// title, else rows that wrap at whole actions. Every action once, in order; never a wrap while
/// one row fits; an action wider than the line has its own row.
#[test]
// One row of all three actions is the point of these cases.
#[allow(clippy::single_range_in_vec_init)]
fn title_placement_follows_the_ruling_at_each_boundary() {
    let widths = [60.0, 100.0, 80.0];
    let row = 60.0 + 100.0 + 80.0 + 2.0 * ACTION_GAP;
    let (title, lead) = (200.0, L - S);
    let place = |beside: f32, under: f32| title_placement(title, lead, &widths, beside, under);
    assert_eq!(place(title + lead + row, 1.0), TitlePlacement::OnTitle);
    assert_eq!(
        place(title + lead + row - 0.5, 1000.0),
        TitlePlacement::Under(vec![0..3])
    );
    assert_eq!(place(10.0, row), TitlePlacement::Under(vec![0..3]));
    assert_eq!(
        place(10.0, row - 0.5),
        TitlePlacement::Under(vec![0..2, 2..3])
    );
    assert_eq!(
        place(10.0, 100.0),
        TitlePlacement::Under(vec![0..1, 1..2, 2..3])
    );
    // Wider than the line: its own row, never dropped.
    assert_eq!(
        place(10.0, 70.0),
        TitlePlacement::Under(vec![0..1, 1..2, 2..3])
    );
    // No actions: nothing moves under the title.
    assert_eq!(
        title_placement(title, lead, &[], 10.0, 10.0),
        TitlePlacement::OnTitle
    );
}

/// F17: the actions follow the title when they fit, by measure, to half a point; else they move
/// together under it; in the narrowest pane they wrap at whole actions. Every word is painted
/// whole inside its clip at every width, the title's context included. The measure here does
/// not use the part's own: it adds the painted title, the context, the gaps and each
/// `Action`'s natural width.
#[test]
fn title_line_keeps_every_action_and_its_word_at_every_width() {
    const EDIT: Word = crate::word!("Edit");
    const LOG: Word = crate::word!("Add to Log");
    const START: Word = crate::word!("Start fight");
    const OPEN: Word = crate::word!("Open in Lookup");
    const WORDS: [&str; 4] = ["Edit", "Add to Log", "Start fight", "Open in Lookup"];
    let (title, kind) = ("Cinder Wolf", "Beast · CR 3");
    let ctx = context();
    let line = || {
        TitleLine::new(title)
            .role(TitleType::Section)
            .context(Some(kind))
            .action(TitleAction::quiet(EDIT).icon(crate::design::icons::EDIT))
            .action(TitleAction::quiet(LOG))
            .action(TitleAction::quiet(START))
            .action(TitleAction::secondary(OPEN))
    };
    let mut need = 0.0;
    let mut row = 0.0;
    frame(&ctx, vec2(1600.0, 300.0), |ui| {
        let text = |words: &str, font: Type| {
            let font = font.font(ui);
            ui.fonts_mut(|fonts| fonts.layout_no_wrap(words.to_owned(), font, colors().text))
                .size()
                .x
                .ceil()
        };
        let actions = [
            Action::quiet("Edit").icon(crate::design::icons::EDIT),
            Action::quiet("Add to Log"),
            Action::quiet("Start fight"),
            Action::secondary("Open in Lookup"),
        ]
        .map(|action| action.density(Density::Read).natural_width(ui).ceil());
        row = actions.iter().sum::<f32>() + 3.0 * XS;
        need = text(title, Type::Section) + XS + text(kind, Type::Small) + (L - S) + row;
    });
    let at = |width: f32| {
        let mut rows = 0;
        let mut left = 0.0;
        let (nodes, painted) = frame_with(&ctx, vec2(width + 40.0, 400.0), Vec::new(), |ui| {
            let rect = Rect::from_min_size(ui.cursor().min, vec2(width, 300.0));
            left = rect.left();
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                rows = line().show(ui).rows;
            });
        });
        for words in WORDS.into_iter().chain([title, kind]) {
            assert!(painted_whole(&painted, words), "{words:?} whole at {width}");
        }
        let rects = WORDS.map(|word| bounds(&nodes, word));
        // The draw keeps to the measure: every action ends inside the width it was given.
        for (word, rect) in WORDS.iter().zip(&rects) {
            assert!(
                rect.right() <= left + width + 0.5,
                "{word:?} inside {width}"
            );
        }
        (rows, rects)
    };
    // One line, to half a point.
    let (rows, beside) = at(need + 0.5);
    assert_eq!(rows, 0, "fits at {}", need + 0.5);
    assert!(
        beside
            .iter()
            .all(|r| (r.center().y - beside[0].center().y).abs() < 1.0)
    );
    let (rows, under) = at(need - 0.5);
    assert_eq!(rows, 1, "one row under the title at {}", need - 0.5);
    assert!(under.iter().all(|r| (r.top() - under[0].top()).abs() < 1.0));
    assert!(under[0].top() > beside[0].bottom() - 1.0, "under the title");
    // The narrowest pane: one row cannot fit, so the rows wrap at whole actions, from the left,
    // in order, and every word is still whole.
    assert!(row > NARROWEST_PANE, "the case needs more than one row");
    let (rows, wrapped) = at(NARROWEST_PANE);
    assert!(rows > 1, "rows wrap at {NARROWEST_PANE}");
    for pair in wrapped.windows(2) {
        let next_row = pair[1].top() > pair[0].bottom() - 1.0;
        assert!(
            next_row || pair[1].left() > pair[0].right() - 1.0,
            "in order"
        );
        if next_row {
            assert!(
                (pair[1].left() - wrapped[0].left()).abs() < 1.0,
                "from the left"
            );
        }
    }
}

/// The longest word a `Word` may have, in the widest letters and with an icon, is painted whole
/// in the narrowest pane.
#[test]
fn the_longest_word_fits_the_narrowest_pane() {
    const LONGEST: Word = crate::word!("WWWWWWWWWWWWWWWWWWWW");
    assert_eq!(LONGEST.as_str().chars().count(), MAX_WORD_CHARS);
    let ctx = context();
    let (_, painted) = frame_with(&ctx, vec2(NARROWEST_PANE + 40.0, 300.0), Vec::new(), |ui| {
        let rect = Rect::from_min_size(ui.cursor().min, vec2(NARROWEST_PANE, 200.0));
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            let _ = TitleLine::new("Cinder Wolf")
                .action(TitleAction::secondary(LONGEST).icon(crate::design::icons::EDIT))
                .show(ui);
        });
    });
    assert!(painted_whole(&painted, LONGEST.as_str()));
}

crate::create_reasons! {
    enum TestReason {
        NoFolder => "Choose a folder first.",
        Long => "Splat cannot add a campaign here, because another campaign already keeps its notes, encounters and runs in this folder, and two campaigns in one folder would write over each other. Choose another folder first, or open the other campaign.",
    }
}

/// F15: nothing moves while the DM types: Create, the field and what is below stay put through
/// no reason, a short reason and a two-line reason, with and without Cancel, and with a preview.
/// Create sits on the last field's line.
#[test]
fn inline_create_moves_nothing_while_the_dm_types() {
    const CREATE: Word = crate::word!("Add campaign");
    const BROWSE: Word = crate::word!("Browse…");
    const CANCEL: Word = crate::word!("Cancel");
    const PREVIEW: &str = "Will create /home/dm/Obsidian/Vault/Campaigns/The Copper Coast and the Salt Road/Modules/Sessions/A very long folder name that goes on/Copper Coast/Campaign.md";
    let ctx = context();
    let draw_at = |width: f32, reason: Option<TestReason>, cancel: bool, preview: bool| {
        let (mut name, mut folder) = (String::from("Copper Coast"), String::from("/vault"));
        let (nodes, painted) = frame_with(&ctx, vec2(width, 500.0), Vec::new(), |ui| {
            let _ = InlineCreate::new()
                .field(
                    CreateField::new(
                        crate::field_id!("test_create_name"),
                        "Campaign name",
                        &mut name,
                    )
                    .help("Any name."),
                )
                .field(
                    CreateField::new(
                        crate::field_id!("test_create_folder"),
                        "Folder",
                        &mut folder,
                    )
                    .side_action(BROWSE)
                    .help("Splat keeps the notes here."),
                )
                .create(Create::primary(CREATE).blocked(reason))
                .kept_line(
                    KeptLine::new()
                        .cancel(CANCEL, cancel)
                        .preview(preview.then_some(PREVIEW)),
                )
                .show(ui);
            let _ = Action::quiet("Below").show(ui);
        });
        let rects = ["Folder", "Browse…", "Add campaign", "Below"].map(|name| bounds(&nodes, name));
        (rects, nodes, painted)
    };
    let draw = |reason, cancel, preview| draw_at(800.0, reason, cancel, preview);
    let (still, _, _) = draw(None, false, false);
    for reason in [None, Some(TestReason::NoFolder), Some(TestReason::Long)] {
        for cancel in [false, true] {
            let (rects, nodes, painted) = draw(reason, cancel, reason.is_none());
            assert_eq!(rects, still, "{reason:?}, Cancel {cancel}");
            if reason == Some(TestReason::Long) {
                // The case can fail: the long reason is really painted on two lines or more.
                let long = painted
                    .iter()
                    .find(|text| text.text == TestReason::Long.words())
                    .expect("the long reason is painted");
                assert!(
                    long.rect.height() > Density::Read.control(),
                    "taller than a control"
                );
                // It fits its kept line: it ends above the help under that line.
                let help = painted
                    .iter()
                    .find(|text| text.text == "Splat keeps the notes here.")
                    .expect("the help is painted");
                assert!(
                    long.rect.bottom() <= help.rect.top(),
                    "the reason has its room"
                );
            }
            if reason.is_none() {
                // The preview keeps one line, cut at its start; its accessible name is whole.
                let preview = painted
                    .iter()
                    .find(|text| text.text.ends_with("Campaign.md"))
                    .expect("the preview is painted");
                assert!(preview.rect.height() < 1.5 * Type::Small.line());
                assert!(preview.text.starts_with('…'), "{}", preview.text);
                let _ = bounds(&nodes, PREVIEW);
            }
        }
    }
    let [field, browse, create, _] = still;
    assert!(
        (create.center().y - field.center().y).abs() < 1.0,
        "Create is on the field's line"
    );
    assert!(browse.right() <= create.left() && field.right() <= browse.left());
    // The contract across a resize: at each width the kept line's height is one for every
    // reason state, and the same again when the width comes back. A resize may change it, as it
    // changes every wrapped text (wide, then narrow, then wide).
    let below = |width: f32| {
        let mut tops = Vec::new();
        for reason in [None, Some(TestReason::NoFolder), Some(TestReason::Long)] {
            for cancel in [false, true] {
                tops.push(draw_at(width, reason, cancel, reason.is_none()).0[3].top());
            }
        }
        assert!(
            tops.iter().all(|top| *top == tops[0]),
            "one height at {width}: {tops:?}"
        );
        tops[0]
    };
    let wide = below(800.0);
    let narrow = below(420.0);
    assert_eq!(
        below(800.0),
        wide,
        "the same height again at the same width"
    );
    assert!(
        narrow >= wide,
        "a narrower form wraps its tallest reason on more lines"
    );
}

/// PR 297: a field keeps its focus when a layout branch changes around it, and two live fields
/// with one name and two row keys keep their own text and focus. A field whose id comes from its
/// place does not keep it, which is the fault this part rules out.
///
/// The stated limit: the types refuse an `egui::Id` and a bare text as a key, but a number read
/// from the layout still compiles, `NAME.with(RowIndex(ui.next_auto_id().value() as usize))`, and
/// so does `TextKey::from_data(format!("{:?}", ui.id()))`. That a key comes from the row's data
/// is the caller's duty; the lint counts `RowIndex(` and `TextKey::` per file, so a review sees
/// each new one.
#[test]
fn a_field_id_keeps_focus_across_a_layout_branch() {
    const NAME: FieldId = crate::field_id!("test_branch_name");
    let ctx = context();
    let (mut first, mut second) = (String::from("Copper"), String::from("Salt"));
    let mut placed = None;
    let mut draw = |wide: bool, focus: bool, events: Vec<egui::Event>| {
        frame_with(&ctx, vec2(800.0, 400.0), events, |ui| {
            let mut add = |ui: &mut Ui| {
                let _ = line_field(ui, NAME.with(RowIndex(0)), "First", &mut first, "", 200.0);
                let kept = line_field(ui, NAME.with(RowIndex(1)), "Second", &mut second, "", 200.0);
                let mut other = String::new();
                let by_place = ui.add(egui::TextEdit::singleline(&mut other));
                if focus {
                    kept.request_focus();
                }
                by_place.id
            };
            let by_place = if wide {
                ui.horizontal(|ui| add(ui)).inner
            } else {
                ui.vertical(|ui| {
                    ui.label("Narrow");
                    add(ui)
                })
                .inner
            };
            if focus {
                placed = Some(by_place);
            }
        });
    };
    let second_id = NAME.with(RowIndex(1)).id();
    draw(true, true, Vec::new());
    draw(true, false, Vec::new());
    assert!(ctx.memory(|memory| memory.has_focus(second_id)));
    draw(false, false, Vec::new());
    draw(false, false, vec![egui::Event::Text("y".into())]);
    assert!(
        ctx.memory(|memory| memory.has_focus(second_id)),
        "the field keeps its focus"
    );
    assert_eq!(
        (first.as_str(), second.as_str()),
        ("Copper", "Salty"),
        "only the focused field takes the text"
    );
    // The field beside them, with an id from its place, is another widget in the other branch.
    let mut moved = None;
    frame(&ctx, vec2(800.0, 400.0), |ui| {
        ui.vertical(|ui| {
            ui.label("Narrow");
            let _ = line_field(
                ui,
                NAME.with(RowIndex(0)),
                "First",
                &mut String::new(),
                "",
                200.0,
            );
            let _ = line_field(
                ui,
                NAME.with(RowIndex(1)),
                "Second",
                &mut String::new(),
                "",
                200.0,
            );
            let mut other = String::new();
            moved = Some(ui.add(egui::TextEdit::singleline(&mut other)).id);
        });
    });
    assert_ne!(
        placed, moved,
        "an id from the place changes with the branch"
    );
}

/// Rung 4 for `FieldId`, through the gate's own `widget-id-clash` function: two live fields with
/// one id, also at the pane's right edge where the warning is clipped; a field and another widget
/// with its id, also inside the field's rect, where egui gives no warning. Two keys stay clean.
#[test]
fn the_gate_finds_two_live_widgets_with_one_id() {
    const NAME: FieldId = crate::field_id!("test_clash_name");
    let ctx = context();
    ctx.options_mut(|options| options.warn_on_id_clash = true);
    let clashes = |case: usize| {
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(500.0, 400.0))),
            ..Default::default()
        };
        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let field = |ui: &mut Ui, key: usize| {
                    line_field(
                        ui,
                        NAME.with(RowIndex(key)),
                        "A",
                        &mut String::new(),
                        "",
                        200.0,
                    )
                };
                match case {
                    // Two keys: clean.
                    0 => {
                        field(ui, 0);
                        field(ui, 1);
                    }
                    // One id twice, far apart.
                    1 => {
                        field(ui, 0);
                        field(ui, 0);
                    }
                    // One id twice at the right edge: the fields and egui's warning run past
                    // the pane's clip.
                    2 => {
                        ui.add_space(20.0);
                        ui.horizontal(|ui| {
                            ui.add_space(380.0);
                            field(ui, 0);
                        });
                        ui.horizontal(|ui| {
                            ui.add_space(380.0);
                            field(ui, 0);
                        });
                    }
                    // A field, then another widget with its id elsewhere.
                    3 => {
                        field(ui, 0);
                        let rect = Rect::from_min_size(pos2(300.0, 300.0), vec2(40.0, 30.0));
                        ui.interact(rect, NAME.with(RowIndex(0)).id(), Sense::click());
                    }
                    // Two other widgets with one id at the right edge: only egui's warning finds
                    // them, and it runs past the pane's clip.
                    5 => {
                        let a = Rect::from_min_size(pos2(440.0, 40.0), vec2(40.0, 24.0));
                        let id = egui::Id::new("test_two_buttons");
                        ui.interact(a, id, Sense::click());
                        ui.interact(a.translate(vec2(0.0, 200.0)), id, Sense::click());
                    }
                    // A field, then another widget with its id inside the field's rect: egui
                    // gives no warning here.
                    _ => {
                        let response = field(ui, 0);
                        let inside = response.rect.shrink(4.0);
                        ui.interact(inside, NAME.with(RowIndex(0)).id(), Sense::click());
                    }
                }
            });
        });
        widget_id_clashes(&ctx, &output.shapes)
    };
    assert!(clashes(0).is_empty(), "two keys: {:?}", clashes(0));
    for case in 1..=5 {
        assert!(!clashes(case).is_empty(), "case {case} is found");
    }
}
