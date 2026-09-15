---
splat_schema: 1
splat_id: f8425ca4-ea07-5bf6-89bb-a6cadc0a1575
splat_kind: rule
splat_key: dnd2024:rules/exploration_vehicles
splat_name: Vehicles
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_vehicles
  parameters:
    land_vehicle:
      pace_choice: true
    water_vehicle:
      pace_choice: false
      speed_source: vehicle
      travel_hours_per_day_maximum: 24
      daily_hours_authority: gm
      daily_hours_factors:
      - vessel
      - crew_size
  implementation: dnd2024:rules_exploration_vehicles
splat_contracts:
- id: dnd2024:rules_exploration_vehicles
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
  - 12
  engine_gaps: &id001
  - dnd2024:rules_exploration_vehicles
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 486299ab28a51f5c08602047f7cea028df3c573f15cbe32cbb005571fcf80970
  parent_name: Exploration
  line_span:
    first: 753
    count: 8
splat_clauses:
- key: source_rules
  text: '# Vehicles Travelers in wagons, carriages, or other land vehicles choose a pace as normal. Characters
    in a waterborne vessel are limited to the speed of the vessel, and they don’t choose a travel pace.
    Depending on the vessel and the size of the crew, ships might be able to travel for up to 24 hours
    per day. “Equipment” includes vehicles for sale.'
  sha256: ba60bed8431d980f072d791534e46f989f40f47ed226feea9ca0b706d8c5a383
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vehicles

<!-- SRD page 12 -->

Travelers in wagons, carriages, or other land vehi-\
cles choose a pace as normal. Characters in a water-\
borne vessel are limited to the speed of the vessel,\
and they don’t choose a travel pace. Depending on\
the vessel and the size of the crew, ships might be\
able to travel for up to 24 hours per day. “Equip-\
ment” includes vehicles for sale.\
