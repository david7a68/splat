---
splat_schema: 1
splat_id: 1be93897-fd36-5693-a41f-a6efb156cc3d
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_19_epic_boon
  parameters:
    feat_choice:
      count: 1
      category_preference: epic_boon
      allowed_categories: any
      must_meet_prerequisites: true
    class_scope:
      class: barbarian
      minimum_level: 19
  implementation: dnd2024:class_features_barbarian_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_barbarian_level_19_epic_boon
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
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: ad07a4a9511d14fff2938d3bbc78784a268423835cf2c0d13866489990b293db
  parent_name: Barbarian
  line_span:
    first: 2444
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 19: Epic Boon You gain an Epic Boon feat (see “Feats”) or another feat of your choice
    for which you qualify. Boon of Irresistible Offense is recommended.'
  sha256: b19f1d7ec23f77085b40c2259261288bd8639b1cae0ef0cba14d87076892936a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 19: Epic Boon

<!-- SRD page 30 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of\
Irresistible Offense is recommended.\
