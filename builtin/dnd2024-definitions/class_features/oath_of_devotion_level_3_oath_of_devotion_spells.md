---
splat_schema: 1
splat_id: b08a4a47-e37e-55ae-85de-18860c15fb64
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells
splat_name: 'Level 3: Oath of Devotion Spells'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
---

# Level 3: Oath of Devotion Spells

## Source text

<!-- SRD page 56 -->

The magic of your oath ensures you always have\
certain spells ready; when you reach a Paladin level\
specified in the Oath of Devotion Spells table, you\
thereafter always have the listed spells prepared.\
Oath of Devotion Spells\
Paladin Level Spells\
3 Protection from Evil and Good,\
Shield of Faith\
5 Aid, Zone of Truth\
9 Beacon of Hope, Dispel Magic\
13 Freedom of Movement,\
Guardian of Faith\
17 Commune, Flame Strike\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/oath_of_devotion_level_3_oath_of_devotion_spells ` |
| ` implementation ` | ` dnd2024:class_features_oath_of_devotion_level_3_oath_of_devotion_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` retain_at_higher_levels ` | ` true ` |
| ` counts_toward_class_prepared_limit ` | ` false ` |

#### Always prepared by class level

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level -->

| Row | level |
| --- | --- |
| ` 0 ` | ` 3 ` |
| ` 1 ` | ` 5 ` |
| ` 2 ` | ` 9 ` |
| ` 3 ` | ` 13 ` |
| ` 4 ` | ` 17 ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/0/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/protection_from_evil_and_good ` |
| ` 1 ` | ` dnd2024:spells/shield_of_faith ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/aid ` |
| ` 1 ` | ` dnd2024:spells/zone_of_truth ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/beacon_of_hope ` |
| ` 1 ` | ` dnd2024:spells/dispel_magic ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/freedom_of_movement ` |
| ` 1 ` | ` dnd2024:spells/guardian_of_faith ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/4/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/commune ` |
| ` 1 ` | ` dnd2024:spells/flame_strike ` |
