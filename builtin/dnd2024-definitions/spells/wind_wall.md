---
splat_schema: 1
splat_id: 051910fb-59e0-55da-9b69-21b335f12d31
splat_kind: spell
splat_key: dnd2024:spells/wind_wall
splat_name: Wind Wall
splat_origin:
  key: dnd2024:reference/spell/wind_wall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 174
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: evocation
  classes:
  - druid
  - ranger
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
    description: a fan and a feather
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/wind_wall
  implementation: dnd2024:spell_wind_wall
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    wall:
      origin: chosen_point_on_ground
      range_feet: 120
      maximum_length_feet: 50
      maximum_height_feet: 15
      shape: caster_choice
      continuous_path_on_ground: true
      wind: strong
      maximum_thickness_feet: 1
    initial:
      targets: all_creatures_in_area
      save:
        ability: strength
        dc: spell_save_dc
      failure_damage:
        dice: 4d8
        type: bludgeoning
      success_damage_scale:
        numerator: 1
        denominator: 2
    blocked:
      gases:
      - fog
      - smoke
      - other_gases
      flying_creatures_and_objects_maximum_size: small
      gaseous_form_creatures: true
    loose_lightweight_material:
      on_enter: fly_upward
    projectiles:
      ordinary:
        examples:
        - arrows
        - bolts
        target_behind_wall:
          deflect: upward
          attack_result: automatic_miss
      exceptions_unaffected:
      - giant_hurled_boulders
      - siege_engine_projectiles
      - similar_projectiles
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_wind_wall
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
    wall_design:
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
  source_sha256: d10ceee9749533218b7a06804fac223d26ae938dbcc5e35546e0148becbab2bf
  source_path: builtin/srd-5.2.1/spell/wind_wall.md
  pages:
  - 174
  engine_gaps: &id001
  - dnd2024:spell_wind_wall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: bce41389326ae70de563a58fe610e772f478fb042ac5730ee1991f013f610f31
splat_clauses:
- key: source_rules
  text: '# Wind Wall Level 3 Evocation (Druid, Ranger) Casting Time: Action Range: 120 feet Components:
    V, S, M (a fan and a feather) Duration: Concentration, up to 1 minute A wall of strong wind rises
    from the ground at a point you choose within range. You can make the wall up to 50 feet long, 15 feet
    high, and 1 foot thick. You can shape the wall in any way you choose so long as it makes one continuous
    path along the ground. The wall lasts for the duration. When the wall appears, each creature in its
    area makes a Strength saving throw, taking 4d8 Bludgeoning damage on a failed save or half as much
    damage on a successful one. The strong wind keeps fog, smoke, and other gases at bay. Small or smaller
    flying creatures or objects can’t pass through the wall. Loose, lightweight materials brought into
    the wall fly upward. Arrows, bolts, and other ordinary projectiles launched at targets behind the
    wall are deflected upward and miss automatically. Boulders hurled by Giants or siege engines, and
    similar projectiles, are unaffected. Creatures in gaseous form can’t pass through it.'
  sha256: 4d7be98e1662e65cf56ab3b3b97d6fd2c9315ba4c690c6d593d667857f3c28d5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wind Wall

<!-- SRD page 174 -->

Level 3 Evocation (Druid, Ranger)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a fan and a feather)\
Duration: Concentration, up to 1 minute\
A wall of strong wind rises from the ground at a\
point you choose within range. You can make the\
wall up to 50 feet long, 15 feet high, and 1 foot\
thick. You can shape the wall in any way you choose\
so long as it makes one continuous path along the\
ground. The wall lasts for the duration.\
When the wall appears, each creature in its area\
makes a Strength saving throw, taking 4d8 Blud-\
geoning damage on a failed save or half as much\
damage on a successful one.\
The strong wind keeps fog, smoke, and other\
gases at bay. Small or smaller flying creatures or\
objects can’t pass through the wall. Loose, light-\
weight materials brought into the wall fly upward.\
Arrows, bolts, and other ordinary projectiles\
launched at targets behind the wall are deflected\
upward and miss automatically. Boulders hurled by\
Giants or siege engines, and similar projectiles, are\
unaffected. Creatures in gaseous form can’t pass\
through it.\

Original layout: [[../../srd-5.2.1/pages/174|page 174]].
