---
splat_schema: 1
splat_id: b6d9dcff-db11-5e04-95c4-42127237d86e
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/032
splat_sources:
- title: SRD 5.2.1
  locator: pp. 32
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_4_ability_score_improvement
  parameters:
    class: bard
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
  implementation: dnd2024:class_features_bard_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_bard_level_4_ability_score_improvement
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
  - 32
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: bae450e6207c5686cfac14b64cc7b86b51bacb70a9e163a221db139a86a1e505
  parent_name: Bard
  line_span:
    first: 2669
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Bard levels 8,
    12, and 16.'
  sha256: 75d387b8c62b3606a9393340462c71dd5bd07b3727302730d7cc0555d55093ce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 32 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Bard levels 8,\
12, and 16.\
