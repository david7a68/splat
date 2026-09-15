---
splat_schema: 1
splat_id: ea2df4c2-7d5f-5022-a797-414b2062b102
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/079
splat_sources:
- title: SRD 5.2.1
  locator: pp. 79
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_4_ability_score_improvement
  parameters:
    class: wizard
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
      class: wizard
      minimum_level: 4
  implementation: dnd2024:class_features_wizard_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_wizard_level_4_ability_score_improvement
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
  - 79
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 5b9f4a9e9d8a56b990484285dcbf1addae7d2193e32e5e73bd9aeb0ec7a9aee2
  parent_name: Wizard
  line_span:
    first: 6727
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Wizard levels
    8, 12, and 16.'
  sha256: cc98b2f0972ca26542be48a8867ea4f267a6dfb0e6926f09a17853d06ceb1261
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 79 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Wizard levels\
8, 12, and 16.\
