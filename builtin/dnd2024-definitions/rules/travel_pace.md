---
splat_schema: 1
splat_id: e29ea5b8-5087-51fb-aa95-c7e475fb5152
splat_kind: rule
splat_key: dnd2024:rules/travel_pace
splat_name: Travel Pace
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/travel_pace
  parameters:
    paces:
    - normal
    - fast
    - slow
    maximum_determined_by: predominant_terrain_per_journey_stage
    terrain_table:
    - terrain: arctic
      maximum_pace: fast
      encounter_distance_ft:
        dice:
          count: 6
          sides: 6
        multiplier: 10
      foraging_dc: 20
      navigation_dc: 10
      search_dc: 10
    - terrain: coastal
      maximum_pace: normal
      encounter_distance_ft:
        dice:
          count: 2
          sides: 10
        multiplier: 10
      foraging_dc: 10
      navigation_dc: 5
      search_dc: 15
    - terrain: desert
      maximum_pace: normal
      encounter_distance_ft:
        dice:
          count: 6
          sides: 6
        multiplier: 10
      foraging_dc: 20
      navigation_dc: 10
      search_dc: 10
    - terrain: forest
      maximum_pace: normal
      encounter_distance_ft:
        dice:
          count: 2
          sides: 8
        multiplier: 10
      foraging_dc: 10
      navigation_dc: 15
      search_dc: 15
    - terrain: grassland
      maximum_pace: fast
      encounter_distance_ft:
        dice:
          count: 6
          sides: 6
        multiplier: 10
      foraging_dc: 15
      navigation_dc: 5
      search_dc: 15
    - terrain: hill
      maximum_pace: normal
      encounter_distance_ft:
        dice:
          count: 2
          sides: 10
        multiplier: 10
      foraging_dc: 15
      navigation_dc: 10
      search_dc: 15
    - terrain: mountain
      maximum_pace: slow
      encounter_distance_ft:
        dice:
          count: 4
          sides: 10
        multiplier: 10
      foraging_dc: 20
      navigation_dc: 15
      search_dc: 20
    - terrain: swamp
      maximum_pace: slow
      encounter_distance_ft:
        dice:
          count: 2
          sides: 8
        multiplier: 10
      foraging_dc: 10
      navigation_dc: 15
      search_dc: 20
    - terrain: underdark
      maximum_pace: normal
      encounter_distance_ft:
        dice:
          count: 2
          sides: 6
        multiplier: 10
      foraging_dc: 20
      navigation_dc: 10
      search_dc: 20
    - terrain: urban
      maximum_pace: normal
      encounter_distance_ft:
        dice:
          count: 2
          sides: 6
        multiplier: 10
      foraging_dc: 20
      navigation_dc: 15
      search_dc: 15
    - terrain: waterborne
      maximum_pace: vehicle_speed
      encounter_distance_ft:
        dice:
          count: 6
          sides: 6
        multiplier: 10
      foraging_dc: 15
      navigation_dc: 10
      search_dc: 15
    arctic_fast_pace_requires_appropriate_equipment: true
    waterborne_uses_vehicle_speed: true
  implementation: dnd2024:rules_travel_pace
splat_contracts:
- id: dnd2024:rules_travel_pace
  inputs:
    party:
      type: participant_list
      required: true
    terrain:
      type: choice
      required: true
    pace:
      type: choice
      required: true
  outputs:
  - pace_limit
  - terrain_checks
  - encounter_distance
  events: []
  capabilities:
  - travel
  - exploration
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 192
  engine_gaps: &id001
  - dnd2024:rules_travel_pace
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 0e21134aa0f856c84fed17e82d98ed87a8999149fa395f93de27110f736d09fd
  parent_name: Travel Pace
  line_span:
    first: 9326
    count: 8
splat_clauses:
- key: source_rules
  text: '# Travel Pace A group of characters can travel overland at a Normal, Fast, or Slow pace, as described
    in “Playing the Game.” During any journey stage, the predominant terrain determines the characters’
    maximum travel pace, as shown in the Maximum Pace column of the Travel Terrain table. Certain factors
    can affect a group’s travel pace.'
  sha256: a10f868ba520100df4fbd4f2a37a11c8c52706443999d1ec4232d8abde8be694
  status: contract_specified
splat_requires_mechanics: *id001
---

# Travel Pace

<!-- SRD page 192 -->

A group of characters can travel overland at a Nor-\
mal, Fast, or Slow pace, as described in “Playing the\
Game.” During any journey stage, the predominant\
terrain determines the characters’ maximum travel\
pace, as shown in the Maximum Pace column of the\
Travel Terrain table. Certain factors can affect a\
group’s travel pace.\
