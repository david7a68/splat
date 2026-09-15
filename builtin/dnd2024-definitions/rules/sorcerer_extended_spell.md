---
splat_schema: 1
splat_id: 17f98164-e101-5653-98ef-d0b0deafa222
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_extended_spell
splat_name: Extended Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_extended_spell
  parameters:
    cost_sorcery_points: 1
    minimum_spell_duration_minutes: 1
    duration_multiplier: 2
    duration_maximum_hours: 24
    concentration_saves_for_affected_spell: advantage
  implementation: dnd2024:rule_sorcerer_extended_spell
splat_contracts:
- id: dnd2024:rule_sorcerer_extended_spell
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
  - concentration
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 67
  engine_gaps: &id001
  - dnd2024:rule_sorcerer_extended_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: bcce13f024a460f58f15f13a0d8f18a69840a502f426b01c39db570efc1b7e47
  parent_name: Sorcerer
  line_span:
    first: 5659
    count: 8
splat_clauses:
- key: source_rules
  text: '# Extended Spell Cost: 1 Sorcery Point When you cast a spell that has a duration of 1 minute
    or longer, you can spend 1 Sorcery Point to double its duration to a maximum duration of 24 hours.
    If the affected spell requires Concentration, you have Advantage on any saving throw you make to maintain
    that Concentration.'
  sha256: 890e5cd3c73dafa67442679afa8a89faf6c6969e63c06a8e55a06f9965880d40
  status: contract_specified
splat_requires_mechanics: *id001
---

# Extended Spell

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you cast a spell that has a duration of 1 min-\
ute or longer, you can spend 1 Sorcery Point to dou-\
ble its duration to a maximum duration of 24 hours.\
If the affected spell requires Concentration, you\
have Advantage on any saving throw you make to\
maintain that Concentration.\
