---
splat_schema: 1
splat_id: fa9efad8-6342-54dc-b7ed-b9c16ff919a0
splat_kind: spell
splat_key: dnd2024:spells/passwall
splat_name: Passwall
splat_origin:
  key: dnd2024:reference/spell/passwall
splat_sources:
- title: SRD 5.2.1
  locator: pp. 151
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: transmutation
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a pinch of sesame seeds
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/passwall
  implementation: dnd2024:spell_passwall
  parameters:
    surface:
      materials:
      - wood
      - plaster
      - stone
      visible: true
    passage:
      dimensions_chooser: caster
      maximum_width_feet: 5
      maximum_height_feet: 8
      maximum_depth_feet: 20
      structural_instability: false
    end:
      subjects: creatures_and_objects_in_passage
      eject_safely: true
      destination: unoccupied_nearest_cast_surface
splat_contracts:
- id: dnd2024:spell_passwall
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
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
  - passage
  - ejected_entities
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: 253e9cab18f7798f8b2c5ca4bc6bed5c9aa7b1040806d93e55d9d07e5c5e12d9
  source_path: builtin/srd-5.2.1/spell/passwall.md
  pages:
  - 151
  engine_gaps: &id001
  - dnd2024:spell_passwall
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 0086ec1c4cae036ee233a3a096ada9949b9920e78343db68df962647b1862cf4
splat_clauses:
- key: source_rules
  text: '# Passwall Level 5 Transmutation (Wizard) Casting Time: Action Range: 30 feet Components: V,
    S, M (a pinch of sesame seeds) Duration: 1 hour A passage appears at a point that you can see on a
    wooden, plaster, or stone surface (such as a wall, ceiling, or floor) within range and lasts for the
    duration. You choose the opening’s dimensions: up to 5 feet wide, 8 feet tall, and 20 feet deep. The
    passage creates no instability in a structure surrounding it. When the opening disappears, any creatures
    or objects still in the passage created by the spell are safely ejected to an unoccupied space nearest
    to the surface on which you cast the spell.'
  sha256: 68d20a6ef74d43fb85d526fe4a9f11405cfc44e1c5734231e59ba4ec16313a71
  status: contract_specified
splat_requires_mechanics: *id001
---

# Passwall

<!-- SRD page 151 -->

Level 5 Transmutation (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a pinch of sesame seeds)\
Duration: 1 hour\
A passage appears at a point that you can see on a\
wooden, plaster, or stone surface (such as a wall,\
ceiling, or floor) within range and lasts for the dura-\
tion. You choose the opening’s dimensions: up to 5\
feet wide, 8 feet tall, and 20 feet deep. The passage\
creates no instability in a structure surrounding it.\
When the opening disappears, any creatures or\
objects still in the passage created by the spell are\
safely ejected to an unoccupied space nearest to the\
surface on which you cast the spell.\

Original layout: [[../../srd-5.2.1/pages/151|page 151]].
