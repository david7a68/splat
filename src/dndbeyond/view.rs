//! D&D Beyond's remote list and the active campaign's link, free of any toolkit, so the egui
//! import list and link block (`DndBeyond::show_import`, `show_link`) and the GPUI pages
//! (`src/gui/pages/settings/`, `src/gui/pages/campaigns/`) share them.
#![warn(deprecated)]
use super::{DndBeyond, LinkSaved};
use crate::state_context::StateContext;
use crate::{auth::Session, campaigns::Campaigns};

/// One remote campaign as a page shows it.
pub struct Remote<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub dm: Option<&'a str>,
    pub players: Option<u64>,
}

impl DndBeyond {
    /// The remote campaigns of the last list.
    pub fn remotes(&self) -> impl Iterator<Item = Remote<'_>> {
        self.campaigns.iter().map(|remote| Remote {
            id: &remote.id,
            name: &remote.name,
            dm: remote.dm_username.as_deref(),
            players: remote.player_count,
        })
    }

    /// A list is on its way.
    pub fn refreshing(&self) -> bool {
        self.pending.is_some()
    }

    /// A list came, possibly empty.
    pub fn loaded(&self) -> bool {
        self.loaded
    }

    /// The import list asks for its first list by itself when it opens.
    pub fn wants_first_list(&self) -> bool {
        !self.loaded && self.error.is_none() && self.pending.is_none()
    }

    /// `Refresh from D&D Beyond`.
    pub fn refresh_list(&mut self, session: Session, ctx: &StateContext) {
        self.refresh(session, ctx);
    }

    /// Why the link's `Refresh list` waits.
    pub fn link_refresh_lock<'a>(
        &self,
        blocked: Option<&'a str>,
        connected: bool,
    ) -> Option<&'a str> {
        if let Some(lock) = blocked {
            Some(lock)
        } else if !connected {
            Some("Connect to D&D Beyond in Settings first.")
        } else if self.pending.is_some() {
            Some("Already refreshing.")
        } else {
            None
        }
    }

    /// `Refresh list` on Link and folder: the list comes again, and with it the linked
    /// campaign's name.
    pub fn refresh_link(&mut self, session: Session, local: &Campaigns, ctx: &StateContext) {
        let Some(campaign) = local.active_campaign() else {
            return;
        };
        self.pending_link = campaign
            .dndbeyond_campaign_id
            .clone()
            .map(|id| (campaign.id, id));
        self.requested_visible_link = self.pending_link.clone();
        self.refresh(session, ctx);
    }

    /// The link's picker is in view this frame: a link the list has not named yet asks for the
    /// list once.
    pub fn link_in_view(
        &mut self,
        session: Option<Session>,
        local: &Campaigns,
        ctx: &StateContext,
    ) {
        self.visible_last_frame = true;
        let Some(campaign) = local.active_campaign() else {
            return;
        };
        let key = campaign
            .dndbeyond_campaign_id
            .clone()
            .map(|id| (campaign.id, id));
        if key.is_some()
            && key != self.requested_visible_link
            && self.pending.is_none()
            && let Some(session) = session
        {
            self.requested_visible_link = key.clone();
            self.pending_link = key;
            self.refresh(session, ctx);
        }
    }

    /// The link's choices: `Not linked`, a saved link the list does not hold, then each remote
    /// campaign, with the reason one already linked elsewhere waits.
    pub fn link_choices(
        &self,
        local: &Campaigns,
    ) -> Vec<(Option<String>, String, Option<&'static str>)> {
        let Some(campaign) = local.active_campaign() else {
            return Vec::new();
        };
        let mut choices = vec![(None, "Not linked".to_owned(), None)];
        if let Some(id) = campaign
            .dndbeyond_campaign_id
            .as_ref()
            .filter(|id| !self.campaigns.iter().any(|c| &c.id == *id))
        {
            let name = campaign
                .dndbeyond_campaign_name
                .clone()
                .unwrap_or_else(|| format!("Campaign {id}"));
            choices.push((Some(id.clone()), name, None));
        }
        for remote in &self.campaigns {
            let lock = local
                .linked_elsewhere(&remote.id)
                .then_some("Already linked to another campaign.");
            choices.push((Some(remote.id.clone()), remote.name.clone(), lock));
        }
        choices
    }

    /// The DM chose a link (or `Not linked`).
    pub fn choose_link(&mut self, local: &mut Campaigns, selected: Option<String>) {
        let Some(campaign) = local.active_campaign() else {
            return;
        };
        if selected == campaign.dndbeyond_campaign_id {
            return;
        }
        match local.link(selected.clone()) {
            Ok(()) => {
                self.requested_visible_link = None;
                self.message.record(LinkSaved(campaign.id));
                self.error = None;
                if let Some(remote) = self
                    .campaigns
                    .iter()
                    .find(|c| Some(&c.id) == selected.as_ref())
                    && let Err(error) =
                        local.cache_remote_name(campaign.id, &remote.id, &remote.name)
                {
                    self.error = Some(error);
                }
            }
            // A failed attempt is the newest answer: the older success ends.
            Err(error) => {
                self.error = Some(error);
                self.end_link_result();
            }
        }
    }
}
