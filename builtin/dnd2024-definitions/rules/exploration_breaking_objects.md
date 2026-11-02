---
splat_schema: 1
splat_id: 46e1c93e-f0f1-5a9e-b49e-72bd046929ef
splat_kind: rule
splat_key: dnd2024:rules/exploration_breaking_objects
splat_name: Breaking Objects
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# Breaking Objects

## Source text

<!-- SRD page 12 -->

As an action, you can automatically break or otherwise destroy a fragile, nonmagical object, such as a glass container or a piece of paper. If you try to damage something more resilient, the GM might use the rules on breaking objects in “Rules Glossary.”

### Marching Order

The adventurers should establish a marching order while they travel, whether indoors or outdoors. A marching order makes it easier to determine which characters are affected by traps, which ones can spot hidden enemies, and which ones are the closest to those enemies if a fight breaks out. You can change your marching order outside combat and record the order any way you like: write it down, for example, or arrange miniatures to show it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_breaking_objects ` |
| ` implementation ` | ` dnd2024:rule_exploration_breaking_objects ` |

### Parameters

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/breaking_objects ` |

#### Fragile nonmagical

<!-- splat:values path=splat_rule/parameters/fragile_nonmagical -->

| Field | Value |
| --- | --- |
| ` action_cost ` | ` 1 ` |
| ` automatically_destroy ` | ` true ` |

#### Marching order

<!-- splat:values path=splat_rule/parameters/marching_order -->

| Field | Value |
| --- | --- |
| ` ordered_participants ` | ` true ` |
| ` change_allowed ` | ` outside_combat ` |
| ` determines ` | ` ["trap_targets", "hidden_enemy_detection", "starting_distances"] ` |
