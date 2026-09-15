---
splat_schema: 1
splat_id: b35da6a2-3388-5b98-9f0e-d9470a955ba8
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_19_epic_boon
splat_name: 'Level 19: Epic Boon'
splat_origin:
  key: dnd2024:reference/pages/079
splat_sources:
- title: SRD 5.2.1
  locator: pp. 79
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_19_epic_boon
  parameters:
    minimum_class_level: 19
    feat_choice:
      count: 1
      category: any
      require_prerequisites: true
      suggested_category: epic_boon
  implementation: dnd2024:class_features_wizard_level_19_epic_boon
splat_contracts:
- id: dnd2024:class_features_wizard_level_19_epic_boon
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
  - 79
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_19_epic_boon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 84a0a380af212f7fdb8e68993cbb1b7faab5a001e7760ad093a3a1c79647178f
  parent_name: Wizard
  line_span:
    first: 6748
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

<!-- SRD page 79 -->

You gain an Epic Boon feat (see “Feats”) or another\
feat of your choice for which you qualify. Boon of\
Spell Recall is recommended.\
