---
splat_schema: 1
splat_id: 3a669937-792d-537d-bc50-56c3344e7a1f
splat_kind: rule_group
splat_key: dnd2024:subclasses/bard_college_of_lore
splat_name: College of Lore
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
---

# College of Lore

## Source text

<!-- SRD page 35 -->

Plumb the Depths of Magical Knowledge\
Bards of the College of Lore collect spells and se-\
crets from diverse sources, such as scholarly tomes,\
mystical rites, and peasant tales. The college’s mem-\
bers gather in libraries and universities to share\
their lore with one another. They also meet at festi-\
vals or affairs of state, where they can expose cor-\
ruption, unravel lies, and poke fun at self-important\
figures of authority.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/bard_college_of_lore ` |
| ` implementation ` | ` dnd2024:subclasses_bard_college_of_lore ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 14 ` |
| ` 1 ` | ` 3 ` |
| ` 2 ` | ` 3 ` |
| ` 3 ` | ` 6 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/college_of_lore_level_14_peerless_skill ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/college_of_lore_level_3_cutting_words ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/college_of_lore_level_6_magical_discoveries ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |
