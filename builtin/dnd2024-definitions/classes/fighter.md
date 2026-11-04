---
splat_schema: 2
splat_id: 6d3c3b76-326b-5f07-8bd3-3802868b70ca
splat_kind: rule_group
splat_key: dnd2024:classes/fighter
splat_name: Fighter
splat_origin: dnd2024:reference/pages/047
splat_sources:
- SRD 5.2.1, pp. 47, https://www.dndbeyond.com/srd
---

# Fighter

## Source text

<!-- SRD page 47 -->

**Core Fighter Traits**

|  |  |
| --- | --- |
| Primary Ability | Strength or Dexterity |
| Hit Point Die | D10 per Fighter level |
| Saving Throw | Strength and Constitution |
| Proficiencies |  |
| Skill Proficiencies | Choose 2: Acrobatics, Animal Handling, Athletics, History, Insight, Intimidation, Persuasion, Perception, or Survival |
| Weapon Proficiencies | Simple and Martial weapons |
| Armor Training | Light, Medium, and Heavy armor and Shields |
| Starting Equipment | Choose A, B, or C: (A) Chain Mail, Greatsword, Flail, 8 Javelins, Dungeoneer’s Pack, and 4 GP; (B) Studded Leather Armor, Scimitar, Shortsword, Longbow, 20 Arrows, Quiver, Dungeoneer’s Pack, and 11 GP; or (C) 155 GP |

**Fighter Features**

| Level | Proficiency Bonus | Class Features | Second Wind | Weapon Mastery |
| --- | --- | --- | --- | --- |
| 1 | +2 | Fighting Style, Second Wind, Weapon Mastery | 2 | 3 |
| 2 | +2 | Action Surge (one use), Tactical Mind | 2 | 3 |
| 3 | +2 | Fighter Subclass | 2 | 3 |
| 4 | +2 | Ability Score Improvement | 3 | 4 |
| 5 | +3 | Extra Attack, Tactical Shift | 3 | 4 |
| 6 | +3 | Ability Score Improvement | 3 | 4 |
| 7 | +3 | Subclass feature | 3 | 4 |
| 8 | +3 | Ability Score Improvement | 3 | 4 |
| 9 | +4 | Indomitable (one use), Tactical Master | 3 | 4 |
| 10 | +4 | Subclass feature | 4 | 5 |
| 11 | +4 | Two Extra Attacks | 4 | 5 |
| 12 | +4 | Ability Score Improvement | 4 | 5 |
| 13 | +5 | Indomitable (two uses), Studied Attacks | 4 | 5 |
| 14 | +5 | Ability Score Improvement | 4 | 5 |
| 15 | +5 | Subclass feature | 4 | 5 |
| 16 | +5 | Ability Score Improvement | 4 | 6 |
| 17 | +6 | Action Surge (two uses), Indomitable (three uses) | 4 | 6 |
| 18 | +6 | Subclass feature | 4 | 6 |
| 19 | +6 | Epic Boon | 4 | 6 |
| 20 | +6 | Three Extra Attacks | 4 | 6 |

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
- c
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
| ` C ` | [Rules](#^splat-93071aae-c "section:c") |

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
- row_5
- row_6
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
| ` row_1 ` | ` Chain mail ` | ` 1 ` |
| ` row_2 ` | ` Greatsword ` | ` 1 ` |
| ` row_3 ` | ` Flail ` | ` 1 ` |
| ` row_4 ` | ` Javelin ` | ` 8 ` |
| ` row_5 ` | ` Dungeoneer s pack ` | ` 1 ` |
| ` row_6 ` | ` Gp ` | ` 4 ` |

###### B

^splat-93071aae-b

###### Grants

<!-- splat:table
path: splat_rule/parameters/starting_equipment/options/@b/grants
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
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
| ` row_1 ` | ` Studded leather armor ` | ` 1 ` |
| ` row_2 ` | ` Scimitar ` | ` 1 ` |
| ` row_3 ` | ` Shortsword ` | ` 1 ` |
| ` row_4 ` | ` Longbow ` | ` 1 ` |
| ` row_5 ` | ` Arrow ` | ` 20 ` |
| ` row_6 ` | ` Quiver ` | ` 1 ` |
| ` row_7 ` | ` Dungeoneer s pack ` | ` 1 ` |
| ` row_8 ` | ` Gp ` | ` 11 ` |

###### C

^splat-93071aae-c

###### Grants

<!-- splat:rows path=splat_rule/parameters/starting_equipment/options/@c/grants -->

| Row | item_type | quantity |
| --- | --- | --- |
| ` 0 ` | ` gp ` | ` 155 ` |

#### Progression

<!-- splat:values path=splat_rule/parameters/progression -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:tables/fighter_progression ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- fighter_level_11_two_extra_attacks
- fighter_level_13_studied_attacks
- fighter_level_19_epic_boon
- fighter_level_1_fighting_style
- fighter_level_1_second_wind
- fighter_level_1_weapon_mastery
- fighter_level_20_three_extra_attacks
- fighter_level_2_action_surge
- fighter_level_2_tactical_mind
- fighter_level_3_fighter_subclass
- fighter_level_4_ability_score_improvement
- fighter_level_5_extra_attack
- fighter_level_5_tactical_shift
- fighter_level_9_indomitable
- fighter_level_9_tactical_master
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
| [Level 11: Two Extra Attacks](<../class_features/fighter_level_11_two_extra_attacks.md> "dnd2024:class_features/fighter_level_11_two_extra_attacks") | ` 11 ` |
| [Level 13: Studied Attacks](<../class_features/fighter_level_13_studied_attacks.md> "dnd2024:class_features/fighter_level_13_studied_attacks") | ` 13 ` |
| [Level 19: Epic Boon](<../class_features/fighter_level_19_epic_boon.md> "dnd2024:class_features/fighter_level_19_epic_boon") | ` 19 ` |
| [Level 1: Fighting Style](<../class_features/fighter_level_1_fighting_style.md> "dnd2024:class_features/fighter_level_1_fighting_style") | ` 1 ` |
| [Level 1: Second Wind](<../class_features/fighter_level_1_second_wind.md> "dnd2024:class_features/fighter_level_1_second_wind") | ` 1 ` |
| [Level 1: Weapon Mastery](<../class_features/fighter_level_1_weapon_mastery.md> "dnd2024:class_features/fighter_level_1_weapon_mastery") | ` 1 ` |
| [Level 20: Three Extra Attacks](<../class_features/fighter_level_20_three_extra_attacks.md> "dnd2024:class_features/fighter_level_20_three_extra_attacks") | ` 20 ` |
| [Level 2: Action Surge](<../class_features/fighter_level_2_action_surge.md> "dnd2024:class_features/fighter_level_2_action_surge") | ` 2 ` |
| [Level 2: Tactical Mind](<../class_features/fighter_level_2_tactical_mind.md> "dnd2024:class_features/fighter_level_2_tactical_mind") | ` 2 ` |
| [Level 3: Fighter Subclass](<../class_features/fighter_level_3_fighter_subclass.md> "dnd2024:class_features/fighter_level_3_fighter_subclass") | ` 3 ` |
| [Level 4: Ability Score Improvement](<../class_features/fighter_level_4_ability_score_improvement.md> "dnd2024:class_features/fighter_level_4_ability_score_improvement") | ` 4 ` |
| [Level 5: Extra Attack](<../class_features/fighter_level_5_extra_attack.md> "dnd2024:class_features/fighter_level_5_extra_attack") | ` 5 ` |
| [Level 5: Tactical Shift](<../class_features/fighter_level_5_tactical_shift.md> "dnd2024:class_features/fighter_level_5_tactical_shift") | ` 5 ` |
| [Level 9: Indomitable](<../class_features/fighter_level_9_indomitable.md> "dnd2024:class_features/fighter_level_9_indomitable") | ` 9 ` |
| [Level 9: Tactical Master](<../class_features/fighter_level_9_tactical_master.md> "dnd2024:class_features/fighter_level_9_tactical_master") | ` 9 ` |
