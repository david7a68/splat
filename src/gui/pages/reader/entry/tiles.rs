//! An entry's actions as tiles (egui `rolls::tile_row`): each tile its icon, name and key number,
//! then its numbers and where it lands; its rules text on hover. A press asks for the roll (Shift
//! for advantage, Ctrl for disadvantage), which the run or the preview stream takes
//! (`rolls::take_ask`); a spell's open tile opens its entry. Under the tiles, in a run, the armed
//! action's status and what can follow it; outside a run, the last preview's result.
use super::OnOpen;
use crate::gui::kit;
use crate::gui::theme::{HAIRLINE_WIDTH, RADIUS_CONTROL, S, Type, TypeExt, XS, colors};
use crate::rolls::plain::{self, TileView};
use crate::rolls::{Chip, SpellOccurrence};
use eframe::egui;
use gpui::{
    AnyElement, ElementId, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, div, px,
};
use splat::content::dice_text::RollLink;

/// What a tile's press does.
#[derive(Clone)]
pub enum Press {
    /// Ask for a roll, with the spell it casts.
    Roll {
        link: RollLink,
        occurrence: Option<SpellOccurrence>,
    },
    /// Open the entry at this place in the reader's links.
    Open(usize),
}

/// A row of tiles and what is under them, then `S`. `chips` are the rolled tiles' actions, which
/// say whether one of them is armed or was previewed.
pub fn tile_row(
    id: &ElementId,
    ctx: Option<&egui::Context>,
    entry: &str,
    tiles: Vec<(TileView, Press)>,
    chips: &[Chip],
    on_open: Option<OnOpen>,
) -> AnyElement {
    if tiles.is_empty() {
        return div().into_any_element();
    }
    let c = colors();
    let row =
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(S)
            .children(tiles.into_iter().enumerate().map(|(at, (view, press))| {
                tile(
                    super::parts::child(id, format!("tile-{at}")),
                    view,
                    press,
                    ctx.cloned(),
                    on_open.clone(),
                )
            }));
    let mut block = div()
        .flex()
        .flex_col()
        .pt(XS)
        .child(row)
        .child(kit::gap(XS));
    let refs: Vec<&Chip> = chips.iter().collect();
    if let Some(ctx) = ctx {
        if let Some(view) = plain::armed_under(ctx, &refs) {
            // What to do now, then what can follow, each on its own line.
            block = block
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_start()
                        .gap(XS)
                        .child(kit::icon(crate::gui::icons::ARMED, Type::Body, c.accent))
                        .child(kit::text(Type::Small, c.text, view.status.clone())),
                )
                .child(div().flex().flex_row().flex_wrap().gap(S).children(
                    view.actions.iter().enumerate().map(|(at, (word, action))| {
                        let ctx = ctx.clone();
                        let action = *action;
                        kit::Button::quiet(super::parts::child(id, format!("armed-{at}")), *word)
                            .line_start()
                            .on_click(move |_, window, _| {
                                plain::send_armed_action(&ctx, action);
                                window.refresh();
                            })
                    }),
                ));
        } else {
            let links: Vec<&RollLink> = chips.iter().map(|chip| &chip.link).collect();
            if let Some(parts) = plain::preview_words(ctx, entry, &links) {
                let mut line = div().flex().flex_row().flex_wrap().items_center().gap(S);
                for (at, part) in parts.into_iter().enumerate() {
                    if at > 0 {
                        line = line.child(kit::text(Type::Body, c.muted, "·"));
                    }
                    line = line.child(kit::text(Type::Strong, c.text, part));
                }
                block = block.child(line);
            }
        }
    }
    block.child(kit::gap(S)).into_any_element()
}

/// One tile: its frame, its icon and name, its keys at the right, its numbers and its facts.
fn tile(
    id: ElementId,
    view: TileView,
    press: Press,
    ctx: Option<egui::Context>,
    on_open: Option<OnOpen>,
) -> AnyElement {
    let c = colors();
    let edge = if view.armed { c.accent } else { c.rule };
    let tip = view.tip.join("\n\n");
    let name = view.name.clone();
    let element = div()
        .id(id)
        .w(px(plain::TILE_WIDTH))
        .flex()
        .flex_col()
        .px(S)
        .py(XS)
        .rounded(RADIUS_CONTROL)
        .border(HAIRLINE_WIDTH)
        .border_color(edge)
        .bg(if view.armed { c.selected } else { c.page })
        .cursor_pointer()
        .hover(move |s| {
            if view.armed {
                s
            } else {
                s.border_color(c.focus)
            }
        })
        .aria_label(view.label.clone())
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(XS)
                .h(Type::Strong.line())
                .child(kit::icon(
                    view.icon,
                    Type::Body,
                    if view.armed { c.accent } else { c.text },
                ))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .ty(Type::Strong)
                        .text_color(c.text)
                        .truncate()
                        .child(view.name.clone()),
                )
                .children(
                    view.keys
                        .map(|keys| div().ty(Type::Small).text_color(c.faint).child(keys)),
                ),
        )
        .child(kit::text(Type::Small, c.muted, view.detail))
        .children(
            view.facts
                .map(|facts| kit::text(Type::Small, c.muted, facts)),
        )
        .with_tooltip(tip)
        .on_click(move |_, window, cx| match &press {
            Press::Roll { link, occurrence } => {
                if let Some(ctx) = &ctx {
                    let keys = window.modifiers();
                    plain::ask_pressed_at(
                        ctx,
                        link,
                        *occurrence,
                        keys.shift,
                        keys.control || keys.platform,
                    );
                    window.refresh();
                }
            }
            Press::Open(place) => {
                if let Some(on_open) = &on_open {
                    on_open(*place, window, cx);
                }
            }
        });
    kit::target(element, name).into_any_element()
}

use kit::WithTooltip;
