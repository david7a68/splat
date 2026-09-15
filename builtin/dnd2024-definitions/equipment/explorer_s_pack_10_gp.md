---
splat_schema: 1
splat_id: ac6d8d9a-faa7-563d-b2ad-29ad961d51d5
splat_kind: item
splat_key: dnd2024:equipment/explorer_s_pack_10_gp
splat_name: Explorer’s Pack (10 GP)
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
    value: 10
    unit: gp
  mass:
    value: 55.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/explorer_s_pack_10_gp
  implementation: dnd2024:equipment_explorer_s_pack_10_gp
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
- id: dnd2024:equipment_explorer_s_pack_10_gp
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
  - dnd2024:equipment_explorer_s_pack_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: e29adc7b5e139ee4474975ba72754e341a19993972e558dad3a3bd352b34e6ba
  line_span:
    first: 8403
    count: 4
splat_clauses:
- key: source_rules
  text: '# Explorer’s Pack (10 GP) An Explorer’s Pack contains the following items: Backpack, Bedroll,
    2 flasks of Oil, 10 days of Rations, Rope, Tinderbox, 10 Torches, and Waterskin.'
  sha256: 1606b10da510366bdc3b0fad2a5d3c6ecaf55c130bbf4d12802bc78f05a7e57e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Explorer’s Pack (10 GP)

<!-- SRD page 97 -->

An Explorer’s Pack contains the following items:\
Backpack, Bedroll, 2 flasks of Oil, 10 days of Ra-\
tions, Rope, Tinderbox, 10 Torches, and Waterskin.\
