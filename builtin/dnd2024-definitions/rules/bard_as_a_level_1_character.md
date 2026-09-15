---
splat_schema: 1
splat_id: 4df2c733-0a44-549c-8f55-e4f5870390ad
splat_kind: rule
splat_key: dnd2024:rules/bard_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/bard_as_a_level_1_character
  parameters:
    requires_total_character_level: 1
    core_traits:
      key: dnd2024:classes/bard
    grant_all_core_traits: true
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/bard_level_1_bardic_inspiration
    - level: 1
      feature:
        key: dnd2024:class_features/bard_level_1_spellcasting
    progression:
    - level: 1
      proficiency_bonus: 2
      bardic_die_sides: 6
      features:
      - key: dnd2024:class_features/bard_level_1_bardic_inspiration
      - key: dnd2024:class_features/bard_level_1_spellcasting
    - level: 2
      proficiency_bonus: 2
      bardic_die_sides: 6
      features:
      - key: dnd2024:class_features/bard_level_2_expertise
      - key: dnd2024:class_features/bard_level_2_jack_of_all_trades
    - level: 3
      proficiency_bonus: 2
      bardic_die_sides: 6
      features:
      - key: dnd2024:class_features/bard_level_3_bard_subclass
    - level: 4
      proficiency_bonus: 2
      bardic_die_sides: 6
      features:
      - key: dnd2024:class_features/bard_level_4_ability_score_improvement
    - level: 5
      proficiency_bonus: 3
      bardic_die_sides: 8
      features:
      - key: dnd2024:class_features/bard_level_5_font_of_inspiration
    - level: 6
      proficiency_bonus: 3
      bardic_die_sides: 8
      features: []
    - level: 7
      proficiency_bonus: 3
      bardic_die_sides: 8
      features:
      - key: dnd2024:class_features/bard_level_7_countercharm
    - level: 8
      proficiency_bonus: 3
      bardic_die_sides: 8
      features: []
    - level: 9
      proficiency_bonus: 4
      bardic_die_sides: 8
      features: []
    - level: 10
      proficiency_bonus: 4
      bardic_die_sides: 10
      features:
      - key: dnd2024:class_features/bard_level_10_magical_secrets
    - level: 11
      proficiency_bonus: 4
      bardic_die_sides: 10
      features: []
    - level: 12
      proficiency_bonus: 4
      bardic_die_sides: 10
      features: []
    - level: 13
      proficiency_bonus: 5
      bardic_die_sides: 10
      features: []
    - level: 14
      proficiency_bonus: 5
      bardic_die_sides: 10
      features: []
    - level: 15
      proficiency_bonus: 5
      bardic_die_sides: 12
      features: []
    - level: 16
      proficiency_bonus: 5
      bardic_die_sides: 12
      features: []
    - level: 17
      proficiency_bonus: 6
      bardic_die_sides: 12
      features: []
    - level: 18
      proficiency_bonus: 6
      bardic_die_sides: 12
      features:
      - key: dnd2024:class_features/bard_level_18_superior_inspiration
    - level: 19
      proficiency_bonus: 6
      bardic_die_sides: 12
      features:
      - key: dnd2024:class_features/bard_level_19_epic_boon
    - level: 20
      proficiency_bonus: 6
      bardic_die_sides: 12
      features:
      - key: dnd2024:class_features/bard_level_20_words_of_creation
    subclass_feature_levels:
    - 3
    - 6
    - 14
  implementation: dnd2024:rules_bard_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_bard_as_a_level_1_character
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
  - 31
  engine_gaps: &id001
  - dnd2024:rules_bard_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d62e2c6db0d86798eddfaf7ba6d2487377c352ee124cd4cf3b6d59270e10bb35
  parent_name: Bard
  line_span:
    first: 2510
    count: 29
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Bard Traits table. Bard Features Proficiency
    Bardic Level Bonus Class Features Die 1 +2 Bardic Inspiration, D6 Spellcasting 2 +2 Expertise, D6
    Jack of All Trades 3 +2 Bard Subclass D6 4 +2 Ability Score Improvement D6 5 +3 Font of Inspiration
    D8 6 +3 Subclass feature D8 7 +3 Countercharm D8 8 +3 Ability Score Improvement D8 9 +4 Expertise
    D8 10 +4 Magical Secrets D10 11 +4 — D10 12 +4 Ability Score Improvement D10 13 +5 — D10 14 +5 Subclass
    feature D10 15 +5 — D12 16 +5 Ability Score Improvement D12 17 +6 — D12 18 +6 Superior Inspiration
    D12 19 +6 Epic Boon D12 20 +6 Words of Creation D12 • Gain the Bard’s level 1 features, which are
    listed in the Bard Features table.'
  sha256: fdd43e9df8a9295b2dce4348d168da2415e5bfd0b0b6ee703e074724a345fb3e
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 31 -->

•  Gain all the traits in the Core Bard Traits table.\
Bard Features\
Proficiency Bardic\
Level Bonus Class Features Die\
1 +2 Bardic Inspiration, D6\
Spellcasting\
2 +2 Expertise, D6\
Jack of All Trades\
3 +2 Bard Subclass D6\
4 +2 Ability Score Improvement D6\
5 +3 Font of Inspiration D8\
6 +3 Subclass feature D8\
7 +3 Countercharm D8\
8 +3 Ability Score Improvement D8\
9 +4 Expertise D8\
10 +4 Magical Secrets D10\
11 +4 — D10\
12 +4 Ability Score Improvement D10\
13 +5 — D10\
14 +5 Subclass feature D10\
15 +5 — D12\
16 +5 Ability Score Improvement D12\
17 +6 — D12\
18 +6 Superior Inspiration D12\
19 +6 Epic Boon D12\
20 +6 Words of Creation D12\
•  Gain the Bard’s level 1 features, which are listed\
in the Bard Features table.\
