---
splat_schema: 1
splat_id: 4080e354-b8a9-5ebf-9da8-b9b1499a1427
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_4_ability_score_improvement
  parameters:
    class: cleric
    grant_at_class_levels:
    - 4
    - 8
    - 12
    - 16
    feat_choice:
      count_each: 1
      default:
        key: dnd2024:feats/general_feats_ability_score_improvement
      allowed_categories: any
      must_meet_prerequisites: true
    class_scope:
      class: cleric
      minimum_level: 4
  implementation: dnd2024:class_features_cleric_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_cleric_level_4_ability_score_improvement
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - feat_grants
  events:
  - character.level_gained
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 37
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 305ce15a3915005e1d039847ff1a8fa4594cb144ba29538bdbbf05b47cbb81a2
  parent_name: Cleric
  line_span:
    first: 3097
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Cleric levels
    8, 12, and 16.'
  sha256: f823af8aa80b5dbda8acd2c897c74fcfeb7dccc3ee7ce09022c805000910958e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 37 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Cleric levels 8,\
12, and 16.\
