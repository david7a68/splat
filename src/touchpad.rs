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
    pub fn apply(self, events: &mut [egui::Event]) {
        let factor = self.factor();
        for event in events {
            if let egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Point,
                delta,
                modifiers,
                ..
            } = event
                && !modifiers.command
            {
                *delta *= factor;
            }
        }
    }
}

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

    fn delta(event: &egui::Event) -> f32 {
        match event {
            egui::Event::MouseWheel { delta, .. } => delta.y,
            _ => unreachable!(),
        }
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
        assert_eq!(delta(&events[0]), -80.0, "a touchpad scroll");
        assert_eq!(delta(&events[1]), -1.0, "a mouse wheel");
        assert_eq!(delta(&events[2]), 1.0, "a page step");
        assert_eq!(delta(&events[3]), 5.0, "a zoom");
        assert_eq!(events[4], egui::Event::Zoom(1.5));
        TouchpadSpeed::One.apply(&mut events);
        assert_eq!(delta(&events[0]), -80.0);
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
        assert_eq!(delta(&input.events[0]), -12.0);
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
