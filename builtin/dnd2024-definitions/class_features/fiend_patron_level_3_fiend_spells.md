---
splat_schema: 1
splat_id: 12d4ee93-4ac1-51e8-ae82-b2f07e20f448
splat_kind: rule
splat_key: dnd2024:class_features/fiend_patron_level_3_fiend_spells
splat_name: 'Level 3: Fiend Spells'
splat_origin:
  key: dnd2024:reference/pages/076
splat_sources:
- title: SRD 5.2.1
  locator: pp. 76
  url: https://www.dndbeyond.com/srd
---

# Level 3: Fiend Spells

## Source text

<!-- SRD page 76 -->

The magic of your patron ensures you always have\
certain spells ready; when you reach a Warlock\
level specified in the Fiend Spells table, you thereaf-\
ter always have the listed spells prepared.\
Fiend Spells\
Warlock Level Spells\
3 Burning Hands, Command,\
Scorching Ray, Suggestion\
5 Fireball, Stinking Cloud\
7 Fire Shield, Wall of Fire\
9 Geas, Insect Plague\

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

<!-- splat:rows path=splat_rule/parameters/always_prepared -->

| Row | minimum_class_level |
| --- | --- |
| ` 0 ` | ` 3 ` |
| ` 1 ` | ` 5 ` |
| ` 2 ` | ` 7 ` |
| ` 3 ` | ` 9 ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/0/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/burning_hands ` |
| ` 1 ` | ` dnd2024:spells/command ` |
| ` 2 ` | ` dnd2024:spells/scorching_ray ` |
| ` 3 ` | ` dnd2024:spells/suggestion ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fireball ` |
| ` 1 ` | ` dnd2024:spells/stinking_cloud ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/fire_shield ` |
| ` 1 ` | ` dnd2024:spells/wall_of_fire ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared/3/spells -->

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
