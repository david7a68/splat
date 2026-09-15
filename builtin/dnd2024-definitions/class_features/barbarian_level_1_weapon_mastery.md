---
splat_schema: 1
splat_id: 70bc35fb-2e4a-55a0-80b8-6cafa38bcb7c
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_1_weapon_mastery
splat_name: 'Level 1: Weapon Mastery'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_1_weapon_mastery
  parameters:
    mastery_choices:
      count:
        table:
          key: dnd2024:tables/barbarian_progression
        row:
          var: actor.class_levels.barbarian
        column: weapon_mastery
      eligible:
        weapon_category:
        - simple
        - martial
        weapon_range: melee
      replacement:
        event: rest.long_completed
        maximum: 1
    class_scope:
      class: barbarian
      minimum_level: 1
  implementation: dnd2024:class_features_barbarian_level_1_weapon_mastery
splat_contracts:
- id: dnd2024:class_features_barbarian_level_1_weapon_mastery
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
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_1_weapon_mastery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: e00114878db681136b633d2da517326c8da704c3e31d01e7900c10c89738c946
  parent_name: Barbarian
  line_span:
    first: 2325
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 1: Weapon Mastery Your training with weapons allows you to use the mastery properties
    of two kinds of Simple or Martial Melee weapons of your choice, such as Greataxes and Handaxes. Whenever
    you finish a Long Rest, you can practice weapon drills and change one of those weapon choices. When
    you reach certain Barbarian levels, you gain the ability to use the mastery properties of more kinds
    of weapons, as shown in the Weapon Mastery column of the Barbarian Features table.'
  sha256: cebd55ad10030a9b5d5eefa92de97e85b172163762591557d732b60e71a231ae
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Weapon Mastery

<!-- SRD page 29 -->

Your training with weapons allows you to use the\
mastery properties of two kinds of Simple or Mar-\
tial Melee weapons of your choice, such as Great-\
axes and Handaxes. Whenever you finish a Long\
Rest, you can practice weapon drills and change one\
of those weapon choices.\
When you reach certain Barbarian levels, you gain\
the ability to use the mastery properties of more\
kinds of weapons, as shown in the Weapon Mastery\
column of the Barbarian Features table.\
