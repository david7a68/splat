---
splat_schema: 1
splat_id: bdda2fb9-a1ef-5e7b-a9bf-b25a82d74042
splat_kind: rule_group
splat_key: dnd2024:classes/cleric
splat_name: Cleric
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
---

# Cleric

## Source text

<!-- SRD page 36 -->

Core Cleric Traits\
Primary Ability Wisdom\
Hit Point Die D8 per Cleric level\
Saving Throw Wisdom and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: History, Insight,\
Medicine, Persuasion, or\
Religion\
Weapon Proficiencies Simple weapons\
Armor Training Light and Medium armor and\
Shields\
Starting Equipment Choose A or B: (A) Chain Shirt,\
Shield, Mace, Holy Symbol,\
Priest’s Pack, and 7 GP; or (B)\
110 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/cleric_level_10_divine_intervention ` |
| ` 1 ` | ` dnd2024:class_features/cleric_level_14_improved_blessed_strikes ` |
| ` 2 ` | ` dnd2024:class_features/cleric_level_19_epic_boon ` |
| ` 3 ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |
| ` 4 ` | ` dnd2024:class_features/cleric_level_1_spellcasting ` |
| ` 5 ` | ` dnd2024:class_features/cleric_level_20_greater_divine_intervention ` |
| ` 6 ` | ` dnd2024:class_features/cleric_level_2_channel_divinity ` |
| ` 7 ` | ` dnd2024:class_features/cleric_level_3_cleric_subclass ` |
| ` 8 ` | ` dnd2024:class_features/cleric_level_4_ability_score_improvement ` |
| ` 9 ` | ` dnd2024:class_features/cleric_level_5_sear_undead ` |
| ` 10 ` | ` dnd2024:class_features/cleric_level_7_blessed_strikes ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/cleric ` |
| ` implementation ` | ` dnd2024:classes_cleric ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` primary_abilities ` | ` ["wisdom"] ` |
| ` saving_throw_proficiencies ` | ` ["wisdom", "charisma"] ` |
| ` weapon_proficiencies ` | ` ["simple"] ` |
| ` armor_training ` | ` ["light", "medium", "shield"] ` |

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
| ` options ` | ` ["history", "insight", "medicine", "persuasion", "religion"] ` |

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
  - item_type: chain_shirt
    quantity: 1
  - item_type: shield
    quantity: 1
  - item_type: mace
    quantity: 1
  - item_type: holy_symbol
    quantity: 1
  - item_type: priest_s_pack
    quantity: 1
  - item_type: gp
    quantity: 7
- id: b
  grants:
  - item_type: gp
    quantity: 110
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/cleric_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 14 ` |
| ` 2 ` | ` 19 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 1 ` |
| ` 5 ` | ` 20 ` |
| ` 6 ` | ` 2 ` |
| ` 7 ` | ` 3 ` |
| ` 8 ` | ` 4 ` |
| ` 9 ` | ` 5 ` |
| ` 10 ` | ` 7 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_10_divine_intervention ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_14_improved_blessed_strikes ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_1_divine_order ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_20_greater_divine_intervention ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_2_channel_divinity ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_3_cleric_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_5_sear_undead ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/cleric_level_7_blessed_strikes ` |
