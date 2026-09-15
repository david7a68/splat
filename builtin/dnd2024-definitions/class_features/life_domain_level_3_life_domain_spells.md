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

Your connection to this divine domain ensures you\
always have certain spells ready. When you reach\
a Cleric level specified in the Life Domain Spells\
table, you thereafter always have the listed spells\
prepared.\
Life Domain Spells\
Cleric Prepared Spells\
Level\
3 Aid, Bless, Cure Wounds,\
Lesser Restoration\
5 Mass Healing Word, Revivify\
7 Aura of Life, Death Ward\
9 Greater Restoration, Mass Cure Wounds\

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

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level -->

| Row | level |
| --- | --- |
| ` 0 ` | ` 3 ` |
| ` 1 ` | ` 5 ` |
| ` 2 ` | ` 7 ` |
| ` 3 ` | ` 9 ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/0/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/aid ` |
| ` 1 ` | ` dnd2024:spells/bless ` |
| ` 2 ` | ` dnd2024:spells/cure_wounds ` |
| ` 3 ` | ` dnd2024:spells/lesser_restoration ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/1/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/mass_healing_word ` |
| ` 1 ` | ` dnd2024:spells/revivify ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/2/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/aura_of_life ` |
| ` 1 ` | ` dnd2024:spells/death_ward ` |

##### Spells

<!-- splat:rows path=splat_rule/parameters/always_prepared_by_class_level/3/spells -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/greater_restoration ` |
| ` 1 ` | ` dnd2024:spells/mass_cure_wounds ` |
