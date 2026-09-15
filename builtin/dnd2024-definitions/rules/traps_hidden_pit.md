---
splat_schema: 1
splat_id: db6c7e4a-fec4-5e35-92da-3ba0bfc1d570
splat_kind: rule
splat_key: dnd2024:rules/traps_hidden_pit
splat_name: Hidden Pit
splat_origin:
  key: dnd2024:reference/pages/200
splat_sources:
- title: SRD 5.2.1
  locator: pp. 200
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/traps_hidden_pit
  parameters:
    severity: nuisance
    default_levels:
    - 1
    - 4
    trigger: creature_moves_onto_lid
    duration: instantaneous
    pit_depth_feet: 10
    trigger_effect:
      lid: opens_and_remains_open
      creature: falls_into_pit
      damage:
        dice:
          count: 1
          sides: 6
      average: 3
      damage_type: bludgeoning
    detect:
      action: study
      ability: intelligence
      skill: investigation
      dc: 15
      success: detect_pit
    disarm:
      requires: detected
      options:
      - wedge: iron_spike_or_similar
        result: lid_cannot_open
      - magic:
          key: dnd2024:spells/arcane_lock
        result: lid_held_shut
      crossing_after_disarm: safe
    escape:
      wall: smooth
      requires_any:
      - climb_speed
      - climbing_gear
      - spell:
          key: dnd2024:spells/spider_climb
      - similar_magic
      gm_easier_variant: cracks_for_handholds_and_footholds
    higher_level_variants:
    - levels:
      - 5
      - 10
      depth_feet: 30
      damage:
        dice:
          count: 3
          sides: 6
      average: 10
    - levels:
      - 11
      - 16
      depth_feet: 60
      damage:
        dice:
          count: 6
          sides: 6
      average: 21
    - levels:
      - 17
      - 20
      depth_feet: 120
      damage:
        dice:
          count: 12
          sides: 6
      average: 42
  implementation: dnd2024:rule_traps_hidden_pit
splat_contracts:
- id: dnd2024:rule_traps_hidden_pit
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
  - creature.enters_lid
  capabilities:
  - traps
  - damage
  - movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 200
  engine_gaps: &id001
  - dnd2024:rule_traps_hidden_pit
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 55cf5501187c034cdc991bb9feb59727899f898ca21c845b7180b126eb465c63
  parent_name: Traps
  line_span:
    first: 10071
    count: 32
splat_clauses:
- key: source_rules
  text: '# Hidden Pit Nuisance Trap (Levels 1–4) Trigger: A creature moves onto the pit’s lid Duration:
    Instantaneous This 10-foot-deep pit has a hinged lid constructed from material identical to the surrounding
    floor. When a creature moves onto the lid, it swings open like a trapdoor, causing the creature to
    fall into the pit. The lid remains open thereafter. A creature that falls into the pit takes 3 (1d6)
    Bludgeoning damage from the fall. Detect and Disarm. As a Study action, a creature can examine the
    section of floor that forms the pit’s lid and make a DC 15 Intelligence (Investigation) check, detecting
    the pit on a successful check. Once the pit is detected, an Iron Spike or a similar object can be
    wedged between the pit’s lid and the surrounding floor to prevent the lid from opening, thereby making
    it safe to cross. The cover can also be held shut using Arcane Lock or similar magic. Escape. A creature
    needs a Climb Speed, climbing gear, or magic such as Spider Climb to scale the pit’s smooth walls.
    You can make the pit easier to escape by adding cracks in the walls big enough to serve as handholds
    and footholds. At Higher Levels. You can scale the trap for higher levels by increasing the pit’s
    depth and damage, as shown in the following table. Levels Pit Depth Bludgeoning Damage 5–10 30 feet
    10 (3d6) 11–16 60 feet 21 (6d6) 17–20 120 feet 42 (12d6)'
  sha256: 8afc515f8d3b4accbb978376b8138cde5de118496d9978f4a0052c525d081862
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hidden Pit

<!-- SRD page 200 -->

Nuisance Trap (Levels 1–4)\
Trigger: A creature moves onto the pit’s lid\
Duration: Instantaneous\
This 10-foot-deep pit has a hinged lid constructed\
from material identical to the surrounding floor.\
When a creature moves onto the lid, it swings open\
like a trapdoor, causing the creature to fall into the\
pit. The lid remains open thereafter.\
A creature that falls into the pit takes 3 (1d6)\
Bludgeoning damage from the fall.\
Detect and Disarm. As a Study action, a creature\
can examine the section of floor that forms the\
pit’s lid and make a DC 15 Intelligence (Investiga-\
tion) check, detecting the pit on a successful check.\
Once the pit is detected, an Iron Spike or a similar\
object can be wedged between the pit’s lid and the\
surrounding floor to prevent the lid from opening,\
thereby making it safe to cross. The cover can also\
be held shut using Arcane Lock or similar magic.\
Escape. A creature needs a Climb Speed, climbing\
gear, or magic such as Spider Climb to scale the pit’s\
smooth walls. You can make the pit easier to escape\
by adding cracks in the walls big enough to serve as\
handholds and footholds.\
At Higher Levels. You can scale the trap for higher\
levels by increasing the pit’s depth and damage, as\
shown in the following table.\
Levels Pit Depth Bludgeoning Damage\
5–10 30 feet 10 (3d6)\
11–16 60 feet 21 (6d6)\
17–20 120 feet 42 (12d6)\
