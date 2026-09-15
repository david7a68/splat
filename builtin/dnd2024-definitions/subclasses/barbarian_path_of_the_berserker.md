---
splat_schema: 1
splat_id: d3008830-4514-5e18-a003-b735f2ac7932
splat_kind: rule_group
splat_key: dnd2024:subclasses/barbarian_path_of_the_berserker
splat_name: Path of the Berserker
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
---

# Path of the Berserker

## Source text

<!-- SRD page 30 -->

Channel Rage into Violent Fury\
Barbarians who walk the Path of the Berserker\
direct their Rage primarily toward violence. Their\
path is one of untrammeled fury, and they thrill in\
the chaos of battle as they allow their Rage to seize\
and empower them.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/barbarian_path_of_the_berserker ` |
| ` implementation ` | ` dnd2024:subclasses_barbarian_path_of_the_berserker ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` barbarian ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:rows path=splat_rule/parameters/feature_grants -->

| Row | class_level |
| --- | --- |
| ` 0 ` | ` 10 ` |
| ` 1 ` | ` 14 ` |
| ` 2 ` | ` 3 ` |
| ` 3 ` | ` 6 ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/0/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/path_of_the_berserker_level_10_retaliation ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/1/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/2/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/path_of_the_berserker_level_3_frenzy ` |

##### Definition

<!-- splat:values path=splat_rule/parameters/feature_grants/3/definition -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |
