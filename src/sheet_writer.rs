//! One computer at a time saves a campaign's D&D Beyond sheets from automatic reads.
//!
//! A DM can run Splat on two computers that share the vault through Obsidian Sync. If both
//! save the same sheet at about the same time, Obsidian Sync merges the two saves, and every
//! line that both added appears twice. So the computer that holds the turn saves the sheets
//! from its reads, and the others only read. A note in the vault holds the turn, because the
//! vault is all the computers share. Obsidian Sync skips files whose names start with a dot,
//! so the note has a plain name. Its front matter has no `splat_` key, so the content reader
//! and World leave it alone.
//!
//! This is a best-effort election, not a lock: Obsidian Sync carries the note with a delay.
//! A new claim saves nothing for `SETTLE` seconds, so a claim that another computer made at
//! about the same time can arrive first. When two claims merge, both computers read the same
//! bytes the same way and agree on one holder. The times are each computer's clock.
//!
//! A reading that the DM asks for, such as Refresh party or a force read, still saves.
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use uuid::Uuid;

/// The note that names the computer that saves the sheets, from the campaign folder. It is
/// outside `Characters/D&D Beyond`, where a character's sheet could have the same name.
pub const FILE: &str = "Characters/Sheet writer.md";
/// The front matter key that marks the note. World skips notes that have it.
pub const KEY: &str = "sheet_writer:";
/// How long a turn lasts without a renewal, in seconds. When its computer stops, another
/// computer takes the turn after this time.
const TURN: u64 = 10 * 60;
/// A computer renews its turn when less than this time remains. Fewer renewals mean fewer
/// writes for Obsidian Sync to carry.
const RENEW_BELOW: u64 = 5 * 60;
/// A new claim saves nothing until this time has passed, so that a claim another computer
/// made at about the same time can arrive through Obsidian Sync first.
pub const SETTLE: u64 = 60;

/// The computer that saves the sheets, since when it claimed the turn, and until when.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Holder {
    pub device: Uuid,
    pub name: String,
    pub since: u64,
    pub until: u64,
}

/// Who saves the sheets from automatic reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    /// This computer saves them.
    Mine,
    /// This computer has just claimed the turn. It saves after `SETTLE`, if the claim holds.
    Settling,
    /// Another computer saves them. The name is that computer's.
    Elsewhere(String),
    /// The note cannot be read or written, so no computer can be sure of its turn. Nothing
    /// saves automatically here. The message says what to fix.
    Unavailable(String),
}

fn path(campaign: &Path) -> PathBuf {
    campaign.join(FILE)
}

/// The holder in the note. A merge by Obsidian Sync can repeat lines, so this reads lines and
/// not YAML: the first device, name and claim time, and the latest end. Each computer reads
/// the same bytes the same way, so they agree on the holder.
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
    let number = |name: &str| {
        value(name)
            .iter()
            .filter_map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>()
    };
    let device = value("device:").first()?.parse().ok()?;
    let name = value("name:").into_iter().next().unwrap_or_default();
    let until = number("until:").into_iter().max()?;
    let since = number("since:").first().copied().unwrap_or(0);
    Some(Holder {
        device,
        name,
        since,
        until,
    })
}

fn text(holder: &Holder) -> String {
    let name = serde_yaml_ng::to_string(&holder.name).unwrap_or_default();
    format!(
        "---\n{KEY}\n  device: {}\n  name: {}\n  since: {}\n  until: {}\n---\n\n\
         Splat on {} saves the D&D Beyond sheets of this campaign. Splat on other computers \
         only reads them, so that Obsidian Sync does not merge two saves of one sheet. Splat \
         renews this note while it runs. When it stops, another computer takes over after {} \
         minutes. Do not edit this note.\n",
        holder.device,
        name.trim(),
        holder.since,
        holder.until,
        holder.name,
        TURN / 60,
    )
}

fn write(campaign: &Path, holder: &Holder) -> Result<(), String> {
    let target = path(campaign);
    let parent = target.parent().unwrap_or(campaign);
    let written = (|| -> std::io::Result<()> {
        fs::create_dir_all(parent)?;
        let mut temp = tempfile::NamedTempFile::new_in(parent)?;
        temp.write_all(text(holder).as_bytes())?;
        temp.as_file().sync_all()?;
        temp.persist(&target).map_err(|e| e.error)?;
        Ok(())
    })();
    written.map_err(|e| {
        format!("Splat could not write {FILE} ({e}), so it does not save sheets automatically.")
    })
}

/// The note, if it exists. A note that is not Splat's, or a path that cannot be read, is an
/// error: Splat never replaces it.
fn existing(campaign: &Path) -> Result<Option<Holder>, String> {
    match fs::read_to_string(path(campaign)) {
        Ok(text) if text.lines().any(|line| line.trim_end() == KEY) => Ok(parse(&text)),
        Ok(_) => Err(format!(
            "{FILE} is not Splat's sheet writer note, so Splat does not save sheets \
             automatically. Move or rename that note."
        )),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!(
            "Splat could not read {FILE} ({e}), so it does not save sheets automatically."
        )),
    }
}

/// A turn ends at `until`. An end further ahead than one turn comes from a clock that is
/// wrong, so it does not hold the turn either.
fn held(holder: &Holder, now: u64) -> bool {
    holder.until > now && holder.until <= now + TURN
}

/// Takes or renews the turn when it is free, run out or this computer's. Otherwise names the
/// computer that has it.
pub fn take(campaign: &Path, device: Uuid, name: &str, now: u64) -> Turn {
    let holder = match existing(campaign) {
        Ok(holder) => holder,
        Err(message) => return Turn::Unavailable(message),
    };
    match holder {
        Some(holder) if holder.device != device && held(&holder, now) => {
            Turn::Elsewhere(holder.name)
        }
        Some(holder) if holder.device == device && held(&holder, now) => {
            if holder.until < now + RENEW_BELOW {
                let renewed = Holder {
                    name: name.to_owned(),
                    until: now + TURN,
                    ..holder.clone()
                };
                if let Err(message) = write(campaign, &renewed) {
                    return Turn::Unavailable(message);
                }
            }
            if now >= holder.since.saturating_add(SETTLE) {
                Turn::Mine
            } else {
                Turn::Settling
            }
        }
        _ => {
            let claim = Holder {
                device,
                name: name.to_owned(),
                since: now,
                until: now + TURN,
            };
            match write(campaign, &claim) {
                Ok(()) => Turn::Settling,
                Err(message) => Turn::Unavailable(message),
            }
        }
    }
}

/// Ends this computer's turn at once, so that another computer can take it. A computer whose
/// sheet saves fail calls this, so that it does not hold the turn while it saves nothing.
pub fn release(campaign: &Path, device: Uuid, now: u64) {
    if let Ok(Some(holder)) = existing(campaign)
        && holder.device == device
    {
        let _ = write(
            campaign,
            &Holder {
                until: now,
                ..holder
            },
        );
    }
}

/// Writes a note for a test that needs a turn already settled.
#[cfg(test)]
pub fn hold(campaign: &Path, holder: &Holder) {
    write(campaign, holder).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: Uuid = Uuid::from_u128(1);
    const B: Uuid = Uuid::from_u128(2);

    fn read(campaign: &Path) -> Option<Holder> {
        existing(campaign).unwrap()
    }

    #[test]
    fn a_claim_settles_before_it_saves_and_another_computer_waits_until_it_runs_out() {
        let root = tempfile::tempdir().unwrap();
        assert_eq!(take(root.path(), A, "Desk", 1_000), Turn::Settling);
        assert_eq!(
            take(root.path(), A, "Desk", 1_000 + SETTLE - 1),
            Turn::Settling
        );
        assert_eq!(take(root.path(), A, "Desk", 1_000 + SETTLE), Turn::Mine);
        assert_eq!(
            take(root.path(), B, "Laptop", 1_000 + SETTLE),
            Turn::Elsewhere("Desk".into())
        );
        // Desk stops. Its turn runs out, and Laptop claims and settles.
        let end = read(root.path()).unwrap().until;
        assert_eq!(take(root.path(), B, "Laptop", end), Turn::Settling);
        assert_eq!(take(root.path(), B, "Laptop", end + SETTLE), Turn::Mine);
        assert_eq!(
            take(root.path(), A, "Desk", end + SETTLE),
            Turn::Elsewhere("Laptop".into())
        );
    }

    #[test]
    fn the_holder_renews_only_when_half_its_turn_has_gone() {
        let root = tempfile::tempdir().unwrap();
        take(root.path(), A, "Desk", 1_000);
        let first = fs::read_to_string(path(root.path())).unwrap();
        assert_eq!(take(root.path(), A, "Desk", 1_000 + SETTLE), Turn::Mine);
        assert_eq!(fs::read_to_string(path(root.path())).unwrap(), first);
        let later = 1_000 + TURN - RENEW_BELOW + 1;
        assert_eq!(take(root.path(), A, "Desk", later), Turn::Mine);
        let renewed = read(root.path()).unwrap();
        assert_eq!((renewed.since, renewed.until), (1_000, later + TURN));
    }

    #[test]
    fn two_claims_that_obsidian_sync_merged_name_one_holder_for_both() {
        let root = tempfile::tempdir().unwrap();
        let merged = format!(
            "---\n{KEY}\n  device: {A}\n  name: Desk\n  device: {B}\n  since: 900\n  until: 1500\n  name: Laptop\n  since: 950\n  until: 1550\n---\n"
        );
        fs::create_dir_all(path(root.path()).parent().unwrap()).unwrap();
        fs::write(path(root.path()), merged).unwrap();
        assert_eq!(
            read(root.path()),
            Some(Holder {
                device: A,
                name: "Desk".into(),
                since: 900,
                until: 1550
            })
        );
        assert_eq!(
            take(root.path(), B, "Laptop", 1_000),
            Turn::Elsewhere("Desk".into())
        );
        assert_eq!(take(root.path(), A, "Desk", 1_000), Turn::Mine);
    }

    #[test]
    fn a_note_that_is_not_splats_is_never_replaced_and_stops_automatic_saves() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(path(root.path()).parent().unwrap()).unwrap();
        fs::write(path(root.path()), "# My own note\n").unwrap();
        assert!(matches!(
            take(root.path(), A, "Desk", 1_000),
            Turn::Unavailable(_)
        ));
        assert_eq!(
            fs::read_to_string(path(root.path())).unwrap(),
            "# My own note\n"
        );
    }

    #[test]
    fn a_note_that_cannot_be_written_stops_automatic_saves() {
        let root = tempfile::tempdir().unwrap();
        // A folder where the note belongs cannot be read or replaced.
        fs::create_dir_all(path(root.path())).unwrap();
        assert!(matches!(
            take(root.path(), A, "Desk", 1_000),
            Turn::Unavailable(_)
        ));
    }

    #[test]
    fn an_end_too_far_ahead_comes_from_a_wrong_clock_and_does_not_hold_the_turn() {
        let root = tempfile::tempdir().unwrap();
        hold(
            root.path(),
            &Holder {
                device: A,
                name: "Desk".into(),
                since: 1_000,
                until: 1_000 + 24 * 60 * 60,
            },
        );
        assert_eq!(take(root.path(), B, "Laptop", 1_000), Turn::Settling);
    }

    #[test]
    fn a_released_turn_goes_to_the_next_computer() {
        let root = tempfile::tempdir().unwrap();
        take(root.path(), A, "Desk", 1_000);
        release(root.path(), B, 1_100);
        assert_eq!(
            take(root.path(), B, "Laptop", 1_100),
            Turn::Elsewhere("Desk".into())
        );
        release(root.path(), A, 1_100);
        assert_eq!(take(root.path(), B, "Laptop", 1_100), Turn::Settling);
    }

    #[test]
    fn a_name_that_needs_quotes_reads_back() {
        let root = tempfile::tempdir().unwrap();
        take(root.path(), A, "DM's: desk", 1_000);
        assert_eq!(read(root.path()).unwrap().name, "DM's: desk");
    }
}
