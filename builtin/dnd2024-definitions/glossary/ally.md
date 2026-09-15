---
splat_schema: 1
splat_id: 8b7bc468-6474-58f7-81fb-da94f040c7d7
splat_kind: rule
splat_key: dnd2024:glossary/ally
splat_name: Ally
splat_origin:
  key: dnd2024:reference/glossary/ally
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/ally
  parameters:
    predicate:
      any:
      - same_adventuring_party
      - friend
      - same_combat_side
      - rule_designated_ally
      - gm_designated_ally
    gm_designation:
      chooser: gm
      options:
      - true
      - false
  implementation: dnd2024:glossary_ally
splat_contracts:
- id: dnd2024:glossary_ally
  inputs:
    subject:
      type: participant
      required: true
    other:
      type: participant
      required: true
    relationship:
      type: choice
      required: true
  outputs:
  - is_ally
  events: []
  capabilities:
  - relationships
splat_conversion:
  status: specified
  category: glossary
  source_sha256: b2dcc11c11893aec0875bd762bc0e51e8b216449e69662a7b7917f6487ada3bf
  source_path: builtin/srd-5.2.1/glossary/ally.md
  pages:
  - 176
  engine_gaps: &id001
  - dnd2024:glossary_ally
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 4970fc92befcfe6b016f9f7ea498eeabb9e028362175c9db246e68d1fefc1b15
splat_clauses:
- key: source_rules
  text: '# Ally A creature is your ally if it is a member of your adventuring party, your friend, on your
    side in combat, or a creature that the rules or the GM designates as your ally.'
  sha256: 89e4b09983855c91cbca910ae6fe328e48d67fbb9ab5caf46896c94146861975
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ally

<!-- SRD page 176 -->

A creature is your ally if it is a member of your ad-\
venturing party, your friend, on your side in combat,\
or a creature that the rules or the GM designates as\
your ally.\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].
