---
splat_schema: 1
splat_id: 2f7ffa54-a1ba-59ec-90a7-79ff2cf07391
splat_kind: rule
splat_key: dnd2024:rules/barbarian_barbarian_class_features
splat_name: Barbarian Class Features
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/barbarian_barbarian_class_features
  parameters:
    class:
      key: dnd2024:classes/barbarian
    grant_when_class_level_reached:
    - level: 11
      feature:
        key: dnd2024:class_features/barbarian_level_11_relentless_rage
    - level: 13
      feature:
        key: dnd2024:class_features/barbarian_level_13_improved_brutal_strike
    - level: 15
      feature:
        key: dnd2024:class_features/barbarian_level_15_persistent_rage
    - level: 17
      feature:
        key: dnd2024:class_features/barbarian_level_17_improved_brutal_strike
    - level: 18
      feature:
        key: dnd2024:class_features/barbarian_level_18_indomitable_might
    - level: 19
      feature:
        key: dnd2024:class_features/barbarian_level_19_epic_boon
    - level: 1
      feature:
        key: dnd2024:class_features/barbarian_level_1_rage
    - level: 1
      feature:
        key: dnd2024:class_features/barbarian_level_1_unarmored_defense
    - level: 1
      feature:
        key: dnd2024:class_features/barbarian_level_1_weapon_mastery
    - level: 20
      feature:
        key: dnd2024:class_features/barbarian_level_20_primal_champion
    - level: 2
      feature:
        key: dnd2024:class_features/barbarian_level_2_danger_sense
    - level: 2
      feature:
        key: dnd2024:class_features/barbarian_level_2_reckless_attack
    - level: 3
      feature:
        key: dnd2024:class_features/barbarian_level_3_barbarian_subclass
    - level: 3
      feature:
        key: dnd2024:class_features/barbarian_level_3_primal_knowledge
    - level: 4
      feature:
        key: dnd2024:class_features/barbarian_level_4_ability_score_improvement
    - level: 5
      feature:
        key: dnd2024:class_features/barbarian_level_5_extra_attack
    - level: 5
      feature:
        key: dnd2024:class_features/barbarian_level_5_fast_movement
    - level: 7
      feature:
        key: dnd2024:class_features/barbarian_level_7_feral_instinct
    - level: 7
      feature:
        key: dnd2024:class_features/barbarian_level_7_instinctive_pounce
    - level: 9
      feature:
        key: dnd2024:class_features/barbarian_level_9_brutal_strike
  implementation: dnd2024:rules_barbarian_barbarian_class_features
splat_contracts:
- id: dnd2024:rules_barbarian_barbarian_class_features
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - class_features
  events: []
  capabilities:
  - character_features
  - level_advancement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 28
  engine_gaps: &id001
  - dnd2024:rules_barbarian_barbarian_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 4524b0525c03f2457c54731608619554abf9d760c6f4785c29b98fdb78af76e3
  parent_name: Barbarian
  line_span:
    first: 2257
    count: 5
splat_clauses:
- key: source_rules
  text: '# Barbarian Class Features As a Barbarian, you gain the following class features when you reach
    the specified Barbarian levels. These features are listed in the Barbarian Features table.'
  sha256: 19d1fe1a09eb8b377e9860096e4d9d4f80e7e61223869e022c931af9e129b583
  status: contract_specified
splat_requires_mechanics: *id001
---

# Barbarian Class Features

<!-- SRD page 28 -->

As a Barbarian, you gain the following class features\
when you reach the specified Barbarian levels.\
These features are listed in the Barbarian Features\
table.\
