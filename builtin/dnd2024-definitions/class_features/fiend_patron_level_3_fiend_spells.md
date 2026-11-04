---
splat_schema: 2
splat_id: 12d4ee93-4ac1-51e8-ae82-b2f07e20f448
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_3_fiend_spells
splat_name: 'Level 3: Fiend Spells'
splat_origin: dnd2024:reference/pages/076
splat_sources:
- SRD 5.2.1, pp. 76, https://www.dndbeyond.com/srd
---

# Level 3: Fiend Spells

## Source text

<!-- SRD page 76 -->

The magic of your patron ensures you always have certain spells ready; when you reach a Warlock level specified in the Fiend Spells table, you thereafter always have the listed spells prepared.

**Fiend Spells**

| Warlock Level | Spells |
| --- | --- |
| 3 | Burning Hands, Command, Scorching Ray, Suggestion |
| 5 | Fireball, Stinking Cloud |
| 7 | Fire Shield, Wall of Fire |
| 9 | Geas, Insect Plague |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fiend_patron_level_3_fiend_spells ` |
| ` implementation ` | ` dnd2024:class_features_fiend_patron_level_3_fiend_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` include_lower_levels ` | ` true ` |
| ` counts_against_prepared_limit ` | ` false ` |

#### Always prepared

<!-- splat:table
path: splat_rule/parameters/always_prepared
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Minimum class level:
    path: minimum_class_level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Minimum class level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` | [Rules](#^splat-403d5a47-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | [Rules](#^splat-403d5a47-row_2 "section:row_2") |
| ` row_3 ` | ` 7 ` | [Rules](#^splat-403d5a47-row_3 "section:row_3") |
| ` row_4 ` | ` 9 ` | [Rules](#^splat-403d5a47-row_4 "section:row_4") |

##### Row 1

^splat-403d5a47-row_1

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/burning_hands ` |
| ` 1 ` | ` dnd2024:spells/command ` |
| ` 2 ` | ` dnd2024:spells/scorching_ray ` |
| ` 3 ` | ` dnd2024:spells/suggestion ` |

##### Row 2

^splat-403d5a47-row_2

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fireball ` |
| ` 1 ` | ` dnd2024:spells/stinking_cloud ` |

##### Row 3

^splat-403d5a47-row_3

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fire_shield ` |
| ` 1 ` | ` dnd2024:spells/wall_of_fire ` |

##### Row 4

^splat-403d5a47-row_4

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/@row_4/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/geas ` |
| ` 1 ` | ` dnd2024:spells/insect_plague ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` warlock ` |
| ` minimum_level ` | ` 3 ` |
