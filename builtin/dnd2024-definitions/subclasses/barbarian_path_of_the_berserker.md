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

### Channel Rage into Violent Fury

Barbarians who walk the Path of the Berserker direct their Rage primarily toward violence. Their path is one of untrammeled fury, and they thrill in the chaos of battle as they allow their Rage to seize and empower them.

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

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- path_of_the_berserker_level_10_retaliation
- path_of_the_berserker_level_14_intimidating_presence
- path_of_the_berserker_level_3_frenzy
- path_of_the_berserker_level_6_mindless_rage
columns:
  Item:
    path: definition
    type: reference
  Class level:
    path: class_level
    type: integer
-->

| Item | Class level |
| --- | --- |
| [Level 10: Retaliation](<../class_features/path_of_the_berserker_level_10_retaliation.md> "dnd2024:class_features/path_of_the_berserker_level_10_retaliation") | ` 10 ` |
| [Level 14: Intimidating Presence](<../class_features/path_of_the_berserker_level_14_intimidating_presence.md> "dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence") | ` 14 ` |
| [Level 3: Frenzy](<../class_features/path_of_the_berserker_level_3_frenzy.md> "dnd2024:class_features/path_of_the_berserker_level_3_frenzy") | ` 3 ` |
| [Level 6: Mindless Rage](<../class_features/path_of_the_berserker_level_6_mindless_rage.md> "dnd2024:class_features/path_of_the_berserker_level_6_mindless_rage") | ` 6 ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |
