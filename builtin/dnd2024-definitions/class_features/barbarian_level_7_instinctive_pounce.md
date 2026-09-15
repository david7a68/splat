---
splat_schema: 1
splat_id: c6636112-4872-5fb4-828c-59f0d3d4780d
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
splat_name: 'Level 7: Instinctive Pounce'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_7_instinctive_pounce
  parameters:
    trigger: rage.entered
    action_cost: included_in_rage_bonus_action
    optional_move:
      maximum_feet:
        op: divide
        args:
        - stat: speed
        - 2
  implementation: dnd2024:class_features_barbarian_level_7_instinctive_pounce
splat_contracts:
- id: dnd2024:class_features_barbarian_level_7_instinctive_pounce
  inputs:
    actor:
      type: participant
      required: true
    destination:
      required: false
      type: location
  outputs:
  - feature_state
  - movement
  events:
  - rage.entered
  capabilities:
  - movement
  - action_economy
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_7_instinctive_pounce
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 42777cdafd05a32f68ffcd2ee80f72f83be4df3e8fb597955d55fe3a0754e3a8
  parent_name: Barbarian
  line_span:
    first: 2383
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 7: Instinctive Pounce As part of the Bonus Action you take to enter your Rage, you can
    move up to half your Speed.'
  sha256: d3a537e6b21e0539ac472e6cc109caa12ab8beb7912af108148b239485c3765f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Instinctive Pounce

<!-- SRD page 29 -->

As part of the Bonus Action you take to enter your\
Rage, you can move up to half your Speed.\
