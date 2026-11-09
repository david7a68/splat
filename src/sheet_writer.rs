//! One computer at a time saves a campaign's D&D Beyond sheets.
//!
//! A DM can run Splat on two computers that share the vault through Obsidian Sync. If both
//! save the same sheet at about the same time, Obsidian Sync merges the two saves, and every
//! line that both added appears twice. So the computer that holds the turn saves the sheets
//! from its reads, and the others only read. A note in the vault holds the turn, because the
//! vault is all the computers share. Obsidian Sync skips files whose names start with a dot,
//! so the note has a plain name. Its front matter has no `splat_` key, so the content reader
//! and World leave it alone.
//!
//! A reading that the DM asks for, such as a force read, still saves on any computer.
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

/// The note that names the computer that saves the sheets, from the campaign folder.
pub const FILE: &str = "Characters/D&D Beyond/Sheet writer.md";
/// The front matter key that marks the note. World skips notes that have it.
pub const KEY: &str = "sheet_writer:";
/// How long a turn lasts without a renewal, in seconds. When its computer stops, another
/// computer takes the turn after this time.
const TURN: u64 = 10 * 60;
/// A computer renews its turn when less than this time remains. Fewer renewals mean fewer
/// writes for Obsidian Sync to carry.
const RENEW_BELOW: u64 = 5 * 60;

/// The computer that saves the sheets, and until when.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Holder {
    pub device: Uuid,
    pub name: String,
    pub until: u64,
}

/// Who saves the sheets from automatic reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    /// This computer saves them.
    Mine,
    /// Another computer saves them. The name is that computer's.
    Elsewhere(String),
}

fn path(campaign: &Path) -> PathBuf {
    campaign.join(FILE)
}

/// The holder in the note. A merge by Obsidian Sync can repeat lines, so this reads lines and
/// not YAML: the first device and name, and the latest time. Each computer reads the same
/// bytes the same way, so they agree on the holder.
fn parse(text: &str) -> Option<Holder> {
    let value = |name: &str| {
        text.lines()
            .filter_map(|line| line.trim().strip_prefix(name))
            .map(|rest| {
                serde_yaml_ng::from_str::<String>(rest.trim())
                    .unwrap_or_else(|_| rest.trim().to_owned())
            })
            .collect::<Vec<_>>()
    };
    let device = value("device:").first()?.parse().ok()?;
    let name = value("name:").into_iter().next().unwrap_or_default();
    let until = value("until:")
        .iter()
        .filter_map(|v| v.parse::<u64>().ok())
        .max()?;
    Some(Holder {
        device,
        name,
        until,
    })
}

/// The holder in the note, if the note exists and names one.
pub fn read(campaign: &Path) -> Option<Holder> {
    parse(&fs::read_to_string(path(campaign)).ok()?)
}

fn text(holder: &Holder) -> String {
    let name = serde_yaml_ng::to_string(&holder.name).unwrap_or_default();
    format!(
        "---\n{KEY}\n  device: {}\n  name: {}\n  until: {}\n---\n\n\
         Splat on {} saves the D&D Beyond sheets in this folder. Splat on other computers only \
         reads them, so that Obsidian Sync does not merge two saves of one sheet. Splat renews \
         this note while it runs. When it stops, another computer takes over after {} minutes. \
         Do not edit this note.\n",
        holder.device,
        name.trim(),
        holder.until,
        holder.name,
        TURN / 60,
    )
}

fn write(campaign: &Path, holder: &Holder) -> std::io::Result<()> {
    let target = path(campaign);
    let parent = target.parent().unwrap_or(campaign);
    fs::create_dir_all(parent)?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    temp.write_all(text(holder).as_bytes())?;
    temp.as_file().sync_all()?;
    temp.persist(&target).map_err(|e| e.error)?;
    Ok(())
}

/// Takes or renews the turn when it is free, run out or this computer's. Otherwise names the
/// computer that has it. A note that cannot be written leaves the turn with this computer, as
/// before there was a turn: the sheet saves report their own failures.
pub fn take(campaign: &Path, device: Uuid, name: &str, now: u64) -> Turn {
    match read(campaign) {
        Some(holder) if holder.device != device && holder.until > now => {
            Turn::Elsewhere(holder.name)
        }
        Some(holder) if holder.device == device && holder.until >= now + RENEW_BELOW => Turn::Mine,
        _ => {
            let _ = write(
                campaign,
                &Holder {
                    device,
                    name: name.to_owned(),
                    until: now + TURN,
                },
            );
            Turn::Mine
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Uuid = Uuid::from_u128(1);
    const B: Uuid = Uuid::from_u128(2);

    #[test]
    fn the_first_computer_takes_the_turn_and_another_waits_until_it_runs_out() {
        let root = tempfile::tempdir().unwrap();
        assert_eq!(take(root.path(), A, "Desk", 1_000), Turn::Mine);
        assert_eq!(
            take(root.path(), B, "Laptop", 1_000 + TURN - 1),
            Turn::Elsewhere("Desk".into())
        );
        assert_eq!(take(root.path(), B, "Laptop", 1_000 + TURN), Turn::Mine);
        assert_eq!(
            take(root.path(), A, "Desk", 1_000 + TURN),
            Turn::Elsewhere("Laptop".into())
        );
    }

    #[test]
    fn the_holder_renews_only_when_half_its_turn_has_gone() {
        let root = tempfile::tempdir().unwrap();
        take(root.path(), A, "Desk", 1_000);
        let first = fs::read_to_string(path(root.path())).unwrap();
        assert_eq!(take(root.path(), A, "Desk", 1_000 + 60), Turn::Mine);
        assert_eq!(fs::read_to_string(path(root.path())).unwrap(), first);
        assert_eq!(
            take(root.path(), A, "Desk", 1_000 + RENEW_BELOW + 1),
            Turn::Mine
        );
        assert_eq!(
            read(root.path()).unwrap().until,
            1_000 + RENEW_BELOW + 1 + TURN
        );
    }

    #[test]
    fn a_note_that_obsidian_sync_merged_still_names_one_holder() {
        let root = tempfile::tempdir().unwrap();
        let merged = format!(
            "---\n{KEY}\n  device: {A}\n  name: Desk\n  device: {B}\n  until: 900\n  name: Laptop\n  until: 2000\n---\n"
        );
        fs::create_dir_all(path(root.path()).parent().unwrap()).unwrap();
        fs::write(path(root.path()), merged).unwrap();
        assert_eq!(
            read(root.path()),
            Some(Holder {
                device: A,
                name: "Desk".into(),
                until: 2000
            })
        );
        assert_eq!(
            take(root.path(), B, "Laptop", 1_500),
            Turn::Elsewhere("Desk".into())
        );
    }

    #[test]
    fn a_name_that_needs_quotes_reads_back() {
        let root = tempfile::tempdir().unwrap();
        take(root.path(), A, "DM's: desk", 1_000);
        assert_eq!(read(root.path()).unwrap().name, "DM's: desk");
    }
}
