---
splat_schema: 1
splat_id: 6ce21e0c-30de-5600-b1d3-e7d2f87973fb
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_4_ability_score_improvement
  parameters:
    class: rogue
    grant_at_class_levels:
    - 4
    - 8
    - 10
    - 12
    - 16
    feat_choice:
      count_each: 1
      default:
        key: dnd2024:feats/general_feats_ability_score_improvement
      allowed_categories: any
      must_meet_prerequisites: true
    class_scope:
      class: rogue
      minimum_level: 4
  implementation: dnd2024:class_features_rogue_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_rogue_level_4_ability_score_improvement
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
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 62966dd61d54f10dfa8e57f2d2eb95a5a72d793cfd4b41b865e2a04711daf995
  parent_name: Rogue
  line_span:
    first: 5262
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Rogue levels
    8, 10, 12, and 16.'
  sha256: b6382bcca077668b679d99b970313b64f75f3b35f2d243b8a1ef42ac67408db0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 63 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Rogue levels\
8, 10, 12, and 16.\
