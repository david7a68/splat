---
splat_schema: 1
splat_id: 6ebcd8ec-1036-5ca6-82ff-1bcd29d51812
splat_kind: spell
splat_key: dnd2024:spells/sleet_storm
splat_name: Sleet Storm
splat_origin:
  key: dnd2024:reference/spell/sleet_storm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: conjuration
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
    feet: 150
  material:
    description: a miniature umbrella
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/sleet_storm
  implementation: dnd2024:spell_sleet_storm
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    area:
      shape: cylinder
      height_feet: 40
      radius_feet: 20
      origin: chosen_point
      range_feet: 150
    properties:
      obscurement: heavy
      exposed_flames: doused
      ground: difficult_terrain
    triggers:
    - event: creature.entered_area
      occurrence_per_turn: first
    - event: turn.start
      subjects: creatures_in_area
    save:
      ability: dexterity
      dc: spell_save_dc
      success: unaffected
      failure:
        condition: prone
        concentration: lost
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_sleet_storm
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
  - creature.entered_area
  - spell.cast_completed
  - turn.start
  capabilities:
  - spellcasting
  - effects
  - areas
  - conditions
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: a84496921b714f9a94c40b761b98431875ee3afd1996187318bc64c3147286e3
  source_path: builtin/srd-5.2.1/spell/sleet_storm.md
  pages:
  - 163
  engine_gaps: &id001
  - dnd2024:spell_sleet_storm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 496a03dd34a4db79fcc8be7ac58d273ed615e0662b3ad5947f6a38bd6a529bf1
splat_clauses:
- key: source_rules
  text: '# Sleet Storm Level 3 Conjuration (Druid, Sorcerer, Wizard) Casting Time: Action Range: 150 feet
    Components: V, S, M (a miniature umbrella) Duration: Concentration, up to 1 minute Until the spell
    ends, sleet falls in a 40-foot-tall, 20-foot-radius Cylinder centered on a point you choose within
    range. The area is Heavily Obscured, and exposed flames in the area are doused. Ground in the Cylinder
    is Difficult Terrain. When a creature enters the Cylinder for the first time on a turn or starts its
    turn there, it must succeed on a Dexterity saving throw or have the Prone condition and lose Concentration.'
  sha256: a55d47bd4b811938abe857e8f686dd9d71a611a23c7251d5e15c97689a05b733
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sleet Storm

<!-- SRD page 163 -->

Level 3 Conjuration (Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a miniature umbrella)\
Duration: Concentration, up to 1 minute\
Until the spell ends, sleet falls in a 40-foot-tall,\
20-foot-radius Cylinder centered on a point you\
choose within range. The area is Heavily Obscured,\
and exposed flames in the area are doused.\
Ground in the Cylinder is Difficult Terrain. When\
a creature enters the Cylinder for the first time on\
a turn or starts its turn there, it must succeed on a\
Dexterity saving throw or have the Prone condition\
and lose Concentration.\

Original layout: [[../../srd-5.2.1/pages/163|page 163]].
