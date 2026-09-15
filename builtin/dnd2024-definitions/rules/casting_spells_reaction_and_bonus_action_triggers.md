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
splat_rule:
  mechanic: dnd2024:rules/casting_spells_reaction_and_bonus_action_triggers
  parameters:
    reaction_spell_requires_trigger: casting_time_entry
    bonus_action_spell_requires_trigger_if_defined: true
  implementation: dnd2024:rule_casting_spells_reaction_and_bonus_action_triggers
splat_contracts:
- id: dnd2024:rule_casting_spells_reaction_and_bonus_action_triggers
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
  - reactions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rule_casting_spells_reaction_and_bonus_action_triggers
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 9c9e42b8b659f36d7f3a42339bcb82ed64335a36f8975af0c1ba4346ec8d246c
  parent_name: Casting Spells
  line_span:
    first: 9159
    count: 6
splat_clauses:
- key: source_rules
  text: '# Reaction and Bonus Action Triggers A spell that has a casting time of a Reaction is cast in
    response to a trigger that is defined in the spell’s Casting Time entry. Some spells that have a casting
    time of a Bonus Action are also cast in response to a trigger defined in the spell.'
  sha256: fa2b9d7655dc5d6f07251036ae8cfd5fc63f019cd343d96b2306b4edc9617405
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reaction and Bonus Action Triggers

<!-- SRD page 105 -->

A spell that has a casting time of a Reaction is cast\
in response to a trigger that is defined in the spell’s\
Casting Time entry. Some spells that have a casting\
time of a Bonus Action are also cast in response to a\
trigger defined in the spell.\
