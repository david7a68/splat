//! Shared reading components. Keep library variants on one visual scale.
use eframe::egui::{self, Color32, RichText};

pub const ACCENT: Color32 = Color32::from_rgb(100, 199, 194);
pub const SECTION_SIZE: f32 = 20.0;

pub fn section_heading(ui: &mut egui::Ui, title: &str) {
    ui.add_space(10.0);
    ui.label(
        RichText::new(title)
            .size(SECTION_SIZE)
            .color(ACCENT)
            .strong(),
    );
    ui.separator();
}

pub fn card(ui: &mut egui::Ui, margin: i8, contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .corner_radius(6.0)
        .inner_margin(margin)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            contents(ui);
        });
}

pub fn facts(ui: &mut egui::Ui, facts: &[(String, String)]) {
    let columns = if ui.available_width() < 480.0 { 2 } else { 3 };
    for row in facts.chunks(columns) {
        let width = (ui.available_width() - ui.spacing().item_spacing.x * (row.len() - 1) as f32)
            / row.len() as f32
            - 20.0;
        let height = row
            .iter()
            .map(|(_, value)| {
                ui.painter()
                    .layout(
                        value.clone(),
                        egui::FontId::proportional(20.0),
                        ui.visuals().text_color(),
                        width.max(1.0),
                    )
                    .size()
                    .y
            })
            .fold(0.0_f32, f32::max)
            + 14.0
            + ui.spacing().item_spacing.y;
        ui.columns(row.len(), |columns| {
            for (column, (label, value)) in columns.iter_mut().zip(row) {
                card(column, 10, |ui| {
                    ui.set_min_height(height);
                    ui.label(
                        RichText::new(label.to_uppercase())
                            .size(11.0)
                            .color(ACCENT)
                            .strong(),
                    );
                    ui.add(egui::Label::new(RichText::new(value).size(20.0).strong()).wrap());
                });
            }
        });
        ui.add_space(6.0);
    }
}

pub fn detail(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(human_label(label)).strong().color(ACCENT));
        ui.label(value);
    });
}

pub fn identity(ui: &mut egui::Ui, text: &str) {
    ui.label(
        RichText::new(text)
            .italics()
            .color(ui.visuals().weak_text_color()),
    );
    ui.add_space(8.0);
}

/// Scope Markdown colours to readers; editors and other workspaces retain theirs.
pub fn markdown(ui: &mut egui::Ui, text: &str, compact: bool) {
    crate::markdown_view::show_reader(ui, text, compact);
}

/// Imported rules use short named paragraphs for actions and benefits.
pub fn source_prose(ui: &mut egui::Ui, text: &str) {
    let text = text
        .split("\n\n")
        .map(|paragraph| {
            if let Some((name, rest)) = paragraph.split_once(". ")
                && name.len() < 65
                && name.starts_with(char::is_alphabetic)
                && !name.contains(['\n', '*', '|', '#'])
            {
                format!("**{name}.** {rest}")
            } else {
                paragraph.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    markdown(ui, &text, false);
}

pub fn human_label(value: &str) -> String {
    let value = value.replace('_', " ");
    let mut chars = value.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}
