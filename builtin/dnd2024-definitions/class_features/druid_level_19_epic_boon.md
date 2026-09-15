---
splat_schema: 1
splat_id: 4e946f22-a250-5eea-b45c-ca387d83d084
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_19_epic_boon
  parameters:
    minimum_class_level: 19
    feat_choice:
      count: 1
      category: any
      require_prerequisites: true
      suggested_category: epic_boon
  implementation: dnd2024:class_features_druid_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_druid_level_19_epic_boon
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
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 299ef29b2a0176522c9d348ff87ba3f05d47c3abbd74cd1957561087dff7a0fe
  parent_name: Druid
  line_span:
    first: 3615
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

<!-- SRD page 43 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of Di-\
mensional Travel is recommended.\
