//! The popup parts prove their rules here, once. Callers do not repeat these proofs.
use std::cell::Cell;

use eframe::egui::{self, Event, Key, Modifiers, PointerButton, Pos2, Rect, pos2, vec2};

use super::*;
use crate::design::{lane_width, pane, theme};

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    theme::install(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// One frame at `size` with `events`. Each frame is a second later, so egui's delays (a hover
/// tip's) pass.
fn frame(
    ctx: &egui::Context,
    size: Vec2,
    events: Vec<Event>,
    mut add: impl FnMut(&mut Ui),
) -> egui::FullOutput {
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

/// A text a frame painted, and whether egui cut it.
struct Text {
    words: String,
    rect: Rect,
    elided: bool,
}

fn texts(output: &egui::FullOutput) -> Vec<Text> {
    fn walk(shape: &egui::Shape, out: &mut Vec<Text>) {
        match shape {
            egui::Shape::Text(text) => out.push(Text {
                words: text.galley.job.text.clone(),
                rect: text.visual_bounding_rect(),
                elided: text.galley.elided,
            }),
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

/// The words painted in a frame.
fn painted(output: &egui::FullOutput) -> Vec<String> {
    texts(output).into_iter().map(|text| text.words).collect()
}

const SIDE: FieldId = crate::field_id!("test_dropdown_side").once(crate::design::Once::Example);
const WORDS: [&str; 4] = ["Allies", "Enemies", "Neutral", "Lair"];

fn list() -> Id {
    SIDE.id().with("list")
}

fn list_rect(ctx: &egui::Context) -> Rect {
    ctx.memory(|memory| memory.area_rect(list()))
        .expect("the list's rect")
}

fn focused(ctx: &egui::Context) -> Option<Id> {
    ctx.memory(|memory| memory.focused())
}

/// What one frame of a test drop-down showed and did.
#[derive(Clone, Copy)]
struct Shown {
    control: Rect,
    pane: Rect,
    changed: bool,
    command: bool,
}

/// How a test drop-down is made.
#[derive(Clone, Copy, Default)]
struct Setup {
    /// Its choices: `WORDS`, or this many numbered ones.
    many: Option<usize>,
    command: bool,
    blocked: bool,
    disabled: bool,
    /// A choice that cannot be chosen now, at this place.
    blocked_choice: Option<usize>,
}

fn dropdown(ui: &mut Ui, current: &Cell<usize>, setup: Setup) -> DropdownResponse {
    let words: Vec<String> = match setup.many {
        Some(count) => (0..count).map(|at| format!("Choice {at}")).collect(),
        None => WORDS.iter().map(|word| (*word).to_owned()).collect(),
    };
    let mut dropdown = Dropdown::new(SIDE, "Side").choice(0, words[0].clone());
    for (at, word) in words.iter().enumerate().skip(1) {
        dropdown = if setup.blocked_choice == Some(at) {
            dropdown.blocked_choice(at, word.clone(), "Linked to Salt Road.")
        } else {
            dropdown.choice(at, word.clone())
        };
    }
    if setup.command {
        dropdown = dropdown.command(crate::word!("Add a side"));
    }
    if setup.blocked {
        dropdown = dropdown.blocked(Some("Close the run first."));
    }
    let mut now = current.get();
    let shown = if setup.disabled {
        ui.add_enabled_ui(false, |ui| dropdown.show(ui, &mut now))
            .inner
    } else {
        dropdown.show(ui, &mut now)
    };
    current.set(now);
    shown
}

/// A pane that ends at a splitter `room` px wide, holding a scroll column, and the drop-down at
/// its foot (`low`) or its head.
fn pane(ui: &mut Ui, room: f32, low: bool, current: &Cell<usize>, setup: Setup) -> Shown {
    let pane = Rect::from_min_size(ui.max_rect().min, vec2(room, ui.max_rect().height()));
    let mut out = None;
    ui.scope_builder(egui::UiBuilder::new().max_rect(pane), |ui| {
        ScrollColumn::new("test_pane").show(ui, |ui| {
            if low {
                let room = ui.max_rect().height() - 2.0 * Density::Read.control();
                ui.allocate_exact_size(vec2(0.0, room), Sense::hover());
            }
            let shown = dropdown(ui, current, setup);
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
                    shown = Some(pane(ui, room, low, &current, Setup::default()));
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

/// In a real pane: a 316 px column ending at a splitter, a scrolled body with the drop-down near
/// its foot, and a footer with a control. The list stays inside the body's room above the
/// footer, clear of the splitter and the lane, and scrolls inside itself when the choices do
/// not fit; a press on a row reaches the row, and a press on the footer's control reaches it,
/// at 960, 1200 and 1920.
#[test]
fn in_a_pane_the_list_stays_above_the_footer() {
    const COLUMN: f32 = 316.0;
    for width in [960.0, 1200.0, 1920.0] {
        let ctx = context();
        // A short column in a tall window: the window has room below the pane, the body has
        // none below its footer.
        let size = vec2(width, 900.0);
        let current = Cell::new(3);
        let control = Cell::new(Rect::NOTHING);
        let column = Cell::new(Rect::NOTHING);
        let link = Cell::new(false);
        let draw = |events| {
            frame(&ctx, size, events, |ui| {
                let rect = Rect::from_min_size(ui.max_rect().min, vec2(COLUMN, 320.0));
                column.set(rect);
                let mut child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(rect)
                        .layout(Layout::top_down(Align::Min)),
                );
                let ui = &mut child;
                let mut pressed = None;
                let mut footer = crate::design::Footer::new("test_pane_footer", COLUMN);
                footer.outcome(
                    crate::outcome::Notice::new(crate::sentence!("The link is off."))
                        .choice(crate::outcome::MessageChoice::Choice("Link again")),
                    &mut pressed,
                );
                footer.pane(ui, pane::Body::Content, |ui| {
                    for line in 0..30 {
                        ui.label(format!("Body line {line}"));
                    }
                    let shown = dropdown(
                        ui,
                        &current,
                        Setup {
                            many: Some(12),
                            ..Setup::default()
                        },
                    );
                    control.set(shown.response.rect);
                    ui.label("After the drop-down");
                });
                if pressed.is_some() {
                    link.set(true);
                }
            })
        };
        for _ in 0..3 {
            draw(Vec::new());
        }
        // Scroll the body until the drop-down sits near the footer.
        let footer = pane::footer_rect(&ctx, Id::new("test_pane_footer")).expect("the footer");
        for _ in 0..80 {
            // Scrolled until the drop-down is in view, just above the footer.
            if control.get().bottom() <= footer.top() - S {
                break;
            }
            draw(vec![
                Event::PointerMoved(pos2(COLUMN / 2.0, 100.0)),
                Event::MouseWheel {
                    unit: egui::MouseWheelUnit::Point,
                    delta: vec2(0.0, -24.0),
                    modifiers: Modifiers::NONE,
                },
            ]);
        }
        draw(Vec::new());
        let record = pane::pane_record(&ctx, Id::new("test_pane_footer")).expect("the pane");
        assert!(record.offset() > 0.0, "the body is scrolled at {width}");
        let footer = record.footer().expect("the footer holds its notice");
        draw(click(control.get().center()));
        draw(Vec::new());
        draw(Vec::new());
        assert!(egui::Popup::is_id_open(&ctx, list()), "open at {width}");
        let popup = list_rect(&ctx);
        assert!(
            popup.bottom() <= footer.top() + 0.5 && popup.top() >= record.room().top() - 0.5,
            "inside the body's room above the footer at {width}: {popup:?}, footer {footer:?}"
        );
        assert!(
            popup.left() >= column.get().left() - 0.5
                && popup.right() <= column.get().right() - lane_width() + 1.0,
            "clear of the splitter and the lane at {width}"
        );
        // Twelve rows do not fit: the list scrolls inside its room.
        assert!(popup.height() < 12.0 * Density::Read.control());
        // A press on a row reaches it.
        let row = pos2(
            popup.center().x,
            popup.top() + 0.5 * Density::Read.control(),
        );
        draw(click(row));
        draw(Vec::new());
        assert!(!egui::Popup::is_id_open(&ctx, list()));
        assert_ne!(current.get(), 3, "the row took the press at {width}");
        assert!(!link.get(), "the footer did not");
        // A press on the footer's control reaches it.
        let words = texts(&draw(Vec::new()));
        let link_at = words
            .iter()
            .find(|text| text.words == "Link again")
            .expect("the footer's control")
            .rect;
        let before = current.get();
        draw(click(link_at.center()));
        draw(Vec::new());
        assert!(link.get(), "the footer's control took the press at {width}");
        assert_eq!(current.get(), before);
    }
}

/// The keyboard: Enter on the control opens the list on the chosen row; an arrow moves to the
/// next row; Enter chooses it and closes the list; focus goes back to the control. Escape closes
/// it with no change, and focus goes back too. In a long list, the focused row is in the list's
/// view when it opens and after each move.
#[test]
fn the_keyboard_opens_moves_chooses_and_returns_focus() {
    let ctx = context();
    let size = vec2(1200.0, 640.0);
    let current = Cell::new(1);
    let changed = Cell::new(false);
    let draw = |events| {
        frame(&ctx, size, events, |ui| {
            let shown = pane(ui, 800.0, false, &current, Setup::default());
            changed.set(changed.get() || shown.changed);
        });
    };
    draw(Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(SIDE.id()));
    draw(Vec::new());
    draw(key(Key::Enter));
    draw(Vec::new());
    assert!(
        egui::Popup::is_id_open(&ctx, list()),
        "Enter opens the list"
    );
    let on_row = focused(&ctx);
    assert!(
        on_row.is_some() && on_row != Some(SIDE.id()),
        "focus is in the list"
    );
    draw(key(Key::ArrowDown));
    draw(Vec::new());
    assert_ne!(focused(&ctx), on_row, "the arrow moves");
    draw(key(Key::Enter));
    draw(Vec::new());
    draw(Vec::new());
    assert!(
        !egui::Popup::is_id_open(&ctx, list()),
        "choosing closes the list"
    );
    assert_eq!(current.get(), 2, "the next row is chosen");
    assert!(changed.get());
    assert_eq!(
        focused(&ctx),
        Some(SIDE.id()),
        "focus is back on the control"
    );
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
    assert_eq!(focused(&ctx), Some(SIDE.id()));

    // A long list: 40 choices, choice 30 chosen.
    let ctx = context();
    let current = Cell::new(30);
    let draw = |events| {
        frame(&ctx, size, events, |ui| {
            let _ = dropdown(
                ui,
                &current,
                Setup {
                    many: Some(40),
                    ..Setup::default()
                },
            );
        });
    };
    let in_view = |ctx: &egui::Context| {
        let popup = list_rect(ctx);
        let row = focused(ctx)
            .and_then(|id| ctx.read_response(id))
            .expect("a focused row")
            .rect;
        popup.contains_rect(row)
    };
    draw(Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(SIDE.id()));
    draw(Vec::new());
    draw(key(Key::Enter));
    draw(Vec::new());
    draw(Vec::new());
    draw(Vec::new());
    assert!(
        in_view(&ctx),
        "the chosen row is in view when the list opens"
    );
    for _ in 0..6 {
        draw(key(Key::ArrowDown));
        draw(Vec::new());
        draw(Vec::new());
        assert!(in_view(&ctx), "the focused row stays in view");
    }
    draw(key(Key::Enter));
    draw(Vec::new());
    assert_eq!(current.get(), 36, "six rows down");
}

/// Press routing: a click on a row chooses exactly that value; a click on the command reports
/// the command and changes nothing; a click outside closes the list with no change. The chosen
/// row has a check mark. A blocked choice shows its reason and takes no pointer or keyboard
/// selection. Two long names that differ at their end are both whole in the open list.
#[test]
fn a_press_reaches_exactly_its_row() {
    let ctx = context();
    let size = vec2(1200.0, 640.0);
    let current = Cell::new(0);
    let control = Cell::new(Rect::NOTHING);
    let command = Cell::new(false);
    // Each row's rect, found by its accessible name.
    let rows = |output: &egui::FullOutput| {
        let nodes = &output
            .platform_output
            .accesskit_update
            .as_ref()
            .unwrap()
            .nodes;
        ["Allies", "Enemies", "Neutral", "Lair", "Add a side"].map(|name| {
            let b = nodes
                .iter()
                .filter(|(_, node)| node.label() == Some(name))
                .find_map(|(_, node)| node.bounds())
                .expect("a row");
            Rect::from_min_max(
                pos2(b.x0 as f32, b.y0 as f32),
                pos2(b.x1 as f32, b.y1 as f32),
            )
        })
    };
    let output = Cell::new(None);
    let draw = |events| {
        let out = frame(&ctx, size, events, |ui| {
            let shown = pane(
                ui,
                800.0,
                false,
                &current,
                Setup {
                    command: true,
                    blocked_choice: Some(1),
                    ..Setup::default()
                },
            );
            control.set(shown.control);
            command.set(command.get() || shown.command);
        });
        let words = painted(&out);
        output.set(Some(out));
        words
    };
    let rows_now = || {
        let out = output.take().expect("a frame");
        let at = rows(&out);
        output.set(Some(out));
        at
    };
    draw(Vec::new());
    draw(click(control.get().center()));
    let words = draw(Vec::new());
    assert!(
        words.iter().any(|w| w == icons::OK),
        "the chosen row has its check mark"
    );
    assert!(
        words.iter().any(|w| w == "Linked to Salt Road."),
        "a blocked choice shows its reason"
    );
    let at = rows_now();
    // The blocked row (1) takes no press.
    draw(click(at[1].center()));
    draw(Vec::new());
    assert_eq!(current.get(), 0, "a blocked choice takes no press");
    // Nor the keyboard: down from the chosen row skips it.
    if !egui::Popup::is_id_open(&ctx, list()) {
        draw(click(control.get().center()));
        draw(Vec::new());
    }
    ctx.memory_mut(|memory| memory.request_focus(SIDE.id()));
    draw(Vec::new());
    draw(key(Key::Escape));
    draw(Vec::new());
    draw(key(Key::Enter));
    draw(Vec::new());
    draw(Vec::new());
    draw(key(Key::ArrowDown));
    draw(Vec::new());
    draw(key(Key::Enter));
    draw(Vec::new());
    assert_ne!(
        current.get(),
        1,
        "a blocked choice takes no keyboard selection"
    );
    assert_eq!(current.get(), 2, "the row after it does");
    assert!(!command.get());
    // The command, after the choices and a rule.
    draw(click(control.get().center()));
    draw(Vec::new());
    let at = rows_now();
    draw(click(at[4].center()));
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

    // Two long names that differ after the control's cut: whole in the list, cut in the
    // control with the whole label in its accessible name.
    const FIRST: &str = "The Emberfall Marches, second campaign of the Copper Coast, northern road";
    const SECOND: &str =
        "The Emberfall Marches, second campaign of the Copper Coast, southern road";
    let ctx = context();
    let control = Cell::new(Rect::NOTHING);
    let draw = |events| {
        frame(&ctx, size, events, |ui| {
            let mut now = 0;
            let shown = Dropdown::new(SIDE, "Campaign")
                .choice(0, FIRST)
                .choice(1, SECOND)
                .width(200.0)
                .show(ui, &mut now);
            control.set(shown.response.rect);
        })
    };
    let closed = draw(Vec::new());
    let cut = texts(&closed).iter().any(|text| text.elided);
    let nodes = closed.platform_output.accesskit_update.unwrap().nodes;
    assert!(
        nodes
            .iter()
            .any(|(_, node)| node.label().is_some_and(|label| label.ends_with(FIRST))),
        "the accessible name holds the whole label"
    );
    assert!(cut, "the control cuts it");
    draw(click(control.get().center()));
    let open = texts(&draw(Vec::new()));
    for name in [FIRST, SECOND] {
        assert!(
            open.iter().any(|text| text.words == name && !text.elided),
            "{name:?} whole in the list"
        );
    }
}

/// A blocked drop-down does not open; its reason is its accessible description. A list that is
/// open when its drop-down becomes blocked, or its parent `Ui` disabled, closes, and takes no
/// pointer or keyboard selection and no command.
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
    assert!(nodes.iter().any(
        |(_, node)| node.label().is_some_and(|label| label.starts_with("Side"))
            && node.description() == Some("Close the run first.")
    ));

    for disabled in [false, true] {
        let ctx = context();
        let current = Cell::new(0);
        let locked = Cell::new(false);
        let control = Cell::new(Rect::NOTHING);
        let command = Cell::new(false);
        let draw = |events| {
            frame(&ctx, vec2(1200.0, 640.0), events, |ui| {
                let shown = dropdown(
                    ui,
                    &current,
                    Setup {
                        command: true,
                        blocked: locked.get() && !disabled,
                        disabled: locked.get() && disabled,
                        ..Setup::default()
                    },
                );
                control.set(shown.response.rect);
                command.set(command.get() || shown.command);
            })
        };
        draw(Vec::new());
        draw(click(control.get().center()));
        draw(Vec::new());
        assert!(
            egui::Popup::is_id_open(&ctx, list()),
            "open before the lock"
        );
        let popup = list_rect(&ctx);
        let rows = [
            pos2(
                popup.center().x,
                popup.top() + 1.5 * Density::Read.control(),
            ),
            pos2(popup.center().x, popup.bottom() - 6.0),
        ];
        locked.set(true);
        draw(Vec::new());
        assert!(
            !egui::Popup::is_id_open(&ctx, list()),
            "the list closes (disabled: {disabled})"
        );
        for at in rows {
            draw(click(at));
            draw(Vec::new());
        }
        draw(key(Key::ArrowDown));
        draw(key(Key::Enter));
        draw(Vec::new());
        assert_eq!(current.get(), 0, "no selection (disabled: {disabled})");
        assert!(!command.get(), "no command (disabled: {disabled})");
    }
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
            let (rect, _) = ui.allocate_exact_size(vec2(120.0, 28.0), Sense::hover());
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
    // On hover: the pointer rests on the control, past egui's delay.
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
                let (rect, _) = ui.allocate_exact_size(vec2(160.0, 28.0), Sense::hover());
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
        let ghost = texts(&output)
            .into_iter()
            .find(|text| text.words == "+ Cinder Wolf")
            .expect("the ghost is painted while dragged")
            .rect;
        assert!(
            ghost.left() >= to.x && ghost.left() <= to.x + M + XS + 1.0,
            "M right of the pointer: {ghost:?}"
        );
        assert!(ctx.content_rect().contains_rect(ghost), "inside the window");
        let carried = egui::DragAndDrop::payload::<u32>(&ctx).map(|p| *p);
        assert_eq!(carried, payload, "the payload, when there is one");
    }
}

/// A 2 × 1 PNG: the texture that arrives after the viewer opens.
const PNG: [u8; 71] = [
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 2, 0, 0, 0, 1, 8, 6, 0,
    0, 0, 244, 34, 127, 138, 0, 0, 0, 14, 73, 68, 65, 84, 120, 156, 99, 248, 207, 192, 240, 31,
    132, 1, 17, 247, 3, 253, 227, 197, 245, 239, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];

/// A known size sizes the viewer before the texture loads: the modal's first (sizing) pass is
/// already as wide as the fitted image, the first visible frame has the image at its fitted
/// size, and when the texture arrives (a 2 × 1 image, the same shape) nothing changes: both
/// dimensions and the aspect ratio. With no size, the viewer waits for the texture.
#[test]
fn a_known_size_sizes_the_viewer_before_the_texture_loads() {
    const URI: &str = "bytes://map.png";
    let natural = vec2(3000.0, 1500.0);
    let window = vec2(1200.0, 800.0);
    let want = fitted(
        natural,
        viewer_room(Rect::from_min_size(Pos2::ZERO, window)),
    );
    for (size, known) in [
        (ImageSize::Known(natural), true),
        (ImageSize::Unknown, false),
    ] {
        let ctx = context();
        egui_extras::install_image_loaders(&ctx);
        let run = || {
            let input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, window)),
                time: Some(ctx.input(|input| input.time) + 1.0),
                ..Default::default()
            };
            let output = ctx.run(input, |ctx| {
                let _ = image_viewer(ctx, "Map", URI, size);
            });
            let nodes = output.platform_output.accesskit_update.unwrap().nodes;
            nodes
                .iter()
                .find(|(_, node)| {
                    node.label() == Some("Map") && node.role() == egui::accesskit::Role::Button
                })
                .and_then(|(_, node)| node.bounds())
                .map(|b| vec2((b.x1 - b.x0) as f32, (b.y1 - b.y0) as f32))
        };
        // The first pass is the modal's sizing pass: its area takes the content's size.
        let _ = run();
        let area = ctx
            .memory(|memory| memory.area_rect(Id::new("image_viewer")))
            .expect("the modal's area");
        // The first visible frame.
        let first = run().expect("the image's target");
        // The texture arrives.
        ctx.include_bytes(URI, PNG.to_vec());
        let mut last = first;
        for _ in 0..4 {
            last = run().expect("the image's target");
        }
        if known {
            assert!(
                area.width() >= want.x - 1.0,
                "the sizing pass is as wide as the image: {area:?} vs {want:?}"
            );
            assert!(
                (first - want).length() < 1.0,
                "fitted on the first visible frame: {first:?} vs {want:?}"
            );
            assert!(
                (last - first).length() < 1.0,
                "the texture changes nothing: {first:?} then {last:?}"
            );
            assert!(
                (last.x / last.y - 2.0).abs() < 0.01,
                "the aspect ratio stays"
            );
        } else {
            assert!(
                (first - want).length() > 1.0,
                "no size before the texture: {first:?}"
            );
        }
    }
}

/// One part draws a chosen row: package X's `design::choice` and its wrapping form,
/// `choice_wrapped`, which the drop-down uses, are each one call of the same painter in
/// `extras.rs`; and the drop-down's file draws no row of its own. This test reads the source with
/// `syn`.
#[test]
fn one_choice_row() {
    let read = |name: &str| {
        let path = format!("{}/src/design/{name}", env!("CARGO_MANIFEST_DIR"));
        syn::parse_file(&std::fs::read_to_string(path).expect("a source file")).expect("it parses")
    };
    let free_fns = |file: &syn::File| -> Vec<syn::ItemFn> {
        file.items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(function) => Some(function.clone()),
                _ => None,
            })
            .collect()
    };
    // The one function each form's body calls, when the body is that one call.
    let only_call = |function: &syn::ItemFn| -> Option<String> {
        let [syn::Stmt::Expr(syn::Expr::Call(call), None)] = function.block.stmts.as_slice() else {
            return None;
        };
        let syn::Expr::Path(path) = call.func.as_ref() else {
            return None;
        };
        Some(path.path.segments.last()?.ident.to_string())
    };
    let extras = free_fns(&read("extras.rs"));
    let form = |name: &str| {
        let function = extras
            .iter()
            .find(|function| function.sig.ident == name)
            .unwrap_or_else(|| panic!("`{name}` is in extras.rs"));
        only_call(function)
    };
    let painter = form("choice");
    assert!(
        painter.is_some(),
        "`choice` is one call of the shared painter"
    );
    assert_eq!(
        form("choice_wrapped"),
        painter,
        "`choice_wrapped` calls the same painter as `choice`"
    );
    // The drop-down's file names `choice_wrapped` and draws no row itself.
    let popups =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/design/popups.rs"))
            .expect("a source file");
    assert!(
        popups.contains("choice_wrapped("),
        "the drop-down's rows are the part"
    );
    assert!(
        !popups.contains("SelectableLabel"),
        "a copy of `design::choice` is back in popups.rs: use `choice_wrapped`"
    );
}

/// K2 (gap K-4): a submenu in a 316 px column, at the pane's right edge, opens neither right nor
/// left of its menu: it opens below its own row, so the row that opened it and the rest of the
/// parent menu stay reachable, and it stays inside the column, clear of the splitter and the
/// lane. A choice in it is reachable, at 960, 1200 and 1920.
#[test]
#[allow(
    clippy::disallowed_methods,
    reason = "owner: the design system and its capture harness draw with egui (design::results::OWNED_ALLOWS)"
)]
fn a_submenu_in_a_narrow_pane_keeps_its_parent_reachable() {
    const COLUMN: f32 = 316.0;
    for width in [960.0, 1200.0, 1920.0] {
        let ctx = context();
        let size = vec2(width, 640.0);
        let chosen = Cell::new(None::<&str>);
        let inside = Cell::new(Rect::NOTHING);
        let draw = |events| {
            frame(&ctx, size, events, |ui| {
                let column = Rect::from_min_size(
                    pos2(ui.max_rect().right() - COLUMN, ui.max_rect().top()),
                    vec2(COLUMN, 600.0),
                );
                let usable = Rect::from_min_max(
                    column.min,
                    pos2(column.right() - lane_width(), column.bottom()),
                );
                inside.set(usable);
                let mut child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(column)
                        .layout(Layout::top_down(Align::Min)),
                );
                let filter = child.button("Filter");
                egui::Popup::menu(&filter).show(|ui| {
                    // A menu as wide as most of the column: the submenu fits on neither side.
                    let _ = ui.button("Sort by challenge rating, then by name");
                    crate::design::menu_submenu(ui, "Type", usable, |ui| {
                        for kind in ["Aberration", "Beast", "Celestial"] {
                            if ui.button(kind).clicked() {
                                chosen.set(Some(kind));
                            }
                        }
                    });
                    let _ = ui.button("Size");
                });
            })
        };
        let bounds = |output: &egui::FullOutput, name: &str| {
            let nodes = &output
                .platform_output
                .accesskit_update
                .as_ref()
                .unwrap()
                .nodes;
            nodes
                .iter()
                .filter(|(_, node)| node.label() == Some(name))
                .find_map(|(_, node)| node.bounds())
                .map(|b| {
                    Rect::from_min_max(
                        pos2(b.x0 as f32, b.y0 as f32),
                        pos2(b.x1 as f32, b.y1 as f32),
                    )
                })
        };
        let first = draw(Vec::new());
        let filter = bounds(&first, "Filter").expect("the menu's button");
        draw(click(filter.center()));
        let menu = draw(Vec::new());
        let row = bounds(&menu, "Type").expect("the submenu's row");
        draw(vec![Event::PointerMoved(row.center())]);
        draw(click(row.center()));
        let mut open = draw(Vec::new());
        for _ in 0..3 {
            open = draw(vec![Event::PointerMoved(row.center())]);
        }
        let aberration = bounds(&open, "Aberration").expect("the submenu is open");
        let row = bounds(&open, "Type").expect("its row");
        assert!(
            // Touching edges are fine; an overlap is not.
            aberration.intersect(row).area() < 1.0,
            "the submenu leaves its row reachable at {width}: {aberration:?} {row:?}"
        );
        assert!(
            aberration.left() >= inside.get().left() - 0.5
                && aberration.right() <= inside.get().right() + 0.5,
            "inside the column, clear of the splitter and the lane at {width}: {aberration:?}"
        );
        // Its choice is reachable.
        draw(vec![Event::PointerMoved(aberration.center())]);
        draw(click(aberration.center()));
        draw(Vec::new());
        assert_eq!(chosen.get(), Some("Aberration"), "the choice at {width}");
    }
}

/// X2: a menu opened in a 316 px column that ends at a splitter, at 960, with choices whose
/// words need more than the column: the menu stays inside the column, clear of the splitter and
/// the lane, whether its control is at the column's left (it opens rightward) or right (it
/// opens leftward), and every choice's row and words stay inside the menu. With short choices,
/// the menu is as wide as its widest choice, not its room.
#[test]
fn a_menu_of_long_choices_stays_beside_its_splitter() {
    const COLUMN: f32 = 316.0;
    const LONG: &str = "Roll initiative for every creature that has not rolled yet, then sort";
    for (right, label) in [(false, LONG), (true, LONG), (false, "All")] {
        let ctx = context();
        let size = vec2(960.0, 640.0);
        let pane = Rect::from_min_size(Pos2::ZERO, vec2(COLUMN, 640.0));
        let rows = std::cell::RefCell::new(Vec::new());
        let button = Cell::new(None::<Response>);
        let draw = |events| {
            frame(&ctx, size, events, |ui| {
                ui.scope_builder(egui::UiBuilder::new().max_rect(pane), |ui| {
                    // A side panel clips its content to its own rect.
                    ui.set_clip_rect(pane);
                    ScrollColumn::new("test_menu_pane").show(ui, |ui| {
                        let layout = if right {
                            Layout::right_to_left(Align::Min)
                        } else {
                            Layout::left_to_right(Align::Min)
                        };
                        let opener = ui.with_layout(layout, |ui| ui.button("Roll")).inner;
                        rows.borrow_mut().clear();
                        crate::design::menu_popup(ui, &opener).show(|ui| {
                            rows.borrow_mut()
                                .push(crate::design::choice(ui, true, label).rect);
                            rows.borrow_mut()
                                .push(crate::design::choice_wrapped(ui, false, label, None).rect);
                        });
                        button.set(Some(opener));
                    });
                });
            })
        };
        draw(Vec::new());
        let opener = button.take().expect("the menu's button");
        draw(click(opener.rect.center()));
        draw(Vec::new());
        let output = draw(Vec::new());
        let id = egui::Popup::default_response_id(&opener);
        assert!(egui::Popup::is_id_open(&ctx, id), "open (right: {right})");
        let menu = ctx
            .memory(|memory| memory.area_rect(id))
            .expect("the menu's rect");
        assert!(
            menu.left() >= pane.left() - 0.5 && menu.right() <= pane.right() - lane_width() + 0.5,
            "the menu {menu:?} is inside the column, clear of the splitter and the lane \
             (right: {right})"
        );
        let rows = rows.borrow();
        assert_eq!(rows.len(), 2, "both choices drawn");
        for row in rows.iter() {
            assert!(
                menu.contains_rect(*row),
                "the row {row:?} is inside the menu {menu:?}"
            );
        }
        if label != LONG {
            assert!(
                menu.width() < (COLUMN - lane_width()) / 2.0,
                "short choices make a narrow menu: {menu:?}"
            );
            continue;
        }
        let words: Vec<Text> = texts(&output)
            .into_iter()
            .filter(|text| text.words == LONG)
            .collect();
        assert_eq!(words.len(), 2, "both choices' words were painted");
        for word in words {
            assert!(
                word.rect.right() <= menu.right() + 0.5,
                "the words {:?} stay inside the menu {menu:?}",
                word.rect
            );
        }
    }
    // X2 review 1: the part decides the menu's width on every draw, not egui's saved size.
    // One popup id, its choices change while it is open: narrow, then a long folder path, then
    // narrow again. The menu follows each (the path does not wrap into a column as narrow as
    // the old menu), always inside the column.
    // X2 review 2: close it, make the pane narrower, open the same popup again: the menu keeps
    // inside the narrower pane.
    {
        const PATH: &str = "Campaigns/Ashen Marches/Encounters/Below the old mill/Second floor";
        let ctx = context();
        let size = vec2(960.0, 640.0);
        let label = Cell::new("All");
        let column = Cell::new(COLUMN);
        let opener = Cell::new(None::<Response>);
        let row = Cell::new(Rect::NOTHING);
        let draw = |events| {
            frame(&ctx, size, events, |ui| {
                let pane = Rect::from_min_size(Pos2::ZERO, vec2(column.get(), 640.0));
                ui.scope_builder(egui::UiBuilder::new().max_rect(pane), |ui| {
                    ui.set_clip_rect(pane);
                    ScrollColumn::new("test_menu_pane").show(ui, |ui| {
                        let button = ui.button("Roll");
                        crate::design::menu_popup(ui, &button).show(|ui| {
                            row.set(
                                crate::design::choice_wrapped(ui, true, label.get(), None).rect,
                            );
                        });
                        opener.set(Some(button));
                    });
                });
            })
        };
        let menu = |button: &Response| {
            let id = egui::Popup::default_response_id(button);
            assert!(egui::Popup::is_id_open(&ctx, id), "the menu is open");
            ctx.memory(|memory| memory.area_rect(id))
                .expect("the menu's rect")
        };
        let settle = |n| {
            for _ in 0..n {
                draw(Vec::new());
            }
        };
        let inside = |menu: Rect, column: f32| {
            menu.left() >= -0.5 && menu.right() <= column - lane_width() + 0.5
        };
        draw(Vec::new());
        let button = opener.take().expect("the button");
        draw(click(button.rect.center()));
        settle(3);
        let narrow = menu(&button);
        assert!(
            narrow.width() < COLUMN / 2.0,
            "x2-measure: a short choice, a narrow menu {narrow:?}"
        );
        label.set(PATH);
        settle(3);
        let wide = menu(&button);
        assert!(
            wide.width() > COLUMN - lane_width() - 2.0 * S && inside(wide, COLUMN),
            "x2-measure: a long path widens the menu to its room {wide:?}"
        );
        assert!(
            row.get().height() < 4.0 * Type::Body.line(),
            "x2-measure: the path wraps in the wide menu, not in a narrow column: {:?}",
            row.get()
        );
        label.set("All");
        settle(3);
        let back = menu(&button);
        assert!(
            back.width() < COLUMN / 2.0,
            "x2-measure: a short choice again, a narrow menu again {back:?}"
        );
        // Wide, close, a narrower pane, reopen.
        label.set(PATH);
        settle(3);
        draw(key(Key::Escape));
        settle(2);
        column.set(200.0);
        settle(2);
        let button = opener.take().expect("the button");
        draw(click(button.rect.center()));
        settle(3);
        let reopened = menu(&button);
        assert!(
            inside(reopened, 200.0),
            "x2-room: reopened in a narrower pane, the menu {reopened:?} keeps inside it"
        );
    }
}
