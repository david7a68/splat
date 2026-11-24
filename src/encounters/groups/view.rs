//! A group's fields, a creature's Changes, a missing member's replacement and the encounter's
//! own groups, as plain data and acts for the GPUI page: the rules
//! (ranges, which rows show, how a change is written) and the words.
#![warn(deprecated)]
use super::*;
use crate::state_context::StateContext;
use std::ops::RangeInclusive;

/// Where a group draft is: the group editor's, or the open encounter's own group at `index`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GroupAt {
    Open,
    Owned(usize),
}

/// Whose Changes a form edits: a roster row by its key, or a group's member by its place.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChangesOf {
    Row(String),
    Member(GroupAt, usize),
}

/// How a change row's number changes, as its drop-down offers it (`ChangeKind`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeHow {
    Same,
    Set,
    Add,
}

impl ChangeHow {
    fn of(value: Option<Change>) -> Self {
        match ChangeKind::of(value) {
            ChangeKind::Same => Self::Same,
            ChangeKind::Set => Self::Set,
            ChangeKind::Add => Self::Add,
        }
    }
}

/// One number of the Changes form (`change_row`).
pub struct ChangeRowView {
    pub label: &'static str,
    pub how: ChangeHow,
    /// `Set to` is a choice (a bonus only changes by an amount).
    pub can_set: bool,
    /// The set value or the amount.
    pub number: i64,
    /// The range its stepper keeps to: the set value's, else the amount's.
    pub range: RangeInclusive<i64>,
    /// What the change does, in the run's words (`change_preview`).
    pub preview: Option<String>,
    /// Why the value cannot stand, under its own field.
    pub fault: Option<String>,
}

/// The form's side row.
pub enum SideView {
    /// A roster row with no changed side: its row sets the side.
    Hidden,
    /// A roster row's changed side, with `Same as row`.
    Row { side: String, sides: Vec<String> },
    /// A group member's side, with `Same as group` while it is changed.
    Group { side: String, changed: bool },
}

/// A creature's Changes (`overrides_form`), as data.
pub struct ChangesForm {
    /// The form's key: its fields are keyed by it.
    pub key: String,
    /// `Changes for <shown name>`.
    pub title: String,
    /// The block cannot be read: the form is only this fault.
    pub unreadable: Option<String>,
    /// A fault no field explains (a change a linked sheet decides).
    pub top_fault: Option<String>,
    pub name: String,
    /// The base name, the name field's placeholder.
    pub name_hint: String,
    pub rows: Vec<ChangeRowView>,
    pub side: SideView,
    /// Starting conditions, unless a linked sheet decides them and there are none.
    pub conditions: Option<Vec<String>>,
    pub note: String,
    /// The creature's loot, one line with `;` between its parts.
    pub loot: Option<String>,
    /// The creature's shown name, for the loot list's words.
    pub loot_name: String,
}

/// One edit of a Changes form.
#[derive(Clone, Debug)]
pub enum ChangeEdit {
    Name(String),
    Note(String),
    How(&'static str, ChangeHow),
    Number(&'static str, i64),
    /// A changed side, or none (`Same as row`, `Same as group`).
    Side(Option<String>),
    /// An official condition on or off.
    Condition(String),
    /// A typed Other effect.
    AddCondition(String),
    RemoveCondition(String),
    Loot(Option<String>),
}

/// The form's numbers, in its order.
const CHANGE_LABELS: [&str; 6] = [
    "Maximum HP",
    "AC",
    "Initiative bonus",
    "Attack bonus",
    "Damage bonus",
    "Speed",
];

/// A number's `Set to` range (none for a bonus) and its `Add` range (`overrides_form`).
fn ranges(label: &str) -> (Option<RangeInclusive<i64>>, RangeInclusive<i64>) {
    match label {
        "Maximum HP" => (Some(1..=1_000_000), -1_000_000..=1_000_000),
        "AC" | "Speed" => (Some(0..=1000), -1000..=1000),
        "Initiative bonus" => (Some(-1000..=1000), -1000..=1000),
        _ => (None, -1000..=1000),
    }
}

fn value_of(overrides: &Overrides, label: &str) -> Option<Change> {
    match label {
        "Maximum HP" => overrides.max_hp,
        "AC" => overrides.armor_class,
        "Initiative bonus" => overrides.initiative_bonus,
        "Speed" => overrides.speed,
        "Attack bonus" => overrides.attack_bonus.map(Change::Add),
        "Damage bonus" => overrides.damage_bonus.map(Change::Add),
        _ => None,
    }
}

fn set_value(overrides: &mut Overrides, label: &str, value: Option<Change>) {
    let amount = value.and_then(|change| match change {
        Change::Add(amount) => Some(amount),
        Change::Set(_) => None,
    });
    match label {
        "Maximum HP" => overrides.max_hp = value,
        "AC" => overrides.armor_class = value,
        "Initiative bonus" => overrides.initiative_bonus = value,
        "Speed" => overrides.speed = value,
        "Attack bonus" => overrides.attack_bonus = amount,
        "Damage bonus" => overrides.damage_bonus = amount,
        _ => {}
    }
}

/// The Changes form of `block`, a change block of a creature `entry` on `side`. `sides` is a
/// roster row's encounter sides (its row sets the side); a group's member has none.
pub fn changes_form(
    key: &str,
    block: &Mapping,
    entry: &Entry,
    side: &str,
    sides: Option<&[String]>,
) -> ChangesForm {
    let shown = shown_name(block, entry.name.clone());
    let mut form = ChangesForm {
        key: key.to_owned(),
        title: format!("Changes for {shown}"),
        unreadable: None,
        top_fault: None,
        name: String::new(),
        name_hint: entry.name.clone(),
        rows: Vec::new(),
        side: SideView::Hidden,
        conditions: None,
        note: String::new(),
        loot: None,
        loot_name: shown,
    };
    let overrides = match Overrides::parse(block) {
        Ok(overrides) => overrides,
        Err(error) => {
            form.unreadable = Some(format!("{error} Repair it in Campaign → Campaign entries."));
            return form;
        }
    };
    let base = Base::of(entry, side);
    let linked = entry.fields.contains_key("splat_external");
    let on_a_field = CHANGE_LABELS.iter().any(|label| {
        single_change(label, value_of(&overrides, label))
            .resolve(&base)
            .is_err()
    });
    form.top_fault = overrides
        .resolve_for(entry, side)
        .err()
        .filter(|_| !on_a_field)
        .map(|error| override_error(&error));
    form.rows = CHANGE_LABELS
        .iter()
        // A linked character's sheet decides its HP. One already here can still be cleared.
        .filter(|label| **label != "Maximum HP" || !linked || overrides.max_hp.is_some())
        .map(|&label| {
            let value = value_of(&overrides, label);
            let preview = single_change(label, value);
            ChangeRowView {
                label,
                how: ChangeHow::of(value),
                can_set: ranges(label).0.is_some(),
                number: match value {
                    Some(Change::Set(number) | Change::Add(number)) => number,
                    None => 0,
                },
                range: match (value, ranges(label)) {
                    (Some(Change::Set(_)), (set, _)) => set.unwrap_or(0..=0),
                    (_, (_, add)) => add,
                },
                preview: preview
                    .rows(&base)
                    .first()
                    .map(crate::encounter_run::change_preview),
                fault: preview.resolve(&base).err().map(|e| override_error(&e)),
            }
        })
        .collect();
    form.side = match sides {
        Some(_) if overrides.side.is_none() => SideView::Hidden,
        Some(sides) => SideView::Row {
            side: overrides.side.clone().unwrap_or_default(),
            sides: sides.to_vec(),
        },
        None => SideView::Group {
            side: overrides.side.clone().unwrap_or_else(|| side.to_owned()),
            changed: overrides.side.is_some(),
        },
    };
    form.conditions =
        (!linked || !overrides.conditions.is_empty()).then(|| overrides.conditions.clone());
    form.name = overrides.name.clone().unwrap_or_default();
    form.note = overrides.note.clone().unwrap_or_default();
    form.loot = overrides.loot.clone();
    form
}

/// Write one edit into `block`, keeping it a block the form can read. True when it changed.
pub fn apply_change(block: &mut Mapping, edit: ChangeEdit) -> bool {
    let Ok(mut overrides) = Overrides::parse(block) else {
        return false;
    };
    let before = overrides.clone();
    match edit {
        ChangeEdit::Name(words) => {
            // Only a name just typed: opening the form must not edit what a note holds.
            overrides.name = (!words.trim().is_empty())
                .then(|| words.trim_start().chars().take(MAX_NAME).collect());
        }
        ChangeEdit::Note(words) => overrides.note = (!words.trim().is_empty()).then_some(words),
        ChangeEdit::How(label, how) => {
            if ChangeHow::of(value_of(&overrides, label)) != how {
                let low = ranges(label)
                    .0
                    .as_ref()
                    .map_or(1, |range| *range.start().max(&1));
                let value = match how {
                    ChangeHow::Same => None,
                    ChangeHow::Set => Some(Change::Set(low)),
                    ChangeHow::Add => Some(Change::Add(1)),
                };
                set_value(&mut overrides, label, value);
            }
        }
        ChangeEdit::Number(label, number) => {
            let (set, add) = ranges(label);
            match value_of(&overrides, label) {
                Some(Change::Set(_)) => {
                    let range = set.unwrap_or(0..=0);
                    let number = number.clamp(*range.start(), *range.end());
                    set_value(&mut overrides, label, Some(Change::Set(number)));
                }
                // Zero changes nothing, so the choice for that is Same: a typed 0 keeps the
                // amount (issue #586).
                Some(Change::Add(_)) if number != 0 => {
                    let number = number.clamp(*add.start(), *add.end());
                    set_value(&mut overrides, label, Some(Change::Add(number)));
                }
                _ => {}
            }
        }
        ChangeEdit::Side(side) => {
            overrides.side = side
                .map(|side| side.trim().to_lowercase())
                .filter(|side| !side.is_empty());
        }
        ChangeEdit::Condition(name) => {
            match overrides
                .conditions
                .iter()
                .position(|c| same_condition(c, &name))
            {
                Some(at) => {
                    overrides.conditions.remove(at);
                }
                None => overrides.conditions.push(name),
            }
        }
        ChangeEdit::AddCondition(text) => {
            let text = text.trim();
            if !text.is_empty() && !overrides.conditions.iter().any(|c| same_condition(c, text)) {
                overrides.conditions.push(text.to_owned());
            }
        }
        ChangeEdit::RemoveCondition(name) => overrides.conditions.retain(|c| *c != name),
        ChangeEdit::Loot(loot) => overrides.loot = loot,
    }
    let changed = overrides != before;
    if changed {
        *block = overrides.to_mapping();
    }
    changed
}

/// The creature a reference names in this read, if it has an entry.
fn entry_of<'a>(snapshot: &'a Snapshot, reference: &EntryRef) -> Option<&'a Entry> {
    snapshot
        .catalogue
        .resolve_ref(reference)
        .ok()
        .and_then(|id| snapshot.catalogue.entries.get(&id))
        .map(|item| &item.entry)
}

/// A group editor's fields (`group_fields`): Default side, Belongs to, the members, the notes.
pub struct GroupFields {
    /// The group's own id: its fields' key.
    pub id: Uuid,
    pub side: String,
    /// Belongs to: `Any encounter`, then each encounter; the chosen one; its help or fault.
    pub belongs: Vec<(Option<Uuid>, String)>,
    pub belongs_chosen: Option<usize>,
    pub belongs_help: Option<&'static str>,
    pub belongs_fault: Option<&'static str>,
    pub count: usize,
    pub heading: String,
    pub members: Vec<MemberRow>,
    /// Members whose rows cannot be read, as their words.
    pub unread: Vec<String>,
    pub notes: String,
}

/// One member of a group.
pub struct MemberRow {
    pub index: usize,
    /// Its number among the group's rows, unread rows counted.
    pub number: usize,
    pub name: String,
    pub detail: String,
    pub hp_missing: bool,
    pub count: u64,
    pub missing: bool,
    pub first: bool,
    pub last: bool,
    /// The key of its Changes form (`member_key`).
    pub key: String,
    pub open: bool,
    /// The change words at the end of its detail line while its form is closed.
    pub change_word: Option<String>,
    /// A missing creature's fault (its words, and whether it is an error).
    pub fault: Option<(String, bool)>,
    /// A change that cannot stand, while its form is closed.
    pub change_fault: Option<String>,
    /// `Replace…` for a missing creature, else `Edit`.
    pub edit_word: &'static str,
    /// The edit button's name: `Replace <name>` or `Changes for <name>`.
    pub edit_name: String,
}

impl Encounters {
    fn group_at(&self, at: GroupAt) -> Option<&GroupDraft> {
        match at {
            GroupAt::Open => self.group_draft.as_ref(),
            GroupAt::Owned(index) => self.draft.as_ref()?.groups.get(index),
        }
    }

    /// A group's fields (`group_fields`), with the loot sources for the change words. `open` is
    /// the encounter being edited, for its own groups.
    pub(super) fn group_fields_at(
        &self,
        at: GroupAt,
        sources: Option<&crate::loot_editor::Sources>,
    ) -> Option<GroupFields> {
        let draft = self.group_at(at)?;
        let snapshot = self.snapshot.as_ref();
        let open = match at {
            GroupAt::Open => None,
            GroupAt::Owned(_) => self
                .draft
                .as_ref()
                .map(|encounter| (encounter.id, encounter.name.clone())),
        };
        let mut belongs: Vec<(Option<Uuid>, String)> = vec![(None, ANY_ENCOUNTER.to_owned())];
        if let Some(snapshot) = snapshot {
            belongs.extend(
                snapshot
                    .encounter_choices()
                    .filter(|(id, _)| open.as_ref().is_none_or(|(open, _)| open != id))
                    .map(|(id, name)| (Some(id), name.to_owned())),
            );
        }
        if let Some((id, name)) = &open {
            belongs.push((Some(*id), name.clone()));
        }
        let current = draft
            .setup
            .encounter
            .as_ref()
            .and_then(|reference| reference.id);
        let mut belongs_chosen = belongs.iter().position(|(id, _)| *id == current);
        let gone = current.is_some() && belongs_chosen.is_none();
        if gone {
            belongs.push((current, MISSING_ENCOUNTER.to_owned()));
            belongs_chosen = Some(belongs.len() - 1);
        }
        // In an encounter's editor, belonging to it goes without saying.
        let own = current.is_some() && current == open.as_ref().map(|(id, _)| *id);
        let unread =
            splat::content::list_formats::unread_group_members(&draft.source, &draft.flags);
        let saved_members = draft.setup.members == draft.saved_setup.members;
        let side = draft.setup.side.clone();
        let members = snapshot.map_or_else(Vec::new, |snapshot| {
            let total = draft.setup.members.len();
            draft
                .setup
                .members
                .iter()
                .enumerate()
                .map(|(index, member)| {
                    member_row(
                        draft,
                        snapshot,
                        (index, total),
                        member,
                        &side,
                        &unread,
                        saved_members,
                        (self.expanded.as_deref(), sources),
                    )
                })
                .collect()
        });
        let count = draft
            .setup
            .members
            .iter()
            .map(|member| member.count)
            .sum::<u64>()
            + unread.iter().filter_map(|member| member.count).sum::<u64>();
        Some(GroupFields {
            id: draft.id(),
            side,
            belongs,
            belongs_chosen,
            belongs_help: match current {
                _ if own => None,
                None => Some("A run of any encounter offers this group."),
                Some(_) if gone => None,
                Some(_) => Some("Only runs of this encounter offer this group."),
            },
            belongs_fault: gone.then_some(GONE_ENCOUNTER),
            count: count as usize,
            heading: if draft.setup.members.is_empty() {
                String::new()
            } else {
                snapshot.map_or_else(String::new, |snapshot| members_heading(draft, snapshot))
            },
            members,
            unread: unread
                .iter()
                .map(|member| member.flag.message.clone())
                .collect(),
            notes: draft.notes.to_string(),
        })
    }

    /// Change a group draft: the group editor's, or one of the open encounter's own.
    pub fn edit_group_at(&mut self, at: GroupAt, edit: impl FnOnce(&mut GroupEdit<'_>)) {
        let draft = match at {
            GroupAt::Open => self.group_draft.as_mut(),
            GroupAt::Owned(index) => self
                .draft
                .as_mut()
                .and_then(|draft| draft.groups.get_mut(index)),
        };
        if let Some(draft) = draft {
            edit(&mut GroupEdit(draft));
        }
    }

    /// A member's change from its row, in any group draft (`member_act` for the open one).
    pub fn member_act_at(&mut self, at: GroupAt, act: MemberAct) {
        let Encounters {
            snapshot,
            group_draft,
            draft,
            expanded,
            ..
        } = self;
        let Some(snapshot) = snapshot.as_ref() else {
            return;
        };
        let group = match at {
            GroupAt::Open => group_draft.as_mut(),
            GroupAt::Owned(index) => draft.as_mut().and_then(|draft| draft.groups.get_mut(index)),
        };
        let Some(group) = group else {
            return;
        };
        let total = group.setup.members.len();
        let path = group.path.clone();
        match act {
            MemberAct::Up(index) if index > 0 && index < total => {
                group.setup.members.swap(index, index - 1);
            }
            MemberAct::Down(index) if index + 1 < total => {
                group.setup.members.swap(index, index + 1);
            }
            MemberAct::Remove(index) if index < total => {
                // Only the removed member's own form closes: a form is keyed by its member.
                if expanded.as_deref()
                    == Some(member_key(&path, &group.setup.members[index]).as_str())
                {
                    *expanded = None;
                }
                remove_member(group, index, snapshot);
            }
            MemberAct::Count(index, count) if index < total => {
                group.setup.members[index].count = count.clamp(1, MAX_COUNT);
            }
            _ => return,
        }
        group.members_saved = None;
        group.touch();
    }

    /// A member's Edit (or `Replace…`): its form opens, or closes when open.
    pub fn toggle_member(&mut self, at: GroupAt, index: usize) {
        let Some(key) = self
            .group_at(at)
            .and_then(|group| Some(member_key(&group.path, group.setup.members.get(index)?)))
        else {
            return;
        };
        self.toggle_row(&key);
    }

    /// The Changes form of a roster row or a member, when it has a creature.
    pub fn changes_form(&self, of: &ChangesOf) -> Option<ChangesForm> {
        let snapshot = self.snapshot.as_ref()?;
        match of {
            ChangesOf::Row(key) => {
                let draft = self.draft.as_ref()?;
                let slot = draft.setup.roster.iter().find(|slot| slot.key == *key)?;
                let entry = entry_of(snapshot, slot.creature.as_ref()?)?;
                let sides: Vec<String> = draft
                    .setup
                    .sides
                    .keys()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect();
                Some(changes_form(
                    &format!("{}:{}", draft.id, slot.key),
                    &slot.overrides,
                    entry,
                    &slot.side,
                    Some(&sides),
                ))
            }
            ChangesOf::Member(at, index) => {
                let group = self.group_at(*at)?;
                let member = group.setup.members.get(*index)?;
                let entry = entry_of(snapshot, &member.creature)?;
                Some(changes_form(
                    &member_key(&group.path, member),
                    &member.overrides,
                    entry,
                    &group.setup.side,
                    None,
                ))
            }
        }
    }

    /// One edit of a Changes form. A member's change is an edit of its group.
    pub fn edit_changes(&mut self, of: &ChangesOf, edit: ChangeEdit) {
        let Encounters {
            snapshot,
            draft,
            group_draft,
            ..
        } = self;
        if snapshot.is_none() {
            return;
        }
        match of {
            ChangesOf::Row(key) => {
                let Some(draft) = draft.as_mut() else {
                    return;
                };
                if let Some(slot) = draft.setup.roster.iter_mut().find(|slot| slot.key == *key) {
                    apply_change(&mut slot.overrides, edit);
                }
                draft.keep_sides();
            }
            ChangesOf::Member(at, index) => {
                let group = match at {
                    GroupAt::Open => group_draft.as_mut(),
                    GroupAt::Owned(owned) => draft
                        .as_mut()
                        .and_then(|draft| draft.groups.get_mut(*owned)),
                };
                if let Some(group) = group
                    && let Some(member) = group.setup.members.get_mut(*index)
                    && apply_change(&mut member.overrides, edit)
                {
                    group.touch();
                }
            }
        }
    }

    /// `Done` on a Changes form, or a replacement: no form stays open.
    pub fn close_form(&mut self) {
        self.expanded = None;
    }

    /// `Use` in a missing member's replacement picker: the member takes `creature`, with a
    /// result whose Undo gives its creature back.
    pub fn replace_member(&mut self, at: GroupAt, index: usize, creature: Uuid) {
        let Encounters {
            snapshot,
            group_draft,
            draft,
            expanded,
            ..
        } = self;
        let Some(snapshot) = snapshot.as_ref() else {
            return;
        };
        let group = match at {
            GroupAt::Open => group_draft.as_mut(),
            GroupAt::Owned(owned) => draft.as_mut().and_then(|draft| draft.groups.get_mut(owned)),
        };
        let Some(group) = group else {
            return;
        };
        let Some(member) = group.setup.members.get_mut(index) else {
            return;
        };
        let reference = EntryRef::id(creature);
        let name = missing_member_name(None, member.creature.link.as_deref());
        let name = Overrides::parse(&member.overrides)
            .ok()
            .and_then(|overrides| overrides.name)
            .map_or(name, |changed| changed.trim().to_owned());
        let chosen = entry_of(snapshot, &reference)
            .map_or_else(|| reference.label(), |entry| entry.name.clone());
        let result = Replaced {
            member: member.id,
            before: std::mem::replace(&mut member.creature, reference.clone()),
            after: reference,
            words: format!("Replaced {name} with {chosen}."),
        };
        group.member_new = Some(MemberUndo::Replaced(result));
        group.members_saved = None;
        group.touch();
        *expanded = None;
    }
}

/// One member's row (`members`), as data.
#[allow(clippy::too_many_arguments)]
fn member_row(
    draft: &GroupDraft,
    snapshot: &Snapshot,
    (index, total): (usize, usize),
    member: &MemberDraft,
    side: &str,
    unread: &[splat::content::list_formats::UnreadGroupMember],
    saved_members: bool,
    (expanded, sources): (Option<&str>, Option<&crate::loot_editor::Sources>),
) -> MemberRow {
    let parsed = Overrides::parse(&member.overrides);
    let assessed = assess_member(
        &snapshot.catalogue,
        &snapshot.creatures,
        &member.creature,
        side,
        parsed.as_ref().map_err(String::as_str),
    );
    let entry = assessed.entry;
    let overrides = entry.and(parsed.as_ref().ok());
    let base = entry.map_or_else(
        || missing_member_name(None, member.creature.link.as_deref()),
        |entry| entry.name.clone(),
    );
    let edit_name = if entry.is_none() {
        format!("Replace {base}")
    } else {
        format!("Changes for {base}")
    };
    let name = overrides
        .and_then(|overrides| overrides.name.as_deref())
        .map_or(base, |changed| changed.trim().to_owned());
    let (detail, hp_missing) = entry
        .zip(assessed.creature)
        .map(|(entry, creature)| {
            let changed = assessed.changed.as_ref().ok().and_then(Option::as_ref);
            let (mut detail, hp_missing) = changed.map_or_else(
                || (creature.detail.clone(), creature.hp_missing),
                |changed| {
                    (
                        super::super::creature_detail(entry, creature.party, Some(changed)),
                        changed.max_hp.is_none(),
                    )
                },
            );
            if member.overrides.contains_key("name") {
                detail.push_str(&format!(" · as {}", entry.name));
            }
            (detail, hp_missing)
        })
        .unwrap_or_default();
    let key = member_key(&draft.path, member);
    let open = expanded == Some(key.as_str());
    let change_word = entry
        .zip(overrides)
        .filter(|_| !open && !member.overrides.is_empty())
        .map(|(entry, overrides)| {
            let missing = match (sources, overrides.loot.as_deref()) {
                (Some(sources), Some(loot)) => {
                    let parts = splat::content::loot::parts(loot);
                    sources.missing(parts.iter().map(String::as_str))
                }
                _ => Vec::new(),
            };
            change_words(overrides, entry, side, &missing)
        });
    let number = index
        + unread
            .iter()
            .filter(|unread| unread.before <= index)
            .count()
        + 1;
    let line = saved_members
        .then(|| draft.member_lines.get(index).copied())
        .flatten();
    let fault = match (entry, assessed.missing) {
        (None, Some((kind, cause))) => {
            let changed = member
                .overrides
                .get("name")
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|changed| !changed.is_empty());
            Some((
                member_fault(
                    changed.unwrap_or(&name),
                    member.creature.link.is_some().then_some(name.as_str()),
                    line,
                    cause,
                ),
                kind.tone() == Tone::Error,
            ))
        }
        _ => None,
    };
    let change_fault = entry.and_then(|entry| {
        let error = assessed.changed.as_ref().err().filter(|_| !open)?;
        Some(member_fault(
            &name,
            Some(&entry.name),
            line,
            &override_error(error),
        ))
    });
    MemberRow {
        index,
        number,
        name,
        detail,
        hp_missing,
        count: member.count,
        missing: entry.is_none(),
        first: index == 0,
        last: index + 1 == total,
        key,
        open,
        change_word,
        fault,
        change_fault,
        edit_word: if entry.is_none() {
            "Replace…"
        } else {
            "Edit"
        },
        edit_name,
    }
}

/// One of the open encounter's own groups (`of_encounter`), as data.
pub struct GroupCard {
    pub index: usize,
    pub path: String,
    pub name: String,
    /// What the search adds to now.
    pub adding: bool,
    /// The other encounters `Move to` offers.
    pub move_to: Vec<(Uuid, String)>,
    pub fields: GroupFields,
    /// Its name field asked for the keyboard (a group just made); asking ends it.
    pub focus: bool,
}

/// The open encounter's groups (`of_encounter`).
pub struct OwnedGroups {
    pub count: usize,
    pub cards: Vec<GroupCard>,
}

/// A press on one of the encounter's own groups.
#[derive(Clone, Debug)]
pub enum OwnedAct {
    /// `Add here`, or `Adding here` to stop.
    AddHere,
    MoveTo(Uuid),
    /// `Make available to any encounter`.
    AnyEncounter,
    Delete,
    Name(String),
}

impl Encounters {
    /// The open encounter's own groups, once they are read.
    pub fn owned_groups(&mut self, ctx: &StateContext) -> Option<OwnedGroups> {
        let sources = crate::loot_editor::sources(ctx);
        let draft = self.draft.as_ref()?;
        if !draft.groups_read {
            return None;
        }
        let this = draft.id;
        let focus = self
            .draft
            .as_mut()
            .and_then(|draft| draft.focus_group.take());
        let draft = self.draft.as_ref()?;
        let move_to: Vec<(Uuid, String)> =
            self.snapshot.as_ref().map_or_else(Vec::new, |snapshot| {
                snapshot
                    .encounter_choices()
                    .filter(|(id, _)| *id != this)
                    .map(|(id, name)| (id, name.to_owned()))
                    .collect()
            });
        let cards: Vec<GroupCard> = draft
            .groups
            .iter()
            .enumerate()
            .filter(|(_, group)| {
                group.setup.encounter.as_ref().and_then(|owner| owner.id) == Some(this)
            })
            .filter_map(|(index, group)| {
                Some(GroupCard {
                    index,
                    path: group.path.clone(),
                    name: group.name.clone(),
                    adding: draft.add_to.as_deref() == Some(group.path.as_str()),
                    move_to: move_to.clone(),
                    fields: self.group_fields_at(GroupAt::Owned(index), sources.as_deref())?,
                    focus: focus.as_deref() == Some(group.path.as_str()),
                })
            })
            .collect();
        Some(OwnedGroups {
            count: cards.len(),
            cards,
        })
    }

    /// A press on one of the encounter's own groups (`of_encounter`).
    pub fn owned_act(&mut self, index: usize, act: OwnedAct) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let Some(group) = draft.groups.get_mut(index) else {
            return;
        };
        let path = group.path.clone();
        match act {
            OwnedAct::AddHere => {
                if draft.add_to.as_deref() == Some(path.as_str()) {
                    draft.add_to = None;
                } else {
                    draft.add_to = Some(path);
                    self.focus_search = true;
                    self.editor_pane = EditorPane::Add;
                }
            }
            OwnedAct::MoveTo(id) => {
                group.setup.encounter = Some(EntryRef::id(id));
                // N1: who owns the group is checked by Save.
                group.touch();
            }
            OwnedAct::Name(words) => {
                group.name = words;
                group.touch();
            }
            OwnedAct::AnyEncounter | OwnedAct::Delete => {
                let delete = matches!(act, OwnedAct::Delete);
                let gone = group.clone();
                if draft.add_to.as_deref() == Some(gone.path()) {
                    draft.add_to = None;
                }
                if delete {
                    let removed = draft.groups.remove(index);
                    draft.deleted_groups.push(removed);
                } else {
                    draft.groups[index].setup.encounter = None;
                    // N1: a group for any encounter is checked against the others' names.
                    draft.groups[index].touch();
                }
                draft
                    .new_results
                    .push(super::super::DraftUndo::RemovedGroup {
                        index,
                        group: Box::new(gone),
                        delete,
                    });
            }
        }
    }

    /// `New group for this encounter`: a group that takes what the search adds, and whose name
    /// takes the keyboard.
    pub fn new_owned_group(&mut self) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let group = GroupDraft::for_encounter(draft.id);
        draft.add_to = Some(group.path.clone());
        draft.focus_group = Some(group.path.clone());
        draft.groups.push(group);
    }
}
