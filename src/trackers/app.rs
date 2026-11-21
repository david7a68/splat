use crate::{lookup::ContentIndex, persistence::Campaign, timeline::Book};
use std::sync::Arc;
use uuid::Uuid;

pub use splat_trackers::trackers::*;

#[path = "reader.rs"]
mod reader;
pub(super) use reader::Reader;

pub fn clock(book: &Book) -> Clock<'_> {
    Clock {
        changes: book.changes(),
        conflict: book.conflict,
        moved_under_edit: book.moved_under_edit.as_deref(),
        unreadable: book.unreadable.as_deref(),
        events: &book.events,
    }
}

pub(super) fn poll(
    trackers: &mut Trackers,
    index: Option<Arc<ContentIndex>>,
    campaign: Option<&Campaign>,
    played: Option<Uuid>,
    timeline: &Book,
) {
    trackers.poll(
        index.map(|index| index as Arc<dyn RuleSource>),
        campaign,
        played,
        clock(timeline),
    );
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
