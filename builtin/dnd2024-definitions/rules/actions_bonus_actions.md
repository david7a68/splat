---
splat_schema: 2
splat_id: ba5b915c-9d78-54d3-9457-816e3aee8ffb
splat_kind: rule
splat_key: dnd2024:rules/actions_bonus_actions
splat_name: Bonus Actions
splat_origin: dnd2024:reference/pages/010
splat_sources:
- SRD 5.2.1, pp. 10, https://www.dndbeyond.com/srd
---

# Bonus Actions

## Source text

<!-- SRD page 10 -->

Various class features, spells, and other abilities let you take an additional action on your turn called a Bonus Action. The Cunning Action feature, for example, allows a Rogue to take a Bonus Action. You can take a Bonus Action only when a special ability, a spell, or another feature of the game states that you can do something as a Bonus Action. You otherwise don’t have a Bonus Action to take.

You can take only one Bonus Action on your turn, so you must choose which Bonus Action to use if you have more than one available.

You choose when to take a Bonus Action during your turn unless the Bonus Action’s timing is specified. Anything that deprives you of your ability to take actions also prevents you from taking a Bonus Action.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/actions_bonus_actions ` |
| ` implementation ` | ` dnd2024:rules_actions_bonus_actions ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` per_turn_limit ` | ` 1 ` |
| ` requires_grant ` | ` true ` |
| ` turn_scope ` | ` own ` |
| ` timing ` | ` actor_choice_unless_specified ` |
| ` blocked_by ` | ` cannot_take_actions ` |
