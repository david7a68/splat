---
splat_schema: 1
splat_id: 580f3471-1bd4-5fcc-b770-f3bf9d7b2dc9
splat_kind: item
splat_key: dnd2024:equipment/burglar_s_pack_16_gp
splat_name: Burglar’s Pack (16 GP)
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
    value: 16
    unit: gp
  mass:
    value: 42.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/burglar_s_pack_16_gp
  implementation: dnd2024:equipment_burglar_s_pack_16_gp
  parameters:
    contents:
    - item:
        key: dnd2024:equipment/backpack_2_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/ball_bearings_1_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/bell_1_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/candle_1_cp
      quantity: 10
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/crowbar_2_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/lantern_hooded_5_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/oil_1_sp
      quantity: 7
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/rations_5_sp
      quantity: 5
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
        key: dnd2024:equipment/waterskin_2_sp
      quantity: 1
      unit: purchase_unit
    grant_contents_on_acquisition: true
    mass:
      value: 42.0
      unit: lb
    grant_idempotency: once_per_acquired_pack_instance
splat_contracts:
- id: dnd2024:equipment_burglar_s_pack_16_gp
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
  - 96
  engine_gaps: &id001
  - dnd2024:equipment_burglar_s_pack_16_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f8b5f414ad7cb20b20639e73908b613c34c51905d050645bfc709e21c2992637
  line_span:
    first: 8317
    count: 5
splat_clauses:
- key: source_rules
  text: '# Burglar’s Pack (16 GP) A Burglar’s Pack contains the following items: Backpack, Ball Bearings,
    Bell, 10 Candles, Crowbar, Hooded Lantern, 7 flasks of Oil, 5 days of Rations, Rope, Tinderbox, and
    Waterskin.'
  sha256: 4ddc0e440edf55ba3519cc4b315a7994da249561763311a7edaa6c7cd3335f58
  status: contract_specified
splat_requires_mechanics: *id001
---

# Burglar’s Pack (16 GP)

<!-- SRD page 96 -->

A Burglar’s Pack contains the following items:\
Backpack, Ball Bearings, Bell, 10 Candles, Crowbar,\
Hooded Lantern, 7 flasks of Oil, 5 days of Rations,\
Rope, Tinderbox, and Waterskin.\
