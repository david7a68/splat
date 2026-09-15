---
splat_schema: 1
splat_id: 995c14de-d1b0-535c-90c3-b03475366d07
splat_kind: rule
splat_key: dnd2024:glossary/advantage
splat_name: Advantage
splat_origin:
  key: dnd2024:reference/glossary/advantage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/advantage
  parameters:
    test_kinds:
    - ability_check
    - attack_roll
    - saving_throw
    dice:
      count: 2
      sides: 20
      select: highest
    same_kind_stacking: false
    opposite_kind_cancellation:
      any_advantage_and_any_disadvantage: true
      result:
        count: 1
        sides: 20
      ignore_source_counts: true
  implementation: dnd2024:glossary_advantage
splat_contracts:
- id: dnd2024:glossary_advantage
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
  source_sha256: d1efe54eb9001955d48760d84dd16e45793ff114d13b6b5c26a6e92db205e86d
  source_path: builtin/srd-5.2.1/glossary/advantage.md
  pages:
  - 176
  engine_gaps: &id001
  - dnd2024:glossary_advantage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: bb0c8e77c131fa9c856f22e7d8864bd350b31efc4db9544d02c707a5acc6b558
splat_clauses:
- key: source_rules
  text: '# Advantage If you have Advantage on a D20 Test, roll two d20s, and use the higher roll. A roll
    can’t be affected by more than one Advantage, and Advantage and Disadvantage on the same roll cancel
    each other. See also “Playing the Game” (“D20 Tests”).'
  sha256: d79e28ca840612a13a9549a51ca6ae8359f6a670279c0fc48f14df5e8d4628f9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Advantage

<!-- SRD page 176 -->

If you have Advantage on a D20 Test, roll two d20s,\
and use the higher roll. A roll can’t be affected by\
more than one Advantage, and Advantage and Dis-\
advantage on the same roll cancel each other. See\
also “Playing the Game” (“D20 Tests”).\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].
