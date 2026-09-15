---
splat_schema: 1
splat_id: b2845038-28aa-5719-81d6-6613f620678f
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_15_slippery_mind
splat_name: 'Level 15: Slippery Mind'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_15_slippery_mind
  parameters:
    saving_throw_proficiencies:
    - wisdom
    - charisma
  implementation: dnd2024:class_features_rogue_level_15_slippery_mind
splat_contracts:
- id: dnd2024:class_features_rogue_level_15_slippery_mind
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - proficiencies
  events:
  - character.feature_gained
  capabilities:
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_15_slippery_mind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: fd88fa5a6d62b5e9f1a817215111977466a195187bf13b19f8653ca730f83186
  parent_name: Rogue
  line_span:
    first: 5332
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 15: Slippery Mind Your cunning mind is exceptionally difficult to control. You gain proficiency
    in Wisdom and Charisma saving throws.'
  sha256: 5ecbc62208c563980af163318ba074411156aa0eaecc32d99571d514d5943bc8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 15: Slippery Mind

<!-- SRD page 63 -->

Your cunning mind is exceptionally difficult to con-\
trol. You gain proficiency in Wisdom and Charisma\
saving throws.\
