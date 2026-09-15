---
splat_schema: 1
splat_id: f189d34d-37cc-5a3a-8c99-504e524dbfd3
splat_kind: spell
splat_key: dnd2024:spells/wall_of_thorns
splat_name: Wall of Thorns
splat_origin:
  key: dnd2024:reference/spell/wall_of_thorns
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a handful of thorns
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/wall_of_thorns
  implementation: dnd2024:spell_wall_of_thorns
  parameters:
    surface: solid
    blocks_line_of_sight: true
    geometry_choice:
      wall:
        maximum_length_feet: 60
        maximum_height_feet: 10
        maximum_thickness_feet: 5
      circle:
        diameter_feet: 20
        maximum_height_feet: 20
        maximum_thickness_feet: 5
    appearance_effect:
      event: spell.cast_completed
      targets: creatures_in_wall
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 7
                sides: 8
                add: 0
            type: piercing
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
    movement:
      per_foot_cost_feet: 4
    later_effect:
      triggers:
      - area.first_entry_on_turn
      - turn.end
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 7
                sides: 8
                add: 0
            type: slashing
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
      maximum_saves_per_creature_per_turn: 1
    scaling:
      per_slot_above: 6
      piercing_dice: 1
      slashing_dice: 1
      sides: 8
splat_contracts:
- id: dnd2024:spell_wall_of_thorns
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - wall_effect
  - saves
  - damage
  - movement_cost
  events:
  - spell.cast_completed
  - turn.end
  - area.enter
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - spatial_areas
  - movement
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7e5a240c6e2b61e76e272d32e7f50de41e6e51b044dd7c6f97f6caf6c2f4af0c
  source_path: builtin/srd-5.2.1/spell/wall_of_thorns.md
  pages:
  - 173
  engine_gaps: &id001
  - dnd2024:spell_wall_of_thorns
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: ffa08868bf2405a7001e3429f528d09ab3e29b7a4fef23528cc7ed1df20320db
splat_clauses:
- key: source_rules
  text: '# Wall of Thorns Level 6 Conjuration (Druid) Casting Time: Action Range: 120 feet Components:
    V, S, M (a handful of thorns) Duration: Concentration, up to 10 minutes You create a wall of tangled
    brush bristling with needle-sharp thorns. The wall appears within range on a solid surface and lasts
    for the duration. You choose to make the wall up to 60 feet long, 10 feet high, and 5 feet thick or
    a circle that has a 20-foot diameter and is up to 20 feet high and 5 feet thick. The wall blocks line
    of sight. When the wall appears, each creature in its area makes a Dexterity saving throw, taking
    7d8 Piercing damage on a failed save or half as much damage on a successful one. A creature can move
    through the wall, albeit slowly and painfully. For every 1 foot a creature moves through the wall,
    it must spend 4 feet of movement. Furthermore, the first time a creature enters a space in the wall
    on a turn or ends its turn there, the creature makes a Dexterity saving throw, taking 7d8 Slashing
    damage on a failed save or half as much damage on a successful one. A creature makes this save only
    once per turn. Using a Higher-Level Spell Slot. Both types of damage increase by 1d8 for each spell
    slot level above 6.'
  sha256: c35bc0d75cd4b190e0182db06b8ecf471f293a28ed0b8e80ce20cfec917d8f05
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wall of Thorns

<!-- SRD page 173 -->

Level 6 Conjuration (Druid)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a handful of thorns)\
Duration: Concentration, up to 10 minutes\
You create a wall of tangled brush bristling with\
needle-sharp thorns. The wall appears within range\
on a solid surface and lasts for the duration. You\
choose to make the wall up to 60 feet long, 10 feet\
high, and 5 feet thick or a circle that has a 20-foot\
diameter and is up to 20 feet high and 5 feet thick.\
The wall blocks line of sight.\
When the wall appears, each creature in its area\
makes a Dexterity saving throw, taking 7d8 Pierc-\
ing damage on a failed save or half as much damage\
on a successful one.\
A creature can move through the wall, albeit\
slowly and painfully. For every 1 foot a creature\
moves through the wall, it must spend 4 feet of\
movement. Furthermore, the first time a creature\
enters a space in the wall on a turn or ends its turn\
there, the creature makes a Dexterity saving throw,\
taking 7d8 Slashing damage on a failed save or half\
as much damage on a successful one. A creature\
makes this save only once per turn.\
Using a Higher-Level Spell Slot. Both types of\
damage increase by 1d8 for each spell slot level\
above 6.\

Original layout: [[../../srd-5.2.1/pages/173|page 173]].
