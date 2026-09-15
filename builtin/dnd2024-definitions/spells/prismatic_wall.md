---
splat_schema: 1
splat_id: 2618e55d-2863-5233-8639-f3def3fded3d
splat_kind: spell
splat_key: dnd2024:spells/prismatic_wall
splat_name: Prismatic Wall
splat_origin:
  key: dnd2024:reference/spell/prismatic_wall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 155
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: abjuration
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/prismatic_wall
  implementation: dnd2024:spell_prismatic_wall
  parameters:
    geometry_choice:
      wall:
        vertical: true
        maximum_length_feet: 90
        maximum_height_feet: 30
        thickness_inches: 1
      globe:
        maximum_diameter_feet: 30
    geometry:
      center: point_in_range
      opaque: true
      creature_occupied_at_creation: instant_end_no_effect
    light:
      bright_radius_feet: 100
      additional_dim_feet: 100
    exempt:
      caster: true
      designated_creatures_at_cast: true
      near_and_pass_through_unharmed: true
    near_effect:
      triggers:
      - creature.moves_within_20_feet
      - turn.start
      can_see_wall_required: true
      save:
        ability: constitution
        dc:
          binding: save_dc
        failure:
          condition: blinded
          minutes: 1
        success:
          effect: none
    passage:
      triggers:
      - creature.reaches_into_wall
      - creature.passes_through_wall
      sequential_layers: true
      save_per_layer: dexterity
    destruction:
      ac: 10
      one_layer_at_time: true
      order:
      - red
      - orange
      - yellow
      - green
      - blue
      - indigo
      - violet
      destroyed_layer_absent_until_spell_end: true
      antimagic_field_effect: false
      dispel_magic_layers:
      - violet
    layers:
    - order: 1
      color: red
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 12
                sides: 6
                add: 0
            type: fire
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
      blocks: nonmagical_ranged_attacks
      destroy:
        damage_type: cold
        minimum_damage: 25
    - order: 2
      color: orange
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 12
                sides: 6
                add: 0
            type: acid
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
      blocks: magical_ranged_attacks
      destroy:
        effect: strong_wind
    - order: 3
      color: yellow
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 12
                sides: 6
                add: 0
            type: lightning
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
      destroy:
        damage_type: force
        minimum_damage: 60
    - order: 4
      color: green
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 12
                sides: 6
                add: 0
            type: poison
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
      destroy:
        spell: passwall
        or_spell:
          minimum_level: 5
          effect: open_portal_on_solid_surface
    - order: 5
      color: blue
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          damage:
            amount:
              dice:
                count: 12
                sides: 6
                add: 0
            type: cold
        success:
          damage_fraction:
            numerator: 1
            denominator: 2
      destroy:
        damage_type: fire
        minimum_damage: 25
    - order: 6
      color: indigo
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          condition: restrained
          repeat_save:
            event: turn.end
            ability: constitution
            count_success_and_failure_separately: true
            consecutive_required: false
            successes_to_end_restrained: 3
            failures_to_petrify: 3
            petrified_until: freed_by_restoring_effect
        success:
          effect: none
      blocks: spells_cast_through
      destroy:
        effect: bright_light
        spell: daylight
    - order: 7
      color: violet
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          condition: blinded
          repeat_save:
            event: caster.next_turn.start
            ability: wisdom
            success:
              end_blinded: true
            failure:
              end_blinded: true
              teleport:
                destination: other_plane
                chooser: gm
        success:
          effect: none
      destroy:
        spell: dispel_magic
splat_contracts:
- id: dnd2024:spell_prismatic_wall
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
    spell_attack_modifier:
      required: false
      type: integer
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
  - wall_layers
  - saves
  - damage
  - conditions
  - planar_transition
  - destroyed_layers
  events:
  - spell.cast_completed
  - turn.start
  - turn.end
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - attacks
  - conditions
  - spatial_areas
  - movement
  - perception
  - choices
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6a2938d3e6e4be83cbdab37cfd8df25b1b0456ae29b0ce3fc904cde2cedbd37e
  source_path: builtin/srd-5.2.1/spell/prismatic_wall.md
  pages:
  - 155
  engine_gaps: &id001
  - dnd2024:spell_prismatic_wall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 4acc90146cd7f3269912d9d541bf9d2c8083521250f784dc65921d056a3dce40
splat_clauses:
- key: source_rules
  text: '# Prismatic Wall Level 9 Abjuration (Bard, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S Duration: 10 minutes A shimmering, multicolored plane of light forms a vertical opaque wall—up
    to 90 feet long, 30 feet high, and 1 inch thick—centered on a point within range. Alternatively, you
    shape the wall into a globe up to 30 feet in diameter centered on a point within range. The wall lasts
    for the duration. If you position the wall in a space occupied by a creature, the spell ends instantly
    without effect. The wall sheds Bright Light within 100 feet and Dim Light for an additional 100 feet.
    You and creatures you designate when you cast the spell can pass through and be near the wall without
    harm. If another creature that can see the wall moves within 20 feet of it or starts its turn there,
    the creature must succeed on a Constitution saving throw or have the Blinded condition for 1 minute.
    The wall consists of seven layers, each with a different color. When a creature reaches into or passes
    through the wall, it does so one layer at a time through all the layers. Each layer forces the creature
    to make a Dexterity saving throw or be affected by that layer’s properties as described in the Prismatic
    Layers table. The wall, which has AC 10, can be destroyed one layer at a time, in order from red to
    violet, by means specific to each layer. If a layer is destroyed, it is gone for the duration. Antimagic
    Field has no effect on the wall, and Dispel Magic can affect only the violet layer. Prismatic Layers
    Order Effects 1 Red. Failed Save: 12d6 Fire damage. Successful Save: Half as much damage. Additional
    Effects: Nonmagical ranged attacks can’t pass through this layer, which is destroyed if it takes at
    least 25 Cold damage. 2 Orange. Failed Save: 12d6 Acid damage. Successful Save: Half as much damage.
    Additional Effects: Magical ranged attacks can’t pass through this layer, which is destroyed by a
    strong wind (such as the one created by Gust of Wind). 3 Yellow. Failed Save: 12d6 Lightning damage.
    Successful Save: Half as much damage. Additional Effects: The layer is destroyed if it takes at least
    60 Force damage. 4 Green. Failed Save: 12d6 Poison damage. Successful Save: Half as much damage. Additional
    Effects: A Passwall spell, or another spell of equal or greater level that can open a portal on a
    solid surface, destroys this layer. 5 Blue. Failed Save: 12d6 Cold damage. Successful Save: Half as
    much damage. Additional Effects: The layer is destroyed if it takes at least 25 Fire damage. 6 Indigo.
    Failed Save: The target has the Restrained condition and makes a Constitution saving throw at the
    end of each of its turns. If it successfully saves three times, the condition ends. If it fails three
    times, it has the Petrified condition until it is freed by an effect like the Greater Restoration
    spell. The successes and failures needn’t be consecutive; keep track of both until the target collects
    three of a kind. Additional Effects: Spells can’t be cast through this layer, which is destroyed by
    Bright Light shed by the Daylight spell. 7 Violet. Failed Save: The target has the Blinded condition
    and makes a Wisdom saving throw at the start of your next turn. On a successful save, the condition
    ends. On a failed save, the condition ends, and the creature teleports to another plane of existence
    (GM’s choice). Additional Effects: This layer is destroyed by Dispel Magic.'
  sha256: 4d0b20a59c73c0b0f5538918b22b2557a4c926e6d512bfafa01759c72d91fb26
  status: contract_specified
splat_requires_mechanics: *id001
---

# Prismatic Wall

<!-- SRD page 155 -->

Level 9 Abjuration (Bard, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: 10 minutes\
A shimmering, multicolored plane of light forms\
a vertical opaque wall—up to 90 feet long, 30 feet\
high, and 1 inch thick—centered on a point within\
range. Alternatively, you shape the wall into a globe\
up to 30 feet in diameter centered on a point within\
range. The wall lasts for the duration. If you posi-\
tion the wall in a space occupied by a creature, the\
spell ends instantly without effect.\
The wall sheds Bright Light within 100 feet and\
Dim Light for an additional 100 feet. You and crea-\
tures you designate when you cast the spell can\
pass through and be near the wall without harm. If\
another creature that can see the wall moves within\
20 feet of it or starts its turn there, the creature\
must succeed on a Constitution saving throw or\
have the Blinded condition for 1 minute.\
The wall consists of seven layers, each with a\
different color. When a creature reaches into or\
passes through the wall, it does so one layer at a\
time through all the layers. Each layer forces the\
creature to make a Dexterity saving throw or be af-\
fected by that layer’s properties as described in the\
Prismatic Layers table.\
The wall, which has AC 10, can be destroyed one\
layer at a time, in order from red to violet, by means\
specific to each layer. If a layer is destroyed, it is\
gone for the duration. Antimagic Field has no effect\
on the wall, and Dispel Magic can affect only the vio-\
let layer.\
Prismatic Layers\
Order Effects\
1 Red. Failed Save: 12d6 Fire damage. Successful\
Save: Half as much damage. Additional Effects:\
Nonmagical ranged attacks can’t pass through\
this layer, which is destroyed if it takes at\
least 25 Cold damage.\
2 Orange. Failed Save: 12d6 Acid damage. Suc-\
cessful Save: Half as much damage. Additional\
Effects: Magical ranged attacks can’t pass\
through this layer, which is destroyed by a\
strong wind (such as the one created by Gust\
of Wind).\
3 Yellow. Failed Save: 12d6 Lightning damage.\
Successful Save: Half as much damage. Addi-\
tional Effects: The layer is destroyed if it takes\
at least 60 Force damage.\
4 Green. Failed Save: 12d6 Poison damage. Suc-\
cessful Save: Half as much damage. Additional\
Effects: A Passwall spell, or another spell of\
equal or greater level that can open a portal\
on a solid surface, destroys this layer.\
5 Blue. Failed Save: 12d6 Cold damage. Success-\
ful Save: Half as much damage. Additional Ef-\
fects: The layer is destroyed if it takes at least\
25 Fire damage.\
6 Indigo. Failed Save: The target has the Re-\
strained condition and makes a Constitution\
saving throw at the end of each of its turns. If\
it successfully saves three times, the condition\
ends. If it fails three times, it has the Petrified\
condition until it is freed by an effect like the\
Greater Restoration spell. The successes and\
failures needn’t be consecutive; keep track of\
both until the target collects three of a kind.\
Additional Effects: Spells can’t be cast through\
this layer, which is destroyed by Bright Light\
shed by the Daylight spell.\
7 Violet. Failed Save: The target has the Blinded\
condition and makes a Wisdom saving throw\
at the start of your next turn. On a successful\
save, the condition ends. On a failed save, the\
condition ends, and the creature teleports\
to another plane of existence (GM’s choice).\
Additional Effects: This layer is destroyed by\
Dispel Magic.\

Original layout: [[../../srd-5.2.1/pages/155|page 155]].
