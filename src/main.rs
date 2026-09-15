pub mod auth;
mod campaign_content;
mod campaigns;
mod content_workspace;
mod copy_dialog;
mod credentials;
mod dndbeyond;
mod encounter_run;
mod encounter_settings;
mod encounters;
mod library;
mod markdown_view;
mod party;
mod party_stream;
mod party_sync;
mod persistence;
mod settings;

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([960.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Splat",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            style
                .text_styles
                .insert(egui::TextStyle::Heading, egui::FontId::proportional(22.0));
            style
                .text_styles
                .insert(egui::TextStyle::Body, egui::FontId::proportional(15.0));
            style
                .text_styles
                .insert(egui::TextStyle::Button, egui::FontId::proportional(15.0));
            style
                .text_styles
                .insert(egui::TextStyle::Small, egui::FontId::proportional(13.0));
            style.spacing.item_spacing = egui::vec2(8.0, 6.0);
            cc.egui_ctx.set_style(style);
            Ok(Box::<Splat>::default())
        }),
    )
}

#[derive(Default)]
struct Splat {
    tab: Tab,
    workspace: Workspace,
    confirm_close: bool,
    discard_on_close: bool,
    library: library::Library,
    campaign_content: campaign_content::CampaignContent,
    copy_dialog: Option<copy_dialog::CopyDialog>,
    settings: settings::Settings,
    campaigns: campaigns::Campaigns,
    dndbeyond: dndbeyond::DndBeyond,
    party: party::Party,
    encounters: encounters::Encounters,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum Workspace {
    #[default]
    Library,
    Campaign,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum Tab {
    #[default]
    Party,
    Content,
    Encounters,
    Creatures,
    Simulations,
    Settings,
}

impl eframe::App for Splat {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested())
            && (self.library.dirty() || self.campaign_content.dirty() || self.encounters.unsaved())
            && !self.discard_on_close
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.confirm_close = true;
        }
        if self.confirm_close {
            egui::Window::new("Unsaved changes")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(if self.encounters.has_run() {
                        "The encounter run has unsaved changes. Return to the run and save before closing, or discard the unsaved changes."
                    } else {
                        "Save your pending content changes before closing, or discard them."
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Keep editing").clicked() {
                            self.confirm_close = false;
                            self.workspace =
                                if self.campaign_content.dirty() || self.encounters.dirty() {
                                    Workspace::Campaign
                                } else {
                                    Workspace::Library
                                };
                            self.tab = if self.encounters.dirty() {
                                Tab::Encounters
                            } else if self.campaign_content.dirty() {
                                Tab::Content
                            } else {
                                Tab::Party
                            };
                        }
                        if ui.button("Discard and close").clicked() {
                            self.discard_on_close = true;
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
        self.settings.poll(ctx);
        let (epoch, session) = self.settings.campaign_session();
        self.dndbeyond.begin_frame();
        self.dndbeyond
            .poll(ctx, epoch, session.is_some(), &mut self.campaigns);
        egui::SidePanel::left("navigation").show(ctx, |ui| {
            if self.copy_dialog.is_some() {
                ui.disable();
            }
            ui.heading("Splat");
            ui.label("D&D 2024");
            ui.separator();
            if ui
                .selectable_value(&mut self.workspace, Workspace::Library, "Library")
                .clicked()
            {
                self.tab = Tab::Party;
            }
            if ui
                .selectable_value(&mut self.workspace, Workspace::Campaign, "Campaign")
                .clicked()
            {
                self.tab = Tab::Content;
            }
            if self.workspace == Workspace::Campaign {
                ui.add_enabled_ui(
                    !self.campaign_content.dirty() && !self.encounters.dirty(),
                    |ui| self.campaigns.selector(ui),
                );
                if self.campaign_content.dirty() || self.encounters.dirty() {
                    ui.weak(
                        "Save or discard content and close the encounter run to change campaigns.",
                    );
                }
                ui.separator();
                for (tab, label) in [
                    (Tab::Content, "Content"),
                    (Tab::Party, "Party"),
                    (Tab::Encounters, "Encounters"),
                    (Tab::Creatures, "Creatures"),
                    (Tab::Simulations, "Simulations"),
                ] {
                    ui.selectable_value(&mut self.tab, tab, label);
                }
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.selectable_value(&mut self.tab, Tab::Settings, "⚙ Settings");
            });
        });
        let active = self.campaigns.active_campaign();
        self.party.sync(
            ctx,
            epoch,
            active.as_ref().and_then(|c| {
                c.dndbeyond_campaign_id
                    .clone()
                    .map(|remote| (c.id, remote, c.path.clone()))
            }),
            session.is_some(),
            self.workspace == Workspace::Campaign && self.tab == Tab::Party,
        );
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.copy_dialog.is_some() {
                ui.disable();
            }
            if self.tab == Tab::Settings {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.settings.show(ui);
                    ui.add_space(20.0);
                    ui.separator();
                    if self.campaign_content.dirty() || self.encounters.dirty() {
                        ui.label(
                            "Save or discard campaign content and close the encounter run before managing campaigns.",
                        );
                    }
                    ui.add_enabled_ui(
                        !self.campaign_content.dirty() && !self.encounters.dirty(),
                        |ui| self.campaigns.show(ui, session, &mut self.dndbeyond),
                    );
                });
                return;
            }
            if self.workspace == Workspace::Library {
                self.library.show(ui, &mut self.campaigns);
                return;
            }
            if let Some(campaign) = self.campaigns.active_campaign() {
                ui.weak(format!("Campaign · {}", campaign.name));
            } else {
                ui.heading("Campaign workspace");
                ui.label("Select a campaign in the sidebar, or add one in Settings.");
                return;
            }
            if self.tab == Tab::Content {
                self.campaign_content.show(ui, &self.campaigns);
                return;
            }
            if self.tab == Tab::Encounters {
                self.encounters.show(ui, &self.campaigns);
                if self.encounters.saved {
                    self.encounters.saved = false;
                    if let Some(campaign) = &active {
                        self.campaign_content.refresh_destination(&campaign.path);
                    }
                }
                return;
            }
            if self.tab == Tab::Party {
                self.party.show(
                    ui,
                    session,
                    active.is_some_and(|c| c.dndbeyond_campaign_id.is_some()),
                );
                return;
            }
            let (title, message) = match self.tab {
                Tab::Settings | Tab::Content => unreachable!(),
                Tab::Party => ("Party", "No characters imported yet."),
                Tab::Encounters => ("Encounters", "No encounters created yet."),
                Tab::Creatures => ("Creatures", "No creatures created yet."),
                Tab::Simulations => ("Simulations", "No simulations run yet."),
            };
            ui.heading(title);
            ui.label(message);
            ui.separator();
            ui.label("Campaign content editing and simulation controls are not implemented yet.");
        });
        if let Some(source) = self
            .library
            .copy
            .take()
            .or_else(|| self.campaign_content.copy.take())
        {
            self.copy_dialog = Some(copy_dialog::CopyDialog::new(source, &self.campaigns, ctx));
        }
        if let Some(dialog) = &mut self.copy_dialog {
            let open = dialog.show(ctx, &self.campaigns);
            if let Some(root) = dialog.refresh.take() {
                self.library.refresh_destination(&root, ctx);
                self.campaign_content.refresh_destination(&root);
            }
            if !open {
                self.copy_dialog = None;
            }
        }
    }
}
