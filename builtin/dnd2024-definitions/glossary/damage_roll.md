---
splat_schema: 1
splat_id: c23ce46c-b3d2-57a7-868b-3c9adfecf599
splat_kind: rule
splat_key: dnd2024:glossary/damage_roll
splat_name: Damage Roll
splat_origin:
  key: dnd2024:reference/glossary/damage_roll
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/damage_roll
  parameters:
    amount:
      op: add
      args:
      - var: rolled_total
      - var: applicable_modifiers
    apply_to: target
  implementation: dnd2024:glossary_damage_roll
splat_contracts:
- id: dnd2024:glossary_damage_roll
  inputs:
    target:
      type: object
      required: true
    rolled_total:
      type: integer
      required: true
    applicable_modifiers:
      type: integer
      required: true
  outputs:
  - damage_amount
  events:
  - damage.rolled
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 64574babd0d607a0a4a017d2d7ec49f637e1866220a691802295432abd80737e
  source_path: builtin/srd-5.2.1/glossary/damage_roll.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_damage_roll
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 86cec1c5961a52ae629aba894f5b3208a5fb95864a1843fa76af87eb04f8bbd0
splat_clauses:
- key: source_rules
  text: '# Damage Roll A damage roll is a die roll, adjusted by any applicable modifiers, that deals damage
    to a target. See also “Playing the Game” (“Damage and Healing”).'
  sha256: 5a44d30e55b60c28b6ba9fcb9967eee5065c62d8460e8f0509ff796caa939376
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage Roll

<!-- SRD page 180 -->

A damage roll is a die roll, adjusted by any applica-\
ble modifiers, that deals damage to a target. See also\
“Playing the Game” (“Damage and Healing”).\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
