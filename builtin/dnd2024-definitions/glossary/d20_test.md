---
splat_schema: 1
splat_id: 6c9e1405-4d99-56f7-b83c-7fe4c9c964e1
splat_kind: rule
splat_key: dnd2024:glossary/d20_test
splat_name: D20 Test
splat_origin:
  key: dnd2024:reference/glossary/d20_test
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/d20_test
  parameters:
    test_kinds:
    - ability_check
    - attack_roll
    - saving_throw
    d20_test_modifiers_apply_to_all_kinds: true
    test_warranted:
      chooser: gm
      options:
      - true
      - false
  implementation: dnd2024:glossary_d20_test
splat_contracts:
- id: dnd2024:glossary_d20_test
  inputs:
    subject:
      type: participant
      required: true
    test_kind:
      type: choice
      required: true
    gm_requires_test:
      type: boolean
      required: true
  outputs:
  - test_requirement
  events: []
  capabilities:
  - checks
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 034ebe727af3373d0eb1787fbf3ad13e82c090869a37b3accf498590aa75eef3
  source_path: builtin/srd-5.2.1/glossary/d20_test.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_d20_test
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: dc8ecce063c4bc1d3d66a3f002e51f4545f83086085bffa3a623014d0892bc85
splat_clauses:
- key: source_rules
  text: '# D20 Test D20 Tests encompass the three main d20 rolls of the game: ability checks, attack rolls,
    and saving throws. If something in the game affects D20 Tests, it affects all three of these rolls.
    The GM determines whether a D20 Test is warranted in a given circumstance. See also “Playing the Game”
    (“D20 Tests”).'
  sha256: 57302b62521a29968d17ace50462ab1fd28ae6e4d432f5951ec8f8f071023f9e
  status: contract_specified
splat_requires_mechanics: *id001
---

# D20 Test

<!-- SRD page 180 -->

D20 Tests encompass the three main d20 rolls of\
the game: ability checks, attack rolls, and saving\
throws. If something in the game affects D20 Tests,\
it affects all three of these rolls. The GM determines\
whether a D20 Test is warranted in a given circum-\
stance. See also “Playing the Game” (“D20 Tests”).\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
