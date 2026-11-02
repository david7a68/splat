---
splat_schema: 1
splat_id: 2d5dc600-fa36-5b63-a785-da2dcb2409a0
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_2_action_surge
splat_name: 'Level 2: Action Surge'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
---

# Level 2: Action Surge

## Source text

<!-- SRD page 48 -->

You can push yourself beyond your normal limits for a moment. On your turn, you can take one additional action, except the Magic action.

Once you use this feature, you can’t do so again until you finish a Short or Long Rest. Starting at level 17, you can use it twice before a rest but only once on a turn.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/fighter_level_2_action_surge ` |
| ` implementation ` | ` dnd2024:class_features_fighter_level_2_action_surge ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` on_own_turn ` | ` true ` |
| ` additional_actions ` | ` 1 ` |
| ` excluded_actions ` | ` ["magic"] ` |

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum_by_fighter_level:
  '2': 1
  '17': 2
recover:
  events:
  - rest.short_rest_completed
  - rest.long_rest_completed
  amount: all_expended
```

#### Limit

<!-- splat:values path=splat_rule/parameters/limit -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` period ` | ` turn ` |
