---
splat_schema: 1
splat_id: 032de82b-7e06-5aa9-9bff-01fee995bcc1
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_18_feral_senses
splat_name: 'Level 18: Feral Senses'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_18_feral_senses
  parameters:
    blindsight_feet: 30
  implementation: dnd2024:class_features_ranger_level_18_feral_senses
splat_contracts:
- id: dnd2024:class_features_ranger_level_18_feral_senses
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - character.feature_gained
  capabilities:
  - senses
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_18_feral_senses
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: bfeee375b55e1fc66ae6d624b745a088771a53010c560136a014c6d337931f75
  parent_name: Ranger
  line_span:
    first: 5002
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 18: Feral Senses Your connection to the forces of nature grants you Blindsight with a
    range of 30 feet.'
  sha256: b17db8f4bce94fffbd7110bcc0486b6f8db7dffd35a54145ea3e8d3f15d3984a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Feral Senses

<!-- SRD page 59 -->

Your connection to the forces of nature grants you\
Blindsight with a range of 30 feet.\
