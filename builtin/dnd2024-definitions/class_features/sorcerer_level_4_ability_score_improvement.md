---
splat_schema: 1
splat_id: 4f35b9b4-6856-55e4-87f9-eb8552ff3fdc
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_4_ability_score_improvement
  parameters:
    class: sorcerer
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
  implementation: dnd2024:class_features_sorcerer_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_4_ability_score_improvement
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
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: a2fb50db9112f298daae70d7e576653c822e345b69865ee4ed228da537c4684d
  parent_name: Sorcerer
  line_span:
    first: 5602
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Sorcerer levels
    8, 12, and 16.'
  sha256: a144d339c893b5cb47307cc8b93aa3a9d61c3f8176ae2aa7d10820c1b57c53ab
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 66 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Sorcerer lev-\
els 8, 12, and 16.\
