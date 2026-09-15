---
splat_schema: 1
splat_id: 8344fcc7-ab23-5c62-a184-db3913394a69
splat_kind: spell
splat_key: dnd2024:spells/grease
splat_name: Grease
splat_origin:
  key: dnd2024:reference/spell/grease
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a bit of pork rind or butter
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/grease
  implementation: dnd2024:spell_grease
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 60
    area:
      shape: square
      side_feet: 10
      surface: ground
      origin: chosen_point
      range_feet: 60
    properties:
      flammable: false
      difficult_terrain: true
    saving_throw_triggers:
    - event: spell.cast_completed
      subjects: creatures_standing_in_area
    - event: creature.entered_area
    - event: turn.end
      subjects: creatures_in_area
    save:
      ability: dexterity
      dc: spell_save_dc
      failure_condition: prone
      success: unaffected
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_grease
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
  - creature.entered_area
  - spell.cast_completed
  - turn.end
  capabilities:
  - spellcasting
  - effects
  - movement
  - areas
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 49972a283d92d9355cb98fc56670327651afea31936c68524b228a105424463f
  source_path: builtin/srd-5.2.1/spell/grease.md
  pages:
  - 137
  engine_gaps: &id001
  - dnd2024:spell_grease
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: faa442336e2311e757b0ea02e30d1cc77ed9e89dbf057111feb73112c381b523
splat_clauses:
- key: source_rules
  text: '# Grease Level 1 Conjuration (Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S, M (a bit of pork rind or butter) Duration: 1 minute Nonflammable grease covers the ground in
    a 10- foot square centered on a point within range and turns it into Difficult Terrain for the duration.
    When the grease appears, each creature standing in its area must succeed on a Dexterity saving throw
    or have the Prone condition. A creature that enters the area or ends its turn there must also succeed
    on that save or fall Prone.'
  sha256: 1edd22c7241a1a3af53242d45e07c530272390fcfb14f49662850652b7255489
  status: contract_specified
splat_requires_mechanics: *id001
---

# Grease

<!-- SRD page 137 -->

Level 1 Conjuration (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of pork rind or butter)\
Duration: 1 minute\
Nonflammable grease covers the ground in a 10-\
foot square centered on a point within range and\
turns it into Difficult Terrain for the duration.\
When the grease appears, each creature stand-\
ing in its area must succeed on a Dexterity saving\
throw or have the Prone condition. A creature that\
enters the area or ends its turn there must also suc-\
ceed on that save or fall Prone.\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].
