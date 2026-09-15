---
splat_schema: 1
splat_id: 162482da-06c9-57c0-bb21-ed2effd1e491
splat_kind: spell
splat_key: dnd2024:spells/locate_object
splat_name: Locate Object
splat_origin:
  key: dnd2024:reference/spell/locate_object
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: divination
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: a forked twig
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/locate_object
  implementation: dnd2024:spell_locate_object
  parameters:
    selection:
      specific_object:
        familiar: true
        describe_or_name: true
        seen_at_least_once_within_feet: 30
      object_kind:
        nearest: true
    detection:
      maximum_distance_feet: 1000
      outputs:
      - direction_to_object
      - movement_direction_if_moving
    blocked:
      material: lead
      thickness: any
      path: direct_caster_to_object
splat_contracts:
- id: dnd2024:spell_locate_object
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
  - object_direction
  - movement_direction
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 58a754c9132d7b8a375292d85d71ab531d5c7ac9f27257864cab0ee6b4316743
  source_path: builtin/srd-5.2.1/spell/locate_object.md
  pages:
  - 145
  engine_gaps: &id001
  - dnd2024:spell_locate_object
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 1a31579932a7317e4f436d649735cc4dca8444aec56dc092f350c17b0275cab3
splat_clauses:
- key: source_rules
  text: '# Locate Object Level 2 Divination (Bard, Cleric, Druid, Paladin, Ranger, Wizard) Casting Time:
    Action Range: Self Components: V, S, M (a forked twig) Duration: Concentration, up to 10 minutes Describe
    or name an object that is familiar to you. You sense the direction to the object’s location if that
    object is within 1,000 feet of you. If the object is in motion, you know the direction of its movement.
    The spell can locate a specific object known to you if you have seen it up close—within 30 feet—at
    least once. Alternatively, the spell can locate the nearest object of a particular kind, such as a
    certain kind of apparel, jewelry, furniture, tool, or weapon. This spell can’t locate an object if
    any thickness of lead blocks a direct path between you and the object.'
  sha256: f1720ae786cd827b5f78a2300dc2e4d764a5533f0e88b75092be3b7068307fa8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Locate Object

<!-- SRD page 145 -->

Level 2 Divination (Bard, Cleric, Druid, Paladin,\
Ranger, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a forked twig)\
Duration: Concentration, up to 10 minutes\
Describe or name an object that is familiar to you.\
You sense the direction to the object’s location if\
that object is within 1,000 feet of you. If the object is\
in motion, you know the direction of its movement.\
The spell can locate a specific object known to you\
if you have seen it up close—within 30 feet—at least\
once. Alternatively, the spell can locate the nearest\
object of a particular kind, such as a certain kind of\
apparel, jewelry, furniture, tool, or weapon.\
This spell can’t locate an object if any thickness\
of lead blocks a direct path between you and the\
object.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]].
