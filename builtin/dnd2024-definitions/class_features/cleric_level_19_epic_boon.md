---
splat_schema: 1
splat_id: 36573d90-126d-562b-8b25-a704e0662e8e
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_19_epic_boon
  parameters:
    minimum_class_level: 19
    feat_choice:
      count: 1
      category: any
      require_prerequisites: true
      suggested_category: epic_boon
  implementation: dnd2024:class_features_cleric_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_cleric_level_19_epic_boon
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
  - 38
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: b4c258d263de2b342ca4514fc7ce9b18ad22953f90911da693e8a53131adf3f5
  parent_name: Cleric
  line_span:
    first: 3138
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 19: Epic Boon You gain an Epic Boon feat (see “Feats”) or another feat of your choice
    for which you qualify. Boon of Fate is recommended.'
  sha256: 98f4d20838b7a7fd33e0049c2879526f257319d9db5e8a845f59c21827314a8c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 19: Epic Boon

<!-- SRD page 38 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of\
Fate is recommended.\
