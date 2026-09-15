---
splat_schema: 1
splat_id: 5f8c100e-d4a7-5610-84ad-e82cee6acd16
splat_kind: rule
splat_key: dnd2024:rules/combat_breaking_up_your_move
splat_name: Breaking Up Your Move
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Breaking Up Your Move

## Source text

<!-- SRD page 14 -->

You can break up your move, using some of its\
movement before and after any action, Bonus Ac-\
tion, or Reaction you take on the same turn. For\
example, if you have a Speed of 30 feet, you could go\
10 feet, take an action, and then go 20 feet.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_breaking_up_your_move ` |
| ` implementation ` | ` dnd2024:rules_combat_breaking_up_your_move ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_segments_allowed ` | ` true ` |
| ` interleave_with ` | ` ["action", "bonus_action", "reaction"] ` |
| ` turn_scope ` | ` same_turn ` |
| ` movement_budget ` | ` remaining_movement ` |
