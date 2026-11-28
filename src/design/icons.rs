//! Named icon glyphs: code points of the bundled Phosphor Regular font
//! (`assets/fonts/Phosphor-Regular.ttf`, MIT). Icons support words. A state method names its
//! icon from here; the GPUI UI draws it (`gui::icons`, `kit::icon`).
//!
//! To add a glyph, look up its code point in the font (phosphoricons.com lists each glyph's
//! name and code point) and add a constant here with the glyph's Phosphor name in its doc line.

/// Phosphor `ARROW_CLOCKWISE`.
pub const REFRESH: &str = "\u{E036}";
/// Phosphor `ARROW_COUNTER_CLOCKWISE`.
pub const UNDO: &str = "\u{E038}";
/// Phosphor `ARROW_DOWN`.
pub const MOVE_DOWN: &str = "\u{E03E}";
/// Phosphor `ARROW_SQUARE_OUT`.
pub const OPEN_OUT: &str = "\u{E5DE}";
/// Phosphor `ARROW_UP`.
pub const MOVE_UP: &str = "\u{E08E}";
/// Phosphor `BED`.
pub const LONG_REST: &str = "\u{E0CC}";
/// Phosphor `BELL`.
pub const BELL: &str = "\u{E0CE}";
/// Phosphor `BOOK_OPEN`.
pub const LIBRARY: &str = "\u{E0E6}";
/// Phosphor `CALENDAR_BLANK`.
pub const DAY: &str = "\u{E10A}";
/// Phosphor `CARET_DOWN`.
pub const DOWN: &str = "\u{E136}";
/// Phosphor `CARET_RIGHT`.
pub const NEXT: &str = "\u{E13A}";
/// Phosphor `CARET_UP`.
pub const UP: &str = "\u{E13C}";
/// Phosphor `CHECK`.
pub const OK: &str = "\u{E182}";
/// Phosphor `CHECK_CIRCLE`.
pub const SUCCESS: &str = "\u{E184}";
/// Phosphor `CHECK_SQUARE`.
pub const CHECK_SQUARE: &str = "\u{E186}";
/// Phosphor `CLOUD_CHECK`.
pub const ONLINE: &str = "\u{E1B0}";
/// Phosphor `CLOUD_SLASH`.
pub const OFFLINE: &str = "\u{E1B6}";
/// Phosphor `COFFEE`.
pub const SHORT_REST: &str = "\u{E1C2}";
/// Phosphor `COINS`.
pub const LOOT: &str = "\u{E78E}";
/// Phosphor `COPY`.
pub const COPY: &str = "\u{E1CA}";
/// Phosphor `CROSSHAIR`.
pub const ARMED: &str = "\u{E1D6}";
/// Phosphor `DICE_FIVE`.
pub const DICE: &str = "\u{E1EE}";
/// Phosphor `DOTS_SIX_VERTICAL`.
pub const DRAG: &str = "\u{EAE2}";
/// Phosphor `DOTS_THREE`.
pub const EXPAND: &str = "\u{E1FE}";
/// Phosphor `EYE_SLASH`.
pub const ABSENT: &str = "\u{E224}";
/// Phosphor `FEATHER`.
pub const DRAFT: &str = "\u{E9C0}";
/// Phosphor `FILE_TEXT`.
pub const DOCUMENT: &str = "\u{E23A}";
/// Phosphor `FIRE`.
pub const AREA: &str = "\u{E242}";
/// Phosphor `FLAG_BANNER`.
pub const CAMPAIGN: &str = "\u{E622}";
/// Phosphor `FOOTPRINTS`.
pub const TRAVEL: &str = "\u{EA88}";
/// Phosphor `GEAR`.
pub const SETTINGS: &str = "\u{E270}";
/// Phosphor `GLOBE_HEMISPHERE_WEST`.
pub const WORLD: &str = "\u{E28C}";
/// Phosphor `HEART_BREAK`.
pub const BLOODIED: &str = "\u{EBE8}";
/// Phosphor `INFO`.
pub const INFO: &str = "\u{E2CE}";
/// Phosphor `KEYBOARD`.
pub const KEYS: &str = "\u{E2D8}";
/// Phosphor `LIGHTNING`.
pub const SYNC_ON: &str = "\u{E2DE}";
/// Phosphor `LINK`.
pub const LINK: &str = "\u{E2E2}";
/// Phosphor `LINK_BREAK`.
pub const SYNC_OFF: &str = "\u{E2E4}";
/// Phosphor `LIST_PLUS`.
pub const ADD_TO_LOG: &str = "\u{E2F8}";
/// Phosphor `LOCK_SIMPLE`.
pub const READ_ONLY: &str = "\u{E308}";
/// Phosphor `MAGIC_WAND`.
pub const SPELL: &str = "\u{E6B6}";
/// Phosphor `MAGNIFYING_GLASS`.
pub const SEARCH: &str = "\u{E30C}";
/// Phosphor `MINUS`.
pub const SUBTRACT: &str = "\u{E32A}";
/// Phosphor `MINUS_CIRCLE`.
pub const ZERO_HP: &str = "\u{E32C}";
/// Phosphor `NOTEBOOK`.
pub const SESSIONS: &str = "\u{E34E}";
/// Phosphor `PATH`.
pub const TIMELINE: &str = "\u{E39C}";
/// Phosphor `PENCIL_LINE`.
pub const CHANGED: &str = "\u{E3B2}";
/// Phosphor `PENCIL_SIMPLE`.
pub const EDIT: &str = "\u{E3B4}";
/// Phosphor `PLAY`.
pub const RUN: &str = "\u{E3D0}";
/// Phosphor `PLUS`.
pub const ADD: &str = "\u{E3D4}";
/// Phosphor `PUSH_PIN`.
pub const PIN: &str = "\u{E3E2}";
/// Phosphor `RECORD`.
pub const RECORD: &str = "\u{E3EE}";
/// Phosphor `SHIELD`.
pub const TEMP_HP: &str = "\u{E40A}";
/// Phosphor `SHIELD_CHECK`.
pub const SAVE_DC: &str = "\u{E40C}";
/// Phosphor `SIDEBAR_SIMPLE`.
pub const SESSION_LIST: &str = "\u{EC24}";
/// Phosphor `SPARKLE`.
pub const CONCENTRATING: &str = "\u{E6A2}";
/// Phosphor `SPINNER`.
pub const BUSY: &str = "\u{E66A}";
/// Phosphor `STAR`.
pub const MAJOR_EVENT: &str = "\u{E46A}";
/// Phosphor `STOP`.
pub const STOP_RECORDING: &str = "\u{E46C}";
/// Phosphor `SWORD`.
pub const ENCOUNTERS: &str = "\u{E5BA}";
/// Phosphor `SWORD`.
pub const ATTACK: &str = "\u{E5BA}";
/// Phosphor `TRASH`.
pub const REMOVE: &str = "\u{E4A6}";
/// Phosphor `USERS_FOUR`.
pub const GROUP: &str = "\u{E68C}";
/// Phosphor `USERS_THREE`.
pub const PARTY: &str = "\u{E68E}";
/// Phosphor `WARNING`.
pub const WARN: &str = "\u{E4E0}";
/// Phosphor `WARNING_OCTAGON`.
pub const ERROR: &str = "\u{E4E4}";
/// Phosphor `WAVEFORM`.
pub const AUDIO: &str = "\u{E802}";
/// Phosphor `X`.
pub const CLOSE: &str = "\u{E4F6}";

/// The step back through a reader's history, before its word `Back` (only the kit draws it).
pub const HISTORY_BACK: &str = "\u{E058}";

/// Raw Phosphor glyphs by their Phosphor names: the marks the kit draws (a checkbox's box, a
/// radio's ring) and the arrows the design lint knows (`arrow-label`).
pub mod phosphor {
    pub const ARROW_ARC_LEFT: &str = "\u{E014}";
    pub const ARROW_BEND_UP_LEFT: &str = "\u{E024}";
    pub const ARROW_CIRCLE_LEFT: &str = "\u{E05A}";
    pub const ARROW_DOWN: &str = "\u{E03E}";
    pub const ARROW_ELBOW_UP_LEFT: &str = "\u{E054}";
    pub const ARROW_FAT_LEFT: &str = "\u{E51A}";
    pub const ARROW_LEFT: &str = "\u{E058}";
    pub const ARROW_RIGHT: &str = "\u{E06C}";
    pub const ARROW_UP: &str = "\u{E08E}";
    pub const ARROW_U_UP_LEFT: &str = "\u{E08A}";
    pub const CARET_DOWN: &str = "\u{E136}";
    pub const CARET_LEFT: &str = "\u{E138}";
    pub const CARET_RIGHT: &str = "\u{E13A}";
    pub const CARET_UP: &str = "\u{E13C}";
    pub const CHECK: &str = "\u{E182}";
    pub const CHECK_SQUARE: &str = "\u{E186}";
    pub const CIRCLE: &str = "\u{E18A}";
    pub const DOTS_THREE_VERTICAL: &str = "\u{E208}";
    pub const EYE: &str = "\u{E220}";
    pub const FOLDER: &str = "\u{E24A}";
    pub const MINUS: &str = "\u{E32A}";
    pub const PLUS: &str = "\u{E3D4}";
    pub const RADIO_BUTTON: &str = "\u{EB08}";
    pub const SQUARE: &str = "\u{E45E}";
    pub const TOGGLE_LEFT: &str = "\u{E674}";
    pub const TOGGLE_RIGHT: &str = "\u{E676}";
    pub const X: &str = "\u{E4F6}";
}
