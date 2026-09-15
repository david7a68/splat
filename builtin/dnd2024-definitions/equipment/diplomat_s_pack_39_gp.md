---
splat_schema: 1
splat_id: 43d65482-fac6-5e5f-8b01-9080c296908a
splat_kind: item
splat_key: dnd2024:equipment/diplomat_s_pack_39_gp
splat_name: Diplomat’s Pack (39 GP)
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
    value: 39
    unit: gp
  mass:
    value: 39.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/diplomat_s_pack_39_gp
  implementation: dnd2024:equipment_diplomat_s_pack_39_gp
  parameters:
    contents:
    - item:
        key: dnd2024:equipment/chest_5_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/clothes_fine_15_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/ink_10_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/ink_pen_2_cp
      quantity: 5
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/lamp_5_sp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/case_map_or_scroll_1_gp
      quantity: 2
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/oil_1_sp
      quantity: 4
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/paper_2_sp
      quantity: 5
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/parchment_1_sp
      quantity: 5
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/perfume_5_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/tinderbox_5_sp
      quantity: 1
      unit: purchase_unit
    grant_contents_on_acquisition: true
    mass:
      value: 39.0
      unit: lb
    grant_idempotency: once_per_acquired_pack_instance
splat_contracts:
- id: dnd2024:equipment_diplomat_s_pack_39_gp
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
  - dnd2024:equipment_diplomat_s_pack_39_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: a04b7e5fa7e5e6ea9f1734bd46f31738b749ac0964b7b31cd768cd0eec13eea1
  line_span:
    first: 8378
    count: 5
splat_clauses:
- key: source_rules
  text: '# Diplomat’s Pack (39 GP) A Diplomat’s Pack contains the following items: Chest, Fine Clothes,
    Ink, 5 Ink Pens, Lamp, 2 Map or Scroll Cases, 4 flasks of Oil, 5 sheets of Paper, 5 sheets of Parchment,
    Perfume, and Tinderbox.'
  sha256: 3f1c7947f62e17ed9acc90577ade8388996c9e6152a8d86fe39a2662be165f44
  status: contract_specified
splat_requires_mechanics: *id001
---

# Diplomat’s Pack (39 GP)

<!-- SRD page 97 -->

A Diplomat’s Pack contains the following items:\
Chest, Fine Clothes, Ink, 5 Ink Pens, Lamp, 2 Map\
or Scroll Cases, 4 flasks of Oil, 5 sheets of Paper, 5\
sheets of Parchment, Perfume, and Tinderbox.\
