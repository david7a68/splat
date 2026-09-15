---
splat_schema: 1
splat_id: 235f10c4-25ec-5b36-8da9-344befa0f4c4
splat_kind: rule
splat_key: dnd2024:tables/monk_progression
splat_name: Monk Progression
splat_origin:
  key: dnd2024:reference/pages/050
splat_sources:
- title: SRD 5.2.1
  locator: pp. 50
  url: https://www.dndbeyond.com/srd
---

# Monk Progression

## Source text

From SRD 5.2.1 page 50.

     Monk Features
              Proficiency                                                       Martial       Focus     Unarmored
      Level     Bonus     Class Features                                         Arts         Points    Movement
        1         +2      Martial Arts, Unarmored Defense                         1d6          —             —
        2         +2      Monk’s Focus, Unarmored Movement,                       1d6           2          +10 ft.
                          Uncanny Metabolism
        3         +2      Deflect Attacks, Monk Subclass                          1d6           3          +10 ft.
        4         +2      Ability Score Improvement, Slow Fall                    1d6           4          +10 ft.
        5         +3      Extra Attack, Stunning Strike                           1d8           5          +10 ft.
        6         +3      Empowered Strikes, Subclass feature                     1d8           6          +15 ft.
        7         +3      Evasion                                                 1d8           7          +15 ft.
        8         +3      Ability Score Improvement                               1d8           8          +15 ft.
        9         +4      Acrobatic Movement                                      1d8           9          +15 ft.
       10         +4      Heightened Focus, Self-Restoration                      1d8          10          +20 ft.
       11         +4      Subclass feature                                       1d10          11          +20 ft.
       12         +4      Ability Score Improvement                              1d10          12          +20 ft.
       13         +5      Deflect Energy                                         1d10          13          +20 ft.
       14         +5      Disciplined Survivor                                   1d10          14          +25 ft.
       15         +5      Perfect Focus                                          1d10          15          +25 ft.
       16         +5      Ability Score Improvement                              1d10          16          +25 ft.
       17         +6      Subclass feature                                       1d12          17          +25 ft.
       18         +6      Superior Defense                                       1d12          18          +30 ft.
       19         +6      Epic Boon                                              1d12          19          +30 ft.
       20         +6      Body and Mind                                          1d12          20          +30 ft.


     Level 1: Martial Arts                                       Level 1: Unarmored Defense
     Your practice of martial arts gives you mastery of          While you aren’t wearing armor or wielding a
     combat styles that use your Unarmed Strike and              Shield, your base Armor Class equals 10 plus your
     Monk weapons, which are the following:                      Dexterity and Wisdom modifiers.
     • Simple Melee weapons                                      Level 2: Monk’s Focus
     • Martial Melee weapons that have the Light                 Your focus and martial training allow you to har-
       property                                                  ness a well of extraordinary energy within yourself.
     You gain the following benefits while you are un-           This energy is represented by Focus Points. Your
     armed or wielding only Monk weapons and you ar-             Monk level determines the number of points you
     en’t wearing armor or wielding a Shield.                    have, as shown in the Focus Points column of the
       Bonus Unarmed Strike. You can make an Un-                 Monk Features table.
     armed Strike as a Bonus Action.                               You can expend these points to enhance or fuel
       Martial Arts Die. You can roll 1d6 in place of the        certain Monk features. You start knowing three
     normal damage of your Unarmed Strike or Monk                such features: Flurry of Blows, Patient Defense, and
     weapons. This die changes as you gain Monk levels,          Step of the Wind, each of which is detailed below.
     as shown in the Martial Arts column of the Monk               When you expend a Focus Point, it is unavailable
     Features table.                                             until you finish a Short or Long Rest, at the end of
       Dexterous Attacks. You can use your Dexterity             which you regain all your expended points.
     modifier instead of your Strength modifier for the            Some features that use Focus Points require your
     attack and damage rolls of your Unarmed Strikes             target to make a saving throw. The save DC equals 8
     and Monk weapons. In addition, when you use the             plus your Wisdom modifier and Proficiency Bonus.
     Grapple or Shove option of your Unarmed Strike,               Flurry of Blows. You can expend 1 Focus Point to
     you can use your Dexterity modifier instead of your         make two Unarmed Strikes as a Bonus Action.
     Strength modifier to determine the save DC.                   Patient Defense. You can take the Disengage
                                                                 action as a Bonus Action. Alternatively, you can


50   System Reference Document 5.2.1

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/monk_progression ` |
| ` implementation ` | ` dnd2024:class_progression_monk_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |

#### Rows

<!-- splat:rows path=splat_rule/parameters/rows -->

| Row | level | proficiency_bonus | features_text | martial_arts | focus_points | unarmored_movement |
| --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 2 ` | ` Martial Arts, Unarmored Defense ` | ` 1d6 ` | ` 0 ` | ` 0 ` |
| ` 1 ` | ` 2 ` | ` 2 ` | ` Monk’s Focus, Unarmored Movement, Uncanny Metabolism ` | ` 1d6 ` | ` 2 ` | ` 10 ` |
| ` 2 ` | ` 3 ` | ` 2 ` | ` Deflect Attacks, Monk Subclass ` | ` 1d6 ` | ` 3 ` | ` 10 ` |
| ` 3 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement, Slow Fall ` | ` 1d6 ` | ` 4 ` | ` 10 ` |
| ` 4 ` | ` 5 ` | ` 3 ` | ` Extra Attack, Stunning Strike ` | ` 1d8 ` | ` 5 ` | ` 10 ` |
| ` 5 ` | ` 6 ` | ` 3 ` | ` Empowered Strikes, Subclass feature ` | ` 1d8 ` | ` 6 ` | ` 15 ` |
| ` 6 ` | ` 7 ` | ` 3 ` | ` Evasion ` | ` 1d8 ` | ` 7 ` | ` 15 ` |
| ` 7 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 1d8 ` | ` 8 ` | ` 15 ` |
| ` 8 ` | ` 9 ` | ` 4 ` | ` Acrobatic Movement ` | ` 1d8 ` | ` 9 ` | ` 15 ` |
| ` 9 ` | ` 10 ` | ` 4 ` | ` Heightened Focus, Self-Restoration ` | ` 1d8 ` | ` 10 ` | ` 20 ` |
| ` 10 ` | ` 11 ` | ` 4 ` | ` Subclass feature ` | ` 1d10 ` | ` 11 ` | ` 20 ` |
| ` 11 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 1d10 ` | ` 12 ` | ` 20 ` |
| ` 12 ` | ` 13 ` | ` 5 ` | ` Deflect Energy ` | ` 1d10 ` | ` 13 ` | ` 20 ` |
| ` 13 ` | ` 14 ` | ` 5 ` | ` Disciplined Survivor ` | ` 1d10 ` | ` 14 ` | ` 25 ` |
| ` 14 ` | ` 15 ` | ` 5 ` | ` Perfect Focus ` | ` 1d10 ` | ` 15 ` | ` 25 ` |
| ` 15 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 1d10 ` | ` 16 ` | ` 25 ` |
| ` 16 ` | ` 17 ` | ` 6 ` | ` Subclass feature ` | ` 1d12 ` | ` 17 ` | ` 25 ` |
| ` 17 ` | ` 18 ` | ` 6 ` | ` Superior Defense ` | ` 1d12 ` | ` 18 ` | ` 30 ` |
| ` 18 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 1d12 ` | ` 19 ` | ` 30 ` |
| ` 19 ` | ` 20 ` | ` 6 ` | ` Body and Mind ` | ` 1d12 ` | ` 20 ` | ` 30 ` |

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/0/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_martial_arts ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/0/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_1_unarmored_defense ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/0/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 6
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 2 ` |
| ` 2 ` | ` grant_feature ` | ` 2 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_monk_s_focus ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_unarmored_movement ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/1/grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_2_uncanny_metabolism ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/1/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 6
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/2/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 3 ` |
| ` 1 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_deflect_attacks ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/2/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_3_monk_subclass ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/2/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 6
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 4 ` |
| ` 1 ` | ` grant_feature ` | ` 4 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/3/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_slow_fall ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/3/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 6
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/4/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 5 ` |
| ` 1 ` | ` grant_feature ` | ` 5 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/4/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_5_extra_attack ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/4/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_5_stunning_strike ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/4/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 8
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/5/grants -->

| Row | operation | at_class_level | class_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 6 ` |  |
| ` 1 ` | ` grant_selected_subclass_features ` |  | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/5/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_6_empowered_strikes ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/5/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 8
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/6/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 7 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/6/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_7_evasion ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/6/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 8
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/7/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/7/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/7/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 8
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/8/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 9 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/8/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_9_acrobatic_movement ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/8/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 8
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/9/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 10 ` |
| ` 1 ` | ` grant_feature ` | ` 10 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/9/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_heightened_focus ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/9/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_10_self_restoration ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/9/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 8
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/10/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 11 ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/10/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 10
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/11/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 12 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/11/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/11/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 10
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/12/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 13 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/12/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_13_deflect_energy ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/12/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 10
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/13/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 14 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/13/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_14_disciplined_survivor ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/13/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 10
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/14/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 15 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/14/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_15_perfect_focus ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/14/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 10
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/15/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 16 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/15/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_4_ability_score_improvement ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/15/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 10
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/16/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 17 ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/16/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 12
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/17/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 18 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/17/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_18_superior_defense ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/17/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 12
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/18/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 19 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/18/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_19_epic_boon ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/18/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 12
bonus: 0
```

##### Grants

<!-- splat:rows path=splat_rule/parameters/rows/19/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 20 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/19/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/monk_level_20_body_and_mind ` |

##### Martial arts dice

<!-- splat:yaml path=splat_rule/parameters/rows/19/martial_arts_dice -->

```yaml
dice:
  count: 1
  sides: 12
bonus: 0
```

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/monk ` |

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
focus_points:
  kind: resource_capacity
  resource: focus_points
  recovery: owning_feature_contract
  level_change_refills: false
martial_arts:
  kind: dice_expression
  applies_to: martial_arts
proficiency_bonus:
  kind: bonus
  applies_to: proficiency
  use_class_row_only_if_single_class: true
unarmored_movement:
  kind: speed_bonus
  unit: ft
  requires: unarmored_movement_feature_eligibility
```
