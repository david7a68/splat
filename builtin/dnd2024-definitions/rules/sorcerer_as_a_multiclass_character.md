---
splat_schema: 1
splat_id: e2fe0d85-6fe9-5261-952f-2805d876fb5e
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/sorcerer
    entry_mode: multiclass
    hit_die_sides: 6
    armor_training: []
    skill_choice:
      count: 0
      options: null
    musical_instrument_choice_count: 0
    weapon_proficiencies: []
    grant_features_at_class_level: 1
    spell_slots:
      key: dnd2024:rules/multiclassing_spellcasting
    level_one_features:
    - key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
    - key: dnd2024:class_features/sorcerer_level_1_spellcasting
  implementation: dnd2024:rule_sorcerer_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rule_sorcerer_as_a_multiclass_character
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
  - 64
  engine_gaps: &id001
  - dnd2024:rule_sorcerer_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: c015204178b2bd74e64b142579752005665e73399414a3e3cc6818c83abaf44a
  parent_name: Sorcerer
  line_span:
    first: 5423
    count: 7
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the Hit Point Die from the Core Sorcerer Traits table. • Gain
    the Sorcerer’s level 1 features, which are listed in the Sorcerer Features table. See the multiclassing
    rules in “Character Creation” to determine your available spell slots.'
  sha256: b4a107aaba8700c92ce6b46444253cc1c0ed1073781d401f09dd5fa447310a93
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 64 -->

•  Gain the Hit Point Die from the Core Sorcerer\
Traits table.\
•  Gain the Sorcerer’s level 1 features, which are\
listed in the Sorcerer Features table. See the mul-\
ticlassing rules in “Character Creation” to deter-\
mine your available spell slots.\
