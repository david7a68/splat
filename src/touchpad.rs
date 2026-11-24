//! Touchpad scrolling speed. A toolkit moves content one point for each unit the system sends,
//! as macOS expects, where the system adds speed and momentum itself. On Linux it does not:
//! Firefox moves about 4 pixels for each unit and Chromium-based apps about 12, so Splat
//! scrolled far less than they did for the same swipe. The DM chooses the factor; the GPUI app
//! applies it (`gui::touchpad`).
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

    /// The saved speed returns before the first frame, with the rest of the saved state
    /// (`Splat::restore`, which the GPUI root calls as it is built), so the first scroll moves
    /// at it. `gui::tests` proves that a scroll moves at the app's speed.
    #[test]
    fn the_saved_speed_applies_from_the_first_scroll() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ui-state.json");
        std::fs::write(&path, r#"{"touchpad_speed":"Twelve"}"#).unwrap();
        let mut app = crate::Splat {
            ui_store: crate::ui_state::Store::at(path),
            ..Default::default()
        };
        assert_eq!(app.touchpad_speed, TouchpadSpeed::Four);
        app.restore();
        assert_eq!(app.touchpad_speed, TouchpadSpeed::Twelve);
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
