---
splat_schema: 1
splat_id: ec4d2bf7-8884-53b5-82af-780a5e103ebf
splat_kind: rule
splat_key: dnd2024:rules/monk_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/monk_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/monk
    entry_mode: multiclass
    hit_die_sides: 8
    armor_training: []
    skill_choice:
      count: 0
      options: null
    musical_instrument_choice_count: 0
    weapon_proficiencies: []
    grant_features_at_class_level: 1
    spell_slots: not_applicable
    level_one_features:
    - key: dnd2024:class_features/monk_level_1_martial_arts
    - key: dnd2024:class_features/monk_level_1_unarmored_defense
  implementation: dnd2024:rule_monk_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rule_monk_as_a_multiclass_character
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
  - 49
  engine_gaps: &id001
  - dnd2024:rule_monk_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 09c100e46d61a768b700941ca427a80e855a2116d229b5f92167a0b3eeb75b61
  parent_name: Monk
  line_span:
    first: 4115
    count: 5
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the Hit Point Die trait from the Core Monk Traits table. •
    Gain the Monk’s level 1 features, which are listed in the Monk Features table.'
  sha256: 4c20b50189f6db0979f1f5e0036de42d8e0cfc5859495ab0bcc468f35d554eaa
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 49 -->

•  Gain the Hit Point Die trait from the Core Monk\
Traits table.\
•  Gain the Monk’s level 1 features, which are listed\
in the Monk Features table.\
