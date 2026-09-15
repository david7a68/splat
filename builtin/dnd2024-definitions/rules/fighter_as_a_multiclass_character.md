---
splat_schema: 1
splat_id: 54be02d7-b7b5-5e60-850c-892a8d7bca02
splat_kind: rule
splat_key: dnd2024:rules/fighter_as_a_multiclass_character
splat_name: As a Multiclass Character
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/fighter_as_a_multiclass_character
  parameters:
    class:
      key: dnd2024:classes/fighter
    entry: multiclass
    hit_die_sides: 10
    armor_training:
    - light
    - medium
    - shield
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/fighter_level_1_fighting_style
    - level: 1
      feature:
        key: dnd2024:class_features/fighter_level_1_second_wind
    - level: 1
      feature:
        key: dnd2024:class_features/fighter_level_1_weapon_mastery
    weapon_proficiencies:
    - martial
  implementation: dnd2024:rules_fighter_as_a_multiclass_character
splat_contracts:
- id: dnd2024:rules_fighter_as_a_multiclass_character
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
  - 47
  engine_gaps: &id001
  - dnd2024:rules_fighter_as_a_multiclass_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 36bdea4f310d34ef6119a09b43b0a770e73a637016a18c53485335e21f642d48
  parent_name: Fighter
  line_span:
    first: 3923
    count: 7
splat_clauses:
- key: source_rules
  text: '# As a Multiclass Character • Gain the following traits from the Core Fighter Traits table: Hit
    Point Die, proficiency with Martial weapons, and training with Light and Medium armor and Shields.
    • Gain the Fighter’s level 1 features, which are listed in the Fighter Features table.'
  sha256: ce0ad22a878fdd7a96e9292c5dcb09f904f08dbda806997b47f1eb7b2efff50a
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Multiclass Character

<!-- SRD page 47 -->

•  Gain the following traits from the Core Fighter\
Traits table: Hit Point Die, proficiency with Mar-\
tial weapons, and training with Light and Medium\
armor and Shields.\
•  Gain the Fighter’s level 1 features, which are\
listed in the Fighter Features table.\
