//! The session list (`Sessions::list_pane`): one row per note, its title wrapping over its date
//! or recording state, `Playing` on the current session's row. It waits while the open note's
//! unsaved words need a choice.
use crate::gui::{SplatRoot, bridge, icons, kit, theme};
use gpui::{
    AnyElement, Context, ElementId, IntoElement, ParentElement, SharedString, Styled, div,
    prelude::*,
};
use theme::{Density, M, S, Type, XS, colors};

pub(super) fn render(root: &mut SplatRoot, cx: &mut Context<SplatRoot>) -> AnyElement {
    let c = colors();
    let locked = root.app.sessions.gv_locked();
    let rows = root.app.sessions.gv_list_rows();
    let rows = rows.into_iter().map(|row| {
        let title_ty = if row.selected { Type::Strong } else { Type::Body };
        let mut detail = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_baseline()
            .gap_x(XS)
            .ty_small();
        if let Some(file) = &row.file {
            detail = detail.child(kit::text(Type::Small, c.muted, file.clone()).w_full());
        }
        let mut spoken = row.detail.clone();
        if let Some(state) = row.state {
            let look = kit::Status::new(state.icon, state.word, bridge::color(state.word_color()))
                .icon_color(bridge::color(state.icon_color()));
            detail = detail.child(kit::status(&look));
            spoken = row.spoken.clone().unwrap_or_else(|| state.word.to_owned());
        } else {
            detail = detail.child(kit::text(Type::Small, c.muted, row.detail.clone()));
        }
        if row.playing {
            detail = detail
                .child(kit::text(Type::Small, c.muted, "·"))
                .child(kit::text(
                    Type::Small,
                    c.muted,
                    format!("{} Playing", icons::RUN),
                ));
            spoken.push_str(" · Playing");
        }
        let name = match &row.file {
            Some(file) => format!("{} · {file}\n{spoken}", row.title),
            None => format!("{} · {spoken}", row.title),
        };
        let id = row.id;
        let content = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .py(M / 2.0)
            .child(kit::text(title_ty, c.text, row.title.clone()))
            .child(detail);
        let mut item = kit::Row::new(
            ElementId::Name(SharedString::from(format!("session-row-{id}"))),
            name,
        )
        .height(Density::Glance.control() + S)
        .grows()
        .selected(row.selected)
        .child(content);
        if !locked {
            item = item.on_click(cx.listener(move |root, _, _, cx| {
                root.app.sessions.gv_choose(id);
                cx.notify();
            }));
        }
        // A row's height follows its wrapped title (`session_row`), at least a Glance target.
        div().w_full().child(item).when(locked, |row| row.opacity(0.5))
    });
    kit::scroll_column(
        "session-list",
        div().flex().flex_col().w_full().gap(gpui::px(2.0)).children(rows),
    )
    .into_any_element()
}

trait SmallText {
    fn ty_small(self) -> Self;
}

impl SmallText for gpui::Div {
    fn ty_small(self) -> Self {
        use theme::TypeExt;
        self.ty(Type::Small)
    }
}
