---
splat_schema: 1
splat_id: aa86c9ec-b987-5ad4-bc95-7b76041a0846
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_4_ability_score_improvement
  parameters:
    class: monk
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
      class: monk
      minimum_level: 4
  implementation: dnd2024:class_features_monk_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_monk_level_4_ability_score_improvement
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
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: da7425597fe2f5dd24c45fd87774c478a160b1941036115bab97fd9ed78f222d
  parent_name: Monk
  line_span:
    first: 4261
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Monk levels 8,
    12, and 16.'
  sha256: 3b4734c4a52278db8227a652d4b6cdd571a480867b3b3d1fb050c7f30c37951b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 51 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Monk levels 8,\
12, and 16.\
