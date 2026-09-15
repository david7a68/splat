---
splat_schema: 1
splat_id: 61429588-f735-549e-9b5e-38558a098248
splat_kind: item
splat_key: dnd2024:equipment/entertainer_s_pack_40_gp
splat_name: Entertainer’s Pack (40 GP)
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
    value: 40
    unit: gp
  mass:
    value: 58.5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/entertainer_s_pack_40_gp
  implementation: dnd2024:equipment_entertainer_s_pack_40_gp
  parameters:
    contents:
    - item:
        key: dnd2024:equipment/backpack_2_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/bedroll_1_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/bell_1_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/lantern_bullseye_10_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/costume_5_gp
      quantity: 3
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/mirror_5_gp
      quantity: 1
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/oil_1_sp
      quantity: 8
      unit: purchase_unit
    - item:
        key: dnd2024:equipment/rations_5_sp
      quantity: 9
      unit: day
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
      value: 58.5
      unit: lb
    grant_idempotency: once_per_acquired_pack_instance
splat_contracts:
- id: dnd2024:equipment_entertainer_s_pack_40_gp
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
  - dnd2024:equipment_entertainer_s_pack_40_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: ed9dbf29ba949c36180e06f77ecb471a1070f8eed02a25601c9eb6a3cc0b5aca
  line_span:
    first: 8398
    count: 5
splat_clauses:
- key: source_rules
  text: '# Entertainer’s Pack (40 GP) An Entertainer’s Pack contains the following items: Backpack, Bedroll,
    Bell, Bullseye Lantern, 3 Costumes, Mirror, 8 flasks of Oil, 9 days of Rations, Tinderbox, and Waterskin.'
  sha256: df7977865d5f90c460577d841e366ef326ba4cb61a8e6c726a23d3598acd68f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Entertainer’s Pack (40 GP)

<!-- SRD page 97 -->

An Entertainer’s Pack contains the following items:\
Backpack, Bedroll, Bell, Bullseye Lantern, 3 Cos-\
tumes, Mirror, 8 flasks of Oil, 9 days of Rations, Tin-\
derbox, and Waterskin.\
