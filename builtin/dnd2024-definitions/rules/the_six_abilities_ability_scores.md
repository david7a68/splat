---
splat_schema: 1
splat_id: e32900e0-05e5-50fd-a5e7-88b8373756fb
splat_kind: rule
splat_key: dnd2024:rules/the_six_abilities_ability_scores
splat_name: Ability Scores
splat_origin:
  key: dnd2024:reference/pages/005
splat_sources:
- title: SRD 5.2.1
  locator: pp. 5
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/the_six_abilities_ability_scores
  parameters:
    normal_min: 1
    adventurer_max: 20
    absolute_max: 30
    feature_can_override_adventurer_max: true
    score_zero_consequence: reducing_effect_definition
    score_bands:
    - min: 1
      max: 1
      category: normal_minimum
    - min: 2
      max: 9
      category: weak
    - min: 10
      max: 11
      category: human_average
    - min: 12
      max: 19
      category: strong
    - min: 20
      max: 20
      category: adventurer_maximum
    - min: 21
      max: 29
      category: extraordinary
    - min: 30
      max: 30
      category: absolute_maximum
  implementation: dnd2024:rules_the_six_abilities_ability_scores
splat_contracts:
- id: dnd2024:rules_the_six_abilities_ability_scores
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - ability_score_constraints
  events: []
  capabilities:
  - character_statistics
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 5
  engine_gaps: &id001
  - dnd2024:rules_the_six_abilities_ability_scores
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 08d53d42347ada980478edadb7a099e47f7fd6f7f1a5ba9618094c04d41e6090
  parent_name: The Six Abilities
  line_span:
    first: 66
    count: 17
splat_clauses:
- key: source_rules
  text: '# Ability Scores Each ability has a score from 1 to 20, although some monsters have a score as
    high as 30. The score represents the magnitude of an ability. The Ability Scores table summarizes
    what the scores mean. Ability Scores Score Meaning 1 This is the lowest a score can normally go. If
    an effect reduces a score to 0, that effect explains what happens. 2–9 This represents a weak capability.
    10–11 This represents the human average. 12–19 This represents a strong capability. 20 This is the
    highest an adventurer’s score can go unless a feature says otherwise. 21–29 This represents an extraordinary
    capability. 30 This is the highest a score can go.'
  sha256: cd520bc39e0e5a5573b40ee177bcffaac730b7262ed5cd5024d20d576540f2f5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Scores

<!-- SRD page 5 -->

Each ability has a score from 1 to 20, although some\
monsters have a score as high as 30. The score\
represents the magnitude of an ability. The Ability\
Scores table summarizes what the scores mean.\
Ability Scores\
Score Meaning\
1 This is the lowest a score can normally go.\
If an effect reduces a score to 0, that effect\
explains what happens.\
2–9 This represents a weak capability.\
10–11 This represents the human average.\
12–19 This represents a strong capability.\
20 This is the highest an adventurer’s score can\
go unless a feature says otherwise.\
21–29 This represents an extraordinary capability.\
30 This is the highest a score can go.\
