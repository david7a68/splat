---
splat_schema: 1
splat_id: 2442f30a-e4d3-5d0b-b6ce-04eabd8ebed5
splat_kind: item
splat_key: dnd2024:equipment/scholar_s_pack_40_gp
splat_name: Scholar’s Pack (40 GP)
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
    value: 40
    unit: gp
  mass:
    value: 22.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/scholar_s_pack_40_gp
  implementation: dnd2024:equipment_scholar_s_pack_40_gp
  parameters:
    contents:
    - item:
        key: dnd2024:equipment/backpack_2_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/book_25_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/ink_10_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/ink_pen_2_cp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/lamp_5_sp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/oil_1_sp
      quantity: 10
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/parchment_1_sp
      quantity: 10
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/tinderbox_5_sp
      quantity: 1
      unit: purchase_unit
    grant_contents_on_acquisition: true
    mass:
      value: 22.0
      unit: lb
    grant_idempotency: once_per_acquired_pack_instance
splat_contracts:
- id: dnd2024:equipment_scholar_s_pack_40_gp
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
  - dnd2024:equipment_scholar_s_pack_40_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 07d2dcb03a7f30bf594621c292f441092d3d1a50695b7eade8479f25b0d714f0
  line_span:
    first: 8626
    count: 4
splat_clauses:
- key: source_rules
  text: '# Scholar’s Pack (40 GP) A Scholar’s Pack contains the following items: Backpack, Book, Ink,
    Ink Pen, Lamp, 10 flasks of Oil, 10 sheets of Parchment, and Tinderbox.'
  sha256: 19307a93ef69e43d319aa7e0525630af074371faa3f81a6bd646892bdddb271a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scholar’s Pack (40 GP)

<!-- SRD page 99 -->

A Scholar’s Pack contains the following items: Back-\
pack, Book, Ink, Ink Pen, Lamp, 10 flasks of Oil, 10\
sheets of Parchment, and Tinderbox.\
