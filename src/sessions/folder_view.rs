//! The Session folder's block on Campaign → Link and folder, free of any toolkit, for the GPUI
//! tab (`src/gui/pages/campaigns/`).
use super::{FolderAct, FolderFault, FolderWait, Sessions};
use crate::design::{self, outcome::plain::PlainLine};
use crate::timeline::Book;

impl Sessions {
    /// The folder field's words.
    pub fn folder_text(&self) -> &str {
        &self.folder_text
    }

    /// An edit of the folder field: it ends a refusal of Save without moving, and the folder
    /// move's result.
    pub fn set_folder_text(&mut self, words: &str) {
        if self.folder_text == words {
            return;
        }
        self.folder_text = words.to_owned();
        if matches!(
            self.folder_error,
            Some(FolderFault::FilesChanged | FolderFault::FilesGone)
        ) {
            self.folder_error = None;
        }
        if let Some(link) = self.folder_link() {
            self.folder_moves.owner_changed(&link);
        }
    }

    /// The folder in use now.
    pub fn folder_now(&self) -> Option<&std::path::Path> {
        self.folder.as_deref()
    }

    /// The save row's state (its word, why Save waits, Save's weight) and whether the draft
    /// changed. `locked`: the page's lock (a run).
    pub fn folder_row<'a>(
        &mut self,
        book: &Book,
        locked: Option<&'a str>,
    ) -> Option<(design::DraftState<'a>, bool)> {
        let campaign = self.campaign.clone()?;
        let changed =
            super::super::resolve(&campaign, &self.folder_text).ok() != self.folder.clone();
        let files = self.folder_files();
        let has_files = self.folder_move_counts(book, &files).is_some();
        let mut state = design::DraftState::new(
            design::save_row::DraftOrigin::Saved,
            if changed {
                design::save_row::Changed::Yes
            } else {
                design::save_row::Changed::No
            },
            design::save_row::Primary::Draft,
        );
        let wait = self
            .folder_wait(book, changed, &files)
            .filter(|wait| *wait != FolderWait::NoChanges)
            .map(FolderWait::words);
        if let Some(why) = locked.or(wait) {
            state = state.waits(why);
        } else if has_files {
            state = state.commits_elsewhere("Choose below to move the files or not.");
        }
        Some((state, changed))
    }

    /// `Discard`: the saved folder comes back.
    pub fn discard_folder(&mut self) {
        let Some(campaign) = self.campaign.clone() else {
            return;
        };
        if let Ok(saved) = super::super::folder_setting(&campaign) {
            self.folder_text = saved;
        }
        self.folder_error = None;
    }

    /// `Save folder`, or one of the warning's two choices.
    pub fn folder_act(&mut self, book: &Book, act: FolderAct) {
        if let Some(campaign) = self.campaign.clone() {
            self.save_folder_act(&campaign, book, act);
        }
    }

    /// A refusal of Save without moving ends when its cause goes (`save_folder_act`).
    pub(super) fn settle_folder_error(&mut self, book: &Book) {
        if let Some(FolderFault::FilesChanged | FolderFault::FilesGone) = &self.folder_error {
            let files = self.folder_files();
            let offered = self.folder_move_counts(book, &files).is_some();
            let gone = matches!(self.folder_error, Some(FolderFault::FilesGone));
            if offered == gone {
                self.folder_error = None;
            }
        }
    }

    /// The block's own messages, under its save row (`folder_messages`), as plain lines. A
    /// refusal of Save without moving ends first when its cause went.
    pub fn folder_lines(&mut self, book: &Book, locked: bool) -> Vec<PlainLine<FolderAct>> {
        self.settle_folder_error(book);
        let frame = design::outcome::PaneFrame::new();
        let mut messages = design::outcome::Messages::new();
        self.folder_messages(&frame, &mut messages, book, locked);
        messages.into_plain()
    }
}
