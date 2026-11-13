//! The popup parts prove their rules here, once. Callers do not repeat these proofs.
use std::cell::Cell;

use eframe::egui::{self, Event, Key, Modifiers, PointerButton, Pos2, Rect, pos2, vec2};

use super::*;
use crate::design::{lane_width, theme};

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    theme::install(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// One frame at `size` with `events`.
fn frame(
    ctx: &egui::Context,
    size: Vec2,
    events: Vec<Event>,
    mut add: impl FnMut(&mut Ui),
) -> egui::FullOutput {
    // Each frame is a second later, so egui's delays (a hover tip's) pass.
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, size)),
        time: Some(ctx.input(|input| input.time) + 1.0),
        events,
        ..Default::default()
    };
    ctx.run(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| add(ui));
    })
}

fn key(key: Key) -> Vec<Event> {
    vec![Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }]
}

fn press(at: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos: at,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

fn click(at: Pos2) -> Vec<Event> {
    vec![Event::PointerMoved(at), press(at, true), press(at, false)]
}

/// The words painted in a frame.
fn painted(output: &egui::FullOutput) -> Vec<String> {
    fn walk(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(text) => out.push(text.galley.job.text.clone()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut out);
    }
    out
}

const SIDE: FieldId = crate::field_id!("test_dropdown_side");
const WORDS: [&str; 4] = ["Allies", "Enemies", "Neutral", "Lair"];

fn list() -> Id {
    SIDE.id().with("list")
}

fn list_rect(ctx: &egui::Context) -> Rect {
    ctx.memory(|memory| memory.area_rect(list()))
        .expect("the list's rect")
}

/// What one frame of the test pane showed and did.
#[derive(Clone, Copy)]
struct Shown {
    control: Rect,
    pane: Rect,
    changed: bool,
    command: bool,
}

/// A pane that ends at a splitter `room` px wide, holding a scroll column, and the drop-down at
/// its foot (`low`) or its head.
fn pane(ui: &mut Ui, room: f32, low: bool, current: &Cell<usize>, command: bool) -> Shown {
    let pane = Rect::from_min_size(ui.max_rect().min, vec2(room, ui.max_rect().height()));
    let mut out = None;
    ui.scope_builder(egui::UiBuilder::new().max_rect(pane), |ui| {
        ScrollColumn::new("test_pane").show(ui, |ui| {
            if low {
                ui.add_space(ui.max_rect().height() - 2.0 * Density::Read.control());
            }
            let mut dropdown = Dropdown::new(SIDE, "Side")
                .choice(0, WORDS[0])
                .choices(WORDS.iter().enumerate().skip(1).map(|(at, w)| (at, *w)));
            if command {
                dropdown = dropdown.command(crate::word!("Add a side"));
            }
            let mut now = current.get();
            let shown = dropdown.show(ui, &mut now);
            current.set(now);
            out = Some(Shown {
                control: shown.response.rect,
                pane,
                changed: shown.changed,
                command: shown.command,
            });
        });
    });
    out.expect("drawn")
}

/// The list opens under its control, as wide as it, inside its pane: clear of the splitter at
/// the pane's edge and of the scroll column's lane, at 960, 1200 and 1920. Near the window's
/// foot it opens above.
#[test]
fn the_list_is_as_wide_as_its_control_and_stays_in_its_pane() {
    for width in [960.0, 1200.0, 1920.0] {
        for low in [false, true] {
            let ctx = context();
            let size = vec2(width, 640.0);
            let room = width - 400.0;
            let current = Cell::new(1);
            let draw = |events| {
                let mut shown = None;
                frame(&ctx, size, events, |ui| {
                    shown = Some(pane(ui, room, low, &current, false));
                });
                shown.expect("drawn")
            };
            let first = draw(Vec::new());
            draw(click(first.control.center()));
            let shown = draw(Vec::new());
            assert!(egui::Popup::is_id_open(&ctx, list()), "open at {width}");
            let popup = list_rect(&ctx);
            let control = shown.control;
            assert!(
                (popup.left() - control.left()).abs() < 1.0
                    && (popup.width() - control.width()).abs() < 1.0,
                "as wide as the control at {width}: {popup:?} {control:?}"
            );
            assert!(
                popup.right() <= shown.pane.right() - lane_width() + 1.0,
                "clear of the lane and the splitter at {width}"
            );
            if low {
                assert!(
                    popup.bottom() <= control.top(),
                    "above at the foot ({width})"
                );
            } else {
                assert!(
                    popup.top() >= control.bottom(),
                    "under the control ({width})"
                );
            }
        }
    }
}

/// The keyboard: Enter on the control opens the list on the chosen row; an arrow moves to the
/// next row; Enter chooses it and closes the list; focus goes back to the control. Escape closes
/// it with no change, and focus goes back too.
#[test]
fn the_keyboard_opens_moves_chooses_and_returns_focus() {
    let ctx = context();
    let size = vec2(1200.0, 640.0);
    let current = Cell::new(1);
    let changed = Cell::new(false);
    let draw = |events| {
        frame(&ctx, size, events, |ui| {
            let shown = pane(ui, 800.0, false, &current, false);
            changed.set(changed.get() || shown.changed);
        });
    };
    let focused = || ctx.memory(|memory| memory.focused());
    draw(Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(SIDE.id()));
    draw(Vec::new());
    draw(key(Key::Enter));
    draw(Vec::new());
    assert!(
        egui::Popup::is_id_open(&ctx, list()),
        "Enter opens the list"
    );
    let on_row = focused();
    assert!(
        on_row.is_some() && on_row != Some(SIDE.id()),
        "focus is in the list"
    );
    draw(key(Key::ArrowDown));
    draw(Vec::new());
    assert_ne!(focused(), on_row, "the arrow moves");
    draw(key(Key::Enter));
    draw(Vec::new());
    draw(Vec::new());
    assert!(
        !egui::Popup::is_id_open(&ctx, list()),
        "choosing closes the list"
    );
    assert_eq!(current.get(), 2, "the next row is chosen");
    assert!(changed.get());
    assert_eq!(focused(), Some(SIDE.id()), "focus is back on the control");
    // Escape: closed, no change, focus back.
    draw(key(Key::Enter));
    draw(Vec::new());
    assert!(egui::Popup::is_id_open(&ctx, list()));
    draw(key(Key::Escape));
    draw(Vec::new());
    draw(Vec::new());
    assert!(
        !egui::Popup::is_id_open(&ctx, list()),
        "Escape closes the list"
    );
    assert_eq!(current.get(), 2);
    assert_eq!(focused(), Some(SIDE.id()));
}

/// Press routing: a click on a row chooses exactly that value; a click on the command reports
/// the command and changes nothing; a click outside closes the list with no change. The
/// chosen row has a check mark.
#[test]
fn a_press_reaches_exactly_its_row() {
    let ctx = context();
    let size = vec2(1200.0, 640.0);
    let current = Cell::new(0);
    let control = Cell::new(Rect::NOTHING);
    let command = Cell::new(false);
    let draw = |events| {
        let output = frame(&ctx, size, events, |ui| {
            let shown = pane(ui, 800.0, false, &current, true);
            control.set(shown.control);
            command.set(command.get() || shown.command);
        });
        painted(&output)
    };
    let row = |at: usize| {
        let popup = list_rect(&ctx);
        pos2(
            popup.center().x,
            popup.top() + (at as f32 + 0.5) * Density::Read.control(),
        )
    };
    draw(Vec::new());
    draw(click(control.get().center()));
    let words = draw(Vec::new());
    assert!(
        words.iter().any(|w| w == icons::OK),
        "the chosen row has its check mark"
    );
    draw(click(row(2)));
    draw(Vec::new());
    assert_eq!(current.get(), 2, "the third row's value");
    assert!(!command.get());
    // The command, after the choices and a rule.
    draw(click(control.get().center()));
    draw(Vec::new());
    let popup = list_rect(&ctx);
    draw(click(pos2(popup.center().x, popup.bottom() - 4.0)));
    draw(Vec::new());
    assert!(command.get(), "the command is reported");
    assert_eq!(current.get(), 2, "and changes no value");
    // A click outside closes with no change.
    draw(click(control.get().center()));
    draw(Vec::new());
    draw(click(pos2(1100.0, 600.0)));
    draw(Vec::new());
    assert!(!egui::Popup::is_id_open(&ctx, list()));
    assert_eq!(current.get(), 2);
}

/// A blocked drop-down does not open; its reason is its accessible description.
#[test]
fn a_blocked_dropdown_does_not_open_and_says_why() {
    let ctx = context();
    let control = Cell::new(Rect::NOTHING);
    let draw = |events| {
        frame(&ctx, vec2(1200.0, 640.0), events, |ui| {
            let mut current = 0;
            let shown = Dropdown::new(SIDE, "Side")
                .choice(0, "Allies")
                .blocked(Some("Close the run first."))
                .show(ui, &mut current);
            control.set(shown.response.rect);
        })
    };
    draw(Vec::new());
    draw(click(control.get().center()));
    let output = draw(Vec::new());
    assert!(!egui::Popup::is_id_open(&ctx, list()));
    let nodes = output.platform_output.accesskit_update.unwrap().nodes;
    assert!(nodes.iter().any(|(_, node)| node.label() == Some("Side")
        && node.description() == Some("Close the run first.")));
}

/// A rich tip shows its typed lines on keyboard focus, and on hover; they are also the
/// control's accessible description.
#[test]
fn a_rich_tip_shows_on_focus_and_on_hover() {
    let ctx = context();
    let id = egui::Id::new("test_tip_target");
    let target = Cell::new(Rect::NOTHING);
    let draw = |events| {
        frame(&ctx, vec2(1200.0, 640.0), events, |ui| {
            let (_, rect) = ui.allocate_space(vec2(120.0, 28.0));
            let response = ui.interact(rect, id, Sense::click());
            target.set(rect);
            let _ = rich_tip(
                response,
                &[
                    TipLine::title("Brannoch Vale"),
                    TipLine::detail("58 of 62 HP"),
                ],
            );
        })
    };
    let quiet = painted(&draw(Vec::new()));
    assert!(
        !quiet.iter().any(|w| w == "Brannoch Vale"),
        "no tip at rest"
    );
    // On hover: the pointer rests on the control; each test frame is a second later, past
    // egui's delay.
    draw(vec![Event::PointerMoved(target.get().center())]);
    draw(Vec::new());
    let words = painted(&draw(Vec::new()));
    assert!(
        words.iter().any(|w| w == "Brannoch Vale"),
        "the tip on hover: {words:?}"
    );
    // On keyboard focus, with the pointer elsewhere.
    draw(vec![Event::PointerMoved(pos2(1100.0, 600.0))]);
    draw(Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(id));
    draw(Vec::new());
    let output = draw(Vec::new());
    let words = painted(&output);
    assert!(
        words.iter().any(|w| w == "Brannoch Vale") && words.iter().any(|w| w == "58 of 62 HP"),
        "the tip on keyboard focus: {words:?}"
    );
    let nodes = output.platform_output.accesskit_update.unwrap().nodes;
    assert!(
        nodes
            .iter()
            .any(|(_, node)| node.description() == Some("Brannoch Vale. 58 of 62 HP"))
    );
}

/// A drag source carries its payload, and while it is dragged its ghost is by the pointer, on
/// the tooltip layer, inside the window. An item with no payload carries none. At rest, no
/// ghost.
#[test]
fn a_drag_carries_its_payload_and_its_ghost() {
    for payload in [Some(7_u32), None] {
        let ctx = context();
        let id = egui::Id::new("test_grip");
        let grip = Cell::new(Rect::NOTHING);
        let draw = |events| {
            frame(&ctx, vec2(1200.0, 640.0), events, |ui| {
                let (_, rect) = ui.allocate_space(vec2(160.0, 28.0));
                let response = ui.interact(rect, id, Sense::click_and_drag());
                grip.set(rect);
                drag_source(ui, &response, payload, Ghost::new("+", "Cinder Wolf"));
            })
        };
        let rest = painted(&draw(Vec::new()));
        assert!(
            !rest.iter().any(|w| w.contains("Cinder Wolf")),
            "no ghost at rest"
        );
        let start = grip.get().center();
        let to = start + vec2(300.0, 200.0);
        draw(vec![Event::PointerMoved(start), press(start, true)]);
        draw(vec![Event::PointerMoved(start + vec2(20.0, 10.0))]);
        let output = draw(vec![Event::PointerMoved(to)]);
        let ghost = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) if text.galley.job.text == "+ Cinder Wolf" => {
                    Some(text.visual_bounding_rect())
                }
                _ => None,
            })
            .expect("the ghost is painted while dragged");
        assert!(
            ghost.left() >= to.x && ghost.left() <= to.x + M + XS + 1.0,
            "M right of the pointer: {ghost:?}"
        );
        assert!(ctx.content_rect().contains_rect(ghost), "inside the window");
        let carried = egui::DragAndDrop::payload::<u32>(&ctx).map(|p| *p);
        assert_eq!(carried, payload, "the payload, when there is one");
    }
}

/// A known size sizes the viewer before the texture loads: the image's target is the size
/// fitted to the window at once. With no size, the viewer waits for the texture.
#[test]
fn a_known_size_sizes_the_viewer_before_the_texture_loads() {
    let natural = vec2(3000.0, 1500.0);
    for (size, expect_fitted) in [
        (ImageSize::Known(natural), true),
        (ImageSize::Unknown, false),
    ] {
        let ctx = context();
        let window = vec2(1200.0, 800.0);
        let mut output = None;
        for _ in 0..4 {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, window)),
                ..Default::default()
            };
            output = Some(ctx.run(input, |ctx| {
                let _ = image_viewer(ctx, "Map", "file:///no/such/map.png", size);
            }));
        }
        let nodes = output
            .unwrap()
            .platform_output
            .accesskit_update
            .unwrap()
            .nodes;
        let image = nodes
            .iter()
            .find(|(_, node)| {
                node.label() == Some("Map") && node.role() == egui::accesskit::Role::Button
            })
            .and_then(|(_, node)| node.bounds())
            .expect("the image's target");
        let want = fitted(
            natural,
            viewer_room(Rect::from_min_size(Pos2::ZERO, window)),
        );
        let width = (image.x1 - image.x0) as f32;
        if expect_fitted {
            assert!(
                (width - want.x).abs() < 1.0,
                "fitted at once: {width} vs {}",
                want.x
            );
        } else {
            assert!(width < want.x - 1.0, "no size before the texture: {width}");
        }
    }
}
