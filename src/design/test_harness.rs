//! Test harnesses that draw a page with the app's theme from its first frame (issue #810).
//!
//! `egui_kittest`'s builders draw a page before the test's next line runs. A test that installs
//! the theme after `build_state` or `build_ui` measures a page's one-time results (an open jump,
//! a first fit, a kept measurement) at egui's default sizes; the theme's sizes apply only later.
//! The app installs its theme before its first frame. These builders do the same: their first
//! frame installs the theme and draws nothing (new fonts apply from the next frame), and the
//! page draws from the next frame on, as PR 807 did by hand.
//!
//! ```
//! use splat_design::design::test_harness::Themed;
//! let mut harness = egui_kittest::Harness::builder()
//!     .with_size(egui::vec2(400.0, 200.0))
//!     .themed_ui_state(|ui, words: &mut String| { ui.label(words.as_str()); }, "Ash".to_owned());
//! harness.run();
//! ```
//!
//! The design lint counts `theme::install(&harness.ctx)` per file (`late-theme`, a count that
//! can only shrink), so a new test builds its harness here instead.
use egui_kittest::{Harness, HarnessBuilder};

/// A harness builder whose page draws with the app's theme from its first frame on.
pub trait Themed<State> {
    /// `build_state` with the theme installed before the page's first frame.
    fn themed_state<'a>(
        self,
        app: impl FnMut(&egui::Context, &mut State) + 'a,
        state: State,
    ) -> Harness<'a, State>;

    /// `build_ui_state` with the theme installed before the page's first frame.
    fn themed_ui_state<'a>(
        self,
        app: impl FnMut(&mut egui::Ui, &mut State) + 'a,
        state: State,
    ) -> Harness<'a, State>;
}

impl<State> Themed<State> for HarnessBuilder<State> {
    fn themed_state<'a>(
        self,
        mut app: impl FnMut(&egui::Context, &mut State) + 'a,
        state: State,
    ) -> Harness<'a, State> {
        let mut themed = false;
        self.build_state(
            move |ctx, state| {
                if first_frame(ctx, &mut themed) {
                    return;
                }
                app(ctx, state);
            },
            state,
        )
    }

    fn themed_ui_state<'a>(
        self,
        mut app: impl FnMut(&mut egui::Ui, &mut State) + 'a,
        state: State,
    ) -> Harness<'a, State> {
        let mut themed = false;
        self.build_ui_state(
            move |ui, state| {
                if first_frame(ui.ctx(), &mut themed) {
                    return;
                }
                app(ui, state);
            },
            state,
        )
    }
}

/// The same for a page with no state of its own (`build` and `build_ui`).
pub trait ThemedPage {
    /// `build` with the theme installed before the page's first frame.
    fn themed<'a>(self, app: impl FnMut(&egui::Context) + 'a) -> Harness<'a>;

    /// `build_ui` with the theme installed before the page's first frame.
    fn themed_ui<'a>(self, app: impl FnMut(&mut egui::Ui) + 'a) -> Harness<'a>;
}

impl ThemedPage for HarnessBuilder {
    fn themed<'a>(self, mut app: impl FnMut(&egui::Context) + 'a) -> Harness<'a> {
        self.themed_state(move |ctx, ()| app(ctx), ())
    }

    fn themed_ui<'a>(self, mut app: impl FnMut(&mut egui::Ui) + 'a) -> Harness<'a> {
        self.themed_ui_state(move |ui, ()| app(ui), ())
    }
}

/// The first frame installs the theme, asks for the next frame, and draws nothing.
fn first_frame(ctx: &egui::Context, themed: &mut bool) -> bool {
    if std::mem::replace(themed, true) {
        return false;
    }
    super::theme::install(ctx);
    ctx.request_repaint();
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_kittest::kittest::Queryable;

    /// Issue #810: a page's first drawn frame already has the theme's sizes, and a one-time
    /// measurement taken on that frame is taken at them. The independent reference is the
    /// theme's own body size, read from a context the theme was installed on.
    #[test]
    fn a_page_first_draws_with_the_theme() {
        let reference = egui::Context::default();
        super::super::theme::install(&reference);
        let _ = reference.run(egui::RawInput::default(), |_| {});
        let body = reference.style().text_styles[&egui::TextStyle::Body].size;
        let plain = egui::Context::default().style().text_styles[&egui::TextStyle::Body].size;
        assert_ne!(body, plain, "the theme sets its own body size");
        let mut harness = egui_kittest::Harness::builder()
            .with_size(egui::vec2(400.0, 200.0))
            .themed_ui_state(
                |ui, first: &mut Option<f32>| {
                    // A one-time measurement: the first frame the page draws.
                    first
                        .get_or_insert_with(|| ui.style().text_styles[&egui::TextStyle::Body].size);
                    ui.label("Ash on the road");
                },
                None,
            );
        harness.run();
        assert_eq!(
            *harness.state(),
            Some(body),
            "measured at the theme's body size"
        );
        assert!(harness.query_by_label("Ash on the road").is_some());
    }
}
