---
splat_schema: 1
splat_id: f0c38a4c-3aae-574b-834e-fe869289d1e4
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_roll_two_d20s
splat_name: Roll Two D20s
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_roll_two_d20s
  parameters:
    roll:
      count: 2
      sides: 20
      add: 0
    keep:
      advantage: highest
      disadvantage: lowest
  implementation: dnd2024:rules_d20_tests_roll_two_d20s
splat_contracts:
- id: dnd2024:rules_d20_tests_roll_two_d20s
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 8
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_roll_two_d20s
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 286169337b7d98c72ae879d81524aa0c8395d89bf60bbf5cb3bba024204e45d5
  parent_name: D20 Tests
  line_span:
    first: 307
    count: 8
splat_clauses:
- key: source_rules
  text: '# Roll Two D20s When a roll has either Advantage or Disadvantage, roll a second d20 when you
    make the roll. Use the higher of the two rolls if you have Advantage, and use the lower roll if you
    have Disadvantage. For example, if you have Disadvantage and roll an 18 and a 3, use the 3. If you
    instead have Advantage and roll those numbers, use the 18.'
  sha256: 76edb7ff431abdb67b74e6fd1fb89ed90a61d82ba0bc5c8c5904c6e064bb293a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Roll Two D20s

<!-- SRD page 8 -->

When a roll has either Advantage or Disadvantage,\
roll a second d20 when you make the roll. Use the\
higher of the two rolls if you have Advantage, and\
use the lower roll if you have Disadvantage. For ex-\
ample, if you have Disadvantage and roll an 18 and a\
3, use the 3. If you instead have Advantage and roll\
those numbers, use the 18.\
