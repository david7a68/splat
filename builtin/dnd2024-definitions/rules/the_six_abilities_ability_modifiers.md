---
splat_schema: 1
splat_id: 863d1efd-e5c9-58c1-9c18-0181f3a7ac24
splat_kind: rule
splat_key: dnd2024:rules/the_six_abilities_ability_modifiers
splat_name: Ability Modifiers
splat_origin:
  key: dnd2024:reference/pages/005
splat_sources:
- title: SRD 5.2.1
  locator: pp. 5, 6
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/the_six_abilities_ability_modifiers
  parameters:
    ability_score_range:
    - 1
    - 30
    ability_modifier:
      op: floor
      args:
      - op: divide
        args:
        - op: subtract
          args:
          - variable: ability_score
          - 10
        - 2
    apply_to: d20_test_using_ability
    fractional_multiplication_and_division_rounding: floor
    explicit_round_up_overrides: true
  implementation: dnd2024:rule_the_six_abilities_ability_modifiers
splat_contracts:
- id: dnd2024:rule_the_six_abilities_ability_modifiers
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    ability_score:
      type: number
      required: true
  outputs:
  - statistics
  events: []
  capabilities:
  - statistics
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 5
  - 6
  engine_gaps: &id001
  - dnd2024:rule_the_six_abilities_ability_modifiers
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 7bef0d3661c235b9c2be33c6cf9eef30b4bab5c46f6ab00c51f0631e44e9601f
  parent_name: The Six Abilities
  line_span:
    first: 83
    count: 20
splat_clauses:
- key: source_rules
  text: '# Ability Modifiers Each ability has a modifier that you apply whenever you make a D20 Test with
    that ability (explained in “D20 Tests”). An ability modifier is derived from its score, as shown in
    the Ability Modifiers table. round down Whenever you divide or multiply a number in the game, round
    down if you end up with a fraction, even if the fraction is one-half or greater. Some rules make an
    exception and tell you to round up. Ability Modifiers Score Modifier Score Modifier 1 -5 16–17 +3
    2–3 -4 18–19 +4 4–5 -3 20–21 +5 6–7 -2 22–23 +6 8–9 -1 24–25 +7 10–11 +0 26–27 +8 12–13 +1 28–29 +9
    14–15 +2 30 +10'
  sha256: be303ea6a9d35f5f95b0dd9d90693c2b2eaea9c9273777bb8e469257c04c944b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Modifiers

<!-- SRD page 5 -->

Each ability has a modifier that you apply whenever\
you make a D20 Test with that ability (explained in\
“D20 Tests”). An ability modifier is derived from its\
score, as shown in the Ability Modifiers table.\
round down\
Whenever you divide or multiply a number in the\
game, round down if you end up with a fraction, even\
if the fraction is one-half or greater. Some rules make\
an exception and tell you to round up.\

<!-- SRD page 6 -->

Ability Modifiers\
Score Modifier Score Modifier\
1 −5 16–17 +3\
2–3 −4 18–19 +4\
4–5 −3 20–21 +5\
6–7 −2 22–23 +6\
8–9 −1 24–25 +7\
10–11 +0 26–27 +8\
12–13 +1 28–29 +9\
14–15 +2 30 +10\
