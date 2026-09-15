---
splat_schema: 1
splat_id: 657f9051-fe6c-5ebf-895a-fbd2eff8ebc7
splat_kind: rule
splat_key: dnd2024:rules/bard_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/bard_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/bard
    entry_mode: multiclass
    hit_die_sides: 8
    armor_training:
    - light
    skill_choice:
      count: 1
      options: all
    musical_instrument_choice_count: 1
    weapon_proficiencies: []
    grant_features_at_class_level: 1
    spell_slots:
      key: dnd2024:rules/multiclassing_spellcasting
    level_one_features:
    - key: dnd2024:class_features/bard_level_1_bardic_inspiration
    - key: dnd2024:class_features/bard_level_1_spellcasting
  implementation: dnd2024:rule_bard_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rule_bard_as_a_multiclass_character
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
  - 31
  engine_gaps: &id001
  - dnd2024:rule_bard_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: c0f4e0e0c753c2e194c368a4717f2bcc44266ab7eb4f85db4c68d83912e3b90f
  parent_name: Bard
  line_span:
    first: 2539
    count: 10
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Bard Traits table: Hit
    Point Die, proficiency in one skill of your choice, proficiency with one Musical Instrument of your
    choice, and training with Light armor. • Gain the Bard’s level 1 features, which are listed in the
    Bard Features table. See the multiclassing rules in “Character Creation” to determine your available
    spell slots.'
  sha256: d471b7d0188dcd78cf9dde2652f85d4bf1f40e36d534d4d1180dcd8508d1d39d
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 31 -->

•  Gain the following traits from the Core Bard\
Traits table: Hit Point Die, proficiency in one skill\
of your choice, proficiency with one Musical In-\
strument of your choice, and training with Light\
armor.\
•  Gain the Bard’s level 1 features, which are listed\
in the Bard Features table. See the multiclassing\
rules in “Character Creation” to determine your\
available spell slots.\
