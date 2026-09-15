---
splat_schema: 1
splat_id: 850bcc82-f5d2-5fd2-bb97-a39bc776c3c4
splat_kind: rule_group
splat_key: dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery
splat_name: 'Sorcerer Subclass: Draconic Sorcery'
splat_origin:
  key: dnd2024:reference/pages/069
splat_sources:
- title: SRD 5.2.1
  locator: pp. 69
  url: https://www.dndbeyond.com/srd
---

# Sorcerer Subclass: Draconic Sorcery

## Source text

<!-- SRD page 69 -->

Breathe the Magic of Dragons\
Your innate magic comes from the gift of a dragon.\
Perhaps an ancient dragon facing death bequeathed\
some of its magical power to you or your ancestor.\
You might have absorbed magic from a site infused\
with dragons’ power. Or perhaps you handled a\
treasure taken from a dragon’s hoard that was\
steeped in draconic power. Or you might have a\
dragon for an ancestor.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/sorcerer_sorcerer_subclass_draconic_sorcery ` |
| ` implementation ` | ` dnd2024:subclasses_sorcerer_sorcerer_subclass_draconic_sorcery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` sorcerer ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 14 ` |
| ` 1 ` | ` 18 ` |
| ` 2 ` | ` 3 ` |
| ` 3 ` | ` 3 ` |
| ` 4 ` | ` 6 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/draconic_sorcery_level_14_dragon_wings ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/draconic_sorcery_level_18_dragon_companion ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/draconic_sorcery_level_3_draconic_resilience ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/draconic_sorcery_level_3_draconic_spells ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |
