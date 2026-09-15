---
splat_schema: 1
splat_id: 507bda38-bdc0-529f-83be-3632e295489e
splat_kind: rule_group
splat_key: dnd2024:classes/rogue
splat_name: Rogue
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
---

# Rogue

## Source text

<!-- SRD page 61 -->

Core Rogue Traits\
Primary Ability Dexterity\
Hit Point Die D8 per Rogue level\
Saving Throw Dexterity and Intelligence\
Proficiencies\
Skill Proficiencies Choose 4: Acrobatics, Ath-\
letics, Deception, Insight,\
Intimidation, Investigation,\
Perception, Persuasion, Sleight\
of Hand, or Stealth\
Weapon Proficiencies Simple weapons and Martial\
weapons that have the Finesse\
or Light property\
Tool Proficiencies Thieves’ Tools\
Armor Training Light armor\
Starting Equipment Choose A or B: (A) Leather Ar-\
mor, 2 Daggers, Short sword,\
Shortbow, 20 Arrows, Quiver,\
Thieves’ Tools, Burglar’s Pack,\
and 8 GP; or (B) 100 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/rogue ` |
| ` implementation ` | ` dnd2024:classes_rogue ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` primary_abilities ` | ` ["dexterity"] ` |

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
- dexterity
- intelligence
skill_proficiencies:
  count: 4
  choices:
  - acrobatics
  - athletics
  - deception
  - insight
  - intimidation
  - investigation
  - perception
  - persuasion
  - sleight_of_hand
  - stealth
weapon_proficiencies:
  simple: true
  martial_required_property_any:
  - finesse
  - light
armor_training:
- light
tool_proficiencies:
- key: dnd2024:equipment/other_tools_thieves_tools_25_gp
```

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
choose: 1
options:
  equipment:
    items:
    - definition:
        key: dnd2024:equipment/leather_armor
      quantity: 1
    - definition:
        key: dnd2024:equipment/dagger
      quantity: 2
    - definition:
        key: dnd2024:equipment/shortsword
      quantity: 1
    - definition:
        key: dnd2024:equipment/shortbow
      quantity: 1
    - definition:
        key: dnd2024:equipment/ammunition_varies
      quantity: 20
      variant: arrows
    - definition:
        key: dnd2024:equipment/quiver_1_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/other_tools_thieves_tools_25_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/burglar_s_pack_16_gp
      quantity: 1
    currency:
      gp: 8
  currency:
    currency:
      gp: 100
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/rogue_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 11 ` |
| ` 1 ` | ` 14 ` |
| ` 2 ` | ` 15 ` |
| ` 3 ` | ` 18 ` |
| ` 4 ` | ` 19 ` |
| ` 5 ` | ` 1 ` |
| ` 6 ` | ` 1 ` |
| ` 7 ` | ` 1 ` |
| ` 8 ` | ` 1 ` |
| ` 9 ` | ` 20 ` |
| ` 10 ` | ` 2 ` |
| ` 11 ` | ` 3 ` |
| ` 12 ` | ` 3 ` |
| ` 13 ` | ` 4 ` |
| ` 14 ` | ` 5 ` |
| ` 15 ` | ` 5 ` |
| ` 16 ` | ` 7 ` |
| ` 17 ` | ` 7 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_11_improved_cunning_strike ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_14_devious_strikes ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_15_slippery_mind ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_18_elusive ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_19_epic_boon ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/5/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_expertise ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/6/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_sneak_attack ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/7/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_thieves_cant ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/8/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_weapon_mastery ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/9/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_20_stroke_of_luck ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/10/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_2_cunning_action ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/11/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_3_rogue_subclass ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/12/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_3_steady_aim ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/13/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_4_ability_score_improvement ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/14/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_5_cunning_strike ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/15/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_5_uncanny_dodge ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/16/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_7_evasion ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/17/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_7_reliable_talent ` |
