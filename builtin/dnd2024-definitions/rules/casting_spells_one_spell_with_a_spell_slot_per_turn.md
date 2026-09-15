---
splat_schema: 1
splat_id: fd705c30-e0b4-553b-b0da-39bbfaed586c
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_one_spell_with_a_spell_slot_per_turn
splat_name: One Spell with a Spell Slot per Turn
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_one_spell_with_a_spell_slot_per_turn
  parameters:
    maximum_spell_slots_expended_to_cast_per_turn: 1
    scope: any_action_economy
  implementation: dnd2024:rules_casting_spells_one_spell_with_a_spell_slot_per_turn
splat_contracts:
- id: dnd2024:rules_casting_spells_one_spell_with_a_spell_slot_per_turn
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_pending
  capabilities:
  - spell_slots
  - action_economy
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_one_spell_with_a_spell_slot_per_turn
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: f46b0c768aad80e60f00c868e80714ed96fbeb533b62a3224edf5e05d01185b1
  parent_name: Casting Spells
  line_span:
    first: 9154
    count: 5
splat_clauses:
- key: source_rules
  text: '# One Spell with a Spell Slot per Turn On a turn, you can expend only one spell slot to cast
    a spell. This rule means you can’t, for example, cast a spell with a spell slot using the Magic action
    and another one using a Bonus Action on the same turn.'
  sha256: df993f81b6f513fc75732c606f1adb467722b89baa99beda5e7034bdd9be9874
  status: contract_specified
splat_requires_mechanics: *id001
---

# One Spell with a Spell Slot per Turn

<!-- SRD page 105 -->

On a turn, you can expend only one spell slot to cast\
a spell. This rule means you can’t, for example, cast\
a spell with a spell slot using the Magic action and\
another one using a Bonus Action on the same turn.\
