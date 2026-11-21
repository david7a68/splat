//! `splat_kind: encounter_group`: a reusable, named list of creatures that a DM adds to a
//! running encounter when they arrive. This reads the note. A run decides what it does.
use super::{Entry, Reference, overrides::Overrides, resource_key};
use serde_yaml_ng::{Mapping, Value};
use uuid::Uuid;

/// A group is a fixed list for one table, so a typing mistake cannot flood a run.
pub const MAX_COUNT: u64 = 100;

#[derive(Clone, Debug, PartialEq)]
pub struct Member {
    pub creature: Reference,
    pub count: u64,
    pub overrides: Overrides,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EncounterGroup {
    pub side: String,
    /// The one encounter this group is for. With none, a run of any encounter offers it.
    pub encounter: Option<Uuid>,
    pub members: Vec<Member>,
}

fn only_fields(map: &Mapping, allowed: &[&str], what: &str) -> Result<(), String> {
    for key in map.keys() {
        let name = key.as_str().unwrap_or("");
        if !allowed.contains(&name) {
            return Err(format!(
                "{what}: unknown field {name}. Use: {}.",
                allowed.join(", ")
            ));
        }
    }
    Ok(())
}

/// One reference, in the form a roster slot holds it, so that the editor can read any note
/// that this accepts.
fn creature(what: &str, value: &Value) -> Result<Reference, String> {
    let map = value
        .as_mapping()
        .ok_or_else(|| format!("{what} needs one creature reference."))?;
    only_fields(map, &["id", "key", "link"], what)?;
    if map.get("link").is_some_and(|link| !link.is_string()) {
        return Err(format!("{what}: a link must be text."));
    }
    let invalid = || format!("{what} needs exactly one valid id or key.");
    match (map.get("id"), map.get("key")) {
        (Some(id), None) => id
            .as_str()
            .and_then(|id| Uuid::parse_str(id).ok())
            .filter(|id| !id.is_nil())
            .map(Reference::Id)
            .ok_or_else(invalid),
        (None, Some(key)) => key
            .as_str()
            .filter(|key| resource_key(key))
            .map(|key| Reference::Key(key.to_owned()))
            .ok_or_else(invalid),
        (None, None) if map.get("link").and_then(Value::as_str).is_some() => {
            Ok(Reference::Id(Uuid::nil()))
        }
        _ => Err(invalid()),
    }
}

/// By id alone, because an encounter a group can belong to is a note of its own campaign, and
/// a package key names something a campaign can only use.
fn encounter(value: &Value) -> Result<Uuid, String> {
    const INVALID: &str = "This group’s encounter needs one valid id.";
    let map = value.as_mapping().ok_or(INVALID)?;
    only_fields(map, &["id", "link"], "This group’s encounter")?;
    if map.get("link").is_some_and(|link| !link.is_string()) {
        return Err("This group’s encounter: a link must be text.".into());
    }
    map.get("id")
        .and_then(Value::as_str)
        .and_then(|id| Uuid::parse_str(id).ok())
        .filter(|id| !id.is_nil())
        .ok_or_else(|| INVALID.to_owned())
}

fn member(number: usize, value: &Value) -> Result<Member, String> {
    let what = format!("Member {number}");
    let map = value
        .as_mapping()
        .ok_or_else(|| format!("{what} must be a mapping."))?;
    only_fields(map, &["creature", "count", "overrides"], &what)?;
    let reference = map
        .get("creature")
        .ok_or_else(|| format!("{what} needs a creature."))?;
    let creature = creature(&what, reference)?;
    let count = match map.get("count") {
        None => 1,
        Some(count) => count
            .as_u64()
            .filter(|count| (1..=MAX_COUNT).contains(count))
            .ok_or_else(|| format!("{what}: count must be from 1 to {MAX_COUNT}."))?,
    };
    let overrides = match map.get("overrides") {
        None => Overrides::default(),
        Some(value) => Overrides::parse(
            value
                .as_mapping()
                .ok_or_else(|| format!("{what}: overrides must be a mapping."))?,
        )
        .map_err(|e| format!("{what}: {e}"))?,
    };
    Ok(Member {
        creature,
        count,
        overrides,
    })
}

impl EncounterGroup {
    pub fn parse(value: &Value) -> Result<Self, String> {
        let map = value
            .as_mapping()
            .ok_or("splat_encounter_group must be a mapping")?;
        only_fields(map, &["side", "encounter", "members"], "This group")?;
        let side = map
            .get("side")
            .and_then(Value::as_str)
            .filter(|side| !side.trim().is_empty())
            .ok_or("This group needs a default side.")?;
        let members = map
            .get("members")
            .and_then(Value::as_sequence)
            .filter(|members| !members.is_empty())
            .ok_or("This group is empty. Add at least one creature.")?;
        Ok(Self {
            side: side.trim().to_owned(),
            encounter: map.get("encounter").map(encounter).transpose()?,
            members: members
                .iter()
                .enumerate()
                .map(|(index, value)| member(index + 1, value))
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn of(entry: &Entry) -> Result<Self, String> {
        let value = entry
            .fields
            .get("splat_encounter_group")
            .ok_or("Missing splat_encounter_group")?;
        Self::parse(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn note(group: &str) -> Result<Entry, String> {
        Entry::parse(
            "group.md",
            &crate::content::formats::fixture(
                "Fixture.md",
                &format!(
                    "splat_id: {}\nsplat_kind: encounter_group\nsplat_name: Rival team\nsplat_encounter_group: {group}\n",
                    Uuid::new_v4()
                ),
                "\nThey arrive through the east gate.\n",
            )?,
        )
    }

    #[test]
    fn a_group_note_keeps_its_order_counts_side_and_notes() {
        let id = Uuid::new_v4();
        let entry = note(&format!(
            "{{side: rivals, members: [{{creature: {{key: 'dnd2024:creatures/goblin_warrior'}}, count: 3}}, {{creature: {{id: {id}}}, overrides: {{max_hp: {{add: 10}}}}}}]}}"
        ))
        .unwrap();
        let group = EncounterGroup::of(&entry).unwrap();
        assert_eq!(group.side, "rivals");
        assert_eq!(group.members[0].count, 3);
        assert_eq!(group.members[1].count, 1, "one, unless it says otherwise");
        assert_eq!(group.members[1].creature, Reference::Id(id));
        assert!(!group.members[1].overrides.is_empty());
        assert!(entry.body.contains("east gate"));
        assert_eq!(group.encounter, None, "for any encounter, unless it says");
    }

    #[test]
    fn a_group_can_belong_to_one_encounter_by_its_id() {
        let (creature, owner) = (Uuid::new_v4(), Uuid::new_v4());
        let entry = note(&format!(
            "{{side: enemies, encounter: {{id: {owner}, link: '[[Arena]]'}}, members: [{{creature: {{id: {creature}}}}}]}}"
        ))
        .unwrap();
        assert_eq!(EncounterGroup::of(&entry).unwrap().encounter, Some(owner));
        for (encounter, words) in [
            (
                "{key: 'dnd2024:encounters/arena'}".to_owned(),
                "unknown field",
            ),
            ("{}".to_owned(), "one valid id"),
            ("arena".to_owned(), "one valid id"),
            (format!("{{id: {}}}", Uuid::nil()), "one valid id"),
            (format!("{{id: {owner}, link: 3}}"), "link must be text"),
            (format!("{{id: {owner}, side: a}}"), "unknown field"),
        ] {
            let error = note(&format!(
                "{{side: enemies, encounter: {encounter}, members: [{{creature: {{id: {creature}}}}}]}}"
            ))
            .unwrap_err();
            assert!(error.contains(words), "{encounter}: {error}");
        }
    }

    #[test]
    fn a_group_for_a_missing_or_wrong_encounter_is_reported_by_validation() {
        let (creature, owner) = (Uuid::new_v4(), Uuid::new_v4());
        let group = note(&format!(
            "{{side: enemies, encounter: {{id: {owner}}}, members: [{{creature: {{id: {creature}}}}}]}}"
        ))
        .unwrap();
        let wrote = |id: Uuid, kind: &str, field: &str| {
            Entry::parse(
                "other.md",
                &crate::content::formats::fixture(
                    "Fixture.md",
                    &format!("splat_id: {id}\nsplat_kind: {kind}\nsplat_name: Other\n{field}\n"),
                    "",
                )
                .expect("schema-2 fixture"),
            )
            .unwrap()
        };
        let monster = wrote(creature, "creature", "splat_creature: {}\nsplat_stats: {}");
        let says = |entries: &[Entry], words: &str| {
            super::super::validate(entries)
                .errors
                .iter()
                .any(|e| e.message.contains(words))
        };
        assert!(says(
            &[group.clone(), monster.clone()],
            "Unresolved reference"
        ));
        let spell = wrote(owner, "spell", "splat_spell: {}");
        assert!(says(
            &[group.clone(), monster.clone(), spell],
            "must reference an encounter"
        ));
        let arena = wrote(
            owner,
            "encounter",
            "splat_encounter: {roster: [], sides: {}, assumptions: [], completion: {mode: manual}, limits: {rounds: 1, resolution_events: 1}}",
        );
        assert!(!says(&[group, monster, arena], "encounter"));
    }

    #[test]
    fn an_invalid_group_is_refused_in_words() {
        let id = Uuid::new_v4();
        for (group, words) in [
            ("{}".to_owned(), "default side"),
            ("{side: enemies}".to_owned(), "empty"),
            ("{side: enemies, members: []}".to_owned(), "empty"),
            ("{side: ' ', members: []}".to_owned(), "default side"),
            ("{side: enemies, members: [3]}".to_owned(), "Member 1"),
            (
                "{side: enemies, members: [{count: 2}]}".to_owned(),
                "needs a creature",
            ),
            (
                format!("{{side: enemies, members: [{{creature: {{id: {id}}}, count: 0}}]}}"),
                "count must be",
            ),
            (
                format!("{{side: enemies, members: [{{creature: {{id: {id}}}, count: 101}}]}}"),
                "count must be",
            ),
            (
                format!("{{side: enemies, members: [{{creature: {{id: {id}}}, level: 3}}]}}"),
                "unknown field",
            ),
            (
                format!(
                    "{{side: enemies, members: [{{creature: {{id: {id}}}}}, {{creature: {{id: {id}}}, overrides: {{max_hp: {{set: 0}}}}}}]}}"
                ),
                "Member 2: max_hp",
            ),
            (
                format!("{{side: enemies, extra: 1, members: [{{creature: {{id: {id}}}}}]}}"),
                "unknown field",
            ),
            (
                format!("{{side: enemies, members: [{{creature: [{{id: {id}}}]}}]}}"),
                "one creature reference",
            ),
            (
                format!("{{side: enemies, members: [{{creature: {{wrap: {{id: {id}}}}}}}]}}"),
                "unknown field",
            ),
            (
                format!("{{side: enemies, members: [{{creature: {{key: {id}}}}}]}}"),
                "valid id or key",
            ),
        ] {
            let error = note(&group).unwrap_err();
            assert!(error.contains(words), "{group}: {error}");
        }
    }

    #[test]
    fn a_roster_slot_with_a_bad_override_names_its_slot() {
        let error = crate::content::formats::fixture("encounter.md", &format!("splat_id: {}\nsplat_kind: encounter\nsplat_name: Arena\nsplat_encounter:\n  roster:\n  - key: rival_one\n    overrides: {{max_hp: {{set: 0}}}}\n", Uuid::new_v4()), "").unwrap_err();
        assert!(
            error.contains("rival_one") && error.contains("max_hp"),
            "{error}"
        );
    }

    #[test]
    fn an_unresolved_or_wrong_creature_is_reported_by_validation() {
        let id = Uuid::new_v4();
        let group = note(&format!(
            "{{side: enemies, members: [{{creature: {{id: {id}}}}}]}}"
        ))
        .unwrap();
        let report = super::super::validate(std::slice::from_ref(&group));
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.message.contains("Unresolved")),
            "{:?}",
            report.errors
        );
        let spell = Entry::parse(
            "spell.md",
            &crate::content::formats::fixture(
                "Fixture.md",
                &format!(
                    "splat_id: {id}\nsplat_kind: spell\nsplat_name: Bless\nsplat_spell: {{}}\n"
                ),
                "",
            )
            .expect("schema-2 fixture"),
        )
        .unwrap();
        let report = super::super::validate(&[group, spell]);
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.message.contains("must reference a creature")),
            "{:?}",
            report.errors
        );
    }
}
