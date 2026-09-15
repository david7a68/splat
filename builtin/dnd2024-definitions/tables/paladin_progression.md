---
splat_schema: 1
splat_id: 865c298e-3797-559d-8907-54fc43e851a1
splat_kind: rule
splat_key: dnd2024:tables/paladin_progression
splat_name: Paladin Progression
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
---

# Paladin Progression

## Source text

From SRD 5.2.1 page 53.

     Paladin Features
              Proficiency                                        Channel    Prepared    —Spell Slots per Spell Level—
      Level     Bonus       Class Features                       Divinity    Spells     1      2      3      4       5
        1         +2        Lay On Hands, Spellcasting,            —           2        2     —      —      —        —
                            Weapon Mastery
        2         +2        Fighting Style, Paladin’s Smite        —           3        2     —      —      —        —
        3         +2        Channel Divinity, Paladin Subclass      2          4        3     —      —      —        —
        4         +2        Ability Score Improvement               2          5        3     —      —      —        —
        5         +3        Extra Attack, Faithful Steed            2          6        4      2     —      —        —
        6         +3        Aura of Protection                      2          6        4      2     —      —        —
        7         +3        Subclass feature                        2          7        4      3     —      —        —
        8         +3        Ability Score Improvement               2          7        4      3     —      —        —
        9         +4        Abjure Foes                             2          9        4      3      2     —        —
       10         +4        Aura of Courage                         2          9        4      3      2     —        —
       11         +4        Radiant Strikes                         3          10       4      3      3     —        —
       12         +4        Ability Score Improvement               3          10       4      3      3     —        —
       13         +5        —                                       3          11       4      3      3      1       —
       14         +5        Restoring Touch                         3          11       4      3      3      1       —
       15         +5        Subclass feature                        3          12       4      3      3      2       —
       16         +5        Ability Score Improvement               3          12       4      3      3      2       —
       17         +6        —                                       3          14       4      3      3      3       1
       18         +6        Aura Expansion                          3          14       4      3      3      3       1
       19         +6        Epic Boon                               3          15       4      3      3      3       2
       20         +6        Subclass feature                        3          15       4      3      3      3       2



53   System Reference Document 5.2.1

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/paladin_progression ` |
| ` implementation ` | ` dnd2024:class_progression_paladin_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |

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
  Channel divinity:
    path: channel_divinity
    type: integer
  Prepared spells:
    path: prepared_spells
    type: integer
  Spell slots 1:
    path: spell_slots_1
    type: integer
  Spell slots 2:
    path: spell_slots_2
    type: integer
  Spell slots 3:
    path: spell_slots_3
    type: integer
  Spell slots 4:
    path: spell_slots_4
    type: integer
  Spell slots 5:
    path: spell_slots_5
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Proficiency bonus | Features text | Channel divinity | Prepared spells | Spell slots 1 | Spell slots 2 | Spell slots 3 | Spell slots 4 | Spell slots 5 | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 2 ` | ` Lay On Hands, Spellcasting, Weapon Mastery ` | ` 0 ` | ` 2 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_1 "section:row_1") |
| ` row_2 ` | ` 2 ` | ` 2 ` | ` Fighting Style, Paladin’s Smite ` | ` 0 ` | ` 3 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_2 "section:row_2") |
| ` row_3 ` | ` 3 ` | ` 2 ` | ` Channel Divinity, Paladin Subclass ` | ` 2 ` | ` 4 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_3 "section:row_3") |
| ` row_4 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement ` | ` 2 ` | ` 5 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_4 "section:row_4") |
| ` row_5 ` | ` 5 ` | ` 3 ` | ` Extra Attack, Faithful Steed ` | ` 2 ` | ` 6 ` | ` 4 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_5 "section:row_5") |
| ` row_6 ` | ` 6 ` | ` 3 ` | ` Aura of Protection ` | ` 2 ` | ` 6 ` | ` 4 ` | ` 2 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_6 "section:row_6") |
| ` row_7 ` | ` 7 ` | ` 3 ` | ` Subclass feature ` | ` 2 ` | ` 7 ` | ` 4 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_7 "section:row_7") |
| ` row_8 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 2 ` | ` 7 ` | ` 4 ` | ` 3 ` | ` 0 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_8 "section:row_8") |
| ` row_9 ` | ` 9 ` | ` 4 ` | ` Abjure Foes ` | ` 2 ` | ` 9 ` | ` 4 ` | ` 3 ` | ` 2 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_9 "section:row_9") |
| ` row_10 ` | ` 10 ` | ` 4 ` | ` Aura of Courage ` | ` 2 ` | ` 9 ` | ` 4 ` | ` 3 ` | ` 2 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_10 "section:row_10") |
| ` row_11 ` | ` 11 ` | ` 4 ` | ` Radiant Strikes ` | ` 3 ` | ` 10 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_11 "section:row_11") |
| ` row_12 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 3 ` | ` 10 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 0 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_12 "section:row_12") |
| ` row_13 ` | ` 13 ` | ` 5 ` | ` — ` | ` 3 ` | ` 11 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 1 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_13 "section:row_13") |
| ` row_14 ` | ` 14 ` | ` 5 ` | ` Restoring Touch ` | ` 3 ` | ` 11 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 1 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_14 "section:row_14") |
| ` row_15 ` | ` 15 ` | ` 5 ` | ` Subclass feature ` | ` 3 ` | ` 12 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 2 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_15 "section:row_15") |
| ` row_16 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 3 ` | ` 12 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 2 ` | ` 0 ` | [Rules](#^splat-0f5cfe33-row_16 "section:row_16") |
| ` row_17 ` | ` 17 ` | ` 6 ` | ` — ` | ` 3 ` | ` 14 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 1 ` | [Rules](#^splat-0f5cfe33-row_17 "section:row_17") |
| ` row_18 ` | ` 18 ` | ` 6 ` | ` Aura Expansion ` | ` 3 ` | ` 14 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 1 ` | [Rules](#^splat-0f5cfe33-row_18 "section:row_18") |
| ` row_19 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 3 ` | ` 15 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 2 ` | [Rules](#^splat-0f5cfe33-row_19 "section:row_19") |
| ` row_20 ` | ` 20 ` | ` 6 ` | ` Subclass feature ` | ` 3 ` | ` 15 ` | ` 4 ` | ` 3 ` | ` 3 ` | ` 3 ` | ` 2 ` | [Rules](#^splat-0f5cfe33-row_20 "section:row_20") |

##### Row 1

^splat-0f5cfe33-row_1

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |
| ` 2 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_1_lay_on_hands ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_1_spellcasting ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/2/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_1_weapon_mastery ` |

##### Row 2

^splat-0f5cfe33-row_2

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_2/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 2 ` |
| ` 1 ` | ` grant_feature ` | ` 2 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_2_fighting_style ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_2/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_2_paladin_s_smite ` |

##### Row 3

^splat-0f5cfe33-row_3

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 3 ` |
| ` 1 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_3_channel_divinity ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_3_paladin_subclass ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_4_ability_score_improvement ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_5_extra_attack ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_5/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_5_faithful_steed ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_6_aura_of_protection ` |

##### Row 7

^splat-0f5cfe33-row_7

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_7/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 7 ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_4_ability_score_improvement ` |

##### Row 9

^splat-0f5cfe33-row_9

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_9/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 9 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_9/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/paladin_level_9_abjure_foes ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_10_aura_of_courage ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_11_radiant_strikes ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_4_ability_score_improvement ` |

##### Row 13

^splat-0f5cfe33-row_13

###### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/@row_13/grants -->

```yaml
[]
```

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
| ` key ` | ` dnd2024:class_features/paladin_level_14_restoring_touch ` |

##### Row 15

^splat-0f5cfe33-row_15

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_15/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 15 ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_4_ability_score_improvement ` |

##### Row 17

^splat-0f5cfe33-row_17

###### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/@row_17/grants -->

```yaml
[]
```

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
| ` key ` | ` dnd2024:class_features/paladin_level_18_aura_expansion ` |

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
| ` key ` | ` dnd2024:class_features/paladin_level_19_epic_boon ` |

##### Row 20

^splat-0f5cfe33-row_20

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_20/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 20 ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/paladin ` |

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
channel_divinity:
  kind: resource_capacity
  resource: channel_divinity
  recovery: owning_feature_contract
  level_change_refills: false
prepared_spells:
  kind: selection_capacity
  selection: prepared_spells
  eligibility: owning_feature_contract
proficiency_bonus:
  kind: bonus
  applies_to: proficiency
  use_class_row_only_if_single_class: true
spell_slots_1:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 1
  recovery: long_rest
spell_slots_2:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 2
  recovery: long_rest
spell_slots_3:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 3
  recovery: long_rest
spell_slots_4:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 4
  recovery: long_rest
spell_slots_5:
  kind: resource_capacity
  resource: spell_slot
  spell_level: 5
  recovery: long_rest
```
