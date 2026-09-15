pub mod auth;
mod campaigns;
mod credentials;
mod dndbeyond;
mod persistence;
mod settings;

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
    settings: settings::Settings,
    campaigns: campaigns::Campaigns,
    dndbeyond: dndbeyond::DndBeyond,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum Tab {
    #[default]
    Party,
    Encounters,
    Creatures,
    Simulations,
    Settings,
}

impl eframe::App for Splat {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        self.settings.poll(ctx);
        let (epoch, session) = self.settings.campaign_session();
        self.dndbeyond.poll(ctx, epoch, session.is_some());
        egui::SidePanel::left("navigation").show(ctx, |ui| {
            ui.heading("Splat");
            ui.label("D&D 2024");
            self.campaigns.selector(ui);
            ui.separator();
            for (tab, label) in [
                (Tab::Party, "Party"),
                (Tab::Encounters, "Encounters"),
                (Tab::Creatures, "Creatures"),
                (Tab::Simulations, "Simulations"),
            ] {
                ui.selectable_value(&mut self.tab, tab, label);
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.selectable_value(&mut self.tab, Tab::Settings, "⚙ Settings");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.tab == Tab::Settings {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.settings.show(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    self.campaigns.show(ui, session, &mut self.dndbeyond);
                });
                return;
            }
            let (title, message) = match self.tab {
                Tab::Settings => unreachable!(),
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
