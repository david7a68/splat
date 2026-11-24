//! The party at a glance, above the Session page while a session is played. Each member is a portrait, a first name, HP with its
//! bar and a few numbers; a press opens the sheet. When the line is short of room, the optional
//! numbers go first, then the portraits, then the names shorten to an initial; no item is cut,
//! and `N more` names the rest.
use crate::gui::kit::{self, IconButton, WithTooltip as _};
use crate::gui::theme::{Density, L, M, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, icons, theme};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, Pixels, SharedString, Styled,
    Window, div, prelude::*, px,
};
use splat_party::party_bar::{self, Glance, Measured, Names, Step, known};

const PORTRAIT: Pixels = px(24.0);

/// A first name, or its initial (`B.`) when even first names do not fit.
fn name_words(glance: &Glance, initial: bool) -> String {
    let first = glance.name.split_whitespace().next().unwrap_or_default();
    if initial {
        first
            .chars()
            .next()
            .map(|letter| format!("{letter}."))
            .unwrap_or_default()
    } else {
        first.to_owned()
    }
}

/// The maximum's slot in the HP readout, measured as egui measured it: three digits at least,
/// or the `/max` words when they are wider.
fn max_slot(window: &Window, glance: &Glance) -> Pixels {
    let max = known(glance.max_hp);
    kit::measure::width(window, Type::Small, &"0".repeat(3.max(max.len()))).max(
        kit::measure::width(
            window,
            Type::Small,
            &splat_party::combat_sync::fraction("", &max),
        ),
    )
}

/// The width of the HP readout (`kit::hp::readout_in_slot` at `Numeral` over `Small`): the
/// current HP, then the maximum's measured slot.
fn hp_width(window: &Window, glance: &Glance) -> Pixels {
    kit::measure::width(window, Type::Numeral, &known(glance.hp)) + max_slot(window, glance)
}

/// A stat as it reads: its label, a space, its value.
fn stat_words(label: &str, value: Option<i64>) -> String {
    format!("{label} {}", known(value))
}

/// One member's parts, measured in the bar's type (`party_bar::Item::measure`).
fn measure(window: &Window, glance: &Glance) -> Measured {
    let small = |words: &str| f32::from(kit::measure::width(window, Type::Small, words));
    let first = glance.name.split_whitespace().next().unwrap_or_default();
    let state = kit::hp::word(glance.hp, glance.max_hp);
    let icon = |glyph: &str| f32::from(kit::measure::icon_width(window, Type::Small, glyph));
    Measured {
        first: (!first.is_empty()).then(|| small(first)),
        initial: (!first.is_empty()).then(|| small(&name_words(glance, true))),
        hp: f32::from(hp_width(window, glance)),
        temp: (glance.temp > 0)
            .then(|| icon(icons::TEMP_HP) + f32::from(XS) + small(&format!("+{}", glance.temp))),
        full_word: state.map(|(glyph, word, _)| icon(glyph) + f32::from(XS) + small(word)),
        icon_word: state.map(|(glyph, _, _)| icon(glyph)),
        stats: glance
            .stats()
            .into_iter()
            .map(|(label, value)| small(&stat_words(label, value)))
            .collect(),
    }
}

/// The bar: each member measured by GPUI's text layout and fitted (`party_bar::fit`). While a fight is open, the bar holds its level and its item widths until
/// the fight ends, so nothing moves back as HP changes.
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
    // While a fight is open, the bar holds its level until the fight ends.
    let fight = app
        .encounters
        .run()
        .filter(|run| !run.ended())
        .map(crate::encounter_run::Run::id);
    let inert = app.copy_dialog.is_some() || app.confirm_close;
    let bar_width =
        window.viewport_size().width - crate::gui::rail::width(root, window, cx) - M * 2.0;
    let room = bar_width
        - kit::measure::width(window, Type::Label, "PARTY")
        - L
        - Density::Read.control()
        - S;
    let gap = M;
    let more_width =
        |hidden: usize| kit::measure::width(window, Type::Small, &format!("{hidden} more"));
    let measured: Vec<Measured> = glances.iter().map(|g| measure(window, g)).collect();
    let fitted = party_bar::fit(
        &measured,
        party_bar::Room {
            room: f32::from(room),
            gap: f32::from(gap),
            bar_width: f32::from(bar_width),
            fight,
            party: glances.iter().map(|glance| glance.id.clone()).collect(),
        },
        |hidden| f32::from(more_width(hidden)),
        root.pages.party.bar_hold.as_ref(),
    );
    root.pages.party.bar_hold = fitted.hold.clone();
    let (step, kept) = (fitted.step, fitted.kept);
    // The source is measured by itself, after the items are fitted: it shows whenever the room
    // they leave holds it, in full or else in its short form. It never takes room from an item.
    let hidden_count = glances.len() - kept;
    let used = px(fitted.widths[..kept].iter().sum::<f32>())
        + gap * kept.saturating_sub(1) as f32
        + if hidden_count == 0 {
            px(0.0)
        } else {
            gap + more_width(hidden_count) + XS * 2.0
        };
    let free = room - used - L;
    let source_width = |words: &str| {
        kit::measure::icon_width(window, Type::Small, source.icon)
            + XS
            + kit::measure::width(window, Type::Small, words)
    };
    let shown_source = [source.word.as_str(), source.short]
        .into_iter()
        .find(|words| source_width(words) <= free)
        .map(str::to_owned);
    // The items' line is the room they were fitted to, less the source beside the close button,
    // so a fitted item is never clipped by a label or button drawn a pixel wider than measured.
    let line_width = room
        - shown_source
            .as_deref()
            .map_or(px(0.0), |words| source_width(words) + S);
    let mut items = div()
        .flex()
        .flex_row()
        .items_center()
        .gap(gap)
        .flex_none()
        .w(line_width)
        .overflow_hidden();
    for (at, (glance, width)) in glances.iter().zip(&fitted.widths).take(kept).enumerate() {
        items = items.child(item(glance, step, px(*width), at, &source.word, window, cx));
    }
    let hidden: Vec<&Glance> = glances.iter().skip(kept).collect();
    if !hidden.is_empty() {
        let lines: Vec<String> = hidden
            .iter()
            .map(|glance| format!("{} · {}", glance.name, glance.facts()[0]))
            .collect();
        let words = lines.join("; ");
        // It takes keyboard focus. Hover and focus show the players it hides, each with HP, and
        // its accessible name lists them too. GPUI's tooltip shows on hover only, so focus
        // draws the same box under the count.
        let count = format!("{} more", hidden.len());
        let focus = root
            .pages
            .party
            .more_focus
            .get_or_insert_with(|| cx.focus_handle().tab_stop(true))
            .clone();
        let more = div()
            .id("party-bar-more")
            .flex()
            .items_center()
            .flex_none()
            .h(Density::Read.row())
            // The ring is inside the measured `XS` sides, so the fit's width holds.
            .px(XS - theme::FOCUS_WIDTH)
            .rounded(theme::RADIUS_CONTROL)
            .border(theme::FOCUS_WIDTH)
            .border_color(gpui::transparent_black())
            .track_focus(&focus)
            .focus_visible(|s| s.border_color(c.focus))
            .aria_label(format!("{count}: {words}"))
            .child(kit::text(Type::Small, c.muted, count.clone()))
            .with_tooltip(lines.join("\n"))
            .when(focus.is_focused(window), |more| {
                // Under the count, its right edge at the count's: the count ends the items, so
                // the box opens toward them and stays in the window.
                more.relative().child(
                    div().absolute().right_0().bottom_0().child(
                        gpui::deferred(gpui::anchored().anchor(gpui::Anchor::TopRight).child(
                            kit::target(
                                kit::tooltip::tip_box(lines.join("\n").into()),
                                "party-bar-more-list",
                            ),
                        ))
                        .with_priority(1),
                    ),
                )
            });
        items = items.child(kit::target(more, count));
    }
    let source_status = shown_source.map(|words| {
        let status = div()
            .id("party-bar-source")
            .child(kit::status(&kit::Status::new(
                source.icon,
                words.clone(),
                theme::color(source.color),
            )))
            .with_tooltip(source.word.clone());
        kit::target(status, words)
    });
    div()
        .flex()
        .flex_row()
        .items_center()
        .h(Density::Read.row() + XS * 2.0)
        .px(M)
        .py(XS)
        .bg(c.page)
        .border_b(crate::gui::theme::HAIRLINE_WIDTH)
        .border_color(c.hairline)
        .when(inert, |bar| bar.opacity(0.6))
        // The spaces the fit's room counts: `L` after the label, `S` before the close button.
        .child(div().mr(L).child(kit::text(Type::Label, c.muted, "Party")))
        .child(kit::target(items, "party-bar-items"))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(S)
                .ml(S)
                .flex_1()
                .min_w_0()
                .justify_end()
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
    width: Pixels,
    at: usize,
    source: &str,
    window: &Window,
    cx: &mut Context<SplatRoot>,
) -> AnyElement {
    let c = colors();
    let name = name_words(glance, step.names == Names::Initial);
    let mut facts = glance.facts();
    facts.push(source.to_owned());
    let label = format!("{} · {}", glance.name, facts.join(" · "));
    let mut row = div()
        .id(ElementId::Name(SharedString::from(format!(
            "party-bar-{at}"
        ))))
        .flex()
        .flex_row()
        .flex_none()
        .items_center()
        .w(width)
        .overflow_hidden()
        .h(Density::Read.row())
        .px(XS)
        .rounded(crate::gui::theme::RADIUS_CONTROL)
        .cursor_pointer()
        .hover(|row| row.bg(c.hover))
        .aria_label(label.clone());
    if step.portrait {
        row = row.child(div().mr(XS).child(kit::portrait(
            glance.portrait.as_deref(),
            &glance.name,
            PORTRAIT,
        )));
    }
    if !name.is_empty() {
        row = row.child(
            div()
                .flex_none()
                .mr(S)
                .child(kit::text(Type::Small, c.text, name)),
        );
    }
    let hp = hp_width(window, glance);
    row = row.child(
        div()
            .flex()
            .flex_col()
            .flex_none()
            .w(hp)
            .gap(px(2.0))
            .child(kit::hp::readout_in_slot(
                glance.hp,
                glance.max_hp,
                Type::Numeral,
                Type::Small,
                max_slot(window, glance),
            ))
            .child(kit::hp::bar(glance.hp, glance.max_hp, glance.temp, hp)),
    );
    if glance.temp > 0 {
        row = row.child(div().ml(XS).child(kit::hp::temp(glance.temp, Type::Small)));
    }
    if let Some((icon, word, color)) = kit::hp::word(glance.hp, glance.max_hp) {
        row = row.child(
            div()
                .ml(S)
                .flex()
                .flex_row()
                .flex_none()
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
    for (label, value) in glance.stats().into_iter().take(step.shown) {
        // One run of words, as it was measured: its muted label, a space, its value.
        row = row.child(
            div()
                .ml(S)
                .flex_none()
                .whitespace_nowrap()
                .ty(Type::Small)
                .text_color(c.text)
                .child(
                    gpui::StyledText::new(stat_words(label, value)).with_highlights([(
                        0..label.len(),
                        gpui::HighlightStyle {
                            color: Some(c.muted),
                            ..Default::default()
                        },
                    )]),
                ),
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
