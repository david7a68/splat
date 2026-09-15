---
splat_schema: 1
splat_id: 86f03b35-d50e-5368-bcab-f83001970d18
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_20_primal_champion
splat_name: 'Level 20: Primal Champion'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_20_primal_champion
  parameters:
    ability_score_increases:
    - ability: strength
      amount: 4
      maximum: 25
    - ability: constitution
      amount: 4
      maximum: 25
  implementation: dnd2024:class_features_barbarian_level_20_primal_champion
splat_contracts:
- id: dnd2024:class_features_barbarian_level_20_primal_champion
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
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_20_primal_champion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c8ff1ffdbf8054f5d6293cef24f4ec20feb5505a461ec796fb90ac06086a5e2e
  parent_name: Barbarian
  line_span:
    first: 2448
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 20: Primal Champion You embody primal power. Your Strength and Constitution scores increase
    by 4, to a maximum of 25.'
  sha256: b69071bfc6dcee1e4d5bbc64aac52140b6bc4b18f447c061bff2699fed3d7873
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Primal Champion

<!-- SRD page 30 -->

You embody primal power. Your Strength and Con-\
stitution scores increase by 4, to a maximum of 25.\
