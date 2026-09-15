---
splat_schema: 1
splat_id: 13143fc7-f575-57c2-8aba-013c1ad33eb0
splat_kind: rule_group
splat_key: dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand
splat_name: 'Monk Subclass: Warrior of the Open Hand'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
---

# Monk Subclass: Warrior of the Open Hand

## Source text

<!-- SRD page 52 -->

Master Unarmed Combat Techniques\
Warriors of the Open Hand are masters of unarmed\
combat. They learn techniques to push and trip\
their opponents and manipulate their own energy\
to protect themselves from harm.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step ` |
| ` 1 ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm ` |
| ` 2 ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique ` |
| ` 3 ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/monk_monk_subclass_warrior_of_the_open_hand ` |
| ` implementation ` | ` dnd2024:subclasses_monk_monk_subclass_warrior_of_the_open_hand ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` subclass ` | ` warrior_of_the_open_hand ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- warrior_of_the_open_hand_level_11_fleet_step
- warrior_of_the_open_hand_level_17_quivering_palm
- warrior_of_the_open_hand_level_3_open_hand_technique
- warrior_of_the_open_hand_level_6_wholeness_of_body
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
| [Level 11: Fleet Step](<../class_features/warrior_of_the_open_hand_level_11_fleet_step.md> "dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step") | ` 11 ` |
| [Level 17: Quivering Palm](<../class_features/warrior_of_the_open_hand_level_17_quivering_palm.md> "dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm") | ` 17 ` |
| [Level 3: Open Hand Technique](<../class_features/warrior_of_the_open_hand_level_3_open_hand_technique.md> "dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique") | ` 3 ` |
| [Level 6: Wholeness of Body](<../class_features/warrior_of_the_open_hand_level_6_wholeness_of_body.md> "dnd2024:class_features/warrior_of_the_open_hand_level_6_wholeness_of_body") | ` 6 ` |
