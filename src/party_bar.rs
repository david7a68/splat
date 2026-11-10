//! The party at a glance, above the Session page while a session is played. Each member is a
//! portrait, HP and a few numbers, with the parts the roster uses. The rest is on hover, and a
//! click opens the sheet.
use crate::design::{self, Density, Type, colors, icons};
use eframe::egui::{self, Color32, pos2, vec2};
use std::{path::PathBuf, sync::Arc};

/// One party member, as the bar shows them.
pub struct Glance {
    pub id: String,
    pub name: String,
    pub portrait: Option<PathBuf>,
    pub hp: Option<i64>,
    pub max_hp: Option<i64>,
    pub temp: i64,
    pub ac: Option<i64>,
    pub perception: Option<i64>,
    pub insight: Option<i64>,
    pub investigation: Option<i64>,
    /// The highest spell save DC, for a caster.
    pub spell_dc: Option<i64>,
    pub conditions: Vec<String>,
}

/// Where the numbers come from, as a status: an icon, a word and a colour.
pub struct Source {
    pub icon: &'static str,
    pub word: String,
    pub color: Color32,
}

/// The bar holds one line of items, so that what is below it does not move.
pub fn height() -> f32 {
    Density::Read.row()
}

/// Smaller than a Party row's readout, so that more numbers fit beside it.
fn hp_fonts(ui: &egui::Ui) -> design::HpFonts {
    design::HpFonts {
        current: Type::Numeral.font(ui),
        max: Type::Small.font(ui),
    }
}
const PORTRAIT: f32 = 24.0;
/// The HP bar under the readout, as on a Party row.
const BAR: f32 = 3.0;

fn known(n: Option<i64>) -> String {
    n.map_or_else(|| "?".to_owned(), |n| n.to_string())
}

impl Glance {
    /// The numbers after HP, in the order the Party list compares them. Only a caster has a DC.
    fn stats(&self) -> Vec<(&'static str, Option<i64>)> {
        let mut stats = vec![
            ("AC", self.ac),
            ("Per", self.perception),
            ("Ins", self.insight),
        ];
        if self.spell_dc.is_some() {
            stats.push(("DC", self.spell_dc));
        }
        stats
    }

    /// Everything the bar knows after the name, a fact to a line, for hover.
    fn facts(&self) -> Vec<String> {
        let mut hp = format!("HP {}/{}", known(self.hp), known(self.max_hp));
        if let Some((_, word, _)) = design::hp_word(self.hp, self.max_hp) {
            hp.push_str(&format!(" · {word}"));
        }
        if self.temp > 0 {
            hp.push_str(&format!(" · Temp HP {}", self.temp));
        }
        let mut armor = format!("AC {}", known(self.ac));
        if let Some(dc) = self.spell_dc {
            armor.push_str(&format!(" · Spell save DC {dc}"));
        }
        vec![
            hp,
            armor,
            format!(
                "Passive Perception {} · Insight {} · Investigation {}",
                known(self.perception),
                known(self.insight),
                known(self.investigation)
            ),
            if self.conditions.is_empty() {
                "No conditions".to_owned()
            } else {
                self.conditions.join(", ")
            },
        ]
    }
}

/// What the DM did with the bar this frame.
pub enum Choice {
    /// Open this member's sheet.
    Open(String),
    Hide,
}

/// One item's parts, measured once with every stat. The fit keeps the first few stats.
struct Item {
    /// The first name, so that the DM need not know each portrait. It goes after every stat.
    name: Option<Arc<egui::Galley>>,
    hp_width: f32,
    temp: Option<Arc<egui::Galley>>,
    word: Option<Arc<egui::Galley>>,
    /// Each stat as one run of text: its muted label, a space, and its value.
    stats: Vec<Arc<egui::Galley>>,
}

impl Item {
    fn measure(ui: &egui::Ui, glance: &Glance) -> Self {
        let c = colors();
        let fonts = hp_fonts(ui);
        let small = Type::Small.font(ui);
        // The same slots as `hp_readout`: the current HP, then the slash and a maximum of three digits.
        let hp_width = design::number_width(ui, &known(glance.hp), &fonts.current)
            + design::number_width(
                ui,
                &"0".repeat(3.max(known(glance.max_hp).len())),
                &fonts.max,
            );
        let painter = ui.painter();
        let temp = (glance.temp > 0).then(|| {
            painter.layout_no_wrap(design::temp_hp_text(glance.temp), small.clone(), c.hp.temp)
        });
        let word = design::hp_word(glance.hp, glance.max_hp).map(|(icon, word, color)| {
            painter.layout_no_wrap(format!("{icon} {word}"), small.clone(), color)
        });
        let first = glance.name.split_whitespace().next().unwrap_or_default();
        let name = (!first.is_empty())
            .then(|| painter.layout_no_wrap(first.to_owned(), small.clone(), c.text));
        let stats = glance
            .stats()
            .into_iter()
            .map(|(label, value)| {
                let mut job = egui::text::LayoutJob::default();
                job.append(label, 0.0, egui::TextFormat::simple(small.clone(), c.muted));
                job.append(
                    &format!(" {}", known(value)),
                    0.0,
                    egui::TextFormat::simple(small.clone(), c.text),
                );
                painter.layout_job(job)
            })
            .collect();
        Self {
            name,
            hp_width,
            temp,
            word,
            stats,
        }
    }

    /// The width with the first `shown` stats.
    fn width(&self, shown: usize) -> f32 {
        design::XS
            + PORTRAIT
            + design::XS
            + self.name.as_ref().map_or(0.0, |g| g.size().x + design::S)
            + self.hp_width
            + self.temp.as_ref().map_or(0.0, |g| design::XS + g.size().x)
            + self.word.as_ref().map_or(0.0, |g| design::S + g.size().x)
            + self
                .stats
                .iter()
                .take(shown)
                .map(|stat| design::S + stat.size().x)
                .sum::<f32>()
            + design::XS
    }
}

/// The bar's contents. It returns what the DM chose.
pub fn show(ui: &mut egui::Ui, glances: &[Glance], source: &Source) -> Option<Choice> {
    let mut choice = None;
    let gap = design::M;
    let label =
        ui.painter()
            .layout_no_wrap("Party".to_owned(), Type::Label.font(ui), colors().muted);
    let source_width = ui
        .painter()
        .layout_no_wrap(
            format!("{} {}", source.icon, source.word),
            Type::Small.font(ui),
            source.color,
        )
        .size()
        .x;
    let mut items: Vec<Item> = glances.iter().map(|g| Item::measure(ui, g)).collect();
    let needed = |shown: usize| {
        items.iter().map(|item| item.width(shown)).sum::<f32>()
            + gap * items.len().saturating_sub(1) as f32
    };
    // Stats come first. The source gives way before any of them, and stays on hover. Every item
    // shows the same number of stats, and they drop from the end.
    let room =
        ui.available_width() - label.size().x - design::L - Density::Read.control() - design::S;
    let with_source = room - source_width - design::L;
    let most = items.iter().map(|item| item.stats.len()).max().unwrap_or(0);
    let shown = (0..=most)
        .rev()
        .find(|&shown| needed(shown) <= room)
        .unwrap_or(0);
    let show_source = needed(shown) <= with_source;
    let room = if show_source { with_source } else { room };
    // Names give way last, all together, and only when no stat is left. Hover keeps them.
    let names = needed(shown) <= room;
    for item in &mut items {
        item.stats.truncate(shown);
        if !names {
            item.name = None;
        }
    }
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing.x = design::L;
        // In its own column, as `Add` is on the plan's strip.
        ui.add(egui::Label::new(label));
        ui.allocate_ui_with_layout(
            vec2(room.max(0.0), height()),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = gap;
                ui.set_clip_rect(ui.clip_rect().intersect(ui.max_rect()));
                for (glance, item) in glances.iter().zip(items) {
                    if paint(ui, glance, item, &source.word).clicked() {
                        choice = Some(Choice::Open(glance.id.clone()));
                    }
                }
            },
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = design::S;
            if design::icon_button(ui, icons::CLOSE, "Hide the party").clicked() {
                choice = Some(Choice::Hide);
            }
            if show_source {
                design::status(
                    ui,
                    design::Status {
                        icon: source.icon,
                        word: &source.word,
                        color: source.color,
                    },
                );
            }
        });
    });
    choice
}

fn paint(ui: &mut egui::Ui, glance: &Glance, item: Item, source: &str) -> egui::Response {
    let c = colors();
    let (rect, response) = ui.allocate_exact_size(
        vec2(item.width(item.stats.len()), height()),
        egui::Sense::click(),
    );
    let mut facts = glance.facts();
    facts.push(source.to_owned());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            true,
            format!("{} · {}", glance.name, facts.join(" · ")),
        )
    });
    let painter = ui.painter();
    if response.hovered() {
        painter.rect_filled(rect, design::RADIUS_CONTROL, c.hover);
    }
    if response.has_focus() {
        painter.rect_stroke(
            rect,
            design::RADIUS_CONTROL,
            egui::Stroke::new(2.0, c.focus),
            egui::StrokeKind::Inside,
        );
    }
    let y = rect.center().y;
    let mut x = rect.left() + design::XS;
    let picture =
        egui::Rect::from_min_size(pos2(x, y - PORTRAIT / 2.0), egui::Vec2::splat(PORTRAIT));
    crate::art::portrait(ui, picture, glance.portrait.as_deref(), &glance.name, None);
    x = picture.right() + design::XS;
    if let Some(name) = item.name {
        let width = name.size().x;
        painter.galley(pos2(x, y - 1.0 - name.size().y / 2.0), name, c.text);
        x += width + design::S;
    }
    // The readout shares the label's line. Its bar takes the item's bottom edge, as on a Party row.
    let line = y - 1.0;
    let right = x + item.hp_width;
    design::hp_readout(
        ui,
        painter,
        right,
        line,
        glance.hp,
        glance.max_hp,
        &hp_fonts(ui),
    );
    let bar = egui::Rect::from_min_max(pos2(x, rect.bottom() - BAR), pos2(right, rect.bottom()));
    design::hp_bar(painter, bar, glance.hp, glance.max_hp, glance.temp);
    x = right;
    let put = |x: &mut f32, before: f32, galley: Arc<egui::Galley>| {
        *x += before;
        let width = galley.size().x;
        painter.galley(pos2(*x, line - galley.size().y / 2.0), galley, c.text);
        *x += width;
    };
    if let Some(temp) = item.temp {
        put(&mut x, design::XS, temp);
    }
    if let Some(word) = item.word {
        put(&mut x, design::S, word);
    }
    for stat in item.stats {
        put(&mut x, design::S, stat);
    }
    response
        .on_hover_ui(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.label(Type::Strong.rich(ui, &glance.name).color(c.text));
            for fact in &facts {
                ui.label(Type::Small.rich(ui, fact).color(c.muted));
            }
        })
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}
