---
splat_schema: 1
splat_id: b2b774d9-ecd2-5cd8-aaa3-e53ec6194472
splat_kind: rule
splat_key: dnd2024:class_features/life_domain_level_3_life_domain_spells
splat_name: 'Level 3: Life Domain Spells'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
---

# Level 3: Life Domain Spells

## Source text

<!-- SRD page 40 -->

Your connection to this divine domain ensures you always have certain spells ready. When you reach a Cleric level specified in the Life Domain Spells table, you thereafter always have the listed spells prepared.

**Life Domain Spells**

| Cleric Level | Prepared Spells |
| --- | --- |
| 3 | Aid, Bless, Cure Wounds, Lesser Restoration |
| 5 | Mass Healing Word, Revivify |
| 7 | Aura of Life, Death Ward |
| 9 | Greater Restoration, Mass Cure Wounds |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/life_domain_level_3_life_domain_spells ` |
| ` implementation ` | ` dnd2024:class_features_life_domain_level_3_life_domain_spells ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` retain_at_higher_levels ` | ` true ` |
| ` counts_toward_class_prepared_limit ` | ` false ` |

#### Always prepared by class level

<!-- splat:table
path: splat_rule/parameters/always_prepared_by_class_level
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Level:
    path: level
    type: integer
  Details:
    type: section
    optional: true
-->

| Key | Level | Details |
| --- | --- | --- |
| ` row_1 ` | ` 3 ` | [Rules](#^splat-a3ddf6f9-row_1 "section:row_1") |
| ` row_2 ` | ` 5 ` | [Rules](#^splat-a3ddf6f9-row_2 "section:row_2") |
| ` row_3 ` | ` 7 ` | [Rules](#^splat-a3ddf6f9-row_3 "section:row_3") |
| ` row_4 ` | ` 9 ` | [Rules](#^splat-a3ddf6f9-row_4 "section:row_4") |

##### Row 1

^splat-a3ddf6f9-row_1

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/@row_1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/aid ` |
| ` 1 ` | ` dnd2024:spells/bless ` |
| ` 2 ` | ` dnd2024:spells/cure_wounds ` |
| ` 3 ` | ` dnd2024:spells/lesser_restoration ` |

##### Row 2

^splat-a3ddf6f9-row_2

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/@row_2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/mass_healing_word ` |
| ` 1 ` | ` dnd2024:spells/revivify ` |

##### Row 3

^splat-a3ddf6f9-row_3

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/@row_3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/aura_of_life ` |
| ` 1 ` | ` dnd2024:spells/death_ward ` |

##### Row 4

^splat-a3ddf6f9-row_4

###### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/@row_4/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/greater_restoration ` |
| ` 1 ` | ` dnd2024:spells/mass_cure_wounds ` |
