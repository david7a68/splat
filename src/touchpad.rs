//! Touchpad scrolling speed. egui moves content one point for each unit the system sends,
//! as macOS expects, where the system adds speed and momentum itself. On Linux it does not:
//! Firefox moves about 4 pixels for each unit and Chromium-based apps about 12, so Splat
//! scrolled far less than they did for the same swipe. The DM chooses the factor.
use eframe::egui;
use serde::{Deserialize, Deserializer, Serialize};

/// How far content moves for each unit of touchpad scrolling. Mouse wheels keep their speed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum TouchpadSpeed {
    /// As the system sends it.
    One,
    Two,
    /// About as far as Firefox scrolls.
    #[default]
    Four,
    Eight,
    /// About as far as Obsidian and other Chromium-based apps scroll.
    Twelve,
}

impl TouchpadSpeed {
    pub const ALL: [(Self, &'static str); 5] = [
        (Self::One, "1×"),
        (Self::Two, "2×"),
        (Self::Four, "4×"),
        (Self::Eight, "8×"),
        (Self::Twelve, "12×"),
    ];

    pub fn factor(self) -> f32 {
        match self {
            Self::One => 1.0,
            Self::Two => 2.0,
            Self::Four => 4.0,
            Self::Eight => 8.0,
            Self::Twelve => 12.0,
        }
    }

    /// Touchpads report scrolling in points and mouse wheels in lines, so only points change.
    /// With the zoom modifier held, a scroll zooms, and zoom keeps its own speed.
    ///
    /// winit does not say what sent a scroll in points. On-button scrolling (a TrackPoint, or
    /// a mouse moved with a button held) also arrives in points, so it gets the same factor.
    ///
    /// egui spreads a step of 8 points or more over the next frames, as it does for a mouse
    /// wheel's notch, and a touchpad would feel that as lag. Smaller steps move at once, so
    /// a faster step goes to egui in parts.
    pub fn apply(self, events: &mut Vec<egui::Event>) {
        let factor = self.factor();
        if factor == 1.0 {
            return;
        }
        let mut faster = Vec::with_capacity(events.len());
        for event in events.drain(..) {
            let egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta,
                modifiers,
                ..
            } = &event
            else {
                faster.push(event);
                continue;
            };
            if modifiers.command {
                faster.push(event);
                continue;
            }
            let delta = *delta * factor;
            let parts = (delta.length() / LARGEST_STEP)
                .ceil()
                .clamp(1.0, MOST_PARTS);
            for _ in 0..parts as usize {
                let mut part = event.clone();
                if let egui::Event::MouseWheel { delta: step, .. } = &mut part {
                    *step = delta / parts;
                }
                faster.push(part);
            }
        }
        *events = faster;
    }
}

/// Below egui's 8-point smoothing threshold.
const LARGEST_STEP: f32 = 7.5;
/// A very fast swipe at 12× can be hundreds of points. Past this many parts, egui smooths the
/// rest rather than taking more events.
const MOST_PARTS: f32 = 64.0;

/// Any value this version does not know, of any type, reads as the default, so the rest of
/// the file still opens.
impl<'de> Deserialize<'de> for TouchpadSpeed {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        Ok(match value.as_str() {
            Some("One") => Self::One,
            Some("Two") => Self::Two,
            Some("Eight") => Self::Eight,
            Some("Twelve") => Self::Twelve,
            _ => Self::Four,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wheel(unit: egui::MouseWheelUnit, y: f32, modifiers: egui::Modifiers) -> egui::Event {
        egui::Event::MouseWheel {
            unit,
            delta: egui::vec2(0.0, y),
            modifiers,
        }
    }

    /// The scroll in these units, and with or without the zoom modifier.
    fn total(events: &[egui::Event], wanted: egui::MouseWheelUnit, zoom: bool) -> f32 {
        events
            .iter()
            .filter_map(|event| match event {
                egui::Event::MouseWheel {
                    unit,
                    delta,
                    modifiers,
                    ..
                } if *unit == wanted && modifiers.command == zoom => Some(delta.y),
                _ => None,
            })
            .sum()
    }

    #[test]
    fn only_touchpad_scrolls_move_further() {
        let none = egui::Modifiers::NONE;
        let mut events = vec![
            wheel(egui::MouseWheelUnit::Point, -10.0, none),
            wheel(egui::MouseWheelUnit::Line, -1.0, none),
            wheel(egui::MouseWheelUnit::Page, 1.0, none),
            wheel(egui::MouseWheelUnit::Point, 5.0, egui::Modifiers::COMMAND),
            egui::Event::Zoom(1.5),
        ];
        TouchpadSpeed::Eight.apply(&mut events);
        let point = egui::MouseWheelUnit::Point;
        assert!(
            (total(&events, point, false) + 80.0).abs() < 1e-3,
            "a touchpad scroll"
        );
        assert_eq!(
            total(&events, egui::MouseWheelUnit::Line, false),
            -1.0,
            "a mouse wheel"
        );
        assert_eq!(
            total(&events, egui::MouseWheelUnit::Page, false),
            1.0,
            "a page step"
        );
        assert_eq!(total(&events, point, true), 5.0, "a zoom");
        assert!(events.contains(&egui::Event::Zoom(1.5)));
        let before = events.clone();
        TouchpadSpeed::One.apply(&mut events);
        assert_eq!(events, before);
    }

    /// egui smooths a large step over the next frames. A faster touchpad must still move the
    /// whole way in the frame its scroll arrives, as it did before.
    #[test]
    fn a_faster_scroll_moves_in_the_frame_it_arrives() {
        let ctx = egui::Context::default();
        for speed in [TouchpadSpeed::Four, TouchpadSpeed::Twelve] {
            let mut events = vec![wheel(
                egui::MouseWheelUnit::Point,
                -5.0,
                egui::Modifiers::NONE,
            )];
            speed.apply(&mut events);
            let mut moved = 0.0;
            let _ = ctx.run(
                egui::RawInput {
                    events,
                    ..Default::default()
                },
                |ctx| moved = ctx.input(|input| input.smooth_scroll_delta.y),
            );
            let wanted = -5.0 * speed.factor();
            assert!(
                (moved - wanted).abs() < 1e-3,
                "{speed:?}: moved {moved} of {wanted}"
            );
        }
    }

    /// eframe asks for this before the first frame, where the rest of the saved state returns.
    #[test]
    fn the_saved_speed_applies_from_the_first_scroll() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ui-state.json");
        std::fs::write(&path, r#"{"touchpad_speed":"Twelve"}"#).unwrap();
        let mut app = crate::Splat {
            ui_store: crate::ui_state::Store::at(path),
            ..Default::default()
        };
        let mut input = egui::RawInput {
            events: vec![wheel(
                egui::MouseWheelUnit::Point,
                -1.0,
                egui::Modifiers::NONE,
            )],
            ..Default::default()
        };
        eframe::App::raw_input_hook(&mut app, &egui::Context::default(), &mut input);
        assert!((total(&input.events, egui::MouseWheelUnit::Point, false) + 12.0).abs() < 1e-3);
    }

    #[test]
    fn the_default_is_four_and_an_unknown_speed_reads_as_it() {
        assert_eq!(TouchpadSpeed::default(), TouchpadSpeed::Four);
        assert_eq!(TouchpadSpeed::Four.factor(), 4.0);
        for unknown in [r#""Twenty""#, "4", "null", "{}", "[1]"] {
            let read: TouchpadSpeed = serde_json::from_str(unknown).unwrap();
            assert_eq!(read, TouchpadSpeed::Four, "{unknown}");
        }
        let saved = serde_json::to_string(&TouchpadSpeed::Twelve).unwrap();
        assert_eq!(
            serde_json::from_str::<TouchpadSpeed>(&saved).unwrap(),
            TouchpadSpeed::Twelve
        );
    }
}
