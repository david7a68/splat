---
splat_schema: 1
splat_id: e082f276-e88d-55d2-bd1f-b0a8b6c23b6d
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_5_extra_attack
splat_name: 'Level 5: Extra Attack'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_5_extra_attack
  parameters:
    action: attack
    turn: actor
    attack_count: 2
    class_scope:
      class: ranger
      minimum_level: 5
  implementation: dnd2024:class_features_ranger_level_5_extra_attack
splat_contracts:
- id: dnd2024:class_features_ranger_level_5_extra_attack
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
  - action.attack
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
  - dnd2024:class_features_ranger_level_5_extra_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 00ec0b56098afeffac01bc3a2cf1a458e41a171c84ed8aa90dd5ca9c48391e86
  parent_name: Ranger
  line_span:
    first: 4964
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 5: Extra Attack You can attack twice instead of once whenever you take the Attack action
    on your turn.'
  sha256: 2c964ae8a86a549f83ed3a77ef83451b810103f825f5d8348ca3badddf25de65
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Extra Attack

<!-- SRD page 59 -->

You can attack twice instead of once whenever you\
take the Attack action on your turn.\
