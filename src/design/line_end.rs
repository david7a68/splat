//! Words that end with their actions (ruling M21, row M0574): a row's quiet actions at the right
//! end of the words' last line when they fit there, else on a line of their own at the right.
//!
//! One owner measures the last line: the actions and the last line's words share one centre,
//! the centre of a line as tall as the actions. The line above keeps its pitch, and the last
//! line starts under it with no gap, so no action reaches over the words above; the actions'
//! extra height is shared above and below the last line's words.
use super::{Type, colors};
use egui::{Align, Layout, Sense, Ui, UiBuilder, vec2};

/// `words` in `kind`, wrapped at the `Ui`'s width, then `add`'s actions: `trailing` wide and
/// `height` tall, laid out from the right. Where they fit after the words' last line, they stand
/// on it, centred with its words; else they take a line of their own at the right, `height`
/// tall. The words' wrapped rows are their own label, and the last line is its own label, so
/// nothing overlaps the words.
pub fn words_ending_with(
    ui: &mut Ui,
    kind: Type,
    words: &str,
    (trailing, height): (f32, f32),
    add: impl FnOnce(&mut Ui),
) {
    let width = ui.available_width();
    let galley = ui
        .painter()
        .layout(words.to_owned(), kind.font(ui), colors().text, width);
    let gap = ui.spacing().item_spacing.x;
    let fits = galley
        .rows
        .last()
        .is_some_and(|row| width - row.rect().width() >= trailing + gap);
    if !fits {
        ui.add(egui::Label::new(kind.rich(ui, words)).wrap());
        actions_line(ui, height, add);
        return;
    }
    let before: usize = galley.rows[..galley.rows.len() - 1]
        .iter()
        .map(|row| row.char_count_including_newline())
        .sum();
    let split = words
        .char_indices()
        .nth(before)
        .map_or(words.len(), |(at, _)| at);
    let (head, last) = words.split_at(split);
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        if !head.trim().is_empty() {
            ui.add(egui::Label::new(kind.rich(ui, head.trim_end())).wrap());
        }
        // The last line: as tall as the actions, its words and its actions on its centre.
        let line = height.max(kind.line());
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), line), Sense::hover());
        let mut row = ui.new_child(
            UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::left_to_right(Align::Center)),
        );
        row.label(kind.rich(&row, last.trim()));
        row.with_layout(Layout::right_to_left(Align::Center), add);
    });
}

/// The actions on a line of their own, `height` tall, laid out from its right end.
fn actions_line(ui: &mut Ui, height: f32, add: impl FnOnce(&mut Ui)) {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), height),
        Layout::right_to_left(Align::Center),
        add,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::{Action, Density};
    use egui_kittest::{Harness, kittest::Queryable};

    const WORDS: &str = "Old Guard: the second seal is taken, and the gate of the cistern opens \
        for the party.";

    /// M0574 (SH15, AH12): where `Done` fits after the words' last line, its words' centre is
    /// the last line's centre within a pixel, and its target never reaches over the line above.
    #[test]
    fn a_last_line_and_its_action_share_one_centre() {
        use crate::design::test_harness::ThemedPage as _;
        let mut h = Harness::builder()
            .with_size(vec2(360.0, 300.0))
            .themed_ui(|ui| {
                let done = Action::quiet("Done").density(Density::Read);
                let trailing = done.natural_width(ui);
                words_ending_with(
                    ui,
                    Type::Body,
                    WORDS,
                    (trailing, Density::Read.control()),
                    |ui| {
                        let _ = Action::quiet("Done").density(Density::Read).show(ui);
                    },
                );
            });
        h.run();
        let done = h.get_by_label("Done").rect();
        let last = h
            .query_all_by_role(egui::accesskit::Role::Label)
            .map(|node| node.rect())
            .max_by(|a, b| a.top().total_cmp(&b.top()))
            .expect("the words' labels");
        let head = h
            .query_all_by_role(egui::accesskit::Role::Label)
            .map(|node| node.rect())
            .min_by(|a, b| a.top().total_cmp(&b.top()))
            .expect("the words' labels");
        assert!(
            head.bottom() <= last.top() + 0.5,
            "the last line is its own label under the others: {head:?} {last:?}"
        );
        assert!(
            (done.center().y - last.center().y).abs() <= 1.0,
            "Done's centre {:.1} and the last line's centre {:.1}",
            done.center().y,
            last.center().y
        );
        assert!(
            done.top() >= head.bottom() - 0.5,
            "Done reaches over the line above: {done:?} {head:?}"
        );
    }
}
