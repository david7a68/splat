---
splat_schema: 1
splat_id: 53d7b4b0-dab8-5eb6-9392-d8d81d0d818f
splat_kind: rule
splat_key: dnd2024:rules/traps_spiked_pit
splat_name: Spiked Pit
splat_origin:
  key: dnd2024:reference/pages/201
splat_sources:
- title: SRD 5.2.1
  locator: pp. 201
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/traps_spiked_pit
  parameters:
    trap:
      severity: deadly
      levels:
      - 1
      - 4
      trigger: creature_moves_onto_lid
      duration: instantaneous
    lid:
      hinged: true
      hidden_as_floor: true
      remains_open_after_trigger: true
    trigger_effect: fall_to_pit_bottom
    depth_feet: 10
    damage:
    - type: bludgeoning
      dice:
        count: 1
        sides: 6
        add: 0
      average: 3
    - type: piercing
      dice:
        count: 2
        sides: 8
        add: 0
      average: 9
    detection:
      economy: study
      check:
        ability: intelligence
        dc: 15
        skill: investigation
      target: lid_floor_section
    disarm:
      after_detection:
      - wedge_lid_with_iron_spike_or_similar
      - hold_lid_closed_with_arcane_lock_or_similar_magic
    escape:
      requires_any:
      - climb_speed
      - climbing_gear
      - wall_climbing_magic
      gm_geometry_option: add_handholds_and_footholds
    scaling:
    - levels:
      - 1
      - 4
      depth_feet: 10
      damage:
      - type: bludgeoning
        dice:
          count: 1
          sides: 6
          add: 0
        average: 3
      - type: piercing
        dice:
          count: 2
          sides: 8
          add: 0
        average: 9
    - levels:
      - 5
      - 10
      depth_feet: 30
      damage:
      - type: bludgeoning
        dice:
          count: 3
          sides: 6
          add: 0
        average: 10
      - type: piercing
        dice:
          count: 3
          sides: 8
          add: 0
        average: 13
    - levels:
      - 11
      - 16
      depth_feet: 60
      damage:
      - type: bludgeoning
        dice:
          count: 6
          sides: 6
          add: 0
        average: 21
      - type: piercing
        dice:
          count: 8
          sides: 8
          add: 0
        average: 36
    - levels:
      - 17
      - 20
      depth_feet: 120
      damage:
      - type: bludgeoning
        dice:
          count: 12
          sides: 6
          add: 0
        average: 42
      - type: piercing
        dice:
          count: 13
          sides: 8
          add: 0
        average: 57
  implementation: dnd2024:rules_traps_spiked_pit
splat_contracts:
- id: dnd2024:rules_traps_spiked_pit
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
  - 201
  engine_gaps: &id001
  - dnd2024:rules_traps_spiked_pit
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 8dfdfb506d9f65097d1ff95f9b6d80fd1f246503f751ecdca116bd7f89a9a9ff
  parent_name: Traps
  line_span:
    first: 10204
    count: 39
splat_clauses:
- key: source_rules
  text: '# Spiked Pit Deadly Trap (Levels 1–4) Trigger: A creature moves onto the pit’s lid Duration:
    Instantaneous A 10-foot-deep pit has a hinged lid constructed from material identical to the surrounding
    floor. When a creature moves onto the lid, it swings open like a trapdoor, causing the creature to
    fall into the pit, which has sharpened wooden or metal spikes at the bottom. The lid remains open
    thereafter. A creature that falls into the pit lands at the bottom and takes 3 (1d6) Bludgeoning damage
    from the fall plus 9 (2d8) Piercing damage from the spikes. Detect and Disarm. As a Study action,
    a creature can examine the section of floor that forms the pit’s lid and make a DC 15 Intelligence
    (Investigation) check, detecting the pit on a successful check. Once the hidden pit is detected, an
    Iron Spike or a similar object can be wedged between the pit’s lid and the surrounding floor to prevent
    the lid from opening, thereby making it safe to cross. The cover can also be held shut using an Arcane
    Lock spell or similar magic. Escape. A creature needs a Climb Speed, climbing gear, or magic such
    as a Spider Climb spell to scale the pit’s smooth walls. You can make the pit easier to escape by
    adding cracks in the walls big enough to serve as handholds and footholds. At Higher Levels. You can
    scale the trap for higher levels by increasing the pit’s depth and damage, as shown in the following
    table. Levels Pit Depth Damage 5–10 30 feet 10 (3d6) Bludgeoning plus 13 (3d8) Piercing 11–16 60 feet
    21 (6d6) Bludgeoning plus 36 (8d8) Piercing 17–20 120 feet 42 (12d6) Bludgeoning plus 57 (13d8) Piercing'
  sha256: 034214d1df8999b35c73ce75c74b6145d022a8e974f678dca69d8b07b616cd8a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spiked Pit

<!-- SRD page 201 -->

Deadly Trap (Levels 1–4)\
Trigger: A creature moves onto the pit’s lid\
Duration: Instantaneous\
A 10-foot-deep pit has a hinged lid constructed from\
material identical to the surrounding floor. When\
a creature moves onto the lid, it swings open like\
a trapdoor, causing the creature to fall into the pit,\
which has sharpened wooden or metal spikes at the\
bottom. The lid remains open thereafter.\
A creature that falls into the pit lands at the\
bottom and takes 3 (1d6) Bludgeoning damage\
from the fall plus 9 (2d8) Piercing damage from\
the spikes.\
Detect and Disarm. As a Study action, a creature\
can examine the section of floor that forms the pit’s\
lid and make a DC 15 Intelligence (Investigation)\
check, detecting the pit on a successful check. Once\
the hidden pit is detected, an Iron Spike or a similar\
object can be wedged between the pit’s lid and the\
surrounding floor to prevent the lid from opening,\
thereby making it safe to cross. The cover can also\
be held shut using an Arcane Lock spell or similar\
magic.\
Escape. A creature needs a Climb Speed, climbing\
gear, or magic such as a Spider Climb spell to scale\
the pit’s smooth walls. You can make the pit easier\
to escape by adding cracks in the walls big enough\
to serve as handholds and footholds.\
At Higher Levels. You can scale the trap for higher\
levels by increasing the pit’s depth and damage, as\
shown in the following table.\
Levels Pit Depth Damage\
5–10 30 feet 10 (3d6) Bludgeoning plus 13\
(3d8) Piercing\
11–16 60 feet 21 (6d6) Bludgeoning plus 36\
(8d8) Piercing\
17–20 120 feet 42 (12d6) Bludgeoning plus 57\
(13d8) Piercing\
