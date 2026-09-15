---
splat_schema: 1
splat_id: f0ae9855-72a2-5415-af89-73df62875bd5
splat_kind: rule
splat_key: dnd2024:rules/traps_collapsing_roof
splat_name: Collapsing Roof
splat_origin:
  key: dnd2024:reference/pages/199
splat_sources:
- title: SRD 5.2.1
  locator: pp. 199
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/traps_collapsing_roof
  parameters:
    severity: deadly
    default_levels:
    - 1
    - 4
    trigger: first_creature_crosses_trip_wire
    wire_height_inches: 3
    support_count: 2
    duration: instantaneous
    targets: creatures_beneath_unstable_ceiling
    save:
      ability: dexterity
      dc: 13
      failure:
        damage:
          dice:
            count: 2
            sides: 10
        average: 11
        type: bludgeoning
      success:
        damage_fraction: 0.5
        rounding: floor
    aftermath: area_becomes_difficult_terrain
    detect:
      action: search
      ability: wisdom
      skill: perception
      dc: 11
      success_reveals:
      - trip_wire
      - unstable_ceiling
    disarm_or_bypass:
      requires: detected
      options:
      - cut_wire
      - avoid_wire
      check_required: false
    higher_level_variants:
    - levels:
      - 5
      - 10
      damage:
        dice:
          count: 4
          sides: 10
      average: 22
      dc: 15
    - levels:
      - 11
      - 16
      damage:
        dice:
          count: 10
          sides: 10
      average: 55
      dc: 17
    - levels:
      - 17
      - 20
      damage:
        dice:
          count: 18
          sides: 10
      average: 99
      dc: 19
    geometry_assumption:
      unstable_ceiling_area: gm_supplied_location
  implementation: dnd2024:rule_traps_collapsing_roof
splat_contracts:
- id: dnd2024:rule_traps_collapsing_roof
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - creature.crosses_trip_wire
  capabilities:
  - traps
  - damage
  - terrain
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 199
  engine_gaps: &id001
  - dnd2024:rule_traps_collapsing_roof
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 0e02d40cce03f4d7a37c7211b4a41680dbd616e29106d0674bca766a0de68062
  parent_name: Traps
  line_span:
    first: 9970
    count: 29
splat_clauses:
- key: source_rules
  text: '# Collapsing Roof Deadly Trap (Levels 1–4) Trigger: A creature crosses a trip wire Duration:
    Instantaneous This trap uses a trip wire to collapse an unstable section of ceiling. The trip wire
    is 3 inches off the ground and stretches between two weak supports that topple when the trip wire
    is pulled. The first creature that crosses the trip wire causes the supports to topple and the unstable
    section of ceiling to collapse. Each creature beneath the unstable section of ceiling must succeed
    on a DC 13 Dexterity saving throw, taking 11 (2d10) Bludgeoning damage on a failed save or half as
    much damage on a successful one. Rubble from the collapse turns the trapped area into Difficult Terrain.
    Detect and Disarm. As a Search action, a creature can examine the trapped area and make a DC 11 Wisdom
    (Perception) check, detecting the trip wire and the unstable section of ceiling on a successful check.
    Once detected, the trip wire can be easily cut or avoided (no ability check required). At Higher Levels.
    You can scale the trap for higher levels by increasing the damage and the save DC, as shown in the
    following table. Levels Bludgeoning Damage Save DC 5–10 22 (4d10) 15 11–16 55 (10d10) 17 17–20 99
    (18d10) 19'
  sha256: f3ac98350386f23b0429c9354b6aa36067087d7e34fe0ebf80036e0e7d54c697
  status: contract_specified
splat_requires_mechanics: *id001
---

# Collapsing Roof

<!-- SRD page 199 -->

Deadly Trap (Levels 1–4)\
Trigger: A creature crosses a trip wire\
Duration: Instantaneous\
This trap uses a trip wire to collapse an unstable\
section of ceiling. The trip wire is 3 inches off the\
ground and stretches between two weak supports\
that topple when the trip wire is pulled.\
The first creature that crosses the trip wire\
causes the supports to topple and the unstable sec-\
tion of ceiling to collapse. Each creature beneath the\
unstable section of ceiling must succeed on a DC 13\
Dexterity saving throw, taking 11 (2d10) Bludgeon-\
ing damage on a failed save or half as much damage\
on a successful one. Rubble from the collapse turns\
the trapped area into Difficult Terrain.\
Detect and Disarm. As a Search action, a creature\
can examine the trapped area and make a DC 11\
Wisdom (Perception) check, detecting the trip wire\
and the unstable section of ceiling on a successful\
check. Once detected, the trip wire can be easily cut\
or avoided (no ability check required).\
At Higher Levels. You can scale the trap for higher\
levels by increasing the damage and the save DC, as\
shown in the following table.\
Levels Bludgeoning Damage Save DC\
5–10 22 (4d10) 15\
11–16 55 (10d10) 17\
17–20 99 (18d10) 19\
