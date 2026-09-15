---
splat_schema: 1
splat_id: 71d76934-1fc8-50be-a15c-cca568917292
splat_kind: rule
splat_key: dnd2024:rules/monk_monk_class_features
splat_name: Monk Class Features
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49, 50
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/monk_monk_class_features
  parameters:
    class_definition:
      key: dnd2024:classes/monk
    grant_features:
      level_basis: class_level
      unlock_when: class_level_gte_feature_level
      from: class_progression
    feature_unlocks:
    - class_level: 10
      feature:
        key: dnd2024:class_features/monk_level_10_heightened_focus
    - class_level: 10
      feature:
        key: dnd2024:class_features/monk_level_10_self_restoration
    - class_level: 13
      feature:
        key: dnd2024:class_features/monk_level_13_deflect_energy
    - class_level: 14
      feature:
        key: dnd2024:class_features/monk_level_14_disciplined_survivor
    - class_level: 15
      feature:
        key: dnd2024:class_features/monk_level_15_perfect_focus
    - class_level: 18
      feature:
        key: dnd2024:class_features/monk_level_18_superior_defense
    - class_level: 19
      feature:
        key: dnd2024:class_features/monk_level_19_epic_boon
    - class_level: 1
      feature:
        key: dnd2024:class_features/monk_level_1_martial_arts
    - class_level: 1
      feature:
        key: dnd2024:class_features/monk_level_1_unarmored_defense
    - class_level: 20
      feature:
        key: dnd2024:class_features/monk_level_20_body_and_mind
    - class_level: 2
      feature:
        key: dnd2024:class_features/monk_level_2_monk_s_focus
    - class_level: 2
      feature:
        key: dnd2024:class_features/monk_level_2_unarmored_movement
    - class_level: 2
      feature:
        key: dnd2024:class_features/monk_level_2_uncanny_metabolism
    - class_level: 3
      feature:
        key: dnd2024:class_features/monk_level_3_deflect_attacks
    - class_level: 3
      feature:
        key: dnd2024:class_features/monk_level_3_monk_subclass
    - class_level: 4
      feature:
        key: dnd2024:class_features/monk_level_4_ability_score_improvement
    - class_level: 4
      feature:
        key: dnd2024:class_features/monk_level_4_slow_fall
    - class_level: 5
      feature:
        key: dnd2024:class_features/monk_level_5_extra_attack
    - class_level: 5
      feature:
        key: dnd2024:class_features/monk_level_5_stunning_strike
    - class_level: 6
      feature:
        key: dnd2024:class_features/monk_level_6_empowered_strikes
    - class_level: 7
      feature:
        key: dnd2024:class_features/monk_level_7_evasion
    - class_level: 9
      feature:
        key: dnd2024:class_features/monk_level_9_acrobatic_movement
    recurring_feature_levels:
      ability_score_improvement:
      - 4
      - 8
      - 12
      - 16
      subclass:
      - 3
      - 6
      - 11
      - 17
  implementation: dnd2024:rules_monk_monk_class_features
splat_contracts:
- id: dnd2024:rules_monk_monk_class_features
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
  - 49
  - 50
  engine_gaps: &id001
  - dnd2024:rules_monk_monk_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 99f3f946bffb744d40c24491b48a6849df2d5b38eeceb8928d336bfa9b8517b4
  parent_name: Monk
  line_span:
    first: 4120
    count: 28
splat_clauses:
- key: source_rules
  text: '# Monk Class Features As a Monk, you gain the following class features when you reach the specified
    Monk levels. These features are listed in the Monk Features table. Monk Features Proficiency Level
    Bonus Class Features 1 +2 Martial Arts, Unarmored Defense 2 +2 Monk’s Focus, Unarmored Movement, Uncanny
    Metabolism 3 +2 Deflect Attacks, Monk Subclass 4 +2 Ability Score Improvement, Slow Fall 5 +3 Extra
    Attack, Stunning Strike 6 +3 Empowered Strikes, Subclass feature 7 +3 Evasion 8 +3 Ability Score Improvement
    9 +4 Acrobatic Movement 10 +4 Heightened Focus, Self-Restoration 11 +4 Subclass feature 12 +4 Ability
    Score Improvement 13 +5 Deflect Energy 14 +5 Disciplined Survivor 15 +5 Perfect Focus 16 +5 Ability
    Score Improvement 17 +6 Subclass feature 18 +6 Superior Defense 19 +6 Epic Boon 20 +6 Body and Mind'
  sha256: cddd0479a744a7127c0d7390a6c6511ed8e82f15d3babacee9e2edd8e550b0be
  status: contract_specified
splat_requires_mechanics: *id001
---

# Monk Class Features

<!-- SRD page 49 -->

As a Monk, you gain the following class features\
when you reach the specified Monk levels. These\
features are listed in the Monk Features table.\

<!-- SRD page 50 -->

Monk Features\
Proficiency\
Level Bonus Class Features\
1 +2 Martial Arts, Unarmored Defense\
2 +2 Monk’s Focus, Unarmored Movement,\
Uncanny Metabolism\
3 +2 Deflect Attacks, Monk Subclass\
4 +2 Ability Score Improvement, Slow Fall\
5 +3 Extra Attack, Stunning Strike\
6 +3 Empowered Strikes, Subclass feature\
7 +3 Evasion\
8 +3 Ability Score Improvement\
9 +4 Acrobatic Movement\
10 +4 Heightened Focus, Self-Restoration\
11 +4 Subclass feature\
12 +4 Ability Score Improvement\
13 +5 Deflect Energy\
14 +5 Disciplined Survivor\
15 +5 Perfect Focus\
16 +5 Ability Score Improvement\
17 +6 Subclass feature\
18 +6 Superior Defense\
19 +6 Epic Boon\
20 +6 Body and Mind\
