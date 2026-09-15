---
splat_schema: 1
splat_id: ce55eb66-205d-561f-b66d-52bc60dae093
splat_kind: spell
splat_key: dnd2024:spells/reverse_gravity
splat_name: Reverse Gravity
splat_origin:
  key: dnd2024:reference/spell/reverse_gravity
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: transmutation
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
    feet: 100
  material:
    description: a lodestone and iron filings
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/reverse_gravity
  implementation: dnd2024:spell_reverse_gravity
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    area:
      shape: cylinder
      radius_feet: 50
      height_feet: 100
      origin: chosen_point
      range_feet: 100
    targets:
      kinds:
      - creature
      - object
      in_area: true
      anchored_to_ground: false
    avoid_fall:
      target_kind: creature
      requires: fixed_object_in_reach
      save:
        ability: dexterity
        dc: spell_save_dc
      success: grab_fixed_object_and_avoid_upward_fall
    upward_fall:
      destination: cylinder_top
      collision_with:
      - ceiling
      - anchored_object
      collision_rule: normal_downward_fall
      no_collision_at_top: hover_for_duration
    spell_end:
      affected_creatures_and_objects: fall_downward
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_reverse_gravity
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
    targets:
      type: participant_list
      required: true
    objects:
      type: object_list
      required: true
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - areas
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 0dca4f9e65aa621346cd2e7a06b3b6c83bad983fa2a8c17df71f726a3aeafa76
  source_path: builtin/srd-5.2.1/spell/reverse_gravity.md
  pages:
  - 159
  engine_gaps: &id001
  - dnd2024:spell_reverse_gravity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: f6c9e7e0baec438f74ce84e6d586cc98de35d5acf38b2e39f062ef668e85b44d
splat_clauses:
- key: source_rules
  text: '# Reverse Gravity Level 7 Transmutation (Druid, Sorcerer, Wizard) Casting Time: Action Range:
    100 feet Components: V, S, M (a lodestone and iron filings) Duration: Concentration, up to 1 minute
    This spell reverses gravity in a 50-foot-radius, 100- foot high Cylinder centered on a point within
    range. All creatures and objects in that area that aren’t anchored to the ground fall upward and reach
    the top of the Cylinder. A creature can make a Dexterity saving throw to grab a fixed object it can
    reach, thus avoiding the fall upward. If a ceiling or an anchored object is encountered in this upward
    fall, creatures and objects strike it just as they would during a downward fall. If an affected creature
    or object reaches the Cylinder’s top without striking anything, it hovers there for the duration.
    When the spell ends, affected objects and creatures fall downward.'
  sha256: 2943146a6d77d89c01f5285c6186bd43d232bfcece76b7d8d6826c63c728ae72
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reverse Gravity

<!-- SRD page 159 -->

Level 7 Transmutation (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 100 feet\
Components: V, S, M (a lodestone and iron filings)\
Duration: Concentration, up to 1 minute\
This spell reverses gravity in a 50-foot-radius, 100-\
foot high Cylinder centered on a point within range.\
All creatures and objects in that area that aren’t\
anchored to the ground fall upward and reach the\
top of the Cylinder. A creature can make a Dexterity\
saving throw to grab a fixed object it can reach, thus\
avoiding the fall upward.\
If a ceiling or an anchored object is encountered\
in this upward fall, creatures and objects strike it\
just as they would during a downward fall. If an af-\
fected creature or object reaches the Cylinder’s top\
without striking anything, it hovers there for the\
duration. When the spell ends, affected objects and\
creatures fall downward.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
