//! A side column with a drag handle on its inner edge. Until the DM drags it, a column
//! has its natural width, which can follow the window. After that it has the DM's width,
//! here and after a restart, and the column beside it takes what is left.
use super::tokens::*;
use eframe::egui::{
    self, Context, Id, InnerResponse, Ui,
    containers::panel::{PanelState, Side},
};
use std::{collections::BTreeMap, ops::RangeInclusive};

/// What a drag leaves for the page beside the column.
const PAGE_KEEPS: f32 = 320.0;

/// The DM's widths live in egui's memory, so that a column deep in a page needs no path
/// back to the app. The app restores them at the start and saves them when they change.
#[derive(Clone, Default)]
struct Widths(BTreeMap<String, f32>);

fn widths_id() -> Id {
    Id::new("column_widths")
}

pub fn restore_column_widths(ctx: &Context, widths: BTreeMap<String, f32>) {
    ctx.data_mut(|data| data.insert_temp(widths_id(), Widths(widths)));
}

pub fn column_widths(ctx: &Context) -> BTreeMap<String, f32> {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<Widths>(widths_id())
            .0
            .clone()
    })
}

/// Whether the DM has chosen this column's width. A page that holds its content to a
/// comfortable width beside a natural column lets it reach a column the DM moved.
pub fn column_dragged(ctx: &Context, id: &str) -> bool {
    ctx.data_mut(|data| {
        data.get_temp_mut_or_default::<Widths>(widths_id())
            .0
            .contains_key(id)
    })
}

pub struct Column {
    id: &'static str,
    side: Side,
    natural: f32,
    range: RangeInclusive<f32>,
    page_keeps: f32,
    frame: egui::Frame,
}

impl Column {
    pub fn left(id: &'static str, natural: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            id,
            side: Side::Left,
            natural,
            range,
            page_keeps: PAGE_KEEPS,
            frame: egui::Frame::new(),
        }
    }

    pub fn right(id: &'static str, natural: f32, range: RangeInclusive<f32>) -> Self {
        Self {
            side: Side::Right,
            ..Self::left(id, natural, range)
        }
    }

    /// A panel beside a page's content: a run's panel, or a timeline event.
    pub fn panel(id: &'static str, natural: f32) -> Self {
        Self::right(id, natural, PANEL_MIN_WIDTH..=PANEL_DRAG_MAX_WIDTH)
    }

    pub fn frame(mut self, frame: egui::Frame) -> Self {
        self.frame = frame;
        self
    }

    /// For a page that needs more than the usual room beside this column.
    pub fn page_keeps(mut self, width: f32) -> Self {
        self.page_keeps = width;
        self
    }

    pub fn show<R>(self, ctx: &Context, add: impl FnOnce(&mut Ui) -> R) -> R {
        let id = self.id;
        let shown = self
            .panel_within(ctx, ctx.available_rect().width())
            .show(ctx, |ui| fill_and_add(ui, add));
        finish_handle(ctx, id, &shown);
        shown.inner
    }

    pub fn show_inside<R>(self, ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
        let id = self.id;
        let ctx = ui.ctx().clone();
        let shown = self
            .panel_within(&ctx, ui.available_width())
            .show_inside(ui, |ui| fill_and_add(ui, add));
        finish_handle(&ctx, id, &shown);
        shown.inner
    }

    fn panel_within(self, ctx: &Context, room: f32) -> egui::SidePanel {
        let id = Id::new(self.id);
        let mut dragged = ctx.data_mut(|data| {
            // Egui would hold on to the width of the last frame, and a window that changes
            // size could not move a natural width any more.
            data.remove::<PanelState>(id);
            data.get_temp_mut_or_default::<Widths>(widths_id())
                .0
                .get(self.id)
                .copied()
        });
        // A natural width can lie outside the range. A window with no room for the page
        // limits even that, and the narrow end of the range goes first.
        let min = self.range.start().min(self.natural);
        let max = self
            .range
            .end()
            .max(self.natural)
            .min((room - self.page_keeps).max(*self.range.start()));
        if ctx.memory(|memory| memory.has_focus(id.with("__resize"))) {
            let (left, right, home) = ctx.input_mut(|input| {
                (
                    input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowLeft),
                    input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowRight),
                    input.consume_key(egui::Modifiers::NONE, egui::Key::Home),
                )
            });
            if home || left || right {
                dragged = if home {
                    None
                } else {
                    let direction = if self.side == Side::Left { 1.0 } else { -1.0 };
                    let delta = (i32::from(right) - i32::from(left)) as f32 * L * direction;
                    Some((dragged.unwrap_or(self.natural).clamp(min, max) + delta).clamp(min, max))
                };
                ctx.data_mut(|data| {
                    let widths = &mut data.get_temp_mut_or_default::<Widths>(widths_id()).0;
                    if let Some(width) = dragged {
                        widths.insert(self.id.to_owned(), width);
                    } else {
                        widths.remove(self.id);
                    }
                });
                ctx.request_repaint();
            }
        }
        egui::SidePanel::new(self.side, id)
            .resizable(true)
            .default_width(dragged.unwrap_or(self.natural))
            .width_range(min..=max)
            .frame(self.frame)
    }
}

/// A resizable panel is only as wide as what it holds, so what it holds takes all of it.
fn fill_and_add<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.set_min_width(ui.available_width());
    add(ui)
}

fn finish_handle<R>(ctx: &Context, id: &'static str, shown: &InnerResponse<R>) {
    // The handle's id is egui's own. A test drags a handle, so an update cannot lose it.
    let handle = Id::new(id).with("__resize");
    let Some(response) = ctx.read_response(handle) else {
        return;
    };
    let column = match id {
        "navigation" => "navigation",
        "library_sections" => "Library categories",
        "library_entries" => "Library entries",
        "world_list_panel" => "World notes",
        "session_list_panel" => "sessions",
        "template_list_panel" => "templates",
        "campaign_modules" => "campaign packages",
        "party_members" => "party members",
        "session_notes" => "session notes",
        "lookup" => "Lookup",
        "timeline_detail_panel" => "timeline details",
        "run_panel" | "run_setup_panel" | "run_ended_log" => "fight panel",
        "encounter_add" => "creature search",
        _ => "side panel",
    };
    ctx.accesskit_node_builder(handle, |node| {
        node.set_role(egui::accesskit::Role::Splitter);
        node.set_label(format!("Resize {column} column"));
        node.set_description("Left and Right move the edge. Home restores the natural width.");
    });
    if response.has_focus() {
        ctx.layer_painter(response.layer_id).vline(
            response.rect.center().x,
            response.rect.y_range(),
            egui::Stroke::new(FOCUS_WIDTH, super::colors().focus),
        );
    }
    if response.dragged() {
        let width = shown.response.rect.width();
        ctx.data_mut(|data| {
            data.get_temp_mut_or_default::<Widths>(widths_id())
                .0
                .insert(id.to_owned(), width);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::{Event, Modifiers, PointerButton, Pos2, pos2};
    use egui_kittest::Harness;
    use std::{cell::Cell, rc::Rc};

    const NATURAL: f32 = 200.0;
    const RANGE: RangeInclusive<f32> = 150.0..=300.0;

    /// A window with one column, and the width that column had in the last frame.
    fn window(column: impl Fn() -> Column + 'static) -> (Harness<'static>, Rc<Cell<f32>>) {
        let width = Rc::new(Cell::new(0.0));
        let seen = width.clone();
        let harness = Harness::builder()
            .with_size(egui::vec2(1000.0, 400.0))
            .build(move |ctx| {
                column().show(ctx, |ui| {
                    seen.set(ui.max_rect().width());
                });
                egui::CentralPanel::default().show(ctx, |_| {});
            });
        (harness, width)
    }

    fn drag(harness: &mut Harness<'_>, from: Pos2, to: Pos2) {
        harness.hover_at(from);
        harness.step();
        for (pos, pressed) in [(from, true), (to, true), (to, false)] {
            harness.hover_at(pos);
            harness.event(Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed,
                modifiers: Modifiers::NONE,
            });
            harness.step();
        }
        harness.remove_cursor();
        harness.step();
    }

    #[test]
    fn iter3_column_handle_resizes_by_keyboard_and_home_restores_natural_width() {
        for side in [Side::Left, Side::Right] {
            let (mut harness, width) = window(move || match side {
                Side::Left => Column::left("c", NATURAL, RANGE),
                Side::Right => Column::right("c", NATURAL, RANGE),
            });
            harness.run_steps(2);
            harness.key_press(egui::Key::Tab);
            harness.run();
            assert_eq!(
                harness.ctx.memory(|memory| memory.focused()),
                Some(Id::new("c").with("__resize"))
            );
            harness.key_press(egui::Key::ArrowRight);
            harness.run();
            assert_eq!(
                width.get(),
                NATURAL + if side == Side::Left { L } else { -L }
            );
            harness.key_press(egui::Key::Home);
            harness.run();
            assert_eq!(width.get(), NATURAL);
            assert!(!column_dragged(&harness.ctx, "c"));
            harness.key_press(egui::Key::ArrowLeft);
            harness.run();
            assert_eq!(
                width.get(),
                NATURAL + if side == Side::Left { -L } else { L }
            );
            assert!(column_dragged(&harness.ctx, "c"));
        }
    }

    #[test]
    fn a_column_keeps_its_natural_width_until_it_is_dragged() {
        let (mut harness, width) = window(|| Column::left("c", NATURAL, RANGE));
        harness.step();
        assert_eq!(width.get(), NATURAL);
        assert!(column_widths(&harness.ctx).is_empty());
        assert!(!column_dragged(&harness.ctx, "c"));
        drag(&mut harness, pos2(NATURAL, 200.0), pos2(260.0, 200.0));
        assert!(column_dragged(&harness.ctx, "c"));
        assert_eq!(width.get(), 260.0);
        assert_eq!(column_widths(&harness.ctx)["c"], 260.0);
    }

    #[test]
    fn a_drag_stops_at_the_ends_of_the_range() {
        let (mut harness, width) = window(|| Column::left("c", NATURAL, RANGE));
        harness.step();
        drag(&mut harness, pos2(NATURAL, 200.0), pos2(900.0, 200.0));
        assert_eq!(width.get(), *RANGE.end());
        drag(&mut harness, pos2(*RANGE.end(), 200.0), pos2(5.0, 200.0));
        assert_eq!(width.get(), *RANGE.start());
    }

    #[test]
    fn a_right_column_follows_the_pointer_from_its_left_edge() {
        let (mut harness, width) = window(|| Column::right("c", NATURAL, RANGE));
        harness.step();
        drag(&mut harness, pos2(800.0, 200.0), pos2(740.0, 200.0));
        assert_eq!(width.get(), 260.0);
    }

    #[test]
    fn a_drag_leaves_the_page_its_room() {
        let (mut harness, width) =
            window(|| Column::left("c", NATURAL, 150.0..=900.0).page_keeps(600.0));
        harness.step();
        drag(&mut harness, pos2(NATURAL, 200.0), pos2(950.0, 200.0));
        assert_eq!(width.get(), 400.0);
    }

    #[test]
    fn a_saved_width_returns_and_a_bad_one_stays_in_the_range() {
        let (mut harness, width) = window(|| Column::left("c", NATURAL, RANGE));
        restore_column_widths(&harness.ctx, BTreeMap::from([("c".to_owned(), 280.0)]));
        harness.step();
        assert_eq!(width.get(), 280.0);
        for (bad, kept) in [(-5.0, 150.0), (f32::INFINITY, 300.0)] {
            restore_column_widths(&harness.ctx, BTreeMap::from([("c".to_owned(), bad)]));
            harness.step();
            assert_eq!(width.get(), kept);
        }
    }

    #[test]
    fn a_natural_width_still_follows_the_window() {
        let natural = Rc::new(Cell::new(NATURAL));
        let source = natural.clone();
        let (mut harness, width) = window(move || Column::left("c", source.get(), RANGE));
        harness.step();
        natural.set(240.0);
        harness.step();
        assert_eq!(width.get(), 240.0);
    }
}
