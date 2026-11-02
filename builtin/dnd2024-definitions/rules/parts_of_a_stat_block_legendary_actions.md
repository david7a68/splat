---
splat_schema: 1
splat_id: 14183f5d-ec29-567b-af1b-b8cf66161e9b
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_legendary_actions
splat_name: Legendary Actions
splat_origin:
  key: dnd2024:reference/pages/257
splat_sources:
- title: SRD 5.2.1
  locator: pp. 257
  url: https://www.dndbeyond.com/srd
---

# Legendary Actions

## Source text

<!-- SRD page 257 -->

If the monster has Legendary Action options, those are listed in this section. A Legendary Action is an action that a monster can take immediately after another creature’s turn. Only one of these actions can be taken at a time and only after another creature’s turn ends. The monster can’t take a Legendary Action if it has the Incapacitated condition or is otherwise unable to take actions.

The monster has a limited number of Legendary Action uses, and that number is specified in the stat block. The monster expends one use whenever it takes a Legendary Action, and it regains all expended uses at the start of each of its turns.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_legendary_actions ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_legendary_actions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` timing ` | ` immediately_after_another_creature_turn_ends ` |
| ` maximum_actions_per_trigger ` | ` 1 ` |
| ` prohibited_if_any ` | ` ["incapacitated", "unable_to_take_actions"] ` |
| ` options_source ` | ` stat_block_legendary_actions ` |
| ` uses_maximum_source ` | ` stat_block ` |
| ` uses_spent_per_action ` | ` 1 ` |

#### Recharge

<!-- splat:values path=splat_rule/parameters/recharge -->

| Field | Value |
| --- | --- |
| ` timing ` | ` start_of_monster_turn ` |
| ` amount ` | ` all_expended_uses ` |
