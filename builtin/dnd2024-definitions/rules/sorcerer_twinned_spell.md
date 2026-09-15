---
splat_schema: 1
splat_id: bbc594f9-2784-5f07-9e9f-19b1a193f626
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_twinned_spell
splat_name: Twinned Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_twinned_spell
  parameters:
    cost_sorcery_points: 1
    spell_eligibility:
      higher_slot_level_can_add_creature_target: true
    effective_spell_level_increase: 1
  implementation: dnd2024:rule_sorcerer_twinned_spell
splat_contracts:
- id: dnd2024:rule_sorcerer_twinned_spell
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_modifications
  events:
  - spell.cast_pending
  capabilities:
  - metamagic
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 67
  engine_gaps: &id001
  - dnd2024:rule_sorcerer_twinned_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 1753d95203672d001d465fb7a6898874a208c36abc89b609089d1e46fae1454f
  parent_name: Sorcerer
  line_span:
    first: 5704
    count: 6
splat_clauses:
- key: source_rules
  text: '# Twinned Spell Cost: 1 Sorcery Point When you cast a spell, such as Charm Person, that can be
    cast with a higher-level spell slot to target an additional creature, you can spend 1 Sorcery Point
    to increase the spell’s effective level by 1.'
  sha256: 0c15cb012664725f4b3972c42700b5c199b2658670e4608470b90817a44da334
  status: contract_specified
splat_requires_mechanics: *id001
---

# Twinned Spell

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you cast a spell, such as Charm Person, that\
can be cast with a higher-level spell slot to target an\
additional creature, you can spend 1 Sorcery Point\
to increase the spell’s effective level by 1.\
