---
splat_schema: 1
splat_id: dc625776-3147-5cd3-98e7-20bf3e5cde4e
splat_kind: rule
splat_key: dnd2024:rules/wizard_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/077
splat_sources:
- title: SRD 5.2.1
  locator: pp. 77
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/wizard_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/wizard
    entry: multiclass
    hit_die_sides: 6
    armor_training: []
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/wizard_level_1_arcane_recovery
    - level: 1
      feature:
        key: dnd2024:class_features/wizard_level_1_ritual_adept
    - level: 1
      feature:
        key: dnd2024:class_features/wizard_level_1_spellcasting
    spell_slots_rule:
      key: dnd2024:rules/multiclassing_spellcasting
  implementation: dnd2024:rules_wizard_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rules_wizard_as_a_multiclass_character
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - multiclass_grants
  events: []
  capabilities:
  - multiclassing
  - character_features
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 77
  engine_gaps: &id001
  - dnd2024:rules_wizard_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 20813e445e25064d7a88a1a8d6ea1490bd18e6cf906f1af7b3aa77bd676023de
  parent_name: Wizard
  line_span:
    first: 6573
    count: 7
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the Hit Point Die from the Core Wizard Traits table. • Gain
    the Wizard’s level 1 features, which are listed in the Wizard Features table. See the multiclassing
    rules in “Character Creation” to determine your available spell slots.'
  sha256: 74f13ba2682aaeef45c7d45b1c0cd0f783fcb3c57bef6c5e5c579da51dc41093
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 77 -->

•  Gain the Hit Point Die from the Core Wizard Traits\
table.\
•  Gain the Wizard’s level 1 features, which are\
listed in the Wizard Features table. See the mul-\
ticlassing rules in “Character Creation” to deter-\
mine your available spell slots.\
