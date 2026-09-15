---
splat_schema: 1
splat_id: 91902e23-67dd-5e0b-a14e-b7494a6ef39b
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_19_epic_boon
  parameters:
    minimum_class_level: 19
    feat_choice:
      count: 1
      category: any
      require_prerequisites: true
      suggested_category: epic_boon
  implementation: dnd2024:class_features_paladin_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_paladin_level_19_epic_boon
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
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: b9b83f31ba647478d7ca53b1fbb951b3daf44a0fbc0230517eb4169e3dff64f1
  parent_name: Paladin
  line_span:
    first: 4658
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 19: Epic Boon You gain an Epic Boon feat (see “Feats”) or another feat of your choice
    for which you qualify. Boon of Truesight is recommended.'
  sha256: 4525cfa26f1c85ca456502ac2853b6b399e9be26f15a4573dddfbf4a83509f50
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 19: Epic Boon

<!-- SRD page 55 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of\
Truesight is recommended.\
