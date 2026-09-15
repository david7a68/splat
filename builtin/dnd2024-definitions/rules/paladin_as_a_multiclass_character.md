---
splat_schema: 1
splat_id: f4ff983a-33bb-5a7b-9161-a0a881dee6b3
splat_kind: rule
splat_key: dnd2024:rules/paladin_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/paladin_as_a_multiclass_character
  parameters:
    class_definition:
      key: dnd2024:classes/paladin
    grant_core_fields:
    - hit_point_die
    weapon_proficiencies:
    - martial
    armor_training:
    - light
    - medium
    - shield
    grant_features:
      class_level: 1
      from: class_definition
    spell_slots: multiclass_spellcasting_rules
  implementation: dnd2024:rules_paladin_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rules_paladin_as_a_multiclass_character
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
  - 53
  engine_gaps: &id001
  - dnd2024:rules_paladin_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 32a5ba666beb86137ff4aba8a29665b67baae1928ef9db050c32be393585e19c
  parent_name: Paladin
  line_span:
    first: 4443
    count: 9
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Paladin Traits table: Hit
    Point Die, proficiency with Martial weapons, and training with Light and Medium armor and Shields.
    • Gain the Paladin’s level 1 features, which are listed in the Paladin Features table. See the multiclassing
    rules in “Character Creation” to determine your available spell slots.'
  sha256: 1d57dad0b0ddbde135125b7130e5716fe95616a615dc302a15986f1dfb327101
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 53 -->

•  Gain the following traits from the Core Paladin\
Traits table: Hit Point Die, proficiency with Mar-\
tial weapons, and training with Light and Medium\
armor and Shields.\
•  Gain the Paladin’s level 1 features, which are\
listed in the Paladin Features table. See the multi-\
classing rules in “Character Creation” to deter-\
mine your available spell slots.\
