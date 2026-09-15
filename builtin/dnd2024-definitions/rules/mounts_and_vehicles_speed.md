---
splat_schema: 1
splat_id: 35bed7e8-3ea2-55b1-a6ff-f772be00fc71
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_speed
splat_name: Speed
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100, 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_speed
  parameters:
    vehicles:
    - id: airship
      speed_mph: 8
      crew: 10
      passengers: 20
      cargo_tons: 1
      ac: 13
      hp: 300
      damage_threshold: null
      cost_gp: 40000
    - id: galley
      speed_mph: 4
      crew: 80
      passengers: 0
      cargo_tons: 150
      ac: 15
      hp: 500
      damage_threshold: 20
      cost_gp: 30000
    - id: keelboat
      speed_mph: 1
      crew: 1
      passengers: 6
      cargo_tons: 0.5
      ac: 15
      hp: 100
      damage_threshold: 10
      cost_gp: 3000
    - id: longship
      speed_mph: 3
      crew: 40
      passengers: 150
      cargo_tons: 10
      ac: 15
      hp: 300
      damage_threshold: 15
      cost_gp: 10000
    - id: rowboat
      speed_mph: 1.5
      crew: 1
      passengers: 3
      cargo_tons: 0
      ac: 11
      hp: 50
      damage_threshold: null
      cost_gp: 50
    - id: sailing_ship
      speed_mph: 2
      crew: 20
      passengers: 20
      cargo_tons: 100
      ac: 15
      hp: 300
      damage_threshold: 15
      cost_gp: 10000
    - id: warship
      speed_mph: 2.5
      crew: 60
      passengers: 60
      cargo_tons: 200
      ac: 15
      hp: 500
      damage_threshold: 20
      cost_gp: 25000
    strong_headwind_speed_multiplier: 0.5
    no_wind:
      sailing_movement: false
      rowing_required: true
    river_boats:
    - keelboat
    - rowboat
    downstream:
      add_current_speed: true
      typical_current_mph: 3
    significant_upstream_current:
      rowing_allowed: false
      shore_draft_animal_towing_allowed: true
    rowboat_weight_lb: 100
  implementation: dnd2024:rules_mounts_and_vehicles_speed
splat_contracts:
- id: dnd2024:rules_mounts_and_vehicles_speed
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
  - 100
  - 101
  engine_gaps: &id001
  - dnd2024:rules_mounts_and_vehicles_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: e77706ac4899831e7e99f284684e97662c7a97f96f18512588c7f6f015ac536b
  parent_name: Mounts and Vehicles
  line_span:
    first: 8734
    count: 20
splat_clauses:
- key: source_rules
  text: '# Speed A ship sailing against a strong wind moves at half speed. In a dead calm (no wind), waterborne
    ships can’t move under sail and must be rowed. Keelboats and Rowboats are used on lakes and rivers.
    If going downstream, add the speed of the current (typically 3 miles per hour) to the speed of the
    vehicle. These vehicles can’t be rowed against any significant current, but they can be pulled upstream
    by draft animals on the shores. A Rowboat can be carried and weighs 100 pounds. Airborne and Waterborne
    Vehicles Ship Speed Crew Passengers Cargo (Tons) Airship 8 mph 10 20 1 Galley 4 mph 80 — 150 Keelboat
    1 mph 1 6 1/2 Longship 3 mph 40 150 10 Rowboat 1½ mph 1 3 — Sailing Ship 2 mph 20 20 100 Warship 2½
    mph 60 60 200'
  sha256: 25b67c5a377cf84e7d7bfd9221f95d720db4a82df3046a08eb5f99a0358b89d1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Speed

<!-- SRD page 100 -->

A ship sailing against a strong wind moves at half\
speed. In a dead calm (no wind), waterborne ships\
can’t move under sail and must be rowed. Keelboats\
and Rowboats are used on lakes and rivers. If going\
downstream, add the speed of the current (typically\
3 miles per hour) to the speed of the vehicle. These\
vehicles can’t be rowed against any significant cur-\
rent, but they can be pulled upstream by draft an-\
imals on the shores. A Rowboat can be carried and\
weighs 100 pounds.\

<!-- SRD page 101 -->

Airborne and Waterborne Vehicles\
Ship Speed Crew Passengers Cargo (Tons)\
Airship 8 mph 10 20 1\
Galley 4 mph 80 — 150\
Keelboat 1 mph 1 6 1/2\
Longship 3 mph 40 150 10\
Rowboat 1½ mph 1 3 —\
Sailing Ship 2 mph 20 20 100\
Warship 2½ mph 60 60 200\
