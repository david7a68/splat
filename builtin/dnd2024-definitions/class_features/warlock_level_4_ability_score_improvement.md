---
splat_schema: 1
splat_id: beb8ffa3-89c5-542b-a44f-41c58aed6bc6
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warlock_level_4_ability_score_improvement
  parameters:
    class: warlock
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
  implementation: dnd2024:class_features_warlock_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_warlock_level_4_ability_score_improvement
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
  - 72
  engine_gaps: &id001
  - dnd2024:class_features_warlock_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: df437c9743837d4d84f09a03a3516e334fdf469d7d31570051028887c60fc860
  parent_name: Warlock
  line_span:
    first: 6103
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Warlock levels
    8, 12, and 16.'
  sha256: 0789942f68f0e51edf11be232043c65834a14501f793a2044ccfc8aab3104e48
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 72 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Warlock levels\
8, 12, and 16.\
