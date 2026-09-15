---
splat_schema: 1
splat_id: 549c3cfc-df28-5bad-89f1-b3cb2ceb2810
splat_kind: rule
splat_key: dnd2024:rules/traps_poisoned_darts
splat_name: Poisoned Darts
splat_origin:
  key: dnd2024:reference/pages/200
splat_sources:
- title: SRD 5.2.1
  locator: pp. 200
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/traps_poisoned_darts
  parameters:
    trap:
      severity: deadly
      levels:
      - 1
      - 4
      trigger: creature_first_moves_onto_pressure_plate_on_turn
      duration: instantaneous
      maximum_activations: 3
      reset_event: next_turn.started
      reset_if_activations_less_than: 3
    targets: all_creatures_in_dart_paths
    save:
      ability: dexterity
      dc: 13
    failure:
      dart_count:
        count: 1
        sides: 3
        add: 0
      damage_per_dart:
        dice:
          count: 1
          sides: 6
          add: 0
        average: 3
        type: poison
    detection:
    - location: walls
      economy: search
      check:
        ability: wisdom
        dc: 15
        skill: perception
      reveals: dart_holes
    - location: floor_pressure_plate
      economy: search
      check:
        ability: wisdom
        dc: 15
        skill: perception
      reveals: pressure_plate
    disarm:
    - operation: plug_all_holes
      materials:
      - wax
      - cloth
      - detritus
      effect: prevents_firing
    - operation: wedge_pressure_plate
      materials:
      - iron_spike
      - similar_object
      effect: prevents_triggering
    scaling:
    - levels:
      - 1
      - 4
      poison_per_dart:
        dice:
          count: 1
          sides: 6
          add: 0
        average: 3
    - levels:
      - 5
      - 10
      poison_per_dart:
        dice:
          count: 2
          sides: 6
          add: 0
        average: 7
    - levels:
      - 11
      - 16
      poison_per_dart:
        dice:
          count: 4
          sides: 6
          add: 0
        average: 14
    - levels:
      - 17
      - 20
      poison_per_dart:
        dice:
          count: 7
          sides: 6
          add: 0
        average: 24
  implementation: dnd2024:rules_traps_poisoned_darts
splat_contracts:
- id: dnd2024:rules_traps_poisoned_darts
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    hazard:
      type: object
      required: true
    event:
      type: event
      required: true
    resolution:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - effects
  - hazard_updates
  events:
  - movement.completed
  - turn.started
  capabilities:
  - hazards
  - effects
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 200
  engine_gaps: &id001
  - dnd2024:rules_traps_poisoned_darts
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: e8e0c58c795a9b39b8aaa6f16f25116f947ffe83309b32ccdee971243f646514
  parent_name: Traps
  line_span:
    first: 10103
    count: 30
splat_clauses:
- key: source_rules
  text: '# Poisoned Darts Deadly Trap (Levels 1–4) Trigger: A creature moves onto a pressure plate Duration:
    Instantaneous, and the trap resets at the start of the next turn if it has activated fewer than three
    times When a creature moves onto this trap’s pressure plate for the first time on a turn, poisoned
    darts shoot from tubes embedded in the surrounding walls. The holes that house these tubes are obscured
    by dust and cobwebs or skillfully hidden amid bas-reliefs, murals, or frescoes. Each creature in the
    darts’ path must succeed on a DC 13 Dexterity saving throw or be struck by 1d3 darts, taking 3 (1d6)
    Poison damage per dart. Detect and Disarm. As a Search action, a creature can examine the walls and
    make a DC 15 Wisdom (Perception) check, detecting the holes on a successful check. Plugging all the
    holes with wax, cloth, or detritus prevents the darts from firing. As a Search action, a creature
    can examine the section of floor that forms the pressure plate and make a DC 15 Wisdom (Perception)
    check, detecting the pressure plate on a successful check. Wedging an Iron Spike or a similar object
    under the pressure plate prevents the trap from triggering. At Higher Levels. You can scale the trap
    for higher levels by increasing each dart’s Poison damage as follows: 7 (2d6) at levels 5–10, 14 (4d6)
    at levels 11–16, or 24 (7d6) at levels 17–20.'
  sha256: 3ed7295e48100cd8809e6ebe516718edbf41080c490ee8626a9c04fbd09ab9e1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Poisoned Darts

<!-- SRD page 200 -->

Deadly Trap (Levels 1–4)\
Trigger: A creature moves onto a pressure plate\
Duration: Instantaneous, and the trap resets at the\
start of the next turn if it has activated fewer than\
three times\
When a creature moves onto this trap’s pressure\
plate for the first time on a turn, poisoned darts\
shoot from tubes embedded in the surrounding\
walls. The holes that house these tubes are ob-\
scured by dust and cobwebs or skillfully hidden\
amid bas-reliefs, murals, or frescoes.\
Each creature in the darts’ path must succeed on\
a DC 13 Dexterity saving throw or be struck by 1d3\
darts, taking 3 (1d6) Poison damage per dart.\
Detect and Disarm. As a Search action, a creature\
can examine the walls and make a DC 15 Wisdom\
(Perception) check, detecting the holes on a suc-\
cessful check. Plugging all the holes with wax, cloth,\
or detritus prevents the darts from firing.\
As a Search action, a creature can examine the\
section of floor that forms the pressure plate and\
make a DC 15 Wisdom (Perception) check, detecting\
the pressure plate on a successful check. Wedging\
an Iron Spike or a similar object under the pressure\
plate prevents the trap from triggering.\
At Higher Levels. You can scale the trap for higher\
levels by increasing each dart’s Poison damage as\
follows: 7 (2d6) at levels 5–10, 14 (4d6) at levels\
11–16, or 24 (7d6) at levels 17–20.\
