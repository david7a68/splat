---
splat_schema: 1
splat_id: ceb10285-1b13-5711-9a2e-fb5076221df7
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_reaction_and_bonus_action_triggers
splat_name: Reaction and Bonus Action Triggers
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
---

# Reaction and Bonus Action Triggers

## Source text

<!-- SRD page 105 -->

A spell that has a casting time of a Reaction is cast in response to a trigger that is defined in the spell’s Casting Time entry. Some spells that have a casting time of a Bonus Action are also cast in response to a trigger defined in the spell.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_reaction_and_bonus_action_triggers ` |
| ` implementation ` | ` dnd2024:rule_casting_spells_reaction_and_bonus_action_triggers ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` reaction_spell_requires_trigger ` | ` casting_time_entry ` |
| ` bonus_action_spell_requires_trigger_if_defined ` | ` true ` |
