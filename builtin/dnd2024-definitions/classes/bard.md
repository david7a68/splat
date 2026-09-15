---
splat_schema: 1
splat_id: 99562e22-b8dc-5d70-be60-dbc025fba62f
splat_kind: rule_group
splat_key: dnd2024:classes/bard
splat_name: Bard
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
---

# Bard

## Source text

<!-- SRD page 31 -->

Core Bard Traits\
Primary Ability Charisma\
Hit Point Die D8 per Bard level\
Saving Throw Dexterity and Charisma\
Proficiencies\
Skill Proficiencies Choose any 3 skills (see “Playing\
the Game”)\
Weapon Proficiencies Simple weapons\
Tool Proficiencies Choose 3 Musical Instruments\
(see “Equipment”)\
Armor Training Light armor\
Starting Equipment Choose A or B: (A) Leather\
Armor, 2 Daggers, Musical\
Instrument of your choice, En-\
tertainer’s Pack, and 19 GP; or\
(B) 90 GP\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/bard_level_10_magical_secrets ` |
| ` 1 ` | ` dnd2024:class_features/bard_level_18_superior_inspiration ` |
| ` 2 ` | ` dnd2024:class_features/bard_level_19_epic_boon ` |
| ` 3 ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |
| ` 4 ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |
| ` 5 ` | ` dnd2024:class_features/bard_level_20_words_of_creation ` |
| ` 6 ` | ` dnd2024:class_features/bard_level_2_expertise ` |
| ` 7 ` | ` dnd2024:class_features/bard_level_2_jack_of_all_trades ` |
| ` 8 ` | ` dnd2024:class_features/bard_level_3_bard_subclass ` |
| ` 9 ` | ` dnd2024:class_features/bard_level_4_ability_score_improvement ` |
| ` 10 ` | ` dnd2024:class_features/bard_level_5_font_of_inspiration ` |
| ` 11 ` | ` dnd2024:class_features/bard_level_7_countercharm ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/bard ` |
| ` implementation ` | ` dnd2024:classes_bard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` primary_abilities ` | ` ["charisma"] ` |
| ` saving_throw_proficiencies ` | ` ["dexterity", "charisma"] ` |
| ` weapon_proficiencies ` | ` ["simple"] ` |
| ` armor_training ` | ` ["light"] ` |

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
| ` count ` | ` 3 ` |
| ` options ` | ` any_skill ` |

#### Tool proficiencies

<!-- splat:rows path=splat_rule/parameters/tool_proficiencies -->

| Row | category | count |
| --- | --- | --- |
| ` 0 ` | ` musical_instrument ` | ` 3 ` |

#### Starting equipment

<!-- splat:yaml path=splat_rule/parameters/starting_equipment -->

```yaml
count: 1
options:
- id: a
  grants:
  - item_type: leather_armor
    quantity: 1
  - item_type: dagger
    quantity: 2
  - item_type: musical_instrument_choice
    quantity: 1
  - item_type: entertainer_s_pack
    quantity: 1
  - item_type: gp
    quantity: 19
- id: b
  grants:
  - item_type: gp
    quantity: 90
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/bard_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 18 ` |
| ` 2 ` | ` 19 ` |
| ` 3 ` | ` 1 ` |
| ` 4 ` | ` 1 ` |
| ` 5 ` | ` 20 ` |
| ` 6 ` | ` 2 ` |
| ` 7 ` | ` 2 ` |
| ` 8 ` | ` 3 ` |
| ` 9 ` | ` 4 ` |
| ` 10 ` | ` 5 ` |
| ` 11 ` | ` 7 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_10_magical_secrets ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_18_superior_inspiration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_1_spellcasting ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_20_words_of_creation ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_2_expertise ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_2_jack_of_all_trades ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_3_bard_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_5_font_of_inspiration ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/11/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/bard_level_7_countercharm ` |
