//! The layout parts prove their rules here, once. Callers do not repeat these proofs.
use eframe::egui::{self, Rect, pos2, vec2};

use super::*;
use crate::design::theme;

/// One frame at `size`, drawing `add` in a central panel. Returns the accessible nodes.
fn frame(
    ctx: &egui::Context,
    size: egui::Vec2,
    add: impl FnMut(&mut Ui),
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    let mut add = add;
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), size)),
        ..Default::default()
    };
    let output = ctx.run(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| add(ui));
    });
    output
        .platform_output
        .accesskit_update
        .map(|update| update.nodes)
        .unwrap_or_default()
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

/// F14: the content's right edge is the same with and without the scroll bar.
#[test]
fn scroll_column_keeps_the_lane_with_and_without_the_bar() {
    let ctx = context();
    let edge = |rows: usize| {
        let mut right = 0.0;
        let mut overflows = false;
        frame(&ctx, vec2(600.0, 300.0), |ui| {
            let output = ScrollColumn::new(("lane", rows))
                .max_height(200.0)
                .show(ui, |ui| {
                    for row in 0..rows {
                        ui.label(format!("Row {row}"));
                    }
                    right = ui.max_rect().right();
                });
            overflows = output.content_size.y > output.inner_rect.height();
        });
        (right, overflows)
    };
    // Two frames each, so the bar's state settles.
    edge(1);
    let (short, short_bar) = edge(1);
    edge(80);
    let (tall, tall_bar) = edge(80);
    assert!(
        !short_bar && tall_bar,
        "one column has a bar and one has none"
    );
    assert_eq!(short, tall, "the content ends at one edge");
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

/// F17: the actions follow the title when they fit, by measure; else all of them move together
/// to one line under the title, each with its word. None is ever dropped.
#[test]
fn title_line_keeps_every_action_and_its_word_at_every_width() {
    const EDIT: Word = crate::word!("Edit");
    const LOG: Word = crate::word!("Add to Log");
    const START: Word = crate::word!("Start fight");
    let ctx = context();
    let line = || {
        TitleLine::new("Cinder Wolf")
            .role(TitleType::Section)
            .action(TitleAction::quiet(EDIT).icon(crate::design::icons::EDIT))
            .action(TitleAction::quiet(LOG))
            .action(TitleAction::secondary(START))
    };
    // The width the title and all actions need on one line, from the same measure.
    let mut need = 0.0;
    frame(&ctx, vec2(1200.0, 300.0), |ui| {
        let probe = line();
        need = probe.title_width(ui) + L + probe.actions_width(ui);
    });
    let at = |width: f32| {
        let mut below = (false, 0);
        let mut seen = (0.0, 0.0);
        let nodes = frame(&ctx, vec2(width + 40.0, 300.0), |ui| {
            let rect = Rect::from_min_size(ui.cursor().min, vec2(width, 200.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                let probe = line();
                seen = (
                    ui.available_width(),
                    probe.title_width(ui) + L + probe.actions_width(ui),
                );
                let shown = line().show(ui);
                below = (shown.below, shown.rows);
            });
        });
        assert_eq!(seen, (width, need), "the measure is the probe's");
        let rects = ["Edit", "Add to Log", "Start fight"].map(|word| bounds(&nodes, word));
        (below, rects)
    };
    for width in [need + 1.0, need + 200.0] {
        let ((below, _), rects) = at(width);
        assert!(!below, "fits at {width}");
        assert!(
            rects
                .iter()
                .all(|r| (r.center().y - rects[0].center().y).abs() < 1.0)
        );
    }
    // Step 2: one row under the title, never wrapped while that row fits the pane.
    let ((below, rows), rects) = at(need - 1.0);
    assert!(
        below && rows == 1,
        "one row under the title at {}",
        need - 1.0
    );
    // Every action is there, on one line, with its word (the node is found by it).
    assert!(rects.iter().all(|r| (r.top() - rects[0].top()).abs() < 1.0));
    let (_, wide) = at(need + 1.0);
    assert!(rects[0].top() > wide[0].bottom() - 1.0, "under the title");
    // Step 3: a pane too narrow for one row wraps at whole actions, from the left.
    let ((below, rows), rects) = at(150.0);
    assert!(below && rows > 1, "rows wrap at 150");
    assert!(rects[1].top() > rects[0].top() || rects[2].top() > rects[1].top());
    let left = rects.iter().map(|r| r.left()).fold(f32::MAX, f32::min);
    for pair in rects.windows(2) {
        // In order: the next action is right of this one on its row, or starts a lower row.
        assert!(pair[1].left() > pair[0].right() - 1.0 || pair[1].top() > pair[0].bottom() - 1.0);
        if pair[1].top() > pair[0].bottom() - 1.0 {
            assert!(
                (pair[1].left() - left).abs() < 1.0 + S,
                "a row starts at the left"
            );
        }
    }
}

/// The ruling's order, by measure, at each boundary: on the title line, else one row under the
/// title, else rows that wrap at whole actions. Every action once, in order; never a wrap while
/// one row fits; an action wider than the pane has its own row.
#[test]
fn title_placement_follows_the_ruling_at_each_boundary() {
    let widths = [60.0, 100.0, 80.0];
    let row = 60.0 + 100.0 + 80.0 + 2.0 * XS;
    let title = 200.0;
    assert_eq!(
        title_placement(title, &widths, XS, title + L + row),
        TitlePlacement::OnTitle
    );
    assert_eq!(
        title_placement(title, &widths, XS, title + L + row - 1.0),
        TitlePlacement::Under(vec![0..3])
    );
    assert_eq!(
        title_placement(title, &widths, XS, row),
        TitlePlacement::Under(vec![0..3])
    );
    assert_eq!(
        title_placement(title, &widths, XS, row - 1.0),
        TitlePlacement::Under(vec![0..2, 2..3])
    );
    assert_eq!(
        title_placement(title, &widths, XS, 100.0),
        TitlePlacement::Under(vec![0..1, 1..2, 2..3])
    );
    // Wider than the pane: its own row, never dropped.
    assert_eq!(
        title_placement(title, &widths, XS, 70.0),
        TitlePlacement::Under(vec![0..1, 1..2, 2..3])
    );
    // No actions: nothing moves under the title.
    assert_eq!(
        title_placement(title, &[], XS, 10.0),
        TitlePlacement::OnTitle
    );
}

/// F15: typing moves nothing: Create, the field, the kept line and the help below stay put, and
/// Create sits on the last field's line.
#[test]
fn inline_create_moves_nothing_while_the_dm_types() {
    const CREATE: Word = crate::word!("Add campaign");
    const BROWSE: Word = crate::word!("Browse…");
    const CANCEL: Word = crate::word!("Cancel");
    let ctx = context();
    let draw = |name: &str, folder: &str, with_preview: bool| {
        let (mut name, mut folder) = (name.to_owned(), folder.to_owned());
        let reason = folder.is_empty().then_some("Choose a folder first.");
        let preview = format!("Will create {folder}/Campaign.md");
        let cancel = (!name.is_empty()).then_some(CANCEL);
        let nodes = frame(&ctx, vec2(800.0, 400.0), |ui| {
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
                        .cancel(cancel)
                        .preview(with_preview.then_some(preview.as_str())),
                )
                .show(ui);
            let _ = Action::quiet("Below").show(ui);
        });
        ["Folder", "Browse…", "Add campaign", "Below"].map(|name| bounds(&nodes, name))
    };
    let empty = draw("", "", true);
    let typed = draw("Copper Coast", "/vault/copper", true);
    // A kept line with nothing on it (no Cancel, no reason, no preview) keeps its height too.
    assert_eq!(
        draw("", "/vault/copper", false),
        empty,
        "an empty kept line stays"
    );
    assert_eq!(empty, typed, "nothing moves as the DM types");
    let [field, browse, create, _] = typed;
    assert!(
        (create.center().y - field.center().y).abs() < 1.0,
        "Create is on the field's line"
    );
    assert!(browse.right() <= create.left() && field.right() <= browse.left());
}

/// PR 297: a field keeps its focus when a layout branch changes around it. A field whose id
/// comes from its place does not, which is the fault this part rules out.
#[test]
fn a_field_id_keeps_focus_across_a_layout_branch() {
    const NAME: FieldId = crate::field_id!("test_branch_name");
    let ctx = context();
    let mut text = String::from("Copper");
    let mut placed = None;
    let mut draw = |wide: bool, focus: bool| {
        frame(&ctx, vec2(800.0, 400.0), |ui| {
            let mut add = |ui: &mut Ui| {
                let kept = line_field(ui, NAME, "Name", &mut text, "", 200.0);
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
    draw(true, true);
    draw(true, false);
    assert!(ctx.memory(|memory| memory.has_focus(NAME.id())));
    draw(false, false);
    draw(false, false);
    assert!(
        ctx.memory(|memory| memory.has_focus(NAME.id())),
        "the field keeps its focus"
    );
    // The field beside it, with an id from its place, is another widget in the other branch.
    let mut moved = None;
    frame(&ctx, vec2(800.0, 400.0), |ui| {
        ui.vertical(|ui| {
            ui.label("Narrow");
            let mut other = String::new();
            let _ = line_field(ui, NAME, "Name", &mut String::new(), "", 200.0);
            moved = Some(ui.add(egui::TextEdit::singleline(&mut other)).id);
        });
    });
    assert_ne!(
        placed, moved,
        "an id from the place changes with the branch"
    );
}
