---
splat_schema: 1
splat_id: e8199b30-6022-583c-9e63-a7d4cc96c741
splat_kind: rule
splat_key: dnd2024:rules/cleric_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/cleric_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/cleric
    entry_mode: multiclass
    hit_die_sides: 8
    armor_training:
    - light
    - medium
    - shield
    skill_choice:
      count: 0
      options: null
    musical_instrument_choice_count: 0
    weapon_proficiencies: []
    grant_features_at_class_level: 1
    spell_slots:
      key: dnd2024:rules/multiclassing_spellcasting
    level_one_features:
    - key: dnd2024:class_features/cleric_level_1_divine_order
    - key: dnd2024:class_features/cleric_level_1_spellcasting
  implementation: dnd2024:rule_cleric_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rule_cleric_as_a_multiclass_character
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - character_changes
  events: []
  capabilities:
  - multiclassing
  - classes
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 36
  engine_gaps: &id001
  - dnd2024:rule_cleric_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 81f10c60aa7c55c4651ebb9b7ec75e05d3f2dc8207e7da136c0ea589c02ffc02
  parent_name: Cleric
  line_span:
    first: 2957
    count: 8
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Cleric Traits table: Hit
    Point Die and training with Light and Medium armor and Shields. • Gain the Cleric’s level 1 features,
    which are listed in the Cleric Features table. See the multiclassing rules in “Character Creation”
    to determine your available spell slots.'
  sha256: c8995c8fcfea71e7984c691dfc4aeb2142d53eccbe6ba6fb8e2a8417a0e403ec
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 36 -->

•  Gain the following traits from the Core Cleric\
Traits table: Hit Point Die and training with Light\
and Medium armor and Shields.\
•  Gain the Cleric’s level 1 features, which are listed\
in the Cleric Features table. See the multiclassing\
rules in “Character Creation” to determine your\
available spell slots.\
