---
splat_schema: 1
splat_id: 5ed175ce-1faa-5182-8c02-eab4dcf4638d
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_saddles
splat_name: Saddles
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_saddles
  parameters:
    saddle_includes:
    - bit
    - bridle
    - reins
    - other_required_equipment
    military_saddle:
      remain_mounted_ability_checks: advantage
    exotic_saddle_required_for:
    - aquatic_mount
    - flying_mount
    mounts:
    - kind: camel
      capacity_lb: 450
      cost_gp: 50
    - kind: elephant
      capacity_lb: 1320
      cost_gp: 200
    - kind: draft_horse
      capacity_lb: 540
      cost_gp: 50
    - kind: riding_horse
      capacity_lb: 480
      cost_gp: 75
    - kind: mastiff
      capacity_lb: 195
      cost_gp: 25
    - kind: mule
      capacity_lb: 420
      cost_gp: 8
    - kind: pony
      capacity_lb: 225
      cost_gp: 30
    - kind: warhorse
      capacity_lb: 540
      cost_gp: 400
    tack_and_vehicles:
    - kind: carriage
      weight_lb: 600
      cost_gp: 100
    - kind: cart
      weight_lb: 200
      cost_gp: 15
    - kind: chariot
      weight_lb: 100
      cost_gp: 250
    - kind: feed_per_day
      weight_lb: 10
      cost_gp: 0.05
    - kind: exotic_saddle
      weight_lb: 40
      cost_gp: 60
    - kind: military_saddle
      weight_lb: 30
      cost_gp: 20
    - kind: riding_saddle
      weight_lb: 25
      cost_gp: 10
    - kind: sled
      weight_lb: 300
      cost_gp: 20
    - kind: stabling_per_day
      weight_lb: null
      cost_gp: 0.5
    - kind: wagon
      weight_lb: 400
      cost_gp: 35
  implementation: dnd2024:rules_mounts_and_vehicles_saddles
splat_contracts:
- id: dnd2024:rules_mounts_and_vehicles_saddles
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - saddle_effects
  - mount_catalog
  - vehicle_catalog
  events: []
  capabilities:
  - mounts
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 100
  engine_gaps: &id001
  - dnd2024:rules_mounts_and_vehicles_saddles
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8599ea05fab2a199819bcf0309a891dbdfed7786a8698ad9d8aca6585e9d1a8e
  parent_name: Mounts and Vehicles
  line_span:
    first: 8701
    count: 29
splat_clauses:
- key: source_rules
  text: '# Saddles A saddle comes with a bit, a bridle, reins, and any other equipment needed to use the
    saddle. A Military Saddle gives Advantage on any ability check you make to remain mounted. An Exotic
    Saddle is required for riding an aquatic or a flying mount. Mounts and Other Animals Item Carrying
    Capacity Cost Camel 450 lb. 50 GP Elephant 1,320 lb. 200 GP Horse, Draft 540 lb. 50 GP Horse, Riding
    480 lb. 75 GP Mastiff 195 lb. 25 GP Mule 420 lb. 8 GP Pony 225 lb. 30 GP Warhorse 540 lb. 400 GP Tack,
    Harness, and Drawn Vehicles Item Weight Cost Carriage 600 lb. 100 GP Cart 200 lb. 15 GP Chariot 100
    lb. 250 GP Feed per day 10 lb. 5 CP Saddle Exotic 40 lb. 60 GP Military 30 lb. 20 GP Riding 25 lb.
    10 GP Sled 300 lb. 20 GP Stabling per day — 5 SP Wagon 400 lb. 35 GP'
  sha256: 48a80ed1ba86676ab2a3c12f2be3cb097e483c6e140ce1719a3846aa231c3181
  status: contract_specified
splat_requires_mechanics: *id001
---

# Saddles

<!-- SRD page 100 -->

A saddle comes with a bit, a bridle, reins, and any\
other equipment needed to use the saddle. A Mil-\
itary Saddle gives Advantage on any ability check\
you make to remain mounted. An Exotic Saddle is\
required for riding an aquatic or a flying mount.\
Mounts and Other Animals\
Item Carrying Capacity Cost\
Camel 450 lb. 50 GP\
Elephant 1,320 lb. 200 GP\
Horse, Draft 540 lb. 50 GP\
Horse, Riding 480 lb. 75 GP\
Mastiff 195 lb. 25 GP\
Mule 420 lb. 8 GP\
Pony 225 lb. 30 GP\
Warhorse 540 lb. 400 GP\
Tack, Harness, and Drawn Vehicles\
Item Weight Cost\
Carriage 600 lb. 100 GP\
Cart 200 lb. 15 GP\
Chariot 100 lb. 250 GP\
Feed per day 10 lb. 5 CP\
Saddle\
Exotic 40 lb. 60 GP\
Military 30 lb. 20 GP\
Riding 25 lb. 10 GP\
Sled 300 lb. 20 GP\
Stabling per day — 5 SP\
Wagon 400 lb. 35 GP\
