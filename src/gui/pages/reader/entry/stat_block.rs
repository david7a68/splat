//! A creature's stat block (egui `library::creature`): identity, vitals, the ability table with
//! its dice cells, the detail rows, then the rules prose with its actions as tiles and its
//! Spellcasting lists as spell tiles.
use super::parts::{self, detail_row, heavy_rule, identity, item, stat_detail};
use super::tiles::{self, Press};
use super::{Out, SpellLinks};
use crate::design::reader::human_label;
use crate::gui::kit;
use crate::gui::theme::{HAIRLINE_WIDTH, L, S, Type, TypeExt, XL, XS, colors};
use crate::library::creature::{self, ABILITIES, ABILITY_CELLS, DETAILS, SAVE_CELL, stat, value};
use crate::library::presentation::ReadingSection;
use crate::library::view::RunValues;
use crate::rolls::plain;
use gpui::{
    AnyElement, InteractiveElement, IntoElement, ParentElement, Pixels, StatefulInteractiveElement,
    Styled, div, prelude::*, px,
};
use kit::WithTooltip;
use splat::content::Entry;
use splat::content::dice_text::{self, D20, RollLink};

/// The ability table's row height: a pointer target, as its numbers roll.
const ABILITY_ROW: Pixels = px(24.0);

/// A styled stat block: its head, then each section's rules prose.
pub(super) fn show(
    out: &mut Out<'_>,
    entry: &Entry,
    sections: &[ReadingSection<'_>],
    values: Option<&RunValues>,
    spells: &mut SpellLinks<'_>,
) {
    stat_head(out, entry, values);
    // A styled stat block's first heading is Markdown's after this gap.
    out.push(kit::gap(S));
    for section in sections {
        if section.title == "Source text" {
            prose(
                out,
                &entry.name,
                &entry.body,
                &creature::rules_prose(entry, section.body),
                values,
                Some(spells),
            );
        } else {
            let body = if section.body.contains("<!-- splat:") {
                creature::authored_prose(section.body)
            } else {
                section.body.to_owned()
            };
            if body.trim().is_empty() {
                continue;
            }
            if section.title != "Overview" {
                out.heading(section.title.clone());
            }
            prose(out, &entry.name, &entry.body, &body, values, Some(spells));
        }
    }
}

/// Rules text written by hand: its actions as tiles, the rest as prose.
pub(super) fn rules_text(out: &mut Out<'_>, name: &str, body: &str, text: &str) {
    prose(out, name, body, text, None, None);
}

/// `text` is one part of the entry's `body`. A tile's number is its action's place in the whole
/// body's list, which is the list Ctrl+number arms from. With no `spells`, a list of spells is
/// prose.
fn prose(
    out: &mut Out<'_>,
    name: &str,
    body: &str,
    text: &str,
    values: Option<&RunValues>,
    mut spells: Option<&mut SpellLinks<'_>>,
) {
    let all = plain::actions_of(body);
    let here = plain::actions_of(text);
    let mut tiles: Vec<(Option<usize>, crate::rolls::Chip)> = Vec::new();
    let mut spellcasting: Option<String> = None;
    let flush = |out: &mut Out<'_>, tiles: &mut Vec<(Option<usize>, crate::rolls::Chip)>| {
        if tiles.is_empty() {
            return;
        }
        let views = out
            .ctx
            .as_ref()
            .map(|ctx| plain::roll_tiles(ctx, name, tiles))
            .unwrap_or_default();
        let presses: Vec<_> = views
            .into_iter()
            .zip(tiles.iter())
            .map(|(view, (_, chip))| {
                (
                    view,
                    Press::Roll {
                        link: chip.link.clone(),
                        occurrence: chip.spell_occurrence,
                    },
                )
            })
            .collect();
        let chips: Vec<_> = tiles.iter().map(|(_, chip)| chip.clone()).collect();
        let id = out.next_id("tiles");
        let row = tiles::tile_row(
            &id,
            out.ctx.as_ref(),
            name,
            presses,
            &chips,
            out.on_open.clone(),
        );
        out.push(row);
        tiles.clear();
    };
    for (at, paragraph) in text
        .split("\n\n")
        .map(str::trim)
        .enumerate()
        .filter(|(_, s)| !s.is_empty())
    {
        // A Spellcasting paragraph's lists follow it, each spell a link.
        if let Some(mut group) = spellcasting.take() {
            if crate::library::spells::aside(paragraph) {
                spellcasting = Some(group);
                continue;
            }
            if paragraph.lines().all(crate::library::spells::list_line) {
                let words = group.split("\n\n").next().unwrap_or_default().to_owned();
                match spells.as_deref_mut() {
                    Some(spells) => spells.list(out, &words, paragraph),
                    None => {
                        out.prose(paragraph);
                    }
                }
                group.push_str("\n\n");
                group.push_str(paragraph);
                spellcasting = Some(group);
                continue;
            }
            if let Some(spells) = spells.as_deref_mut() {
                spells.rest(out, &group);
            }
        }
        // A paragraph with an attack, a save or damage is an action tile; its text is in hover.
        if let Some((_, chip)) = here.iter().find(|(place, _)| *place == at) {
            tiles.push((plain::place_of(&all, chip), chip.clone()));
            continue;
        }
        flush(out, &mut tiles);
        if let Some(speed) = values.and_then(|values| values.speed.as_ref())
            && paragraph.lines().any(|line| line.starts_with("Speed "))
        {
            for line in paragraph.lines() {
                if let Some(printed) = line.strip_prefix("Speed ") {
                    changed_speed(out, printed, speed);
                } else {
                    out.prose(line);
                }
            }
        } else {
            out.prose(paragraph);
        }
        if crate::library::spells::spellcasting(paragraph) {
            spellcasting = Some(paragraph.to_owned());
        } else if crate::library::spells::cast_names(paragraph).is_some()
            && let Some(spells) = spells.as_deref_mut()
        {
            spells.casts(out, paragraph);
        }
    }
    if let Some(group) = spellcasting
        && let Some(spells) = spells
    {
        spells.rest(out, &group);
    }
    flush(out, &mut tiles);
}

/// A Speed row whose run value differs from the printed one, in the changed colour.
fn changed_speed(out: &mut Out<'_>, printed: &str, speed: &str) {
    let preview = splat_content_ui::roster::change_preview(&splat::content::overrides::Row {
        label: "Speed",
        base: printed.into(),
        changed: speed.into(),
        by_hand: true,
        in_rolls: false,
    });
    let row = stat_detail("Speed", preview, colors().participant.changed, out.window);
    out.push(row);
}

/// One cell of the ability table: numbers stand on their right edge, so a column reads down.
fn cell(width: f32, text: &str, role: Type, color: gpui::Hsla) -> AnyElement {
    let numeric = text.starts_with(|c: char| c.is_ascii_digit() || "+\u{2212}\u{2014}".contains(c));
    div()
        .w(px(width))
        .h(ABILITY_ROW)
        .flex_none()
        .flex()
        .items_center()
        .when(numeric, |cell| cell.justify_end())
        .ty(role)
        .text_color(color)
        .whitespace_nowrap()
        .child(text.to_owned())
        .into_any_element()
}

/// A right-aligned number in the ability table that rolls.
fn link_cell(out: &Out<'_>, width: f32, text: &str, link: &RollLink) -> AnyElement {
    let c = colors();
    let ctx = out.ctx.clone();
    let pressed = link.clone();
    let id = parts::child(&out.id, format!("dice-{}", link.destination()));
    let element = div()
        .id(id)
        .w(px(width))
        .h(ABILITY_ROW)
        .flex_none()
        .flex()
        .items_center()
        .justify_end()
        .cursor_pointer()
        .ty(Type::Stat)
        .text_color(c.accent)
        .hover(move |s| s.text_color(c.focus))
        .whitespace_nowrap()
        .aria_label(crate::rolls::hint(link))
        .child(
            div()
                .border_b(HAIRLINE_WIDTH)
                .border_color(c.accent.opacity(0.6))
                .child(text.to_owned()),
        )
        .with_tooltip(crate::rolls::hint(link))
        .on_click(move |_, window, _| {
            if let Some(ctx) = &ctx {
                let keys = window.modifiers();
                crate::rolls::ask_pressed(ctx, &pressed, keys.shift, keys.control || keys.platform);
                window.refresh();
            }
        });
    kit::target(element, format!("{} {text}", dice_name(link))).into_any_element()
}

/// The name of a dice cell for tests and the harness (`Strength +6`).
fn dice_name(link: &RollLink) -> String {
    crate::rolls::link_label(link).to_owned()
}

/// One ability row: label, score, the modifier that rolls a check, and the save.
fn ability_row(out: &Out<'_>, entry: &Entry, ability: &str, saves: bool) -> AnyElement {
    let c = colors();
    let modifier = stat(entry, ability).map(|score| (score - 10).div_euclid(2));
    // An absent override does not establish proficiency or a save total.
    let save = stat(entry, &format!("save_{ability}"));
    let [label, score, modifier_cell] = ABILITY_CELLS;
    let name = dice_text::ability(ability).unwrap_or(ability);
    let mut row = div()
        .flex()
        .flex_row()
        .gap(XS)
        .child(cell(
            label,
            &ability[..3].to_uppercase(),
            Type::StatLabel,
            c.text,
        ))
        .child(cell(score, &value(entry, ability), Type::Stat, c.text))
        .child(match modifier {
            Some(bonus) => link_cell(
                out,
                modifier_cell,
                &crate::design::signed(bonus),
                &RollLink::d20(D20::Check, bonus, name),
            ),
            None => cell(modifier_cell, "—", Type::Stat, c.muted),
        });
    match save {
        Some(bonus) => {
            row = row.child(link_cell(
                out,
                SAVE_CELL,
                &format!("Save {}", crate::design::signed(bonus)),
                &RollLink::d20(D20::Save, bonus, name),
            ));
        }
        None if saves => row = row.child(cell(SAVE_CELL, "", Type::Stat, c.muted)),
        None => {}
    }
    row.into_any_element()
}

/// Armor class, hit points and initiative under capital labels, side by side.
fn vitals(out: &mut Out<'_>, entry: &Entry, values: Option<&RunValues>) {
    let c = colors();
    let printed = creature::vital_rows(entry);
    let rows: Vec<(&str, String)> = if values.is_some() {
        ["ARMOR CLASS", "HIT POINTS", "INITIATIVE"]
            .into_iter()
            .map(|label| {
                (
                    label,
                    printed
                        .iter()
                        .find(|(key, _)| *key == label)
                        .map_or_else(|| "Unknown".to_owned(), |(_, text)| text.clone()),
                )
            })
            .collect()
    } else {
        printed
    };
    let mut line = div().flex().flex_row().flex_wrap().gap_x(L).gap_y(px(2.0));
    for (label, text) in rows {
        let run_hp = values.filter(|_| label == "HIT POINTS").map(|values| {
            crate::design::fraction(
                values
                    .current_hp
                    .map_or_else(|| "Unknown".to_owned(), |hp| hp.to_string()),
                values
                    .hp
                    .map_or_else(|| "Unknown".to_owned(), |hp| hp.to_string()),
            )
        });
        let printed = match label {
            "ARMOR CLASS" => stat(entry, "armor_class"),
            "HIT POINTS" => stat(entry, "max_hp"),
            _ => stat(entry, "initiative"),
        };
        let invalid = printed.is_none() && text != "Unknown";
        let live = values.map(|values| match label {
            "ARMOR CLASS" => values.armor_class,
            "HIT POINTS" => values.hp,
            _ => values.initiative,
        });
        let shown = live.map_or_else(
            || text.clone(),
            |n| {
                n.map_or_else(
                    || "Unknown".to_owned(),
                    |n| {
                        if label == "INITIATIVE" {
                            crate::design::signed(n)
                        } else {
                            n.to_string()
                        }
                    },
                )
            },
        );
        let shown = run_hp.unwrap_or(shown);
        let value = if invalid {
            kit::text(
                Type::Prose,
                c.result.warning,
                format!("{} {shown}", crate::gui::icons::WARN),
            )
        } else {
            kit::text(Type::Prose, c.text, shown)
        };
        let mut column = div()
            .flex()
            .flex_col()
            .pr(S)
            .child(kit::text(Type::Label, c.muted, label))
            .child(value);
        if let Some(live) = live
            && live != printed
        {
            column = column.child(kit::text(
                Type::Small,
                c.muted,
                format!("printed {}", creature::unbroken_dice(&text)),
            ));
        }
        line = line.child(column);
    }
    out.push(item(line));
}

/// Identity, vitals, abilities and details: what the entry has, and no empty cells.
pub(super) fn stat_head(out: &mut Out<'_>, entry: &Entry, values: Option<&RunValues>) {
    out.push(kit::gap(S));
    out.push(identity(creature::stat_identity(entry)));
    out.push(heavy_rule());
    out.push(kit::gap(S));
    vitals(out, entry, values);
    out.push(kit::gap(S));
    // The rulebook's table: physical abilities beside mental ones, with a clear gutter between
    // them. A creature with no saving throw has no Save column. A column too narrow for both
    // halves puts the mental half under the physical one.
    let saves = ABILITIES
        .iter()
        .any(|ability| stat(entry, &format!("save_{ability}")).is_some());
    let any_ability = ABILITIES
        .iter()
        .any(|ability| stat(entry, ability).is_some());
    if any_ability {
        out.push(parts::rule());
        out.push(kit::gap(XS));
        let halves = ABILITIES.chunks(3).map(|half| {
            div().flex().flex_col().children(
                half.iter()
                    .map(|ability| ability_row(out, entry, ability, saves)),
            )
        });
        let table = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_x(XL)
            .children(halves);
        out.push(item(table));
        out.push(kit::gap(XS));
        out.push(parts::rule());
        out.push(kit::gap(XS));
    }
    let mut details = div().flex().flex_col();
    for (key, label) in DETAILS {
        if let Some(text) = creature::detail(entry, key).filter(|text| !text.is_empty()) {
            if label == "Speed"
                && let Some(speed) = values.and_then(|values| values.speed.as_ref())
            {
                let preview =
                    splat_content_ui::roster::change_preview(&splat::content::overrides::Row {
                        label: "Speed",
                        base: text.clone(),
                        changed: speed.clone(),
                        by_hand: true,
                        in_rolls: false,
                    });
                details = details.child(stat_detail(
                    "Speed",
                    preview,
                    colors().participant.changed,
                    out.window,
                ));
            } else if label == "Skills" {
                details = details.child(skills_detail(out, &text));
            } else {
                details = details.child(stat_detail(label, text, colors().text, out.window));
            }
        }
    }
    out.push(details);
    out.push(kit::gap(XS));
    out.push(heavy_rule());
}

/// The Skills row, each bonus a dice link whose target covers its skill's name.
fn skills_detail(out: &mut Out<'_>, text: &str) -> AnyElement {
    match creature::skills_markdown(text) {
        Some(markdown) => {
            let id = out.next_id("skills");
            detail_row(
                &human_label("Skills"),
                XL,
                kit::Markdown::new(id, markdown)
                    .skills()
                    .full_width()
                    .document(out.document),
                out.window,
            )
            .into_any_element()
        }
        None => {
            stat_detail("Skills", text.to_owned(), colors().text, out.window).into_any_element()
        }
    }
}

impl SpellLinks<'_> {
    /// A list of spells (`At Will: Druidcraft, Light`) after its Spellcasting paragraph: its
    /// label, its tiles, and the spells not found.
    pub(super) fn list(&mut self, out: &mut Out<'_>, spellcasting: &str, paragraph: &str) {
        let caster = crate::library::spells::caster(self.entry, spellcasting);
        for text in paragraph.lines() {
            // Resolved with the entry, unless the reader's words differ from its body's.
            let resolved = std::sync::Arc::clone(&self.resolved);
            let fresh;
            let line = match resolved.line(text, caster, &mut self.next_list) {
                Some(line) => line,
                None => match crate::library::spells::line(
                    text,
                    caster,
                    &resolved.references,
                    self.links,
                ) {
                    Some(line) => {
                        fresh = line;
                        &fresh
                    }
                    None => continue,
                },
            };
            out.push(parts::help(line.label.clone()));
            self.draw(out, line);
            if !line.missing.is_empty() {
                out.push(parts::help(format!(
                    "Not found in this content: {}",
                    line.missing.join(", ")
                )));
            }
        }
    }

    /// After a paragraph that casts spells by name: their tiles.
    pub(super) fn casts(&mut self, out: &mut Out<'_>, paragraph: &str) {
        let resolved = std::sync::Arc::clone(&self.resolved);
        let fresh;
        let line = match resolved.cast(paragraph, &mut self.next_cast) {
            Some(line) => line,
            None => {
                fresh = crate::library::spells::cast_line(
                    self.entry,
                    paragraph,
                    &resolved.references,
                    self.links,
                );
                fresh.as_ref()
            }
        };
        if let Some(line) = line {
            self.draw(out, line);
        }
    }

    /// After a Spellcasting paragraph and its lists: the spells they name or the entry gives
    /// that no list showed, as links.
    pub(super) fn rest(&mut self, out: &mut Out<'_>, text: &str) {
        let mut targets = self.resolved.references.clone();
        let named = self.resolved.named_in(text).map_or_else(
            || self.links.spells_named_in(text),
            <[(usize, String)]>::to_vec,
        );
        for named in named {
            if !targets.contains(&Some(named.clone())) {
                targets.push(Some(named));
            }
        }
        targets.retain(|target| {
            target
                .as_ref()
                .is_none_or(|(place, _)| !self.shown.contains(place))
        });
        if targets.is_empty() {
            return;
        }
        let c = colors();
        let mut line = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap_x(S)
            .child(kit::text(Type::Small, c.muted, "Spell descriptions:"));
        for (at, target) in targets.into_iter().enumerate() {
            if let Some((index, name)) = target {
                let on_open = out.on_open.clone();
                line = line.child(
                    kit::Link::new(parts::child(&out.id, format!("spell-link-{at}")), name)
                        .on_click(move |_, window, cx| {
                            if let Some(on_open) = &on_open {
                                on_open(index, window, cx);
                            }
                        }),
                );
            } else {
                line = line.child(
                    div()
                        .id(parts::child(&out.id, format!("spell-gone-{at}")))
                        .child(kit::text(Type::Small, c.muted, "Spell unavailable"))
                        .with_tooltip(
                            "This spell cannot be found, or two files claim to be the same entry in this package.",
                        ),
                );
            }
        }
        out.push(item(line));
    }

    /// A line's spells as a row of tiles: each that rolls, and each that opens its spell.
    fn draw(&mut self, out: &mut Out<'_>, line: &crate::library::spells::SpellLine) {
        let mut tiles = Vec::new();
        let mut chips = Vec::new();
        for spell in &line.tiles {
            match &spell.chip {
                Some(chip) => {
                    let view = out
                        .ctx
                        .as_ref()
                        .and_then(|ctx| {
                            plain::roll_tiles(ctx, &self.entry.name, &[(spell.place, chip.clone())])
                                .pop()
                        })
                        .unwrap_or_else(|| {
                            plain::open_tile(&spell.name, &spell.facts, &spell.text)
                        });
                    tiles.push((
                        view,
                        Press::Roll {
                            link: chip.link.clone(),
                            occurrence: chip.spell_occurrence,
                        },
                    ));
                    chips.push(chip.clone());
                }
                None => tiles.push((
                    plain::open_tile(&spell.name, &spell.facts, &spell.text),
                    Press::Open(spell.open),
                )),
            }
        }
        let id = out.next_id("spell-tiles");
        let row = tiles::tile_row(
            &id,
            out.ctx.as_ref(),
            &self.entry.name,
            tiles,
            &chips,
            out.on_open.clone(),
        );
        out.push(row);
        self.shown.extend(line.tiles.iter().map(|spell| spell.open));
    }
}
