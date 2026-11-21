use crate::{
    content_workspace::CampaignDocument, lookup::ContentIndex, persistence::Campaign,
    timeline::Book,
};
use splat_content::content::Entry;
use std::sync::Arc;
use uuid::Uuid;

pub use splat_trackers::trackers::*;

impl RuleSource for ContentIndex {
    fn manifest(&self, campaign: &Campaign) -> Result<String, String> {
        CampaignDocument::load(campaign).map(|document| document.path)
    }

    fn note(&self, id: Uuid) -> Option<(&Entry, bool)> {
        let item = self.items.iter().find(|item| item.entry.id == id)?;
        Some((&item.entry, item.is_campaign_note()))
    }

    fn loot_sources(&self) -> Arc<splat_campaign::loot::Sources> {
        ContentIndex::loot_sources(self)
    }
}

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
