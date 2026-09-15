---
splat_schema: 1
splat_id: 433830ab-3fc4-5e39-84bb-882a97aa6945
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_4_ability_score_improvement
  parameters:
    class: fighter
    grant_at_class_levels:
    - 4
    - 6
    - 8
    - 12
    - 14
    - 16
    feat_choice:
      count_each: 1
      default:
        key: dnd2024:feats/general_feats_ability_score_improvement
      allowed_categories: any
      must_meet_prerequisites: true
    class_scope:
      class: fighter
      minimum_level: 4
  implementation: dnd2024:class_features_fighter_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_fighter_level_4_ability_score_improvement
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
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 3578e7b2b283b2e35a351bda3ec78d02d46a525d2fd731c4af8771c25fcc1d19
  parent_name: Fighter
  line_span:
    first: 4008
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Fighter levels
    6, 8, 12, 14, and 16.'
  sha256: beca968644342e36684100f8c47ca810420ec9de182110698477fb1da30fa6a6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 48 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Fighter levels\
6, 8, 12, 14, and 16.\
