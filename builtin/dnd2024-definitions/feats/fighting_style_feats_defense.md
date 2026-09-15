---
splat_schema: 1
splat_id: f591f989-571f-5ebb-af12-81da3f512204
splat_kind: rule
splat_key: dnd2024:feats/fighting_style_feats_defense
splat_name: Defense
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/fighting_style_feats_defense
  parameters:
    feat_category: fighting_style
    prerequisite_feature: fighting_style
    when_armor_category:
    - light
    - medium
    - heavy
    armor_class_bonus: 1
  implementation: dnd2024:feats_fighting_style_feats_defense
splat_contracts:
- id: dnd2024:feats_fighting_style_feats_defense
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.feat_selected
  - armor_class.calculated
  capabilities:
  - feats
  - armor_class
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_fighting_style_feats_defense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c3b327ee266fefd2f22f1d96070c49338afb297a930b731d2a2926b235a248ee
  parent_name: Fighting Style Feats
  line_span:
    first: 7549
    count: 5
splat_clauses:
- key: source_rules
  text: '# Defense Fighting Style Feat (Prerequisite: Fighting Style Feature) While you’re wearing Light,
    Medium, or Heavy armor, you gain a +1 bonus to Armor Class.'
  sha256: f29a0ac05a7fd6ccfdb1dc9845f0ab4f15ed21a69ee3cc00d47078f4771b9b96
  status: contract_specified
splat_requires_mechanics: *id001
---

# Defense

<!-- SRD page 88 -->

Fighting Style Feat (Prerequisite: Fighting Style\
Feature)\
While you’re wearing Light, Medium, or Heavy ar-\
mor, you gain a +1 bonus to Armor Class.\
