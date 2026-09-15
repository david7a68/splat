---
splat_schema: 1
splat_id: e742b115-ac2a-5f2d-b2b4-77ebb417f155
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
splat_name: 'Level 4: Ability Score Improvement'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_4_ability_score_improvement
  parameters:
    class: barbarian
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
  implementation: dnd2024:class_features_barbarian_level_4_ability_score_improvement
splat_contracts:
- id: dnd2024:class_features_barbarian_level_4_ability_score_improvement
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
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_4_ability_score_improvement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: e5989d84830135ecb557fc97e7dae84e81604317c560ca2458f9f011ed1c3e7d
  parent_name: Barbarian
  line_span:
    first: 2369
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 4: Ability Score Improvement You gain the Ability Score Improvement feat (see “Feats”)
    or another feat of your choice for which you qualify. You gain this feature again at Barbarian levels
    8, 12, and 16.'
  sha256: 7b46047da04916fc21502b9a93b377627329113e285e01c5211cabc6750d4635
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 4: Ability Score Improvement

<!-- SRD page 29 -->

You gain the Ability Score Improvement feat (see\
“Feats”) or another feat of your choice for which you\
qualify. You gain this feature again at Barbarian lev-\
els 8, 12, and 16.\
