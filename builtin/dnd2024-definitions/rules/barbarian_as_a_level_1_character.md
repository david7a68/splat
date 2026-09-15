---
splat_schema: 1
splat_id: b96d0eea-daf0-56bb-a9e5-324f249d7afe
splat_kind: rule
splat_key: dnd2024:rules/barbarian_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/barbarian_as_a_level_1_character
  parameters:
    class:
      key: dnd2024:classes/barbarian
    entry_mode: level_one_character
    grant_all_core_traits: true
    grant_features_at_class_level: 1
    embedded_feature_progression_source:
      key: dnd2024:tables/barbarian_progression
    level_one_features:
    - key: dnd2024:class_features/barbarian_level_1_rage
    - key: dnd2024:class_features/barbarian_level_1_unarmored_defense
    - key: dnd2024:class_features/barbarian_level_1_weapon_mastery
  implementation: dnd2024:rule_barbarian_as_a_level_1_character
splat_contracts:
- id: dnd2024:rule_barbarian_as_a_level_1_character
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
  - classes
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 28
  engine_gaps: &id001
  - dnd2024:rule_barbarian_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 2e234a061dc3fa8613a51cea8f8db7c8a53e678e5b397d0335d1a4bb2920e7ad
  parent_name: Barbarian
  line_span:
    first: 2223
    count: 28
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Barbarian Traits table. Barbarian
    Features Proficiency Level Bonus Class Features 1 +2 Rage, Unarmored Defense, Weapon Mastery 2 +2
    Danger Sense, Reckless Attack 3 +2 Barbarian Subclass, Primal Knowledge 4 +2 Ability Score Improvement
    5 +3 Extra Attack, Fast Movement 6 +3 Subclass feature 7 +3 Feral Instinct, Instinctive Pounce 8 +3
    Ability Score Improvement 9 +4 Brutal Strike 10 +4 Subclass feature 11 +4 Relentless Rage 12 +4 Ability
    Score Improvement 13 +5 Improved Brutal Strike 14 +5 Subclass feature 15 +5 Persistent Rage 16 +5
    Ability Score Improvement 17 +6 Improved Brutal Strike 18 +6 Indomitable Might 19 +6 Epic Boon 20
    +6 Primal Champion • Gain the Barbarian’s level 1 features, which are listed in the Barbarian Features
    table.'
  sha256: 769e147ef4d31b0fe9e71bbd7acca97298f487723246ce73b887030edf4cf014
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 28 -->

•  Gain all the traits in the Core Barbarian Traits\
table.\
Barbarian Features\
Proficiency\
Level Bonus Class Features\
1 +2 Rage, Unarmored Defense, Weapon Mastery\
2 +2 Danger Sense, Reckless Attack\
3 +2 Barbarian Subclass, Primal Knowledge\
4 +2 Ability Score Improvement\
5 +3 Extra Attack, Fast Movement\
6 +3 Subclass feature\
7 +3 Feral Instinct, Instinctive Pounce\
8 +3 Ability Score Improvement\
9 +4 Brutal Strike\
10 +4 Subclass feature\
11 +4 Relentless Rage\
12 +4 Ability Score Improvement\
13 +5 Improved Brutal Strike\
14 +5 Subclass feature\
15 +5 Persistent Rage\
16 +5 Ability Score Improvement\
17 +6 Improved Brutal Strike\
18 +6 Indomitable Might\
19 +6 Epic Boon\
20 +6 Primal Champion\
•  Gain the Barbarian’s level 1 features, which are\
listed in the Barbarian Features table.\
