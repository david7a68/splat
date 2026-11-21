use crate::design::{self, Density, Type, colors, icons};
use eframe::egui;
/// `N content issues to repair`: the words of a page's collapsing content issue count.
pub fn content_issue_count(count: usize) -> String {
    design::count(count, "content issue to repair", "content issues to repair")
}

/// A collapsing content issue count is a status, in one look on every page: the warning icon
/// and colour, then the count, at the size of a section title.
pub fn issue_count_title(ui: &egui::Ui, heading: &str) -> egui::text::LayoutJob {
    let format = egui::TextFormat {
        font_id: egui::TextStyle::Button.resolve(ui.style()),
        color: colors().result.warning,
        ..Default::default()
    };
    let mut job = egui::text::LayoutJob::default();
    job.append(icons::WARN, 0.0, format.clone());
    job.append(" ", 0.0, format.clone());
    job.append(heading, 0.0, format);
    job
}

/// A warning's file line: the path, whole, then `Edit in Obsidian` and `Show file`. A path too
/// long to share the line with its links wraps on lines of its own above them; it is never cut.
pub fn file_line(ui: &mut egui::Ui, root: &std::path::Path, relative: &str) {
    let links = crate::world::entry_file_links_width(ui, true);
    let budget = ui.available_width() - links - design::L;
    if design::number_width(ui, relative, &Type::Small.font(ui)) <= budget {
        crate::world::entry_file_actions(ui, root, relative, true);
        return;
    }
    let path = root.join(relative);
    let response =
        ui.add(egui::Label::new(Type::Small.rich(ui, relative).color(colors().muted)).wrap());
    design::named(ui, &response, &path.display().to_string());
    response.on_hover_text(path.display().to_string());
    ui.horizontal_wrapped(|ui| crate::world::entry_file_links(ui, &path, true));
}

pub fn creature_name_link(ui: &mut egui::Ui, name: &str) -> egui::Response {
    let galley = design::one_line(
        ui,
        name,
        Type::Body.font(ui),
        colors().text,
        ui.available_width(),
    );
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(galley.size().x, Density::Read.control() - design::XS),
        egui::Sense::click(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Link, ui.is_enabled(), name));
    if ui.is_rect_visible(rect) {
        let y = rect.center().y - galley.size().y / 2.0;
        if response.hovered() {
            ui.painter().hline(
                rect.x_range(),
                y + galley.size().y - design::HAIRLINE_WIDTH,
                egui::Stroke::new(design::HAIRLINE_WIDTH, colors().text),
            );
        }
        ui.painter()
            .galley(egui::pos2(rect.left(), y), galley, colors().text);
        if response.has_focus() {
            design::focus_ring(ui, rect);
        }
    }
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

pub fn draft_switch_reason() -> &'static str {
    "Save or discard to switch campaign."
}

/// The campaign switch lock of a draft whose Save is blocked: Discard is the one way out, so
/// the reason does not name Save (round 25 FB6).
pub fn blocked_draft_switch_reason() -> &'static str {
    "Discard to switch campaign."
}
