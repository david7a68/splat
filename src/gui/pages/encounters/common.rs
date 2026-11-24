//! Parts the Encounters page's views share: the page's acts with the campaign, text measures,
//! footer lines, a row's `⋯` menu and a row's second line.
use crate::campaigns::Campaigns;
use crate::gui::theme::{self, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, bridge, icons, kit};
use crate::persistence::Campaign;
use eframe::egui;
use gpui::{
    AnyElement, App, ClickEvent, Context, Div, ElementId, Hsla, ParentElement, Pixels,
    SharedString, Styled, Window, div, px,
};
use splat_encounters_ui::encounters::Encounters;
use splat_encounters_ui::encounters::FooterPress;
use splat_encounters_ui::encounters::model::Line;

/// Run `act` on the page's state with the active campaign and the bridge's egui context (for
/// the state methods that start a read in the background). Nothing runs with no campaign.
pub(super) fn with_page(
    root: &mut SplatRoot,
    act: impl FnOnce(&mut Encounters, &egui::Context, &Campaign, &Campaigns),
) {
    let SplatRoot { app, bridge, .. } = root;
    let Some(active) = app.campaigns.active_campaign() else {
        return;
    };
    let mut act = Some(act);
    let encounters = &mut app.encounters;
    let campaigns = &app.campaigns;
    bridge.pass(|ctx| {
        if let Some(act) = act.take() {
            act(encounters, ctx, &active, campaigns);
        }
    });
}

/// A click handler that runs `act` on the page (`with_page`) and draws again.
pub(super) fn on_page(
    cx: &mut Context<SplatRoot>,
    act: impl Fn(&mut Encounters, &egui::Context, &Campaign, &Campaigns) + 'static,
) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    cx.listener(move |root: &mut SplatRoot, _: &ClickEvent, _, cx| {
        with_page(root, |page, ctx, active, campaigns| {
            act(page, ctx, active, campaigns)
        });
        cx.notify();
    })
}

/// The width of `words` set in `ty`, one line.
pub(super) fn text_width(window: &mut Window, ty: Type, words: &str) -> Pixels {
    if words.is_empty() {
        return px(0.0);
    }
    let font = gpui::Font {
        family: SharedString::new_static(ty.family()),
        features: gpui::FontFeatures::default(),
        fallbacks: None,
        weight: ty.weight(),
        style: ty.style(),
    };
    let run = gpui::TextRun {
        len: words.len(),
        font,
        color: colors().text,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    window
        .text_system()
        .shape_line(words.to_owned().into(), ty.size(), &[run], None)
        .width
}

/// A pane's footer lines (`design::outcome::plain`) as kit messages
/// (`kit::messages::lines`); each press goes to `Encounters::footer_press` with its ticket.
pub(super) fn footer(
    id: &'static str,
    lines: Vec<Line>,
    cx: &mut Context<SplatRoot>,
) -> Vec<AnyElement> {
    kit::messages::lines(
        id,
        lines,
        cx.listener(
            |root: &mut SplatRoot, pressed: &kit::messages::Pressed<FooterPress>, _, cx| {
                if let Some(press) = pressed.take() {
                    with_page(root, |page, ctx, active, campaigns| {
                        page.footer_press(ctx, press, active, campaigns)
                    });
                    cx.notify();
                }
            },
        ),
    )
}

/// A row's quiet `⋯` menu, named for the row (`More actions for <name>`).
pub(super) fn row_menu(id: impl Into<ElementId>, name: &str, items: Vec<kit::MenuItem>) -> Div {
    let id: ElementId = id.into();
    let trigger_id = ElementId::NamedChild(std::sync::Arc::new(id.clone()), "button".into());
    let words = format!("More actions for {name}");
    let mut menu = kit::MenuButton::new(
        id,
        kit::Button::quiet(trigger_id, icons::EXPAND)
            .plain()
            .name(words.clone())
            .hint(words),
    );
    for item in items {
        menu = menu.item(item);
    }
    div().flex_none().child(menu)
}

/// A row's second line: its parts in muted `Small`, the run icon first when `status`, and a
/// warning part (`Changed on disk`, `Offered in no fight`) with its icon in the warning colour.
pub(super) fn detail_line(detail: &str, status: bool) -> Div {
    let c = colors();
    let parts: Vec<&str> = detail.split(" · ").collect();
    let warning = parts
        .iter()
        .position(|part| matches!(*part, "Changed on disk" | "Offered in no fight"));
    let mut line = div()
        .flex()
        .flex_row()
        .items_baseline()
        .min_w_0()
        .overflow_hidden()
        .whitespace_nowrap()
        .ty(Type::Small)
        .text_color(c.muted);
    if status {
        line = line.child(small_icon(icons::RUN, c.muted).mr(XS));
    }
    if warning.is_none() {
        return line.child(detail.to_owned());
    }
    for (index, part) in parts.into_iter().enumerate() {
        if index > 0 {
            line = line.child(" · ");
        }
        if warning == Some(index) {
            line = line
                .child(small_icon(icons::WARN, c.result.warning).mr(XS))
                .child(div().text_color(c.result.warning).child(part.to_owned()));
        } else {
            line = line.child(part.to_owned());
        }
    }
    line
}

/// An icon at `Small`'s size.
pub(super) fn small_icon(glyph: &'static str, color: Hsla) -> Div {
    kit::icon(glyph, Type::Small, color)
}

/// Small muted words that wrap: a reason or a help line in a list.
pub(super) fn reason(words: impl Into<SharedString>) -> Div {
    kit::text(Type::Small, colors().muted, words)
}

/// An egui colour from a state method, as GPUI's.
pub(super) fn egui_color(color: egui::Color32) -> Hsla {
    bridge::color(color)
}

/// The list measure as a width cap.
pub(super) fn measure() -> Pixels {
    theme::LIST_MEASURE
}

/// A side choice (egui `side_control`): Party, Enemies, the encounter's other sides and
/// `Other…`, which opens a field for a new side's name. `id` keys it; `name` names it.
pub(super) fn side_choice(
    root: &SplatRoot,
    (id, name): (String, &'static str),
    side: &str,
    sides: &[String],
    on_pick: impl Fn(&mut SplatRoot, &str) + Clone + 'static,
    cx: &mut Context<SplatRoot>,
) -> Div {
    let mut named: Vec<String> = vec!["party".to_owned(), "enemies".to_owned()];
    for other in sides {
        if !named.contains(other) {
            named.push(other.clone());
        }
    }
    if !named.iter().any(|known| known == side) && !side.trim().is_empty() {
        named.push(side.to_owned());
    }
    let other_open = root.pages.encounters.other_side.as_deref() == Some(id.as_str());
    let chosen = if other_open {
        Some(named.len())
    } else {
        named.iter().position(|known| known == side)
    };
    let mut options: Vec<String> = named
        .iter()
        .map(|side| crate::design::reader::human_label(side))
        .collect();
    options.push("Other…".to_owned());
    let (picked, pick, open_id) = (named.clone(), on_pick.clone(), id.clone());
    let mut row = div().flex().flex_row().items_center().gap(theme::S).child(
        kit::Dropdown::new(ElementId::Name(id.clone().into()), name, options, chosen)
            .width(px(104.0))
            .density(theme::Density::Read)
            .on_select(cx.listener(move |root: &mut SplatRoot, at: &usize, _, cx| {
                match picked.get(*at) {
                    Some(side) => {
                        root.pages.encounters.other_side = None;
                        pick(root, side);
                    }
                    None => root.pages.encounters.other_side = Some(open_id.clone()),
                }
                cx.notify();
            })),
    );
    if other_open {
        row = row.child(
            kit::TextField::new(ElementId::Name(format!("{id}:other").into()), "")
                .name(format!("Other side for {name}"))
                .placeholder("Side name")
                .width(px(96.0))
                .focus(true)
                .on_change(cx.listener(move |root: &mut SplatRoot, words: &str, _, cx| {
                    on_pick(root, words);
                    cx.notify();
                })),
        );
    }
    row
}
