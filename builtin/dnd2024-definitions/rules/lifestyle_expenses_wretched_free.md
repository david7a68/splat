---
splat_schema: 1
splat_id: 4b98a7da-5bb9-5961-83d8-20881041149c
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_wretched_free
splat_name: Wretched (Free)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses_wretched_free
  parameters:
    lifestyle:
      id: wretched
      cost_gp_per_day: 0
    displaced_vehicle_table:
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
  implementation: dnd2024:rules_lifestyle_expenses_wretched_free
splat_contracts:
- id: dnd2024:rules_lifestyle_expenses_wretched_free
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
    days:
      required: false
      type: integer
  outputs:
  - cost
  - availability
  events: []
  capabilities:
  - economy
  - services
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  engine_gaps: &id001
  - dnd2024:rules_lifestyle_expenses_wretched_free
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 28ad7d124d28740d267132e077e8ef2cda7eaf22871331e17e134ad1bdaf697f
  parent_name: Lifestyle Expenses
  line_span:
    first: 8789
    count: 12
splat_clauses:
- key: source_rules
  text: '# Wretched (Free) You survive via chance and charity. You’re often exposed to natural dangers
    as a result of sleeping outside. AC HP Damage Threshold Cost 13 300 — 40,000 GP 15 500 20 30,000 GP
    15 100 10 3,000 GP 15 300 15 10,000 GP 11 50 — 50 GP 15 300 15 10,000 GP 15 500 20 25,000 GP'
  sha256: 3730082e756a1b3d6de20cb2ee9000f202763c6df052c641ac9bdad2b771a165
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wretched (Free)

<!-- SRD page 101 -->

You survive via chance and charity. You’re often\
exposed to natural dangers as a result of sleeping\
outside.\
AC HP Damage Threshold Cost\
13 300 — 40,000 GP\
15 500 20 30,000 GP\
15 100 10 3,000 GP\
15 300 15 10,000 GP\
11 50 — 50 GP\
15 300 15 10,000 GP\
15 500 20 25,000 GP\
