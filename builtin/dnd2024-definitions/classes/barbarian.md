---
splat_schema: 1
splat_id: 87bf41fd-c017-5c9e-aeb8-4307c9489216
splat_kind: rule_group
splat_key: dnd2024:classes/barbarian
splat_name: Barbarian
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28
  url: https://www.dndbeyond.com/srd
---

# Barbarian

## Source text

<!-- SRD page 28 -->

Core Barbarian Traits\
Primary Ability Strength\
Hit Point Die D12 per Barbarian level\
Saving Throw Strength and Constitution\
Proficiencies\
Skill Proficiencies Choose 2: Animal Handling,\
Athletics, Intimidation, Na-\
ture, Perception, or Survival\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light and Medium armor and\
Shields\
Starting Equipment Choose A or B: (A) Greataxe,\
4 Handaxes, Explorer’s Pack,\
and 15 GP; or (B) 75 GP\

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

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
count: 1
options:
- id: a
  grants:
  - item_type: greataxe
    quantity: 1
  - item_type: handaxe
    quantity: 4
  - item_type: explorer_s_pack
    quantity: 1
  - item_type: gp
    quantity: 15
- id: b
  grants:
  - item_type: gp
    quantity: 75
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/barbarian_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 11 ` |
| ` 1 ` | ` 13 ` |
| ` 2 ` | ` 15 ` |
| ` 3 ` | ` 17 ` |
| ` 4 ` | ` 18 ` |
| ` 5 ` | ` 19 ` |
| ` 6 ` | ` 1 ` |
| ` 7 ` | ` 1 ` |
| ` 8 ` | ` 1 ` |
| ` 9 ` | ` 20 ` |
| ` 10 ` | ` 2 ` |
| ` 11 ` | ` 2 ` |
| ` 12 ` | ` 3 ` |
| ` 13 ` | ` 3 ` |
| ` 14 ` | ` 4 ` |
| ` 15 ` | ` 5 ` |
| ` 16 ` | ` 5 ` |
| ` 17 ` | ` 7 ` |
| ` 18 ` | ` 7 ` |
| ` 19 ` | ` 9 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_11_relentless_rage ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_13_improved_brutal_strike ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_15_persistent_rage ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_17_improved_brutal_strike ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_18_indomitable_might ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_1_rage ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_1_unarmored_defense ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_1_weapon_mastery ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_20_primal_champion ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_2_danger_sense ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/11/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_2_reckless_attack ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/12/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_3_barbarian_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/13/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_3_primal_knowledge ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/14/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/15/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_5_extra_attack ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/16/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_5_fast_movement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/17/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_7_feral_instinct ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/18/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_7_instinctive_pounce ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/19/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/barbarian_level_9_brutal_strike ` |
