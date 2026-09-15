---
splat_schema: 1
splat_id: 09a6667d-e827-5ada-98db-6fea9ecb97de
splat_kind: rule_group
splat_key: dnd2024:subclasses/ranger_ranger_subclass_hunter
splat_name: 'Ranger Subclass: Hunter'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
---

# Ranger Subclass: Hunter

## Source text

<!-- SRD page 61 -->

Protect Nature and People from Destruction\
You stalk prey in the wilds and elsewhere, using\
your abilities as a Hunter to protect nature and\
people everywhere from forces that would destroy\
them.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/ranger_ranger_subclass_hunter ` |
| ` implementation ` | ` dnd2024:subclasses_ranger_ranger_subclass_hunter ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 11 ` |
| ` 1 ` | ` 15 ` |
| ` 2 ` | ` 3 ` |
| ` 3 ` | ` 3 ` |
| ` 4 ` | ` 7 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/hunter_level_11_superior_hunter_s_prey ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/hunter_level_15_superior_hunter_s_defense ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/hunter_level_3_hunter_s_lore ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/hunter_level_3_hunter_s_prey ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/4/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/hunter_level_7_defensive_tactics ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |
