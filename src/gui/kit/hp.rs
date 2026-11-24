//! HP at a glance (`design::hp_readout`, `hp_bar`, `hp_word`): `26/33` with the current HP in its
//! state's colour and the maximum muted, a thin bar with temporary HP as a segment above it, and
//! the state's word (Bloodied, 0 HP) so colour is never the only sign.
use crate::design::{HpState, hp_state};
use crate::gui::icons;
use crate::gui::theme::{Type, TypeExt, XS, colors};
use gpui::{Div, Hsla, ParentElement, Pixels, SharedString, Styled, div, px, relative};

/// The colour of HP in a state (`design::hp_color`).
pub fn hp_color(state: HpState) -> Hsla {
    let hp = colors().hp;
    match state {
        HpState::Unknown => hp.unknown,
        HpState::Healthy => hp.healthy,
        HpState::Damaged => hp.damaged,
        HpState::Bloodied => hp.bloodied,
        HpState::Zero => hp.zero,
    }
}

/// The current HP's colour in a readout (healthy HP is plain text).
fn current_color(state: HpState) -> Hsla {
    let c = colors();
    match state {
        HpState::Zero => c.hp.zero,
        HpState::Bloodied => c.hp.bloodied,
        HpState::Damaged => c.hp.damaged,
        HpState::Unknown => c.muted,
        HpState::Healthy => c.text,
    }
}

/// `26/33`: `current` in its role, `/33` in `max`. The maximum keeps a slot three digits wide,
/// so the slash holds its place from row to row.
pub fn readout(hp: Option<i64>, max: Option<i64>, current: Type, max_role: Type) -> Div {
    let c = colors();
    let state = hp_state(hp, max);
    let max_text = max.map_or_else(|| "?".to_owned(), |max| max.to_string());
    let slot = max_role.size() * 0.62 * (3.max(max_text.len()) as f32 + 1.0);
    div()
        .flex()
        .flex_row()
        .items_baseline()
        .flex_none()
        .child(
            div()
                .ty(current)
                .text_color(current_color(state))
                .child(hp.map_or_else(|| "?".to_owned(), |hp| hp.to_string())),
        )
        .child(
            div()
                .ty(max_role)
                .text_color(c.muted)
                .min_w(slot)
                .child(format!("/{max_text}")),
        )
}

/// A thin HP bar `width` wide, with temporary HP as a segment above its start.
pub fn bar(hp: Option<i64>, max: Option<i64>, temp: i64, width: Pixels) -> Div {
    let c = colors();
    let mut track = div()
        .relative()
        .w(width)
        .h(px(3.0))
        .rounded(px(1.0))
        .bg(c.hp.track);
    if let (Some(hp), Some(max)) = (hp, max)
        && max > 0
    {
        let fraction = (hp as f32 / max as f32).clamp(0.0, 1.0);
        track = track.child(
            div()
                .absolute()
                .top_0()
                .left_0()
                .h_full()
                .w(relative(fraction))
                .rounded(px(1.0))
                .bg(hp_color(hp_state(Some(hp), Some(max)))),
        );
        if temp > 0 {
            let part = (temp as f32 / max as f32).clamp(0.0, 1.0);
            track = track.child(
                div()
                    .absolute()
                    .top(px(-2.0))
                    .left_0()
                    .h(px(2.0))
                    .min_w(px(3.0))
                    .w(relative(part))
                    .bg(c.hp.temp),
            );
        }
    }
    track
}

/// The state's icon, word and colour, when it needs a word (`design::hp_word`).
pub fn word(hp: Option<i64>, max: Option<i64>) -> Option<(&'static str, &'static str, Hsla)> {
    let c = colors();
    match hp_state(hp, max) {
        HpState::Zero => Some((icons::ZERO_HP, "0 HP", c.hp.zero)),
        HpState::Bloodied => Some((icons::BLOODIED, "Bloodied", c.hp.bloodied)),
        HpState::Unknown => Some((icons::WARN, "HP unknown", c.muted)),
        _ => None,
    }
}

/// Temporary HP beside a readout: the shield and its number.
pub fn temp(temp: i64, role: Type) -> Div {
    let c = colors();
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(XS)
        .flex_none()
        .child(
            div()
                .icon_font(role)
                .text_color(c.hp.temp)
                .child(SharedString::new_static(icons::TEMP_HP)),
        )
        .child(div().ty(role).text_color(c.hp.temp).child(format!("+{temp}")))
}
