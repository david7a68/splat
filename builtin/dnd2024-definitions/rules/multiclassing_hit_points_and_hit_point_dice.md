---
splat_schema: 1
splat_id: 7582bdb4-11fc-5bbb-b962-a27a64a4cea2
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_hit_points_and_hit_point_dice
splat_name: Hit Points and Hit Point Dice
splat_origin:
  key: dnd2024:reference/pages/025
splat_sources:
- title: SRD 5.2.1
  locator: pp. 25
  url: https://www.dndbeyond.com/srd
---

# Hit Points and Hit Point Dice

## Source text

<!-- SRD page 25 -->

You gain the Hit Points from your new class as described for levels after 1. You gain the level 1 Hit Points for a class only when your total character level is 1.

Add together the Hit Dice granted by all your classes to form your pool of Hit Dice. If these dice are the same die type, you can pool them together. For example, both the Fighter and the Paladin have a d10 Hit Die, so if you are a level 5 Fighter / level 5 Paladin, you have ten d10 Hit Dice. If your classes give you Hit Dice of different types, track them separately. If you are a level 5 Cleric / level 5 Paladin, for example, you have five d8 Hit Dice and five d10 Hit Dice.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_hit_points_and_hit_point_dice ` |
| ` implementation ` | ` dnd2024:rules_multiclassing_hit_points_and_hit_point_dice ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` new_class_at_later_character_level ` | ` normal_level_gain_hp ` |

#### Class level 1 maximum hit die hp only if

<!-- splat:values path=splat_rule/parameters/class_level_1_maximum_hit_die_hp_only_if -->

| Field | Value |
| --- | --- |
| ` total_character_level ` | ` 1 ` |

#### Hit dice

<!-- splat:values path=splat_rule/parameters/hit_dice -->

| Field | Value |
| --- | --- |
| ` sum_all_classes ` | ` true ` |
| ` pool_by_die_size ` | ` true ` |
| ` track_different_die_sizes_separately ` | ` true ` |
