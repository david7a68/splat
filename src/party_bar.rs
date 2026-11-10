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
    /// The portrait goes after the optional stats and before AC, so that words name each item.
    portrait: bool,
    /// The name drawn: the first name, its initial, or none (see [`Names`]).
    name: Option<Arc<egui::Galley>>,
    /// The first name, so that the DM need not know each portrait.
    first: Option<Arc<egui::Galley>>,
    /// The first name's initial (`B.`), when even first names do not fit. Hover and the
    /// accessible name keep the full name.
    initial: Option<Arc<egui::Galley>>,
    hp_width: f32,
    temp: Option<Arc<egui::Galley>>,
    /// The HP state's icon and word, or the icon alone (see [`Step`]).
    word: Option<Arc<egui::Galley>>,
    full_word: Option<Arc<egui::Galley>>,
    icon_word: Option<Arc<egui::Galley>>,
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
        let state = design::hp_word(glance.hp, glance.max_hp);
        let word = state.map(|(icon, word, color)| {
            painter.layout_no_wrap(format!("{icon} {word}"), small.clone(), color)
        });
        let icon_word = state
            .map(|(icon, _, color)| painter.layout_no_wrap(icon.to_owned(), small.clone(), color));
        let first = glance.name.split_whitespace().next().unwrap_or_default();
        let name = (!first.is_empty())
            .then(|| painter.layout_no_wrap(first.to_owned(), small.clone(), c.text));
        let initial = first
            .chars()
            .next()
            .map(|letter| painter.layout_no_wrap(format!("{letter}."), small.clone(), c.text));
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
            portrait: true,
            name: name.clone(),
            first: name,
            initial,
            hp_width,
            temp,
            word: word.clone(),
            full_word: word,
            icon_word,
            stats,
        }
    }

    /// The width with the first `shown` stats.
    fn width(&self, shown: usize) -> f32 {
        design::XS
            + if self.portrait {
                PORTRAIT + design::XS
            } else {
                0.0
            }
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

/// Which name each item draws.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Names {
    First,
    Initial,
}

/// One way to draw the bar: how many stats, with or without portraits, which names, and
/// whether the HP state keeps its word.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Step {
    shown: usize,
    portrait: bool,
    names: Names,
    state_word: bool,
}

/// From the fullest bar down. The source gives way first, and stays on hover. Then the optional
/// stats (Per, Ins, DC) drop from the end, down to AC. Then the portraits go together, then AC.
/// HP and temporary HP always stay. Only when first names and HP do not fit do the names
/// shorten to an initial (`B.`). If that does not fit, initials stay and the HP state keeps its
/// icon and colour without its word. No item is ever cut off: when even that does not fit, the
/// items that fit show whole and `N more` names the rest. Every item shows the same parts, and
/// hover and the accessible name keep everything.
fn steps(most: usize) -> Vec<Step> {
    let step = |shown, portrait, names, state_word| Step {
        shown,
        portrait,
        names,
        state_word,
    };
    let mut steps: Vec<Step> = (most.min(1)..=most)
        .rev()
        .map(|shown| step(shown, true, Names::First, true))
        .collect();
    steps.extend([
        step(most.min(1), false, Names::First, true),
        step(0, false, Names::First, true),
        step(0, false, Names::Initial, true),
        step(0, false, Names::Initial, false),
    ]);
    steps
}

/// Draw every item as `step` says.
fn set(items: &mut [Item], step: Step) {
    for item in items {
        item.portrait = step.portrait;
        item.name = match step.names {
            Names::First => item.first.clone(),
            Names::Initial => item.initial.clone(),
        };
        item.word = if step.state_word {
            item.full_word.clone()
        } else {
            item.icon_word.clone()
        };
    }
}

/// The width of every item drawn as `step` says, and the gaps between them.
fn needed(items: &mut [Item], step: Step, gap: f32) -> f32 {
    set(items, step);
    items.iter().map(|item| item.width(step.shown)).sum::<f32>()
        + gap * items.len().saturating_sub(1) as f32
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
    let room =
        ui.available_width() - label.size().x - design::L - Density::Read.control() - design::S;
    let with_source = room - source_width - design::L;
    let steps = steps(items.iter().map(|item| item.stats.len()).max().unwrap_or(0));
    let show_source = needed(&mut items, steps[0], gap) <= with_source;
    let room = if show_source { with_source } else { room };
    let step = steps
        .iter()
        .copied()
        .find(|&step| needed(&mut items, step, gap) <= room)
        .unwrap_or_else(|| {
            // Below every step, the narrowest one, so that the most items show whole.
            steps
                .iter()
                .copied()
                .min_by(|a, b| needed(&mut items, *a, gap).total_cmp(&needed(&mut items, *b, gap)))
                .unwrap_or(steps[0])
        });
    set(&mut items, step);
    for item in &mut items {
        item.stats.truncate(step.shown);
    }
    // No item is ever cut off. When even the narrowest step does not fit, the items that fit
    // show whole, and `N more` names the rest on hover and in its accessible name.
    let (painter, small) = (ui.painter().clone(), Type::Small.font(ui));
    let more = |hidden: usize| {
        painter.layout_no_wrap(format!("{hidden} more"), small.clone(), colors().muted)
    };
    let widths: Vec<f32> = items.iter().map(|item| item.width(step.shown)).collect();
    let kept = (0..=items.len())
        .rev()
        .find(|&kept| {
            let shown = widths[..kept].iter().sum::<f32>() + gap * kept.saturating_sub(1) as f32;
            let hidden = items.len() - kept;
            let count = if hidden == 0 {
                0.0
            } else {
                gap + more(hidden).size().x + 2.0 * design::XS
            };
            shown + count <= room
        })
        .unwrap_or(0);
    items.truncate(kept);
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
                let hidden = &glances[kept..];
                if !hidden.is_empty() {
                    more_players(ui, more(hidden.len()), hidden);
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

/// The players that do not fit, as a muted count. It takes keyboard focus. Hover, focus and the
/// accessible name list each one with HP.
fn more_players(ui: &mut egui::Ui, count: Arc<egui::Galley>, hidden: &[Glance]) {
    let c = colors();
    let size = count.size() + vec2(2.0 * design::XS, 0.0);
    let (rect, response) = ui.allocate_exact_size(
        vec2(size.x, height()),
        egui::Sense::focusable_noninteractive(),
    );
    ui.painter().galley(
        egui::pos2(
            rect.left() + design::XS,
            rect.center().y - count.size().y / 2.0,
        ),
        count,
        c.muted,
    );
    if response.has_focus() {
        ui.painter().rect_stroke(
            rect,
            design::RADIUS_CONTROL,
            egui::Stroke::new(2.0, c.focus),
            egui::StrokeKind::Inside,
        );
    }
    let hp = |glance: &Glance| glance.facts().into_iter().next().unwrap_or_default();
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            true,
            format!(
                "{} more: {}",
                hidden.len(),
                hidden
                    .iter()
                    .map(|glance| format!("{} · {}", glance.name, hp(glance)))
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        )
    });
    let list = |ui: &mut egui::Ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for glance in hidden {
            ui.label(Type::Strong.rich(ui, &glance.name).color(c.text));
            ui.label(Type::Small.rich(ui, hp(glance)).color(c.muted));
        }
    };
    if response.has_focus() {
        response.show_tooltip_ui(list);
    } else {
        response.on_hover_ui(list);
    }
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
    if item.portrait {
        let picture =
            egui::Rect::from_min_size(pos2(x, y - PORTRAIT / 2.0), egui::Vec2::splat(PORTRAIT));
        crate::art::portrait(ui, picture, glance.portrait.as_deref(), &glance.name, None);
        x = picture.right() + design::XS;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    const NAMES: [&str; 6] = [
        "Brannoch Ironhide",
        "Iselle Varn",
        "Kestrel Moor",
        "Oren Thistlewick",
        "Sable Quill",
        "Wren Ashdown",
    ];

    /// `count` players. The first `bloodied` are at 11/41; the last `temp` have 5 temporary HP.
    fn party(count: usize, bloodied: usize, temp: usize) -> Vec<Glance> {
        NAMES[..count]
            .iter()
            .enumerate()
            .map(|(at, name)| Glance {
                id: at.to_string(),
                name: (*name).to_owned(),
                portrait: None,
                hp: Some(if at < bloodied { 11 } else { 41 }),
                max_hp: Some(41),
                temp: if at + temp >= count { 5 } else { 0 },
                ac: Some(15),
                perception: Some(14),
                insight: Some(11),
                investigation: Some(10),
                spell_dc: Some(13),
                conditions: Vec::new(),
            })
            .collect()
    }

    /// What one bar width draws.
    #[derive(Debug, PartialEq)]
    struct Drawn {
        /// Items that paint their whole first name, and items that paint its initial.
        firsts: usize,
        initials: usize,
        portraits: usize,
        /// Items whose accessible name is the full name, then HP.
        spoken: usize,
        ac: usize,
        per: usize,
        /// Items that paint the word `Bloodied` beside its icon.
        bloodied: usize,
        /// Items that paint the Bloodied state: its icon alone, or its icon and word.
        states: usize,
        /// Each item's rect, and the rect of Hide the party at the bar's right edge.
        items: Vec<egui::Rect>,
        close: egui::Rect,
        /// The accessible name of `N more` and its rect, when some players do not fit.
        more: Option<String>,
        more_rect: Option<egui::Rect>,
        /// Painted shapes that reach past their clip rect: what the DM would see cut off.
        cut: Vec<String>,
    }

    impl Drawn {
        /// Nothing painted reaches past its clip rect (the room box, which ends before the source
        /// status when it shows). Every item and `N more` lie before Hide the party, and Hide the
        /// party is in the window.
        fn whole(&self, width: f32) -> bool {
            self.cut.is_empty()
                && self.close.right() <= width
                && self
                    .items
                    .iter()
                    .chain(&self.more_rect)
                    .all(|item| item.left() >= 0.0 && item.right() <= self.close.left())
        }

        fn parts(&self) -> (usize, usize, usize, usize, usize, usize) {
            (
                self.firsts,
                self.initials,
                self.portraits,
                self.ac,
                self.per,
                self.bloodied,
            )
        }
    }

    fn draw(width: f32, glances: &[Glance]) -> Drawn {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        ctx.enable_accesskit();
        let source = Source {
            icon: icons::OK,
            word: "From tonight".to_owned(),
            color: colors().muted,
        };
        let output = ctx.run(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(width, 200.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show(ui, glances, &source);
                });
            },
        );
        fn walk(shape: &egui::Shape, texts: &mut Vec<String>, circles: &mut usize) {
            match shape {
                egui::Shape::Vec(shapes) => {
                    shapes.iter().for_each(|shape| walk(shape, texts, circles));
                }
                egui::Shape::Text(text) => texts.push(text.galley.job.text.clone()),
                egui::Shape::Circle(_) => *circles += 1,
                _ => {}
            }
        }
        let (mut texts, mut circles, mut cut) = (Vec::new(), 0, Vec::new());
        for clipped in &output.shapes {
            walk(&clipped.shape, &mut texts, &mut circles);
            let bounds = clipped.shape.visual_bounding_rect();
            if bounds.is_positive() && !clipped.clip_rect.expand(0.5).contains_rect(bounds) {
                cut.push(format!("{bounds:?} past {:?}", clipped.clip_rect));
            }
        }
        let first = |glance: &Glance| glance.name.split_whitespace().next().unwrap().to_owned();
        let painted = |word: String| texts.iter().filter(|text| **text == word).count();
        let nodes = output.platform_output.accesskit_update.unwrap().nodes;
        let rect = |node: &egui::accesskit::Node| {
            let b = node.bounds().expect("bounds");
            egui::Rect::from_min_max(
                egui::pos2(b.x0 as f32, b.y0 as f32),
                egui::pos2(b.x1 as f32, b.y1 as f32),
            )
        };
        let items: Vec<_> = glances
            .iter()
            .filter_map(|glance| {
                nodes
                    .iter()
                    .find(|(_, node)| {
                        node.label().is_some_and(|label| {
                            label.starts_with(&format!("{} · HP ", glance.name))
                        })
                    })
                    .map(|(_, node)| rect(node))
            })
            .collect();
        let stat = |label: &str| texts.iter().filter(|text| text.starts_with(label)).count();
        Drawn {
            firsts: glances.iter().map(|g| painted(first(g))).sum(),
            initials: glances
                .iter()
                .map(|g| painted(format!("{}.", first(g).chars().next().unwrap())))
                .sum(),
            portraits: circles,
            spoken: items.len(),
            ac: stat("AC "),
            per: stat("Per "),
            bloodied: texts
                .iter()
                .filter(|text| text.ends_with(" Bloodied"))
                .count(),
            states: texts
                .iter()
                .filter(|text| *text == icons::BLOODIED || text.ends_with(" Bloodied"))
                .count(),
            items,
            more: nodes
                .iter()
                .filter_map(|(_, node)| node.label().or_else(|| node.value()))
                .map(|words| words.to_string())
                .find(|words| words.contains(" more: ")),
            more_rect: nodes
                .iter()
                .find(|(_, node)| {
                    node.label()
                        .or_else(|| node.value())
                        .is_some_and(|words| words.contains(" more: "))
                })
                .map(|(_, node)| rect(node)),
            cut,
            close: nodes
                .iter()
                .find(|(_, node)| node.label() == Some("Hide the party"))
                .map(|(_, node)| rect(node))
                .expect("Hide the party"),
        }
    }

    /// `N more` is in the tab order. Focus on it shows the same list as hover: each hidden player's
    /// full name.
    #[test]
    fn n_more_takes_keyboard_focus_and_shows_the_hidden_players() {
        let ctx = egui::Context::default();
        design::theme::install(&ctx);
        ctx.enable_accesskit();
        let glances = party(6, 6, 6);
        let source = Source {
            icon: icons::OK,
            word: "From tonight".to_owned(),
            color: colors().muted,
        };
        let frame = |events: Vec<egui::Event>| {
            ctx.run(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        vec2(500.0, 200.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        show(ui, &glances, &source);
                    });
                },
            )
        };
        let tab = egui::Event::Key {
            key: egui::Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        let mut output = frame(Vec::new());
        let words = |output: &egui::FullOutput, id| {
            output
                .platform_output
                .accesskit_update
                .as_ref()?
                .nodes
                .iter()
                .find(|(node, _)| *node == id)
                .and_then(|(_, node)| node.label().or_else(|| node.value()))
                .map(str::to_owned)
        };
        // The number of hidden players, when focus is on `N more`.
        let focused_more = |output: &egui::FullOutput| {
            output
                .platform_output
                .accesskit_update
                .as_ref()
                .and_then(|update| words(output, update.focus))
                .filter(|words| words.contains(" more: "))
                .and_then(|words| words.split_whitespace().next()?.parse::<usize>().ok())
        };
        let more = |output: &egui::FullOutput| focused_more(output).is_some();
        let mut presses = 0;
        while !more(&output) && presses < 12 {
            output = frame(vec![tab.clone()]);
            presses += 1;
        }
        let hidden = focused_more(&output).expect("Tab reaches N more within 12 presses");
        assert!(hidden > 0 && hidden < NAMES.len(), "{hidden} hidden");
        // The list shows while focus stays.
        output = frame(Vec::new());
        output = if more(&output) {
            frame(Vec::new())
        } else {
            output
        };
        let mut texts = Vec::new();
        fn walk(shape: &egui::Shape, texts: &mut Vec<String>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, texts)),
                egui::Shape::Text(text) => texts.push(text.galley.job.text.clone()),
                _ => {}
            }
        }
        for clipped in &output.shapes {
            walk(&clipped.shape, &mut texts);
        }
        for name in &NAMES[NAMES.len() - hidden..] {
            assert!(
                texts.iter().any(|text| text == name),
                "focus shows {name}: {texts:?}"
            );
        }
    }

    /// FT3 and its repair: the optional stats drop first, then the portraits go together, then
    /// AC. Then first names shorten to an initial, then the state word to its icon. No item is
    /// ever cut off: at every width tested, from 300 px to 2000 px, each item lies whole before
    /// Hide the party, and the players that do not fit are named by `N more`.
    #[test]
    fn no_item_is_cut_off_and_ac_stays_until_the_portraits_have_gone() {
        for (count, bloodied, temp) in [(5, 3, 2), (5, 5, 5), (6, 1, 6), (6, 6, 6), (5, 0, 0)] {
            let glances = party(count, bloodied, temp);
            let drawn: Vec<_> = (30..=200)
                .map(|step| (step * 10, draw(step as f32 * 10.0, &glances)))
                .collect();
            let case = format!("{count} players, {bloodied} Bloodied, {temp} with temporary HP");
            for (width, at) in &drawn {
                assert!(
                    at.whole(*width as f32),
                    "{case}: cut off at {width}: {at:?}"
                );
                // Every player is named: by its item, or by `N more`.
                let hidden: Vec<_> = glances[at.spoken..].iter().map(|g| &g.name).collect();
                assert!(
                    at.spoken > 0
                        && (hidden.is_empty() && at.more.is_none()
                            || at.more.as_ref().is_some_and(|more| {
                                more.starts_with(&format!("{} more: ", hidden.len()))
                                    && hidden
                                        .iter()
                                        .all(|name| more.contains(&format!("{name} · HP ")))
                            })),
                    "{case}: {hidden:?} at {width}: {at:?}"
                );
                let n = at.spoken;
                // Each shown player at 11/41 keeps the Bloodied state: its icon, with or without
                // its word.
                assert_eq!(
                    at.states,
                    bloodied.min(n),
                    "{case}: the Bloodied state stays at {width}: {at:?}"
                );
                let (firsts, initials, portraits, ac, per, _) = at.parts();
                assert!(
                    [0, n].contains(&firsts)
                        && [0, n].contains(&initials)
                        && [0, n].contains(&portraits)
                        && [0, n].contains(&ac)
                        && [0, n].contains(&per),
                    "{case}: the parts go together, never item by item, at {width}: {at:?}"
                );
                assert!(
                    firsts + initials == n,
                    "{case}: names stay at {width}: {at:?}"
                );
                assert!(
                    portraits == 0 || ac == n,
                    "{case}: AC before portraits at {width}"
                );
                assert!(
                    per == 0 || portraits == n,
                    "{case}: portraits before Per at {width}"
                );
                assert!(
                    initials == 0 || ac == 0,
                    "{case}: AC before first names at {width}"
                );
            }
            assert_eq!(
                drawn.last().unwrap().1.parts(),
                (count, 0, count, count, count, bloodied),
                "{case}: a wide bar shows everything"
            );
            assert!(
                drawn[0].1.spoken < count,
                "{case}: the narrowest bar keeps whole items and counts the rest"
            );
        }
        // Every step of the order is reached, narrow to wide, for 3 Bloodied and 2 with temp HP.
        let glances = party(5, 3, 2);
        let drawn: Vec<_> = (30..=200)
            .map(|step| draw(step as f32 * 10.0, &glances).parts())
            .collect();
        let order = [
            (0, 5, 0, 0, 0, 0),
            (0, 5, 0, 0, 0, 3),
            (5, 0, 0, 0, 0, 3),
            (5, 0, 0, 5, 0, 3),
            (5, 0, 5, 5, 0, 3),
            (5, 0, 5, 5, 5, 3),
        ];
        let reached: Vec<_> = order
            .iter()
            .map(|step| drawn.iter().position(|at| at == step))
            .collect();
        assert!(
            reached.iter().all(Option::is_some) && reached.is_sorted(),
            "each step shows at some width, narrow to wide: {reached:?} {drawn:?}"
        );
    }
}
