---
splat_schema: 1
splat_id: dabd8d80-9888-5295-84df-53ba331c6743
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_4_ability_score_improvement
  parameters:
    class: druid
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
      class: druid
      minimum_level: 4
  implementation: dnd2024:class_features_druid_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_druid_level_4_ability_score_improvement
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
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 77cd4e65cf328716b7dd0ab4a18379fc2cd7c6e0516c49085745dc80a27b51be
  parent_name: Druid
  line_span:
    first: 3578
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Druid levels
    8, 12, and 16.'
  sha256: 26a649b06db3faef906fa875867143a4c0ee30e19f36bd5aec56a1458cb3af1f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 43 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Druid levels 8,\
12, and 16.\
