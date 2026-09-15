---
splat_schema: 1
splat_id: ead59709-203b-5520-be75-a6ad1a7e69e4
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_high_altitude
splat_name: High Altitude
splat_origin:
  key: dnd2024:reference/pages/195
splat_sources:
- title: SRD 5.2.1
  locator: pp. 195
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/environmental_effects_high_altitude
  parameters:
    altitude_feet_minimum: 10000
    travel_hours_multiplier: 2
    acclimation:
      days_minimum: 30
      above_altitude_feet: 20000
      above_altitude_requires_native: true
      acclimated_exempt_from_multiplier: true
  implementation: dnd2024:rules_environmental_effects_high_altitude
splat_contracts:
- id: dnd2024:rules_environmental_effects_high_altitude
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
  - 195
  engine_gaps: &id001
  - dnd2024:rules_environmental_effects_high_altitude
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 6efe990a4b9b455dbe71da014f2556a6e774e594946ce892a1e330175591603c
  parent_name: Environmental Effects
  line_span:
    first: 9651
    count: 12
splat_clauses:
- key: source_rules
  text: '# High Altitude Traveling at altitudes of 10,000 feet or higher above sea level is taxing for
    most creatures because of the reduced amount of oxygen in the air. Each hour such a creature spends
    traveling at high altitude counts as 2 hours for the purpose of determining how long that creature
    can travel (see “Travel Pace”). Creatures can become acclimated to a high altitude by spending 30
    days or more at this elevation. Creatures can’t become acclimated to elevations above 20,000 feet
    unless they are native to such environments.'
  sha256: 2141e44f1afcabb8eddd475e13155f47cbcab0a5aa7e14e85b077ef068408880
  status: contract_specified
splat_requires_mechanics: *id001
---

# High Altitude

<!-- SRD page 195 -->

Traveling at altitudes of 10,000 feet or higher above\
sea level is taxing for most creatures because of the\
reduced amount of oxygen in the air. Each hour such\
a creature spends traveling at high altitude counts\
as 2 hours for the purpose of determining how long\
that creature can travel (see “Travel Pace”).\
Creatures can become acclimated to a high alti-\
tude by spending 30 days or more at this elevation.\
Creatures can’t become acclimated to elevations\
above 20,000 feet unless they are native to such\
environments.\
