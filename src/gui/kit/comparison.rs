//! A draft beside the file it would replace (`design::grouped_draft_comparison`): a summary line
//! (`3 lines differ · Under the heading “Rook”`) with `Next difference` while there is more than
//! one change, the two columns' headings, then the two texts aligned line by line (equal lines
//! side by side, a line on one side only facing a hatched `No line`), each differing row on the
//! selection fill with an accent edge. It opens on the first change, and `Next difference` moves
//! from change to change (a run of differing lines is one change), wrapping to the first.
//!
//! The alignment (`design::aligned`, the egui comparison's) is kept under the part's id and made
//! again only when either text changes; the rows are a measured virtual list, so a long note
//! costs a frame no more than a short one.
use super::button::Button;
use super::text::{label, text};
use crate::design::{Aligned, aligned};
use crate::gui::theme::{Density, HAIRLINE_WIDTH, S, SCROLL_BAR_WIDTH, Type, XS, colors};
use gpui::{
    App, Bounds, ElementId, IntoElement, ListAlignment, ListOffset, ListState, ParentElement,
    PathBuilder, Pixels, RenderOnce, SharedString, Styled, Window, canvas, div, point, prelude::*,
    px,
};
use std::rc::Rc;

/// The draft and the file now, side by side.
///
/// ```ignore
/// kit::DraftComparison::new(("note-conflict", note.id), &draft, &disk)
/// ```
#[derive(IntoElement)]
pub struct DraftComparison {
    id: ElementId,
    texts: [SharedString; 2],
    headings: [SharedString; 2],
    source: bool,
}

impl DraftComparison {
    /// `draft` is the DM's words, `disk` the file's now. Its state is kept under `id`: give it
    /// the identity of what it compares.
    pub fn new(
        id: impl Into<ElementId>,
        draft: impl Into<SharedString>,
        disk: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            texts: [draft.into(), disk.into()],
            headings: ["Your draft".into(), "In the file now".into()],
            source: false,
        }
    }

    /// Markdown source in `Mono` (a repair, an entry's source), not the note's prose.
    pub fn source(mut self) -> Self {
        self.source = true;
        self
    }

    /// The two columns' headings (default `Your draft`, `In the file now`).
    pub fn headings(
        mut self,
        draft: impl Into<SharedString>,
        disk: impl Into<SharedString>,
    ) -> Self {
        self.headings = [draft.into(), disk.into()];
        self
    }
}

/// What the part keeps between frames.
struct Kept {
    /// The texts the alignment was made for.
    texts: [SharedString; 2],
    aligned: Rc<Aligned>,
    list: ListState,
    /// The change shown (a row), and whether the list still has to scroll to it.
    current: Option<usize>,
    scroll: bool,
    /// The list's room in the last frame: a new room shows the change again.
    room: Option<gpui::Size<Pixels>>,
}

impl Kept {
    fn of(texts: [SharedString; 2]) -> Self {
        let aligned = Rc::new(aligned([texts[0].as_ref(), texts[1].as_ref()]));
        // One row more than the texts: the room after them (`row`), so the last change can come
        // to the top's third as the first does.
        let list =
            ListState::new(aligned.rows.len() + 1, ListAlignment::Top, px(200.0)).measure_all();
        Self {
            texts,
            current: aligned.groups.first().copied(),
            aligned,
            list,
            scroll: true,
            room: None,
        }
    }
}

/// The change after `current`, wrapping to the first.
fn next_change(groups: &[usize], current: Option<usize>) -> Option<usize> {
    let first = *groups.first()?;
    Some(
        groups
            .iter()
            .copied()
            .find(|&line| line > current.unwrap_or(first))
            .unwrap_or(first),
    )
}

/// The count, and the heading the change is under (the last `#` line of the draft's side at or
/// above it), in one sentence.
fn summary(aligned: &Aligned, current: Option<usize>) -> String {
    let count = if aligned.groups.is_empty() {
        "No lines differ".to_owned()
    } else {
        crate::design::count(aligned.changed, "line differs", "lines differ")
    };
    let line = current.unwrap_or(0);
    let heading = aligned
        .rows
        .iter()
        .take(line + 1)
        .rev()
        .filter_map(|row| row[0].as_deref())
        .find(|words| words.trim_start().starts_with('#'));
    match heading {
        Some(heading) => format!(
            "{count} · Under the heading “{}”",
            heading.trim_start_matches('#').trim()
        ),
        None => count,
    }
}

impl RenderOnce for DraftComparison {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let c = colors();
        let texts = self.texts.clone();
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| Kept::of(texts));
        state.update(cx, |kept, _| {
            if kept.texts != self.texts {
                let current = kept.current;
                *kept = Kept::of(self.texts.clone());
                // The same change stays shown while it still is one.
                if current.is_some_and(|line| kept.aligned.groups.contains(&line)) {
                    kept.current = current;
                }
            }
        });
        let (aligned, list, current) = {
            let kept = state.read(cx);
            (kept.aligned.clone(), kept.list.clone(), kept.current)
        };
        // Bring the change shown to the top's third: two rows of what comes before it stay.
        let room = list.viewport_bounds().size;
        state.update(cx, |kept, _| {
            if kept.room != Some(room) {
                kept.room = Some(room);
                kept.scroll = true;
            }
            if std::mem::take(&mut kept.scroll) {
                let item_ix = kept.current.map_or(0, |line| line.saturating_sub(2));
                kept.list.scroll_to(ListOffset {
                    item_ix,
                    offset_in_item: px(0.0),
                });
            }
        });
        let mut line = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(S)
            .min_h(Density::Read.control())
            .child(text(Type::Small, c.muted, summary(&aligned, current)));
        if aligned.groups.len() > 1 {
            let groups = aligned.groups.clone();
            let kept = state.clone();
            line = line.child(
                Button::quiet(
                    ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "next".into()),
                    "Next difference",
                )
                .density(Density::Read)
                .on_click(move |_, _, cx| {
                    kept.update(cx, |kept, cx| {
                        kept.current = next_change(&groups, kept.current);
                        kept.scroll = true;
                        cx.notify();
                    });
                }),
            );
        }
        let headings = div()
            .flex()
            .flex_row()
            .gap(S)
            .pr(SCROLL_BAR_WIDTH)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(label(self.headings[0].clone(), 0)),
            )
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .child(label(self.headings[1].clone(), 0)),
            );
        let ty = if self.source { Type::Mono } else { Type::Prose };
        let rows = aligned.clone();
        // egui's room after the rows: prose keeps three lines of the texts in view at the end;
        // source, one line.
        let after = if self.source {
            ty.line()
        } else {
            (room.height - ty.line() * 3.0).max(px(0.0))
        };
        let body = super::virtual_list::measured_column(
            ElementId::NamedChild(std::sync::Arc::new(self.id.clone()), "rows".into()),
            &list,
            move |at, _, _| row(&rows, at, ty, after),
        );
        div()
            .flex()
            .flex_col()
            .size_full()
            .gap(XS)
            .child(line)
            .child(headings)
            .child(div().flex_1().min_h_0().child(body))
    }
}

/// One aligned row: the two sides, each on the selection fill with an accent edge when they
/// differ, a missing line hatched.
fn row(aligned: &Aligned, at: usize, ty: Type, after: Pixels) -> gpui::AnyElement {
    let c = colors();
    let Some(sides) = aligned.rows.get(at) else {
        return div().h(after).into_any_element();
    };
    let differs = sides[0] != sides[1];
    let cell = |side: &Option<String>| {
        let cell = div()
            .flex_1()
            .min_w_0()
            .min_h(ty.line())
            .bg(if differs { c.selected } else { c.page })
            .when(differs, |cell| {
                cell.border_l(HAIRLINE_WIDTH).border_color(c.accent)
            })
            .pl(XS + S)
            .pr(XS);
        match side {
            Some(words) => cell.child(text(ty, c.text, words.clone())),
            None => super::target(cell.relative().child(hatch()), "No line"),
        }
    };
    div()
        .w_full()
        .flex()
        .flex_row()
        .items_stretch()
        .gap(S)
        .child(cell(&sides[0]))
        .child(cell(&sides[1]))
        .into_any_element()
}

/// A missing line's hatch: rule-coloured diagonals `S` apart across the cell.
fn hatch() -> impl IntoElement {
    canvas(
        |_, _, _| {},
        |bounds: Bounds<Pixels>, _, window, _| {
            let height = bounds.size.height;
            let mut x = bounds.left() - height;
            while x < bounds.right() {
                let mut path = PathBuilder::stroke(HAIRLINE_WIDTH);
                path.move_to(point(x, bounds.bottom()));
                path.line_to(point(x + height, bounds.top()));
                if let Ok(path) = path.build() {
                    window.with_content_mask(Some(gpui::ContentMask { bounds }), |window| {
                        window.paint_path(path, colors().rule);
                    });
                }
                x += S;
            }
        },
    )
    .absolute()
    .top_0()
    .left_0()
    .size_full()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_change_moves_through_the_groups_and_wraps() {
        let groups = [1, 4, 9];
        assert_eq!(next_change(&groups, Some(1)), Some(4));
        assert_eq!(next_change(&groups, Some(4)), Some(9));
        assert_eq!(
            next_change(&groups, Some(9)),
            Some(1),
            "after the last, the first"
        );
        assert_eq!(next_change(&[], None), None);
    }

    #[test]
    fn the_summary_names_the_heading_above_the_change() {
        let aligned = aligned([
            "# Rook\nold\n## Tonight\nsame\nmine",
            "# Rook\nnew\n## Tonight\nsame\ntheirs",
        ]);
        assert_eq!(aligned.groups, vec![1, 4]);
        assert_eq!(
            summary(&aligned, Some(1)),
            "2 lines differ · Under the heading “Rook”"
        );
        assert_eq!(
            summary(&aligned, Some(4)),
            "2 lines differ · Under the heading “Tonight”"
        );
        assert_eq!(
            summary(&crate::design::aligned(["a", "a"]), None),
            "No lines differ"
        );
    }
}
