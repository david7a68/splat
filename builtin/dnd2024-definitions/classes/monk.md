---
splat_schema: 1
splat_id: d2158bed-a30b-5114-8de1-f38d011610c0
splat_kind: rule_group
splat_key: dnd2024:classes/monk
splat_name: Monk
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
---

# Monk

## Source text

<!-- SRD page 49 -->

Core Monk Traits\
Primary Ability Dexterity and Wisdom\
Hit Point Die D8 per Monk level\
Saving Throw Strength and Dexterity\
Proficiencies\
Skill Proficiencies Choose 2: Acrobatics, Athlet-\
ics, History, Insight, Religion,\
or Stealth\
Weapon Proficiencies Simple weapons and Martial\
weapons that have the Light\
property\
Tool Proficiencies Choose one type of Artisan’s\
Tools or Musical Instrument (see\
“Equipment”)\
Armor Training None\
Starting Equipment Choose A or B: (A) Spear, 5\
Daggers, Artisan’s Tools or\
Musical Instrument chosen\
for the tool proficiency above,\
Explorer’s Pack, and 11 GP; or\
(B) 50 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/monk ` |
| ` implementation ` | ` dnd2024:classes_monk ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` primary_abilities ` | ` ["dexterity", "wisdom"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` count_per_class_level ` | ` 1 ` |
| ` sides ` | ` 8 ` |

#### First class traits

<!-- splat:yaml path=splat_rule/parameters/first_class_traits -->

```yaml
save_proficiencies:
- strength
- dexterity
skill_proficiencies:
  count: 2
  choices:
  - acrobatics
  - athletics
  - history
  - insight
  - religion
  - stealth
weapon_proficiencies:
  simple: true
  martial_required_property_any:
  - light
armor_training: []
tool_proficiency_choice:
  count: 1
  categories:
  - artisans_tools
  - musical_instrument
```

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
choose: 1
options:
  equipment:
    items:
    - definition:
        key: dnd2024:equipment/spear
      quantity: 1
    - definition:
        key: dnd2024:equipment/dagger
      quantity: 5
    - definition:
        key: dnd2024:equipment/explorer_s_pack_10_gp
      quantity: 1
    currency:
      gp: 11
    matching_proficient_tool: true
  currency:
    currency:
      gp: 50
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/monk_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 10 ` |
| ` 2 ` | ` 13 ` |
| ` 3 ` | ` 14 ` |
| ` 4 ` | ` 15 ` |
| ` 5 ` | ` 18 ` |
| ` 6 ` | ` 19 ` |
| ` 7 ` | ` 1 ` |
| ` 8 ` | ` 1 ` |
| ` 9 ` | ` 20 ` |
| ` 10 ` | ` 2 ` |
| ` 11 ` | ` 2 ` |
| ` 12 ` | ` 2 ` |
| ` 13 ` | ` 3 ` |
| ` 14 ` | ` 3 ` |
| ` 15 ` | ` 4 ` |
| ` 16 ` | ` 4 ` |
| ` 17 ` | ` 5 ` |
| ` 18 ` | ` 5 ` |
| ` 19 ` | ` 6 ` |
| ` 20 ` | ` 7 ` |
| ` 21 ` | ` 9 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_heightened_focus ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_self_restoration ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_13_deflect_energy ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_14_disciplined_survivor ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_15_perfect_focus ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/5/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_18_superior_defense ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/6/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_19_epic_boon ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/7/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_martial_arts ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/8/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_unarmored_defense ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/9/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_20_body_and_mind ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/10/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_monk_s_focus ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/11/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_unarmored_movement ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/12/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_uncanny_metabolism ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/13/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_deflect_attacks ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/14/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_monk_subclass ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/15/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/16/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_slow_fall ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/17/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_5_extra_attack ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/18/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_5_stunning_strike ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/19/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_6_empowered_strikes ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/20/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_7_evasion ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/21/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_9_acrobatic_movement ` |
