//! A status whose icon and words take different colours (package K2, gap K-7; ruling R5).
//!
//! `Status` paints its icon and its word in one colour. A [`KindStatus`] paints them apart, and
//! its [`StatusKind`] decides both colours and the icon: a closed set, so no caller passes a
//! free colour. Every status painter (`status`, `wrapped_status`, `rail_status_detail`,
//! `status_link`, `inline_status_link`) takes either, as a [`Tinted`] look.
use eframe::egui::Color32;

use super::{Status, colors, icons};

/// What a status painter paints: an icon and words, each in its own colour. It is made only
/// from a `Status` (one colour for both) or a [`KindStatus`] (its kind's colours).
///
/// ```
/// use splat::layout::{KindStatus, StatusKind, Tinted};
/// let _: Tinted = KindStatus::new(StatusKind::Recording, "Recording 12:03").into();
/// ```
///
/// Free colours are not a look:
///
/// ```compile_fail,E0451
/// let _ = splat::layout::Tinted {
///     icon: "●",
///     word: "Recording",
///     icon_color: eframe::egui::Color32::RED,
///     word_color: eframe::egui::Color32::WHITE,
/// };
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Tinted<'a> {
    pub(super) icon: &'a str,
    pub(super) word: &'a str,
    pub(super) icon_color: Color32,
    pub(super) word_color: Color32,
}

impl<'a> From<Status<'a>> for Tinted<'a> {
    fn from(status: Status<'a>) -> Self {
        Self {
            icon: status.icon,
            word: status.word,
            icon_color: status.color,
            word_color: status.color,
        }
    }
}

/// The statuses whose icon and words differ in colour. Each decides its icon and both colours.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusKind {
    /// A live recording: only its record icon in the `recording` token; its words (and its
    /// time) in the text colour (R5).
    Recording,
}

impl StatusKind {
    pub fn icon(self) -> &'static str {
        match self {
            Self::Recording => icons::RECORD,
        }
    }

    pub fn icon_color(self) -> Color32 {
        match self {
            Self::Recording => colors().recording,
        }
    }

    pub fn word_color(self) -> Color32 {
        match self {
            Self::Recording => colors().text,
        }
    }
}

/// A status of a [`StatusKind`], with its words (`Recording 12:03`).
///
/// ```compile_fail,E0061
/// let _ = splat::layout::KindStatus::new(
///     splat::layout::StatusKind::Recording,
///     "Recording",
///     eframe::egui::Color32::RED,
/// );
/// ```
#[derive(Clone, Copy, Debug)]
pub struct KindStatus<'a> {
    kind: StatusKind,
    word: &'a str,
}

impl<'a> KindStatus<'a> {
    pub fn new(kind: StatusKind, word: &'a str) -> Self {
        Self { kind, word }
    }
}

impl<'a> From<KindStatus<'a>> for Tinted<'a> {
    fn from(status: KindStatus<'a>) -> Self {
        Self {
            icon: status.kind.icon(),
            word: status.word,
            icon_color: status.kind.icon_color(),
            word_color: status.kind.word_color(),
        }
    }
}

#[cfg(test)]
mod tests {
    use eframe::egui::{self, Pos2, Rect, vec2};

    use super::*;
    use crate::design::{status, status_link, theme};

    /// The colour each painted part of `words` has, in the frame's text shapes.
    fn colours(output: &egui::FullOutput, part: &str) -> Vec<Color32> {
        let mut found = Vec::new();
        for clipped in &output.shapes {
            if let egui::Shape::Text(text) = &clipped.shape {
                let job = &text.galley.job;
                for section in &job.sections {
                    if job.text[section.byte_range.clone()].trim() == part {
                        found.push(section.format.color);
                    }
                }
            }
        }
        found
    }

    /// R5: a recording's status paints only its icon in the `recording` token, its words in the
    /// text colour, as a status and as a status link. A plain `Status` keeps one colour.
    #[test]
    fn a_kind_decides_its_icon_and_word_colours() {
        let ctx = egui::Context::default();
        theme::install(&ctx);
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(800.0, 400.0))),
            ..Default::default()
        };
        let output = ctx.run(input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                status(
                    ui,
                    KindStatus::new(StatusKind::Recording, "Recording 12:03"),
                );
                status_link(
                    ui,
                    KindStatus::new(StatusKind::Recording, "Recording 12:04"),
                );
                status(
                    ui,
                    Status {
                        icon: icons::OK,
                        word: "Saved",
                        color: colors().muted,
                    },
                );
            });
        });
        let c = colors();
        assert_eq!(colours(&output, icons::RECORD), [c.recording, c.recording]);
        assert_eq!(colours(&output, "Recording 12:03"), [c.text]);
        assert_eq!(colours(&output, "Recording 12:04"), [c.text]);
        assert_eq!(colours(&output, icons::OK), [c.muted]);
        assert_eq!(colours(&output, "Saved"), [c.muted]);
    }
}
