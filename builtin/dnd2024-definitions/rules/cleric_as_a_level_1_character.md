---
splat_schema: 1
splat_id: f2ba5c21-ec3d-517e-9038-6e49be5d389f
splat_kind: rule
splat_key: dnd2024:rules/cleric_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/cleric_as_a_level_1_character
  parameters:
    requires_total_character_level: 1
    core_traits:
      key: dnd2024:classes/cleric
    grant_all_core_traits: true
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/cleric_level_1_divine_order
    - level: 1
      feature:
        key: dnd2024:class_features/cleric_level_1_spellcasting
    progression:
    - level: 1
      proficiency_bonus: 2
      channel_divinity_uses: 0
      features:
      - key: dnd2024:class_features/cleric_level_1_divine_order
      - key: dnd2024:class_features/cleric_level_1_spellcasting
    - level: 2
      proficiency_bonus: 2
      channel_divinity_uses: 2
      features:
      - key: dnd2024:class_features/cleric_level_2_channel_divinity
    - level: 3
      proficiency_bonus: 2
      channel_divinity_uses: 2
      features:
      - key: dnd2024:class_features/cleric_level_3_cleric_subclass
    - level: 4
      proficiency_bonus: 2
      channel_divinity_uses: 2
      features:
      - key: dnd2024:class_features/cleric_level_4_ability_score_improvement
    - level: 5
      proficiency_bonus: 3
      channel_divinity_uses: 2
      features:
      - key: dnd2024:class_features/cleric_level_5_sear_undead
    - level: 6
      proficiency_bonus: 3
      channel_divinity_uses: 3
      features: []
    - level: 7
      proficiency_bonus: 3
      channel_divinity_uses: 3
      features:
      - key: dnd2024:class_features/cleric_level_7_blessed_strikes
    - level: 8
      proficiency_bonus: 3
      channel_divinity_uses: 3
      features: []
    - level: 9
      proficiency_bonus: 4
      channel_divinity_uses: 3
      features: []
    - level: 10
      proficiency_bonus: 4
      channel_divinity_uses: 3
      features:
      - key: dnd2024:class_features/cleric_level_10_divine_intervention
    - level: 11
      proficiency_bonus: 4
      channel_divinity_uses: 3
      features: []
    - level: 12
      proficiency_bonus: 4
      channel_divinity_uses: 3
      features: []
    - level: 13
      proficiency_bonus: 5
      channel_divinity_uses: 3
      features: []
    - level: 14
      proficiency_bonus: 5
      channel_divinity_uses: 3
      features:
      - key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
    - level: 15
      proficiency_bonus: 5
      channel_divinity_uses: 3
      features: []
    - level: 16
      proficiency_bonus: 5
      channel_divinity_uses: 3
      features: []
    - level: 17
      proficiency_bonus: 6
      channel_divinity_uses: 3
      features: []
    - level: 18
      proficiency_bonus: 6
      channel_divinity_uses: 4
      features: []
    - level: 19
      proficiency_bonus: 6
      channel_divinity_uses: 4
      features:
      - key: dnd2024:class_features/cleric_level_19_epic_boon
    - level: 20
      proficiency_bonus: 6
      channel_divinity_uses: 4
      features:
      - key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
    subclass_feature_levels:
    - 3
    - 6
    - 17
  implementation: dnd2024:rules_cleric_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_cleric_as_a_level_1_character
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
  - 36
  engine_gaps: &id001
  - dnd2024:rules_cleric_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 9482f8cddc9c8939a780ce3ff19000e74eddd5d2ad832cd55ca8964b032266a8
  parent_name: Cleric
  line_span:
    first: 2930
    count: 27
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Cleric Traits table. • Gain the Cleric’s
    level 1 features, which are listed in the Cleric Features table. Cleric Features Proficiency Channel
    Level Bonus Class Features Divinity 1 +2 Spellcasting, Divine Order — 2 +2 Channel Divinity 2 3 +2
    Cleric Subclass 2 4 +2 Ability Score Improvement 2 5 +3 Sear Undead 2 6 +3 Subclass feature 3 7 +3
    Blessed Strikes 3 8 +3 Ability Score Improvement 3 9 +4 — 3 10 +4 Divine Intervention 3 11 +4 — 3
    12 +4 Ability Score Improvement 3 13 +5 — 3 14 +5 Improved Blessed Strikes 3 15 +5 — 3 16 +5 Ability
    Score Improvement 3 17 +6 Subclass feature 3 18 +6 — 4 19 +6 Epic Boon 4 20 +6 Greater Divine Intervention
    4'
  sha256: cabe37f216ad4cc34189b16e4fecad5e6893043c40e9450235d412f52727aee9
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 36 -->

•  Gain all the traits in the Core Cleric Traits table.\
•  Gain the Cleric’s level 1 features, which are listed\
in the Cleric Features table.\
Cleric Features\
Proficiency Channel\
Level Bonus Class Features Divinity\
1 +2 Spellcasting, Divine Order —\
2 +2 Channel Divinity 2\
3 +2 Cleric Subclass 2\
4 +2 Ability Score Improvement 2\
5 +3 Sear Undead 2\
6 +3 Subclass feature 3\
7 +3 Blessed Strikes 3\
8 +3 Ability Score Improvement 3\
9 +4 — 3\
10 +4 Divine Intervention 3\
11 +4 — 3\
12 +4 Ability Score Improvement 3\
13 +5 — 3\
14 +5 Improved Blessed Strikes 3\
15 +5 — 3\
16 +5 Ability Score Improvement 3\
17 +6 Subclass feature 3\
18 +6 — 4\
19 +6 Epic Boon 4\
20 +6 Greater Divine Intervention 4\
