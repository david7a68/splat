pub use splat_content_ui::design::*;

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use super::extras::test_space as space;
    use super::reader::{detail, linked_detail};
    use super::*;

    /// The painted text shapes of a harness: each galley's words, where they are drawn, and
    /// whether egui cut them to an ellipsis.
    fn painted(harness: &egui_kittest::Harness<'_>) -> Vec<(String, Rect, bool)> {
        fn walk(shape: &egui::Shape, out: &mut Vec<(String, Rect, bool)>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, out)),
                egui::Shape::Text(text) => out.push((
                    text.galley.job.text.clone(),
                    Rect::from_min_size(text.pos, text.galley.size()),
                    text.galley.elided,
                )),
                _ => {}
            }
        }
        let mut out = Vec::new();
        for shape in &harness.output().shapes {
            walk(&shape.shape, &mut out);
        }
        out
    }

    fn painted_texts(harness: &egui_kittest::Harness<'_>) -> Vec<(String, Rect)> {
        painted(harness)
            .into_iter()
            .map(|(words, rect, _)| (words, rect))
            .collect()
    }
    use eframe::egui::{self, Rect, Ui, vec2};
    use splat_content::content::dice_text::{D20, RollLink};

    #[test]
    fn a_wrapped_message_keeps_its_whole_action() {
        use crate::design::test_harness::ThemedPage as _;
        for width in [240.0, 360.0, 440.0, 880.0] {
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(width, 240.0))
                .themed_ui(|ui| {
                    message_with_action(ui, Tone::Info,
                        "The party has full HP after this long rest. Every changed value is kept for undo.",
                        "Undo delete saved run");
                });
            harness.run_steps(2);
            // No words are cut to an ellipsis or run past the window's edges.
            let cut: Vec<_> = painted(&harness)
                .into_iter()
                .filter(|(_, rect, elided)| {
                    *elided || rect.left() < -1.0 || rect.right() > width + 1.0
                })
                .collect();
            assert!(cut.is_empty(), "width {width}: {cut:?}");
            // iter-23-S FK3: beside the words, the action starts `L` after their last letter.
            if width >= 880.0 {
                use egui_kittest::kittest::Queryable;
                let words = harness
                    .output()
                    .shapes
                    .iter()
                    .find_map(|shape| match &shape.shape {
                        egui::Shape::Text(text)
                            if text.galley.job.text.starts_with("The party has full HP") =>
                        {
                            Some(Rect::from_min_size(text.pos, text.galley.size()))
                        }
                        _ => None,
                    })
                    .expect("the words are painted");
                let action = harness.get_by_label("Undo delete saved run").rect();
                assert!(
                    action.top() < words.bottom()
                        && (action.left() - words.right() - L).abs() <= 1.0,
                    "width {width}: words {words:?}, action {action:?}"
                );
            }
        }
        // iter-25-A AB2: in a wide pane, the message and its action stay inside `LIST_MEASURE`,
        // as every notice does. The words alone are wider than the measure.
        const LONG: &str = "This is the open session, not the session you played. Changes here go to \
            the open session's note, so go back to the played session before you add what happened.";
        let mut harness = egui_kittest::Harness::builder()
            .with_size(vec2(1800.0, 240.0))
            .themed_ui(|ui| {
                message_with_action(ui, Tone::Warning, LONG, "Go to the played session");
            });
        harness.run_steps(2);
        let texts = painted_texts(&harness);
        let words = texts
            .iter()
            .find(|(text, _)| text == LONG)
            .map(|(_, rect)| *rect)
            .expect("the words are painted");
        use egui_kittest::kittest::Queryable;
        let action = harness.get_by_label("Go to the played session").rect();
        let left = words.left().min(action.left()) - S - 1.0;
        let right = words.right().max(action.right());
        assert!(
            right - left <= LIST_MEASURE + 0.5 && words.height() > Type::Body.line(),
            "the message spans {left}..{right}: words {words:?}, action {action:?}"
        );
        // The same cap holds for the Undo message (the message with links is gone: its last
        // caller, Campaign content, moved to the typed footer).
        {
            let links = false;
            let mut harness = egui_kittest::Harness::builder()
                .with_size(vec2(1800.0, 240.0))
                .themed_ui(move |ui| {
                    undo_message(ui, Tone::Success, LONG, "Undo");
                });
            harness.run_steps(2);
            let texts = painted_texts(&harness);
            let right = texts
                .iter()
                .map(|(_, rect)| rect.right())
                .fold(0.0, f32::max);
            let left = texts
                .iter()
                .map(|(_, rect)| rect.left())
                .fold(f32::MAX, f32::min);
            assert!(
                !texts.is_empty() && right - left <= LIST_MEASURE,
                "links {links}: the message spans {left}..{right}"
            );
        }
    }

    /// The baselines of the label and of the value's first word, as painted.
    fn baselines(row: impl Fn(&mut Ui), label: &str, first_word: &str) -> (f32, f32) {
        let context = egui::Context::default();
        theme::install(&context);
        let mut found = (None, None);
        // The second pass draws with the first pass's layout memory, as the app does.
        for _ in 0..2 {
            let output = context.run(egui::RawInput::default(), |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| row(ui));
            });
            found = (None, None);
            for shape in &output.shapes {
                let egui::Shape::Text(text) = &shape.shape else {
                    continue;
                };
                for row in &text.galley.rows {
                    let words: String = row.glyphs.iter().map(|glyph| glyph.chr).collect();
                    let baseline = |at: usize| text.pos.y + row.pos.y + row.glyphs[at].pos.y;
                    if words == label && found.0.is_none() {
                        found.0 = Some(baseline(0));
                    } else if let Some(at) = words.find(first_word)
                        && found.1.is_none()
                    {
                        found.1 = Some(baseline(words[..at].chars().count()));
                    }
                }
            }
        }
        (
            found.0.expect("the label is painted"),
            found.1.expect("the first value word is painted"),
        )
    }

    #[test]
    fn a_linked_row_label_shares_its_first_value_baseline() {
        let skills = format!(
            "{}, {}",
            RollLink::d20(D20::Skill, 11, "Perception").markdown("Perception +11"),
            RollLink::d20(D20::Skill, 3, "Stealth").markdown("Stealth +3"),
        );
        let (label, value) = baselines(
            |ui| {
                linked_detail(ui, "Skills", |ui| {
                    crate::markdown_view::show_compact(ui, &skills);
                });
            },
            "Skills",
            "Perception",
        );
        assert!(
            (label - value).abs() <= 0.5,
            "the Skills label baseline {label} differs from Perception's {value}"
        );
    }

    #[test]
    fn words_far_below_the_row_do_not_pull_the_label_down() {
        // A value whose first painted words start more than a control height below the row's
        // top, as a scrolled or wrapped value can: the label stays on its own line.
        let (label, value) = baselines(
            |ui| {
                linked_detail(ui, "Skills", |ui| {
                    ui.vertical(|ui| {
                        space(ui, Density::Read.control() * 3.0);
                        ui.label(Type::Stat.rich(ui, "Perception +11"));
                    });
                });
            },
            "Skills",
            "Perception",
        );
        assert!(
            value - label > Density::Read.control() * 2.0,
            "the Skills label {label} followed words {value} that are not on its line"
        );
    }

    #[test]
    fn a_plain_row_label_shares_its_value_baseline() {
        let (label, value) = baselines(
            |ui| detail(ui, "Senses", "Darkvision 60 ft., Passive Perception 15"),
            "Senses",
            "Darkvision",
        );
        assert!(
            (label - value).abs() <= 0.5,
            "the Senses label baseline {label} differs from its value's {value}"
        );
    }

    #[test]
    fn a_rolling_tile_matches_a_stat_tiles_label_and_value() {
        use crate::design::test_harness::ThemedPage as _;
        // The rolling tile has the stat tile's label line and value type; only its value rolls.
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(400.0, 200.0))
            .themed_ui(|ui| {
                ui.horizontal(|ui| {
                    stat_tile(ui, &fact::tile_label("Str", 8), "−1");
                    let link = RollLink::d20(D20::Check, 2, "Dexterity");
                    let _ = fact::roll_tile(ui, &fact::tile_label("Dex", 14), "+2", &link);
                });
            });
        harness.run();
        let style = |words: &str| {
            harness
                .output()
                .shapes
                .iter()
                .find_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) if text.galley.job.text == words => {
                        let format = &text.galley.job.sections[0].format;
                        Some((format.font_id.clone(), text.fallback_color))
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("{words} is painted"))
        };
        assert_eq!(style("Str: 8"), style("Dex: 14"), "one label line");
        assert_eq!(style("−1").0, style("+2").0, "one value type");
    }
}
