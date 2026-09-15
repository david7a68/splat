---
splat_schema: 1
splat_id: efbd366f-30e0-509f-b4e7-8ced200babb5
splat_kind: rule_group
splat_key: dnd2024:classes/druid
splat_name: Druid
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
---

# Druid

## Source text

<!-- SRD page 41 -->

Core Druid Traits\
Primary Ability Wisdom\
Hit Point Die D8 per Druid level\
Saving Throw Intelligence and Wisdom\
Proficiencies\
Skill Proficiencies Choose 2: Animal Handling,\
Arcana, Insight, Medicine, Na-\
ture, Perception, Religion, or\
Survival\
Weapon Proficiencies Simple weapons\
Tool Proficiencies Herbalism Kit\
Armor Training Light armor and Shields\
Starting Equipment Choose A or B: (A) Leather Ar-\
mor, Shield, Sickle, Druidic Fo-\
cus (Quarterstaff), Explorer’s\
Pack, Herbalism Kit, and 9 GP;\
or (B) 50 GP\
Druid Features\
Proficiency Wild\
Level Bonus Class Features Shape\
1 +2 Spellcasting, Druidic, —\
Primal Order\
2 +2 Wild Shape, 2\
Wild Companion\
3 +2 Druid Subclass 2\
4 +2 Ability Score Improvement 2\
5 +3 Wild Resurgence 2\
6 +3 Subclass feature 3\
7 +3 Elemental Fury 3\
8 +3 Ability Score Improvement 3\
9 +4 — 3\
10 +4 Subclass feature 3\
11 +4 — 3\
12 +4 Ability Score Improvement 3\
13 +5 — 3\
14 +5 Subclass feature 3\
15 +5 Improved Elemental Fury 3\
16 +5 Ability Score Improvement 3\
17 +6 — 4\
18 +6 Beast Spells 4\
19 +6 Epic Boon 4\
20 +6 Archdruid 4\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/druid_level_15_improved_elemental_fury ` |
| ` 1 ` | ` dnd2024:class_features/druid_level_18_beast_spells ` |
| ` 2 ` | ` dnd2024:class_features/druid_level_19_epic_boon ` |
| ` 3 ` | ` dnd2024:class_features/druid_level_1_druidic ` |
| ` 4 ` | ` dnd2024:class_features/druid_level_1_primal_order ` |
| ` 5 ` | ` dnd2024:class_features/druid_level_1_spellcasting ` |
| ` 6 ` | ` dnd2024:class_features/druid_level_20_archdruid ` |
| ` 7 ` | ` dnd2024:class_features/druid_level_2_wild_companion ` |
| ` 8 ` | ` dnd2024:class_features/druid_level_2_wild_shape ` |
| ` 9 ` | ` dnd2024:class_features/druid_level_3_druid_subclass ` |
| ` 10 ` | ` dnd2024:class_features/druid_level_4_ability_score_improvement ` |
| ` 11 ` | ` dnd2024:class_features/druid_level_5_wild_resurgence ` |
| ` 12 ` | ` dnd2024:class_features/druid_level_7_elemental_fury ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/druid ` |
| ` implementation ` | ` dnd2024:classes_druid ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` primary_abilities ` | ` ["wisdom"] ` |
| ` saving_throw_proficiencies ` | ` ["intelligence", "wisdom"] ` |
| ` weapon_proficiencies ` | ` ["simple"] ` |
| ` armor_training ` | ` ["light", "shield"] ` |
| ` tool_proficiencies ` | ` ["herbalism_kit"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 8 ` |
| ` per_class_level ` | ` 1 ` |

#### Skill proficiencies

<!-- splat:values path=splat_rule/parameters/skill_proficiencies -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` options ` | ` ["animal_handling", "arcana", "insight", "medicine", "nature", "perception", "religion", "survival"] ` |

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
count: 1
options:
- id: a
  grants:
  - item_type: leather_armor
    quantity: 1
  - item_type: shield
    quantity: 1
  - item_type: sickle
    quantity: 1
  - item_type: druidic_focus_quarterstaff
    quantity: 1
  - item_type: explorer_s_pack
    quantity: 1
  - item_type: herbalism_kit
    quantity: 1
  - item_type: gp
    quantity: 9
- id: b
  grants:
  - item_type: gp
    quantity: 50
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/druid_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 15 ` |
| ` 1 ` | ` 18 ` |
| ` 2 ` | ` 19 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 1 ` |
| ` 5 ` | ` 1 ` |
| ` 6 ` | ` 20 ` |
| ` 7 ` | ` 2 ` |
| ` 8 ` | ` 2 ` |
| ` 9 ` | ` 3 ` |
| ` 10 ` | ` 4 ` |
| ` 11 ` | ` 5 ` |
| ` 12 ` | ` 7 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_15_improved_elemental_fury ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_18_beast_spells ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_1_druidic ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_1_primal_order ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_20_archdruid ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_2_wild_companion ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_2_wild_shape ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_3_druid_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/11/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_5_wild_resurgence ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/12/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/druid_level_7_elemental_fury ` |
