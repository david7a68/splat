---
splat_schema: 1
splat_id: a6b1254d-f8a5-55fb-a778-d8b0d1e04488
splat_kind: spell
splat_key: dnd2024:spells/mirage_arcane
splat_name: Mirage Arcane
splat_origin:
  key: dnd2024:reference/spell/mirage_arcane
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149, 150
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: illusion
  classes:
  - bard
  - druid
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: 10 days
  casting_time: 10 minutes
  ritual: false
  range:
    mode: sight
splat_rule:
  mechanic: dnd2024:spells/mirage_arcane
  implementation: dnd2024:spell_mirage_arcane
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 864000
    area:
      maximum_square_miles: 1
      range: sight
    terrain:
      appearance_design: caster_choice
      modalities:
      - visual
      - audible
      - tactile
      - olfactory
      movement_effect_choices:
      - create_difficult_terrain
      - remove_difficult_terrain
      - impede_movement
      terrain_geometry: supplied_design
    structures:
      alter_appearance: true
      add_illusory_structures: true
    creatures:
      disguise: false
      conceal: false
      add: false
    removed_illusory_piece:
      event: object.left_area
      effect: immediately_disappear
    truesight:
      see_true_terrain: true
      aware_of_illusion: true
      removes_other_modalities: false
      physical_interaction_remains_possible: true
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_mirage_arcane
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    area:
      type: location
      required: true
    terrain_design:
      type: choice
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - object.left_area
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - illusions
  - areas
splat_conversion:
  status: specified
  category: spell
  source_sha256: b726d5867a53e8a6d70054c1f82e70d5ff1aecad4f741a377a17265d0c5fb0dd
  source_path: builtin/srd-5.2.1/spell/mirage_arcane.md
  pages:
  - 149
  - 150
  engine_gaps: &id001
  - dnd2024:spell_mirage_arcane
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 6bc7a63cbdd32378e403e8bd9fe16e8d22a41d7ac582d44c26104f547b2b7996
splat_clauses:
- key: source_rules
  text: '# Mirage Arcane Level 7 Illusion (Bard, Druid, Wizard) Casting Time: 10 minutes Range: Sight
    Components: V, S Duration: 10 days You make terrain in an area up to 1 mile square look, sound, smell,
    and even feel like some other sort of terrain. Open fields or a road could be made to resemble a swamp,
    hill, crevasse, or some other rough or impassable terrain. A pond can be made to seem like a grassy
    meadow, a precipice like a gentle slope, or a rock-strewn gully like a wide and smooth road. Similarly,
    you can alter the appearance of structures or add them where none are present. The spell doesn’t disguise,
    conceal, or add creatures. The illusion includes audible, visual, tactile, and olfactory elements,
    so it can turn clear ground into Difficult Terrain (or vice versa) or otherwise impede movement through
    the area. Any piece of the illusory terrain (such as a rock or stick) that is removed from the spell’s
    area disappears immediately. Creatures with Truesight can see through the illusion to the terrain’s
    true form; however, all other elements of the illusion remain, so while the creature is aware of the
    illusion’s presence, the creature can still physically interact with the illusion.'
  sha256: 58a15003be30c86279267bc21104821f9434054059e067de80e7ed89ec7f38e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mirage Arcane

<!-- SRD page 149 -->

Level 7 Illusion (Bard, Druid, Wizard)\
Casting Time: 10 minutes\
Range: Sight\
Components: V, S\
Duration: 10 days\
You make terrain in an area up to 1 mile square\
look, sound, smell, and even feel like some other\
sort of terrain. Open fields or a road could be made\
to resemble a swamp, hill, crevasse, or some other\
rough or impassable terrain. A pond can be made to\
seem like a grassy meadow, a precipice like a gentle\
slope, or a rock-strewn gully like a wide and smooth\
road.\
Similarly, you can alter the appearance of struc-\
tures or add them where none are present. The spell\
doesn’t disguise, conceal, or add creatures.\
The illusion includes audible, visual, tactile, and\
olfactory elements, so it can turn clear ground\

<!-- SRD page 150 -->

into Difficult Terrain (or vice versa) or otherwise\
impede movement through the area. Any piece\
of the illusory terrain (such as a rock or stick)\
that is removed from the spell’s area disappears\
immediately.\
Creatures with Truesight can see through the illu-\
sion to the terrain’s true form; however, all other el-\
ements of the illusion remain, so while the creature\
is aware of the illusion’s presence, the creature can\
still physically interact with the illusion.\

Original layout: [[../../srd-5.2.1/pages/149|page 149]], [[../../srd-5.2.1/pages/150|page 150]].
