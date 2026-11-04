---
splat_schema: 2
splat_id: 9b2c8581-02be-5564-b8d4-1b6e6abd3e0c
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_14_disciplined_survivor
splat_name: 'Level 14: Disciplined Survivor'
splat_origin: dnd2024:reference/pages/052
splat_sources:
- SRD 5.2.1, pp. 52, https://www.dndbeyond.com/srd
---

# Level 14: Disciplined Survivor

## Source text

<!-- SRD page 52 -->

Your physical and mental discipline grant you proficiency in all saving throws.

Additionally, whenever you make a saving throw and fail, you can expend 1 Focus Point to reroll it, and you must use the new roll.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_14_disciplined_survivor ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_14_disciplined_survivor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` saving_throw_proficiency ` | ` all ` |

#### Failure reroll

<!-- splat:yaml path=splat_rule/parameters/failure_reroll -->

```yaml
optional: true
cost:
  resource: monk.focus_points
  amount: 1
must_use_new_roll: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 14 ` |
