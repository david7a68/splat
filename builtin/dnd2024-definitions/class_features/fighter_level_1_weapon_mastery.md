---
splat_schema: 1
splat_id: 367ab48d-99fb-5a8b-9657-3c5fd6f1e72a
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_1_weapon_mastery
  parameters:
    mastery_choices:
      count:
        table:
          key: dnd2024:tables/fighter_progression
        row:
          var: actor.class_levels.fighter
        column: weapon_mastery
      eligible:
        weapon_category:
        - simple
        - martial
      replacement:
        event: rest.long_completed
        maximum: 1
    class_scope:
      class: fighter
      minimum_level: 1
  implementation: dnd2024:class_features_fighter_level_1_weapon_mastery
splat_contracts:
- id: dnd2024:class_features_fighter_level_1_weapon_mastery
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - character.level_gained
  - rest.long_completed
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_1_weapon_mastery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: dd25b7eca6aa144b134fe10e0852efa365c5d25078cf5684d81412e1ebf3d7d9
  parent_name: Fighter
  line_span:
    first: 3975
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 1: Weapon Mastery Your training with weapons allows you to use the mastery properties
    of three kinds of Simple or Martial weapons of your choice. Whenever you finish a Long Rest, you can
    practice weapon drills and change one of those weapon choices. When you reach certain Fighter levels,
    you gain the ability to use the mastery properties of more kinds of weapons, as shown in the Weapon
    Mastery column of the Fighter Features table.'
  sha256: 8f79cd3021374aa75688102133b7336c72d81ed16aed7f3d7a6b56d71aa07f04
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Weapon Mastery

<!-- SRD page 48 -->

Your training with weapons allows you to use the\
mastery properties of three kinds of Simple or Mar-\
tial weapons of your choice. Whenever you finish\
a Long Rest, you can practice weapon drills and\
change one of those weapon choices.\
When you reach certain Fighter levels, you gain\
the ability to use the mastery properties of more\
kinds of weapons, as shown in the Weapon Mastery\
column of the Fighter Features table.\
