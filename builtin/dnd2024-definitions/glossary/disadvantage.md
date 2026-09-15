---
splat_schema: 1
splat_id: b8a9f899-c1a9-5da3-b770-90f84c5c8bf2
splat_kind: rule
splat_key: dnd2024:glossary/disadvantage
splat_name: Disadvantage
splat_origin:
  key: dnd2024:reference/glossary/disadvantage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/disadvantage
  parameters:
    test_kinds:
    - ability_check
    - attack_roll
    - saving_throw
    dice:
      count: 2
      sides: 20
      select: lowest
    same_kind_stacking: false
    opposite_kind_cancellation:
      any_advantage_and_any_disadvantage: true
      result:
        count: 1
        sides: 20
      ignore_source_counts: true
  implementation: dnd2024:glossary_disadvantage
splat_contracts:
- id: dnd2024:glossary_disadvantage
  inputs:
    subject:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - roll_mode
  events:
  - d20_test.pending
  capabilities:
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: cb9a3b014ca02bf7c4c5775cdeeee33b4b97df0b7f21261991950ae1997496a6
  source_path: builtin/srd-5.2.1/glossary/disadvantage.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_disadvantage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: aab0b14fd70daaaf87b639acca2e83d61d54656881e20a9535002114d7f04e26
splat_clauses:
- key: source_rules
  text: '# Disadvantage If you have Disadvantage on a D20 Test, roll two d20s and use the lower roll.
    A roll can’t be affected by more than one Disadvantage, and Advantage and Disadvantage on the same
    roll cancel each other. See also “Playing the Game” (“D20 Tests”).'
  sha256: ff8d5ccdcce70cf273b5b4465139e25e92d833b760093162b243b4aff79a0b13
  status: contract_specified
splat_requires_mechanics: *id001
---

# Disadvantage

<!-- SRD page 181 -->

If you have Disadvantage on a D20 Test, roll two\
d20s and use the lower roll. A roll can’t be affected\
by more than one Disadvantage, and Advantage and\
Disadvantage on the same roll cancel each other. See\
also “Playing the Game” (“D20 Tests”).\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
