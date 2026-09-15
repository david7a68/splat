---
splat_schema: 1
splat_id: 55bf91d5-ff23-5d1a-b322-3d1a5559caf7
splat_kind: item
splat_key: dnd2024:equipment/case_map_or_scroll_1_gp
splat_name: Case, Map or Scroll (1 GP)
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
    value: 1
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/case_map_or_scroll_1_gp
  implementation: dnd2024:equipment_case_map_or_scroll_1_gp
  parameters:
    capacity:
      alternative_capacities:
      - item:
          key: dnd2024:equipment/paper_2_sp
        count: 10
      - item:
          key: dnd2024:equipment/parchment_1_sp
        count: 5
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_case_map_or_scroll_1_gp
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
  - dnd2024:equipment_case_map_or_scroll_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 829b5d0e94dc3ef3c87bc6f3eba3da500e5a77f96a738536692c099144c2624b
  line_span:
    first: 8335
    count: 3
splat_clauses:
- key: source_rules
  text: '# Case, Map or Scroll (1 GP) A Map or Scroll Case holds up to 10 sheets of paper or 5 sheets
    of parchment.'
  sha256: 7df5cb358edffd89e47b2495e0ef966d5cd160beb4f885af766f1c7236d098df
  status: contract_specified
splat_requires_mechanics: *id001
---

# Case, Map or Scroll (1 GP)

<!-- SRD page 96 -->

A Map or Scroll Case holds up to 10 sheets of paper\
or 5 sheets of parchment.\
