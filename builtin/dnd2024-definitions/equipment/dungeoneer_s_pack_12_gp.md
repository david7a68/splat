---
splat_schema: 1
splat_id: 295e1c4b-fe8f-5ea6-8b40-00531a7419fd
splat_kind: item
splat_key: dnd2024:equipment/dungeoneer_s_pack_12_gp
splat_name: Dungeoneer’s Pack (12 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 12
    unit: gp
  mass:
    value: 55.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/dungeoneer_s_pack_12_gp
  implementation: dnd2024:equipment_dungeoneer_s_pack_12_gp
  parameters:
    contents:
    - item:
        key: dnd2024:equipment/backpack_2_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/caltrops_1_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/crowbar_2_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/oil_1_sp
      quantity: 2
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/rations_5_sp
      quantity: 10
      unit: day
    - item:
        key: dnd2024:equipment/rope_1_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/tinderbox_5_sp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/torch_1_cp
      quantity: 10
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/waterskin_2_sp
      quantity: 1
      unit: purchase_unit
    grant_contents_on_acquisition: true
    mass:
      value: 55.0
      unit: lb
    grant_idempotency: once_per_acquired_pack_instance
splat_contracts:
- id: dnd2024:equipment_dungeoneer_s_pack_12_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - inventory_grants
  events:
  - item.acquired
  capabilities:
  - inventory
  - item_bundles
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_dungeoneer_s_pack_12_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: d41b7dc86201d983374565d1373e4204d363ba77205eb597ec48391e7de9f6f8
  line_span:
    first: 8393
    count: 5
splat_clauses:
- key: source_rules
  text: '# Dungeoneer’s Pack (12 GP) A Dungeoneer’s Pack contains the following items: Backpack, Caltrops,
    Crowbar, 2 flasks of Oil, 10 days of Rations, Rope, Tinderbox, 10 Torches, and Waterskin.'
  sha256: 0539d372c031a4d3bfbdcb9ada8ea81774e8bef121ebcaee9d2f331b921a2791
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dungeoneer’s Pack (12 GP)

<!-- SRD page 97 -->

A Dungeoneer’s Pack contains the following items:\
Backpack, Caltrops, Crowbar, 2 flasks of Oil, 10\
days of Rations, Rope, Tinderbox, 10 Torches, and\
Waterskin.\
