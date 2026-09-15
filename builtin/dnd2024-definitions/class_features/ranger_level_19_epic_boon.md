---
splat_schema: 1
splat_id: 3dcd6117-93dd-5e49-8f3c-b5a44828bfe8
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_19_epic_boon
  parameters:
    feat_choice:
      count: 1
      category_preference: epic_boon
      allowed_categories: any
      must_meet_prerequisites: true
    class_scope:
      class: ranger
      minimum_level: 19
  implementation: dnd2024:class_features_ranger_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_ranger_level_19_epic_boon
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
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: e9e1e3d2e89968d90b0615545e5a171ad757da0031b47ba8cfd636271588362e
  parent_name: Ranger
  line_span:
    first: 5005
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 19: Epic Boon You gain an Epic Boon feat (see “Feats”) or another feat of your choice
    for which you qualify. Boon of Dimensional Travel is recommended.'
  sha256: 05ad26dc3a67865b66b59e906d30f9dbd1fb686bd7170a815098a9739a9f96a6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 19: Epic Boon

<!-- SRD page 59 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of Di-\
mensional Travel is recommended.\
