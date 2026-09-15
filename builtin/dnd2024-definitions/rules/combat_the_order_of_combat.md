---
splat_schema: 1
splat_id: b54910bb-1adf-5923-8e2e-949f0a5e6630
splat_kind: rule
splat_key: dnd2024:rules/combat_the_order_of_combat
splat_name: The Order of Combat
splat_origin:
  key: dnd2024:reference/pages/013
splat_sources:
- title: SRD 5.2.1
  locator: pp. 13
  url: https://www.dndbeyond.com/srd
---

# The Order of Combat

## Source text

<!-- SRD page 13 -->

A typical combat encounter is a clash between two\
sides: a flurry of weapon swings, feints, parries,\
footwork, and spellcasting. The game organizes\
combat into a cycle of rounds and turns. A round\
represents about 6 seconds in the game world.\
During a round, each participant in a battle takes a\
turn. The order of turns is determined at the begin-\
ning of combat when everyone rolls Initiative. Once\
everyone has taken a turn, the fight continues to the\
next round if neither side is defeated.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_the_order_of_combat ` |
| ` implementation ` | ` dnd2024:rule_combat_the_order_of_combat ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` round_seconds ` | ` 6 ` |
| ` turns_per_round_per_participant ` | ` 1 ` |
| ` initial_order ` | ` initiative ` |
| ` repeat_rounds_until ` | ` side_defeated ` |
