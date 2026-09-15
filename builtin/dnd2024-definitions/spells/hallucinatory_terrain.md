---
splat_schema: 1
splat_id: 54c22968-f68b-5337-aef0-496804aee395
splat_kind: spell
splat_key: dnd2024:spells/hallucinatory_terrain
splat_name: Hallucinatory Terrain
splat_origin:
  key: dnd2024:reference/spell/hallucinatory_terrain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: illusion
  classes:
  - bard
  - druid
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: 10 minutes
  ritual: false
  range:
    feet: 300
  material:
    description: a mushroom
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/hallucinatory_terrain
  implementation: dnd2024:spell_hallucinatory_terrain
  parameters:
    area:
      shape: cube
      side_feet: 150
      target: natural_terrain
    illusion:
      replacement_choice: natural_terrain_kind
      senses:
      - sight
      - hearing
      - smell
      tactile_unchanged: true
      excluded:
      - manufactured_structures
      - equipment
      - creatures
    discern:
      touch_obvious_difference: automatic
      otherwise:
        action: study
        ability: intelligence
        skill: investigation
        dc:
          binding: save_dc
        success: disbelieve
      after_discern: vague_image_over_real_terrain
splat_contracts:
- id: dnd2024:spell_hallucinatory_terrain
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
  - terrain_illusion
  - discernment_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - spatial_areas
  - perception
  - inventory
  - choices
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: cf6fe9e0e38c2bf89502df65c333490b1e5d7db46406c74093b3fd0d170822ee
  source_path: builtin/srd-5.2.1/spell/hallucinatory_terrain.md
  pages:
  - 139
  engine_gaps: &id001
  - dnd2024:spell_hallucinatory_terrain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: bc0682b4e87d77c210a81aaf5d2ff9d9852af5b28bd00258cb84efa8870b6f83
splat_clauses:
- key: source_rules
  text: '# Hallucinatory Terrain Level 4 Illusion (Bard, Druid, Warlock, Wizard) Casting Time: 10 minutes
    Range: 300 feet Components: V, S, M (a mushroom) Duration: 24 hours You make natural terrain in a
    150-foot Cube in range look, sound, and smell like another sort of natural terrain. Thus, open fields
    or a road can be made to resemble a swamp, hill, crevasse, or some other difficult or impassable terrain.
    A pond can be made to seem like a grassy meadow, a precipice like a gentle slope, or a rock-strewn
    gully like a wide and smooth road. Manufactured structures, equipment, and creatures within the area
    aren’t changed. The tactile characteristics of the terrain are unchanged, so creatures entering the
    area are likely to notice the illusion. If the difference isn’t obvious by touch, a creature examining
    the illusion can take the Study action to make an Intelligence (Investigation) check against your
    spell save DC to disbelieve it. If a creature discerns that the terrain is illusory, the creature
    sees a vague image superimposed on the real terrain.'
  sha256: 08482ae819b56de9d83d2a3ab8792ac9dd02c7bd74e9f88e2711426d9f155222
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hallucinatory Terrain

<!-- SRD page 139 -->

Level 4 Illusion (Bard, Druid, Warlock, Wizard)\
Casting Time: 10 minutes\
Range: 300 feet\
Components: V, S, M (a mushroom)\
Duration: 24 hours\
You make natural terrain in a 150-foot Cube in\
range look, sound, and smell like another sort of\
natural terrain. Thus, open fields or a road can be\
made to resemble a swamp, hill, crevasse, or some\
other difficult or impassable terrain. A pond can be\
made to seem like a grassy meadow, a precipice like\
a gentle slope, or a rock-strewn gully like a wide and\
smooth road. Manufactured structures, equipment,\
and creatures within the area aren’t changed.\
The tactile characteristics of the terrain are un-\
changed, so creatures entering the area are likely\
to notice the illusion. If the difference isn’t obvious\
by touch, a creature examining the illusion can take\
the Study action to make an Intelligence (Investiga-\
tion) check against your spell save DC to disbelieve\
it. If a creature discerns that the terrain is illusory,\
the creature sees a vague image superimposed on\
the real terrain.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]].
