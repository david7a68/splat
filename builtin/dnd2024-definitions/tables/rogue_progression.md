---
splat_schema: 1
splat_id: ace29b26-c1ca-5c14-b3f1-327e2642ecd2
splat_kind: rule
splat_key: dnd2024:tables/rogue_progression
splat_name: Rogue Progression
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
---

# Rogue Progression

## Source text

From SRD 5.2.1 page 62.

     Rogue Features
              Proficiency
      Level   Bonus         Class Features                                                        Sneak Attack
        1         +2        Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery                     1d6
        2         +2        Cunning Action                                                             1d6
        3         +2        Rogue Subclass, Steady Aim                                                 2d6
        4         +2        Ability Score Improvement                                                  2d6
        5         +3        Cunning Strike, Uncanny Dodge                                              3d6
        6         +3        Expertise                                                                  3d6
        7         +3        Evasion, Reliable Talent                                                   4d6
        8         +3        Ability Score Improvement                                                  4d6
        9         +4        Subclass feature                                                           5d6
       10         +4        Ability Score Improvement                                                  5d6
       11         +4        Improved Cunning Strike                                                    6d6
       12         +4        Ability Score Improvement                                                  6d6
       13         +5        Subclass feature                                                            7d6
       14         +5        Devious Strikes                                                             7d6
       15         +5        Slippery Mind                                                              8d6
       16         +5        Ability Score Improvement                                                  8d6
       17         +6        Subclass feature                                                           9d6
       18         +6        Elusive                                                                    9d6
       19         +6        Epic Boon                                                                 10d6
       20         +6        Stroke of Luck                                                            10d6


     roll if you have Advantage on the roll and the attack         Whenever you finish a Long Rest, you can change
     uses a Finesse or a Ranged weapon. The extra dam-           the kinds of weapons you chose. For example, you
     age’s type is the same as the weapon’s type.                could switch to using the mastery properties of
       You don’t need Advantage on the attack roll if at         Scimitars and Shortswords.
     least one of your allies is within 5 feet of the target,
     the ally doesn’t have the Incapacitated condition,          Level 2: Cunning Action
     and you don’t have Disadvantage on the attack roll.         Your quick thinking and agility allow you to move
       The extra damage increases as you gain Rogue              and act quickly. On your turn, you can take one of
     levels, as shown in the Sneak Attack column of the          the following actions as a Bonus Action: Dash, Dis-
     Rogue Features table.                                       engage, or Hide.

     Level 1: Thieves’ Cant                                      Level 3: Rogue Subclass
     You picked up various languages in the commu-               You gain a Rogue subclass of your choice. The Thief
     nities where you plied your roguish talents. You            subclass is detailed after this class’s description. A
     know Thieves’ Cant and one other language of your           subclass is a specialization that grants you features
     choice, which you choose from the language tables           at certain Rogue levels. For the rest of your career,
     in “Character Creation.”                                    you gain each of your subclass’s features that are of
                                                                 your Rogue level or lower.
     Level 1: Weapon Mastery
     Your training with weapons allows you to use the            Level 3: Steady Aim
     mastery properties of two kinds of weapons of your          As a Bonus Action, you give yourself Advantage on
     choice with which you have proficiency, such as             your next attack roll on the current turn. You can
     Daggers and Shortbows.                                      use this feature only if you haven’t moved during
                                                                 this turn, and after you use it, your Speed is 0 until
                                                                 the end of the current turn.




62   System Reference Document 5.2.1

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/rogue_progression ` |
| ` implementation ` | ` dnd2024:class_progression_rogue_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |

#### Rows

<!-- splat:table
path: splat_rule/parameters/rows
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
- row_9
- row_10
- row_11
- row_12
- row_13
- row_14
- row_15
- row_16
- row_17
- row_18
- row_19
- row_20
columns:
  Key:
    type: key
  Level:
    path: level
    type: integer
  Proficiency bonus:
    path: proficiency_bonus
    type: integer
  Features text:
    path: features_text
    type: string
  Sneak attack:
    path: sneak_attack
    type: string
  Sneak attack dice dice count:
    path: sneak_attack_dice/dice/count
    type: integer
  Sneak attack dice dice sides:
    path: sneak_attack_dice/dice/sides
    type: integer
  Sneak attack dice bonus:
    path: sneak_attack_dice/bonus
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Proficiency bonus | Features text | Sneak attack | Sneak attack dice dice count | Sneak attack dice dice sides | Sneak attack dice bonus | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 2 ` | ` Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery ` | ` 1d6 ` | ` 1 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_1 "section:row_1") |
| ` row_2 ` | ` 2 ` | ` 2 ` | ` Cunning Action ` | ` 1d6 ` | ` 1 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_2 "section:row_2") |
| ` row_3 ` | ` 3 ` | ` 2 ` | ` Rogue Subclass, Steady Aim ` | ` 2d6 ` | ` 2 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_3 "section:row_3") |
| ` row_4 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement ` | ` 2d6 ` | ` 2 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_4 "section:row_4") |
| ` row_5 ` | ` 5 ` | ` 3 ` | ` Cunning Strike, Uncanny Dodge ` | ` 3d6 ` | ` 3 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_5 "section:row_5") |
| ` row_6 ` | ` 6 ` | ` 3 ` | ` Expertise ` | ` 3d6 ` | ` 3 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_6 "section:row_6") |
| ` row_7 ` | ` 7 ` | ` 3 ` | ` Evasion, Reliable Talent ` | ` 4d6 ` | ` 4 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_7 "section:row_7") |
| ` row_8 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 4d6 ` | ` 4 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_8 "section:row_8") |
| ` row_9 ` | ` 9 ` | ` 4 ` | ` Subclass feature ` | ` 5d6 ` | ` 5 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_9 "section:row_9") |
| ` row_10 ` | ` 10 ` | ` 4 ` | ` Ability Score Improvement ` | ` 5d6 ` | ` 5 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_10 "section:row_10") |
| ` row_11 ` | ` 11 ` | ` 4 ` | ` Improved Cunning Strike ` | ` 6d6 ` | ` 6 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_11 "section:row_11") |
| ` row_12 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 6d6 ` | ` 6 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_12 "section:row_12") |
| ` row_13 ` | ` 13 ` | ` 5 ` | ` Subclass feature ` | ` 7d6 ` | ` 7 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_13 "section:row_13") |
| ` row_14 ` | ` 14 ` | ` 5 ` | ` Devious Strikes ` | ` 7d6 ` | ` 7 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_14 "section:row_14") |
| ` row_15 ` | ` 15 ` | ` 5 ` | ` Slippery Mind ` | ` 8d6 ` | ` 8 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_15 "section:row_15") |
| ` row_16 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 8d6 ` | ` 8 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_16 "section:row_16") |
| ` row_17 ` | ` 17 ` | ` 6 ` | ` Subclass feature ` | ` 9d6 ` | ` 9 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_17 "section:row_17") |
| ` row_18 ` | ` 18 ` | ` 6 ` | ` Elusive ` | ` 9d6 ` | ` 9 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_18 "section:row_18") |
| ` row_19 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 10d6 ` | ` 10 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_19 "section:row_19") |
| ` row_20 ` | ` 20 ` | ` 6 ` | ` Stroke of Luck ` | ` 10d6 ` | ` 10 ` | ` 6 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_20 "section:row_20") |

##### Row 1

^splat-0f5cfe33-row_1

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |
| ` 2 ` | ` grant_feature ` | ` 1 ` |
| ` 3 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_expertise ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_sneak_attack ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_thieves_cant ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/3/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_weapon_mastery ` |

##### Row 2

^splat-0f5cfe33-row_2

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_2/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_2_cunning_action ` |

##### Row 3

^splat-0f5cfe33-row_3

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` choose_subclass ` | ` 3 ` |
| ` 1 ` | ` grant_feature ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_3_rogue_subclass ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_3_steady_aim ` |

##### Row 4

^splat-0f5cfe33-row_4

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_4/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 4 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_4/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_4_ability_score_improvement ` |

##### Row 5

^splat-0f5cfe33-row_5

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_5/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 5 ` |
| ` 1 ` | ` grant_feature ` | ` 5 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_5/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_5_cunning_strike ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_5/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_5_uncanny_dodge ` |

##### Row 6

^splat-0f5cfe33-row_6

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_6/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_6/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_1_expertise ` |

##### Row 7

^splat-0f5cfe33-row_7

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_7/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 7 ` |
| ` 1 ` | ` grant_feature ` | ` 7 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_7/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_7_evasion ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_7/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_7_reliable_talent ` |

##### Row 8

^splat-0f5cfe33-row_8

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_8/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_8/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_4_ability_score_improvement ` |

##### Row 9

^splat-0f5cfe33-row_9

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_9/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 9 ` |

##### Row 10

^splat-0f5cfe33-row_10

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_10/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 10 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_10/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_4_ability_score_improvement ` |

##### Row 11

^splat-0f5cfe33-row_11

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_11/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 11 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_11/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_11_improved_cunning_strike ` |

##### Row 12

^splat-0f5cfe33-row_12

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_12/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 12 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_12/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_4_ability_score_improvement ` |

##### Row 13

^splat-0f5cfe33-row_13

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_13/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 13 ` |

##### Row 14

^splat-0f5cfe33-row_14

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_14/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 14 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_14/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_14_devious_strikes ` |

##### Row 15

^splat-0f5cfe33-row_15

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_15/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 15 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_15/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_15_slippery_mind ` |

##### Row 16

^splat-0f5cfe33-row_16

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_16/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 16 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_16/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_4_ability_score_improvement ` |

##### Row 17

^splat-0f5cfe33-row_17

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_17/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 17 ` |

##### Row 18

^splat-0f5cfe33-row_18

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_18/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 18 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_18/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_18_elusive ` |

##### Row 19

^splat-0f5cfe33-row_19

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_19/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 19 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_19/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_19_epic_boon ` |

##### Row 20

^splat-0f5cfe33-row_20

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_20/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 20 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_20/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/rogue_level_20_stroke_of_luck ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/rogue ` |

#### Lookup

<!-- splat:values path=splat_rule/parameters/lookup -->

| Field | Value |
| --- | --- |
| ` input ` | ` class_level ` |
| ` minimum ` | ` 1 ` |
| ` maximum ` | ` 20 ` |
| ` match ` | ` exact ` |
| ` numeric_columns ` | ` current_capacity_or_value ` |
| ` feature_grants ` | ` all_crossed_levels ` |
| ` repeat_application ` | ` idempotent_per_class_level_and_feature ` |

#### Multiclass

<!-- splat:yaml path=splat_rule/parameters/multiclass -->

```yaml
proficiency_bonus_source: total_character_level
spell_slot_override_rule:
  key: dnd2024:rules/multiclassing_spellcasting
pact_magic_pool_separate: true
```

#### Column semantics

<!-- splat:yaml path=splat_rule/parameters/column_semantics -->

```yaml
proficiency_bonus:
  kind: bonus
  applies_to: proficiency
  use_class_row_only_if_single_class: true
sneak_attack:
  kind: dice_expression
  applies_to: sneak_attack
```
