---
splat_schema: 2
splat_id: bc135be7-382d-5214-94a8-11e493bc924d
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_3_steady_aim
splat_name: 'Level 3: Steady Aim'
splat_origin: dnd2024:reference/pages/062
splat_sources:
- SRD 5.2.1, pp. 62, https://www.dndbeyond.com/srd
---

# Level 3: Steady Aim

## Source text

<!-- SRD page 62 -->

As a Bonus Action, you give yourself Advantage on your next attack roll on the current turn. You can use this feature only if you haven’t moved during this turn, and after you use it, your Speed is 0 until the end of the current turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_3_steady_aim ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_3_steady_aim ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` requires_movement_so_far_this_turn ` | ` 0 ` |

#### Next attack roll

<!-- splat:values path=splat_rule/parameters/next_attack_roll -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |
| ` expires ` | ` current_turn_end ` |

#### Speed override

<!-- splat:values path=splat_rule/parameters/speed_override -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 0 ` |
| ` until ` | ` current_turn_end ` |
