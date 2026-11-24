//! A character's sheet (egui: `Party::sheet_view`, `splat_party::party::sheet_page`, `overview`,
//! `details`, `background`): its name and kind line, the page tabs, the Combat sync section, and
//! the page. The Run page's side panel can show the same pages (`page`).
use crate::gui::kit::{self, Link, Message, Section, Segmented, TextField, Tone};
use crate::gui::theme::{
    Density, L, LIST_MEASURE, M, READER_MEASURE, S, Type, TypeExt, XS, colors,
};
use crate::gui::{SplatRoot, icons};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, Window, div,
    prelude::*, px,
};
use splat::content::dice_text::{D20, RollLink};
use splat_character::character::{ABILITIES, EntitySnapshot};
use splat_party::party::{
    BACKGROUND_PAGE, LiveValues, SHEET_PAGES, SHEET_PORTRAIT, array, calendar_time,
    overview_death_saves, sheet_entries, signed, skill_order, text, val,
};

/// A tile's width (`TILE_WIDTH`).
const TILE_WIDTH: gpui::Pixels = px(112.0);

fn id(words: String) -> ElementId {
    ElementId::Name(SharedString::from(words))
}

/// The sheet of the selected character, for the Party page's right column.
pub(super) fn view(
    root: &mut SplatRoot,
    connected: bool,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let Some(selected) = root.app.party.reader.selected.clone() else {
        return kit::text(
            Type::Body,
            c.muted,
            "Select a character to read their sheet.",
        )
        .into_any_element();
    };
    let mut column = div().flex().flex_col().size_full().min_w_0();
    if let Some(error) = root.app.party.reader.errors.get(&selected) {
        column = column.child(div().pb(S).child(Message::new(Tone::Error, error.clone())));
    }
    let record = splat_party_page::most_recent_sheet(&root.app.party, &selected).cloned();
    let lines = sync_footer(root, connected, cx);
    let Some(record) = record else {
        let mut body = div().flex().flex_col();
        if connected {
            body = body.child(sync_section(root, window, cx));
        }
        if !root.app.party.reader.loading() {
            body = body.child(kit::text(
                Type::Body,
                c.muted,
                "No local copy is available for this character.",
            ));
        }
        let mut pane = kit::Pane::new("party-sheet").flush().child(body);
        for line in lines {
            pane = pane.footer(line);
        }
        return column.child(pane).into_any_element();
    };
    let portrait = root
        .app
        .party
        .reader
        .local_scope()
        .and_then(|scope| splat_party::party_sync::portrait_path(scope, &record));
    let url = format!("https://www.dndbeyond.com/characters/{selected}");
    column = column
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .child(kit::portrait(
                    portrait.as_deref(),
                    &record.definition.splat_name,
                    px(SHEET_PORTRAIT),
                ))
                .child(kit::text(
                    Type::Display,
                    c.text,
                    record.definition.splat_name.clone(),
                )),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap(L)
                .min_h(px(24.0))
                .child(kit::text(
                    Type::Small,
                    c.muted,
                    splat_party_page::class_line(&record),
                ))
                .child(
                    Link::new("party-sheet-ddb", "D&D Beyond ↗")
                        .small()
                        .on_click(move |_, _, cx| cx.open_url(&url)),
                ),
        );
    let page = root.pages.party.page;
    column = column
        .child(
            Segmented::new(
                "party-sheet-pages",
                SHEET_PAGES.map(|(_, words)| words),
                page,
            )
            .on_select(cx.listener(|root, index: &usize, _, cx| {
                root.pages.party.page = *index;
                root.pages.party.search.clear();
                cx.notify();
            })),
        )
        .child(kit::rule());
    if page > 0 && sheet_entries(&record, page).len() > 8 {
        column = column.child(
            div().pt(XS).w(px(240.0)).child(
                TextField::new("party_page_filter", root.pages.party.search.clone())
                    .placeholder(format!("Filter {}…", SHEET_PAGES[page].1.to_lowercase()))
                    .name("Filter this page")
                    .on_change(cx.listener(|root, words: &str, _, cx| {
                        root.pages.party.search = words.to_owned();
                        cx.notify();
                    })),
            ),
        );
    }
    let mut body = div().flex().flex_col().pr(M).pb(L);
    if connected {
        body = body.child(sync_section(root, window, cx));
    }
    let live = root
        .app
        .party
        .tonight_values(&record)
        .map(|(hp, temporary_hp)| LiveValues {
            hp: Some(hp),
            max_hp: record.stat("max_hp"),
            temporary_hp,
            death_saves: record.initial.death_saves.clone(),
            death_saves_shown: false,
            conditions: splat_party::party::sheet_labels(&record),
        });
    let reader = Reader {
        key: "party",
        search: root.pages.party.search.clone(),
        roller: None,
        actions: None,
    };
    body = body.child(page_body(root, &record, page, live.as_ref(), reader, cx));
    if page == 0 {
        if root.pages.party.open.contains(TECHNICAL) {
            review(root, &record);
        }
        body = body.child(kit::gap(M)).child(technical(root, &record, cx));
    }
    let mut pane = kit::Pane::new(("party-sheet-page", page))
        .flush()
        .child(body);
    for line in lines {
        pane = pane.footer(line);
    }
    column
        .child(div().flex_1().min_h_0().child(pane))
        .into_any_element()
}

/// The sheet pane's footer: the sync panel's result (`CombatSync::footer_messages`).
fn sync_footer(root: &SplatRoot, connected: bool, cx: &mut Context<SplatRoot>) -> Vec<AnyElement> {
    if !connected {
        return Vec::new();
    }
    let frame = crate::design::outcome::PaneFrame::new();
    let mut messages = crate::design::outcome::Messages::new();
    root.app.party.combat.footer_messages(&frame, &mut messages);
    kit::messages::lines(
        "party-sheet-footer",
        messages.into_plain(),
        crate::gui::pages::settings::on_press(cx, |root, press, cx| {
            root.app.party.combat.footer_press(Some(press));
            cx.notify();
        }),
    )
}

/// `Combat sync · On`: a caret section whose title says the sync state while it is closed. Its
/// body is the sync editor (`sync_editor::panel`), as the run's Correct tab shows it too.
fn sync_section(root: &mut SplatRoot, window: &Window, cx: &mut Context<SplatRoot>) -> AnyElement {
    if std::mem::take(&mut root.app.party.wants_combat_sync) {
        root.pages.party.sync_open = true;
    }
    let combat = &root.app.party.combat;
    // Until the first read, `On` would say more than Splat knows: the heading says it waits.
    let state = if !combat.enabled {
        "Off"
    } else if combat.state() == splat_party::combat_sync::SyncState::FirstRead {
        "Waiting for the first sheet read"
    } else {
        "On"
    };
    let open = root.pages.party.sync_open;
    // One id for every state, so the panel stays open when the DM ticks its box.
    let mut section = Section::new("combat_sync_header", format!("Combat sync · {state}"), open)
        .on_toggle(cx.listener(|root, open: &bool, _, cx| {
            root.pages.party.sync_open = *open;
            cx.notify();
        }));
    if open {
        section = section
            .child(
                // The sheet's scroll bar floats over its right edge: the panel keeps clear of it.
                div().pr(L).child(super::sync_editor::panel(
                    super::sync_editor::Host::Party,
                    Some(&root.app.party.combat),
                    window,
                    cx,
                )),
            )
            .child(kit::gap(S))
            .child(kit::rule());
    }
    section.into_any_element()
}

/// Who reads a sheet's page: the Party page, or the run's Sheet tab.
pub(crate) struct Reader {
    /// Keeps one host's open entries apart from another's (`party`, `run`).
    pub key: &'static str,
    /// `Filter this page`'s words.
    pub search: String,
    /// The run's row whose numbers roll for it, or `None`: a press is then a preview.
    pub roller: Option<usize>,
    /// The run's own action tiles, in place of the sheet's attacks: in a run they arm.
    pub actions: Option<AnyElement>,
}

/// One page of a sheet. `key` keeps one host's open entries apart from another's.
pub(crate) fn page_body(
    root: &SplatRoot,
    record: &EntitySnapshot,
    page: usize,
    live: Option<&LiveValues>,
    reader: Reader,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    match page {
        0 => overview(record, live, &reader),
        BACKGROUND_PAGE => background(root, record, reader.key, cx),
        _ => details(root, record, page, reader, cx),
    }
}

fn section(title: &str, count: usize, first: bool) -> AnyElement {
    kit::heading(kit::Heading::Group { title, count }, first).into_any_element()
}

/// One tile: a muted label line over its value.
fn tile(label: impl Into<SharedString>, value: impl Into<SharedString>) -> AnyElement {
    let c = colors();
    div()
        .flex()
        .flex_col()
        .w(TILE_WIDTH)
        .min_h(px(44.0))
        .child(kit::text::one_line(Type::Small, c.muted, label))
        .child(kit::text(Type::NumeralLarge, c.text, value))
        .into_any_element()
}

fn tiles(children: Vec<AnyElement>) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .flex_wrap()
        .gap_x(S)
        .gap_y(XS)
        .max_w(TILE_WIDTH * 6.0 + S * 6.0)
        .children(children)
        .into_any_element()
}

/// `Label: value · value` (`design::fact::line`).
fn fact(label: &str, values: impl IntoIterator<Item = impl std::fmt::Display>) -> AnyElement {
    let values: Vec<String> = values.into_iter().map(|value| value.to_string()).collect();
    kit::text(
        Type::Body,
        colors().text,
        crate::design::fact::tile_label(label, values.join(" · ")),
    )
    .into_any_element()
}

/// Rows of items in two columns (`sheet_columns`).
fn columns(items: Vec<AnyElement>) -> AnyElement {
    let rows = items.len().div_ceil(2);
    let mut left = div().flex().flex_col().flex_1().min_w_0().gap(XS);
    let mut right = div().flex().flex_col().flex_1().min_w_0().gap(XS);
    for (at, item) in items.into_iter().enumerate() {
        if at < rows {
            left = left.child(item);
        } else {
            right = right.child(item);
        }
    }
    div()
        .flex()
        .flex_row()
        .gap(S)
        .child(left)
        .child(right)
        .into_any_element()
}

/// A bonus in the numeral type that rolls (or a dash), then its words.
fn bonus_line(
    bonus: AnyElement,
    words: impl Into<SharedString>,
    marks: Option<String>,
) -> AnyElement {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(S)
        .min_h(px(24.0))
        .child(bonus)
        .child(kit::text(Type::Body, c.text, words))
        .children(marks.map(|marks| kit::text(Type::Small, c.muted, marks)))
        .into_any_element()
}

/// A d20 roll for an ability, a save or a skill, named in book case (egui `sheets::d20`).
fn d20(what: D20, bonus: i64, name: &str) -> RollLink {
    RollLink::d20(what, bonus, crate::design::reader::human_label(name))
}

/// A bonus that rolls for the sheet's character (egui `numeral_link`), or a dash when the sheet
/// has none.
fn numeral(
    id: String,
    bonus: Option<i64>,
    record: &EntitySnapshot,
    reader: &Reader,
    link: impl FnOnce(i64) -> RollLink,
) -> AnyElement {
    match bonus {
        Some(bonus) => kit::RollNumber::new(
            ElementId::Name(SharedString::from(format!("{}-{id}", reader.key))),
            crate::design::signed(bonus),
            Type::Numeral,
            link(bonus),
        )
        .entry(record.definition.splat_name.clone())
        .roller(reader.roller)
        .into_any_element(),
        None => div()
            .min_w(px(24.0))
            .ty(Type::Numeral)
            .text_color(colors().text)
            .child("—")
            .into_any_element(),
    }
}

fn overview(record: &EntitySnapshot, live: Option<&LiveValues>, reader: &Reader) -> AnyElement {
    let c = colors();
    let mut column = div().flex().flex_col().max_w(LIST_MEASURE);
    let (hp, max, temp) = live.map_or_else(
        || {
            (
                record.initial.hp.to_string(),
                val(record, "max_hp"),
                record.initial.temporary_hp,
            )
        },
        |live| {
            let known =
                |value: Option<i64>| value.map_or_else(|| "?".to_owned(), |n| n.to_string());
            (known(live.hp), known(live.max_hp), live.temporary_hp)
        },
    );
    column = column.child(section("Combat", 0, true)).child(tiles(vec![
        tile("Hit points", splat_party::combat_sync::fraction(hp, max)),
        tile("Armor class", val(record, "armor_class")),
        tile("Temp HP", temp.to_string()),
        tile("Proficiency", signed(record, "proficiency_bonus")),
        tile("Initiative", signed(record, "initiative")),
        tile(
            "Inspiration",
            if record.initial.inspiration {
                "Yes"
            } else {
                "No"
            },
        ),
    ]));
    let abilities = ABILITIES
        .iter()
        .map(|ability| {
            let label = crate::design::fact::tile_label(
                &format!("{}{}", ability[..1].to_uppercase(), &ability[1..3]),
                val(record, ability),
            );
            let key = format!("{ability}_modifier");
            let value = signed(record, &key);
            match record.stat(&key) {
                Some(bonus) => div()
                    .w(TILE_WIDTH)
                    .min_h(px(44.0))
                    .child(kit::fact::roll_tile(
                        &label,
                        div().flex().child(
                            kit::RollNumber::new(
                                ElementId::Name(SharedString::from(format!(
                                    "{}-check-{ability}",
                                    reader.key
                                ))),
                                value,
                                Type::NumeralLarge,
                                d20(D20::Check, bonus, ability),
                            )
                            .entry(record.definition.splat_name.clone())
                            .roller(reader.roller),
                        ),
                    ))
                    .into_any_element(),
                None => tile(label, value),
            }
        })
        .collect();
    column = column
        .child(section("Ability scores", 0, false))
        .child(tiles(abilities));
    let mut passive: Vec<AnyElement> = [
        ("Perception", "passive_perception"),
        ("Insight", "passive_insight"),
        ("Investigation", "passive_investigation"),
    ]
    .into_iter()
    .map(|(label, key)| tile(label, val(record, key)))
    .collect();
    for (key, stat) in &record.definition.splat_stats {
        if let Some(mode) = key.strip_prefix("speed_") {
            passive.push(tile(
                crate::design::reader::human_label(mode),
                format!("{} ft", stat.value),
            ));
        }
    }
    column = column
        .child(section("Passive scores & movement", 0, false))
        .child(tiles(passive));
    let saves = ABILITIES
        .iter()
        .map(|ability| {
            let mut name = ability.to_string();
            name[..1].make_ascii_uppercase();
            bonus_line(
                numeral(
                    format!("save-{ability}"),
                    record.stat(&format!("{ability}_save")),
                    record,
                    reader,
                    |bonus| d20(D20::Save, bonus, ability),
                ),
                name,
                None,
            )
        })
        .collect();
    column = column
        .child(section("Saving throws", 0, false))
        .child(columns(saves));
    let skills = skill_order(record)
        .into_iter()
        .map(|at| {
            let skill = &record.character.skills[at];
            let mut marks = Vec::new();
            if skill.expertise {
                marks.push("Expertise");
            } else if skill.proficient {
                marks.push("Proficient");
            } else if skill.half_proficient {
                marks.push("Half");
            }
            if skill.advantage {
                marks.push("Advantage");
            }
            if skill.disadvantage {
                marks.push("Disadvantage");
            }
            bonus_line(
                numeral(
                    format!("skill-{at}"),
                    record.stat(&skill.stat),
                    record,
                    reader,
                    |bonus| d20(D20::Skill, bonus, &skill.name),
                ),
                skill.name.clone(),
                (!marks.is_empty()).then(|| marks.join(" · ")),
            )
        })
        .collect();
    column = column
        .child(section("Skills", 0, false))
        .child(columns(skills));
    column = column.child(section("Resources", 0, false));
    if record.definition.splat_resources.is_empty() {
        column = column.child(kit::text(
            Type::Body,
            c.muted,
            "No limited-use pools returned.",
        ));
    }
    for (key, pool) in &record.definition.splat_resources {
        column = column.child(fact(
            &pool.name,
            [format!(
                "{} remaining",
                splat_party::combat_sync::fraction(
                    record
                        .initial
                        .resources
                        .get(key)
                        .map(|v| v.to_string())
                        .unwrap_or("—".into()),
                    pool.max
                )
            )],
        ));
    }
    let mut proficiencies = div().flex().flex_col().flex_1().min_w_0().child(section(
        "Proficiencies & languages",
        0,
        true,
    ));
    for group in &record.character.proficiencies {
        let values = text(&group["values"]);
        if !values.is_empty() {
            proficiencies = proficiencies.child(fact(text(&group["group"]), values.split(", ")));
        }
    }
    let mut defenses =
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .child(section("Defenses & senses", 0, true));
    for (label, values) in [
        ("Resistance", &record.character.resistances),
        ("Immunity", &record.character.immunities),
        ("Vulnerability", &record.character.vulnerabilities),
    ] {
        if !values.is_empty() {
            defenses = defenses.child(fact(label, values.iter().map(|value| text(&value["name"]))));
        }
    }
    for sense in &record.character.senses {
        defenses = defenses.child(fact(text(&sense["name"]), [text(&sense["distance"])]));
    }
    if let Some(words) = overview_death_saves(record, live) {
        defenses = defenses
            .child(section("Death saves", 0, false))
            .child(kit::text(Type::Body, c.text, words));
    }
    defenses = defenses.child(section("Conditions", 0, false));
    let names: Vec<String> = live.map_or_else(
        || {
            record
                .initial
                .conditions
                .iter()
                .map(|condition| {
                    condition["name"]
                        .as_str()
                        .map(str::to_owned)
                        .unwrap_or_else(|| condition.to_string())
                })
                .collect()
        },
        |live| {
            live.conditions
                .iter()
                .map(|condition| condition.words())
                .collect()
        },
    );
    if names.is_empty() {
        defenses = defenses.child(kit::text(Type::Body, c.text, "None"));
    }
    for name in names {
        defenses = defenses.child(kit::text(Type::Body, c.text, name));
    }
    column
        .child(kit::gap(S))
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .gap(S)
                .child(proficiencies.min_w(px(240.0)))
                .child(defenses.min_w(px(240.0))),
        )
        .into_any_element()
}

/// Prose in the reader's measure, a paragraph at a time.
fn prose(label: Option<&str>, words: &str) -> AnyElement {
    let c = colors();
    let mut column = div().flex().flex_col().gap(XS).max_w(READER_MEASURE);
    if let Some(label) = label {
        column = column
            .child(kit::gap(S))
            .child(kit::text(Type::Small, c.muted, label.to_owned()));
    }
    for paragraph in words.split("\n\n") {
        column = column.child(kit::text(Type::ProseCompact, c.text, paragraph.to_owned()));
    }
    column.into_any_element()
}

/// A quiet caret section whose open state is kept under `key` in the page's state.
fn quiet(
    root: &SplatRoot,
    key: String,
    title: &str,
    cx: &mut Context<SplatRoot>,
    body: impl FnOnce() -> Vec<AnyElement>,
) -> AnyElement {
    let open = root.pages.party.open.contains(&key);
    let toggle = key.clone();
    let mut section = kit::QuietSection::new(id(format!("party-quiet-{key}")), title.to_owned())
        .open(open)
        .on_toggle(cx.listener(move |root, open: &bool, _, cx| {
            if *open {
                root.pages.party.open.insert(toggle.clone());
            } else {
                root.pages.party.open.remove(&toggle);
            }
            cx.notify();
        }));
    if open {
        section = section.children(body());
    }
    section.into_any_element()
}

fn background(
    root: &SplatRoot,
    record: &EntitySnapshot,
    key: &str,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    use splat_character::character::profile::{
        characteristic_rows, companion_line, detail_rows, story_rows,
    };
    let c = colors();
    let profile = record.profile();
    let details = detail_rows(&profile.details);
    let characteristics = characteristic_rows(&profile.characteristics);
    let story = story_rows(&profile.story);
    let mut column = div().flex().flex_col().max_w(LIST_MEASURE);
    if profile.background.is_none()
        && details.is_empty()
        && characteristics.is_empty()
        && story.is_empty()
        && profile.companions.is_empty()
    {
        return column
            .child(section("Background", 0, true))
            .child(kit::text(
                Type::Body,
                c.muted,
                "This sheet has no background, details or notes.",
            ))
            .into_any_element();
    }
    if let Some(background) = &profile.background {
        column = column
            .child(section("Background", 0, true))
            .child(kit::text(
                Type::Strong,
                c.text,
                if background.custom {
                    format!("{} (custom)", background.name)
                } else {
                    background.name.clone()
                },
            ));
        if let Some((_, feature)) = &profile.background_feature {
            column = column.child(fact("Feature", [&feature.name]));
        }
        if !background.feats.is_empty() {
            column = column.child(fact("Feats", &background.feats));
        }
        if !background.description.is_empty() || profile.background_feature.is_some() {
            let description = background.description.clone();
            let feature = profile.background_feature.clone();
            column = column.child(quiet(
                root,
                format!("{key}-background-text"),
                "Background text",
                cx,
                move || {
                    let mut body = Vec::new();
                    if !description.is_empty() {
                        body.push(prose(None, &description));
                    }
                    if let Some((_, feature)) = &feature {
                        body.push(prose(Some(&feature.name), &feature.description));
                    }
                    body
                },
            ));
        }
    }
    if !details.is_empty() {
        column = column.child(section("Details", 0, false)).child(columns(
            details
                .iter()
                .map(|(label, value)| fact(label, [value]))
                .collect(),
        ));
    }
    for (title, rows) in [("Characteristics", characteristics), ("Notes", story)] {
        if rows.is_empty() {
            continue;
        }
        column = column.child(section(title, 0, false));
        for (label, words) in rows {
            column = column.child(prose(Some(label), words));
        }
    }
    if !profile.companions.is_empty() {
        column = column
            .child(section("Companions", profile.companions.len(), false))
            .child(kit::help(
                "Stat blocks are not imported. HP is the printed value.",
            ));
        for companion in profile.companions.values() {
            column = column
                .child(kit::gap(S))
                .child(kit::text(Type::Strong, c.text, companion.name.clone()))
                .child(kit::text(Type::Small, c.muted, companion_line(companion)));
            if !companion.description.is_empty() {
                column = column.child(prose(None, &companion.description));
            }
        }
    }
    column.into_any_element()
}

fn details(
    root: &SplatRoot,
    record: &EntitySnapshot,
    page: usize,
    reader: Reader,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let key = reader.key;
    let search = reader.search.to_lowercase();
    let mut column = div().flex().flex_col().max_w(LIST_MEASURE);
    match page {
        1 => {
            column = column.child(section("Attacks", record.character.attacks.len(), true));
            // Each attack is an action tile: in a run it arms (the run's own tiles), elsewhere
            // it previews. A tile keeps its number in the whole list while a search hides
            // others.
            column = column.child(match reader.actions {
                Some(actions) => actions,
                None => attack_tiles(record, &search, key, cx),
            });
        }
        2 => {
            column = column.child(section("Spellcasting", 0, true)).child(kit::help(
                "Spell text shows the printed numbers. Splat does not work out upcasting or this character’s own bonuses.",
            ));
            for (label, which) in [
                ("Spell save DC", "saveDcs"),
                ("Spell attack", "spellAttacks"),
                ("Casting modifier", "modifiers"),
            ] {
                for value in array(&record.character.casting[which]) {
                    let number = if which == "spellAttacks" {
                        value["value"]
                            .as_i64()
                            .map_or_else(|| value["value"].to_string(), crate::design::signed)
                    } else {
                        value["value"].to_string()
                    };
                    let sources: Vec<String> = std::iter::once(number)
                        .chain(
                            array(&value["sources"])
                                .iter()
                                .filter_map(serde_json::Value::as_str)
                                .map(str::to_owned),
                        )
                        .collect();
                    column = column.child(fact(label, sources));
                }
            }
            for (pool_key, pool) in &record.definition.splat_resources {
                if pool.kind.as_deref() != Some("spell_slot") {
                    continue;
                }
                column = column.child(fact(
                    &pool.name,
                    [format!(
                        "{} remaining",
                        splat_party::combat_sync::fraction(
                            record
                                .initial
                                .resources
                                .get(pool_key)
                                .map(|v| v.to_string())
                                .unwrap_or("—".into()),
                            pool.max
                        )
                    )],
                ));
            }
        }
        _ => {}
    }
    let entries = sheet_entries(record, page);
    let filtered: Vec<_> = entries
        .iter()
        .filter(|(name, source, _, _)| format!("{name} {source}").to_lowercase().contains(&search))
        .collect();
    if page != 2 {
        column = column.child(section(
            match page {
                1 => "Action descriptions",
                3 => "Equipment & coins",
                _ => "Features & traits",
            },
            filtered.len(),
            false,
        ));
    }
    if page == 3 {
        use splat_character::character::profile;
        column = column.child(fact(
            "Coins",
            [profile::coins(&record.source.raw)
                .map_or_else(|| "None".into(), |coins| profile::coin_line(&coins))],
        ));
    }
    let mut level_counts = std::collections::BTreeMap::new();
    for row in &filtered {
        *level_counts.entry(row.3).or_insert(0) += 1;
    }
    if filtered.is_empty() {
        column = column.child(kit::text(
            Type::Body,
            c.muted,
            if search.is_empty() {
                "No entries returned for this character."
            } else {
                "No entries match this filter."
            },
        ));
    }
    let mut last_level = None;
    for (at, (name, meta, description, level)) in filtered.into_iter().enumerate() {
        if page == 2 && (at == 0 || last_level != *level) {
            column = column.child(section(
                &match level {
                    Some(0) => "Cantrips".to_owned(),
                    Some(n) => format!("Level {n} spells"),
                    None => "Other spells".to_owned(),
                },
                level_counts.get(level).copied().unwrap_or(0),
                false,
            ));
            last_level = *level;
        }
        let open_key = format!("{key}-{page}-{at}-{name}");
        let open = root.pages.party.open.contains(&open_key);
        let toggle = open_key.clone();
        let row = div()
            .id(id(format!("party-entry-{open_key}")))
            .flex()
            .flex_row()
            .items_start()
            .w_full()
            .min_h(if meta.is_empty() {
                Density::Read.row()
            } else {
                px(40.0)
            })
            .px(S)
            .py(XS)
            .rounded(crate::gui::theme::RADIUS_CONTROL)
            .cursor_pointer()
            .when(open, |row| row.bg(c.selected))
            .hover(|row| row.bg(c.hover))
            .on_click(cx.listener(move |root, _, _, cx| {
                if !root.pages.party.open.remove(&toggle) {
                    root.pages.party.open.insert(toggle.clone());
                }
                cx.notify();
            }))
            .child(
                div()
                    .w(px(16.0))
                    .flex_none()
                    .icon_font(Type::Small)
                    .text_color(c.muted)
                    .child(SharedString::new_static(if open {
                        icons::DOWN
                    } else {
                        icons::NEXT
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .min_w_0()
                    .child(kit::text::one_line(Type::Body, c.text, name.clone()))
                    .when(!meta.is_empty(), |line| {
                        line.child(kit::text::one_line(Type::Small, c.muted, meta.clone()))
                    }),
            );
        let row = kit::target(row, format!("{name} · {meta}"));
        column = column.child(row);
        if open {
            column = column.child(div().pl(px(24.0)).pb(S).child(prose(None, description)));
        }
    }
    column.into_any_element()
}

/// The open-state key of `Technical details`.
const TECHNICAL: &str = "party-technical";

/// The sheet's campaign content entry and its character check, made once for each content
/// version (egui `PartyPage::content_entry`, `character_review`): the page draws every frame.
pub(crate) struct Checked {
    hash: String,
    entry: Result<splat_content::content::Entry, String>,
    review: Option<Result<splat_character::character::check::Review, String>>,
}

/// Make the check of this sheet's content version, when it is not made yet.
fn review(root: &mut SplatRoot, record: &EntitySnapshot) {
    if root
        .pages
        .party
        .checked
        .as_ref()
        .is_some_and(|checked| checked.hash == record.source.content_hash)
    {
        return;
    }
    let entry = record.entry();
    let review = entry
        .as_ref()
        .ok()
        .map(|entry| splat_character::character::check::review(record, entry));
    root.pages.party.checked = Some(Checked {
        hash: record.source.content_hash.clone(),
        entry,
        review,
    });
}

/// The attacks as action tiles that preview their roll, with the last preview's result under
/// them (egui `rolls::tiles` outside a run).
fn attack_tiles(
    record: &EntitySnapshot,
    search: &str,
    key: &str,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    use crate::gui::pages::reader::entry::tiles;
    let Some(ctx) = cx
        .try_global::<kit::markdown::Host>()
        .map(|host| host.0.clone())
    else {
        return div().into_any_element();
    };
    let name = &record.definition.splat_name;
    let chips: Vec<(Option<usize>, crate::rolls::Chip)> = splat_party::party::sheet_chips(record)
        .into_iter()
        .enumerate()
        .filter(|(_, chip)| chip.label.to_lowercase().contains(search))
        .map(|(place, chip)| (Some(place), chip))
        .collect();
    let views = crate::rolls::plain::roll_tiles(&ctx, name, &chips);
    let presses = views
        .into_iter()
        .zip(chips.iter())
        .map(|(view, (_, chip))| {
            (
                view,
                tiles::Press::Roll {
                    link: chip.link.clone(),
                    occurrence: chip.spell_occurrence,
                },
            )
        })
        .collect();
    let shown: Vec<_> = chips.into_iter().map(|(_, chip)| chip).collect();
    tiles::tile_row(
        &id(format!("{key}-attack-tiles")),
        Some(&ctx),
        name,
        presses,
        &shown,
        None,
    )
}

/// A quiet section of the technical details, kept open under `key`, whose body is `lines`.
fn quiet_lines(
    root: &SplatRoot,
    key: &str,
    title: &str,
    lines: Vec<AnyElement>,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    quiet(root, format!("party-tech-{key}"), title, cx, move || lines)
}

/// `Technical details`: the record behind the sheet (`source_and_parity`): its source notes, the
/// optional character rules, the character check, the campaign content counts and what the
/// import left, when the sheet was fetched, and its files.
fn technical(root: &SplatRoot, record: &EntitySnapshot, cx: &mut Context<SplatRoot>) -> AnyElement {
    use splat_party_page::technical as words;
    let c = colors();
    let record = record.clone();
    let campaign = root
        .app
        .party
        .reader
        .campaign_path()
        .map(std::path::Path::to_path_buf);
    let scope = root.app.party.reader.local_scope();
    // The content's parts, each made only while the section is open.
    let mut content: Vec<AnyElement> = Vec::new();
    if root.pages.party.open.contains(TECHNICAL)
        && let Some(Checked {
            entry: Ok(entry),
            review,
            ..
        }) = root
            .pages
            .party
            .checked
            .as_ref()
            .filter(|checked| checked.hash == record.source.content_hash)
    {
        if let Some(options) = words::options(entry) {
            let mut lines: Vec<AnyElement> = options
                .into_iter()
                .map(|line| kit::text(Type::Body, c.text, line).into_any_element())
                .collect();
            lines.push(kit::help(words::OPTIONS_HELP).into_any_element());
            content.push(quiet_lines(
                root,
                "options",
                "Optional character rules",
                lines,
                cx,
            ));
        }
        if let Some(review) = review {
            let lines = words::check(review)
                .into_iter()
                .map(|line| match line {
                    words::CheckLine::Title(title) => {
                        kit::text(Type::Strong, c.text, title).into_any_element()
                    }
                    words::CheckLine::Words(words) => kit::text(Type::Body, c.text, words)
                        .max_w(READER_MEASURE)
                        .into_any_element(),
                })
                .collect();
            content.push(quiet_lines(root, "check", "Character check", lines, cx));
        }
        content.push(kit::help(words::content_counts(entry)).into_any_element());
        if let Some(import) = words::import(entry) {
            content.push(kit::help(import.summary).into_any_element());
            for (at, (title, reasons)) in import.lists.into_iter().enumerate() {
                let lines = reasons
                    .into_iter()
                    .map(|reason| {
                        kit::text(Type::Body, c.text, reason)
                            .max_w(READER_MEASURE)
                            .into_any_element()
                    })
                    .collect();
                content.push(quiet_lines(root, &format!("import-{at}"), title, lines, cx));
            }
        }
    }
    let files_open = root.pages.party.open.contains("party-file-details");
    let files = kit::QuietSection::new("party-file-details", "File details")
        .open(files_open)
        .on_toggle(cx.listener(|root, open: &bool, _, cx| {
            if *open {
                root.pages.party.open.insert("party-file-details".into());
            } else {
                root.pages.party.open.remove("party-file-details");
            }
            cx.notify();
        }));
    let files = if files_open {
        let mut lines = vec![
            format!(
                "Preserved {} source fields and {} computed fields.",
                record.source.raw.as_object().map_or(0, |o| o.len()),
                record.source.computed.as_object().map_or(0, |o| o.len())
            ),
            format!("Local creature UUID: {}", record.definition.splat_id),
            format!(
                "Source character: {} · Campaign: {}",
                record.source.character_id, record.source.campaign_id
            ),
        ];
        if let Some(root) = &campaign {
            lines.push(format!(
                "Campaign copy: {}",
                record.campaign_path(root).display()
            ));
        }
        if let Some(scope) = scope
            && let Ok(cache) = crate::hermetic::character_cache()
            && let Ok(path) = cache.path(
                scope,
                &record.source.campaign_id,
                &record.source.character_id,
            )
        {
            lines.push(format!("Local copy: {}", path.display()));
        }
        files.children(lines.into_iter().map(kit::help))
    } else {
        files
    };
    quiet(root, TECHNICAL.into(), "Technical details", cx, move || {
        let mut body: Vec<AnyElement> =
            vec![kit::help("D&D Beyond supplies these totals.").into_any_element()];
        // The first two source notes repeat the totals sentence and internal schema details.
        // Keep later notices, such as two services returning different names.
        for message in record.coverage.iter().skip(2) {
            body.push(kit::help(message.clone()).into_any_element());
        }
        body.extend(content);
        body.push(
            kit::help(format!(
                "Fetched {} UTC. Source details {} UTC.",
                calendar_time(record.source.fetched_at),
                calendar_time(record.source.raw_fetched_at),
            ))
            .into_any_element(),
        );
        if let Some(modified) = &record.source.source_modified {
            body.push(
                kit::help(format!("Beyond source modification: {modified}")).into_any_element(),
            );
        }
        body.push(
            kit::help(
                "D&D Beyond can take time to show a change in every service. Splat checks again for delayed changes.",
            )
            .into_any_element(),
        );
        body.push(files.into_any_element());
        body
    })
}
