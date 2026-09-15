---
splat_schema: 1
splat_id: 0ac6271b-08dd-5592-acc1-10391ec2ffa3
splat_kind: item
splat_key: dnd2024:equipment/map_1_gp
splat_name: Map (1 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 1
    unit: gp
  mass:
    value: 0.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/map_1_gp
  implementation: dnd2024:equipment_map_1_gp
  parameters:
    consult:
      accurate: true
      scope: find_way_in_represented_place
      check_bonus: 5
      ability: wisdom
      skill: survival
    mass:
      value: 0.0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_map_1_gp
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
  - 98
  engine_gaps: &id001
  - dnd2024:equipment_map_1_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: cd19b80fe59d8604d0d3ac329c0c012f7de96a7409f83d1b9f416baa6a263046
  line_span:
    first: 8508
    count: 4
splat_clauses:
- key: source_rules
  text: '# Map (1 GP) If you consult an accurate Map, you gain a +5 bonus to Wisdom (Survival) checks
    you make to find your way in the place represented on it.'
  sha256: cd12a22bc29b0f7f1ed1a62b9e345a02b08b15bc0c2e981dabb22f74960186e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Map (1 GP)

<!-- SRD page 98 -->

If you consult an accurate Map, you gain a +5 bonus\
to Wisdom (Survival) checks you make to find your\
way in the place represented on it.\
