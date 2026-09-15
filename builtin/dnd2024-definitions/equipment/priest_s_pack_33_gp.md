---
splat_schema: 1
splat_id: b42094da-a7f0-5ca4-9f72-a1d9357cd344
splat_kind: item
splat_key: dnd2024:equipment/priest_s_pack_33_gp
splat_name: Priest’s Pack (33 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 33
    unit: gp
  mass:
    value: 29.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/priest_s_pack_33_gp
  implementation: dnd2024:equipment_priest_s_pack_33_gp
  parameters:
    contents:
    - item:
        key: dnd2024:equipment/backpack_2_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/blanket_5_sp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/holy_water_25_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/lamp_5_sp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/rations_5_sp
      quantity: 7
      unit: day
    - item:
        key: dnd2024:equipment/robe_1_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/tinderbox_5_sp
      quantity: 1
      unit: purchase_unit
    grant_contents_on_acquisition: true
    mass:
      value: 29.0
      unit: lb
    grant_idempotency: once_per_acquired_pack_instance
splat_contracts:
- id: dnd2024:equipment_priest_s_pack_33_gp
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
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_priest_s_pack_33_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 886a90384a160776483b76882b71d8f3819852bc24741b3c882f65af9bf7ca89
  line_span:
    first: 8593
    count: 4
splat_clauses:
- key: source_rules
  text: '# Priest’s Pack (33 GP) A Priest’s Pack contains the following items: Backpack, Blanket, Holy
    Water, Lamp, 7 days of Rations, Robe, and Tinderbox.'
  sha256: d31fed6a6dc706cbb7728a7fd2981db39cd17369d34f8c5de8cb46cfc7b893a2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Priest’s Pack (33 GP)

<!-- SRD page 99 -->

A Priest’s Pack contains the following items: Back-\
pack, Blanket, Holy Water, Lamp, 7 days of Rations,\
Robe, and Tinderbox.\
