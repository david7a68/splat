---
splat_schema: 1
splat_id: 7d02f8c2-4e0e-53c3-bbc4-f282c49e487e
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_4_ability_score_improvement
  parameters:
    class: paladin
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
      class: paladin
      minimum_level: 4
  implementation: dnd2024:class_features_paladin_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_paladin_level_4_ability_score_improvement
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
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: d30fd061ee5541704b0410af17a5d99849e855496cde453ea19946b4e2d16708
  parent_name: Paladin
  line_span:
    first: 4601
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Paladin levels
    8, 12, and 16.'
  sha256: 7fb9dba856237872204160a4301e06e3e7d7f560ddfdd686b855cbc8ecdadd67
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 55 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Paladin levels\
8, 12, and 16.\
