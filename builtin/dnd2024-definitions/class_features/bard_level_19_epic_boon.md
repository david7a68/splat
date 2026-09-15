---
splat_schema: 1
splat_id: 5f5d9eb3-212f-5ad9-84c5-e0b04c59865a
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/033
splat_sources:
- title: SRD 5.2.1
  locator: pp. 33
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_19_epic_boon
  parameters:
    feat_choice:
      count: 1
      category_preference: epic_boon
      allowed_categories: any
      must_meet_prerequisites: true
    class_scope:
      class: bard
      minimum_level: 19
  implementation: dnd2024:class_features_bard_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_bard_level_19_epic_boon
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.feature_acquired
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 33
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 12aa3b3e39001a63b9b506a55d8ca27ee80a60c246f28befc01c13622ace26ca
  parent_name: Bard
  line_span:
    first: 2702
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 19: Epic Boon You gain an Epic Boon feat (see “Feats”) or another feat of your choice
    for which you qualify. Boon of Spell Recall is recommended.'
  sha256: 6e51c8e37e4946bebfd3c2f66eed886fc9a54cb9e23f8882d2a1e845d1c0d37f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 19: Epic Boon

<!-- SRD page 33 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of\
Spell Recall is recommended.\
