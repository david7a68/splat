---
splat_schema: 1
splat_id: 20810ca1-9835-5698-ae23-9f0cccaaae71
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_special_movement
splat_name: Special Movement
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/travel_pace_special_movement
  parameters:
    eligibility: sustained_special_movement_speed
    miles_per_hour:
      op: divide
      args:
      - variable: speed_feet
      - 10
    normal_miles_per_day:
      op: multiply
      args:
      - variable: miles_per_hour
      - variable: hours_traveled
    typical_hours_traveled: 8
    fast_miles_per_day:
      op: floor
      args:
      - op: multiply
        args:
        - variable: normal_miles_per_day
        - numerator: 4
          denominator: 3
    slow_miles_per_day:
      op: floor
      args:
      - op: multiply
        args:
        - variable: normal_miles_per_day
        - numerator: 2
          denominator: 3
    fast_pace_regardless_of_terrain_when_any:
    - flying
    - special_movement_ignores_difficult_terrain
    terrain_table:
    - terrain: arctic
      maximum_pace: fast
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 6
            sides: 6
        - 10
      foraging_dc: 20
      navigation_dc: 10
      search_dc: 10
    - terrain: coastal
      maximum_pace: normal
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 2
            sides: 10
        - 10
      foraging_dc: 10
      navigation_dc: 5
      search_dc: 15
    - terrain: desert
      maximum_pace: normal
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 6
            sides: 6
        - 10
      foraging_dc: 20
      navigation_dc: 10
      search_dc: 10
    - terrain: forest
      maximum_pace: normal
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 2
            sides: 8
        - 10
      foraging_dc: 10
      navigation_dc: 15
      search_dc: 15
    - terrain: grassland
      maximum_pace: fast
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 6
            sides: 6
        - 10
      foraging_dc: 15
      navigation_dc: 5
      search_dc: 15
    - terrain: hill
      maximum_pace: normal
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 2
            sides: 10
        - 10
      foraging_dc: 15
      navigation_dc: 10
      search_dc: 15
    - terrain: mountain
      maximum_pace: slow
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 4
            sides: 10
        - 10
      foraging_dc: 20
      navigation_dc: 15
      search_dc: 20
    - terrain: swamp
      maximum_pace: slow
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 2
            sides: 8
        - 10
      foraging_dc: 10
      navigation_dc: 15
      search_dc: 20
    - terrain: underdark
      maximum_pace: normal
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 2
            sides: 6
        - 10
      foraging_dc: 20
      navigation_dc: 10
      search_dc: 20
    - terrain: urban
      maximum_pace: normal
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 2
            sides: 6
        - 10
      foraging_dc: 20
      navigation_dc: 15
      search_dc: 15
    - terrain: waterborne
      maximum_pace: vehicle_speed
      encounter_distance_feet:
        op: multiply
        args:
        - dice:
            count: 6
            sides: 6
        - 10
      foraging_dc: 15
      navigation_dc: 10
      search_dc: 15
    arctic_fast_requires: appropriate_equipment
    waterborne_speed_source: vehicle
  implementation: dnd2024:rule_travel_pace_special_movement
splat_contracts:
- id: dnd2024:rule_travel_pace_special_movement
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    hours_traveled:
      type: number
      required: true
    miles_per_hour:
      type: number
      required: true
    normal_miles_per_day:
      type: number
      required: true
    speed_feet:
      type: number
      required: true
  outputs:
  - travel_rates
  events: []
  capabilities:
  - travel
  - random_tables
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 192
  engine_gaps: &id001
  - dnd2024:rule_travel_pace_special_movement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: d4b82644a1d307ccd96ecde5f034d8ebf13b6efad017555e1b98e7ee6ac70a55
  parent_name: Travel Pace
  line_span:
    first: 9348
    count: 28
splat_clauses:
- key: source_rules
  text: '# Special Movement If a party can travel at a high Speed for an extended time, as with a spell
    such as Wind Walk or a magic item such as a Carpet of Flying, translate the party’s Speed into travel
    rates using these rules: Travel Terrain Terrain Maximum Pace Encounter Distance Arctic Fast* 6d6 ×
    10 feet Coastal Normal 2d10 × 10 feet Desert Normal 6d6 × 10 feet Forest Normal 2d8 × 10 feet Grassland
    Fast 6d6 × 10 feet Hill Normal 2d10 × 10 feet Mountain Slow 4d10 × 10 feet Swamp Slow 2d8 × 10 feet
    Underdark Normal 2d6 × 10 feet Urban Normal 2d6 × 10 feet Waterborne Special† 6d6 × 10 feet *Appropriate
    equipment (such as skis) is necessary to keep up a Fast pace in Arctic terrain. †Characters’ rate
    of travel while waterborne depends on the vehicle carrying them; see “Vehicles.” Miles per hour =
    Speed ÷ 10 Miles per day (Normal pace) = Miles per hour × number of hours traveled (typically 8) Fast
    pace = Miles per day × 1⅓ (round down) Slow pace = Miles per day × 2/3 (round down) If the characters
    are flying or their special movement allows them to ignore Difficult Terrain, they can move at a Fast
    pace regardless of the terrain.'
  sha256: 72fa3dbff17c17b1ce555198acedb50f721f7f6f201a9fe2217f698807778275
  status: contract_specified
splat_requires_mechanics: *id001
---

# Special Movement

<!-- SRD page 192 -->

If a party can travel at a high Speed for an extended\
time, as with a spell such as Wind Walk or a magic\
item such as a Carpet of Flying, translate the party’s\
Speed into travel rates using these rules:\
Travel Terrain\
Terrain Maximum Pace Encounter Distance\
Arctic Fast* 6d6 × 10 feet\
Coastal Normal 2d10 × 10 feet\
Desert Normal 6d6 × 10 feet\
Forest Normal 2d8 × 10 feet\
Grassland Fast 6d6 × 10 feet\
Hill Normal 2d10 × 10 feet\
Mountain Slow 4d10 × 10 feet\
Swamp Slow 2d8 × 10 feet\
Underdark Normal 2d6 × 10 feet\
Urban Normal 2d6 × 10 feet\
Waterborne Special† 6d6 × 10 feet\
*Appropriate equipment (such as skis) is necessary to keep up a Fast pace in Arctic terrain.\
†Characters’ rate of travel while waterborne depends on the vehicle carrying them; see “Vehicles.”\
Miles per hour = Speed ÷ 10\
Miles per day (Normal pace) = Miles per hour\
× number of hours traveled (typically 8)\
Fast pace = Miles per day × 1⅓ (round down)\
Slow pace = Miles per day × 2/3 (round down)\
If the characters are flying or their special move-\
ment allows them to ignore Difficult Terrain, they\
can move at a Fast pace regardless of the terrain.\
