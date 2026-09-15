---
splat_schema: 1
splat_id: f4823d01-8fa3-5e60-9f72-1af3248f7d9f
splat_kind: rule_group
splat_key: dnd2024:classes/paladin
splat_name: Paladin
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
---

# Paladin

## Source text

<!-- SRD page 53 -->

Core Paladin Traits\
Primary Ability Strength and Charisma\
Hit Point Die D10 per Paladin level\
Saving Throw Wisdom and Charisma\
Proficiencies\
Skill Proficiencies Choose 2: Athletics, Insight,\
Intimidation, Medicine,\
Persuasion, or Religion\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light, Medium, and Heavy ar-\
mor and Shields\
Starting Equipment Choose A or B: (A) Chain Mail,\
Shield, Longsword, 6 Javelins,\
Holy Symbol, Priest’s Pack,\
and 9 GP; or (B) 150 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/paladin ` |
| ` implementation ` | ` dnd2024:classes_paladin ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` primary_abilities ` | ` ["strength", "charisma"] ` |

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
- wisdom
- charisma
skill_proficiencies:
  count: 2
  choices:
  - athletics
  - insight
  - intimidation
  - medicine
  - persuasion
  - religion
weapon_proficiencies:
  simple: true
  martial: true
armor_training:
- light
- medium
- heavy
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
        key: dnd2024:equipment/chain_mail
      quantity: 1
    - definition:
        key: dnd2024:equipment/shield
      quantity: 1
    - definition:
        key: dnd2024:equipment/longsword
      quantity: 1
    - definition:
        key: dnd2024:equipment/javelin
      quantity: 6
    - definition:
        key: dnd2024:equipment/holy_symbol_varies
      quantity: 1
    - definition:
        key: dnd2024:equipment/priest_s_pack_33_gp
      quantity: 1
    currency:
      gp: 9
  currency:
    currency:
      gp: 150
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/paladin_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 11 ` |
| ` 2 ` | ` 14 ` |
| ` 3 ` | ` 18 ` |
| ` 4 ` | ` 19 ` |
| ` 5 ` | ` 1 ` |
| ` 6 ` | ` 1 ` |
| ` 7 ` | ` 1 ` |
| ` 8 ` | ` 2 ` |
| ` 9 ` | ` 2 ` |
| ` 10 ` | ` 3 ` |
| ` 11 ` | ` 3 ` |
| ` 12 ` | ` 4 ` |
| ` 13 ` | ` 5 ` |
| ` 14 ` | ` 5 ` |
| ` 15 ` | ` 6 ` |
| ` 16 ` | ` 9 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_10_aura_of_courage ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_11_radiant_strikes ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_14_restoring_touch ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_18_aura_expansion ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_19_epic_boon ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/5/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_1_lay_on_hands ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/6/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_1_spellcasting ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/7/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_1_weapon_mastery ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/8/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_2_fighting_style ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/9/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_2_paladin_s_smite ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/10/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_3_channel_divinity ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/11/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_3_paladin_subclass ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/12/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_4_ability_score_improvement ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/13/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_5_extra_attack ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/14/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_5_faithful_steed ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/15/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_6_aura_of_protection ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/16/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_9_abjure_foes ` |
