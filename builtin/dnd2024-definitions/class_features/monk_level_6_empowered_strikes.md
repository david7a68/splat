---
splat_schema: 1
splat_id: 581fcd42-654d-51b2-9fa8-e5e5624a67cc
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_6_empowered_strikes
splat_name: 'Level 6: Empowered Strikes'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_6_empowered_strikes
  parameters:
    trigger: unarmed_strike.damage
    damage_type_choice:
    - force
    - normal
    class_scope:
      class: monk
      minimum_level: 6
  implementation: dnd2024:class_features_monk_level_6_empowered_strikes
splat_contracts:
- id: dnd2024:class_features_monk_level_6_empowered_strikes
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - damage.pending
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_6_empowered_strikes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 93b463c99ce23237c2643a99c91ba020010c6e65ee238dc3286e1516c30ac110
  parent_name: Monk
  line_span:
    first: 4283
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 6: Empowered Strikes Whenever you deal damage with your Unarmed Strike, it can deal your
    choice of Force damage or its normal damage type.'
  sha256: ec207fe8a1a104ae9580c31ae8732fdc9dd62afd4579dc0ae38579bf530eb9af
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Empowered Strikes

<!-- SRD page 51 -->

Whenever you deal damage with your Unarmed\
Strike, it can deal your choice of Force damage or its\
normal damage type.\
