---
splat_schema: 1
splat_id: d22e5548-f5fd-5cc5-b1b6-7a3037ad2e02
splat_kind: rule
splat_key: dnd2024:glossary/resistance
splat_name: Resistance
splat_origin:
  key: dnd2024:reference/glossary/resistance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/resistance
  parameters:
    matching_damage_multiplier: 0.5
    rounding: down
    applications_per_damage_instance_max: 1
  implementation: dnd2024:glossary_resistance
splat_contracts:
- id: dnd2024:glossary_resistance
  inputs:
    subject:
      type: object
      required: true
    damage_type:
      type: choice
      required: true
    damage_amount:
      type: integer
      required: true
  outputs:
  - damage_amount
  events:
  - damage.pending
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 2f56d2b10037f473aade68b04c499771c69408a1084c6e7d06d52c9291655e72
  source_path: builtin/srd-5.2.1/glossary/resistance.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_resistance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 8033f340021ef8acefeea4e5750dec1ce52f154f528c2e4b791e69c8bb770927
splat_clauses:
- key: source_rules
  text: '# Resistance If you have Resistance to a damage type, damage of that type is halved against you
    (round down). Resistance is applied only once to an instance of damage. See also “Playing the Game”
    (“Damage and Healing”).'
  sha256: b1fc87599c7696693122a1898c6c37c347eb10ca0fe45d488f572295e3720604
  status: contract_specified
splat_requires_mechanics: *id001
---

# Resistance

<!-- SRD page 187 -->

If you have Resistance to a damage type, damage\
of that type is halved against you (round down).\
Resistance is applied only once to an instance of\
damage. See also “Playing the Game” (“Damage and\
Healing”).\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
