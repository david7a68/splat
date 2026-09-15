---
splat_schema: 1
splat_id: a42e0a5b-0341-5b22-b5e1-000d11d47c62
splat_kind: spell
splat_key: dnd2024:spells/daylight
splat_name: Daylight
splat_origin:
  key: dnd2024:reference/spell/daylight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: evocation
  classes:
  - cleric
  - druid
  - paladin
  - ranger
  - sorcerer
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/daylight
  implementation: dnd2024:spell_daylight
  parameters:
    origin_choice:
      point:
        shape: sphere
      object:
        shape: emanation
        not_worn: true
        not_carried: true
        opaque_cover_blocks: true
    light:
      sunlight: true
      bright_radius_feet: 60
      additional_dim_feet: 60
    dispel:
      event: area.overlap
      target: darkness_spell
      maximum_spell_level: 3
splat_contracts:
- id: dnd2024:spell_daylight
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
  - sunlight_effect
  - dispelled_effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - spatial_areas
  - perception
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: ffbba253b702b505b616f1b15955271d59966feacdeb9ae144652c36d42b2eb5
  source_path: builtin/srd-5.2.1/spell/daylight.md
  pages:
  - 122
  engine_gaps: &id001
  - dnd2024:spell_daylight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 9d86a462feb99d02a6ccfd5224547ba6e192bb4961568e6b7b244f81da827de9
splat_clauses:
- key: source_rules
  text: '# Daylight Level 3 Evocation (Cleric, Druid, Paladin, Ranger, Sorcerer) Casting Time: Action
    Range: 60 feet Components: V, S Duration: 1 hour For the duration, sunlight spreads from a point within
    range and fills a 60-foot-radius Sphere. The sunlight’s area is Bright Light and sheds Dim Light for
    an additional 60 feet. Alternatively, you cast the spell on an object that isn’t being worn or carried,
    causing the sunlight to fill a 60-foot Emanation originating from that object. Covering that object
    with something opaque, such as a bowl or helm, blocks the sunlight. If any of this spell’s area overlaps
    with an area of Darkness created by a spell of level 3 or lower, that other spell is dispelled.'
  sha256: 651ee6812e6e0781e9b3637567325472cd9a74514af0b2b28139c12f7eeda3fc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Daylight

<!-- SRD page 122 -->

Level 3 Evocation (Cleric, Druid, Paladin, Ranger,\
Sorcerer)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: 1 hour\
For the duration, sunlight spreads from a point\
within range and fills a 60-foot-radius Sphere. The\
sunlight’s area is Bright Light and sheds Dim Light\
for an additional 60 feet.\
Alternatively, you cast the spell on an object that\
isn’t being worn or carried, causing the sunlight\
to fill a 60-foot Emanation originating from that\
object. Covering that object with something opaque,\
such as a bowl or helm, blocks the sunlight.\
If any of this spell’s area overlaps with an area of\
Darkness created by a spell of level 3 or lower, that\
other spell is dispelled.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].
