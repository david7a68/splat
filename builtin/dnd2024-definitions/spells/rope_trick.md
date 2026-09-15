---
splat_schema: 1
splat_id: 9f77dd14-43cf-572f-a834-44b88653818b
splat_kind: spell
splat_key: dnd2024:spells/rope_trick
splat_name: Rope Trick
splat_origin:
  key: dnd2024:reference/spell/rope_trick
splat_sources:
- title: SRD 5.2.1
  locator: pp. 159
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
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
    mode: touch
  material:
    description: a segment of rope
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/rope_trick
  implementation: dnd2024:spell_rope_trick
  parameters:
    targeting:
      kind: rope
      touch: true
    rope:
      upper_end_hovers_until: perpendicular_or_ceiling_reached
      can_pull_inside: true
      can_drop_outside: true
    portal:
      location: rope_upper_end
      invisible: true
      width_feet: 3
      height_feet: 5
      destination: extradimensional_space
      access: climb_rope
      duration: spell_duration
    space:
      capacity_creatures: 8
      maximum_creature_size: medium
      blocked_crossing:
      - attacks
      - spells
      - other_effects
      vision_from_inside_through_portal: true
      spell_end: contents_drop_out
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_rope_trick
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      type: location_list
      required: true
    objects:
      type: object_list
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: 19d8883c273caab0d8920a6bdc4dd964772d8e66fa55f633cf1d97328fbfe744
  source_path: builtin/srd-5.2.1/spell/rope_trick.md
  pages:
  - 159
  engine_gaps: &id001
  - dnd2024:spell_rope_trick
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 13c3a8c561c60449d765c68845bc2a5ea1b913ce7145d305427126e0e633352c
splat_clauses:
- key: source_rules
  text: '# Rope Trick Level 2 Transmutation (Wizard) Casting Time: Action Range: Touch Components: V,
    S, M (a segment of rope) Duration: 1 hour You touch a rope. One end of it hovers upward until the
    rope hangs perpendicular to the ground or the rope reaches a ceiling. At the rope’s upper end, an
    Invisible 3-foot-by-5-foot portal opens to an extradimensional space that lasts until the spell ends.
    That space can be reached by climbing the rope, which can be pulled into or dropped out of it. The
    space can hold up to eight Medium or smaller creatures. Attacks, spells, and other effects can’t pass
    into or out of the space, but creatures inside it can see through the portal. Anything inside the
    space drops out when the spell ends.'
  sha256: 95e9538c78ee48ca2649e438791b3d01d48238affdaa55be0163e0fdb47760d6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rope Trick

<!-- SRD page 159 -->

Level 2 Transmutation (Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a segment of rope)\
Duration: 1 hour\
You touch a rope. One end of it hovers upward until\
the rope hangs perpendicular to the ground or the\
rope reaches a ceiling. At the rope’s upper end, an\
Invisible 3-foot-by-5-foot portal opens to an extra-\
dimensional space that lasts until the spell ends.\
That space can be reached by climbing the rope,\
which can be pulled into or dropped out of it.\
The space can hold up to eight Medium or smaller\
creatures. Attacks, spells, and other effects can’t\
pass into or out of the space, but creatures inside\
it can see through the portal. Anything inside the\
space drops out when the spell ends.\

Original layout: [[../../srd-5.2.1/pages/159|page 159]].
