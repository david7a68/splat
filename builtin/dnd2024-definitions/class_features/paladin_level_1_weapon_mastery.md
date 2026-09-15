---
splat_schema: 1
splat_id: 444170bd-6766-5fc5-b386-ecded380d0e0
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/054
splat_sources:
- title: SRD 5.2.1
  locator: pp. 54
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_1_weapon_mastery
  parameters:
    mastery_choice:
      count: 2
      category: weapon_kind
      requires_proficiency: true
    grants: chosen_weapons_mastery_properties
    replace:
      event: rest.long_rest_completed
      maximum: all
  implementation: dnd2024:class_features_paladin_level_1_weapon_mastery
splat_contracts:
- id: dnd2024:class_features_paladin_level_1_weapon_mastery
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
  - character.feature_gained
  - rest.long_rest_completed
  capabilities:
  - weapon_mastery
  - character_choices
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 54
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_1_weapon_mastery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: a7ab9e3529bbea56156f899d590ac3ed0b14c3e2a48649f74889d9b4c0fbee2e
  parent_name: Paladin
  line_span:
    first: 4531
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 1: Weapon Mastery Your training with weapons allows you to use the mastery properties
    of two kinds of weapons of your choice with which you have proficiency, such as Longswords and Javelins.
    Whenever you finish a Long Rest, you can change the kinds of weapons you chose. For example, you could
    switch to using the mastery properties of Halberds and Flails.'
  sha256: 613acc981358452764d2eb40b118acdeb04718a3bba309ec65b1b345fe1e01d8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Weapon Mastery

<!-- SRD page 54 -->

Your training with weapons allows you to use the\
mastery properties of two kinds of weapons of your\
choice with which you have proficiency, such as\
Longswords and Javelins.\
Whenever you finish a Long Rest, you can change\
the kinds of weapons you chose. For example, you\
could switch to using the mastery properties of Hal-\
berds and Flails.\
