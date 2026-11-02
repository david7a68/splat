---
splat_schema: 1
splat_id: ff00ca23-2b98-5663-bd18-04b6c3208cd5
splat_kind: rule
splat_key: dnd2024:tables/warlock_progression
splat_name: Warlock Progression
splat_origin:
  key: dnd2024:reference/pages/071
splat_sources:
- title: SRD 5.2.1
  locator: pp. 71
  url: https://www.dndbeyond.com/srd
---

# Warlock Progression

## Source text

**Warlock Features**

| Level | Proficiency Bonus | Class Features | Eldritch Invocations | Cantrips | Prepared Spells | Pact Slots | Pact Slot Level |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | +2 | Eldritch Invocations, Pact Magic | 1 | 2 | 2 | 1 | 1 |
| 2 | +2 | Magical Cunning | 3 | 2 | 3 | 2 | 1 |
| 3 | +2 | Warlock Subclass | 3 | 2 | 4 | 2 | 2 |
| 4 | +2 | Ability Score Improvement | 3 | 3 | 5 | 2 | 2 |
| 5 | +3 | — | 5 | 3 | 6 | 2 | 3 |
| 6 | +3 | Subclass feature | 5 | 3 | 7 | 2 | 3 |
| 7 | +3 | — | 6 | 3 | 8 | 2 | 4 |
| 8 | +3 | Ability Score Improvement | 6 | 3 | 9 | 2 | 4 |
| 9 | +4 | Contact Patron | 7 | 3 | 10 | 2 | 5 |
| 10 | +4 | Subclass feature | 7 | 4 | 10 | 2 | 5 |
| 11 | +4 | Mystic Arcanum (level 6 spell) | 7 | 4 | 11 | 3 | 5 |
| 12 | +4 | Ability Score Improvement | 8 | 4 | 11 | 3 | 5 |
| 13 | +5 | Mystic Arcanum (level 7 spell) | 8 | 4 | 12 | 3 | 5 |
| 14 | +5 | Subclass feature | 8 | 4 | 12 | 3 | 5 |
| 15 | +5 | Mystic Arcanum (level 8 spell) | 9 | 4 | 13 | 3 | 5 |
| 16 | +5 | Ability Score Improvement | 9 | 4 | 13 | 3 | 5 |
| 17 | +6 | Mystic Arcanum (level 9 spell) | 9 | 4 | 14 | 4 | 5 |
| 18 | +6 | — | 10 | 4 | 14 | 4 | 5 |
| 19 | +6 | Epic Boon | 10 | 4 | 15 | 4 | 5 |
| 20 | +6 | Eldritch Master | 10 | 4 | 15 | 4 | 5 |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:tables/warlock_progression ` |
| ` implementation ` | ` dnd2024:class_progression_warlock_progression ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |

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
  Eldritch invocations:
    path: eldritch_invocations
    type: integer
  Cantrips:
    path: cantrips
    type: integer
  Prepared spells:
    path: prepared_spells
    type: integer
  Pact slots:
    path: pact_slots
    type: integer
  Pact slot level:
    path: pact_slot_level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Proficiency bonus | Features text | Eldritch invocations | Cantrips | Prepared spells | Pact slots | Pact slot level | Details |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 2 ` | ` Eldritch Invocations, Pact Magic ` | ` 1 ` | ` 2 ` | ` 2 ` | ` 1 ` | ` 1 ` | [Rules](#^splat-0f5cfe33-row_1 "section:row_1") |
| ` row_2 ` | ` 2 ` | ` 2 ` | ` Magical Cunning ` | ` 3 ` | ` 2 ` | ` 3 ` | ` 2 ` | ` 1 ` | [Rules](#^splat-0f5cfe33-row_2 "section:row_2") |
| ` row_3 ` | ` 3 ` | ` 2 ` | ` Warlock Subclass ` | ` 3 ` | ` 2 ` | ` 4 ` | ` 2 ` | ` 2 ` | [Rules](#^splat-0f5cfe33-row_3 "section:row_3") |
| ` row_4 ` | ` 4 ` | ` 2 ` | ` Ability Score Improvement ` | ` 3 ` | ` 3 ` | ` 5 ` | ` 2 ` | ` 2 ` | [Rules](#^splat-0f5cfe33-row_4 "section:row_4") |
| ` row_5 ` | ` 5 ` | ` 3 ` | ` — ` | ` 5 ` | ` 3 ` | ` 6 ` | ` 2 ` | ` 3 ` | [Rules](#^splat-0f5cfe33-row_5 "section:row_5") |
| ` row_6 ` | ` 6 ` | ` 3 ` | ` Subclass feature ` | ` 5 ` | ` 3 ` | ` 7 ` | ` 2 ` | ` 3 ` | [Rules](#^splat-0f5cfe33-row_6 "section:row_6") |
| ` row_7 ` | ` 7 ` | ` 3 ` | ` — ` | ` 6 ` | ` 3 ` | ` 8 ` | ` 2 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_7 "section:row_7") |
| ` row_8 ` | ` 8 ` | ` 3 ` | ` Ability Score Improvement ` | ` 6 ` | ` 3 ` | ` 9 ` | ` 2 ` | ` 4 ` | [Rules](#^splat-0f5cfe33-row_8 "section:row_8") |
| ` row_9 ` | ` 9 ` | ` 4 ` | ` Contact Patron ` | ` 7 ` | ` 3 ` | ` 10 ` | ` 2 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_9 "section:row_9") |
| ` row_10 ` | ` 10 ` | ` 4 ` | ` Subclass feature ` | ` 7 ` | ` 4 ` | ` 10 ` | ` 2 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_10 "section:row_10") |
| ` row_11 ` | ` 11 ` | ` 4 ` | ` Mystic Arcanum (level 6 spell) ` | ` 7 ` | ` 4 ` | ` 11 ` | ` 3 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_11 "section:row_11") |
| ` row_12 ` | ` 12 ` | ` 4 ` | ` Ability Score Improvement ` | ` 8 ` | ` 4 ` | ` 11 ` | ` 3 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_12 "section:row_12") |
| ` row_13 ` | ` 13 ` | ` 5 ` | ` Mystic Arcanum (level 7 spell) ` | ` 8 ` | ` 4 ` | ` 12 ` | ` 3 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_13 "section:row_13") |
| ` row_14 ` | ` 14 ` | ` 5 ` | ` Subclass feature ` | ` 8 ` | ` 4 ` | ` 12 ` | ` 3 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_14 "section:row_14") |
| ` row_15 ` | ` 15 ` | ` 5 ` | ` Mystic Arcanum (level 8 spell) ` | ` 9 ` | ` 4 ` | ` 13 ` | ` 3 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_15 "section:row_15") |
| ` row_16 ` | ` 16 ` | ` 5 ` | ` Ability Score Improvement ` | ` 9 ` | ` 4 ` | ` 13 ` | ` 3 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_16 "section:row_16") |
| ` row_17 ` | ` 17 ` | ` 6 ` | ` Mystic Arcanum (level 9 spell) ` | ` 9 ` | ` 4 ` | ` 14 ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_17 "section:row_17") |
| ` row_18 ` | ` 18 ` | ` 6 ` | ` — ` | ` 10 ` | ` 4 ` | ` 14 ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_18 "section:row_18") |
| ` row_19 ` | ` 19 ` | ` 6 ` | ` Epic Boon ` | ` 10 ` | ` 4 ` | ` 15 ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_19 "section:row_19") |
| ` row_20 ` | ` 20 ` | ` 6 ` | ` Eldritch Master ` | ` 10 ` | ` 4 ` | ` 15 ` | ` 4 ` | ` 5 ` | [Rules](#^splat-0f5cfe33-row_20 "section:row_20") |

##### Row 1

^splat-0f5cfe33-row_1

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_1/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 1 ` |
| ` 1 ` | ` grant_feature ` | ` 1 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_1_eldritch_invocations ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_1/grants/1/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_1_pact_magic ` |

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
| ` key ` | ` dnd2024:class_features/warlock_level_2_magical_cunning ` |

##### Row 3

^splat-0f5cfe33-row_3

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_3/grants -->

| Row | operation | at_class_level |
| --- | --- | --- |
| ` 0 ` | ` choose_subclass ` | ` 3 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_3/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_3_warlock_subclass ` |

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
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Row 5

^splat-0f5cfe33-row_5

###### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/@row_5/grants -->

```yaml
[]
```

##### Row 6

^splat-0f5cfe33-row_6

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_6/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 6 ` |

##### Row 7

^splat-0f5cfe33-row_7

###### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/@row_7/grants -->

```yaml
[]
```

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
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

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
| ` key ` | ` dnd2024:class_features/warlock_level_9_contact_patron ` |

##### Row 10

^splat-0f5cfe33-row_10

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_10/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 10 ` |

##### Row 11

^splat-0f5cfe33-row_11

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_11/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 11 ` | ` 6 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_11/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

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
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Row 13

^splat-0f5cfe33-row_13

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_13/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 13 ` | ` 7 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_13/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

##### Row 14

^splat-0f5cfe33-row_14

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_14/grants -->

| Row | operation | class_level |
| --- | --- | --- |
| ` 0 ` | ` grant_selected_subclass_features ` | ` 14 ` |

##### Row 15

^splat-0f5cfe33-row_15

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_15/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 15 ` | ` 8 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_15/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

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
| ` key ` | ` dnd2024:class_features/warlock_level_4_ability_score_improvement ` |

##### Row 17

^splat-0f5cfe33-row_17

###### Grants

<!-- splat:rows path=splat_rule/parameters/rows/@row_17/grants -->

| Row | operation | at_class_level | arcanum_spell_level |
| --- | --- | --- | --- |
| ` 0 ` | ` grant_feature ` | ` 17 ` | ` 9 ` |

###### Feature

<!-- splat:values path=splat_rule/parameters/rows/@row_17/grants/0/feature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |

##### Row 18

^splat-0f5cfe33-row_18

###### Grants

<!-- splat:yaml path=splat_rule/parameters/rows/@row_18/grants -->

```yaml
[]
```

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
| ` key ` | ` dnd2024:class_features/warlock_level_19_epic_boon ` |

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
| ` key ` | ` dnd2024:class_features/warlock_level_20_eldritch_master ` |

#### Class definition

<!-- splat:values path=splat_rule/parameters/class_definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/warlock ` |

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
cantrips:
  kind: selection_capacity
  selection: cantrips
  eligibility: owning_feature_contract
eldritch_invocations:
  kind: selection_capacity
  selection: eldritch_invocations
  eligibility: owning_feature_contract
pact_slot_level:
  kind: spell_level
  applies_to: pact_slots
pact_slots:
  kind: resource_capacity
  resource: pact_slots
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
```
