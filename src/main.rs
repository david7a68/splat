use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native("Splat", options, Box::new(|_| Ok(Box::<Splat>::default())))
}

#[derive(Default)]
struct Splat {
    tab: Tab,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum Tab {
    #[default]
    Party,
    Encounters,
    Creatures,
    Simulations,
}

impl eframe::App for Splat {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("navigation").show(ctx, |ui| {
            ui.heading("Splat");
            ui.label("D&D 2024");
            ui.separator();
            for (tab, label) in [
                (Tab::Party, "Party"),
                (Tab::Encounters, "Encounters"),
                (Tab::Creatures, "Creatures"),
                (Tab::Simulations, "Simulations"),
            ] {
                ui.selectable_value(&mut self.tab, tab, label);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (title, message) = match self.tab {
                Tab::Party => ("Party", "No characters imported yet."),
                Tab::Encounters => ("Encounters", "No encounters created yet."),
                Tab::Creatures => ("Creatures", "No creatures created yet."),
                Tab::Simulations => ("Simulations", "No simulations run yet."),
            };
            ui.heading(title);
            ui.label(message);
            ui.separator();
            ui.label("Initial application shell. Import, editing, and simulation are not implemented yet.");
        });
    }
}
