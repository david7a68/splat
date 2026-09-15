---
splat_schema: 1
splat_id: d41bb77c-c62e-5e75-ad2b-d04aff455eb0
splat_kind: spell
splat_key: dnd2024:spells/creation
splat_name: Creation
splat_origin:
  key: dnd2024:reference/spell/creation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 121
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: illusion
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Special
  casting_time: 1 minute
  ritual: false
  range:
    feet: 30
  material:
    description: a paintbrush
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/creation
  implementation: dnd2024:spell_creation
  parameters:
    spell_lifetime:
      concentration: false
      determined_by: material_duration_table
    object:
      count: 1
      range_feet: 30
      permitted_matter:
      - vegetable
      - mineral
      form_seen_before: true
      material_seen_before: true
      maximum_cube_side_feet:
        base: 5
        increment_per_slot: 5
        base_slot: 5
    duration_by_material_seconds:
      vegetable: 86400
      stone: 43200
      crystal: 43200
      precious_metal: 3600
      gems: 600
      adamantine: 60
      mithral: 60
    mixed_material_duration: minimum_component_duration
    unlisted_mineral_duration:
      kind: unresolved_source_case
      required_input: material_duration_resolution
      no_automatic_default: true
    used_as_other_spell_material:
      effect: other_spell_fails
splat_contracts:
- id: dnd2024:spell_creation
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    object_design:
      type: choice
      required: true
    origin:
      type: location
      required: true
    material_duration_resolution:
      required: false
      type: duration
  outputs:
  - effects
  - objects
  - unresolved_material_duration
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - world_objects
  - campaign_time
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2a6dff46b840792797022372975365154e84b50412d4c351c121ee7d8da40f5d
  source_path: builtin/srd-5.2.1/spell/creation.md
  pages:
  - 121
  engine_gaps: &id001
  - dnd2024:spell_creation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 3269268da9f48863a6901f6b47572e504aa58b2839528814baefbdf24351d2c9
splat_clauses:
- key: source_rules
  text: '# Creation Level 5 Illusion (Sorcerer, Wizard) Casting Time: 1 minute Range: 30 feet Components:
    V, S, M (a paintbrush) Duration: Special You pull wisps of shadow material from the Shadowfell to
    create an object within range. It is either an object of vegetable matter (soft goods, rope, wood,
    and the like) or mineral matter (stone, crystal, metal, and the like). The object must be no larger
    than a 5-foot Cube, and the object must be of a form and material that you have seen. The spell’s
    duration depends on the object’s material, as shown in the Materials table. If the object is composed
    of multiple materials, use the shortest duration. Using any object created by this spell as another
    spell’s Material component causes the other spell to fail. Materials Material Duration Vegetable matter
    24 hours Stone or crystal 12 hours Precious metals 1 hour Gems 10 minutes Adamantine or mithral 1
    minute Using a Higher-Level Spell Slot. The Cube increases by 5 feet for each spell slot level above
    5.'
  sha256: 922b9b7405f21f224eb7be09a2b82de8ea3019f29eb44097f33be533719811db
  status: contract_specified
splat_requires_mechanics: *id001
---

# Creation

<!-- SRD page 121 -->

Level 5 Illusion (Sorcerer, Wizard)\
Casting Time: 1 minute\
Range: 30 feet\
Components: V, S, M (a paintbrush)\
Duration: Special\
You pull wisps of shadow material from the Shad-\
owfell to create an object within range. It is either\
an object of vegetable matter (soft goods, rope,\
wood, and the like) or mineral matter (stone, crys-\
tal, metal, and the like). The object must be no\
larger than a 5-foot Cube, and the object must be of\
a form and material that you have seen.\
The spell’s duration depends on the object’s ma-\
terial, as shown in the Materials table. If the object\
is composed of multiple materials, use the shortest\
duration. Using any object created by this spell as\
another spell’s Material component causes the other\
spell to fail.\
Materials\
Material Duration\
Vegetable matter 24 hours\
Stone or crystal 12 hours\
Precious metals 1 hour\
Gems 10 minutes\
Adamantine or mithral 1 minute\
Using a Higher-Level Spell Slot. The Cube in-\
creases by 5 feet for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/121|page 121]].
