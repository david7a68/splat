---
splat_schema: 1
splat_id: 7ef9958a-2469-5e21-ae1b-eb9291455538
splat_kind: spell
splat_key: dnd2024:spells/wall_of_fire
splat_name: Wall of Fire
splat_origin:
  key: dnd2024:reference/spell/wall_of_fire
splat_sources:
- title: SRD 5.2.1
  locator: pp. 172
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: evocation
  classes:
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a piece of charcoal
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/wall_of_fire
  implementation: dnd2024:spell_wall_of_fire
  parameters:
    surface: solid
    opaque: true
    geometry_choice:
      wall:
        maximum_length_feet: 60
        maximum_height_feet: 20
        thickness_feet: 1
      ring:
        maximum_diameter_feet: 20
        maximum_height_feet: 20
        thickness_feet: 1
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
                count: 5
                sides: 8
                add: 0
            type: fire
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
    hot_side:
      chooser: caster
      chosen_at: cast
      damage:
        amount:
          dice:
            count: 5
            sides: 8
            add: 0
        type: fire
      event: turn.end
      eligible:
      - within_10_feet_of_hot_side
      - inside_wall
    wall_damage:
      events:
      - area.first_entry_on_turn
      - turn.end
      inside_wall: true
      damage:
        amount:
          dice:
            count: 5
            sides: 8
            add: 0
        type: fire
      save: false
      same_end_turn_occurrence_not_duplicated: true
    cold_side_damage: 0
    scaling:
      per_slot_above: 4
      damage_dice: 1
      sides: 8
splat_contracts:
- id: dnd2024:spell_wall_of_fire
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
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 779875d416c463fb33a32ae379bc2667b11a1cc9d8a1ef18449e7380dbbfb143
  source_path: builtin/srd-5.2.1/spell/wall_of_fire.md
  pages:
  - 172
  engine_gaps: &id001
  - dnd2024:spell_wall_of_fire
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 21027448c94e188fda65d1ab7fc110436add558e2e20fc0f01bdcaef6a651496
splat_clauses:
- key: source_rules
  text: '# Wall of Fire Level 4 Evocation (Druid, Sorcerer, Wizard) Casting Time: Action Range: 120 feet
    Components: V, S, M (a piece of charcoal) Duration: Concentration, up to 1 minute You create a wall
    of fire on a solid surface within range. You can make the wall up to 60 feet long, 20 feet high, and
    1 foot thick, or a ringed wall up to 20 feet in diameter, 20 feet high, and 1 foot thick. The wall
    is opaque and lasts for the duration. When the wall appears, each creature in its area makes a Dexterity
    saving throw, taking 5d8 Fire damage on a failed save or half as much damage on a successful one.
    One side of the wall, selected by you when you cast this spell, deals 5d8 Fire damage to each creature
    that ends its turn within 10 feet of that side or inside the wall. A creature takes the same damage
    when it enters the wall for the first time on a turn or ends its turn there. The other side of the
    wall deals no damage. Using a Higher-Level Spell Slot. The damage increases by 1d8 for each spell
    slot level above 4.'
  sha256: b67622dd2495dc2c966a9381b22a9d12da08ad562f3472bf995ae1f797c579b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wall of Fire

<!-- SRD page 172 -->

Level 4 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a piece of charcoal)\
Duration: Concentration, up to 1 minute\
You create a wall of fire on a solid surface within\
range. You can make the wall up to 60 feet long, 20\
feet high, and 1 foot thick, or a ringed wall up to 20\
feet in diameter, 20 feet high, and 1 foot thick. The\
wall is opaque and lasts for the duration.\
When the wall appears, each creature in its area\
makes a Dexterity saving throw, taking 5d8 Fire\
damage on a failed save or half as much damage on\
a successful one.\
One side of the wall, selected by you when you\
cast this spell, deals 5d8 Fire damage to each crea-\
ture that ends its turn within 10 feet of that side or\
inside the wall. A creature takes the same damage\
when it enters the wall for the first time on a turn or\
ends its turn there. The other side of the wall deals\
no damage.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 4.\

Original layout: [[../../srd-5.2.1/pages/172|page 172]].
