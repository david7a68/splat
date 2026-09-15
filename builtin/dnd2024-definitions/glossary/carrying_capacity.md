---
splat_schema: 1
splat_id: e041315e-4ff5-595e-aea6-d711a3482fcd
splat_kind: rule
splat_key: dnd2024:glossary/carrying_capacity
splat_name: Carrying Capacity
splat_origin:
  key: dnd2024:reference/glossary/carrying_capacity
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/carrying_capacity
  parameters:
    weight_unit: pound
    carry_strength_multipliers:
      tiny: 7.5
      small: 15
      medium: 15
      large: 30
      huge: 60
      gargantuan: 120
    drag_lift_push_strength_multipliers:
      tiny: 15
      small: 30
      medium: 30
      large: 60
      huge: 120
      gargantuan: 240
    over_carry_capacity:
      modes:
      - drag
      - lift
      - push
      speed_cap_feet: 5
  implementation: dnd2024:glossary_carrying_capacity
splat_contracts:
- id: dnd2024:glossary_carrying_capacity
  inputs:
    subject:
      type: participant
      required: true
    weight_pounds:
      type: number
      required: true
    mode:
      type: choice
      required: true
  outputs:
  - maximum_weight
  - speed_cap
  events: []
  capabilities:
  - inventory
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 39e89ad0baae270bdfb1c0362b4224ee8e27b848a0a1b494df24cbd7a8ef823b
  source_path: builtin/srd-5.2.1/glossary/carrying_capacity.md
  pages:
  - 178
  engine_gaps: &id001
  - dnd2024:glossary_carrying_capacity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: cd91e6ba368d84505bcfe7a3b711f3077c7b10c93713b331ed102d6061c1f204
splat_clauses:
- key: source_rules
  text: '# Carrying Capacity Your size and Strength score determine the maximum weight in pounds that
    you can carry, as shown in the Carrying Capacity table. The table also shows the maximum weight you
    can drag, lift, or push. While dragging, lifting, or pushing weight in excess of the maximum weight
    you can carry, your Speed can be no more than 5 feet. Carrying Capacity Creature Size Carry Drag/Lift/Push
    Tiny Str. × 7.5 lb. Str. × 15 lb. Small/Medium Str. × 15 lb. Str. × 30 lb. Large Str. × 30 lb. Str.
    × 60 lb. Huge Str. × 60 lb. Str. × 120 lb. Gargantuan Str. × 120 lb. Str. × 240 lb.'
  sha256: d5eeeab4a97409b2d0091979a570d9442b3176460b071cd6f865c3b94c3e4dcd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Carrying Capacity

<!-- SRD page 178 -->

Your size and Strength score determine the maxi-\
mum weight in pounds that you can carry, as shown\
in the Carrying Capacity table. The table also shows\
the maximum weight you can drag, lift, or push.\
While dragging, lifting, or pushing weight in ex-\
cess of the maximum weight you can carry, your\
Speed can be no more than 5 feet.\
Carrying Capacity\
Creature Size Carry Drag/Lift/Push\
Tiny Str. × 7.5 lb. Str. × 15 lb.\
Small/Medium Str. × 15 lb. Str. × 30 lb.\
Large Str. × 30 lb. Str. × 60 lb.\
Huge Str. × 60 lb. Str. × 120 lb.\
Gargantuan Str. × 120 lb. Str. × 240 lb.\

Original layout: [[../../srd-5.2.1/pages/178|page 178]].
