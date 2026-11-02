---
splat_schema: 1
splat_id: 7ad9f262-223a-55e6-a517-c4b8f0277e37
splat_kind: rule
splat_key: dnd2024:rules/exploration_time_limited_object_interactions
splat_name: Time-Limited Object Interactions
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# Time-Limited Object Interactions

## Source text

<!-- SRD page 12 -->

When time is short, such as in combat, interactions with objects are limited: one free interaction per turn. That interaction must occur during a creature’s movement or action. Any additional interactions require the Utilize action, as explained in “Combat” later in “Playing the Game.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_time_limited_object_interactions ` |
| ` implementation ` | ` dnd2024:rule_exploration_time_limited_object_interactions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` free_object_interactions_per_turn ` | ` 1 ` |
| ` during ` | ` ["movement", "action"] ` |
| ` additional_interaction_action ` | ` utilize ` |
| ` when ` | ` time_limited ` |
