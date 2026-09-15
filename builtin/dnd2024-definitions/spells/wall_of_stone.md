---
splat_schema: 1
splat_id: 2c639407-c344-5139-94ff-f9af21495524
splat_kind: spell
splat_key: dnd2024:spells/wall_of_stone
splat_name: Wall of Stone
splat_origin:
  key: dnd2024:reference/spell/wall_of_stone
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
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
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a cube of granite
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/wall_of_stone
  implementation: dnd2024:spell_wall_of_stone
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    wall:
      material: stone
      magical: false
      origin: chosen_point
      range_feet: 120
      panel_count: 10
      panel_choice:
      - width_feet: 10
        height_feet: 10
        thickness_inches: 6
      - width_feet: 10
        height_feet: 20
        thickness_inches: 3
      every_panel_contiguous_with_another: true
    appearance_collision:
      creature_cut_by_wall:
        push_to_side: caster_choice
      enclosed_creature:
        save:
          ability: dexterity
          dc: spell_save_dc
        success:
          optional: true
          economy: reaction
          movement_max: target_speed
          destination: no_longer_enclosed
    geometry:
      shape: caster_choice
      overlap_creature_or_object: false
      vertical_required: false
      firm_foundation_required: false
      must_merge_with_existing_stone: true
      solid_existing_stone_support_required: true
      span_greater_than_feet:
        threshold: 20
        panel_size_scale:
          numerator: 1
          denominator: 2
        purpose: supports
      crude_features_allowed: true
    panels:
      armor_class: 15
      hit_points_per_thickness_inch: 30
      damage_immunity:
      - poison
      - psychic
      at_zero_hit_points: destroyed
      connected_panel_collapse:
        chooser: gm
        selection: connected_panels
    duration:
      full_concentration_seconds: 600
      full_concentration_result:
        permanent: true
        dispellable: false
      earlier_spell_end: wall_disappears
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_wall_of_stone
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
    panel_design:
      type: choice
      required: true
    collision_sides:
      type: choice
      required: true
    collapse_choice:
      required: false
      type: choice
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - objects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - world_objects
  - areas
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2535614729f802bebb9fbfa545e726e8f07438eea9c6f89b8e41f2c1991dd34d
  source_path: builtin/srd-5.2.1/spell/wall_of_stone.md
  pages:
  - 173
  engine_gaps: &id001
  - dnd2024:spell_wall_of_stone
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 444b98a55c8605cd8bd3f3a0eb646a1116a80b4877f06f8d554a00f39493c5ef
splat_clauses:
- key: source_rules
  text: '# Wall of Stone Level 5 Evocation (Druid, Sorcerer, Wizard) Casting Time: Action Range: 120 feet
    Components: V, S, M (a cube of granite) Duration: Concentration, up to 10 minutes A nonmagical wall
    of solid stone springs into existence at a point you choose within range. The wall is 6 inches thick
    and is composed of ten 10-foot-by- 10-foot panels. Each panel must be contiguous with another panel.
    Alternatively, you can create 10-footby-20-foot panels that are only 3 inches thick. If the wall cuts
    through a creature’s space when it appears, the creature is pushed to one side of the wall (you choose
    which side). If a creature would be surrounded on all sides by the wall (or the wall and another solid
    surface), that creature can make a Dexterity saving throw. On a success, it can use its Reaction to
    move up to its Speed so that it is no longer enclosed by the wall. The wall can have any shape you
    desire, though it can’t occupy the same space as a creature or object. The wall doesn’t need to be
    vertical or rest on a firm foundation. It must, however, merge with and be solidly supported by existing
    stone. Thus, you can use this spell to bridge a chasm or create a ramp. If you create a span greater
    than 20 feet in length, you must halve the size of each panel to create supports. You can crudely
    shape the wall to create battlements and the like. The wall is an object made of stone that can be
    damaged and thus breached. Each panel has AC 15 and 30 Hit Points per inch of thickness, and it has
    Immunity to Poison and Psychic damage. Reducing a panel to 0 Hit Points destroys it and might cause
    connected panels to collapse at the GM’s discretion. If you maintain your Concentration on this spell
    for its full duration, the wall becomes permanent and can’t be dispelled. Otherwise, the wall disappears
    when the spell ends.'
  sha256: e82b38dde1e2f3ecf22ca3723559db67ee0bd02d1e0553c6b50f09c08c3c0137
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wall of Stone

<!-- SRD page 173 -->

Level 5 Evocation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a cube of granite)\
Duration: Concentration, up to 10 minutes\
A nonmagical wall of solid stone springs into exis-\
tence at a point you choose within range. The wall\
is 6 inches thick and is composed of ten 10-foot-by-\
10-foot panels. Each panel must be contiguous with\
another panel. Alternatively, you can create 10-foot-\
by-20-foot panels that are only 3 inches thick.\
If the wall cuts through a creature’s space when\
it appears, the creature is pushed to one side of the\
wall (you choose which side). If a creature would\
be surrounded on all sides by the wall (or the wall\
and another solid surface), that creature can make\
a Dexterity saving throw. On a success, it can use its\
Reaction to move up to its Speed so that it is no lon-\
ger enclosed by the wall.\
The wall can have any shape you desire, though it\
can’t occupy the same space as a creature or object.\
The wall doesn’t need to be vertical or rest on a firm\
foundation. It must, however, merge with and be\
solidly supported by existing stone. Thus, you can\
use this spell to bridge a chasm or create a ramp.\
If you create a span greater than 20 feet in length,\
you must halve the size of each panel to create sup-\
ports. You can crudely shape the wall to create bat-\
tlements and the like.\
The wall is an object made of stone that can be\
damaged and thus breached. Each panel has AC 15\
and 30 Hit Points per inch of thickness, and it has\
Immunity to Poison and Psychic damage. Reducing\
a panel to 0 Hit Points destroys it and might cause\
connected panels to collapse at the GM’s discretion.\
If you maintain your Concentration on this spell\
for its full duration, the wall becomes permanent\
and can’t be dispelled. Otherwise, the wall disap-\
pears when the spell ends.\

Original layout: [[../../srd-5.2.1/pages/173|page 173]].
