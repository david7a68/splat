---
splat_schema: 1
splat_id: 3098f39d-b34d-58bd-a335-7d92cebdbfef
splat_kind: rule
splat_key: dnd2024:feats/general_feats_ability_score_improvement
splat_name: Ability Score Improvement
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/general_feats_ability_score_improvement
  parameters:
    feat_category: general
    prerequisites:
      minimum_character_level: 4
    repeatable: true
    ability_increase:
      patterns:
      - - 2
      - - 1
        - 1
      distinct_abilities: true
      eligible_abilities: all
      maximum_score: 20
  implementation: dnd2024:feats_general_feats_ability_score_improvement
splat_contracts:
- id: dnd2024:feats_general_feats_ability_score_improvement
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - ability_increases
  events:
  - feat.selected
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 87
  engine_gaps: &id001
  - dnd2024:feats_general_feats_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 434b0729714ea6e6232aee04440b742bdaf2e5c1dd877834527f8603fa93e4ec
  parent_name: General Feats
  line_span:
    first: 7521
    count: 7
splat_clauses:
- key: source_rules
  text: '# Ability Score Improvement General Feat (Prerequisite: Level 4+) Increase one ability score
    of your choice by 2, or increase two ability scores of your choice by 1. This feat can’t increase
    an ability score above 20. Repeatable. You can take this feat more than once.'
  sha256: 07ed1861b055b9df18be2acfe5a7930e8ee30d8741a7080abbee1995ee010862
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Score Improvement

<!-- SRD page 87 -->

General Feat (Prerequisite: Level 4+)\
Increase one ability score of your choice by 2, or\
increase two ability scores of your choice by 1. This\
feat can’t increase an ability score above 20.\
Repeatable. You can take this feat more than\
once.\
