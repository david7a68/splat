---
splat_schema: 1
splat_id: 6e0d3e4e-ef3d-50f9-a877-dd7560d45c5a
splat_kind: rule
splat_key: dnd2024:rules/warlock_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_as_a_multiclass_character
  parameters:
    class_definition:
      key: dnd2024:classes/warlock
    grant_core_fields:
    - hit_point_die
    weapon_proficiencies: []
    armor_training:
    - light
    grant_features:
      class_level: 1
      from: class_definition
    spell_slots: multiclass_spellcasting_rules
  implementation: dnd2024:rules_warlock_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rules_warlock_as_a_multiclass_character
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:rules_warlock_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 0f467c55b075b56be0fa1e467307998ac477ca055d591c4b8320a45f1e84687e
  parent_name: Warlock
  line_span:
    first: 5954
    count: 8
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Warlock Traits table: Hit
    Point Die and training with Light armor. • Gain the Warlock’s level 1 features, which are listed in
    the Warlock Features table. See the multiclassing rules in “Character Creation” to determine your
    available spell slots.'
  sha256: c9193780b332526a9d411b71725dd605de0f4432f5eb991f0c8bd27f29969f22
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 70 -->

•  Gain the following traits from the Core Warlock\
Traits table: Hit Point Die and training with Light\
armor.\
•  Gain the Warlock’s level 1 features, which are\
listed in the Warlock Features table. See the mul-\
ticlassing rules in “Character Creation” to deter-\
mine your available spell slots.\
