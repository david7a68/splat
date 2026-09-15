---
splat_schema: 1
splat_id: 0cdad0a1-f0bb-5c2a-859a-469173a32d0f
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_vehicles
splat_name: Vehicles
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/travel_pace_vehicles
  parameters:
    travel_rate: vehicle_speed_mph
    travel_pace_choice: false
  implementation: dnd2024:rules_travel_pace_vehicles
splat_contracts:
- id: dnd2024:rules_travel_pace_vehicles
  inputs:
    travelers:
      type: participant_list
      required: true
    vehicle:
      required: false
      type: object
    journey:
      type: resolution
      required: true
  outputs:
  - travel_rate
  - travel_limits
  events: []
  capabilities:
  - travel
  - vehicles
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 192
  engine_gaps: &id001
  - dnd2024:rules_travel_pace_vehicles
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 16c0feb9e604a19f97f345eb7c53c93688d515c24c0f95505ff7701d5f33f103
  parent_name: Travel Pace
  line_span:
    first: 9376
    count: 5
splat_clauses:
- key: source_rules
  text: '# Vehicles Characters traveling in a vehicle use the vehicle’s speed in miles per hour (as shown
    in “Equipment”) to determine their rate of travel, and they don’t choose a travel pace.'
  sha256: 4dd535a3b95b80fa1150ed3c2abc72606845148a2823a0ffbcea06d2770fe14c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vehicles

<!-- SRD page 192 -->

Characters traveling in a vehicle use the vehicle’s\
speed in miles per hour (as shown in “Equipment”)\
to determine their rate of travel, and they don’t\
choose a travel pace.\
