---
splat_schema: 1
splat_id: 041d2982-f0cd-504e-85bd-9ad8aa43016e
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_4_ability_score_improvement
  parameters:
    class: ranger
    grant_at_levels:
    - 4
    - 8
    - 12
    - 16
    feat_choice:
      count_per_milestone: 1
      category: any
      require_prerequisites: true
      default:
        key: dnd2024:feats/general_feats_ability_score_improvement
  implementation: dnd2024:class_features_ranger_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_ranger_level_4_ability_score_improvement
  inputs:
    actor:
      type: participant
      required: true
    selections:
      required: false
      type: choice
  outputs:
  - feature_state
  - choices
  events:
  - character.level_gained
  capabilities:
  - character_choices
  - feats
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: b65069512ae0e9d8a6067315c11a0e7b1e2bfbcb793b0eba5527ccd34e99d68f
  parent_name: Ranger
  line_span:
    first: 4959
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Ranger levels
    8, 12, and 16.'
  sha256: dc09095ebfb0d74e13b6b2b12db101d0d3fcb452c8389efd2c26b1ad4c55cfae
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 59 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Ranger levels\
8, 12, and 16.\
