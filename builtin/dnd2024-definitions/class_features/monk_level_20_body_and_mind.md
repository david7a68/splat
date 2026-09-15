---
splat_schema: 1
splat_id: 7383c4de-2cb9-53e1-a6ed-aac188f9b9cd
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_20_body_and_mind
splat_name: 'Level 20: Body and Mind'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_20_body_and_mind
  parameters:
    ability_increases:
    - ability: dexterity
      amount: 4
      maximum: 25
    - ability: wisdom
      amount: 4
      maximum: 25
    class_scope:
      class: monk
      minimum_level: 20
  implementation: dnd2024:class_features_monk_level_20_body_and_mind
splat_contracts:
- id: dnd2024:class_features_monk_level_20_body_and_mind
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.feature_acquired
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_20_body_and_mind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: a9238874937527cb8799204923349a32c3cbe004e063115963add77a90d29788
  parent_name: Monk
  line_span:
    first: 4346
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 20: Body and Mind You have developed your body and mind to new heights. Your Dexterity
    and Wisdom scores increase by 4, to a maximum of 25.'
  sha256: d68d3788b282f41a8554d43a7489298e3a0c2e311125758665c7845a1375c117
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Body and Mind

<!-- SRD page 52 -->

You have developed your body and mind to new\
heights. Your Dexterity and Wisdom scores increase\
by 4, to a maximum of 25.\
