---
splat_schema: 2
splat_id: 47f944dd-d983-5f6d-962f-459cbedb682c
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_5_tactical_shift
splat_name: 'Level 5: Tactical Shift'
splat_origin: dnd2024:reference/pages/048
splat_sources:
- SRD 5.2.1, pp. 48, https://www.dndbeyond.com/srd
---

# Level 5: Tactical Shift

## Source text

<!-- SRD page 48 -->

Whenever you activate your Second Wind with a Bonus Action, you can move up to half your Speed without provoking Opportunity Attacks.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_5_tactical_shift ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_5_tactical_shift ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` actor.second_wind_activated_with_bonus_action ` |

#### Optional movement

<!-- splat:yaml path=splat_rule/parameters/optional_movement -->

```yaml
maximum_feet:
  op: multiply
  args:
  - var: actor.speed
  - 0.5
opportunity_attacks: false
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` fighter ` |
| ` minimum_level ` | ` 5 ` |
