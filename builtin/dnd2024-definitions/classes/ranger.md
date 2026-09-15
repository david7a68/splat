---
splat_schema: 1
splat_id: aed521b4-3fb7-58f7-a35e-bc8ab5848eef
splat_kind: rule_group
splat_key: dnd2024:classes/ranger
splat_name: Ranger
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
---

# Ranger

## Source text

<!-- SRD page 57 -->

Core Ranger Traits\
Primary Ability Dexterity and Wisdom\
Hit Point Die D10 per Ranger level\
Saving Throw Strength and Dexterity\
Proficiencies\
Skill Proficiencies Choose 3: Animal Handling,\
Athletics, Insight, Investigation,\
Nature, Perception, Stealth, or\
Survival\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light and Medium armor and\
Shields\
Starting Equipment Choose A or B: (A) Studded\
Leather Armor, Scimitar,\
Shortsword, Longbow, 20 Ar-\
rows, Quiver, Druidic Focus\
(sprig of mistletoe), Explorer’s\
Pack, and 7 GP; or (B) 150 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/ranger ` |
| ` implementation ` | ` dnd2024:classes_ranger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` primary_abilities ` | ` ["dexterity", "wisdom"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` count_per_class_level ` | ` 1 ` |
| ` sides ` | ` 10 ` |

#### First class traits

<!-- splat:yaml path=splat_rule/parameters/first_class_traits -->

```yaml
save_proficiencies:
- strength
- dexterity
skill_proficiencies:
  count: 3
  choices:
  - animal_handling
  - athletics
  - insight
  - investigation
  - nature
  - perception
  - stealth
  - survival
weapon_proficiencies:
  simple: true
  martial: true
armor_training:
- light
- medium
- shield
```

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
choose: 1
options:
  equipment:
    items:
    - definition:
        key: dnd2024:equipment/studded_leather_armor
      quantity: 1
    - definition:
        key: dnd2024:equipment/scimitar
      quantity: 1
    - definition:
        key: dnd2024:equipment/shortsword
      quantity: 1
    - definition:
        key: dnd2024:equipment/longbow
      quantity: 1
    - definition:
        key: dnd2024:equipment/ammunition_varies
      quantity: 20
      variant: arrows
    - definition:
        key: dnd2024:equipment/quiver_1_gp
      quantity: 1
    - definition:
        key: dnd2024:equipment/druidic_focus_varies
      quantity: 1
      variant: sprig_of_mistletoe
    - definition:
        key: dnd2024:equipment/explorer_s_pack_10_gp
      quantity: 1
    currency:
      gp: 7
  currency:
    currency:
      gp: 150
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/ranger_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 13 ` |
| ` 2 ` | ` 14 ` |
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
| ` 13 ` | ` 4 ` |
| ` 14 ` | ` 5 ` |
| ` 15 ` | ` 6 ` |
| ` 16 ` | ` 9 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_10_tireless ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_13_relentless_hunter ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_14_nature_s_veil ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_17_precise_hunter ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_18_feral_senses ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/5/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_19_epic_boon ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/6/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_1_favored_enemy ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/7/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_1_spellcasting ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/8/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_1_weapon_mastery ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/9/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_20_foe_slayer ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/10/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_2_deft_explorer ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/11/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_2_fighting_style ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/12/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_3_ranger_subclass ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/13/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_4_ability_score_improvement ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/14/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_5_extra_attack ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/15/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_6_roving ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/16/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/ranger_level_9_expertise ` |
