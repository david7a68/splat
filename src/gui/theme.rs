//! Splat's tokens as GPUI values: colours, the type ramp, spacing, radii, widths and fonts.
//!
//! The colours come from `src/design/tokens.rs` (`design::Color`, turned into GPUI colours by
//! [`color`]); the sizes are their GPUI copy. `docs/design-system.md` explains every value. A test
//! (`the_gpui_theme_is_the_design_tokens`) keeps the two equal.
//!
//! Pages and kit parts take every colour, size and font from here, never a literal. Text styles
//! come from [`Type`] through [`TypeExt::ty`]; spaces from [`XS`]..[`XXL`]; control heights from
//! [`Density`].
use gpui::{App, FontStyle, FontWeight, Hsla, Pixels, Rgba, SharedString, Styled, TextAlign, px};
use std::sync::LazyLock;

/// Whose turn it is and whose is next.
#[derive(Clone, Copy, Debug)]
pub struct Turn {
    pub current: Hsla,
    pub next: Hsla,
}

/// Hit point states.
#[derive(Clone, Copy, Debug)]
pub struct Hp {
    pub healthy: Hsla,
    pub damaged: Hsla,
    pub bloodied: Hsla,
    pub zero: Hsla,
    pub temp: Hsla,
    pub unknown: Hsla,
    pub track: Hsla,
}

/// A run participant's marks.
#[derive(Clone, Copy, Debug)]
pub struct Participant {
    pub absent: Hsla,
    pub concentrating: Hsla,
    pub condition: Hsla,
    pub changed: Hsla,
}

/// The sides of a fight.
#[derive(Clone, Copy, Debug)]
pub struct Side {
    pub party: Hsla,
    pub enemies: Hsla,
    pub other: Hsla,
}

/// D&D Beyond sync states.
#[derive(Clone, Copy, Debug)]
pub struct Sync {
    pub off: Hsla,
    pub first_read: Hsla,
    pub matching: Hsla,
    pub pending: Hsla,
    pub conflict: Hsla,
    pub read_only: Hsla,
    pub error: Hsla,
}

/// The account's connection.
#[derive(Clone, Copy, Debug)]
pub struct Connection {
    pub connected: Hsla,
    pub offline: Hsla,
    pub busy: Hsla,
}

/// A document's save state.
#[derive(Clone, Copy, Debug)]
pub struct Document {
    pub clean: Hsla,
    pub unsaved: Hsla,
    pub saved: Hsla,
    pub invalid: Hsla,
    pub external: Hsla,
    pub read_only: Hsla,
}

/// A result's tone (`Tone`).
#[derive(Clone, Copy, Debug)]
pub struct ResultTones {
    pub error: Hsla,
    pub warning: Hsla,
    pub success: Hsla,
    pub info: Hsla,
}

/// Every colour token. Names say what a colour means (`docs/design-system.md`, Colour tokens).
#[derive(Clone, Copy, Debug)]
pub struct Colors {
    pub page: Hsla,
    pub rail: Hsla,
    pub raised: Hsla,
    pub well: Hsla,
    pub hover: Hsla,
    pub selected: Hsla,
    pub current_row: Hsla,
    pub hairline: Hsla,
    pub rule: Hsla,
    pub field_border: Hsla,
    pub text: Hsla,
    pub muted: Hsla,
    pub faint: Hsla,
    pub disabled: Hsla,
    pub accent: Hsla,
    pub accent_fill: Hsla,
    pub accent_hover: Hsla,
    pub on_accent: Hsla,
    pub focus: Hsla,
    pub destructive: Hsla,
    pub recording: Hsla,
    /// The dim layer behind a dialog.
    pub backdrop: Hsla,
    /// A raised layer's shadow.
    pub shadow: Hsla,
    /// Selected text behind the caret.
    pub selection: Hsla,
    pub turn: Turn,
    pub hp: Hp,
    pub participant: Participant,
    pub side: Side,
    pub sync: Sync,
    pub connection: Connection,
    pub document: Document,
    pub result: ResultTones,
}

/// A design token's colour as a GPUI colour.
pub fn color(token: crate::design::Color) -> Hsla {
    Rgba {
        r: f32::from(token.r()) / 255.0,
        g: f32::from(token.g()) / 255.0,
        b: f32::from(token.b()) / 255.0,
        a: f32::from(token.a()) / 255.0,
    }
    .into()
}

/// Black at `alpha` (a backdrop, a shadow).
fn black(alpha: f32) -> Hsla {
    Hsla {
        a: alpha,
        ..gpui::black()
    }
}

/// The one dark theme: the design tokens' colours, and the layers' own.
static DARK: LazyLock<Colors> = LazyLock::new(|| {
    let t = crate::design::colors();
    Colors {
        page: color(t.page),
        rail: color(t.rail),
        raised: color(t.raised),
        well: color(t.well),
        hover: color(t.hover),
        selected: color(t.selected),
        current_row: color(t.current_row),
        hairline: color(t.hairline),
        rule: color(t.rule),
        field_border: color(t.field_border),
        text: color(t.text),
        muted: color(t.muted),
        faint: color(t.faint),
        disabled: color(t.disabled),
        accent: color(t.accent),
        accent_fill: color(t.accent_fill),
        accent_hover: color(t.accent_hover),
        on_accent: color(t.on_accent),
        focus: color(t.focus),
        destructive: color(t.destructive),
        recording: color(t.recording),
        // Behind a dialog, and under a layer.
        backdrop: black(120.0 / 255.0),
        shadow: black(110.0 / 255.0),
        selection: Hsla {
            a: 0.28,
            ..color(t.accent)
        },
        turn: Turn {
            current: color(t.turn.current),
            next: color(t.turn.next),
        },
        hp: Hp {
            healthy: color(t.hp.healthy),
            damaged: color(t.hp.damaged),
            bloodied: color(t.hp.bloodied),
            zero: color(t.hp.zero),
            temp: color(t.hp.temp),
            unknown: color(t.hp.unknown),
            track: color(t.hp.track),
        },
        participant: Participant {
            absent: color(t.participant.absent),
            concentrating: color(t.participant.concentrating),
            condition: color(t.participant.condition),
            changed: color(t.participant.changed),
        },
        side: Side {
            party: color(t.side.party),
            enemies: color(t.side.enemies),
            other: color(t.side.other),
        },
        sync: Sync {
            off: color(t.sync.off),
            first_read: color(t.sync.first_read),
            matching: color(t.sync.matching),
            pending: color(t.sync.pending),
            conflict: color(t.sync.conflict),
            read_only: color(t.sync.read_only),
            error: color(t.sync.error),
        },
        connection: Connection {
            connected: color(t.connection.connected),
            offline: color(t.connection.offline),
            busy: color(t.connection.busy),
        },
        document: Document {
            clean: color(t.document.clean),
            unsaved: color(t.document.unsaved),
            saved: color(t.document.saved),
            invalid: color(t.document.invalid),
            external: color(t.document.external),
            read_only: color(t.document.read_only),
        },
        result: ResultTones {
            error: color(t.result.error),
            warning: color(t.result.warning),
            success: color(t.result.success),
            info: color(t.result.info),
        },
    }
});

/// The active colours. Splat has one warm dark theme.
pub fn colors() -> &'static Colors {
    &DARK
}

// Spacing: a 4 px grid.
pub const XS: Pixels = px(4.0);
pub const S: Pixels = px(8.0);
pub const M: Pixels = px(12.0);
pub const L: Pixels = px(16.0);
pub const XL: Pixels = px(24.0);
pub const XXL: Pixels = px(32.0);

/// An icon and its gap before a word: the body glyph, `XS`, and 2 px.
pub const ICON_SLOT: Pixels = px(20.0);

pub const RADIUS_CONTROL: Pixels = px(3.0);
pub const RADIUS_LAYER: Pixels = px(6.0);
pub const HAIRLINE_WIDTH: Pixels = px(1.0);
pub const FOCUS_WIDTH: Pixels = px(2.0);
/// The accent bar of a selected navigation item and of a message's tone.
pub const MARK_WIDTH: Pixels = px(3.0);
/// The underline of a chosen segment.
pub const UNDERLINE_WIDTH: Pixels = px(2.0);
/// A scroll bar's width (egui's `bar_width`).
pub const SCROLL_BAR_WIDTH: Pixels = px(8.0);
/// The gap egui leaves between items in a column (`item_spacing.y`).
pub const LINE_GAP: Pixels = px(6.0);
/// The rail's own small gap between its items (`item_spacing.y = 2`).
pub const RAIL_ITEM_GAP: Pixels = px(2.0);
/// The width of a column's drag handle.
pub const SPLITTER_WIDTH: Pixels = px(10.0);
/// The minimum size of every target (WCAG 2.5.8).
pub const MIN_TARGET: Pixels = px(24.0);

pub const RAIL_WIDTH: Pixels = px(136.0);
pub const RAIL_WIDE_WIDTH: Pixels = px(188.0);
pub const RAIL_WIDE_FROM: Pixels = px(1400.0);
pub const RAIL_DRAG_MAX_WIDTH: Pixels = px(320.0);
pub const PANEL_MIN_WIDTH: Pixels = px(340.0);
pub const PANEL_MAX_WIDTH: Pixels = px(560.0);
pub const PANEL_DRAG_MAX_WIDTH: Pixels = px(720.0);
pub const RUN_LIST_KEEPS: Pixels = px(454.0);
/// What a side panel leaves for the page beside it: the run's list and its margins.
pub const PANEL_LEAVES_PAGE: Pixels = px(454.0 + 2.0 * 12.0);
/// What the rail leaves for the page and a side panel.
pub const RAIL_LEAVES_WINDOW: Pixels = px(454.0 + 2.0 * 12.0 + 340.0);
pub const READER_MEASURE: Pixels = px(488.0);
pub const FORM_WIDTH: Pixels = px(560.0);
pub const LIST_MEASURE: Pixels = px(880.0);
/// A tooltip's widest line.
pub const TOOLTIP_WIDTH: Pixels = px(320.0);
/// A dialog's distance from the window's top and sides.
pub const DIALOG_INSET: Pixels = XXL;

/// How dense a control is, by what it is for (`docs/design-system.md`, Space, shape and density).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Density {
    /// The run's `Next turn` and damage entry.
    Glance,
    /// Run setup, Encounters, Party, Lookup results, the rail.
    Scan,
    /// Library, Settings, Campaign, the run's footer.
    Read,
}

impl Density {
    /// A control's height.
    pub const fn control(self) -> Pixels {
        match self {
            Self::Glance => px(36.0),
            Self::Scan => px(30.0),
            Self::Read => px(28.0),
        }
    }

    /// A list row's height.
    pub const fn row(self) -> Pixels {
        match self {
            Self::Glance => px(36.0),
            Self::Scan => px(32.0),
            Self::Read => px(28.0),
        }
    }
}

/// The run's own type and row scale (`RunScale::fit`).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct RunScale {
    pub row: f32,
    pub name: f32,
    pub hp: f32,
    pub hp_max: f32,
    pub stat: f32,
    pub condition: f32,
    pub mark: f32,
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
    /// The largest scale whose rows all fit `height` with no scrolling.
    pub fn fit(height: f32, width: f32, rows: usize) -> Self {
        let each = height / rows.max(1) as f32;
        [RUN_LARGE, RUN_REGULAR]
            .into_iter()
            .find(|scale| each >= scale.row && width >= scale.max_width - 120.0)
            .unwrap_or(RUN_COMPACT)
    }
}

/// The font families Splat ships (`assets/fonts/`).
pub mod family {
    /// IBM Plex Sans: the working UI.
    pub const UI: &str = "IBM Plex Sans";
    /// Source Serif 4: what the books say.
    pub const SERIF: &str = "Source Serif 4";
    /// Source Serif 4 Subhead: titles, 18 to 28 px.
    pub const SERIF_TITLE: &str = "Source Serif 4 Subhead";
    /// Source Serif 4 SmText: stat blocks, 13 to 14 px.
    pub const SERIF_SMALL: &str = "Source Serif 4 SmText";
    /// Hack: source text and code, egui's bundled monospace (`splat_assets::MONO`).
    pub const MONO: &str = "Hack";
    /// Phosphor icons.
    pub const ICONS: &str = crate::gui::icons::FAMILY;
}

/// Each role in the type ramp (`docs/design-system.md`, Type). Do not add sizes outside it.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

    /// The size in pixels.
    pub const fn size(self) -> Pixels {
        px(match self {
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
        })
    }

    /// The line height in pixels.
    pub const fn line(self) -> Pixels {
        px(match self {
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
        })
    }

    /// The family.
    pub const fn family(self) -> &'static str {
        match self {
            Self::Display | Self::Title | Self::Section => family::SERIF_TITLE,
            Self::Heading
            | Self::Name
            | Self::Label
            | Self::Numeral
            | Self::Body
            | Self::Small
            | Self::Strong
            | Self::NumeralLarge
            | Self::Round => family::UI,
            Self::Prose | Self::ProseCompact | Self::ProseStrong | Self::ProseItalic => {
                family::SERIF
            }
            Self::StatLabel | Self::Stat => family::SERIF_SMALL,
            Self::Mono => family::MONO,
        }
    }

    /// The weight within the family.
    pub const fn weight(self) -> FontWeight {
        match self {
            Self::Display | Self::Title | Self::Section => FontWeight::SEMIBOLD,
            Self::Heading | Self::Name | Self::Label | Self::Numeral => FontWeight::SEMIBOLD,
            Self::Strong => FontWeight::MEDIUM,
            Self::NumeralLarge | Self::Round => FontWeight::BOLD,
            Self::ProseStrong | Self::StatLabel => FontWeight::SEMIBOLD,
            _ => FontWeight::NORMAL,
        }
    }

    /// Italic or upright.
    pub const fn style(self) -> FontStyle {
        match self {
            Self::ProseItalic => FontStyle::Italic,
            _ => FontStyle::Normal,
        }
    }

    /// `Label` is set in capitals, with egui's +0.6 tracking (`kit::text` draws both: GPUI has no
    /// letter spacing, so it puts a hair space between the letters of each word, 0.65 px in IBM
    /// Plex at 11 px).
    pub const fn capitals(self) -> bool {
        matches!(self, Self::Label)
    }
}

/// Text styles on any GPUI element.
pub trait TypeExt: Styled + Sized {
    /// Set this element's text in a role of the type ramp.
    fn ty(self, ty: Type) -> Self {
        let mut this = self
            .font_family(SharedString::new_static(ty.family()))
            .font_weight(ty.weight())
            .text_size(ty.size())
            .line_height(ty.line());
        if ty.style() == FontStyle::Italic {
            this = this.italic();
        }
        this
    }

    /// Icon glyphs (`gui::icons`) at a role's size and line.
    fn icon_font(self, ty: Type) -> Self {
        self.font_family(SharedString::new_static(family::ICONS))
            .font_weight(FontWeight::NORMAL)
            .text_size(ty.size())
            .line_height(ty.line())
    }

    /// Right-aligned text, for numbers.
    fn number_align(mut self) -> Self {
        self.text_style().text_align = Some(TextAlign::Right);
        self
    }
}

impl<T: Styled> TypeExt for T {}

/// The fonts Splat ships, plus the icon font.
pub fn font_bytes() -> Vec<std::borrow::Cow<'static, [u8]>> {
    splat_assets::FONTS
        .iter()
        .map(|(_, bytes)| std::borrow::Cow::Borrowed(*bytes))
        .chain([
            std::borrow::Cow::Borrowed(splat_assets::PHOSPHOR),
            std::borrow::Cow::Borrowed(splat_assets::MONO),
        ])
        .collect()
}

/// Load the fonts and set gpui-component's theme from Splat's tokens, so its widgets (inputs,
/// scroll bars, menus, popovers, dialogs) match the kit. Call once, after
/// `gpui_component::init`.
pub fn install(cx: &mut App) {
    if let Err(error) = cx.text_system().add_fonts(font_bytes()) {
        eprintln!("Splat could not load its fonts: {error}");
    }
    use gpui_component::{Theme, ThemeMode};
    Theme::change(ThemeMode::Dark, None, cx);
    Theme::update(cx, |theme| {
        let k = colors();
        let t = &mut theme.colors;
        t.background = k.page;
        t.foreground = k.text;
        t.border = k.field_border;
        t.input = k.field_border;
        t.ring = k.focus;
        t.caret = k.focus;
        t.selection = k.selection;
        t.muted = k.raised;
        t.muted_foreground = k.muted;
        t.accent = k.hover;
        t.accent_foreground = k.text;
        t.primary = k.accent_fill;
        t.primary_hover = k.accent_hover;
        t.primary_active = k.accent_hover;
        t.primary_foreground = k.on_accent;
        t.secondary = k.raised;
        t.secondary_hover = k.hover;
        t.secondary_active = k.selected;
        t.secondary_foreground = k.text;
        t.danger = k.destructive;
        t.danger_hover = k.destructive;
        t.danger_active = k.destructive;
        t.danger_foreground = k.on_accent;
        t.warning = k.result.warning;
        t.success = k.result.success;
        t.info = k.result.info;
        t.link = k.accent;
        t.link_hover = k.accent_hover;
        t.link_active = k.accent;
        t.popover = k.raised;
        t.popover_foreground = k.text;
        t.list = k.page;
        t.list_hover = k.hover;
        t.list_active = k.selected;
        t.list_active_border = k.selected;
        t.list_even = k.page;
        t.list_head = k.page;
        t.scrollbar = k.page;
        t.scrollbar_thumb = k.rule;
        t.scrollbar_thumb_hover = k.field_border;
        t.sidebar = k.rail;
        t.sidebar_foreground = k.muted;
        t.sidebar_border = k.hairline;
        t.sidebar_accent = k.selected;
        t.sidebar_accent_foreground = k.text;
        t.tab = k.page;
        t.tab_bar = k.page;
        t.tab_active = k.page;
        t.tab_active_foreground = k.text;
        t.tab_foreground = k.muted;
        t.switch = k.field_border;
        t.switch_thumb = k.text;
        t.title_bar = k.rail;
        t.title_bar_border = k.hairline;
        t.drag_border = k.focus;
        t.drop_target = k.selection;
        t.table = k.page;
        t.table_head = k.page;
        t.table_hover = k.hover;
        t.table_active = k.selected;
        t.table_row_border = k.hairline;
        theme.font_family = SharedString::new_static(family::UI);
        theme.font_size = Type::Body.size();
        theme.mono_font_family = SharedString::new_static(family::MONO);
        theme.mono_font_size = Type::Mono.size();
        theme.radius = RADIUS_CONTROL;
        theme.radius_lg = RADIUS_LAYER;
        theme.shadow = true;
        // R43: no motion that draws the eye.
        theme.focus_ring = true;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::design::tokens;

    fn same(name: &str, gpui: Hsla, token: crate::design::Color) {
        let rgba = gpui.to_rgb();
        let channels = [rgba.r, rgba.g, rgba.b].map(|v| (v * 255.0).round() as i32);
        let expected = [token.r(), token.g(), token.b()].map(i32::from);
        let close = channels
            .iter()
            .zip(expected)
            .all(|(a, b)| (a - b).abs() <= 1);
        assert!(close, "{name}: {channels:?} != {expected:?}");
    }

    /// The GPUI theme and the design tokens are one design: the colours arrive whole, and the
    /// sizes are the same.
    #[test]
    fn the_gpui_theme_is_the_design_tokens() {
        let (g, e) = (colors(), tokens::colors());
        for (name, a, b) in [
            ("page", g.page, e.page),
            ("rail", g.rail, e.rail),
            ("raised", g.raised, e.raised),
            ("well", g.well, e.well),
            ("hover", g.hover, e.hover),
            ("selected", g.selected, e.selected),
            ("current_row", g.current_row, e.current_row),
            ("hairline", g.hairline, e.hairline),
            ("rule", g.rule, e.rule),
            ("field_border", g.field_border, e.field_border),
            ("text", g.text, e.text),
            ("muted", g.muted, e.muted),
            ("faint", g.faint, e.faint),
            ("disabled", g.disabled, e.disabled),
            ("accent", g.accent, e.accent),
            ("accent_fill", g.accent_fill, e.accent_fill),
            ("on_accent", g.on_accent, e.on_accent),
            ("focus", g.focus, e.focus),
            ("destructive", g.destructive, e.destructive),
            ("hp.bloodied", g.hp.bloodied, e.hp.bloodied),
            ("hp.temp", g.hp.temp, e.hp.temp),
            ("side.enemies", g.side.enemies, e.side.enemies),
            ("sync.conflict", g.sync.conflict, e.sync.conflict),
            ("document.unsaved", g.document.unsaved, e.document.unsaved),
            ("result.error", g.result.error, e.result.error),
            ("result.warning", g.result.warning, e.result.warning),
            ("result.success", g.result.success, e.result.success),
            (
                "participant.concentrating",
                g.participant.concentrating,
                e.participant.concentrating,
            ),
        ] {
            same(name, a, b);
        }
        for ty in Type::ALL {
            let token = match ty {
                Type::Display => tokens::Type::Display,
                Type::Title => tokens::Type::Title,
                Type::Section => tokens::Type::Section,
                Type::Heading => tokens::Type::Heading,
                Type::Body => tokens::Type::Body,
                Type::Strong => tokens::Type::Strong,
                Type::Name => tokens::Type::Name,
                Type::Small => tokens::Type::Small,
                Type::Label => tokens::Type::Label,
                Type::Numeral => tokens::Type::Numeral,
                Type::NumeralLarge => tokens::Type::NumeralLarge,
                Type::Round => tokens::Type::Round,
                Type::Prose => tokens::Type::Prose,
                Type::ProseCompact => tokens::Type::ProseCompact,
                Type::ProseStrong => tokens::Type::ProseStrong,
                Type::ProseItalic => tokens::Type::ProseItalic,
                Type::StatLabel => tokens::Type::StatLabel,
                Type::Stat => tokens::Type::Stat,
                Type::Mono => tokens::Type::Mono,
            };
            assert_eq!(f32::from(ty.size()), token.size(), "{ty:?} size");
            assert_eq!(f32::from(ty.line()), token.line(), "{ty:?} line");
        }
        for (density, token) in [
            (Density::Glance, tokens::Density::Glance),
            (Density::Scan, tokens::Density::Scan),
            (Density::Read, tokens::Density::Read),
        ] {
            assert_eq!(f32::from(density.control()), token.control());
            assert_eq!(f32::from(density.row()), token.row());
        }
        assert_eq!(RUN_LARGE.row, tokens::RUN_LARGE.row);
        assert_eq!(f32::from(L), tokens::L);
        assert_eq!(f32::from(RAIL_WIDTH), tokens::RAIL_WIDTH);
        assert_eq!(f32::from(PANEL_LEAVES_PAGE), tokens::PANEL_LEAVES_PAGE);
        assert_eq!(f32::from(RAIL_LEAVES_WINDOW), tokens::RAIL_LEAVES_WINDOW);
        assert_eq!(f32::from(READER_MEASURE), tokens::READER_MEASURE);
    }
}
