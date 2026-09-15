---
splat_schema: 1
splat_id: 73579bfa-70aa-5fd8-9085-b1cc03856533
splat_kind: rule
splat_key: dnd2024:glossary/damage
splat_name: Damage
splat_origin:
  key: dnd2024:reference/glossary/damage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/damage
  parameters:
    targets:
    - creature
    - object
    hit_points_change:
      op: negate
      args:
      - var: damage_amount
  implementation: dnd2024:glossary_damage
splat_contracts:
- id: dnd2024:glossary_damage
  inputs:
    subject:
      type: object
      required: true
    damage_amount:
      type: integer
      required: true
  outputs:
  - hit_points_delta
  events:
  - damage.applied
  capabilities:
  - damage
  - hit_points
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 99cbdd3ca92a28bda8f51b1835ed543d82989cf62b8900e5e132be00c2859516
  source_path: builtin/srd-5.2.1/glossary/damage.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_damage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 74a087413abfed0fcf45b5cef2ded121b14cea686743343b95d2ed66bdf7da54
splat_clauses:
- key: source_rules
  text: '# Damage Damage represents harm that causes a creature or an object to lose Hit Points.'
  sha256: 33321cd3bf08b238d76557d5e295d9c09adafb0d7bc6528798073d3478ddea2c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage

<!-- SRD page 180 -->

Damage represents harm that causes a creature or\
an object to lose Hit Points.\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
