---
splat_schema: 1
splat_id: 0fceeadb-1b2e-5bef-955c-b63f7abd2bcc
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_19_epic_boon
  parameters:
    minimum_class_level: 19
    feat_choice:
      count: 1
      category: any
      require_prerequisites: true
      suggested_category: epic_boon
  implementation: dnd2024:class_features_fighter_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_fighter_level_19_epic_boon
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
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 25b819aeda742518258ec224e6b2b085d70f4aa7a4f6d6941ee4462710d754cf
  parent_name: Fighter
  line_span:
    first: 4041
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 19: Epic Boon You gain an Epic Boon feat (see “Feats”) or another feat of your choice
    for which you qualify. Boon of Combat Prowess is recommended.'
  sha256: 72040d4a0d02097be2694b5b5b8c792aa9d0e9fa5d0ff35008040bdaacae2afe
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 19: Epic Boon

<!-- SRD page 48 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of\
Combat Prowess is recommended.\
