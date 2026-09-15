---
splat_schema: 1
splat_id: 110756e4-fd60-5ef1-90e2-dd5e90eb40c2
splat_kind: item
splat_key: dnd2024:equipment/component_pouch_25_gp
splat_name: Component Pouch (25 GP)
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
    value: 25
    unit: gp
  mass:
    value: 2.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/component_pouch_25_gp
  implementation: dnd2024:equipment_component_pouch_25_gp
  parameters:
    watertight: true
    material_components:
      available: all_free_components_of_owner_spells
      priced_components_included: false
      rule:
        key: dnd2024:rules/spellcasting
    mass:
      value: 2.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_component_pouch_25_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - item_properties
  - conditional_modifiers
  events: []
  capabilities:
  - equipment
  - conditional_modifiers
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_component_pouch_25_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 11c30bb285b063051cfffba97c600fc279aadf2c767f525ef868bb8a263e094d
  line_span:
    first: 8366
    count: 4
splat_clauses:
- key: source_rules
  text: '# Component Pouch (25 GP) A Component Pouch is watertight and filled with compartments that hold
    all the free Material components of your spells.'
  sha256: 4bf734d3d9a1d40535c5033e813b9c667b60bca41a38e81699301ea46c91f95e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Component Pouch (25 GP)

<!-- SRD page 97 -->

A Component Pouch is watertight and filled with\
compartments that hold all the free Material com-\
ponents of your spells.\
