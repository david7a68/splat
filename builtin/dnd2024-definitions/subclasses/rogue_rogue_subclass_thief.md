---
splat_schema: 2
splat_id: a00d37c5-3843-50df-8b89-64582a44a633
splat_kind: rule_group
splat_key: dnd2024:subclasses/rogue_rogue_subclass_thief
splat_name: 'Rogue Subclass: Thief'
splat_origin: dnd2024:reference/pages/064
splat_sources:
- SRD 5.2.1, pp. 64, https://www.dndbeyond.com/srd
---

# Rogue Subclass: Thief

## Source text

<!-- SRD page 64 -->

### Hunt for Treasure as a Classic Adventurer

A mix of burglar, treasure hunter, and explorer, you are the epitome of an adventurer. In addition to improving your agility and stealth, you gain abilities useful for delving into ruins and getting maximum benefit from the magic items you find there.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/thief_level_13_use_magic_device ` |
| ` 1 ` | ` dnd2024:class_features/thief_level_17_thief_s_reflexes ` |
| ` 2 ` | ` dnd2024:class_features/thief_level_3_fast_hands ` |
| ` 3 ` | ` dnd2024:class_features/thief_level_3_second_story_work ` |
| ` 4 ` | ` dnd2024:class_features/thief_level_9_supreme_sneak ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/rogue_rogue_subclass_thief ` |
| ` implementation ` | ` dnd2024:subclasses_rogue_rogue_subclass_thief ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` subclass ` | ` thief ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- thief_level_13_use_magic_device
- thief_level_17_thief_s_reflexes
- thief_level_3_fast_hands
- thief_level_3_second_story_work
- thief_level_9_supreme_sneak
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
| [Level 13: Use Magic Device](<../class_features/thief_level_13_use_magic_device.md> "dnd2024:class_features/thief_level_13_use_magic_device") | ` 13 ` |
| [Level 17: Thief’s Reflexes](<../class_features/thief_level_17_thief_s_reflexes.md> "dnd2024:class_features/thief_level_17_thief_s_reflexes") | ` 17 ` |
| [Level 3: Fast Hands](<../class_features/thief_level_3_fast_hands.md> "dnd2024:class_features/thief_level_3_fast_hands") | ` 3 ` |
| [Level 3: Second-Story Work](<../class_features/thief_level_3_second_story_work.md> "dnd2024:class_features/thief_level_3_second_story_work") | ` 3 ` |
| [Level 9: Supreme Sneak](<../class_features/thief_level_9_supreme_sneak.md> "dnd2024:class_features/thief_level_9_supreme_sneak") | ` 9 ` |
