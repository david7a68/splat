//! Reader and roll widgets shared by the content screens and desktop.
pub use splat_design::design::*;

pub mod reader {
    pub use crate::markdown_view::{show_compact as compact_markdown, show_reader as markdown};
    pub use splat_design::design::reader::*;

    /// Imported rules use short named paragraphs for actions and benefits.
    pub fn source_prose(ui: &mut eframe::egui::Ui, text: &str) {
        let text = text
            .split("\n\n")
            .map(run_in_markdown)
            .collect::<Vec<_>>()
            .join("\n\n");
        markdown(ui, &text);
    }
}

pub mod fact {
    pub use splat_design::design::fact::{line, tile_label, words};

    pub fn roll_tile(
        ui: &mut eframe::egui::Ui,
        label: &str,
        value: &str,
        link: &splat_content::content::dice_text::RollLink,
    ) -> eframe::egui::Response {
        splat_design::design::fact::roll_tile(ui, label, |ui| {
            crate::rolls::link(ui, value, super::Type::NumeralLarge, link)
        })
    }
}
