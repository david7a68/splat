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

/// A speed this version does not know reads as the default, so the rest of the file still opens.
impl<'de> Deserialize<'de> for TouchpadSpeed {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let name = String::deserialize(deserializer)?;
        Ok(match name.as_str() {
            "One" => Self::One,
            "Two" => Self::Two,
            "Eight" => Self::Eight,
            "Twelve" => Self::Twelve,
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

    #[test]
    fn the_default_is_four_and_an_unknown_speed_reads_as_it() {
        assert_eq!(TouchpadSpeed::default(), TouchpadSpeed::Four);
        assert_eq!(TouchpadSpeed::Four.factor(), 4.0);
        let read: TouchpadSpeed = serde_json::from_str(r#""Twenty""#).unwrap();
        assert_eq!(read, TouchpadSpeed::Four);
        let saved = serde_json::to_string(&TouchpadSpeed::Twelve).unwrap();
        assert_eq!(
            serde_json::from_str::<TouchpadSpeed>(&saved).unwrap(),
            TouchpadSpeed::Twelve
        );
    }
}
