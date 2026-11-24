//! Read-only Markdown presentation. Content cannot execute UI actions, but it can say which note
//! a link names, and a roll table can ask for a roll. Whoever drew it decides what happens next.
use crate::state_context::{Slot, StateContext};

/// A link to a note, as `world::with_links` writes one.
pub use splat_campaign::note::NOTE_LINK;
/// An embedded image, as `world::with_links` writes one.
pub const FILE_LINK: &str = "splat-file:";

/// The note link the DM pressed.
const CLICKED_NOTE: Slot<String> = Slot::new("markdown_clicked_note");
/// The heading the next draw scrolls to.
const WANTED_HEADING: Slot<String> = Slot::new("markdown_wanted_heading");
/// A pressed bold reference, for Lookup.
const CLICKED_LOOKUP: Slot<String> = Slot::new("markdown_clicked_lookup");

/// The note link the DM pressed, for whoever just drew this Markdown.
pub fn take_note_link(ctx: &StateContext) -> Option<String> {
    ctx.take(CLICKED_NOTE)
}

/// A resolved bold reference pressed in the Reader, for Lookup to open.
pub fn take_lookup_link(ctx: &StateContext) -> Option<String> {
    ctx.take(CLICKED_LOOKUP)
}

/// Give a pressed bold reference to Lookup, as a press in the Reader does ([`take_lookup_link`]).
pub fn send_lookup_link(ctx: &StateContext, words: String) {
    ctx.set(CLICKED_LOOKUP, words);
}

/// Ask the next draw to scroll to this heading. `None` ends the request.
pub fn want_heading(ctx: &StateContext, heading: Option<String>) {
    ctx.put(WANTED_HEADING, heading);
}

/// True once, for the heading a draw was asked to show.
pub fn wanted_heading(ctx: &StateContext, heading: &str) -> bool {
    let wanted = ctx.get(WANTED_HEADING);
    if wanted.is_some_and(|wanted| wanted.eq_ignore_ascii_case(heading.trim())) {
        want_heading(ctx, None);
        return true;
    }
    false
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkTarget {
    Missing,
    Note(String),
    Lookup(String),
    Url(String),
    /// A dice link in a stat block or a sheet. The run or the preview stream rolls it.
    Roll(splat::content::dice_text::RollLink),
}

impl LinkTarget {
    /// Where a Markdown link's destination goes, if it can be followed.
    pub fn from_destination(destination: &str) -> Option<Self> {
        if let Some(note) = destination.strip_prefix(NOTE_LINK) {
            Some(Self::Note(note.to_owned()))
        } else if let Some(link) =
            splat::content::dice_text::RollLink::from_destination(destination)
        {
            Some(Self::Roll(link))
        } else if destination.starts_with("https://") || destination.starts_with("http://") {
            Some(Self::Url(destination.to_owned()))
        } else {
            None
        }
    }
}

/// Give a pressed note link to whichever page takes it next. `from` is the note it is in.
pub fn send_note_link(ctx: &StateContext, note: String, from: Option<uuid::Uuid>) {
    ctx.set(crate::requests::CLICKED_READER_DOCUMENT, from);
    ctx.set(CLICKED_NOTE, note);
}
