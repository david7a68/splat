---
splat_schema: 1
splat_id: b20c166f-8a50-5e0a-8568-5250aacd02d1
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_5_fast_movement
splat_name: 'Level 5: Fast Movement'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_5_fast_movement
  parameters:
    speed_bonus_feet: 10
    unless_armor_category: heavy
  implementation: dnd2024:class_features_barbarian_level_5_fast_movement
splat_contracts:
- id: dnd2024:class_features_barbarian_level_5_fast_movement
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.feature_gained
  capabilities:
  - character_features
  - effects
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_5_fast_movement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: a1e0c01f52b104da3508e6d39b61c3b36e68d0b0b3c643af881d60ab031dc1e1
  parent_name: Barbarian
  line_span:
    first: 2377
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 5: Fast Movement Your speed increases by 10 feet while you aren’t wearing Heavy armor.'
  sha256: 9ddcc9d343d16ec2faa621ac0a6979481a2752a8991fb615b893b828d6a1356c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Fast Movement

<!-- SRD page 29 -->

Your speed increases by 10 feet while you aren’t\
wearing Heavy armor.\
