//! The one entry reader. The Library, Lookup, the run's side panel, and campaign
//! entries all call `show`, so an entry looks the same everywhere (T6.4).
#![allow(deprecated)] // Old message functions; Wave 2 moves this file to `design::outcome`.
use crate::design::{self};
use splat::content::{Entry, Reference};

/// What the reader's links can open. An index belongs to the implementer.
pub trait Links {
    /// The one entry a reference names, or nothing if it is missing or ambiguous.
    fn find(&self, reference: &Reference) -> Option<(usize, String)>;
    /// Like `find`, for a reference that must name a spell.
    fn find_spell(&self, reference: &Reference) -> Option<(usize, String)>;
    /// Spells named in the words of a stat block paragraph.
    fn spells_named_in(&self, text: &str) -> Vec<(usize, String)>;
    /// The one spell with this name, in any case.
    fn spell_named(&self, name: &str) -> Option<(usize, String)>;
    /// The entry at a place that `find`, `find_spell` or `spell_named` gave.
    fn entry(&self, index: usize) -> Option<&Entry>;
    /// Where an entry's resolved Spellcasting is kept, with the content's other caches; `None`
    /// to resolve it on each call.
    fn spell_cache(&self) -> Option<&super::spells::SpellCache> {
        None
    }
}

/// For an entry shown with nothing to link to.
pub struct NoLinks;
impl Links for NoLinks {
    fn find(&self, _: &Reference) -> Option<(usize, String)> {
        None
    }
    fn find_spell(&self, _: &Reference) -> Option<(usize, String)> {
        None
    }
    fn spells_named_in(&self, _: &str) -> Vec<(usize, String)> {
        Vec::new()
    }
    fn spell_named(&self, _: &str) -> Option<(usize, String)> {
        None
    }
    fn entry(&self, _: usize) -> Option<&Entry> {
        None
    }
}

/// Where an entry comes from.
pub struct Origin<'a> {
    pub path: &'a str,
    pub owner: &'a str,
    pub builtin: bool,
}

/// Values from an open run. The saved entry stays unchanged.
pub struct RunValues {
    pub armor_class: Option<i64>,
    pub hp: Option<i64>,
    pub current_hp: Option<i64>,
    pub initiative: Option<i64>,
    pub speed: Option<String>,
}

const REFERENCE_MAX: f32 = 360.0;
/// The reader's measure, its gap and the margin at their widest.
pub const WIDE_READER: f32 = design::READER_MEASURE + design::XL + REFERENCE_MAX;
