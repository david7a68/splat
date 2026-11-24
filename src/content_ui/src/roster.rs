use splat::content::{
    Entry,
    overrides::{Base, Overrides, Resolved, Row},
};
use std::sync::Arc;
use uuid::Uuid;
/// One line of an encounter group, as a run adds it.
#[derive(Clone, Debug)]
pub struct GroupMember {
    pub entry: Arc<Entry>,
    pub count: u64,
    pub overrides: Overrides,
}
/// A campaign's encounter group, ready to add. Its members are resolved when it loads.
#[derive(Clone, Debug)]
pub struct RunGroup {
    pub id: Uuid,
    pub path: String,
    pub name: String,
    pub side: String,
    /// The one encounter this group is for. With none, it is for any encounter.
    pub encounter: Option<Uuid>,
    /// That encounter's name, while this campaign still has it.
    pub encounter_name: Option<String>,
    pub members: Vec<GroupMember>,
    pub notes: String,
    /// What this group's creatures are worth. Overrides do not change it.
    pub xp: u64,
    /// The creatures with no XP in their entry, so that the total is not read as complete.
    pub xp_unknown: Vec<String>,
    /// How many creatures the note names that this campaign cannot find.
    pub unavailable: u64,
    /// Missing members retain their position, count and display name for the summary.
    pub unavailable_members: Vec<(usize, u64, String)>,
    /// Every reason this group cannot be added now.
    pub problem: Option<String>,
    /// Unread rows do not block the readable members.
    pub warnings: Vec<MemberFault>,
}

/// Why one member of a group cannot join. The check that finds the fault decides its kind, and
/// every view of the group (its editor, the Encounters list, Arrivals, the session plan) takes
/// the fault's tone and repair link from that kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberFault {
    pub kind: FaultKind,
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultKind {
    /// The note names a creature that this campaign's packages do not have.
    NotInPackages,
    /// The member's changes make a creature that cannot be, such as a maximum HP below 1.
    InvalidChange,
    /// The member can be read but not run: a D&D Beyond sheet, no maximum HP.
    CannotJoin,
    /// The member's row, or its creature's file, is there and cannot be read (R41, lead ruling
    /// s30 B11): an error wherever it shows.
    Unreadable,
}

impl FaultKind {
    /// An invalid value and unreadable data are errors wherever they show (R41). A member that is
    /// only left out is a warning: the group still loads, and its other members can join.
    pub fn tone(self) -> crate::design::Tone {
        match self {
            Self::InvalidChange | Self::Unreadable => crate::design::Tone::Error,
            Self::NotInPackages | Self::CannotJoin => crate::design::Tone::Warning,
        }
    }
}

impl MemberFault {
    pub fn new(kind: FaultKind, text: String) -> Self {
        Self { kind, text }
    }
    pub fn tone(&self) -> crate::design::Tone {
        self.kind.tone()
    }
}

impl std::ops::Deref for MemberFault {
    type Target = str;
    fn deref(&self) -> &str {
        &self.text
    }
}

#[cfg(any(test, feature = "test-support"))]
impl From<&str> for MemberFault {
    fn from(text: &str) -> Self {
        Self::new(FaultKind::CannotJoin, text.to_owned())
    }
}

impl RunGroup {
    /// The group's own reasons it cannot join, such as `No members can join.`: the lines of
    /// `problem` after the ones its member faults wrote. Every view draws the member faults with
    /// their own tone, then these as warnings.
    pub fn own_problems(&self) -> Vec<&str> {
        let members = self
            .warnings
            .iter()
            .map(|fault| fault.lines().count())
            .sum();
        self.problem
            .as_deref()
            .map(|problem| problem.lines().skip(members).collect())
            .unwrap_or_default()
    }

    /// The first member's bonus decides a shared roll, as it does for any initiative group.
    pub fn leader(&self) -> Option<Resolved> {
        let member = self.members.first()?;
        let base = Base::of(&member.entry, &self.side);
        member.overrides.resolve(&base).ok()
    }
    /// The group's own facts in the one order of `difficulty::FightAfter` (side, count, then
    /// its XP when `worth`): `Enemies · 3 members · XP 150`. The Encounters list, the plan
    /// (through `summary`) and Arrivals all start a group's line with these words.
    pub fn facts(&self, worth: bool) -> String {
        FightAfter {
            side: &crate::design::reader::human_label(&self.side),
            members: &self.count_label(),
            worth: &if worth {
                self.xp_label()
            } else {
                String::new()
            },
        }
        .facts()
    }
    /// How many, on which side, and what they are worth, in one line.
    pub fn summary(&self) -> String {
        // When nobody can join, the count says it all: no XP, no `Will add: none`.
        if self.joinable_count() == 0 && self.unavailable > 0 {
            return self.facts(false);
        }
        let mut summary = self.facts(true);
        if self
            .members
            .iter()
            .any(|member| member.overrides.changes_creature())
        {
            // FJ5: `Changed` keeps its muted pencil wherever a group's summary shows, as in
            // Arrivals. The no-break space keeps the two on one line.
            summary.push_str(&format!(
                " · {}\u{a0}Changed",
                crate::design::icons::CHANGED
            ));
        }
        if self
            .members
            .iter()
            .any(|member| member.overrides.loot.is_some())
        {
            summary.push_str(" · Loot");
        }
        let members = self.member_summary();
        if !members.is_empty() {
            summary.push_str(&format!(" · {members}"));
        }
        summary
    }
    pub fn joinable_count(&self) -> u64 {
        self.members.iter().map(|member| member.count).sum()
    }
    pub fn count_label(&self) -> String {
        if self.unavailable_members.is_empty() {
            crate::design::count(self.creature_count() as usize, "member", "members")
        } else {
            format!(
                "{} of {} can join",
                self.joinable_count(),
                self.creature_count()
            )
        }
    }
    pub fn xp_label(&self) -> String {
        if self.xp_unknown.is_empty() {
            format!("XP {}", grouped_number(self.xp))
        } else if self.xp == 0 && self.members.len() == self.xp_unknown.len() {
            "XP not known".to_owned()
        } else {
            format!("XP {} + unknown", grouped_number(self.xp))
        }
    }
    pub fn member_summary(&self) -> String {
        let members: Vec<_> = self
            .members
            .iter()
            .map(|member| {
                let base = Base::of(&member.entry, &self.side);
                let name = member
                    .overrides
                    .resolve(&base)
                    .map_or(base.name, |resolved| resolved.name);
                let name = if member.overrides.name.is_some() {
                    format!("{name} (as {})", member.entry.name)
                } else {
                    name
                };
                format!(
                    "{} × {name}",
                    crate::design::count(member.count as usize, "", "").trim()
                )
            })
            .collect();
        let summary = members.join(", ");
        if self.unavailable > 0 {
            format!(
                "Will add: {}",
                if summary.is_empty() { "none" } else { &summary }
            )
        } else {
            summary
        }
    }

    /// Every creature the note names, whether or not it can be found now.
    pub fn creature_count(&self) -> u64 {
        self.members
            .iter()
            .map(|member| member.count)
            .fold(self.unavailable, u64::saturating_add)
    }
}

pub fn grouped_number(number: impl ToString) -> String {
    let text = number.to_string();
    let digits = text.trim_start_matches('-');
    let mut out = if text.starts_with('-') {
        "−".to_owned()
    } else {
        String::new()
    };
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// Arrivals' heading line of a group, after its name (the name is the heading's own and never
/// part of this line, so no form of the line can paint it twice; SB1): its side, its member
/// count, and its own XP (`XP 150`). The group's own XP shows only where the fight has no form:
/// a heading that says `Fight after: XP 450` never says a second XP number (FJ3).
pub struct FightAfter<'a> {
    pub side: &'a str,
    pub members: &'a str,
    pub worth: &'a str,
}

impl FightAfter<'_> {
    /// The group's own facts in the line's one order: `Enemies · 3 members · XP 150`. The
    /// Encounters list, the plan (`RunGroup::summary`) and Arrivals start a group with them.
    pub fn facts(&self) -> String {
        [self.side, self.members, self.worth]
            .into_iter()
            .filter(|words| !words.is_empty())
            .collect::<Vec<_>>()
            .join(" · ")
    }
}

pub fn change_preview(row: &Row) -> String {
    fn numbers(text: &str) -> String {
        let mut result = String::new();
        let mut number = String::new();
        for c in text.chars().chain(std::iter::once('\0')) {
            if c.is_ascii_digit() || (c == '-' && number.is_empty()) {
                number.push(c);
            } else {
                if !number.is_empty() {
                    result.push_str(&grouped_number(&number));
                    number.clear();
                }
                if c != '\0' {
                    result.push(c);
                }
            }
        }
        result
    }
    format!(
        "{} → {}{}",
        numbers(if row.label == "Speed" {
            row.base.trim().trim_end_matches('.')
        } else {
            &row.base
        }),
        numbers(&row.changed),
        if row.by_hand {
            " · add by hand"
        } else if row.in_rolls {
            " · in Splat’s rolls"
        } else {
            ""
        }
    )
}

/// A UTC date from seconds since 1970, by Howard Hinnant's `civil_from_days`.
pub fn calendar_date(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

/// A member's fault is a typed fault: its tone from its kind (an invalid value and unreadable
/// data are errors, R41; a missing member or one that cannot join is a warning), its words as
/// the loader wrote them (`Unknown creature · line 13: Not in this campaign’s packages.`).
impl crate::design::outcome::Fault for MemberFault {
    fn severity(&self) -> crate::design::outcome::Severity {
        match self.tone() {
            crate::design::Tone::Error => crate::design::outcome::Severity::CannotRead,
            _ => crate::design::outcome::Severity::NeedsFixing,
        }
    }
    #[allow(
        clippy::disallowed_methods,
        reason = "a member fault’s words are the loader’s, with the DM’s names in them"
    )]
    fn cause(&self) -> crate::design::outcome::Sentence {
        crate::design::outcome::Sentence::outside(self.text.as_str())
            .unwrap_or(crate::sentence!("This member cannot join."))
    }
}
