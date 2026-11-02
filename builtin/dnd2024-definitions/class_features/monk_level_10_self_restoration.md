---
splat_schema: 1
splat_id: 93e92cf7-fb60-58fc-b95f-3470fbd3f6d6
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_10_self_restoration
splat_name: 'Level 10: Self-Restoration'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
---

# Level 10: Self-Restoration

## Source text

<!-- SRD page 52 -->

Through sheer force of will, you can remove one of the following conditions from yourself at the end of each of your turns: Charmed, Frightened, or Poisoned.

In addition, forgoing food and drink doesn’t give you levels of Exhaustion.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/monk_level_10_self_restoration ` |
| ` implementation ` | ` dnd2024:class_features_monk_level_10_self_restoration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` exhaustion_prevention_sources ` | ` ["lack_of_food", "lack_of_drink"] ` |

#### Condition removal

<!-- splat:values path=splat_rule/parameters/condition_removal -->

| Field | Value |
| --- | --- |
| ` event ` | ` actor.turn.end ` |
| ` optional ` | ` true ` |
| ` choose_maximum ` | ` 1 ` |
| ` conditions ` | ` ["charmed", "frightened", "poisoned"] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` monk ` |
| ` minimum_level ` | ` 10 ` |
