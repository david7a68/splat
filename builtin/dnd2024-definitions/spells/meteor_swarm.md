---
splat_schema: 1
splat_id: e453db69-a71f-5b63-87bc-c0c86214f934
splat_kind: spell
splat_key: dnd2024:spells/meteor_swarm
splat_name: Meteor Swarm
splat_origin:
  key: dnd2024:reference/spell/meteor_swarm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 9
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: 1 mile
splat_rule:
  mechanic: dnd2024:spells/meteor_swarm
  implementation: dnd2024:spell_meteor_swarm
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    areas:
      count: 4
      different_points: true
      point_visible: true
      range_miles: 1
      impact: ground
      shape: sphere
      radius_feet: 40
    targets:
      kind: creature
      in_any_area: true
      overlapping_areas_affect_count: 1
    save:
      ability: dexterity
      dc: spell_save_dc
    damage:
    - dice: 20d6
      type: fire
    - dice: 20d6
      type: bludgeoning
    success_damage_scale:
      numerator: 1
      denominator: 2
    objects:
      magical: false
      worn: false
      carried: false
      in_area: true
      take_damage: true
      flammable_effect: start_burning
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_meteor_swarm
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origins:
      type: location_list
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
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - damage
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 9533d19d0ebda7863c79b7bf50edc8335c437dc6a94b9ae7701d3c940637fd43
  source_path: builtin/srd-5.2.1/spell/meteor_swarm.md
  pages:
  - 149
  engine_gaps: &id001
  - dnd2024:spell_meteor_swarm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: f59616dce4bd4be5216386aa17c85ff47b4683a160c53487dfd01e1fe6e79c70
splat_clauses:
- key: source_rules
  text: '# Meteor Swarm Level 9 Evocation (Sorcerer, Wizard) Casting Time: Action Range: 1 mile Components:
    V, S Duration: Instantaneous Blazing orbs of fire plummet to the ground at four different points you
    can see within range. Each creature in a 40-foot-radius Sphere centered on each of those points makes
    a Dexterity saving throw. A creature takes 20d6 Fire damage and 20d6 Bludgeoning damage on a failed
    save or half as much damage on a successful one. A creature in the area of more than one fiery Sphere
    is affected only once. A nonmagical object that isn’t being worn or carried also takes the damage
    if it’s in the spell’s area, and the object starts burning if it’s flammable.'
  sha256: fec71321d2cf98ecba3213d810b1d2dcad373f102c0dc5911b8fce654f8ef83a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Meteor Swarm

<!-- SRD page 149 -->

Level 9 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 1 mile\
Components: V, S\
Duration: Instantaneous\
Blazing orbs of fire plummet to the ground at four\
different points you can see within range. Each\
creature in a 40-foot-radius Sphere centered on\
each of those points makes a Dexterity saving\
throw. A creature takes 20d6 Fire damage and 20d6\
Bludgeoning damage on a failed save or half as\
much damage on a successful one. A creature in the\
area of more than one fiery Sphere is affected only\
once.\
A nonmagical object that isn’t being worn or car-\
ried also takes the damage if it’s in the spell’s area,\
and the object starts burning if it’s flammable.\

Original layout: [[../../srd-5.2.1/pages/149|page 149]].
