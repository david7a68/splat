---
splat_schema: 1
splat_id: 3dae3c2f-76fc-57ce-a4b1-3b629c7e6a0b
splat_kind: rule
splat_key: dnd2024:rules/ranger_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/ranger_as_a_level_1_character
  parameters:
    requires_total_character_level: 1
    core_traits:
      key: dnd2024:classes/ranger
    grant_all_core_traits: true
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/ranger_level_1_favored_enemy
    - level: 1
      feature:
        key: dnd2024:class_features/ranger_level_1_spellcasting
    - level: 1
      feature:
        key: dnd2024:class_features/ranger_level_1_weapon_mastery
  implementation: dnd2024:rules_ranger_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_ranger_as_a_level_1_character
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - core_traits
  - class_features
  events: []
  capabilities:
  - character_creation
  - character_features
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 57
  engine_gaps: &id001
  - dnd2024:rules_ranger_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 9b7f6c27d99cbbe9d27bb910ca685ca732047c8e0f47fd81a209eda1535135e2
  parent_name: Ranger
  line_span:
    first: 4811
    count: 4
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Ranger Traits table. • Gain the Ranger’s
    level 1 features, which are listed in the Ranger Features table.'
  sha256: 8439580ea0974ae845eeea977a140e83d309322611c32aa30572a6ee86f2f7bb
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 57 -->

•  Gain all the traits in the Core Ranger Traits table.\
•  Gain the Ranger’s level 1 features, which are\
listed in the Ranger Features table.\
