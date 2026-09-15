---
splat_schema: 1
splat_id: 6d3c3b76-326b-5f07-8bd3-3802868b70ca
splat_kind: rule_group
splat_key: dnd2024:classes/fighter
splat_name: Fighter
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
---

# Fighter

## Source text

<!-- SRD page 47 -->

Core Fighter Traits\
Primary Ability Strength or Dexterity\
Hit Point Die D10 per Fighter level\
Saving Throw Strength and Constitution\
Proficiencies\
Skill Proficiencies Choose 2: Acrobatics, Animal\
Handling, Athletics, History,\
Insight, Intimidation, Persua-\
sion, Perception, or Survival\
Weapon Proficiencies Simple and Martial weapons\
Armor Training Light, Medium, and Heavy ar-\
mor and Shields\
Starting Equipment Choose A, B, or C: (A) Chain\
Mail, Greatsword, Flail, 8 Jav-\
elins, Dungeoneer’s Pack, and\
4 GP; (B) Studded Leather\
Armor, Scimitar, Short sword,\
Longbow, 20 Arrows, Quiver,\
Dungeoneer’s Pack, and 11\
GP; or (C) 155 GP\
Fighter Features\
Proficiency\
Level Bonus Class Features\
1 +2 Fighting Style, Second Wind, Weapon Mastery\
2 +2 Action Surge (one use), Tactical Mind\
3 +2 Fighter Subclass\
4 +2 Ability Score Improvement\
5 +3 Extra Attack, Tactical Shift\
6 +3 Ability Score Improvement\
7 +3 Subclass feature\
8 +3 Ability Score Improvement\
9 +4 Indomitable (one use), Tactical Master\
10 +4 Subclass feature\
11 +4 Two Extra Attacks\
12 +4 Ability Score Improvement\
13 +5 Indomitable (two uses), Studied Attacks\
14 +5 Ability Score Improvement\
15 +5 Subclass feature\
16 +5 Ability Score Improvement\
17 +6 Action Surge (two uses), Indomitable (three uses)\
18 +6 Subclass feature\
19 +6 Epic Boon\
20 +6 Three Extra Attacks\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` class ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/fighter_level_11_two_extra_attacks ` |
| ` 1 ` | ` dnd2024:class_features/fighter_level_13_studied_attacks ` |
| ` 2 ` | ` dnd2024:class_features/fighter_level_19_epic_boon ` |
| ` 3 ` | ` dnd2024:class_features/fighter_level_1_fighting_style ` |
| ` 4 ` | ` dnd2024:class_features/fighter_level_1_second_wind ` |
| ` 5 ` | ` dnd2024:class_features/fighter_level_1_weapon_mastery ` |
| ` 6 ` | ` dnd2024:class_features/fighter_level_20_three_extra_attacks ` |
| ` 7 ` | ` dnd2024:class_features/fighter_level_2_action_surge ` |
| ` 8 ` | ` dnd2024:class_features/fighter_level_2_tactical_mind ` |
| ` 9 ` | ` dnd2024:class_features/fighter_level_3_fighter_subclass ` |
| ` 10 ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |
| ` 11 ` | ` dnd2024:class_features/fighter_level_5_extra_attack ` |
| ` 12 ` | ` dnd2024:class_features/fighter_level_5_tactical_shift ` |
| ` 13 ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |
| ` 14 ` | ` dnd2024:class_features/fighter_level_9_tactical_master ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:classes/fighter ` |
| ` implementation ` | ` dnd2024:classes_fighter ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` primary_abilities ` | ` ["strength", "dexterity"] ` |
| ` saving_throw_proficiencies ` | ` ["strength", "constitution"] ` |
| ` weapon_proficiencies ` | ` ["simple", "martial"] ` |
| ` armor_training ` | ` ["light", "medium", "heavy", "shield"] ` |

#### Hit point die

<!-- splat:values path=splat_rule/parameters/hit_point_die -->

| Field | Value |
| --- | --- |
| ` sides ` | ` 10 ` |
| ` per_class_level ` | ` 1 ` |

#### Skill proficiencies

<!-- splat:values path=splat_rule/parameters/skill_proficiencies -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` options ` | ` ["acrobatics", "animal_handling", "athletics", "history", "insight", "intimidation", "persuasion", "perception", "survival"] ` |

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
  - item_type: chain_mail
    quantity: 1
  - item_type: greatsword
    quantity: 1
  - item_type: flail
    quantity: 1
  - item_type: javelin
    quantity: 8
  - item_type: dungeoneer_s_pack
    quantity: 1
  - item_type: gp
    quantity: 4
- id: b
  grants:
  - item_type: studded_leather_armor
    quantity: 1
  - item_type: scimitar
    quantity: 1
  - item_type: shortsword
    quantity: 1
  - item_type: longbow
    quantity: 1
  - item_type: arrow
    quantity: 20
  - item_type: quiver
    quantity: 1
  - item_type: dungeoneer_s_pack
    quantity: 1
  - item_type: gp
    quantity: 11
- id: c
  grants:
  - item_type: gp
    quantity: 155
```

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/fighter_progression ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 11 ` |
| ` 1 ` | ` 13 ` |
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
| ` 12 ` | ` 5 ` |
| ` 13 ` | ` 9 ` |
| ` 14 ` | ` 9 ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_11_two_extra_attacks ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_13_studied_attacks ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_19_epic_boon ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_fighting_style ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/4/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_second_wind ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/5/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_1_weapon_mastery ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/6/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_20_three_extra_attacks ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/7/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_action_surge ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/8/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_2_tactical_mind ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/9/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_3_fighter_subclass ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/10/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_4_ability_score_improvement ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/11/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_5_extra_attack ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/12/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_5_tactical_shift ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/13/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_indomitable ` |

##### Feature

<!-- splat:values path=splat_rule/parameters/feature_grants/14/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/fighter_level_9_tactical_master ` |
