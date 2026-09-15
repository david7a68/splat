---
splat_schema: 1
splat_id: b95c4da6-7550-576c-a3f2-5b00548dd0cf
splat_kind: rule
splat_key: dnd2024:rules/create_your_character_generate_your_scores
splat_name: Generate Your Scores
splat_origin:
  key: dnd2024:reference/pages/021
splat_sources:
- title: SRD 5.2.1
  locator: pp. 21
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/create_your_character_generate_your_scores
  parameters:
    method_choice:
      authority: gm_or_player
      options:
      - standard_array
      - random_generation
      - point_cost
    standard_array:
    - 15
    - 14
    - 13
    - 12
    - 10
    - 8
    random_generation:
      repeat: 6
      roll:
        count: 4
        sides: 6
        add: 0
      keep_highest: 3
      aggregate: sum
    point_cost:
      budget: 27
      score_costs:
        '8': 0
        '9': 1
        '10': 2
        '11': 3
        '12': 4
        '13': 5
        '14': 7
        '15': 9
      score_count: 6
  implementation: dnd2024:rules_create_your_character_generate_your_scores
splat_contracts:
- id: dnd2024:rules_create_your_character_generate_your_scores
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
  - 21
  engine_gaps: &id001
  - dnd2024:rules_create_your_character_generate_your_scores
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 9408da3ce5818aaadedd0b50447410d1cdbb98b4a27f5d4cf90e0f725cb05c94
  parent_name: Create Your Character
  line_span:
    first: 1513
    count: 19
splat_clauses:
- key: source_rules
  text: '# Generate Your Scores Determine your ability scores by using one of the following three methods.
    Your GM might prefer you to use a particular one. Standard Array. Use the following six scores for
    your abilities: 15, 14, 13, 12, 10, 8. Random Generation. Roll four d6s and record the total of the
    highest three dice. Do this five more times, so you have six numbers. Point Cost. You have 27 points
    to spend on your ability scores. The cost of each score is shown on the Ability Score Point Costs
    table. For example, a score of 14 costs 7 of your 27 points. Ability Score Point Costs Score Cost
    Score Cost 8 0 12 4 9 1 13 5 10 2 14 7 11 3 15 9'
  sha256: c2e496ce0ec337a373d916afd48c3340631acbcd1b2aa44661af09aca4bfc82a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Generate Your Scores

<!-- SRD page 21 -->

Determine your ability scores by using one of the\
following three methods. Your GM might prefer you\
to use a particular one.\
Standard Array. Use the following six scores for\
your abilities: 15, 14, 13, 12, 10, 8.\
Random Generation. Roll four d6s and record\
the total of the highest three dice. Do this five more\
times, so you have six numbers.\
Point Cost. You have 27 points to spend on your\
ability scores. The cost of each score is shown on\
the Ability Score Point Costs table. For example, a\
score of 14 costs 7 of your 27 points.\
Ability Score Point Costs\
Score Cost Score Cost\
8 0 12 4\
9 1 13 5\
10 2 14 7\
11 3 15 9\
