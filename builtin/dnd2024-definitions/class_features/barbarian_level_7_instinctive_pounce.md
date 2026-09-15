---
splat_schema: 1
splat_id: c6636112-4872-5fb4-828c-59f0d3d4780d
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
splat_name: 'Level 7: Instinctive Pounce'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
---

# Level 7: Instinctive Pounce

## Source text

<!-- SRD page 29 -->

As part of the Bonus Action you take to enter your\
Rage, you can move up to half your Speed.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/barbarian_level_7_instinctive_pounce ` |
| ` implementation ` | ` dnd2024:class_features_barbarian_level_7_instinctive_pounce ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` rage.entered ` |
| ` action_cost ` | ` included_in_rage_bonus_action ` |

#### Optional move

<!-- splat:yaml path=splat_rule/parameters/optional_move -->

```yaml
maximum_feet:
  op: divide
  args:
  - stat: speed
  - 2
```
