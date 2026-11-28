//! The campaign list's and form's acts and reads, free of any toolkit, for the GPUI pages
//! (`src/gui/pages/settings/`, `src/gui/pages/campaigns/`).
use super::{Campaigns, FormKept, FormReason, folder_preview};
use crate::design::CreateReasons as _;
use crate::persistence::Campaign;
use uuid::Uuid;

impl Campaigns {
    /// The settings could be read and may be saved.
    pub fn writable(&self) -> bool {
        self.writable
    }

    /// Why the list's controls wait: the page's lock, else unreadable settings.
    pub fn list_lock<'a>(&self, blocked: Option<&'a str>) -> Option<&'a str> {
        blocked.or((!self.writable).then_some("Reload valid campaign settings first."))
    }

    /// The manifest error of one campaign, said on its row.
    pub fn manifest_error(&self, id: Uuid) -> Option<&str> {
        self.manifest_errors.get(&id).map(String::as_str)
    }

    /// The campaign whose details form is open.
    pub fn editing(&self) -> Option<Uuid> {
        self.editing
    }

    /// The import form is open (for any remote campaign).
    pub fn import_open(&self) -> bool {
        self.remote.is_some()
    }

    /// `Edit details` of `campaign`: its details form opens with its saved name and folder; the
    /// add form's words wait for it. `focus`: the name field takes the keyboard (Link and folder).
    pub fn start_edit(&mut self, campaign: &Campaign, focus: bool) {
        if self.editing.is_none() {
            self.add_name = self.name.clone();
            self.add_path = self.path.clone();
        }
        self.remote = None;
        self.editing = Some(campaign.id);
        self.end_discard();
        if focus {
            self.focus_name = true;
        }
        self.name = campaign.name.clone();
        self.path = campaign.path.to_string_lossy().into_owned();
    }

    /// `Remove…` on a row: the confirm dialog opens.
    pub fn ask_remove(&mut self, id: Uuid) {
        self.confirm_remove = Some(id);
    }

    /// The campaign the remove dialog asks about, while it is open.
    pub fn removing(&self) -> Option<Campaign> {
        let id = self.confirm_remove?;
        self.data.campaigns.iter().find(|c| c.id == id).cloned()
    }

    /// `Keep the campaign` (or Escape): the dialog closes.
    pub fn keep_campaign(&mut self) {
        self.confirm_remove = None;
    }

    /// `Remove from Splat`: Splat forgets the campaign; its folder stays.
    pub fn remove_confirmed(&mut self) {
        let Some(id) = self.confirm_remove.take() else {
            return;
        };
        let mut data = self.data.clone();
        data.campaigns.retain(|c| c.id != id);
        if data.active_campaign_id == Some(id) {
            data.active_campaign_id = None;
        }
        self.save(data);
        // The result of registering a removed campaign ends with it.
        if !self.data.campaigns.iter().any(|c| c.id == id) {
            self.campaign_copy = None;
            self.results.owner_changed(&super::FormResult::Added);
        }
    }

    /// `Reload from disk`.
    pub fn reload_from_disk(&mut self) {
        self.reload();
    }

    /// `Open` on a row: the campaign becomes the active one.
    pub fn open_campaign(&mut self, id: Uuid) {
        self.select(Some(id));
    }

    /// The form's name and folder as typed.
    pub fn form_words(&self) -> (&str, &str) {
        (&self.name, &self.path)
    }

    pub fn set_form_name(&mut self, words: &str) {
        self.name = words.to_owned();
    }

    pub fn set_form_path(&mut self, words: &str) {
        self.path = words.to_owned();
    }

    /// The name field asked for the keyboard (once).
    pub fn take_focus_name(&mut self) -> bool {
        std::mem::take(&mut self.focus_name)
    }

    /// The details form holds the saved name and folder.
    pub fn form_unchanged(&self) -> bool {
        self.details_unchanged()
    }

    /// What the form's Add or Save waits for, in words.
    pub fn form_reason_words(&self) -> Option<&'static str> {
        self.form_reason(self.details_unchanged())
            .map(FormReason::words)
    }

    /// `Will create …/Salt Road`: the folder the add form makes, by its last name.
    pub fn form_preview(&self) -> Option<String> {
        folder_preview(&self.path)
    }

    /// `Browse…`: the folder the DM picks goes in the folder field.
    pub fn browse_folder(&mut self) {
        if let Some(path) = crate::hermetic::pick_folder() {
            self.path = path.to_string_lossy().into_owned();
        }
    }

    /// `Cancel`: the form closes; the add form's words come back after an edit.
    pub fn cancel_form(&mut self) {
        self.clear_form();
    }

    /// `Discard` in the details form: the saved name and folder come back, and the dropped draft
    /// is a kept result with its Undo.
    pub fn discard_details(&mut self) {
        if let Some(saved) = self
            .data
            .campaigns
            .iter()
            .find(|c| Some(c.id) == self.editing)
        {
            self.results.record(FormKept::Discarded {
                campaign: saved.id,
                name: self.name.clone(),
                path: self.path.clone(),
            });
            self.name = saved.name.clone();
            self.path = saved.path.to_string_lossy().into_owned();
        }
    }

    /// An edit of the details form ends the discard's result.
    pub fn details_edited(&mut self) {
        if !self.details_unchanged() {
            self.end_discard();
        }
    }

    /// `Add campaign` or `Save details`: true when a campaign was added. A failure is the list's
    /// error.
    pub fn commit_form(&mut self) -> bool {
        let editing = self.editing.is_some();
        match self.save_form() {
            Ok(()) => !editing,
            Err(error) => {
                self.error = Some(error);
                false
            }
        }
    }

    /// Whether the add form is open, as the page that draws it says (`form_open`).
    pub fn set_form_open(&mut self, open: bool) {
        self.form_open = open;
    }

    /// The list's or the form's error.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// `Register as a new campaign`, from the first campaign's form error.
    pub fn register_copy_now(&mut self) {
        if let Err(error) = self.register_campaign_copy() {
            self.error = Some(error);
        }
    }
}
