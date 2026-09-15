---
splat_schema: 1
splat_id: 2bac9887-e161-5870-b542-59c077890473
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/058
splat_sources:
- title: SRD 5.2.1
  locator: pp. 58, 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_1_weapon_mastery
  parameters:
    mastery_choice:
      count: 2
      category: weapon_kind
      requires_proficiency: true
    grants: chosen_weapons_mastery_properties
    replace:
      event: rest.long_rest_completed
      maximum: all
  implementation: dnd2024:class_features_ranger_level_1_weapon_mastery
splat_contracts:
- id: dnd2024:class_features_ranger_level_1_weapon_mastery
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
  - 58
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_1_weapon_mastery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 14d0c074ebfac876a55187824798860170ae3e121ff4804b33db72970f47eefe
  parent_name: Ranger
  line_span:
    first: 4922
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 1: Weapon Mastery Your training with weapons allows you to use the mastery properties
    of two kinds of weapons of your choice with which you have proficiency, such as Longbows and Shortswords.
    Whenever you finish a Long Rest, you can change the kinds of weapons you chose. For example, you could
    switch to using the mastery properties of Scimitars and Longswords.'
  sha256: b3a6d5362306de44aa9d9fc2c9443f72146773a5546a274fbc481ee1ac48963f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Weapon Mastery

<!-- SRD page 58 -->

Your training with weapons allows you to use the\
mastery properties of two kinds of weapons of your\

<!-- SRD page 59 -->

choice with which you have proficiency, such as\
Longbows and Shortswords.\
Whenever you finish a Long Rest, you can change\
the kinds of weapons you chose. For example, you\
could switch to using the mastery properties of\
Scimitars and Longswords.\
