//! What a class, level and subclass grant under the SRD catalogue. This reads only typed rule
//! parameters and knows nothing about D&D Beyond. Contracts listed in `content::adapters` use
//! strict shapes, so an unknown field is a load error and cannot be dropped silently. Class notes
//! and feat features stay open gaps: their traits and options are read here, but not all of them.
use crate::content::{
    Entry,
    adapters::{CLASSES, SUBCLASSES},
    definitions,
};
use serde::{Deserialize, de::DeserializeOwned};
use serde_yaml_ng::Value;
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reference {
    key: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TableParameters {
    class: String,
    class_definition: Reference,
    rows: Vec<Row>,
    lookup: Lookup,
    multiclass: Multiclass,
    column_semantics: BTreeMap<String, Column>,
}

#[derive(Debug, Deserialize)]
struct Row {
    level: u32,
    proficiency_bonus: u32,
    features_text: String,
    #[serde(default)]
    grants: Vec<TableGrant>,
    #[serde(flatten)]
    columns: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
enum TableGrant {
    GrantFeature {
        feature: Reference,
        at_class_level: u32,
        arcanum_spell_level: Option<u32>,
        maximum_uses: Option<u32>,
    },
    ChooseSubclass {
        feature: Reference,
        at_class_level: u32,
    },
    GrantSelectedSubclassFeatures {
        class_level: u32,
    },
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Lookup {
    input: String,
    minimum: u32,
    maximum: u32,
    #[serde(rename = "match")]
    matching: String,
    numeric_columns: String,
    feature_grants: String,
    repeat_application: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Multiclass {
    proficiency_bonus_source: String,
    spell_slot_override_rule: Reference,
    pact_magic_pool_separate: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum ColumnKind {
    Bonus,
    DamageBonus,
    DiceExpression,
    ResourceCapacity,
    SelectionCapacity,
    SpeedBonus,
    SpellLevel,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Column {
    kind: ColumnKind,
    selection: Option<String>,
    resource: Option<String>,
    spell_level: Option<usize>,
    eligibility: Option<String>,
    recovery: Option<String>,
    level_change_refills: Option<bool>,
    applies_to: Option<String>,
    requires: Option<String>,
    unit: Option<String>,
    #[serde(default)]
    use_class_row_only_if_single_class: bool,
}

/// A class's rule. The catalogue writes its first-level traits either in one block or directly.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ClassParameters {
    class: String,
    primary_abilities: Vec<String>,
    hit_point_die: HitDie,
    progression: Reference,
    feature_grants: Vec<LevelledGrant>,
    starting_equipment: StartingEquipment,
    first_class_traits: Option<Traits>,
    saving_throw_proficiencies: Option<Vec<String>>,
    skill_proficiencies: Option<SkillChoice>,
    weapon_proficiencies: Option<Value>,
    armor_training: Option<Value>,
    tool_proficiencies: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Traits {
    save_proficiencies: Vec<String>,
    skill_proficiencies: SkillChoice,
    weapon_proficiencies: Value,
    armor_training: Value,
    tool_proficiencies: Option<Value>,
    tool_proficiency_choice: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct HitDie {
    sides: u32,
    #[serde(alias = "count_per_class_level")]
    per_class_level: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SkillChoice {
    count: u32,
    #[serde(alias = "choices")]
    options: SkillOptions,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum SkillOptions {
    Listed(Vec<String>),
    /// The Bard chooses from every skill: `any_skill`.
    Named(String),
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StartingEquipment {
    #[serde(alias = "choose")]
    count: u32,
    options: EquipmentOptions,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EquipmentOptions {
    Listed(Vec<Value>),
    Named(BTreeMap<String, Value>),
}

impl EquipmentOptions {
    /// Each option's name and what it gives. Listed options carry their name in `id`.
    fn by_name(self) -> Result<BTreeMap<String, Value>, String> {
        match self {
            Self::Named(options) => Ok(options),
            Self::Listed(options) => options
                .into_iter()
                .map(|option| {
                    let id = option
                        .get("id")
                        .and_then(Value::as_str)
                        .ok_or("Starting equipment option has no id")?
                        .to_owned();
                    Ok((id, option))
                })
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LevelledGrant {
    #[serde(alias = "definition")]
    feature: Reference,
    #[serde(alias = "class_level")]
    minimum_class_level: u32,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SubclassParameters {
    class: String,
    feature_grants: Vec<LevelledGrant>,
    subclass: Option<String>,
    minimum_class_level: Option<u32>,
    grant_eligible_features: Option<EligibleFeatures>,
}

/// Features apply once the class reaches their level, and they stay.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EligibleFeatures {
    on: Vec<String>,
    comparison: String,
    persistent: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SubclassFeature {
    subclass_choice: SubclassChoice,
    grant_features: Option<GrantFeatures>,
    feature_grant_policy: Option<GrantPolicy>,
    class_scope: Option<Scope>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GrantFeatures {
    from: String,
    when_required_class_level_lte_current: bool,
    reevaluate_on_class_level_gain: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GrantPolicy {
    class_level_at_least_feature_level: bool,
    events: Vec<String>,
    persistent: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SubclassChoice {
    count: u32,
    class: String,
    #[serde(default)]
    srd_options: Vec<Reference>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FeatFeature {
    feat_choice: FeatChoice,
    class: Option<String>,
    #[serde(default, alias = "grant_at_class_levels")]
    grant_at_levels: Vec<u32>,
    minimum_class_level: Option<u32>,
    class_scope: Option<Scope>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FeatChoice {
    #[serde(alias = "count_each", alias = "count_per_milestone")]
    count: u32,
    category: Option<String>,
    allowed_categories: Option<String>,
    #[serde(alias = "category_preference")]
    suggested_category: Option<String>,
    #[serde(default, alias = "must_meet_prerequisites")]
    require_prerequisites: bool,
    default: Option<Reference>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Scope {
    class: String,
    minimum_level: u32,
}

#[derive(Debug, Deserialize)]
struct MulticlassSpellcasting {
    effective_caster_level: EffectiveCasterLevel,
    slots_by_effective_level: Vec<EffectiveLevelSlots>,
}

#[derive(Debug, Deserialize)]
struct EffectiveCasterLevel {
    full_level_classes: Vec<String>,
    half_level_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EffectiveLevelSlots {
    level: u32,
    slots: BTreeMap<String, u32>,
}

fn rule_parameters(entry: &Entry) -> Option<&Value> {
    entry
        .fields
        .get("splat_rule")
        .and_then(|rule| rule.get("parameters"))
}

fn required_parameters(entry: &Entry) -> Result<&Value, String> {
    rule_parameters(entry).ok_or_else(|| format!("{}: no rule parameters", entry.name))
}

fn parameters<T: DeserializeOwned>(entry: &Entry) -> Result<T, String> {
    serde_yaml_ng::from_value(required_parameters(entry)?.clone())
        .map_err(|e| format!("{}: {e}", entry.name))
}

/// What a choice accepts. An open choice is checked by the rule that asks for it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Allowed {
    Any,
    Only(Vec<String>),
}

impl Allowed {
    pub fn permits(&self, value: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Only(options) => options.iter().any(|option| option == value),
        }
    }
}

/// One decision a character's rules ask for. `source` is the rule that asks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChoiceSlot {
    pub source: String,
    pub kind: ChoiceKind,
    pub count: u32,
    pub at_level: u32,
    pub allowed: Allowed,
    /// The pick a rule suggests, such as Ability Score Improvement. Others stay allowed.
    pub default: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChoiceKind {
    Skill,
    Subclass,
    Feat,
    WeaponMastery,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrantedFeature {
    pub key: String,
    pub at_level: u32,
    /// Mystic Arcanum grants the same feature once for each spell level.
    pub arcanum_spell_level: Option<u32>,
    /// Action Surge and Indomitable gain more uses at higher levels.
    pub maximum_uses: Option<u32>,
}

/// The first-level traits of a class. Only the character's starting class gets all of them.
#[derive(Debug)]
pub struct ClassTraits {
    pub primary_abilities: Vec<String>,
    pub saves: Vec<String>,
    pub weapon_proficiencies: Value,
    pub armor_training: Value,
    pub tool_proficiencies: Option<Value>,
    pub starting_equipment_count: u32,
    pub starting_equipment_options: BTreeMap<String, Value>,
}

struct ClassRules {
    key: String,
    hit_die_sides: u32,
    rows: Vec<Row>,
    columns: BTreeMap<String, Column>,
    traits: ClassTraits,
    skills: SkillChoice,
    subclass_keys: Vec<String>,
}

struct SubclassRules {
    class: String,
    features: Vec<LevelledGrant>,
}

struct FeatMilestones {
    levels: Vec<u32>,
    count: u32,
    default: Option<String>,
}

struct SubclassSlot {
    count: u32,
    options: Vec<String>,
}

struct Catalogue {
    classes: BTreeMap<String, ClassRules>,
    subclasses: BTreeMap<String, SubclassRules>,
    feat_features: BTreeMap<String, FeatMilestones>,
    subclass_features: BTreeMap<String, SubclassSlot>,
    entries: BTreeMap<&'static str, &'static Entry>,
    multiclass: MulticlassSpellcasting,
}

impl Catalogue {
    fn rule_parameters(&self, key: &str) -> Option<&Value> {
        rule_parameters(self.entries.get(key)?)
    }
}

/// Whether a grant policy runs only on events that the build covers.
fn supported_events(events: &[String]) -> bool {
    events
        .iter()
        .all(|event| ["subclass.selected", "character.level_gained"].contains(&event.as_str()))
}

fn load() -> Result<Catalogue, String> {
    let entries: BTreeMap<&'static str, &'static Entry> = definitions::entries()?
        .iter()
        .filter_map(|entry| entry.key.as_deref().map(|key| (key, entry)))
        .collect();
    let get = |key: &str| {
        entries
            .get(key)
            .copied()
            .ok_or_else(|| format!("Missing catalogue rule {key}"))
    };
    let mut subclasses = BTreeMap::new();
    for subclass in SUBCLASSES {
        let key = format!("dnd2024:subclasses/{subclass}");
        let parameters: SubclassParameters = parameters(get(&key)?)?;
        let name_matches = parameters.subclass.as_ref().is_none_or(|name| {
            subclass.ends_with(name.as_str()) && subclass.starts_with(parameters.class.as_str())
        });
        let policy_supported = parameters.grant_eligible_features.as_ref().is_none_or(|p| {
            p.persistent
                && p.comparison == "class_level_at_least_threshold"
                && supported_events(&p.on)
        });
        if !name_matches
            || parameters.minimum_class_level.is_some_and(|l| l != 3)
            || !policy_supported
        {
            return Err(format!("{subclass}: unsupported subclass rule"));
        }
        subclasses.insert(
            key,
            SubclassRules {
                class: parameters.class,
                features: parameters.feature_grants,
            },
        );
    }
    let mut classes = BTreeMap::new();
    let mut feat_features = BTreeMap::new();
    let mut subclass_features = BTreeMap::new();
    for class in CLASSES {
        let table: TableParameters =
            parameters(get(&format!("dnd2024:tables/{class}_progression"))?)?;
        let definition: ClassParameters = parameters(get(&table.class_definition.key)?)?;
        if table.class != class
            || definition.class != class
            || definition.hit_point_die.per_class_level != 1
            || definition.progression.key != format!("dnd2024:tables/{class}_progression")
        {
            return Err(format!("{class}: class and progression disagree"));
        }
        check_table(class, &table)?;
        let traits = match definition.first_class_traits {
            Some(traits) => traits,
            None => Traits {
                save_proficiencies: definition
                    .saving_throw_proficiencies
                    .ok_or_else(|| format!("{class}: no saving throws"))?,
                skill_proficiencies: definition
                    .skill_proficiencies
                    .ok_or_else(|| format!("{class}: no skill choice"))?,
                weapon_proficiencies: definition
                    .weapon_proficiencies
                    .ok_or_else(|| format!("{class}: no weapon proficiencies"))?,
                armor_training: definition
                    .armor_training
                    .ok_or_else(|| format!("{class}: no armor training"))?,
                tool_proficiencies: definition.tool_proficiencies,
                tool_proficiency_choice: None,
            },
        };
        if let SkillOptions::Named(name) = &traits.skill_proficiencies.options
            && name != "any_skill"
        {
            return Err(format!("{class}: unknown skill choice {name}"));
        }
        let subclass_keys: Vec<String> = subclasses
            .iter()
            .filter(|(_, rules)| rules.class == class)
            .map(|(key, _)| key.clone())
            .collect();
        for row in &table.rows {
            for grant in &row.grants {
                let key = match grant {
                    TableGrant::GrantFeature { feature, .. }
                    | TableGrant::ChooseSubclass { feature, .. } => &feature.key,
                    TableGrant::GrantSelectedSubclassFeatures { .. } => continue,
                };
                let entry = get(key)?;
                if key.ends_with("_ability_score_improvement") || key.ends_with("_epic_boon") {
                    feat_features
                        .entry(key.clone())
                        .or_insert(feat_milestones(entry)?);
                }
                if matches!(grant, TableGrant::ChooseSubclass { .. })
                    && !subclass_features.contains_key(key)
                {
                    subclass_features.insert(key.clone(), subclass_slot(class, entry)?);
                }
            }
        }
        classes.insert(
            class.to_owned(),
            ClassRules {
                key: table.class_definition.key,
                hit_die_sides: definition.hit_point_die.sides,
                rows: table.rows,
                columns: table.column_semantics,
                traits: ClassTraits {
                    primary_abilities: definition.primary_abilities,
                    saves: traits.save_proficiencies,
                    weapon_proficiencies: traits.weapon_proficiencies,
                    armor_training: traits.armor_training,
                    tool_proficiencies: traits
                        .tool_proficiencies
                        .or(traits.tool_proficiency_choice),
                    starting_equipment_count: definition.starting_equipment.count,
                    starting_equipment_options: definition.starting_equipment.options.by_name()?,
                },
                skills: traits.skill_proficiencies,
                subclass_keys,
            },
        );
        for grant in definition.feature_grants {
            get(&grant.feature.key)?;
        }
    }
    let multiclass = parameters(get("dnd2024:rules/multiclassing_spellcasting")?)?;
    Ok(Catalogue {
        classes,
        subclasses,
        feat_features,
        subclass_features,
        entries,
        multiclass,
    })
}

/// The lookup and column rules every table must follow before its values can be trusted.
fn check_table(class: &str, table: &TableParameters) -> Result<(), String> {
    let lookup = &table.lookup;
    if lookup.input != "class_level"
        || (lookup.minimum, lookup.maximum) != (1, 20)
        || lookup.matching != "exact"
        || lookup.numeric_columns != "current_capacity_or_value"
        || lookup.feature_grants != "all_crossed_levels"
        || lookup.repeat_application != "idempotent_per_class_level_and_feature"
        || table.multiclass.proficiency_bonus_source != "total_character_level"
        || table.multiclass.spell_slot_override_rule.key
            != "dnd2024:rules/multiclassing_spellcasting"
        || !table.multiclass.pact_magic_pool_separate
        || table.rows.len() != 20
    {
        return Err(format!("{class}: unsupported progression lookup"));
    }
    for (name, column) in &table.column_semantics {
        let spell_slot = column.resource.as_deref() == Some("spell_slot");
        if spell_slot && column.recovery.as_deref() != Some("long_rest")
            || name == "pact_slots" && column.level_change_refills != Some(false)
            || column.kind == ColumnKind::Bonus && !column.use_class_row_only_if_single_class
        {
            return Err(format!("{class}: unsupported {name} column"));
        }
    }
    for (index, row) in table.rows.iter().enumerate() {
        if row.level as usize != index + 1 {
            return Err(format!("{class}: progression rows are out of order"));
        }
        // A dice column also has its parsed form beside it, such as `martial_arts_dice`.
        let mut columns: Vec<_> = row
            .columns
            .keys()
            .map(String::as_str)
            .filter(|name| !is_dice_form(name, &table.column_semantics))
            .collect();
        columns.push("proficiency_bonus");
        columns.sort_unstable();
        if !columns
            .iter()
            .copied()
            .eq(table.column_semantics.keys().map(String::as_str))
        {
            return Err(format!(
                "{class}: level {} columns differ from their meanings",
                row.level
            ));
        }
    }
    Ok(())
}

fn is_dice_form(name: &str, columns: &BTreeMap<String, Column>) -> bool {
    name.strip_suffix("_dice")
        .and_then(|base| columns.get(base))
        .is_some_and(|column| column.kind == ColumnKind::DiceExpression)
}

fn feat_milestones(entry: &Entry) -> Result<FeatMilestones, String> {
    let feature: FeatFeature = parameters(entry)?;
    let levels = if !feature.grant_at_levels.is_empty() {
        feature.grant_at_levels
    } else if let Some(level) = feature.minimum_class_level {
        vec![level]
    } else if let Some(scope) = &feature.class_scope {
        vec![scope.minimum_level]
    } else {
        return Err(format!("{}: feat choice has no level", entry.name));
    };
    if !feature.feat_choice.require_prerequisites {
        return Err(format!("{}: feat prerequisites must apply", entry.name));
    }
    let categories = [
        feature.feat_choice.category.as_deref(),
        feature.feat_choice.allowed_categories.as_deref(),
    ];
    if categories.iter().flatten().any(|c| *c != "any") {
        return Err(format!("{}: unsupported feat category limit", entry.name));
    }
    if let Some(suggested) = feature.feat_choice.suggested_category.as_deref()
        && suggested != "epic_boon"
    {
        return Err(format!("{}: unknown suggested category", entry.name));
    }
    if feature.class.is_some_and(|class| {
        feature
            .class_scope
            .as_ref()
            .is_some_and(|scope| scope.class != class)
    }) {
        return Err(format!("{}: class scope disagrees", entry.name));
    }
    Ok(FeatMilestones {
        levels,
        count: feature.feat_choice.count,
        default: feature.feat_choice.default.map(|feat| feat.key),
    })
}

fn subclass_slot(class: &str, entry: &Entry) -> Result<SubclassSlot, String> {
    // Wizard Subclass also specifies spellbook copying. That part stays an open engine gap.
    let mut parameters = required_parameters(entry)?.clone();
    if let Some(map) = parameters.as_mapping_mut() {
        map.remove("spellbook_copying_sidebar");
    }
    let feature: SubclassFeature =
        serde_yaml_ng::from_value(parameters).map_err(|e| format!("{}: {e}", entry.name))?;
    let grants_by_level = feature.grant_features.as_ref().is_some_and(|g| {
        g.from == "selected_subclass"
            && g.when_required_class_level_lte_current
            && g.reevaluate_on_class_level_gain
    }) || feature.feature_grant_policy.as_ref().is_some_and(|p| {
        p.class_level_at_least_feature_level && p.persistent && supported_events(&p.events)
    });
    if feature.subclass_choice.class != class
        || !grants_by_level
        || feature
            .class_scope
            .as_ref()
            .is_some_and(|scope| scope.class != class || scope.minimum_level != 3)
    {
        return Err(format!("{}: unsupported subclass choice", entry.name));
    }
    Ok(SubclassSlot {
        count: feature.subclass_choice.count,
        options: feature
            .subclass_choice
            .srd_options
            .into_iter()
            .map(|option| option.key)
            .collect(),
    })
}

fn catalogue() -> Result<&'static Catalogue, String> {
    static CATALOGUE: OnceLock<Result<Catalogue, String>> = OnceLock::new();
    CATALOGUE.get_or_init(load).as_ref().map_err(Clone::clone)
}

/// One class on a character. `previous_level` is zero for a full build. It lets a caller ask only
/// for what a level change granted: table grants apply to every level crossed. When a subclass is
/// chosen after its level has passed, `subclass_features` gives what the choice grants.
#[derive(Clone, Copy, Debug)]
pub struct Selection<'a> {
    pub class: &'a str,
    pub level: u32,
    pub previous_level: u32,
    pub subclass: Option<&'a str>,
    /// Only a character's first class gets its first-level traits and skill choice.
    pub starting: bool,
}

#[derive(Clone, Debug)]
pub struct ClassBuild {
    pub class: String,
    pub key: String,
    pub level: u32,
    /// The table's own words for what this level grants.
    pub features_text: String,
    pub hit_die_sides: u32,
    pub features: Vec<GrantedFeature>,
    /// `None` for a class taken by multiclassing, which has its own smaller set of proficiencies.
    pub traits: Option<&'static ClassTraits>,
    pub choices: Vec<ChoiceSlot>,
    pub selection_capacities: BTreeMap<String, Capacity>,
    pub resource_capacities: BTreeMap<String, Capacity>,
    /// Column values with other meanings, such as a Monk's Martial Arts die.
    pub values: BTreeMap<String, ColumnValue>,
    /// Spell slots from this class's table. They recover on a Long Rest.
    pub spell_slots: [u32; 9],
    pub pact_slots: Option<PactSlots>,
    pub problems: Vec<String>,
}

/// A table number with the rule that governs it. `owning_feature_contract` names the feature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capacity {
    pub count: u32,
    pub eligibility: Option<String>,
    pub recovery: Option<String>,
    pub refills_on_level_change: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnValue {
    pub text: String,
    pub unit: Option<String>,
    pub applies_to: Option<String>,
    /// The feature that must apply before this value does, such as Rage for its damage.
    pub requires: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PactSlots {
    pub count: u32,
    pub spell_level: u32,
    /// Which rule restores these slots. The table names the feature that owns the rule.
    pub recovery: Option<String>,
    /// Gaining a level does not refill them. This module never changes an actor's state.
    pub refills_on_level_change: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct Build {
    pub level: u32,
    pub proficiency_bonus: u32,
    pub classes: Vec<ClassBuild>,
    /// `None` when another class outside the catalogue could change the result.
    pub spell_slots: Option<[u32; 9]>,
}

/// `other_levels` counts class levels the catalogue does not cover. They still raise the
/// proficiency bonus, and they make the combined spell slots unknown.
pub fn build(selections: &[Selection], other_levels: u32) -> Result<Build, String> {
    let catalogue = catalogue()?;
    let level = selections.iter().map(|s| s.level).sum::<u32>() + other_levels;
    if !(1..=20).contains(&level) {
        return Err(format!("Character level {level} is outside 1 to 20"));
    }
    let classes = selections
        .iter()
        .map(|selection| class_build(catalogue, selection))
        .collect::<Result<Vec<_>, _>>()?;
    let proficiency_bonus = catalogue
        .classes
        .values()
        .next()
        .and_then(|class| class.rows.get(level as usize - 1))
        .map(|row| row.proficiency_bonus)
        .ok_or("Catalogue has no progression tables")?;
    let spell_slots = (other_levels == 0).then(|| combined_slots(catalogue, &classes));
    Ok(Build {
        level,
        proficiency_bonus,
        classes,
        spell_slots,
    })
}

fn class_build(catalogue: &'static Catalogue, selection: &Selection) -> Result<ClassBuild, String> {
    let rules = catalogue
        .classes
        .get(selection.class)
        .ok_or_else(|| format!("Unknown class {}", selection.class))?;
    if !(1..=20).contains(&selection.level) || selection.previous_level > selection.level {
        return Err(format!(
            "{}: invalid level change {} to {}",
            selection.class, selection.previous_level, selection.level
        ));
    }
    let row = &rules.rows[selection.level as usize - 1];
    let subclass = selection
        .subclass
        .map(|key| {
            catalogue
                .subclasses
                .get(key)
                .filter(|rules| rules.class == selection.class)
                .ok_or_else(|| format!("{key} is not a {} subclass", selection.class))
        })
        .transpose()?;
    let mut build = ClassBuild {
        class: selection.class.to_owned(),
        key: rules.key.clone(),
        level: selection.level,
        features_text: row.features_text.clone(),
        hit_die_sides: rules.hit_die_sides,
        features: Vec::new(),
        traits: selection.starting.then_some(&rules.traits),
        choices: Vec::new(),
        selection_capacities: BTreeMap::new(),
        resource_capacities: BTreeMap::new(),
        values: BTreeMap::new(),
        spell_slots: [0; 9],
        pact_slots: None,
        problems: Vec::new(),
    };
    if selection.subclass.is_some() && selection.level < 3 {
        build.problems.push(format!(
            "{} has a subclass before class level 3.",
            selection.class
        ));
    }
    if selection.starting && selection.previous_level == 0 {
        build.choices.push(ChoiceSlot {
            source: rules.key.clone(),
            kind: ChoiceKind::Skill,
            count: rules.skills.count,
            at_level: 1,
            allowed: match &rules.skills.options {
                SkillOptions::Listed(options) => Allowed::Only(options.clone()),
                SkillOptions::Named(_) => Allowed::Any,
            },
            default: None,
        });
    }
    let crossed = selection.previous_level as usize..selection.level as usize;
    for crossed_row in &rules.rows[crossed] {
        for grant in &crossed_row.grants {
            add_grant(catalogue, rules, subclass, grant, &mut build)?;
        }
    }
    add_mastery_slot(catalogue, rules, selection, &mut build)?;
    add_columns(rules, row, &mut build)?;
    build.features.sort_by_key(|f| f.at_level);
    Ok(build)
}

fn subclass_grants_at(
    subclass: Option<&SubclassRules>,
    level: u32,
) -> impl Iterator<Item = &LevelledGrant> {
    subclass
        .into_iter()
        .flat_map(|rules| &rules.features)
        .filter(move |grant| grant.minimum_class_level == level)
}

fn add_grant(
    catalogue: &Catalogue,
    rules: &ClassRules,
    subclass: Option<&SubclassRules>,
    grant: &TableGrant,
    build: &mut ClassBuild,
) -> Result<(), String> {
    let mut grant_feature = |key: &str, at_level, arcanum_spell_level, maximum_uses| {
        let exists = build
            .features
            .iter()
            .any(|f| f.key == key && f.at_level == at_level);
        if !exists {
            build.features.push(GrantedFeature {
                key: key.to_owned(),
                at_level,
                arcanum_spell_level,
                maximum_uses,
            });
        }
    };
    match grant {
        TableGrant::GrantFeature {
            feature,
            at_class_level,
            arcanum_spell_level,
            maximum_uses,
        } => {
            grant_feature(
                &feature.key,
                *at_class_level,
                *arcanum_spell_level,
                *maximum_uses,
            );
            if let Some(milestones) = catalogue.feat_features.get(&feature.key)
                && milestones.levels.contains(at_class_level)
            {
                build.choices.push(ChoiceSlot {
                    source: feature.key.clone(),
                    kind: ChoiceKind::Feat,
                    count: milestones.count,
                    at_level: *at_class_level,
                    allowed: Allowed::Any,
                    default: milestones.default.clone(),
                });
            }
        }
        TableGrant::ChooseSubclass {
            feature,
            at_class_level,
        } => {
            grant_feature(&feature.key, *at_class_level, None, None);
            for grant in subclass_grants_at(subclass, *at_class_level) {
                grant_feature(&grant.feature.key, *at_class_level, None, None);
            }
            let slot = catalogue
                .subclass_features
                .get(&feature.key)
                .ok_or_else(|| format!("{}: not a subclass choice", feature.key))?;
            build.choices.push(ChoiceSlot {
                source: feature.key.clone(),
                kind: ChoiceKind::Subclass,
                count: slot.count,
                at_level: *at_class_level,
                allowed: Allowed::Only(if slot.options.is_empty() {
                    rules.subclass_keys.clone()
                } else {
                    slot.options.clone()
                }),
                default: None,
            });
        }
        TableGrant::GrantSelectedSubclassFeatures { class_level } => {
            for grant in subclass_grants_at(subclass, *class_level) {
                grant_feature(&grant.feature.key, *class_level, None, None);
            }
        }
    }
    Ok(())
}

/// Weapon Mastery asks for one weapon per point in the table's column, or a fixed number. Each
/// level that adds weapons gets its own slot, so a level change asks only for what it added.
fn add_mastery_slot(
    catalogue: &Catalogue,
    rules: &ClassRules,
    selection: &Selection,
    build: &mut ClassBuild,
) -> Result<(), String> {
    let key = format!(
        "dnd2024:class_features/{}_level_1_weapon_mastery",
        build.class
    );
    let Some(parameters) = catalogue.rule_parameters(&key) else {
        return Ok(());
    };
    let fixed = parameters
        .get("mastery_choices")
        .or_else(|| parameters.get("mastery_choice"))
        .and_then(|choices| choices.get("count"))
        .and_then(Value::as_u64);
    let count_at = |level: u32| -> Result<u64, String> {
        if level == 0 {
            return Ok(0);
        }
        fixed
            .or_else(|| {
                rules.rows[level as usize - 1]
                    .columns
                    .get("weapon_mastery")
                    .and_then(Value::as_u64)
            })
            .ok_or_else(|| format!("{}: no Weapon Mastery count", build.class))
    };
    for level in selection.previous_level + 1..=selection.level {
        let added = count_at(level)?.saturating_sub(count_at(level - 1)?);
        if added > 0 {
            build.choices.push(ChoiceSlot {
                source: key.clone(),
                kind: ChoiceKind::WeaponMastery,
                count: added as u32,
                at_level: level,
                allowed: Allowed::Any,
                default: None,
            });
        }
    }
    Ok(())
}

fn add_columns(rules: &ClassRules, row: &Row, build: &mut ClassBuild) -> Result<(), String> {
    let mut pact = (None, None);
    for (name, value) in &row.columns {
        if is_dice_form(name, &rules.columns) {
            continue;
        }
        let column = rules
            .columns
            .get(name)
            .ok_or_else(|| format!("{}: unknown column {name}", build.class))?;
        let number = || {
            value
                .as_u64()
                .map(|n| n as u32)
                .ok_or_else(|| format!("{}: {name} must be a whole number", build.class))
        };
        match column.kind {
            ColumnKind::SelectionCapacity => {
                let selection = column.selection.as_ref().unwrap_or(name);
                build.selection_capacities.insert(
                    selection.clone(),
                    Capacity {
                        count: number()?,
                        eligibility: column.eligibility.clone(),
                        recovery: None,
                        refills_on_level_change: None,
                    },
                );
            }
            ColumnKind::ResourceCapacity if column.resource.as_deref() == Some("spell_slot") => {
                let level = column
                    .spell_level
                    .filter(|level| (1..=9).contains(level))
                    .ok_or_else(|| format!("{}: {name} has no spell level", build.class))?;
                build.spell_slots[level - 1] = number()?;
            }
            ColumnKind::ResourceCapacity if name == "pact_slots" => pact.0 = Some(number()?),
            ColumnKind::SpellLevel if name == "pact_slot_level" => pact.1 = Some(number()?),
            ColumnKind::ResourceCapacity => {
                let resource = column.resource.as_ref().unwrap_or(name);
                build.resource_capacities.insert(
                    resource.clone(),
                    Capacity {
                        count: number()?,
                        eligibility: None,
                        recovery: column.recovery.clone(),
                        refills_on_level_change: column.level_change_refills,
                    },
                );
            }
            // The proficiency bonus comes from the total level, on `Build`.
            ColumnKind::Bonus => {}
            ColumnKind::DamageBonus
            | ColumnKind::DiceExpression
            | ColumnKind::SpeedBonus
            | ColumnKind::SpellLevel => {
                build.values.insert(
                    name.clone(),
                    ColumnValue {
                        text: scalar(value),
                        unit: column.unit.clone(),
                        applies_to: column.applies_to.clone(),
                        requires: column.requires.clone(),
                    },
                );
            }
        }
    }
    if let (Some(count), Some(spell_level)) = pact {
        let column = rules.columns.get("pact_slots");
        build.pact_slots = Some(PactSlots {
            count,
            spell_level,
            recovery: column.and_then(|c| c.recovery.clone()),
            refills_on_level_change: column.and_then(|c| c.level_change_refills),
        });
    }
    Ok(())
}

fn scalar(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        Value::Bool(flag) => flag.to_string(),
        other => format!("{other:?}"),
    }
}

/// One spellcasting class uses its own table. Two or more use the Multiclass Spellcaster table.
fn combined_slots(catalogue: &Catalogue, classes: &[ClassBuild]) -> [u32; 9] {
    let casters: Vec<&ClassBuild> = classes
        .iter()
        .filter(|class| class.spell_slots.iter().any(|slots| *slots > 0))
        .collect();
    match casters.as_slice() {
        [] => [0; 9],
        [single] => single.spell_slots,
        many => {
            let rule = &catalogue.multiclass.effective_caster_level;
            let effective: u32 = many
                .iter()
                .map(|class| {
                    if rule.full_level_classes.contains(&class.class) {
                        class.level
                    } else if rule.half_level_classes.contains(&class.class) {
                        class.level.div_ceil(2)
                    } else {
                        0
                    }
                })
                .sum();
            let mut result = [0; 9];
            if let Some(row) = catalogue
                .multiclass
                .slots_by_effective_level
                .iter()
                .find(|row| row.level == effective)
            {
                for (level, count) in &row.slots {
                    if let Some(slot) = level
                        .parse::<usize>()
                        .ok()
                        .filter(|level| (1..=9).contains(level))
                    {
                        result[slot - 1] = *count;
                    }
                }
            }
            result
        }
    }
}

/// What a subclass grants a class of this level: the subclass contracts' `feature_grants` and
/// `feature_state`, because a subclass note specifies nothing else. Use it when the choice comes
/// after the level.
pub fn subclass_features(subclass: &str, class_level: u32) -> Result<Vec<GrantedFeature>, String> {
    let rules = catalogue()?
        .subclasses
        .get(subclass)
        .ok_or_else(|| format!("Unknown subclass {subclass}"))?;
    Ok(rules
        .features
        .iter()
        .filter(|grant| grant.minimum_class_level <= class_level)
        .map(|grant| GrantedFeature {
            key: grant.feature.key.clone(),
            at_level: grant.minimum_class_level,
            arcanum_spell_level: None,
            maximum_uses: None,
        })
        .collect())
}

/// The resource column a class feature takes its uses from, such as Rage's `rages`.
pub fn resource_column(class: &str, feature: &str) -> Result<Option<String>, String> {
    let catalogue = catalogue()?;
    let (Some(rules), Some(parameters)) = (
        catalogue.classes.get(class),
        catalogue.rule_parameters(feature),
    ) else {
        return Ok(None);
    };
    let mut found = Vec::new();
    columns_named(parameters, &mut found);
    Ok(found
        .into_iter()
        .find(|name| {
            rules
                .columns
                .get(*name)
                .is_some_and(|column| column.kind == ColumnKind::ResourceCapacity)
        })
        .map(str::to_owned))
}

fn columns_named<'a>(value: &'a Value, found: &mut Vec<&'a str>) {
    match value {
        Value::Mapping(map) => {
            found.extend(map.get("column").and_then(Value::as_str));
            map.values().for_each(|value| columns_named(value, found));
        }
        Value::Sequence(items) => items.iter().for_each(|value| columns_named(value, found)),
        _ => {}
    }
}

/// Whether a class's Weapon Mastery rule accepts a weapon. A rule that names no limit accepts any
/// catalogue weapon, because its other limits depend on the character's proficiencies.
pub fn mastery_allows(feature: &str, weapon: &str) -> Result<bool, String> {
    let catalogue = catalogue()?;
    let eligible = catalogue
        .rule_parameters(feature)
        .and_then(|parameters| parameters.get("mastery_choices"))
        .and_then(|choices| choices.get("eligible"));
    let category = catalogue
        .rule_parameters(weapon)
        .and_then(|parameters| parameters.get("weapon_category"))
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{weapon} is not a catalogue weapon"))?;
    let Some(eligible) = eligible else {
        return Ok(true);
    };
    let (kind, range) = category.split_once('_').unwrap_or((category, ""));
    let kind_ok = eligible
        .get("weapon_category")
        .and_then(Value::as_sequence)
        .is_none_or(|kinds| kinds.iter().any(|k| k.as_str() == Some(kind)));
    let range_ok = eligible
        .get("weapon_range")
        .and_then(Value::as_str)
        .is_none_or(|wanted| wanted == range);
    Ok(kind_ok && range_ok)
}

/// What a character brings to a feat's prerequisites.
#[derive(Debug)]
pub struct FeatContext {
    pub level: u32,
    pub scores: BTreeMap<String, i64>,
    /// Lowercase feature kinds the character has, such as `spellcasting` or `fighting_style`.
    pub features: Vec<&'static str>,
}

/// Whether a character qualifies for a catalogue feat. The error says why not.
pub fn feat_allowed(feat: &str, context: &FeatContext) -> Result<(), String> {
    let catalogue = catalogue()?;
    let entry = catalogue
        .entries
        .get(feat)
        .ok_or_else(|| format!("Unknown feat {feat}"))?;
    let parameters = required_parameters(entry)?;
    let prerequisites = parameters.get("prerequisites");
    let minimum_level = prerequisites
        .and_then(|p| p.get("minimum_character_level"))
        .or_else(|| parameters.get("minimum_character_level"))
        .and_then(Value::as_u64);
    if let Some(minimum) = minimum_level
        && u64::from(context.level) < minimum
    {
        return Err(format!("{} needs character level {minimum}.", entry.name));
    }
    let feature = prerequisites
        .and_then(|p| p.get("feature"))
        .or_else(|| parameters.get("prerequisite_feature"))
        .and_then(Value::as_str);
    if let Some(feature) = feature
        && !context.features.contains(&feature)
    {
        return Err(format!("{} needs {feature}.", entry.name));
    }
    if let Some(any) = prerequisites.and_then(|p| p.get("any_ability_score_minimum")) {
        let minimum = any.get("minimum").and_then(Value::as_i64).unwrap_or(0);
        let meets = any
            .get("abilities")
            .and_then(Value::as_sequence)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .any(|ability| context.scores.get(ability).is_some_and(|s| *s >= minimum));
        if !meets {
            return Err(format!(
                "{} needs an ability score of {minimum}.",
                entry.name
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::adapters::implemented;
    use serde::Deserialize;

    fn selection(class: &str, level: u32, starting: bool) -> Selection<'_> {
        Selection {
            class,
            level,
            previous_level: 0,
            subclass: None,
            starting,
        }
    }

    fn single(class: &str, level: u32) -> Build {
        build(&[selection(class, level, true)], 0).unwrap()
    }

    /// The published table: `| Level | Proficiency Bonus | Class Features | ...`.
    fn source_rows(entry: &Entry) -> Vec<Vec<String>> {
        let table = entry
            .body
            .split("## Rule")
            .next()
            .unwrap()
            .lines()
            .filter(|line| line.starts_with("| "))
            .skip(2);
        table
            .map(|line| {
                line.trim_matches('|')
                    .split('|')
                    .map(|cell| cell.trim().to_owned())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn every_table_matches_its_published_rows() {
        let catalogue = catalogue().unwrap();
        for class in CLASSES {
            let entry = catalogue.entries[format!("dnd2024:tables/{class}_progression").as_str()];
            let rows = source_rows(entry);
            assert_eq!(rows.len(), 20, "{class}");
            for (index, cells) in rows.iter().enumerate() {
                let row = &catalogue.classes[class].rows[index];
                assert_eq!(cells[0], (index + 1).to_string(), "{class}");
                assert_eq!(cells[1], format!("+{}", row.proficiency_bonus), "{class}");
                assert_eq!(cells[2], row.features_text, "{class} level {}", index + 1);
                let listed = if cells[2] == "—" {
                    0
                } else {
                    cells[2].split(',').count()
                };
                assert_eq!(row.grants.len(), listed, "{class} level {}", index + 1);
            }
        }
    }

    #[test]
    fn slot_columns_match_the_published_table() {
        let catalogue = catalogue().unwrap();
        for (class, columns) in [
            ("bard", 9),
            ("cleric", 9),
            ("druid", 9),
            ("paladin", 5),
            ("ranger", 5),
            ("sorcerer", 9),
            ("wizard", 9),
        ] {
            let entry = catalogue.entries[format!("dnd2024:tables/{class}_progression").as_str()];
            for (index, cells) in source_rows(entry).iter().enumerate() {
                let built = single(class, index as u32 + 1).classes.remove(0);
                let mut published = [0; 9];
                for (slot, cell) in published.iter_mut().zip(&cells[cells.len() - columns..]) {
                    *slot = cell.parse().unwrap_or(0);
                }
                assert_eq!(built.spell_slots, published, "{class} level {}", index + 1);
            }
        }
    }

    #[test]
    fn subclass_levels_match_the_table_and_grants_are_complete() {
        let catalogue = catalogue().unwrap();
        for (class, rules) in &catalogue.classes {
            let table_levels: Vec<u32> = rules
                .rows
                .iter()
                .flat_map(|row| &row.grants)
                .filter_map(|grant| match grant {
                    TableGrant::GrantSelectedSubclassFeatures { class_level } => Some(*class_level),
                    _ => None,
                })
                .collect();
            for key in &rules.subclass_keys {
                let subclass = &catalogue.subclasses[key];
                for grant in &subclass.features {
                    assert!(
                        grant.minimum_class_level == 3
                            || table_levels.contains(&grant.minimum_class_level),
                        "{class}: {} at level {} has no table row",
                        grant.feature.key,
                        grant.minimum_class_level
                    );
                }
            }
            let built = build(
                &[Selection {
                    subclass: rules.subclass_keys.first().map(String::as_str),
                    ..selection(class, 20, true)
                }],
                0,
            )
            .unwrap();
            let subclass = &catalogue.subclasses[&rules.subclass_keys[0]];
            for grant in &subclass.features {
                assert!(
                    built.classes[0]
                        .features
                        .iter()
                        .any(|f| f.key == grant.feature.key),
                    "{class}: {} was not granted",
                    grant.feature.key
                );
            }
        }
    }

    #[test]
    fn feat_milestones_follow_their_own_rule_and_the_table() {
        let catalogue = catalogue().unwrap();
        for class in CLASSES {
            let at = |key: &str| -> Vec<u32> {
                catalogue.classes[class]
                    .rows
                    .iter()
                    .flat_map(|row| &row.grants)
                    .filter_map(|grant| match grant {
                        TableGrant::GrantFeature {
                            feature,
                            at_class_level,
                            ..
                        } if feature.key == key => Some(*at_class_level),
                        _ => None,
                    })
                    .collect()
            };
            let improvement =
                format!("dnd2024:class_features/{class}_level_4_ability_score_improvement");
            assert_eq!(
                at(&improvement),
                catalogue.feat_features[&improvement].levels,
                "{class}"
            );
            let boon = format!("dnd2024:class_features/{class}_level_19_epic_boon");
            assert_eq!(at(&boon), vec![19], "{class}");
            assert_eq!(catalogue.feat_features[&boon].levels, vec![19], "{class}");
        }
    }

    #[test]
    fn known_values_at_a_level() {
        let barbarian = single("barbarian", 1).classes.remove(0);
        assert_eq!(barbarian.resource_capacities["rages"].count, 2);
        assert_eq!(barbarian.selection_capacities["weapon_mastery"].count, 2);
        assert_eq!(barbarian.spell_slots, [0; 9]);
        assert_eq!(barbarian.hit_die_sides, 12);
        assert_eq!(barbarian.traits.unwrap().starting_equipment_count, 1);
        assert_eq!(
            barbarian.traits.unwrap().starting_equipment_options.len(),
            2
        );
        let wizard = single("wizard", 5);
        assert_eq!(wizard.proficiency_bonus, 3);
        assert_eq!(wizard.spell_slots.unwrap(), [4, 3, 2, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            wizard.classes[0].selection_capacities["prepared_spells"].count,
            9
        );
        assert_eq!(wizard.classes[0].selection_capacities["cantrips"].count, 4);
        let warlock = single("warlock", 11).classes.remove(0);
        assert_eq!(
            warlock.pact_slots,
            Some(PactSlots {
                count: 3,
                spell_level: 5,
                recovery: Some("owning_feature_contract".into()),
                refills_on_level_change: Some(false),
            })
        );
        assert_eq!(warlock.spell_slots, [0; 9]);
    }

    #[test]
    fn multiclass_uses_total_level_and_the_shared_slot_table() {
        // The SRD's worked example: a level 4 Ranger / level 3 Sorcerer counts as level 5.
        let built = build(
            &[
                selection("ranger", 4, true),
                selection("sorcerer", 3, false),
            ],
            0,
        )
        .unwrap();
        assert_eq!(built.level, 7);
        assert_eq!(built.proficiency_bonus, 3);
        assert_eq!(built.spell_slots.unwrap(), [4, 3, 2, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            built.classes[0].selection_capacities["prepared_spells"].count,
            5
        );
        assert_eq!(
            built.classes[1].selection_capacities["prepared_spells"].count,
            6
        );
        assert!(built.classes[0].traits.is_some());
        assert!(built.classes[1].traits.is_none());
        // Only the starting class offers its first-level skill choice.
        let skill = |class: &ClassBuild| class.choices.iter().any(|c| c.kind == ChoiceKind::Skill);
        assert!(skill(&built.classes[0]));
        assert!(!skill(&built.classes[1]));
    }

    #[test]
    fn pact_magic_stays_separate_and_outside_levels_hide_combined_slots() {
        let built = build(
            &[selection("warlock", 3, true), selection("wizard", 3, false)],
            0,
        )
        .unwrap();
        assert_eq!(built.spell_slots.unwrap(), [4, 2, 0, 0, 0, 0, 0, 0, 0]);
        assert!(built.classes[0].pact_slots.is_some());
        let unknown = build(&[selection("wizard", 3, true)], 2).unwrap();
        assert_eq!(unknown.level, 5);
        assert_eq!(unknown.proficiency_bonus, 3);
        assert_eq!(unknown.spell_slots, None);
    }

    #[test]
    fn a_level_change_grants_only_the_levels_crossed() {
        let gained = build(
            &[Selection {
                previous_level: 3,
                ..selection("barbarian", 5, true)
            }],
            0,
        )
        .unwrap()
        .classes
        .remove(0);
        let levels: Vec<u32> = gained.features.iter().map(|f| f.at_level).collect();
        assert!(
            levels.iter().all(|level| (4..=5).contains(level)),
            "{levels:?}"
        );
        assert!(
            gained
                .features
                .iter()
                .any(|f| f.key.ends_with("extra_attack"))
        );
        assert!(!gained.choices.iter().any(|c| c.kind == ChoiceKind::Skill));
        assert!(
            gained
                .choices
                .iter()
                .any(|c| c.kind == ChoiceKind::Feat && c.at_level == 4)
        );
    }

    #[test]
    fn subclass_rules_apply() {
        let berserker = "dnd2024:subclasses/barbarian_path_of_the_berserker";
        let selected = |level| Selection {
            subclass: Some(berserker),
            ..selection("barbarian", level, true)
        };
        let early = build(&[selected(2)], 0).unwrap();
        assert_eq!(early.classes[0].problems.len(), 1);
        let features = |level| {
            build(&[selected(level)], 0)
                .unwrap()
                .classes
                .remove(0)
                .features
        };
        assert!(
            features(3)
                .iter()
                .any(|f| f.key.ends_with("level_3_frenzy"))
        );
        assert!(
            !features(5)
                .iter()
                .any(|f| f.key.ends_with("level_6_mindless_rage"))
        );
        assert!(
            features(6)
                .iter()
                .any(|f| f.key.ends_with("level_6_mindless_rage"))
        );
        let wrong = Selection {
            subclass: Some(berserker),
            ..selection("wizard", 3, true)
        };
        assert!(build(&[wrong], 0).is_err());
    }

    #[test]
    fn feat_prerequisites_are_enforced() {
        let at = |level| FeatContext {
            level,
            scores: BTreeMap::from([("strength".to_owned(), 12), ("dexterity".to_owned(), 13)]),
            features: vec!["spellcasting"],
        };
        let improvement = "dnd2024:feats/general_feats_ability_score_improvement";
        assert!(feat_allowed(improvement, &at(3)).is_err());
        assert!(feat_allowed(improvement, &at(4)).is_ok());
        let grappler = "dnd2024:feats/general_feats_grappler";
        assert!(feat_allowed(grappler, &at(4)).is_ok());
        let mut weak = at(4);
        weak.scores.remove("dexterity");
        assert!(feat_allowed(grappler, &weak).is_err());
        let archery = "dnd2024:feats/fighting_style_feats_archery";
        assert!(feat_allowed(archery, &at(4)).is_err());
        let recall = "dnd2024:feats/epic_boon_feats_boon_of_spell_recall";
        assert!(feat_allowed(recall, &at(18)).is_err());
        assert!(feat_allowed(recall, &at(19)).is_ok());
    }

    #[test]
    fn barbarian_mastery_accepts_melee_weapons_only() {
        let feature = "dnd2024:class_features/barbarian_level_1_weapon_mastery";
        assert!(mastery_allows(feature, "dnd2024:equipment/greataxe").unwrap());
        assert!(!mastery_allows(feature, "dnd2024:equipment/longbow").unwrap());
        let rogue = "dnd2024:class_features/rogue_level_1_weapon_mastery";
        assert!(mastery_allows(rogue, "dnd2024:equipment/longbow").unwrap());
    }

    #[test]
    fn rage_takes_its_uses_from_the_rages_column() {
        assert_eq!(
            resource_column("barbarian", "dnd2024:class_features/barbarian_level_1_rage")
                .unwrap()
                .as_deref(),
            Some("rages")
        );
    }

    #[test]
    fn column_rules_reach_the_result() {
        let barbarian = single("barbarian", 1).classes.remove(0);
        assert_eq!(
            barbarian.values["rage_damage"].requires.as_deref(),
            Some("rage_feature_eligibility")
        );
        assert!(!barbarian.values.contains_key("proficiency_bonus"));
        let monk = single("monk", 2).classes.remove(0);
        assert_eq!(
            monk.values["unarmored_movement"].unit.as_deref(),
            Some("ft")
        );
        let warlock = single("warlock", 1).classes.remove(0);
        assert_eq!(
            warlock.selection_capacities["cantrips"]
                .eligibility
                .as_deref(),
            Some("owning_feature_contract")
        );
    }

    #[test]
    fn a_later_level_can_add_a_weapon_mastery_choice() {
        let change = |previous_level, level| {
            let selection = Selection {
                class: "barbarian",
                level,
                previous_level,
                subclass: None,
                starting: true,
            };
            build(&[selection], 0).unwrap().classes.remove(0).choices
        };
        let mastery = |choices: &[ChoiceSlot]| {
            choices
                .iter()
                .find(|c| c.kind == ChoiceKind::WeaponMastery)
                .map(|c| (c.count, c.at_level))
        };
        assert_eq!(mastery(&change(0, 3)), Some((2, 1)));
        assert_eq!(mastery(&change(3, 4)), Some((1, 4)));
        assert_eq!(mastery(&change(4, 9)), None);
        // Each level that adds a weapon keeps its own level.
        let levels: Vec<(u32, u32)> = change(3, 10)
            .iter()
            .filter(|c| c.kind == ChoiceKind::WeaponMastery)
            .map(|c| (c.count, c.at_level))
            .collect();
        assert_eq!(levels, vec![(1, 4), (1, 10)]);
    }

    #[test]
    fn a_subclass_chosen_late_grants_what_its_level_allows() {
        let berserker = "dnd2024:subclasses/barbarian_path_of_the_berserker";
        let features = subclass_features(berserker, 6).unwrap();
        assert_eq!(features.len(), 2, "{features:?}");
        assert!(features.iter().any(|f| f.key.ends_with("level_3_frenzy")));
        assert!(
            features
                .iter()
                .any(|f| f.key.ends_with("level_6_mindless_rage"))
        );
        assert!(subclass_features("dnd2024:subclasses/unknown", 6).is_err());
    }

    /// Both paths to a subclass's features agree: choosing late, and reaching the level.
    #[test]
    fn every_subclass_grants_the_same_features_by_both_paths() {
        let catalogue = catalogue().unwrap();
        for (key, rules) in &catalogue.subclasses {
            for level in [3, 6, 20] {
                let late: Vec<String> = subclass_features(key, level)
                    .unwrap()
                    .into_iter()
                    .map(|feature| feature.key)
                    .collect();
                assert!(!late.is_empty(), "{key}");
                let selection = Selection {
                    class: &rules.class,
                    level,
                    previous_level: 0,
                    subclass: Some(key),
                    starting: true,
                };
                let built = build(&[selection], 0).unwrap().classes.remove(0);
                let granted: Vec<&str> = built.features.iter().map(|f| f.key.as_str()).collect();
                for feature in &late {
                    assert!(granted.contains(&feature.as_str()), "{key}: {feature}");
                }
                let plain = Selection {
                    subclass: None,
                    ..selection
                };
                let without = build(&[plain], 0).unwrap().classes.remove(0).features;
                assert_eq!(granted.len(), without.len() + late.len(), "{key} {level}");
            }
        }
    }

    /// A listed contract may ask only for inputs and outputs this module handles. If a contract
    /// needs more, such as inventory changes, it cannot be on the implemented list.
    #[test]
    fn implemented_contracts_ask_only_for_what_this_module_handles() {
        #[derive(Deserialize)]
        struct Registry {
            contracts: Vec<Contract>,
        }
        #[derive(Deserialize)]
        struct Contract {
            id: String,
            inputs: BTreeMap<String, serde_json::Value>,
            outputs: Vec<String>,
            events: Vec<String>,
        }
        let registry: Registry = serde_json::from_str(include_str!(
            "../../builtin/dnd2024-definitions/contracts.json"
        ))
        .unwrap();
        let mut seen = 0;
        for contract in registry.contracts {
            if !implemented().contains(&contract.id) {
                continue;
            }
            seen += 1;
            let (inputs, outputs, events): (&[&str], &[&str], &[&str]) =
                if contract.id.starts_with("dnd2024:class_progression_") {
                    (
                        // `choices` is optional. Eligibility belongs to each owning feature.
                        &[
                            "actor",
                            "class_level",
                            "previous_class_level",
                            "selected_subclass",
                            "choices",
                        ],
                        &[
                            "feature_grants",
                            "selection_capacities",
                            "resource_capacities",
                            "class_values",
                        ],
                        &["character.class_level_changed"],
                    )
                } else if contract.id.starts_with("dnd2024:subclasses_") {
                    (
                        &["actor", "selection"],
                        &["feature_grants", "feature_state"],
                        &[
                            "subclass.selected",
                            "character.level_gained",
                            "character.subclass_selected",
                            "character.class_level_gained",
                        ],
                    )
                } else {
                    assert!(contract.id.ends_with("_subclass"), "{}", contract.id);
                    (
                        &["actor", "selection", "selections"],
                        &["feature_state", "choices", "subclass", "feature_grants"],
                        &[
                            "subclass.selected",
                            "character.level_gained",
                            "character.feature_gained",
                        ],
                    )
                };
            for input in contract.inputs.keys() {
                assert!(inputs.contains(&input.as_str()), "{}: {input}", contract.id);
            }
            for output in &contract.outputs {
                assert!(
                    outputs.contains(&output.as_str()),
                    "{}: {output}",
                    contract.id
                );
            }
            for event in &contract.events {
                assert!(events.contains(&event.as_str()), "{}: {event}", contract.id);
            }
        }
        assert_eq!(seen, implemented().len());
    }
}
