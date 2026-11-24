//! The member list (egui: `member_header`, `member_row`): to compare the party at a glance,
//! each member's portrait, name, class and level, conditions, the compare columns (passive
//! Perception and Insight, spell DC), AC and HP with its bar.
use crate::gui::kit::{self, Row};
use crate::gui::theme::{Density, S, Type, TypeExt, XS, colors};
use crate::gui::{SplatRoot, icons};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, Pixels, SharedString, Styled, div,
    px,
};

const AC_WIDTH: Pixels = px(34.0);
const HP_WIDTH: Pixels = px(112.0);
const MEMBER_ROW: Pixels = px(54.0);
const ROW_PORTRAIT: Pixels = px(38.0);
const COMPARE: [&str; 3] = ["PER", "INS", "DC"];
const COMPARE_WORDS: [&str; 3] = ["Per", "Ins", "DC"];
const COMPARE_WIDTH: Pixels = px(30.0);
/// A list narrower than this has no room for the compare columns: they take a line.
const COMPARE_FROM: Pixels = px(480.0);

fn numbers(cells: impl IntoIterator<Item = (Pixels, AnyElement)>) -> gpui::Div {
    div().flex().flex_row().items_center().flex_none().children(
        cells
            .into_iter()
            .map(|(width, cell)| div().w(width).flex().justify_end().child(cell)),
    )
}

/// The list's header, its rule and its rows, in a column that scrolls.
pub(super) fn list(root: &mut SplatRoot, width: Pixels, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let wide = width - crate::gui::theme::M >= COMPARE_FROM;
    let label = |words: &'static str| kit::text(Type::Label, c.muted, words).into_any_element();
    let mut header_cells: Vec<(Pixels, AnyElement)> = Vec::new();
    if wide {
        for word in COMPARE {
            header_cells.push((COMPARE_WIDTH, label(word)));
        }
    }
    header_cells.push((AC_WIDTH, label("AC")));
    header_cells.push((HP_WIDTH, label("HP")));
    let header = div()
        .flex()
        .flex_row()
        .items_center()
        .h(px(18.0))
        .px(S)
        .child(div().flex_1().child(label("NAME")))
        .child(numbers(header_cells));
    let party = &root.app.party;
    let reader = &party.reader;
    let selected = reader.selected.clone();
    let mut rows = div().flex().flex_col();
    for member in &reader.members {
        let record = reader.sheets.get(&member.id);
        let portrait = record.and_then(|record| {
            splat_party::party_sync::portrait_path(reader.local_scope()?, record)
        });
        let from_fights = record.and_then(|record| party.tonight_values(record));
        let local_edits = reader.conflicts.contains_key(&member.id);
        let compare = splat_party_page::compare_values(record);
        let level = record.map(|r| r.character.level).or(member.level);
        let classes = record.map_or_else(String::new, |r| {
            r.character
                .classes
                .iter()
                .filter_map(|class| class["name"].as_str())
                .collect::<Vec<_>>()
                .join(" / ")
        });
        let conditions = record.map_or_else(Vec::new, splat_party::party::sheet_conditions);
        let summary = if record.is_none() {
            "No local copy yet".to_owned()
        } else {
            splat_party_page::classes_and_level(&classes, level)
        };
        let hp = from_fights
            .map(|(hp, _)| hp)
            .or_else(|| record.map(|r| r.initial.hp));
        let max_hp = record.and_then(|r| r.stat("max_hp"));
        let ac = record.and_then(|r| r.stat("armor_class"));
        let temp =
            from_fights.map_or_else(|| record.map_or(0, |r| r.initial.temporary_hp), |v| v.1);
        let compared = COMPARE_WORDS
            .iter()
            .zip(&compare)
            .filter(|(word, value)| **word != "DC" || value.as_str() != "—")
            .map(|(word, value)| format!("{word} {value}"))
            .collect::<Vec<_>>()
            .join(" · ");
        let known = |n: Option<i64>| n.map_or_else(|| "?".to_owned(), |n| n.to_string());
        let name = format!(
            "{}{} · {summary} · {compared} · HP {} · AC {}{}",
            member.name,
            if local_edits { " · Local edits" } else { "" },
            splat_party::combat_sync::fraction(known(hp), known(max_hp)),
            known(ac),
            conditions
                .iter()
                .map(|condition| format!(" · {condition}"))
                .collect::<String>(),
        );
        let mut detail = div()
            .flex()
            .flex_row()
            .items_center()
            .overflow_hidden()
            .whitespace_nowrap()
            .child(kit::text(Type::Small, c.muted, summary));
        if local_edits {
            detail = detail
                .child(kit::text(Type::Small, c.muted, " · "))
                .child(kit::status(&kit::Status::new(
                    icons::DRAFT,
                    "Local edits",
                    c.document.unsaved,
                )));
        }
        match crate::design::hp_state(hp, max_hp) {
            crate::design::HpState::Bloodied => {
                detail = detail
                    .child(kit::text(Type::Small, c.muted, " · "))
                    .child(kit::status(&kit::Status::new(
                        icons::BLOODIED,
                        "Bloodied",
                        c.hp.bloodied,
                    )));
            }
            crate::design::HpState::Zero => {
                detail = detail.child(kit::text(Type::Small, c.hp.zero, " · 0 HP"));
            }
            _ => {}
        }
        for condition in &conditions {
            detail = detail.child(kit::text(
                Type::Small,
                c.participant.condition,
                format!(" · {condition}"),
            ));
        }
        let mut cells: Vec<(Pixels, AnyElement)> = Vec::new();
        if wide {
            for value in &compare {
                cells.push((
                    COMPARE_WIDTH,
                    kit::text(Type::Numeral, c.muted, value.clone()).into_any_element(),
                ));
            }
        }
        cells.push((
            AC_WIDTH,
            kit::text(Type::Numeral, c.text, known(ac)).into_any_element(),
        ));
        let mut readout = div().flex().flex_row().items_baseline().gap(S);
        if temp > 0 {
            readout = readout.child(kit::hp::temp(temp, Type::Numeral));
        }
        readout = readout.child(kit::hp::readout(hp, max_hp, Type::NumeralLarge, Type::Numeral));
        cells.push((
            HP_WIDTH,
            div()
                .flex()
                .flex_col()
                .items_end()
                .gap(XS)
                .child(readout)
                .child(kit::hp::bar(hp, max_hp, temp, px(86.0)))
                .into_any_element(),
        ));
        let id = member.id.clone();
        let is_selected = selected.as_deref() == Some(member.id.as_str());
        let row = Row::new(
            ElementId::Name(SharedString::from(format!("party-member-{}", member.id))),
            name,
        )
        .height(if wide { MEMBER_ROW } else { MEMBER_ROW + px(16.0) })
        .selected(is_selected)
        .on_click(cx.listener(move |root, _, _, cx| {
            root.app.party.reader.select(id.clone());
            root.pages.party.search.clear();
            cx.notify();
        }))
        .child(kit::portrait(portrait.as_deref(), &member.name, ROW_PORTRAIT))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .child(kit::text::one_line(Type::Name, c.text, member.name.clone()))
                .child(detail)
                .when_some((!wide).then_some(compared), |column, compared| {
                    column.child(kit::text(Type::Small, c.muted, compared))
                }),
        )
        .child(numbers(cells));
        rows = rows.child(
            div()
                .border_b(crate::gui::theme::HAIRLINE_WIDTH)
                .border_color(c.hairline)
                .child(row),
        );
    }
    let _ = Density::Read;
    div()
        .flex()
        .flex_col()
        .size_full()
        .pr(crate::gui::theme::M)
        .child(header)
        .child(kit::rule())
        .child(
            div()
                .flex_1()
                .min_h_0()
                .child(kit::scroll_column("party_members_list", rows)),
        )
        .into_any_element()
}

trait WhenSome: Sized {
    fn when_some<T>(self, value: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self;
}

impl WhenSome for gpui::Div {
    fn when_some<T>(self, value: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self {
        match value {
            Some(value) => then(self, value),
            None => self,
        }
    }
}

#[allow(dead_code)]
fn _type_ext_in_use(div: gpui::Div) -> gpui::Div {
    div.ty(Type::Body)
}
