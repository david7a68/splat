---
splat_schema: 1
splat_id: 2537a1b0-19bf-59d6-ac0d-dbe5dc645806
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_20_foe_slayer
splat_name: 'Level 20: Foe Slayer'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_20_foe_slayer
  parameters:
    spell:
      key: dnd2024:spells/hunter_s_mark
    replace_damage_die_sides: 10
  implementation: dnd2024:class_features_ranger_level_20_foe_slayer
splat_contracts:
- id: dnd2024:class_features_ranger_level_20_foe_slayer
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - damage
  events:
  - spell.damage_rolled
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_20_foe_slayer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: abc378715e4a71db1c4022e5a04e0b4de12a5f9377a2888ff89f9318c927de46
  parent_name: Ranger
  line_span:
    first: 5009
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 20: Foe Slayer The damage die of your Hunter’s Mark is a d10 rather than a d6.'
  sha256: 399546a8fba433b5035cf5999276a7309f40454d2c0a659ecd131df616437b9c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Foe Slayer

<!-- SRD page 59 -->

The damage die of your Hunter’s Mark is a d10\
rather than a d6.\
