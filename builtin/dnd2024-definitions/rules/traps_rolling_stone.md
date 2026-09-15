---
splat_schema: 1
splat_id: a367499e-87dd-54dd-bdb3-c16d1fee56f7
splat_kind: rule
splat_key: dnd2024:rules/traps_rolling_stone
splat_name: Rolling Stone
splat_origin:
  key: dnd2024:reference/pages/201
splat_sources:
- title: SRD 5.2.1
  locator: pp. 201
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/traps_rolling_stone
  parameters:
    severity_by_level:
    - levels:
      - 11
      - 16
      severity: deadly
    - levels:
      - 17
      - 20
      severity: nuisance
    trigger: creature_enters_pressure_plate
    duration: until_stone_stops
    stone:
      shape: sphere
      radius_feet: 5
      size: large
      material: solid_stone
      ac: 17
      hit_points: 100
      damage_threshold: 10
      damage_immunities:
      - poison
      - psychic
      initiative_bonus: 8
      movement_feet_per_turn: 60
    initiative_participants: stone_and_nearby_creatures
    movement:
      direction: single_direction
      obstacle_can_redirect: true
      can_pass_creature_spaces: true
      creatures_can_pass_stone_space: true
      stone_space: difficult_terrain
      stops_at: wall_or_similar_barrier
      turns_corners: false
      curving_passages_allow_redirection: true
    collision:
      triggers:
      - stone_enters_creature_space_first_time_on_turn
      - creature_enters_rolling_stone_space
      save:
        ability: dexterity
        dc: 15
        failure:
          damage:
            dice:
              count: 10
              sides: 10
          average: 55
          type: bludgeoning
          condition: prone
        success: no_effect
    detect:
      action: study
      ability: intelligence
      skill: investigation
      dc: 15
      success: deduce_pressure_plate_function
    disarm:
      wedge_under_plate: iron_spike_or_similar
      effect: prevents_trigger
    slow:
      action_cost: 1
      ability: strength
      skill: athletics
      dc: 20
      success:
        movement_feet_per_turn_subtract: 15
        at_zero: stone_stops
  implementation: dnd2024:rule_traps_rolling_stone
splat_contracts:
- id: dnd2024:rule_traps_rolling_stone
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - effects
  events:
  - pressure_plate.entered
  - turn.started
  - stone.enters_creature_space
  - creature.enters_stone_space
  capabilities:
  - traps
  - objects
  - movement
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 201
  engine_gaps: &id001
  - dnd2024:rule_traps_rolling_stone
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: ff19bd3ba4215173460a3fd04461e96cfdd60d084c7f81a9f2861dc46f2066a2
  parent_name: Traps
  line_span:
    first: 10163
    count: 41
splat_clauses:
- key: source_rules
  text: '# Rolling Stone Deadly Trap (Levels 11–16) or Nuisance Trap (Levels 17–20) Trigger: A creature
    moves onto a pressure plate Duration: Until the stone stops rolling When a creature moves onto a hidden
    pressure plate, a 5-foot-radius orb of solid stone is released from a secret compartment and begins
    to roll. The stone and all creatures nearby roll Initiative; the stone gets a +8 bonus on its Initiative
    roll. On its turn, the stone moves 60 feet in one direction, changing course if redirected by an obstacle.
    The stone can move through creatures’ spaces, and creatures can move through the stone’s space, treating
    it as Difficult Terrain. Whenever the stone enters a creature’s space for the first time on a turn
    or a creature enters the stone’s space while the stone is rolling, that creature must succeed on a
    DC 15 Dexterity saving throw or take 55 (10d10) Bludgeoning damage and have the Prone condition. The
    stone stops when it hits a wall or similar barrier. It can’t go around corners, but creative dungeon
    builders incorporate curving turns into nearby passages that allow the stone to keep moving. Detect
    and Disarm. As a Study action, a creature can examine the section of floor that forms the pressure
    plate and make a DC 15 Intelligence (Investigation) check, deducing the pressure plate’s function
    on a successful check. Wedging an Iron Spike or a similar object under the pressure plate prevents
    the trap from triggering. Destroy the Stone. The stone is a Large object that has AC 17, HP 100, a
    Damage Threshold of 10, and Immunity to Poison and Psychic damage. Slow the Stone. As an action, a
    creature can try to slow down the stone with a DC 20 Strength (Athletics) check. If the check is successful,
    the distance the stone moves on its turn is reduced by 15 feet. If that distance drops to 0, it stops
    moving and is no longer a threat.'
  sha256: c5c33142f9de87b50e20e466ef9c385f1def36e173bac4643c41fbb5c620d949
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rolling Stone

<!-- SRD page 201 -->

Deadly Trap (Levels 11–16) or Nuisance Trap (Levels\
17–20)\
Trigger: A creature moves onto a pressure plate\
Duration: Until the stone stops rolling\
When a creature moves onto a hidden pressure\
plate, a 5-foot-radius orb of solid stone is released\
from a secret compartment and begins to roll. The\
stone and all creatures nearby roll Initiative; the\
stone gets a +8 bonus on its Initiative roll.\
On its turn, the stone moves 60 feet in one direc-\
tion, changing course if redirected by an obstacle.\
The stone can move through creatures’ spaces,\
and creatures can move through the stone’s space,\
treating it as Difficult Terrain. Whenever the stone\
enters a creature’s space for the first time on a turn\
or a creature enters the stone’s space while the\
stone is rolling, that creature must succeed on a DC\
15 Dexterity saving throw or take 55 (10d10) Blud-\
geoning damage and have the Prone condition.\
The stone stops when it hits a wall or similar\
barrier. It can’t go around corners, but creative\
dungeon builders incorporate curving turns into\
nearby passages that allow the stone to keep\
moving.\
Detect and Disarm. As a Study action, a creature\
can examine the section of floor that forms the\
pressure plate and make a DC 15 Intelligence (In-\
vestigation) check, deducing the pressure plate’s\
function on a successful check. Wedging an Iron\
Spike or a similar object under the pressure plate\
prevents the trap from triggering.\
Destroy the Stone. The stone is a Large object\
that has AC 17, HP 100, a Damage Threshold of 10,\
and Immunity to Poison and Psychic damage.\
Slow the Stone. As an action, a creature can try to\
slow down the stone with a DC 20 Strength (Athlet-\
ics) check. If the check is successful, the distance the\
stone moves on its turn is reduced by 15 feet. If that\
distance drops to 0, it stops moving and is no longer\
a threat.\
