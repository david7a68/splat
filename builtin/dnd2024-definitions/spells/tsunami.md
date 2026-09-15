---
splat_schema: 1
splat_id: e1dd804f-df5d-58a0-ab02-c00e841c8ce3
splat_kind: spell
splat_key: dnd2024:spells/tsunami
splat_name: Tsunami
splat_origin:
  key: dnd2024:reference/spell/tsunami
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 6 rounds
  casting_time: 1 minute
  ritual: false
  range:
    mode: 1 mile
splat_rule:
  mechanic: dnd2024:spells/tsunami
  implementation: dnd2024:spell_tsunami
  parameters:
    spell_lifetime:
      concentration: true
      maximum_rounds: 6
    wall:
      origin: chosen_point
      range_miles: 1
      maximum_length_feet: 300
      maximum_height_feet: 300
      maximum_duration_rounds: 6
      maximum_thickness_feet: 50
    initial:
      targets: all_creatures_in_area
      save:
        ability: strength
        dc: spell_save_dc
      failure_damage:
        dice: 6d10
        type: bludgeoning
      success_damage_scale:
        numerator: 1
        denominator: 2
    movement:
      event: turn.start
      subject: caster
      only_turns_after_wall_appeared: true
      distance_feet: 50
      direction: away_from_caster
      carry: creatures_inside
    moving_wall_damage:
      targets:
        kinds: creature
        maximum_size: huge
        positions:
        - inside_wall
        - space_entered_by_wall
      save:
        ability: strength
        dc: spell_save_dc
        success: no_damage
      failure_damage:
        initial_dice: 5d10
        type: bludgeoning
        later_round_dice_decrement: 1
      damage_limit:
        count: 1
        per: round
        scope: creature
    decay:
      event: turn.end
      subject: caster
      sequence: after_wall_movement_turn
      height_decrease_feet: 50
      future_damage_decrease_d10: 1
      height_zero: end_spell
    trapped_movement:
      method: swimming
      check:
        ability: strength
        skill: athletics
        dc: spell_save_dc
      success: may_move
      failure: cannot_move
      exit_wall: fall_to_ground
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_tsunami
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    wall_dimensions:
      type: choice
      required: true
    targets:
      type: participant_list
      required: true
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  events:
  - concentration.ended
  - spell.cast_completed
  - turn.end
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - movement
  - areas
  - damage
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 768941e57c87066f3085c1b71efee6c54f1c427f426a89c199d2d2bbc72e148e
  source_path: builtin/srd-5.2.1/spell/tsunami.md
  pages:
  - 171
  engine_gaps: &id001
  - dnd2024:spell_tsunami
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: cf9493a951d587739d09db3c9f61636124021c28b8e62c01f67600c79c7c84e9
splat_clauses:
- key: source_rules
  text: '# Tsunami Level 8 Conjuration (Druid) Casting Time: 1 minute Range: 1 mile Components: V, S Duration:
    Concentration, up to 6 rounds A wall of water springs into existence at a point you choose within
    range. You can make the wall up to 300 feet long, 300 feet high, and 50 feet thick. The wall lasts
    for the duration. When the wall appears, each creature in its area makes a Strength saving throw,
    taking 6d10 Bludgeoning damage on a failed save or half as much damage on a successful one. At the
    start of each of your turns after the wall appears, the wall, along with any creatures in it, moves
    50 feet away from you. Any Huge or smaller creature inside the wall or whose space the wall enters
    when it moves must succeed on a Strength saving throw or take 5d10 Bludgeoning damage. A creature
    can take this damage only once per round. At the end of the turn, the wall’s height is reduced by
    50 feet, and the damage the wall deals on later rounds is reduced by 1d10. When the wall reaches 0
    feet in height, the spell ends. A creature caught in the wall can move by swimming. Because of the
    wave’s force, though, the creature must succeed on a Strength (Athletics) check against your spell
    save DC to move at all. If it fails the check, it can’t move. A creature that moves out of the wall
    falls to the ground.'
  sha256: 78fa46551b66045ea80a64fbc1f27eda0b262344abed5cb22773536044346c6b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tsunami

<!-- SRD page 171 -->

Level 8 Conjuration (Druid)\
Casting Time: 1 minute\
Range: 1 mile\
Components: V, S\
Duration: Concentration, up to 6 rounds\
A wall of water springs into existence at a point you\
choose within range. You can make the wall up to\
300 feet long, 300 feet high, and 50 feet thick. The\
wall lasts for the duration.\
When the wall appears, each creature in its area\
makes a Strength saving throw, taking 6d10 Blud-\
geoning damage on a failed save or half as much\
damage on a successful one.\
At the start of each of your turns after the wall\
appears, the wall, along with any creatures in it,\
moves 50 feet away from you. Any Huge or smaller\
creature inside the wall or whose space the wall\
enters when it moves must succeed on a Strength\
saving throw or take 5d10 Bludgeoning damage. A\
creature can take this damage only once per round.\
At the end of the turn, the wall’s height is reduced\
by 50 feet, and the damage the wall deals on later\
rounds is reduced by 1d10. When the wall reaches 0\
feet in height, the spell ends.\
A creature caught in the wall can move by swim-\
ming. Because of the wave’s force, though, the crea-\
ture must succeed on a Strength (Athletics) check\
against your spell save DC to move at all. If it fails\
the check, it can’t move. A creature that moves out\
of the wall falls to the ground.\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].
