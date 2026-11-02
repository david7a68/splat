//! Compare a D&D Beyond character with what the SRD catalogue says it should have. D&D Beyond stays
//! the source of truth: this reports differences and changes nothing. A class counts only through
//! a source-pinned link, because a name alone does not prove the edition.
use super::build::{self, Build, ChoiceKind, ChoiceSlot, ClassBuild, FeatContext, Selection};
use super::rules::{self, source_id};
use super::{EntitySnapshot, list, string};
use crate::content::character::Sheet;
use crate::content::{Entry, definitions};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// D&D Beyond and the SRD catalogue disagree.
    Differs,
    /// The rules ask for a decision the sheet does not have.
    MissingChoice,
    /// The sheet's decision is one the rules do not allow.
    InvalidChoice,
    /// Splat has no pinned link for this, so it could not compare it.
    NotChecked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Finding {
    pub kind: FindingKind,
    pub message: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Review {
    /// What was compared and agreed, in plain words.
    pub matched: Vec<String>,
    pub findings: Vec<Finding>,
}

impl Review {
    pub fn count(&self, kind: FindingKind) -> usize {
        self.findings.iter().filter(|f| f.kind == kind).count()
    }

    fn add(&mut self, kind: FindingKind, message: String) {
        self.findings.push(Finding { kind, message });
    }

    /// Record one comparison: a match in plain words, or the difference.
    fn compare<T: PartialEq + std::fmt::Debug>(&mut self, what: &str, beyond: T, splat: T) {
        if beyond == splat {
            self.matched.push(what.to_owned());
        } else {
            self.add(
                FindingKind::Differs,
                format!("{what}: D&D Beyond has {beyond:?}; the SRD rules give {splat:?}."),
            );
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Links {
    version: u32,
    classes: BTreeMap<String, ClassLink>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClassLink {
    name: String,
    class: String,
    source_sha256: String,
    features: BTreeMap<String, FeatureLink>,
    skills: BTreeMap<String, OptionLink>,
    weapons: BTreeMap<String, OptionLink>,
    subclasses: BTreeMap<String, OptionLink>,
    feats: BTreeMap<String, OptionLink>,
}

#[derive(Debug, Deserialize)]
struct FeatureLink {
    source_sha256: String,
    /// The level that was reviewed. The source fingerprint does not include it.
    required_level: Option<u64>,
    rule: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionLink {
    label: String,
    value: String,
    source_sha256: String,
}

fn links() -> Result<&'static Links, String> {
    static LINKS: OnceLock<Result<Links, String>> = OnceLock::new();
    LINKS
        .get_or_init(|| {
            let links: Links =
                serde_json::from_str(include_str!("../../builtin/dndbeyond/class-links.json"))
                    .map_err(|e| e.to_string())?;
            if links.version != 1 {
                return Err("Unsupported class link version".into());
            }
            Ok(links)
        })
        .as_ref()
        .map_err(Clone::clone)
}

fn class_fingerprint(definition: &Value) -> String {
    let selected: BTreeMap<_, _> = ["definitionKey", "id", "name", "sources"]
        .into_iter()
        .map(|key| (key, definition[key].clone()))
        .collect();
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&selected).expect("JSON source"))
    )
}

fn whole(value: &Value) -> Option<u32> {
    value.as_u64().map(|n| n as u32)
}

/// A provider class with its link, if the source still matches what was reviewed.
struct Linked<'a> {
    source: &'a Value,
    link: &'a ClassLink,
    features: Vec<ProviderFeature<'a>>,
}

struct ProviderFeature<'a> {
    id: String,
    name: &'a str,
    level: u64,
    /// The reviewed rule, if there is one. Read it only together with `pinned`.
    rule: Option<&'a str>,
    /// The provider's source still matches what was reviewed.
    pinned: bool,
    /// A feat that grants a class feature. It has no level on the class's table.
    from_feat: bool,
    of_subclass: bool,
}

impl<'a> Linked<'a> {
    fn new(source: &'a Value, link: &'a ClassLink, raw: &'a Value) -> Self {
        let subclass_ids: BTreeSet<String> = list(&source["subclassDefinition"]["classFeatures"])
            .iter()
            .map(|feature| source_id(&feature["id"]))
            .collect();
        let describe = |feature: &'a Value, from_feat: bool| {
            let id = source_id(&feature["id"]);
            let found = link.features.get(&id);
            let required_level = feature["requiredLevel"].as_u64();
            ProviderFeature {
                name: string(&feature["name"]),
                // A changed level is reported as a changed source, at the level that was reviewed.
                level: found
                    .and_then(|l| l.required_level)
                    .or(required_level)
                    .unwrap_or(0),
                rule: found.and_then(|l| l.rule.as_deref()),
                pinned: found.is_some_and(|l| {
                    l.source_sha256 == rules::fingerprint(feature)
                        && l.required_level == required_level
                }),
                from_feat,
                of_subclass: subclass_ids.contains(&id),
                id,
            }
        };
        let features = list(&source["classFeatures"])
            .iter()
            .map(|item| describe(&item["definition"], false))
            .chain(
                list(&raw["feats"])
                    .iter()
                    .map(|item| describe(&item["definition"], true)),
            )
            .collect();
        Self {
            source,
            link,
            features,
        }
    }

    fn level(&self) -> u32 {
        whole(&self.source["level"]).unwrap_or(0)
    }

    fn is_starting(&self) -> bool {
        self.source["isStartingClass"] == true
    }

    /// The selected subclass's link, if it has one and its source still matches.
    fn subclass_link(&self) -> Option<&OptionLink> {
        let definition = &self.source["subclassDefinition"];
        self.link
            .subclasses
            .get(&source_id(&definition["id"]))
            .filter(|link| link.source_sha256 == class_fingerprint(definition))
    }

    /// Provider features this class has now. A subclass's features need a linked subclass.
    fn current_features(&self) -> impl Iterator<Item = &ProviderFeature<'_>> {
        let subclass_linked = self.subclass_link().is_some();
        self.features.iter().filter(move |feature| {
            !feature.from_feat
                && feature.level <= u64::from(self.level())
                && (!feature.of_subclass || subclass_linked)
        })
    }

    /// The provider features that prove a rule now. A changed source proves nothing, and neither
    /// does a level this class has not reached. With a level, only that level's feature counts:
    /// one rule can have a separate row for each level.
    fn proven_by(&self, rule: &str, at_level: Option<u32>) -> BTreeSet<&str> {
        self.features
            .iter()
            .filter(|feature| feature.pinned && feature.rule == Some(rule))
            .filter(|feature| feature.level <= u64::from(self.level()))
            .filter(|feature| at_level.is_none_or(|level| feature.level == u64::from(level)))
            .map(|feature| feature.id.as_str())
            .collect()
    }
}

pub fn review(record: &EntitySnapshot, entry: &Entry) -> Result<Review, String> {
    review_with(record, entry, links()?)
}

fn review_with(record: &EntitySnapshot, entry: &Entry, links: &Links) -> Result<Review, String> {
    let sheet: Sheet = entry
        .fields
        .get("splat_character")
        .cloned()
        .map(serde_yaml_ng::from_value)
        .transpose()
        .map_err(|e| e.to_string())?
        .ok_or("Imported character has no sheet")?;
    let mut review = Review::default();
    let raw = &record.source.raw;
    let mut linked = Vec::new();
    let mut other_levels = 0;
    for class in list(&raw["classes"]) {
        let definition = &class["definition"];
        let found = links
            .classes
            .get(&source_id(&definition["id"]))
            .filter(|link| link.source_sha256 == class_fingerprint(definition));
        match found {
            Some(link) => linked.push(Linked::new(class, link, raw)),
            None => {
                other_levels += whole(&class["level"]).unwrap_or(0);
                review.add(
                    FindingKind::NotChecked,
                    format!(
                        "{}: no reviewed link to an SRD class, so its class rules were not compared.",
                        string(&definition["name"])
                    ),
                );
            }
        }
    }
    let selections: Vec<Selection> = linked
        .iter()
        .map(|class| Selection {
            class: &class.link.class,
            level: class.level(),
            previous_level: 0,
            subclass: class.subclass_link().map(|option| option.value.as_str()),
            starting: class.is_starting(),
        })
        .collect();
    let built = build::build(&selections, other_levels)?;
    let computed = &record.source.computed;
    review.compare(
        "Character level",
        whole(&computed["level"]),
        Some(built.level),
    );
    review.compare(
        "Proficiency bonus",
        whole(&computed["proficiencyBonus"]),
        Some(built.proficiency_bonus),
    );
    compare_spell_slots(record, &built, &mut review);
    for (class, class_build) in linked.iter().zip(&built.classes) {
        compare_class(record, &sheet, class, class_build, &built, &mut review)?;
    }
    Ok(review)
}

/// The imported pools of one kind, as spell level and count. Only a single class has pools.
fn pools(record: &EntitySnapshot, prefix: &str) -> BTreeMap<i64, i64> {
    record
        .definition
        .splat_resources
        .iter()
        .filter(|(key, _)| key.starts_with(prefix))
        .filter_map(|(_, pool)| Some((pool.level?, pool.max)))
        .collect()
}

fn compare_spell_slots(record: &EntitySnapshot, built: &Build, review: &mut Review) {
    let pact = built
        .classes
        .iter()
        .find_map(|class| class.pact_slots.as_ref());
    let casts = built
        .spell_slots
        .is_some_and(|slots| slots.iter().any(|count| *count > 0));
    if list(&record.source.raw["classes"]).len() > 1 {
        if casts || pact.is_some() {
            review.add(
                FindingKind::NotChecked,
                "Spell slots: Splat does not import slots for a multiclass character, so they were not compared.".into(),
            );
        }
        return;
    }
    if let Some(slots) = built.spell_slots {
        let expected: BTreeMap<i64, i64> = (1..)
            .zip(slots)
            .filter(|(_, count)| *count > 0)
            .map(|(level, count)| (level, i64::from(count)))
            .collect();
        review.compare("Spell slots", pools(record, "spell_slots_"), expected);
    }
    if let Some(pact) = pact {
        let expected: BTreeMap<i64, i64> = (pact.count > 0)
            .then_some((i64::from(pact.spell_level), i64::from(pact.count)))
            .into_iter()
            .collect();
        review.compare("Pact slots", pools(record, "pact_slots_"), expected);
    }
}

fn compare_class(
    record: &EntitySnapshot,
    sheet: &Sheet,
    class: &Linked,
    built: &ClassBuild,
    all: &Build,
    review: &mut Review,
) -> Result<(), String> {
    let name = &class.link.name;
    let definition = &class.source["definition"];
    for problem in &built.problems {
        review.add(FindingKind::InvalidChoice, problem.clone());
    }
    review.compare(
        &format!("{name} hit die"),
        whole(&definition["hitDice"]),
        Some(built.hit_die_sides),
    );
    if let Some(traits) = built.traits {
        compare_saves(record, class, built.key.as_str(), &traits.saves, review);
    }
    let spell_rules = &definition["spellRules"];
    let level = built.level as usize;
    for (capacity, column) in [
        ("cantrips", "levelCantripsKnownMaxes"),
        ("prepared_spells", "levelPreparedSpellMaxes"),
    ] {
        if let Some(expected) = built.selection_capacities.get(capacity) {
            let expected = expected.count;
            review.compare(
                &format!("{name} {}", capacity.replace('_', " ")),
                spell_rules[column].get(level).and_then(whole),
                Some(expected),
            );
        }
    }
    compare_features(class, built, review);
    compare_resources(record, sheet, class, built, review)?;
    let feats = feat_context(record, all);
    let choices = Choices {
        class,
        rows: choice_rows(record),
        offered: offered_options(record),
    };
    for slot in &merged_slots(&built.choices) {
        match slot.kind {
            ChoiceKind::Subclass => check_subclass(class, slot, review),
            ChoiceKind::Skill => {
                choices.check(slot, &class.link.skills, "skill", review, |_| Ok(true))?
            }
            ChoiceKind::WeaponMastery => choices.check(
                slot,
                &class.link.weapons,
                "Weapon Mastery weapon",
                review,
                |weapon| build::mastery_allows(&slot.source, weapon),
            )?,
            ChoiceKind::Feat => choices.check(slot, &class.link.feats, "feat", review, |feat| {
                Ok(build::feat_allowed(feat, &feats).is_ok())
            })?,
        }
    }
    Ok(())
}

/// A sheet records Weapon Mastery picks together, so its per-level slots count as one choice.
fn merged_slots(slots: &[ChoiceSlot]) -> Vec<ChoiceSlot> {
    let mut merged: Vec<ChoiceSlot> = Vec::new();
    for slot in slots {
        match merged
            .iter_mut()
            .find(|m| slot.kind == ChoiceKind::WeaponMastery && m.kind == slot.kind)
        {
            Some(first) => first.count += slot.count,
            None => merged.push(slot.clone()),
        }
    }
    merged
}

fn compare_saves(
    record: &EntitySnapshot,
    class: &Linked,
    class_key: &str,
    saves: &[String],
    review: &mut Review,
) {
    let core = class.proven_by(class_key, None);
    let what = format!("{} saving throws", class.link.name);
    if core.is_empty() {
        review.add(
            FindingKind::NotChecked,
            format!("{what}: the class traits have no reviewed source, so they were not compared."),
        );
        return;
    }
    let beyond: BTreeSet<&str> = list(&record.source.raw["modifiers"]["class"])
        .iter()
        .filter(|modifier| {
            string(&modifier["type"]) == "proficiency"
                && core.contains(source_id(&modifier["componentId"]).as_str())
        })
        .filter_map(|modifier| string(&modifier["subType"]).strip_suffix("-saving-throws"))
        .collect();
    let expected: BTreeSet<&str> = saves.iter().map(String::as_str).collect();
    review.compare(&what, beyond, expected);
}

fn compare_features(class: &Linked, built: &ClassBuild, review: &mut Review) {
    let name = &class.link.name;
    // A changed source is reported below as not compared, not as a missing feature.
    let provider: BTreeSet<&str> = class
        .current_features()
        .filter_map(|feature| feature.rule)
        .collect();
    let granted: BTreeSet<&str> = built.features.iter().map(|f| f.key.as_str()).collect();
    let findings_before = review.findings.len();
    for grant in &built.features {
        if !provider.contains(grant.key.as_str()) {
            review.add(
                FindingKind::Differs,
                format!(
                    "{name}: the SRD rules grant {} at level {}. D&D Beyond does not show it.",
                    feature_name(&grant.key),
                    grant.at_level
                ),
            );
        }
    }
    for feature in class.current_features() {
        match feature.rule {
            None => review.add(
                FindingKind::NotChecked,
                format!(
                    "{name}: {} has no reviewed link to an SRD rule, so it was not compared.",
                    feature.name
                ),
            ),
            Some(_) if !feature.pinned => review.add(
                FindingKind::NotChecked,
                format!(
                    "{name}: {} changed on D&D Beyond since it was reviewed, so it was not compared.",
                    feature.name
                ),
            ),
            // A class's core traits link to the class itself, which is not a granted feature.
            Some(rule) if rule != built.key && !granted.contains(rule) => review.add(
                FindingKind::Differs,
                format!(
                    "{name}: D&D Beyond grants {} ({rule}) by level {}. The SRD rules do not.",
                    feature.name, built.level
                ),
            ),
            Some(_) => {}
        }
    }
    if review.findings.len() == findings_before {
        review.matched.push(format!("{name} features"));
    }
}

fn catalogue_entry(key: &str) -> Result<Option<&'static Entry>, String> {
    Ok(definitions::entries()?
        .iter()
        .find(|entry| entry.key.as_deref() == Some(key)))
}

fn feature_name(key: &str) -> String {
    catalogue_entry(key)
        .ok()
        .flatten()
        .map_or_else(|| key.to_owned(), |entry| entry.name.clone())
}

/// Compare a feature's uses, such as Rage's, with the imported pool it is bound to.
fn compare_resources(
    record: &EntitySnapshot,
    sheet: &Sheet,
    class: &Linked,
    built: &ClassBuild,
    review: &mut Review,
) -> Result<(), String> {
    for grant in &built.features {
        let Some(column) = build::resource_column(&built.class, &grant.key)? else {
            continue;
        };
        let Some(capacity) = built.resource_capacities.get(&column) else {
            continue;
        };
        let Some(entry) = catalogue_entry(&grant.key)? else {
            continue;
        };
        let pool = sheet
            .resource_bindings
            .values()
            .find(|binding| binding.rule.id == entry.id)
            .and_then(|binding| record.definition.splat_resources.get(&binding.pool));
        if let Some(pool) = pool {
            review.compare(
                &format!("{} {}", class.link.name, column.replace('_', " ")),
                pool.max,
                i64::from(capacity.count),
            );
        }
    }
    Ok(())
}

fn check_subclass(class: &Linked, slot: &ChoiceSlot, review: &mut Review) {
    let name = &class.link.name;
    let definition = &class.source["subclassDefinition"];
    if definition["id"].is_null() {
        review.add(
            FindingKind::MissingChoice,
            format!("{name} needs a subclass at level {}.", slot.at_level),
        );
        return;
    }
    let Some(option) = class.subclass_link() else {
        review.add(
            FindingKind::NotChecked,
            format!(
                "{name}: subclass {} has no reviewed link, so its features were not compared.",
                string(&definition["name"])
            ),
        );
        return;
    };
    if slot.allowed.permits(&option.value) {
        review.matched.push(format!("{name} subclass"));
    } else {
        review.add(
            FindingKind::InvalidChoice,
            format!("{name}: {} is not an allowed subclass.", option.label),
        );
    }
}

/// A character's scores and feature kinds, for feat prerequisites.
fn feat_context(record: &EntitySnapshot, all: &Build) -> FeatContext {
    FeatContext {
        level: all.level,
        scores: list(&record.source.computed["abilities"])
            .iter()
            .filter_map(|ability| {
                Some((
                    string(&ability["label"]).to_lowercase(),
                    ability["score"].as_i64()?,
                ))
            })
            .collect(),
        features: all
            .classes
            .iter()
            .flat_map(|class| &class.features)
            .filter_map(|feature| {
                ["spellcasting", "fighting_style"]
                    .into_iter()
                    .find(|kind| feature.key.ends_with(kind))
            })
            .collect(),
    }
}

struct OfferedOption<'a> {
    fingerprint: String,
    label: &'a str,
}

/// Every option D&D Beyond offers on this sheet, by its definition and ID, so a link can prove its
/// source has not changed. The same ID in another definition proves nothing.
fn offered_options(record: &EntitySnapshot) -> BTreeMap<(String, String), OfferedOption<'_>> {
    list(&record.source.raw["choices"]["choiceDefinitions"])
        .iter()
        .flat_map(|definition| {
            let scope = string(&definition["id"]);
            list(&definition["options"]).iter().map(move |option| {
                (
                    (scope.to_owned(), source_id(&option["id"])),
                    OfferedOption {
                        fingerprint: rules::option_fingerprint(option),
                        label: string(&option["label"]),
                    },
                )
            })
        })
        .collect()
}

/// One decision on the sheet: the feature that asks, what it offers and what was picked.
struct ChoiceRow {
    /// The choice definition that holds this row's options.
    scope: String,
    component: String,
    selected: Option<String>,
    offers: BTreeSet<String>,
}

fn choice_rows(record: &EntitySnapshot) -> Vec<ChoiceRow> {
    record.source.raw["choices"]
        .as_object()
        .into_iter()
        .flat_map(|groups| groups.values())
        .flat_map(list)
        .filter(|row| !row["componentId"].is_null())
        .map(|row| ChoiceRow {
            scope: format!("{}-{}", row["componentTypeId"], row["type"]),
            component: source_id(&row["componentId"]),
            selected: (!row["optionValue"].is_null()).then(|| source_id(&row["optionValue"])),
            offers: list(&row["optionIds"]).iter().map(source_id).collect(),
        })
        .collect()
}

struct Choices<'a> {
    class: &'a Linked<'a>,
    rows: Vec<ChoiceRow>,
    offered: BTreeMap<(String, String), OfferedOption<'a>>,
}

impl Choices<'_> {
    /// A choice answered by rows on the sheet: count the answers, then check each linked one.
    /// A feature can hold rows for other things, such as tools beside skills, so a row counts only
    /// if it offers an option this choice knows. Feat links are not known ahead of time, so a feat
    /// choice takes every row of its level's feature.
    fn check(
        &self,
        slot: &ChoiceSlot,
        options: &BTreeMap<String, OptionLink>,
        what: &str,
        review: &mut Review,
        accepted: impl Fn(&str) -> Result<bool, String>,
    ) -> Result<(), String> {
        let name = &self.class.link.name;
        let is_feat = slot.kind == ChoiceKind::Feat;
        let components = self
            .class
            .proven_by(&slot.source, is_feat.then_some(slot.at_level));
        let rows: Vec<&ChoiceRow> = self
            .rows
            .iter()
            .filter(|row| components.contains(row.component.as_str()))
            .filter(|row| is_feat || row.offers.iter().any(|id| options.contains_key(id)))
            .collect();
        if components.is_empty() || (is_feat && rows.is_empty()) {
            review.add(
                FindingKind::NotChecked,
                format!(
                    "{name}: the {what} choice for level {} has no reviewed source on D&D Beyond, so it was not compared.",
                    slot.at_level
                ),
            );
            return Ok(());
        }
        let answers: Vec<(&ChoiceRow, &str)> = rows
            .iter()
            .filter_map(|row| Some((*row, row.selected.as_deref()?)))
            .collect();
        let mut valid = true;
        let mut seen = BTreeSet::new();
        for (row, id) in answers.iter().copied() {
            let offered = self.offered.get(&(row.scope.clone(), id.to_owned()));
            let label = offered.map_or(id, |o| o.label);
            if !row.offers.is_empty() && !row.offers.contains(id) {
                valid = false;
                review.add(
                    FindingKind::InvalidChoice,
                    format!("{name}: {label} is not an option this {what} choice offers."),
                );
                continue;
            }
            let link = options.get(id).filter(|link| {
                offered.map(|o| o.fingerprint.as_str()) == Some(link.source_sha256.as_str())
            });
            let Some(link) = link else {
                valid = false;
                review.add(
                    FindingKind::NotChecked,
                    format!("{name}: {what} {label} has no reviewed link, so it was not compared."),
                );
                continue;
            };
            if !slot.allowed.permits(&link.value) || !accepted(&link.value)? {
                valid = false;
                review.add(
                    FindingKind::InvalidChoice,
                    format!("{name}: {} is not an allowed {what}.", link.label),
                );
            }
            // Each feat answers its own level. The same skill or weapon twice gives nothing extra.
            if !is_feat && !seen.insert(link.value.as_str()) {
                valid = false;
                review.add(
                    FindingKind::InvalidChoice,
                    format!("{name}: {} is chosen twice as a {what}.", link.label),
                );
            }
        }
        let chosen = answers.len() as u32;
        if chosen > slot.count {
            review.add(
                FindingKind::Differs,
                format!(
                    "{name}: D&D Beyond has {chosen} {what} choices. The SRD rules allow {}.",
                    slot.count
                ),
            );
        } else if chosen < slot.count {
            review.add(
                FindingKind::MissingChoice,
                format!(
                    "{name} needs {} {what} choices; {chosen} are set.",
                    slot.count
                ),
            );
        } else if valid {
            review.matched.push(format!("{name} {what} choices"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const CLASS_ID: u64 = 99;
    const SKILLS: &[(u64, &str)] = &[(61, "Athletics"), (62, "Nature"), (63, "Perception")];
    const WEAPONS: &[(u64, &str, &str)] = &[
        (71, "Greataxe (Cleave)", "dnd2024:equipment/greataxe"),
        (72, "Handaxe (Vex)", "dnd2024:equipment/handaxe"),
        (73, "Longbow (Slow)", "dnd2024:equipment/longbow"),
    ];

    const FEATS: &[(u64, &str, &str)] = &[
        (
            81,
            "Ability Score Improvement",
            "dnd2024:feats/general_feats_ability_score_improvement",
        ),
        (82, "Grappler", "dnd2024:feats/general_feats_grappler"),
    ];

    fn feature(id: u64, name: &str, level: u64) -> Value {
        json!({"definition": {"id": id, "name": name, "requiredLevel": level, "description": name}})
    }

    fn option(id: u64, label: &str) -> Value {
        json!({"id": id, "label": label, "description": null, "sourceId": null})
    }

    fn class_source(level: u64) -> Value {
        json!({
            "id": 8,
            "level": level,
            "hitDiceUsed": 0,
            "isStartingClass": true,
            "definition": {"id": CLASS_ID, "definitionKey": "class:99", "name": "Barbarian",
                "sources": [], "hitDice": 12, "spellRules": {"levelSpellSlots": []}},
            "classFeatures": [
                feature(1, "Core Barbarian Traits", 1),
                feature(2, "Rage", 1),
                feature(3, "Unarmored Defense", 1),
                feature(4, "Weapon Mastery", 1),
                feature(5, "Danger Sense", 2),
                feature(6, "Reckless Attack", 2),
                feature(7, "Barbarian Subclass", 3),
                feature(8, "Ability Score Improvement", 4),
                feature(9, "8: Ability Score Improvement", 8),
            ],
        })
    }

    fn choice(component: u64, id: &str, value: Option<u64>, offers: &[u64]) -> Value {
        json!({"id": id, "componentId": component, "componentTypeId": 5, "type": 2,
            "optionValue": value, "optionIds": offers, "label": "Choose", "isOptional": false})
    }

    /// A Barbarian on a synthetic sheet: `skills` and `masteries` hold the selected option IDs.
    fn record(level: u64, skills: &[u64], masteries: &[u64]) -> EntitySnapshot {
        let mut record = crate::character::tests::sample();
        record.source.raw["classes"] = json!([class_source(level)]);
        record.source.computed["level"] = json!(level);
        record.source.computed["proficiencyBonus"] = json!(2);
        record.source.raw["modifiers"] = json!({"class": [
            {"type": "proficiency", "subType": "strength-saving-throws", "componentId": 1},
            {"type": "proficiency", "subType": "constitution-saving-throws", "componentId": 1},
        ]});
        let mut options: Vec<Value> = SKILLS.iter().map(|(id, l)| option(*id, l)).collect();
        options.extend(WEAPONS.iter().map(|(id, l, _)| option(*id, l)));
        options.extend(FEATS.iter().map(|(id, l, _)| option(*id, l)));
        let skill_ids: Vec<u64> = SKILLS.iter().map(|(id, _)| *id).collect();
        let weapon_ids: Vec<u64> = WEAPONS.iter().map(|(id, ..)| *id).collect();
        let mut class_choices: Vec<Value> = skills
            .iter()
            .enumerate()
            .map(|(i, id)| choice(1, &format!("skill-{i}"), Some(*id), &skill_ids))
            .collect();
        class_choices.extend(
            masteries
                .iter()
                .enumerate()
                .map(|(i, id)| choice(4, &format!("mastery-{i}"), Some(*id), &weapon_ids)),
        );
        record.source.raw["choices"] = json!({
            "class": class_choices,
            "choiceDefinitions": [{"id": "5-2", "options": options}],
        });
        record.definition.splat_resources.clear();
        record
    }

    fn links(record: &EntitySnapshot) -> Links {
        let class = &record.source.raw["classes"][0];
        let features = list(&class["classFeatures"])
            .iter()
            .map(|item| {
                let feature = &item["definition"];
                let id = feature["id"].as_u64().unwrap();
                let rule = match id {
                    1 => "dnd2024:classes/barbarian",
                    2 => "dnd2024:class_features/barbarian_level_1_rage",
                    3 => "dnd2024:class_features/barbarian_level_1_unarmored_defense",
                    4 => "dnd2024:class_features/barbarian_level_1_weapon_mastery",
                    5 => "dnd2024:class_features/barbarian_level_2_danger_sense",
                    6 => "dnd2024:class_features/barbarian_level_2_reckless_attack",
                    8 | 9 => "dnd2024:class_features/barbarian_level_4_ability_score_improvement",
                    _ => "dnd2024:class_features/barbarian_level_3_barbarian_subclass",
                };
                (
                    id.to_string(),
                    FeatureLink {
                        source_sha256: rules::fingerprint(feature),
                        required_level: feature["requiredLevel"].as_u64(),
                        rule: Some(rule.to_owned()),
                    },
                )
            })
            .collect();
        let link = |id: u64, label: &str, value: &str| {
            (
                id.to_string(),
                OptionLink {
                    label: label.to_owned(),
                    value: value.to_owned(),
                    source_sha256: rules::option_fingerprint(&option(id, label)),
                },
            )
        };
        Links {
            version: 1,
            classes: BTreeMap::from([(
                CLASS_ID.to_string(),
                ClassLink {
                    name: "Barbarian".into(),
                    class: "barbarian".into(),
                    source_sha256: class_fingerprint(&class["definition"]),
                    features,
                    skills: SKILLS
                        .iter()
                        .map(|(id, l)| link(*id, l, &l.to_lowercase()))
                        .collect(),
                    weapons: WEAPONS.iter().map(|(id, l, v)| link(*id, l, v)).collect(),
                    subclasses: BTreeMap::new(),
                    feats: FEATS.iter().map(|(id, l, v)| link(*id, l, v)).collect(),
                },
            )]),
        }
    }

    fn run(record: &EntitySnapshot) -> Review {
        run_with(record, &links(record))
    }

    fn run_with(record: &EntitySnapshot, links: &Links) -> Review {
        let entry = record.entry().unwrap();
        review_with(record, &entry, links).unwrap()
    }

    fn messages(review: &Review, kind: FindingKind) -> Vec<&str> {
        review
            .findings
            .iter()
            .filter(|f| f.kind == kind)
            .map(|f| f.message.as_str())
            .collect()
    }

    #[test]
    fn a_correct_sheet_matches_and_has_no_findings() {
        let review = run(&record(1, &[61, 62], &[71, 72]));
        assert_eq!(review.findings, vec![]);
        for what in [
            "Character level",
            "Proficiency bonus",
            "Barbarian hit die",
            "Barbarian saving throws",
            "Barbarian features",
            "Barbarian skill choices",
            "Barbarian Weapon Mastery weapon choices",
        ] {
            assert!(review.matched.iter().any(|m| m == what), "{what}");
        }
    }

    #[test]
    fn a_wrong_total_or_die_is_a_difference() {
        let mut wrong = record(1, &[61, 62], &[71, 72]);
        wrong.source.computed["proficiencyBonus"] = json!(3);
        wrong.source.raw["classes"][0]["definition"]["hitDice"] = json!(10);
        // The hit die is not part of the class pin, so the link still holds.
        let review = run(&wrong);
        let differs = messages(&review, FindingKind::Differs);
        assert_eq!(differs.len(), 2, "{differs:?}");
        assert!(differs[0].contains("Proficiency bonus"));
        assert!(differs[1].contains("hit die"));
    }

    #[test]
    fn unset_and_excess_choices_are_reported() {
        let review = run(&record(1, &[61], &[71, 72]));
        assert_eq!(
            messages(&review, FindingKind::MissingChoice),
            vec!["Barbarian needs 2 skill choices; 1 are set."]
        );
        let review = run(&record(1, &[61, 62, 63], &[71, 72]));
        assert_eq!(
            messages(&review, FindingKind::Differs),
            vec!["Barbarian: D&D Beyond has 3 skill choices. The SRD rules allow 2."]
        );
    }

    #[test]
    fn a_choice_the_rule_forbids_is_invalid() {
        // Barbarian Weapon Mastery accepts melee weapons, so a Longbow is not allowed.
        let review = run(&record(1, &[61, 62], &[71, 73]));
        assert_eq!(
            messages(&review, FindingKind::InvalidChoice),
            vec!["Barbarian: Longbow (Slow) is not an allowed Weapon Mastery weapon."]
        );
    }

    #[test]
    fn a_changed_option_or_feature_is_not_compared() {
        let source = record(1, &[61, 62], &[71, 72]);
        let links = links(&source);
        let mut changed = source.clone();
        changed.source.raw["choices"]["choiceDefinitions"][0]["options"][0]["description"] =
            json!("Changed");
        changed.source.raw["classes"][0]["classFeatures"][1]["definition"]["description"] =
            json!("Changed");
        let review = run_with(&changed, &links);
        let unchecked = messages(&review, FindingKind::NotChecked);
        assert_eq!(unchecked.len(), 2, "{unchecked:?}");
        assert!(unchecked.iter().any(|m| m.contains("skill Athletics")));
        assert!(unchecked.iter().any(|m| m.contains("Rage changed")));
        // D&D Beyond still shows Rage, so the changed text is not a difference.
        assert_eq!(messages(&review, FindingKind::Differs), Vec::<&str>::new());
        assert!(!review.matched.contains(&"Barbarian features".to_owned()));
    }

    #[test]
    fn the_same_option_id_in_another_definition_proves_nothing() {
        let source = record(1, &[61, 62], &[71, 72]);
        let links = links(&source);
        let mut changed = source.clone();
        let definitions = &mut changed.source.raw["choices"]["choiceDefinitions"];
        definitions[0]["options"][0]["description"] = json!("Changed");
        definitions
            .as_array_mut()
            .unwrap()
            .push(json!({"id": "6-2", "options": [option(61, "Athletics")]}));
        let review = run_with(&changed, &links);
        assert!(
            messages(&review, FindingKind::NotChecked)
                .iter()
                .any(|m| m.contains("skill Athletics"))
        );
        assert!(
            !review
                .matched
                .contains(&"Barbarian skill choices".to_owned())
        );
    }

    #[test]
    fn a_changed_level_is_a_changed_source() {
        let source = record(1, &[61, 62], &[71, 72]);
        let links = links(&source);
        let mut changed = source.clone();
        changed.source.raw["classes"][0]["classFeatures"][3]["definition"]["requiredLevel"] =
            json!(10);
        let review = run_with(&changed, &links);
        let unchecked = messages(&review, FindingKind::NotChecked);
        assert!(
            unchecked
                .iter()
                .any(|m| m.contains("Weapon Mastery changed")),
            "{unchecked:?}"
        );
        assert_eq!(messages(&review, FindingKind::Differs), Vec::<&str>::new());
        assert!(!review.matched.iter().any(|m| m.contains("Weapon Mastery")));
    }

    #[test]
    fn a_choice_kept_for_a_later_level_does_not_count_yet() {
        let mut source = record(1, &[61, 62], &[71, 72]);
        source.source.raw["classes"][0]["classFeatures"]
            .as_array_mut()
            .unwrap()
            .push(feature(10, "4: Weapon Mastery", 4));
        let mut links = links(&source);
        let class = links.classes.get_mut(&CLASS_ID.to_string()).unwrap();
        class.features.get_mut("10").unwrap().rule =
            Some("dnd2024:class_features/barbarian_level_1_weapon_mastery".into());
        source.source.raw["choices"]["class"]
            .as_array_mut()
            .unwrap()
            .push(choice(10, "later", Some(73), &[71, 72, 73]));
        let review = run_with(&source, &links);
        assert_eq!(review.findings, vec![]);
    }

    #[test]
    fn a_pick_the_row_does_not_offer_or_a_repeat_is_invalid() {
        let mut offered_elsewhere = record(1, &[61, 62], &[71, 72]);
        offered_elsewhere.source.raw["choices"]["class"][0]["optionIds"] = json!([62]);
        let review = run(&offered_elsewhere);
        assert_eq!(
            messages(&review, FindingKind::InvalidChoice),
            vec!["Barbarian: Athletics is not an option this skill choice offers."]
        );
        assert!(
            !review
                .matched
                .contains(&"Barbarian skill choices".to_owned())
        );
        let review = run(&record(1, &[61, 61], &[71, 72]));
        assert_eq!(
            messages(&review, FindingKind::InvalidChoice),
            vec!["Barbarian: Athletics is chosen twice as a skill."]
        );
        assert!(
            !review
                .matched
                .contains(&"Barbarian skill choices".to_owned())
        );
    }

    #[test]
    fn weapon_mastery_slots_from_several_levels_count_as_one_choice() {
        let source = record(10, &[61, 62], &[71, 72]);
        let review = run(&source);
        assert!(
            messages(&review, FindingKind::MissingChoice)
                .contains(&"Barbarian needs 4 Weapon Mastery weapon choices; 2 are set."),
            "{review:?}"
        );
    }

    #[test]
    fn changed_class_traits_do_not_prove_saves_or_skills() {
        let source = record(1, &[61, 62], &[71, 72]);
        let links = links(&source);
        let mut changed = source.clone();
        changed.source.raw["classes"][0]["classFeatures"][0]["definition"]["description"] =
            json!("Changed");
        let review = run_with(&changed, &links);
        let unchecked = messages(&review, FindingKind::NotChecked);
        assert!(unchecked.iter().any(|m| m.contains("saving throws")));
        assert!(unchecked.iter().any(|m| m.contains("skill choice")));
        assert_eq!(
            messages(&review, FindingKind::MissingChoice),
            Vec::<&str>::new()
        );
        assert!(!review.matched.iter().any(|m| m.contains("saving throws")));
        assert!(!review.matched.iter().any(|m| m.contains("skill choices")));
    }

    #[test]
    fn a_tool_row_on_the_same_feature_is_not_a_skill_answer() {
        let mut source = record(1, &[61, 62], &[71, 72]);
        source.source.raw["choices"]["class"]
            .as_array_mut()
            .unwrap()
            .push(choice(1, "tool", Some(99), &[99]));
        let review = run(&source);
        assert_eq!(review.findings, vec![]);
    }

    /// Set each listed Ability Score Improvement level's feature to pick a feat.
    fn with_feats(mut source: EntitySnapshot, picks: &[(u64, u64)]) -> EntitySnapshot {
        let rows = source.source.raw["choices"]["class"]
            .as_array_mut()
            .unwrap();
        for (component, feat) in picks {
            rows.push(choice(
                *component,
                &format!("feat-{component}"),
                Some(*feat),
                &[],
            ));
        }
        source
    }

    #[test]
    fn each_feat_level_counts_its_own_answer() {
        let feat = "feat choices";
        let both = run(&with_feats(
            record(8, &[61, 62], &[71, 72]),
            &[(8, 81), (9, 81)],
        ));
        assert!(
            !messages(&both, FindingKind::MissingChoice)
                .iter()
                .any(|m| m.contains("feat")),
            "{both:?}"
        );
        assert!(
            !messages(&both, FindingKind::Differs)
                .iter()
                .any(|m| m.contains(feat))
        );
        assert_eq!(both.matched.iter().filter(|m| m.ends_with(feat)).count(), 2);
        // A row at level 8 that is unset is missing, and level 4's answer does not fill it.
        let mut one = with_feats(record(8, &[61, 62], &[71, 72]), &[(8, 81)]);
        one.source.raw["choices"]["class"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id": "feat-9", "componentId": 9, "optionValue": null, "optionIds": []}));
        let one = run(&one);
        assert!(
            messages(&one, FindingKind::MissingChoice)
                .iter()
                .any(|m| m.contains("needs 1 feat choices; 0 are set"))
        );
    }

    #[test]
    fn feat_prerequisites_read_the_ability_label() {
        let scored = |strength: i64| {
            let mut source = with_feats(record(4, &[61, 62], &[71, 72]), &[(8, 82)]);
            source.source.computed["abilities"] = json!([
                {"name": "str", "label": "Strength", "score": strength},
                {"name": "dex", "label": "Dexterity", "score": 10},
            ]);
            run(&source)
        };
        let strong = scored(13);
        assert!(strong.matched.iter().any(|m| m.ends_with("feat choices")));
        let weak = scored(12);
        assert_eq!(
            messages(&weak, FindingKind::InvalidChoice),
            vec!["Barbarian: Grappler is not an allowed feat."]
        );
    }

    #[test]
    fn a_subclass_needs_its_own_pin_and_brings_its_features() {
        let berserker = "dnd2024:subclasses/barbarian_path_of_the_berserker";
        let mut source = record(3, &[61, 62], &[71, 72]);
        let definition = json!({"id": 200, "definitionKey": "sub:200", "name": "Berserker",
            "sources": [], "classFeatures": [{"id": 201}]});
        source.source.raw["classes"][0]["subclassDefinition"] = definition.clone();
        source.source.raw["classes"][0]["classFeatures"]
            .as_array_mut()
            .unwrap()
            .push(feature(201, "Frenzy", 3));
        let mut pinned = links(&source);
        let class = pinned.classes.get_mut(&CLASS_ID.to_string()).unwrap();
        class.subclasses.insert(
            "200".into(),
            OptionLink {
                label: "Berserker".into(),
                value: berserker.into(),
                source_sha256: class_fingerprint(&definition),
            },
        );
        let rule = "dnd2024:class_features/path_of_the_berserker_level_3_frenzy";
        class.features.get_mut("201").unwrap().rule = Some(rule.into());
        let review = run_with(&source, &pinned);
        assert!(review.matched.contains(&"Barbarian subclass".to_owned()));
        // Frenzy is shown, so only the table's Primal Knowledge is missing from this short sheet.
        assert!(
            !messages(&review, FindingKind::Differs)
                .iter()
                .any(|m| m.contains("Frenzy"))
        );
        source.source.raw["classes"][0]["subclassDefinition"]["name"] = json!("Changed");
        let changed = run_with(&source, &pinned);
        assert!(!changed.matched.contains(&"Barbarian subclass".to_owned()));
        assert!(
            messages(&changed, FindingKind::NotChecked)
                .iter()
                .any(|m| m.contains("subclass Changed"))
        );
    }

    fn built(selections: &[Selection]) -> Build {
        build::build(selections, 0).unwrap()
    }

    fn class(class: &str, level: u32) -> Selection<'_> {
        Selection {
            class,
            level,
            previous_level: 0,
            subclass: None,
            starting: true,
        }
    }

    #[test]
    fn multiclass_slots_are_not_compared_because_they_are_not_imported() {
        let mut source = record(5, &[], &[]);
        let second = class_source(2);
        source.source.raw["classes"]
            .as_array_mut()
            .unwrap()
            .push(second);
        let mut review = Review::default();
        compare_spell_slots(
            &source,
            &built(&[class("wizard", 3), class("cleric", 2)]),
            &mut review,
        );
        assert_eq!(review.matched, Vec::<String>::new());
        assert_eq!(review.findings.len(), 1);
        assert_eq!(review.findings[0].kind, FindingKind::NotChecked);
        // Two non-casters have no slots to compare.
        let mut review = Review::default();
        compare_spell_slots(
            &source,
            &built(&[class("barbarian", 3), class("fighter", 2)]),
            &mut review,
        );
        assert_eq!(review.findings, vec![]);
    }

    #[test]
    fn a_warlock_compares_pact_slots_and_has_no_spell_slots() {
        let pool = |level: i64, max: i64| crate::execution::model::Pool {
            name: "Pact".into(),
            max,
            initial: None,
            kind: Some("spell_slot".into()),
            level: Some(level),
        };
        let mut source = record(5, &[], &[]);
        source
            .definition
            .splat_resources
            .insert("pact_slots_3".into(), pool(3, 2));
        let mut review = Review::default();
        compare_spell_slots(&source, &built(&[class("warlock", 5)]), &mut review);
        assert_eq!(review.findings, vec![]);
        assert!(review.matched.contains(&"Pact slots".to_owned()));
        assert!(review.matched.contains(&"Spell slots".to_owned()));
        source
            .definition
            .splat_resources
            .insert("pact_slots_3".into(), pool(3, 1));
        let mut review = Review::default();
        compare_spell_slots(&source, &built(&[class("warlock", 5)]), &mut review);
        assert_eq!(review.count(FindingKind::Differs), 1);
    }

    #[test]
    fn an_unlinked_class_is_not_compared_and_keeps_its_levels() {
        let source = record(1, &[61, 62], &[71, 72]);
        let empty = Links {
            version: 1,
            classes: BTreeMap::new(),
        };
        let review = run_with(&source, &empty);
        assert_eq!(
            messages(&review, FindingKind::NotChecked),
            vec![
                "Barbarian: no reviewed link to an SRD class, so its class rules were not compared."
            ]
        );
        assert!(review.matched.contains(&"Character level".to_owned()));
    }

    #[test]
    fn a_missing_subclass_is_reported_at_level_three() {
        let review = run(&record(3, &[61, 62], &[71, 72]));
        assert_eq!(
            messages(&review, FindingKind::MissingChoice),
            vec!["Barbarian needs a subclass at level 3."]
        );
        // Primal Knowledge is granted by the table and is not on this short provider sheet.
        assert!(
            messages(&review, FindingKind::Differs)
                .iter()
                .any(|m| m.contains("Primal Knowledge"))
        );
    }

    #[test]
    fn every_shipped_link_points_at_its_own_class_in_the_catalogue() {
        let links = super::links().unwrap();
        let entries = definitions::entries().unwrap();
        let known = |key: &str| entries.iter().any(|e| e.key.as_deref() == Some(key));
        let is_digest = |digest: &str| digest.len() == 64;
        assert!(links.classes.len() >= 2);
        for (id, link) in &links.classes {
            let class = link.class.as_str();
            assert!(id.parse::<u64>().is_ok(), "{id}");
            assert!(is_digest(&link.source_sha256), "{id}");
            let built = build::build(&[super::tests::class(class, 1)], 0).unwrap();
            let slot = built.classes[0]
                .choices
                .iter()
                .find(|c| c.kind == ChoiceKind::Skill)
                .unwrap();
            for feature in link.features.values() {
                assert!(is_digest(&feature.source_sha256));
                let Some(rule) = feature.rule.as_deref() else {
                    continue;
                };
                assert!(known(rule), "{rule}");
                assert!(
                    rule == format!("dnd2024:classes/{class}")
                        || rule.starts_with(&format!("dnd2024:class_features/{class}_")),
                    "{rule} is not a {class} rule"
                );
            }
            for option in link.skills.values() {
                assert!(slot.allowed.permits(&option.value), "{}", option.value);
                assert!(is_digest(&option.source_sha256));
            }
            for option in link.weapons.values() {
                assert!(known(&option.value), "{}", option.value);
                assert!(
                    option.value.starts_with("dnd2024:equipment/"),
                    "{}",
                    option.value
                );
                assert!(is_digest(&option.source_sha256));
            }
        }
    }
}
