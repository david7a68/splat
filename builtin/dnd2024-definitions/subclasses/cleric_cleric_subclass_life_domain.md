---
splat_schema: 1
splat_id: bf2323c0-8261-5c65-8ab4-dc2ad40e1c6c
splat_kind: rule_group
splat_key: dnd2024:subclasses/cleric_cleric_subclass_life_domain
splat_name: 'Cleric Subclass: Life Domain'
splat_origin:
  key: dnd2024:reference/pages/040
splat_sources:
- title: SRD 5.2.1
  locator: pp. 40
  url: https://www.dndbeyond.com/srd
---

# Cleric Subclass: Life Domain

## Source text

<!-- SRD page 40 -->

Soothe the Hurts of the World\
The Life Domain focuses on the positive energy that\
helps sustain all life in the multiverse. Clerics who\
tap into this domain are masters of healing, using\
that life force to cure many hurts.\
Existence itself relies on the positive energy as-\
sociated with this domain, so a Cleric of almost any\
religious tradition might choose it. This domain is\
particularly associated with agricultural deities,\
gods of healing or endurance, and gods of home and\
community. Religious orders of healing also seek\
the magic of this domain.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/life_domain_level_17_supreme_healing ` |
| ` 1 ` | ` dnd2024:class_features/life_domain_level_3_disciple_of_life ` |
| ` 2 ` | ` dnd2024:class_features/life_domain_level_3_life_domain_spells ` |
| ` 3 ` | ` dnd2024:class_features/life_domain_level_3_preserve_life ` |
| ` 4 ` | ` dnd2024:class_features/life_domain_level_6_blessed_healer ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/cleric_cleric_subclass_life_domain ` |
| ` implementation ` | ` dnd2024:subclasses_cleric_cleric_subclass_life_domain ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` cleric ` |
| ` subclass ` | ` life_domain ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- life_domain_level_17_supreme_healing
- life_domain_level_3_disciple_of_life
- life_domain_level_3_life_domain_spells
- life_domain_level_3_preserve_life
- life_domain_level_6_blessed_healer
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
| [Level 17: Supreme Healing](<../class_features/life_domain_level_17_supreme_healing.md> "dnd2024:class_features/life_domain_level_17_supreme_healing") | ` 17 ` |
| [Level 3: Disciple of Life](<../class_features/life_domain_level_3_disciple_of_life.md> "dnd2024:class_features/life_domain_level_3_disciple_of_life") | ` 3 ` |
| [Level 3: Life Domain Spells](<../class_features/life_domain_level_3_life_domain_spells.md> "dnd2024:class_features/life_domain_level_3_life_domain_spells") | ` 3 ` |
| [Level 3: Preserve Life](<../class_features/life_domain_level_3_preserve_life.md> "dnd2024:class_features/life_domain_level_3_preserve_life") | ` 3 ` |
| [Level 6: Blessed Healer](<../class_features/life_domain_level_6_blessed_healer.md> "dnd2024:class_features/life_domain_level_6_blessed_healer") | ` 6 ` |
