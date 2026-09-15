---
splat_schema: 1
splat_id: 6f92e855-bb92-5ee3-afd3-376606d4fd63
splat_kind: item
splat_key: dnd2024:equipment/backpack_2_gp
splat_name: Backpack (2 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 2
    unit: gp
  mass:
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/backpack_2_gp
  implementation: dnd2024:equipment_backpack_2_gp
  parameters:
    capacity:
      mass_lb: 30
      volume_ft3: 1
    also_usable_as: saddlebag
    mass:
      value: 5.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_backpack_2_gp
  inputs:
    item:
      type: item_instance
      required: true
    contents:
      required: false
      type: item_list
  outputs:
  - capacity_constraints
  events: []
  capabilities:
  - containers
  - inventory
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_backpack_2_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 669c60785e6253abc8818af1fb2c9d487cd8024ae7a0716680c294fb0317636c
  line_span:
    first: 8277
    count: 3
splat_clauses:
- key: source_rules
  text: '# Backpack (2 GP) A Backpack holds up to 30 pounds within 1 cubic foot. It can also serve as
    a saddlebag.'
  sha256: 30b0f12fa95c55840c9431d0b70871dc1605d56f097222c018e4097639ee278d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Backpack (2 GP)

<!-- SRD page 96 -->

A Backpack holds up to 30 pounds within 1 cubic\
foot. It can also serve as a saddlebag.\
