---
splat_schema: 2
splat_id: cca19111-402b-530c-8d0a-fcbe7562f7c7
splat_kind: rule
splat_key: dnd2024:rules/combat_combat_step_by_step
splat_name: Combat Step by Step
splat_origin: dnd2024:reference/pages/013
splat_sources:
- SRD 5.2.1, pp. 13, https://www.dndbeyond.com/srd
---

# Combat Step by Step

## Source text

<!-- SRD page 13 -->

Combat unfolds in these steps:

1: Establish Positions. The Game Master determines where all the characters and monsters are located. Given the adventurers’ marching order or their stated positions in the room or other location, the GM figures out where the adversaries are—how far away and in what direction.

2: Roll Initiative. Everyone involved in the combat encounter rolls Initiative, determining the order of combatants’ turns.

3: Take Turns. Each participant in the battle takes a turn in Initiative order. When everyone involved in the combat has had a turn, the round ends. Repeat this step until the fighting stops.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_combat_step_by_step ` |
| ` implementation ` | ` dnd2024:rules_combat_combat_step_by_step ` |

### Parameters

#### Sequence

<!-- splat:rows path=splat_rule/parameters/sequence -->

| Row | operation | authority | inputs | participants | order | round_end | repeat_until |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` assign_positions ` | ` gm ` | ` ["marching_order", "stated_positions"] ` |  |  |  |  |
| ` 1 ` | ` roll_initiative ` |  |  | ` all ` |  |  |  |
| ` 2 ` | ` take_turns ` |  |  |  | ` initiative ` | ` all_participants_acted ` | ` combat_ended ` |
