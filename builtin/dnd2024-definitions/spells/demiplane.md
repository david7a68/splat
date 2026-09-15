---
splat_schema: 1
splat_id: ba44ffb5-d3ba-51e7-b550-a6a5e0ec0729
splat_kind: spell
splat_key: dnd2024:spells/demiplane
splat_name: Demiplane
splat_origin:
  key: dnd2024:reference/spell/demiplane
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122, 123
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: conjuration
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - somatic
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/demiplane
  implementation: dnd2024:spell_demiplane
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 3600
    doorway:
      size: medium
      range_feet: 60
      surface:
        flat: true
        solid: true
        visible: true
      can_open_close: true
    destination_choices:
      chooser: caster
      options:
      - new_demiplane
      - caster_previous_demiplane
      - other_caster_demiplane
      other_caster_requirement: know_nature_and_contents
    new_room:
      empty: true
      length_feet: 30
      width_feet: 30
      height_feet: 30
      material:
        chooser: caster
        options:
        - wood
        - stone
    end:
      door_vanishes: true
      objects_remain: true
      creatures:
        chooser: each_creature
        options:
        - remain
        - exit
        exit:
          destination: nearest_unoccupied_space_to_former_door
          condition: prone
splat_contracts:
- id: dnd2024:spell_demiplane
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    door_location:
      type: location
      required: true
    destination:
      type: choice
      required: true
    room_material:
      required: false
      type: choice
    exit_choices:
      required: false
      type: choice
  outputs:
  - effects
  - transports
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: b925d3321088ea91459ace90e2f4a02c2edb503ec4df07ea242eb3fee50b60c9
  source_path: builtin/srd-5.2.1/spell/demiplane.md
  pages:
  - 122
  - 123
  engine_gaps: &id001
  - dnd2024:spell_demiplane
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: fb0465a1374ccf9d5e8fc96fa149fd0bc0e6cda9e28124587b0e047d34d96448
splat_clauses:
- key: source_rules
  text: '# Demiplane Level 8 Conjuration (Sorcerer, Warlock, Wizard) Casting Time: Action Range: 60 feet
    Components: S Duration: 1 hour You create a shadowy Medium door on a flat solid surface that you can
    see within range. This door can be opened and closed, and it leads to a demiplane that is an empty
    room 30 feet in each dimension, made of wood or stone (your choice). When the spell ends, the door
    vanishes, and any objects inside the demiplane remain there. Any creatures inside also remain unless
    they opt to be shunted through the door as it vanishes, landing with the Prone condition in the unoccupied
    spaces closest to the door’s former space. Each time you cast this spell, you can create a new demiplane
    or connect the shadowy door to a demiplane you created with a previous casting of this spell. Additionally,
    if you know the nature and contents of a demiplane created by a casting of this spell by another creature,
    you can connect the shadowy door to that demiplane instead.'
  sha256: 666b10b9ff28372844487b0792c6159fd4208e0d08327a3f84376f9bf679677c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Demiplane

<!-- SRD page 122 -->

Level 8 Conjuration (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: S\
Duration: 1 hour\

<!-- SRD page 123 -->

You create a shadowy Medium door on a flat solid\
surface that you can see within range. This door can\
be opened and closed, and it leads to a demiplane\
that is an empty room 30 feet in each dimension,\
made of wood or stone (your choice).\
When the spell ends, the door vanishes, and any\
objects inside the demiplane remain there. Any\
creatures inside also remain unless they opt to be\
shunted through the door as it vanishes, landing\
with the Prone condition in the unoccupied spaces\
closest to the door’s former space.\
Each time you cast this spell, you can create a\
new demiplane or connect the shadowy door to a\
demiplane you created with a previous casting of\
this spell. Additionally, if you know the nature and\
contents of a demiplane created by a casting of this\
spell by another creature, you can connect the shad-\
owy door to that demiplane instead.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]], [[../../srd-5.2.1/pages/123|page 123]].
