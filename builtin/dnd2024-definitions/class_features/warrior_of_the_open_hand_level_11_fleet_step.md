---
splat_schema: 1
splat_id: 9ccb7da8-f94b-586e-af14-bd64723d0330
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step
splat_name: 'Level 11: Fleet Step'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
---

# Level 11: Fleet Step

## Source text

<!-- SRD page 52 -->

When you take a Bonus Action other than Step of the Wind, you can also use Step of the Wind immediately after that Bonus Action.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step ` |
| ` implementation ` | ` dnd2024:class_features_warrior_of_the_open_hand_level_11_fleet_step ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.bonus_action_completed ` |
| ` exclude ` | ` step_of_the_wind ` |

#### Grant

<!-- splat:yaml path=splat_rule/parameters/grant -->

```yaml
feature:
  key: dnd2024:class_features/monk_level_2_monk_s_focus
option: step_of_the_wind
timing: immediately_after_trigger
additional_bonus_action_cost: false
normal_optional_focus_costs_apply: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 11 ` |
