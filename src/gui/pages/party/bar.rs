//! The party at a glance, above the Session page while a session is played (egui:
//! `party_bar::show`, `Splat::party_bar`). Each member is a portrait, a first name, HP with its
//! bar and a few numbers; a press opens the sheet. When the line is short of room, the optional
//! numbers go first, then the portraits, then the names shorten to an initial; no item is cut,
//! and `N more` names the rest.
use crate::gui::kit::{self, IconButton, WithTooltip as _};
use crate::gui::theme::{Density, L, M, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, bridge, icons};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, Pixels, SharedString, Styled,
    Window, div, prelude::*, px,
};
use splat_party::party_bar::Glance;

const PORTRAIT: Pixels = px(24.0);

fn known(n: Option<i64>) -> String {
    n.map_or_else(|| "?".to_owned(), |n| n.to_string())
}

/// The numbers after HP, in the Party list's order. Only a caster has a DC.
fn stats(glance: &Glance) -> Vec<(&'static str, Option<i64>)> {
    let mut stats = vec![
        ("AC", glance.ac),
        ("Per", glance.perception),
        ("Ins", glance.insight),
    ];
    if glance.spell_dc.is_some() {
        stats.push(("DC", glance.spell_dc));
    }
    stats
}

/// Everything the bar knows of a member, a fact to a line, for hover and its name.
fn facts(glance: &Glance) -> Vec<String> {
    let mut hp = format!(
        "HP {}",
        splat_party::combat_sync::fraction(known(glance.hp), known(glance.max_hp))
    );
    if let Some((_, word, _)) = kit::hp::word(glance.hp, glance.max_hp) {
        hp.push_str(&format!(" · {word}"));
    }
    if glance.temp > 0 {
        hp.push_str(&format!(" · Temp HP {}", glance.temp));
    }
    let mut armor = format!("AC {}", known(glance.ac));
    if let Some(dc) = glance.spell_dc {
        armor.push_str(&format!(" · Spell save DC {dc}"));
    }
    vec![
        hp,
        armor,
        format!(
            "Passive Perception {} · Insight {} · Investigation {}",
            known(glance.perception),
            known(glance.insight),
            known(glance.investigation)
        ),
        if glance.conditions.is_empty() {
            "No conditions".to_owned()
        } else {
            glance
                .conditions
                .iter()
                .map(|condition| condition.words())
                .collect::<Vec<_>>()
                .join(", ")
        },
    ]
}

/// One way to draw the bar, from the fullest down (`party_bar::steps`).
#[derive(Clone, Copy)]
struct Step {
    shown: usize,
    portrait: bool,
    initial: bool,
    state_word: bool,
}

fn steps(most: usize) -> Vec<Step> {
    let step = |shown, portrait, initial, state_word| Step {
        shown,
        portrait,
        initial,
        state_word,
    };
    let mut steps: Vec<Step> = (most.min(1)..=most)
        .rev()
        .map(|shown| step(shown, true, false, true))
        .collect();
    steps.extend([
        step(most.min(1), false, false, true),
        step(0, false, false, true),
        step(0, false, true, true),
        step(0, false, true, false),
    ]);
    steps
}

/// The width of a run of `Small` words, by the type's average letter.
fn small_width(words: &str) -> Pixels {
    Type::Small.size() * 0.56 * words.chars().count() as f32
}

/// An item's width at a step.
fn width(glance: &Glance, step: Step) -> Pixels {
    let first = glance.name.split_whitespace().next().unwrap_or_default();
    let name = if step.initial {
        format!("{}.", first.chars().next().unwrap_or(' '))
    } else {
        first.to_owned()
    };
    let hp = Type::Numeral.size() * 0.62 * known(glance.hp).len() as f32
        + Type::Small.size() * 0.62 * 4.0;
    let mut total = XS + small_width(&name) + S + hp + XS;
    if step.portrait {
        total += PORTRAIT + XS;
    }
    if glance.temp > 0 {
        total += XS + small_width(&format!("+{}", glance.temp)) + px(14.0);
    }
    if let Some((_, word, _)) = kit::hp::word(glance.hp, glance.max_hp) {
        total += S
            + px(14.0)
            + if step.state_word {
                small_width(word)
            } else {
                px(0.0)
            };
    }
    for (label, value) in stats(glance).into_iter().take(step.shown) {
        total += S + small_width(&format!("{label} {}", known(value)));
    }
    total
}

/// The bar.
pub(super) fn render(
    root: &mut SplatRoot,
    window: &mut Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let app = &root.app;
    let run = app
        .encounters
        .run()
        .map(crate::encounter_run::Run::party_now)
        .unwrap_or_default();
    let glances = app.party.glances(&run);
    let source = app.party.glance_source(!run.is_empty());
    let inert = app.copy_dialog.is_some() || app.confirm_close;
    let room = window.viewport_size().width
        - crate::gui::theme::RAIL_WIDTH
        - M * 2.0
        - small_width("PARTY")
        - L
        - Density::Read.control()
        - S;
    let gap = M;
    let most = glances.iter().map(|g| stats(g).len()).max().unwrap_or(0);
    let all = steps(most);
    let needed = |step: Step| {
        glances.iter().map(|g| width(g, step)).sum::<Pixels>()
            + gap * glances.len().saturating_sub(1) as f32
    };
    let step = all
        .iter()
        .copied()
        .find(|step| needed(*step) <= room)
        .unwrap_or(*all.last().expect("a step"));
    // The items that fit whole, from the first; `N more` names the rest.
    let mut used = px(0.0);
    let mut kept = 0;
    for (at, glance) in glances.iter().enumerate() {
        let next = used + width(glance, step) + if at > 0 { gap } else { px(0.0) };
        let hidden = glances.len() - at - 1;
        let more = if hidden > 0 {
            gap + small_width(&format!("{hidden} more")) + XS * 2.0
        } else {
            px(0.0)
        };
        if next + more > room {
            break;
        }
        used = next;
        kept += 1;
    }
    let free = room - used - L;
    let shown_source = [source.word.as_str(), source.short]
        .into_iter()
        .find(|words| small_width(words) + px(14.0) <= free)
        .map(str::to_owned);
    let mut items = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(gap)
        .flex_1()
        .min_w_0()
        .overflow_hidden();
    for (at, glance) in glances.iter().take(kept).enumerate() {
        items = items.child(item(glance, step, at, &source.word, cx));
    }
    let hidden: Vec<&Glance> = glances.iter().skip(kept).collect();
    if !hidden.is_empty() {
        let words = hidden
            .iter()
            .map(|glance| format!("{} · {}", glance.name, facts(glance)[0]))
            .collect::<Vec<_>>()
            .join("; ");
        items = items.child(
            div()
                .id("party-bar-more")
                .px(XS)
                .child(kit::text(
                    Type::Small,
                    c.muted,
                    format!("{} more", hidden.len()),
                ))
                .with_tooltip(words),
        );
    }
    let source_status = shown_source.map(|words| {
        div()
            .id("party-bar-source")
            .child(kit::status(&kit::Status::new(
                source.icon,
                words,
                bridge::color(source.color),
            )))
            .with_tooltip(source.word.clone())
    });
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(L)
        .h(Density::Read.row() + XS * 2.0)
        .px(M)
        .py(XS)
        .bg(c.page)
        .border_b(crate::gui::theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .when(inert, |bar| bar.opacity(0.6))
        .child(kit::text(Type::Label, c.muted, "Party"))
        .child(items)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .flex_none()
                .children(source_status)
                .child(
                    IconButton::new("party-bar-hide", icons::CLOSE, "Hide the party").on_click(
                        cx.listener(|root, _, _, cx| {
                            root.app.sessions.party_bar = Some(false);
                            cx.notify();
                        }),
                    ),
                ),
        )
        .into_any_element()
}

/// One member: its portrait, first name, HP over its bar, temporary HP, its state's word and
/// its numbers. A press opens the sheet.
fn item(
    glance: &Glance,
    step: Step,
    at: usize,
    source: &str,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let first = glance.name.split_whitespace().next().unwrap_or_default();
    let name = if step.initial {
        first
            .chars()
            .next()
            .map(|letter| format!("{letter}."))
            .unwrap_or_default()
    } else {
        first.to_owned()
    };
    let mut facts = facts(glance);
    facts.push(source.to_owned());
    let label = format!("{} · {}", glance.name, facts.join(" · "));
    let mut row = div()
        .id(ElementId::Name(SharedString::from(format!(
            "party-bar-{at}"
        ))))
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .h(Density::Read.row())
        .px(XS)
        .rounded(crate::gui::theme::RADIUS_CONTROL)
        .cursor_pointer()
        .hover(|row| row.bg(c.hover))
        .aria_label(label.clone());
    if step.portrait {
        row = row.child(kit::portrait(
            glance.portrait.as_deref(),
            &glance.name,
            PORTRAIT,
        ));
    }
    row = row.child(div().pr(XS).child(kit::text(Type::Small, c.text, name)));
    row = row.child(
        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(kit::hp::readout(
                glance.hp,
                glance.max_hp,
                Type::Numeral,
                Type::Small,
            ))
            .child(kit::hp::bar(
                glance.hp,
                glance.max_hp,
                glance.temp,
                px(48.0),
            )),
    );
    if glance.temp > 0 {
        row = row.child(kit::hp::temp(glance.temp, Type::Small));
    }
    if let Some((icon, word, color)) = kit::hp::word(glance.hp, glance.max_hp) {
        row = row.child(
            div()
                .pl(XS)
                .flex()
                .flex_row()
                .items_center()
                .gap(XS)
                .child(
                    div()
                        .icon_font(Type::Small)
                        .text_color(color)
                        .child(SharedString::new_static(icon)),
                )
                .when(step.state_word, |row| {
                    row.child(kit::text(Type::Small, color, word))
                }),
        );
    }
    for (label, value) in stats(glance).into_iter().take(step.shown) {
        row = row.child(
            div()
                .pl(XS)
                .flex()
                .flex_row()
                .gap(px(3.0))
                .child(kit::text(Type::Small, c.muted, label))
                .child(kit::text(Type::Small, c.text, known(value))),
        );
    }
    let id = glance.id.clone();
    let row = row
        .tooltip(kit::tooltip::tip(SharedString::from(format!(
            "{}\n{}",
            glance.name,
            facts.join("\n")
        ))))
        .on_click(cx.listener(move |root, _, _, cx| {
            root.app.party.choose(id.clone());
            root.show_page(crate::Page::Party, cx);
        }));
    kit::target(row, label).into_any_element()
}
