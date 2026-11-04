---
splat_schema: 2
splat_id: 87bf41fd-c017-5c9e-aeb8-4307c9489216
splat_kind: rule_group
splat_key: dnd2024:classes/barbarian
splat_name: Barbarian
splat_origin: dnd2024:reference/pages/028
splat_sources:
- SRD 5.2.1, pp. 28, https://www.dndbeyond.com/srd
---

# Barbarian

## Source text

<!-- SRD page 28 -->

**Core Barbarian Traits**

|  |  |
| --- | --- |
| Primary Ability | Strength |
| Hit Point Die | D12 per Barbarian level |
| Saving Throw | Strength and Constitution |
| Proficiencies |  |
| Skill Proficiencies | Choose 2: Animal Handling, Athletics, Intimidation, Nature, Perception, or Survival |
| Weapon Proficiencies | Simple and Martial weapons |
| Armor Training | Light and Medium armor and Shields |
| Starting Equipment | Choose A or B: (A) Greataxe, 4 Handaxes, Explorer’s Pack, and 15 GP; or (B) 75 GP |

**Barbarian Features**

| Level | Proficiency Bonus | Class Features | Rages | Rage Damage | Weapon Mastery |
| --- | --- | --- | --- | --- | --- |
| 1 | +2 | Rage, Unarmored Defense, Weapon Mastery | 2 | +2 | 2 |
| 2 | +2 | Danger Sense, Reckless Attack | 2 | +2 | 2 |
| 3 | +2 | Barbarian Subclass, Primal Knowledge | 3 | +2 | 2 |
| 4 | +2 | Ability Score Improvement | 3 | +2 | 3 |
| 5 | +3 | Extra Attack, Fast Movement | 3 | +2 | 3 |
| 6 | +3 | Subclass feature | 4 | +2 | 3 |
| 7 | +3 | Feral Instinct, Instinctive Pounce | 4 | +2 | 3 |
| 8 | +3 | Ability Score Improvement | 4 | +2 | 3 |
| 9 | +4 | Brutal Strike | 4 | +3 | 3 |
| 10 | +4 | Subclass feature | 4 | +3 | 4 |
| 11 | +4 | Relentless Rage | 4 | +3 | 4 |
| 12 | +4 | Ability Score Improvement | 5 | +3 | 4 |
| 13 | +5 | Improved Brutal Strike | 5 | +3 | 4 |
| 14 | +5 | Subclass feature | 5 | +3 | 4 |
| 15 | +5 | Persistent Rage | 5 | +3 | 4 |
| 16 | +5 | Ability Score Improvement | 5 | +4 | 4 |
| 17 | +6 | Improved Brutal Strike | 6 | +4 | 4 |
| 18 | +6 | Indomitable Might | 6 | +4 | 4 |
| 19 | +6 | Epic Boon | 6 | +4 | 4 |
| 20 | +6 | Primal Champion | 6 | +4 | 4 |

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/barbarian_level_11_relentless_rage ` |
| ` 1 ` | ` dnd2024:class_features/barbarian_level_13_improved_brutal_strike ` |
| ` 2 ` | ` dnd2024:class_features/barbarian_level_15_persistent_rage ` |
| ` 3 ` | ` dnd2024:class_features/barbarian_level_17_improved_brutal_strike ` |
| ` 4 ` | ` dnd2024:class_features/barbarian_level_18_indomitable_might ` |
| ` 5 ` | ` dnd2024:class_features/barbarian_level_19_epic_boon ` |
| ` 6 ` | ` dnd2024:class_features/barbarian_level_1_rage ` |
| ` 7 ` | ` dnd2024:class_features/barbarian_level_1_unarmored_defense ` |
| ` 8 ` | ` dnd2024:class_features/barbarian_level_1_weapon_mastery ` |
| ` 9 ` | ` dnd2024:class_features/barbarian_level_20_primal_champion ` |
| ` 10 ` | ` dnd2024:class_features/barbarian_level_2_danger_sense ` |
| ` 11 ` | ` dnd2024:class_features/barbarian_level_2_reckless_attack ` |
| ` 12 ` | ` dnd2024:class_features/barbarian_level_3_barbarian_subclass ` |
| ` 13 ` | ` dnd2024:class_features/barbarian_level_3_primal_knowledge ` |
| ` 14 ` | ` dnd2024:class_features/barbarian_level_4_ability_score_improvement ` |
| ` 15 ` | ` dnd2024:class_features/barbarian_level_5_extra_attack ` |
| ` 16 ` | ` dnd2024:class_features/barbarian_level_5_fast_movement ` |
| ` 17 ` | ` dnd2024:class_features/barbarian_level_7_feral_instinct ` |
| ` 18 ` | ` dnd2024:class_features/barbarian_level_7_instinctive_pounce ` |
| ` 19 ` | ` dnd2024:class_features/barbarian_level_9_brutal_strike ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/barbarian ` |
| ` implementation ` | ` dnd2024:classes_barbarian ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` primary_abilities ` | ` ["strength"] ` |
| ` saving_throw_proficiencies ` | ` ["strength", "constitution"] ` |
| ` weapon_proficiencies ` | ` ["simple", "martial"] ` |
| ` armor_training ` | ` ["light", "medium", "shield"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 12 ` |
| ` per_class_level ` | ` 1 ` |

#### Skill proficiencies

<!-- splat:values path=splat_rule/parameters/skill_proficiencies -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` options ` | ` ["animal_handling", "athletics", "intimidation", "nature", "perception", "survival"] ` |

#### Tool proficiencies

<!-- splat:yaml path=splat_rule/parameters/tool_proficiencies -->

```yaml
[]
```

#### Starting equipment

<!-- splat:values path=splat_rule/parameters/starting_equipment -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options
key: Variant
order:
- a
- b
columns:
  Variant:
    path: id
    type: symbol
  Details:
    type: section
    optional: true
-->

| Variant | Details |
| --- | --- |
| ` A ` | [Rules](#^splat-93071aae-a "section:a") |
| ` B ` | [Rules](#^splat-93071aae-b "section:b") |

###### A

^splat-93071aae-a

###### Grants

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options/@a/grants
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Item type:
    path: item_type
    type: symbol
  Quantity:
    path: quantity
    type: integer
-->

| Key | Item type | Quantity |
| --- | --- | --- |
| ` row_1 ` | ` Greataxe ` | ` 1 ` |
| ` row_2 ` | ` Handaxe ` | ` 4 ` |
| ` row_3 ` | ` Explorer s pack ` | ` 1 ` |
| ` row_4 ` | ` Gp ` | ` 15 ` |

###### B

^splat-93071aae-b

###### Grants

<!-- splat:rows path=splat_rule/parameters/starting_equipment/options/@b/grants -->

| Row | item_type | quantity |
| --- | --- | --- |
| ` 0 ` | ` gp ` | ` 75 ` |

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/barbarian_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- barbarian_level_11_relentless_rage
- barbarian_level_13_improved_brutal_strike
- barbarian_level_15_persistent_rage
- barbarian_level_17_improved_brutal_strike
- barbarian_level_18_indomitable_might
- barbarian_level_19_epic_boon
- barbarian_level_1_rage
- barbarian_level_1_unarmored_defense
- barbarian_level_1_weapon_mastery
- barbarian_level_20_primal_champion
- barbarian_level_2_danger_sense
- barbarian_level_2_reckless_attack
- barbarian_level_3_barbarian_subclass
- barbarian_level_3_primal_knowledge
- barbarian_level_4_ability_score_improvement
- barbarian_level_5_extra_attack
- barbarian_level_5_fast_movement
- barbarian_level_7_feral_instinct
- barbarian_level_7_instinctive_pounce
- barbarian_level_9_brutal_strike
columns:
  Feature:
    path: feature
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Feature | Minimum class level |
| --- | --- |
| [Level 11: Relentless Rage](<../class_features/barbarian_level_11_relentless_rage.md> "dnd2024:class_features/barbarian_level_11_relentless_rage") | ` 11 ` |
| [Level 13: Improved Brutal Strike](<../class_features/barbarian_level_13_improved_brutal_strike.md> "dnd2024:class_features/barbarian_level_13_improved_brutal_strike") | ` 13 ` |
| [Level 15: Persistent Rage](<../class_features/barbarian_level_15_persistent_rage.md> "dnd2024:class_features/barbarian_level_15_persistent_rage") | ` 15 ` |
| [Level 17: Improved Brutal Strike](<../class_features/barbarian_level_17_improved_brutal_strike.md> "dnd2024:class_features/barbarian_level_17_improved_brutal_strike") | ` 17 ` |
| [Level 18: Indomitable Might](<../class_features/barbarian_level_18_indomitable_might.md> "dnd2024:class_features/barbarian_level_18_indomitable_might") | ` 18 ` |
| [Level 19: Epic Boon](<../class_features/barbarian_level_19_epic_boon.md> "dnd2024:class_features/barbarian_level_19_epic_boon") | ` 19 ` |
| [Level 1: Rage](<../class_features/barbarian_level_1_rage.md> "dnd2024:class_features/barbarian_level_1_rage") | ` 1 ` |
| [Level 1: Unarmored Defense](<../class_features/barbarian_level_1_unarmored_defense.md> "dnd2024:class_features/barbarian_level_1_unarmored_defense") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/barbarian_level_1_weapon_mastery.md> "dnd2024:class_features/barbarian_level_1_weapon_mastery") | ` 1 ` |
| [Level 20: Primal Champion](<../class_features/barbarian_level_20_primal_champion.md> "dnd2024:class_features/barbarian_level_20_primal_champion") | ` 20 ` |
| [Level 2: Danger Sense](<../class_features/barbarian_level_2_danger_sense.md> "dnd2024:class_features/barbarian_level_2_danger_sense") | ` 2 ` |
| [Level 2: Reckless Attack](<../class_features/barbarian_level_2_reckless_attack.md> "dnd2024:class_features/barbarian_level_2_reckless_attack") | ` 2 ` |
| [Level 3: Barbarian Subclass](<../class_features/barbarian_level_3_barbarian_subclass.md> "dnd2024:class_features/barbarian_level_3_barbarian_subclass") | ` 3 ` |
| [Level 3: Primal Knowledge](<../class_features/barbarian_level_3_primal_knowledge.md> "dnd2024:class_features/barbarian_level_3_primal_knowledge") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/barbarian_level_4_ability_score_improvement.md> "dnd2024:class_features/barbarian_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Extra Attack](<../class_features/barbarian_level_5_extra_attack.md> "dnd2024:class_features/barbarian_level_5_extra_attack") | ` 5 ` |
| [Level 5: Fast Movement](<../class_features/barbarian_level_5_fast_movement.md> "dnd2024:class_features/barbarian_level_5_fast_movement") | ` 5 ` |
| [Level 7: Feral Instinct](<../class_features/barbarian_level_7_feral_instinct.md> "dnd2024:class_features/barbarian_level_7_feral_instinct") | ` 7 ` |
| [Level 7: Instinctive Pounce](<../class_features/barbarian_level_7_instinctive_pounce.md> "dnd2024:class_features/barbarian_level_7_instinctive_pounce") | ` 7 ` |
| [Level 9: Brutal Strike](<../class_features/barbarian_level_9_brutal_strike.md> "dnd2024:class_features/barbarian_level_9_brutal_strike") | ` 9 ` |
