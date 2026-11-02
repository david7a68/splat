//! Design tokens: colour by role, type roles, spacing, shape and density.
//! Every raw colour and text size in the app lives in this file.
use eframe::egui::{Color32, FontFamily, FontId, RichText, TextStyle, Ui};

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

#[derive(Clone, Copy)]
pub struct Turn {
    pub current: Color32,
    pub next: Color32,
}
#[derive(Clone, Copy)]
pub struct Hp {
    pub healthy: Color32,
    pub damaged: Color32,
    pub bloodied: Color32,
    pub zero: Color32,
    pub temp: Color32,
    pub unknown: Color32,
    pub track: Color32,
}
#[derive(Clone, Copy)]
pub struct Participant {
    pub absent: Color32,
    pub concentrating: Color32,
    pub condition: Color32,
}
#[derive(Clone, Copy)]
pub struct Side {
    pub party: Color32,
    pub enemies: Color32,
    pub other: Color32,
}
#[derive(Clone, Copy)]
pub struct Sync {
    pub off: Color32,
    pub first_read: Color32,
    pub matching: Color32,
    pub pending: Color32,
    pub conflict: Color32,
    pub read_only: Color32,
    pub error: Color32,
}
#[derive(Clone, Copy)]
pub struct Connection {
    pub connected: Color32,
    pub offline: Color32,
    pub busy: Color32,
}
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Document {
    pub clean: Color32,
    pub unsaved: Color32,
    pub saved: Color32,
    pub invalid: Color32,
    pub external: Color32,
    pub read_only: Color32,
}
#[derive(Clone, Copy)]
pub struct ResultTones {
    pub error: Color32,
    pub warning: Color32,
    pub success: Color32,
    pub info: Color32,
}

/// One theme. A later theme is another value of this struct. Every family is
/// complete, even where no screen uses one state yet (see the brief, section 6).
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Colors {
    pub page: Color32,
    pub rail: Color32,
    pub raised: Color32,
    pub well: Color32,
    pub hover: Color32,
    pub selected: Color32,
    pub current_row: Color32,
    pub hairline: Color32,
    pub rule: Color32,
    pub field_border: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub faint: Color32,
    pub disabled: Color32,
    pub accent: Color32,
    pub accent_fill: Color32,
    pub accent_hover: Color32,
    pub on_accent: Color32,
    pub focus: Color32,
    pub destructive: Color32,
    pub turn: Turn,
    pub hp: Hp,
    pub participant: Participant,
    pub side: Side,
    pub sync: Sync,
    pub connection: Connection,
    pub document: Document,
    pub result: ResultTones,
}

const GREEN: Color32 = rgb(0x93BF97);
const SAND: Color32 = rgb(0xD9BC74);
const ROSE: Color32 = rgb(0xF29BC4);
const RED: Color32 = rgb(0xF4898A);
const MUTED: Color32 = rgb(0xBDB3A3);
const FAINT: Color32 = rgb(0x9D9384);

pub const DARK: Colors = Colors {
    page: rgb(0x1B1815),
    rail: rgb(0x151210),
    raised: rgb(0x25211C),
    well: rgb(0x120F0D),
    hover: rgb(0x29241F),
    selected: rgb(0x2F2A24),
    current_row: rgb(0x343D55),
    hairline: rgb(0x342E28),
    rule: rgb(0x4A4238),
    field_border: rgb(0x7A6E5F),
    text: rgb(0xE9E1D3),
    muted: MUTED,
    faint: FAINT,
    disabled: rgb(0x8A7F70),
    accent: rgb(0xA9BDE6),
    accent_fill: rgb(0x8FA6D4),
    accent_hover: rgb(0xA9BDE6),
    on_accent: rgb(0x14110E),
    focus: rgb(0xC9D6F2),
    destructive: RED,
    turn: Turn {
        current: rgb(0xA9BDE6),
        next: MUTED,
    },
    hp: Hp {
        healthy: rgb(0x8FB08A),
        damaged: rgb(0xC9B27C),
        bloodied: rgb(0xEBA176),
        zero: RED,
        temp: rgb(0x86C5C0),
        unknown: FAINT,
        track: rgb(0x3D362E),
    },
    participant: Participant {
        absent: FAINT,
        concentrating: rgb(0x7CCBEA),
        condition: rgb(0xDCCBA9),
    },
    side: Side {
        party: rgb(0xCFC6B4),
        enemies: rgb(0xFA8F55),
        other: rgb(0xB4C08F),
    },
    sync: Sync {
        off: FAINT,
        first_read: MUTED,
        matching: GREEN,
        pending: SAND,
        conflict: ROSE,
        read_only: MUTED,
        error: RED,
    },
    connection: Connection {
        connected: GREEN,
        offline: FAINT,
        busy: MUTED,
    },
    document: Document {
        clean: FAINT,
        unsaved: SAND,
        saved: GREEN,
        invalid: RED,
        external: SAND,
        read_only: MUTED,
    },
    result: ResultTones {
        error: RED,
        warning: SAND,
        success: GREEN,
        info: MUTED,
    },
};

/// The active theme. Dark is the only one for now.
pub fn colors() -> &'static Colors {
    &DARK
}

// The 4 px grid.
pub const XS: f32 = 4.0;
pub const S: f32 = 8.0;
pub const M: f32 = 12.0;
pub const L: f32 = 16.0;
pub const XL: f32 = 24.0;
pub const XXL: f32 = 32.0;

pub const RADIUS_CONTROL: f32 = 3.0;
pub const RADIUS_LAYER: f32 = 6.0;
pub const HAIRLINE_WIDTH: f32 = 1.0;
pub const FOCUS_WIDTH: f32 = 2.0;

pub const RAIL_WIDTH: f32 = 136.0;
/// Past this window width the rail can afford a campaign's whole name.
pub const RAIL_WIDE_WIDTH: f32 = 188.0;
pub const RAIL_WIDE_FROM: f32 = 1400.0;
pub const PANEL_MIN_WIDTH: f32 = 340.0;
pub const PANEL_MAX_WIDTH: f32 = 560.0;
pub const READER_MEASURE: f32 = 488.0;
pub const FORM_WIDTH: f32 = 560.0;

/// Control and row sizes. A level belongs to a screen, not to the app.
#[derive(Clone, Copy, PartialEq)]
pub enum Density {
    Glance,
    Scan,
    Read,
}
impl Density {
    pub fn control(self) -> f32 {
        match self {
            Self::Glance => 36.0,
            Self::Scan => 30.0,
            Self::Read => 28.0,
        }
    }
    pub fn row(self) -> f32 {
        match self {
            Self::Glance => 36.0,
            Self::Scan => 32.0,
            Self::Read => 28.0,
        }
    }
}

/// The live run's own type sizes. They grow with the room each row can have, because
/// the DM reads this screen from a seated distance (brief, principle 2).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RunScale {
    pub row: f32,
    pub name: f32,
    pub hp: f32,
    pub hp_max: f32,
    pub stat: f32,
    pub condition: f32,
    pub mark: f32,
    /// The widest a roster row should be at this size, so a name stays near its HP.
    pub max_width: f32,
}

pub const RUN_COMPACT: RunScale = RunScale {
    row: 38.0,
    name: 16.0,
    hp: 20.0,
    hp_max: 13.0,
    stat: 15.0,
    condition: 14.0,
    mark: 12.0,
    max_width: 560.0,
};
pub const RUN_REGULAR: RunScale = RunScale {
    row: 52.0,
    name: 18.0,
    hp: 24.0,
    hp_max: 15.0,
    stat: 16.0,
    condition: 15.0,
    mark: 13.0,
    max_width: 640.0,
};
pub const RUN_LARGE: RunScale = RunScale {
    row: 68.0,
    name: 22.0,
    hp: 30.0,
    hp_max: 18.0,
    stat: 19.0,
    condition: 16.0,
    mark: 14.0,
    max_width: 760.0,
};

impl RunScale {
    /// The largest size at which every row still fits in this height and width.
    pub fn fit(height: f32, width: f32, rows: usize) -> Self {
        let each = height / rows.max(1) as f32;
        [RUN_LARGE, RUN_REGULAR]
            .into_iter()
            .find(|scale| each >= scale.row && width >= scale.max_width - 120.0)
            .unwrap_or(RUN_COMPACT)
    }
}

/// The named type scale. See `docs/design-system.md`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Display,
    Title,
    Section,
    Heading,
    Body,
    Strong,
    Name,
    Small,
    Label,
    Numeral,
    NumeralLarge,
    Round,
    Prose,
    ProseCompact,
    ProseStrong,
    ProseItalic,
    StatLabel,
    Stat,
    Mono,
}

pub const FAMILY_UI: &str = "ui";
pub const FAMILY_UI_MEDIUM: &str = "ui-medium";
pub const FAMILY_UI_SEMIBOLD: &str = "ui-semibold";
pub const FAMILY_UI_BOLD: &str = "ui-bold";
pub const FAMILY_SERIF: &str = "serif";
pub const FAMILY_SERIF_ITALIC: &str = "serif-italic";
pub const FAMILY_SERIF_SEMIBOLD: &str = "serif-semibold";
pub const FAMILY_SERIF_TITLE: &str = "serif-title";
pub const FAMILY_SERIF_SMALL: &str = "serif-small";
pub const FAMILY_SERIF_SMALL_STRONG: &str = "serif-small-strong";

impl Type {
    pub const ALL: [Self; 19] = [
        Self::Display,
        Self::Title,
        Self::Section,
        Self::Heading,
        Self::Body,
        Self::Strong,
        Self::Name,
        Self::Small,
        Self::Label,
        Self::Numeral,
        Self::NumeralLarge,
        Self::Round,
        Self::Prose,
        Self::ProseCompact,
        Self::ProseStrong,
        Self::ProseItalic,
        Self::StatLabel,
        Self::Stat,
        Self::Mono,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Display => "display",
            Self::Title => "title",
            Self::Section => "section",
            Self::Heading => "heading",
            Self::Body => "body",
            Self::Strong => "strong",
            Self::Name => "name",
            Self::Small => "small",
            Self::Label => "label",
            Self::Numeral => "numeral",
            Self::NumeralLarge => "numeral-large",
            Self::Round => "round",
            Self::Prose => "prose",
            Self::ProseCompact => "prose-compact",
            Self::ProseStrong => "prose-strong",
            Self::ProseItalic => "prose-italic",
            Self::StatLabel => "stat-label",
            Self::Stat => "stat",
            Self::Mono => "mono",
        }
    }

    pub fn size(self) -> f32 {
        match self {
            Self::Display => 26.0,
            Self::Title => 20.0,
            Self::Section => 17.0,
            Self::Heading | Self::Body | Self::Strong | Self::Numeral => 14.0,
            Self::Name => 15.0,
            Self::Small => 12.0,
            Self::Label => 11.0,
            Self::NumeralLarge | Self::Round => 18.0,
            Self::Prose | Self::ProseStrong | Self::ProseItalic => 16.0,
            Self::ProseCompact => 15.0,
            Self::StatLabel | Self::Stat | Self::Mono => 13.0,
        }
    }

    /// The line height, for prose and for fixed rows.
    pub fn line(self) -> f32 {
        match self {
            Self::Display => 32.0,
            Self::Title => 26.0,
            Self::Section => 23.0,
            Self::Heading | Self::Body | Self::Strong | Self::Name | Self::Numeral => 20.0,
            Self::Small => 16.0,
            Self::Label => 14.0,
            Self::NumeralLarge | Self::Round => 22.0,
            Self::Prose | Self::ProseStrong | Self::ProseItalic => 24.0,
            Self::ProseCompact => 22.0,
            Self::StatLabel | Self::Stat | Self::Mono => 18.0,
        }
    }

    pub fn family(self) -> &'static str {
        match self {
            Self::Display | Self::Title | Self::Section => FAMILY_SERIF_TITLE,
            Self::Heading | Self::Name | Self::Label | Self::Numeral => FAMILY_UI_SEMIBOLD,
            Self::Body | Self::Small | Self::Mono => FAMILY_UI,
            Self::Strong => FAMILY_UI_MEDIUM,
            Self::NumeralLarge | Self::Round => FAMILY_UI_BOLD,
            Self::Prose | Self::ProseCompact => FAMILY_SERIF,
            Self::ProseStrong => FAMILY_SERIF_SEMIBOLD,
            Self::ProseItalic => FAMILY_SERIF_ITALIC,
            Self::StatLabel => FAMILY_SERIF_SMALL_STRONG,
            Self::Stat => FAMILY_SERIF_SMALL,
        }
    }

    pub fn tracking(self) -> f32 {
        if self == Self::Label { 0.6 } else { 0.0 }
    }

    pub fn style(self) -> TextStyle {
        TextStyle::Name(self.name().into())
    }

    /// The font for this role. A context with no installed theme, such as a unit
    /// test's, has only egui's families, so it falls back to the same size there.
    pub fn font(self, ui: &Ui) -> FontId {
        if self == Self::Mono {
            return FontId::new(self.size(), FontFamily::Monospace);
        }
        ui.style()
            .text_styles
            .get(&self.style())
            .cloned()
            .unwrap_or_else(|| FontId::proportional(self.size()))
    }

    /// This role's face at another size, for the run's scale.
    pub fn sized(self, ui: &Ui, size: f32) -> FontId {
        let mut font = self.font(ui);
        font.size = size;
        font
    }

    pub fn rich(self, ui: &Ui, text: impl Into<String>) -> RichText {
        RichText::new(text)
            .font(self.font(ui))
            .extra_letter_spacing(self.tracking())
    }
}

#[cfg(test)]
pub(crate) fn contrast(a: Color32, b: Color32) -> f32 {
    fn channel(value: u8) -> f32 {
        let value = f32::from(value) / 255.0;
        if value <= 0.03928 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    }
    fn luminance(color: Color32) -> f32 {
        0.2126 * channel(color.r()) + 0.7152 * channel(color.g()) + 0.0722 * channel(color.b())
    }
    let (a, b) = (luminance(a), luminance(b));
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_text(c: &Colors) -> Vec<(&'static str, Color32)> {
        vec![
            ("text", c.text),
            ("muted", c.muted),
            ("faint", c.faint),
            ("accent", c.accent),
            ("turn.current", c.turn.current),
            ("turn.next", c.turn.next),
            ("hp.bloodied", c.hp.bloodied),
            ("hp.zero", c.hp.zero),
            ("hp.temp", c.hp.temp),
            ("hp.unknown", c.hp.unknown),
            ("participant.absent", c.participant.absent),
            ("participant.concentrating", c.participant.concentrating),
            ("participant.condition", c.participant.condition),
            ("side.party", c.side.party),
            ("side.enemies", c.side.enemies),
            ("side.other", c.side.other),
            ("sync.off", c.sync.off),
            ("sync.first_read", c.sync.first_read),
            ("sync.matching", c.sync.matching),
            ("sync.pending", c.sync.pending),
            ("sync.conflict", c.sync.conflict),
            ("sync.read_only", c.sync.read_only),
            ("sync.error", c.sync.error),
            ("connection.connected", c.connection.connected),
            ("connection.offline", c.connection.offline),
            ("connection.busy", c.connection.busy),
            ("document.clean", c.document.clean),
            ("document.unsaved", c.document.unsaved),
            ("document.saved", c.document.saved),
            ("document.invalid", c.document.invalid),
            ("document.external", c.document.external),
            ("document.read_only", c.document.read_only),
            ("result.error", c.result.error),
            ("result.warning", c.result.warning),
            ("result.success", c.result.success),
            ("result.info", c.result.info),
            ("destructive", c.destructive),
        ]
    }

    /// WCAG 1.4.3: text on each surface where it appears.
    #[test]
    fn text_tokens_meet_contrast_on_their_surfaces() {
        let c = colors();
        let surfaces = [
            ("page", c.page),
            ("rail", c.rail),
            ("raised", c.raised),
            ("well", c.well),
            ("hover", c.hover),
            ("selected", c.selected),
        ];
        let mut failures = Vec::new();
        for (surface, background) in surfaces {
            for (name, foreground) in all_text(c) {
                let ratio = contrast(foreground, background);
                if ratio < 4.5 {
                    failures.push(format!("{name} on {surface}: {ratio:.2}"));
                }
            }
        }
        assert!(failures.is_empty(), "Below 4.5:1:\n{}", failures.join("\n"));
    }

    /// The current row is a stronger fill, so faint text does not go on it. The row
    /// uses muted text in place of the faint family.
    #[test]
    fn the_current_row_stands_out_and_its_text_stays_readable() {
        let c = colors();
        assert!(contrast(c.current_row, c.page) >= 1.6);
        let faint_family = [
            c.faint,
            c.hp.unknown,
            c.participant.absent,
            c.sync.off,
            c.connection.offline,
            c.document.clean,
        ];
        for (name, foreground) in all_text(c) {
            if !faint_family.contains(&foreground) {
                let ratio = contrast(foreground, c.current_row);
                assert!(ratio >= 4.5, "{name} on current_row: {ratio:.2}");
            }
        }
    }

    /// What the installed style paints, not only what a token says.
    #[test]
    fn a_resting_field_edge_is_three_to_one_on_every_surface() {
        let ctx = eframe::egui::Context::default();
        super::super::theme::install(&ctx);
        let edge = ctx.style().visuals.widgets.inactive.bg_stroke.color;
        let c = colors();
        for (name, background) in [("page", c.page), ("rail", c.rail), ("raised", c.raised)] {
            let ratio = contrast(edge, background);
            assert!(ratio >= 3.0, "on {name}: {ratio:.2}");
        }
    }

    /// A meaning must not look like another that appears in the same slot.
    #[test]
    fn neighbouring_meanings_have_distinct_colours() {
        fn distance(a: Color32, b: Color32) -> i32 {
            (i32::from(a.r()) - i32::from(b.r())).abs()
                + (i32::from(a.g()) - i32::from(b.g())).abs()
                + (i32::from(a.b()) - i32::from(b.b())).abs()
        }
        let c = colors();
        for (name, a, b) in [
            (
                "party, concentrating",
                c.side.party,
                c.participant.concentrating,
            ),
            ("enemies, damaged", c.side.enemies, c.hp.damaged),
            ("bloodied, conflict", c.hp.bloodied, c.sync.conflict),
            ("enemies, conflict", c.side.enemies, c.sync.conflict),
            ("enemies, bloodied", c.side.enemies, c.hp.bloodied),
            ("enemies, error", c.side.enemies, c.result.error),
            ("side, condition", c.side.enemies, c.participant.condition),
            ("temp, healthy", c.hp.temp, c.hp.healthy),
        ] {
            assert!(distance(a, b) >= 60, "{name}: {}", distance(a, b));
        }
    }

    #[test]
    fn the_run_scale_grows_only_when_rows_fit() {
        assert_eq!(RunScale::fit(432.0, 450.0, 12), RUN_COMPACT);
        assert_eq!(RunScale::fit(432.0, 600.0, 8), RUN_REGULAR);
        assert_eq!(RunScale::fit(800.0, 760.0, 8), RUN_LARGE);
        assert_eq!(RunScale::fit(800.0, 760.0, 20), RUN_COMPACT);
    }

    /// WCAG 1.4.3 for large text and 1.4.11 for controls and state graphics.
    #[test]
    fn large_text_and_ui_tokens_meet_three_to_one() {
        let c = colors();
        for (name, foreground, background) in [
            ("primary label", c.on_accent, c.accent_fill),
            ("primary hover label", c.on_accent, c.accent_hover),
            ("field edge on page", c.field_border, c.page),
            ("field edge on rail", c.field_border, c.rail),
            ("field edge on raised", c.field_border, c.raised),
            ("focus on page", c.focus, c.page),
            ("focus on raised", c.focus, c.raised),
            ("focus on current row", c.focus, c.current_row),
            ("bar on current row", c.turn.current, c.current_row),
            ("primary fill on page", c.accent_fill, c.page),
            ("disabled on page", c.disabled, c.page),
            ("bar healthy on track", c.hp.healthy, c.page),
            ("bar damaged on page", c.hp.damaged, c.page),
            ("bar bloodied on page", c.hp.bloodied, c.page),
            ("bar zero on page", c.hp.zero, c.page),
            ("bar temp on page", c.hp.temp, c.page),
        ] {
            let ratio = contrast(foreground, background);
            assert!(ratio >= 3.0, "{name}: {ratio:.2}");
        }
    }

    #[test]
    fn no_pure_black_or_white_in_the_neutrals() {
        let c = colors();
        for color in [c.page, c.rail, c.well, c.text, c.focus, c.on_accent] {
            assert_ne!(color, Color32::BLACK);
            assert_ne!(color, Color32::WHITE);
        }
    }

    #[test]
    fn every_control_height_meets_the_minimum_target() {
        for density in [Density::Glance, Density::Scan, Density::Read] {
            assert!(density.control() >= 24.0 && density.row() >= 24.0);
        }
    }
}
