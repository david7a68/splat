---
splat_schema: 1
splat_id: 8eba4d5b-02d3-5f10-bfd3-b5201f5d3732
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_9_acrobatic_movement
splat_name: 'Level 9: Acrobatic Movement'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_9_acrobatic_movement
  parameters:
    requires:
      armor_worn: false
      shield_wielded: false
      turn: actor
    movement_surfaces:
    - vertical
    - liquid
    falling_during_movement: false
    class_scope:
      class: monk
      minimum_level: 9
  implementation: dnd2024:class_features_monk_level_9_acrobatic_movement
splat_contracts:
- id: dnd2024:class_features_monk_level_9_acrobatic_movement
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - participant.moved
  capabilities:
  - character_features
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_9_acrobatic_movement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: b6afcca680c0660b2c3f79df7b36cdfd6b7eb29300ccb99eda39605a46419528
  parent_name: Monk
  line_span:
    first: 4294
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 9: Acrobatic Movement While you aren’t wearing armor or wielding a Shield, you gain the
    ability to move along vertical surfaces and across liquids on your turn without falling during the
    movement.'
  sha256: 97c0f230f7a919e27e1c350d921a22f104f127976a8bcb90194c5980936b9264
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Acrobatic Movement

<!-- SRD page 51 -->

While you aren’t wearing armor or wielding a\
Shield, you gain the ability to move along vertical\
surfaces and across liquids on your turn without\
falling during the movement.\
