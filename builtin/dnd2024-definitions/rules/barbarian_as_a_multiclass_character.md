---
splat_schema: 1
splat_id: f1eb1136-7b9b-5b53-9dfb-0e404ed5029f
splat_kind: rule
splat_key: dnd2024:rules/barbarian_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/barbarian_as_a_multiclass_character
  parameters:
    class_definition:
      key: dnd2024:classes/barbarian
    grant_core_fields:
    - hit_point_die
    weapon_proficiencies:
    - martial
    armor_training:
    - shield
    grant_features:
      class_level: 1
      from: class_definition
    spell_slots: not_applicable
  implementation: dnd2024:rules_barbarian_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rules_barbarian_as_a_multiclass_character
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
  - 28
  engine_gaps: &id001
  - dnd2024:rules_barbarian_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 488c5eac3479f57bceb68a0a90c827ff1a5ce3ee5cbde2f6e7a8141171afe217
  parent_name: Barbarian
  line_span:
    first: 2251
    count: 6
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Barbarian Traits table:
    Hit Point Die, proficiency with Martial weapons, and training with Shields. • Gain the Barbarian’s
    level 1 features, which are listed in the Barbarian Features table.'
  sha256: 79f0ba954bdc704ce2980cfe26fc787005aaa6ef1d7c5a3a1563fd852391dd47
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 28 -->

•  Gain the following traits from the Core Barbarian\
Traits table: Hit Point Die, proficiency with Mar-\
tial weapons, and training with Shields.\
•  Gain the Barbarian’s level 1 features, which are\
listed in the Barbarian Features table.\
