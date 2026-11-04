---
splat_schema: 2
splat_id: 2c8a934c-b8f9-5a47-b274-d843b1687006
splat_kind: rule
splat_key: dnd2024:glossary/simultaneous_effects
splat_name: Simultaneous Effects
splat_origin: dnd2024:reference/glossary/simultaneous_effects
splat_sources:
- SRD 5.2.1, pp. 187, https://www.dndbeyond.com/srd
---

# Simultaneous Effects

## Source text

<!-- SRD page 187 -->

If two or more things happen at the same time on a turn, the person at the game table—player or GM— whose turn it is decides the order in which those things happen. For example, if two effects occur at the start of a player character’s turn, the player decides which of the effects happens first.

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/simultaneous_effects ` |
| ` implementation ` | ` dnd2024:glossary_simultaneous_effects ` |

### Parameters

#### When

<!-- splat:values path=splat_rule/parameters/when -->

| Field | Value |
| --- | --- |
| ` same_time ` | ` true ` |
| ` during_turn ` | ` true ` |
| ` minimum_effects ` | ` 2 ` |

#### Order choice

<!-- splat:values path=splat_rule/parameters/order_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` current_turn_creature_controller ` |
| ` options ` | ` all_permutations_of_simultaneous_effects ` |
