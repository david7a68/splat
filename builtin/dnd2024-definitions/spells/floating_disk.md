---
splat_schema: 1
splat_id: 2cf078a5-c631-5ea9-b405-61451c723b20
splat_kind: spell
splat_key: dnd2024:spells/floating_disk
splat_name: Floating Disk
splat_origin:
  key: dnd2024:reference/spell/floating_disk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 133
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: conjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 30
  material:
    description: a drop of mercury
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/floating_disk
  implementation: dnd2024:spell_floating_disk
  parameters:
    creation:
      shape: horizontal_circular_plane
      material: force
      diameter_feet: 3
      thickness_inches: 1
      height_above_ground_feet: 3
      space:
        unoccupied: true
        visible: true
        within_spell_range: true
      capacity_pounds: 500
      duration: spell_duration
    movement:
      immobile_if_caster_within_feet: 20
      follows_if_caster_farther_than_feet: 20
      follow_distance_maximum_feet: 20
      allowed_terrain:
      - uneven
      - stairs
      - slopes
      cannot_cross_elevation_change_feet_minimum: 10
    end_if:
      load_pounds_greater_than: 500
      caster_distance_feet_greater_than: 100
    end_cleanup:
      contents: fall_to_ground
splat_contracts:
- id: dnd2024:spell_floating_disk
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - items
  events:
  - spell.cast_completed
  - effect.ended
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: ccf9c03617d5d9255643f72ac80b51616d8fceb62a3f0afdb5234945e0886527
  source_path: builtin/srd-5.2.1/spell/floating_disk.md
  pages:
  - 133
  engine_gaps: &id001
  - dnd2024:spell_floating_disk
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 5d42fa3ced1a38e47455c4671d0c36a275fccae1dfe6f0981a6854ddc0c125bb
splat_clauses:
- key: source_rules
  text: '# Floating Disk Level 1 Conjuration (Wizard) Casting Time: Action or Ritual Range: 30 feet Components:
    V, S, M (a drop of mercury) Duration: 1 hour This spell creates a circular, horizontal plane of force,
    3 feet in diameter and 1 inch thick, that floats 3 feet above the ground in an unoccupied space of
    your choice that you can see within range. The disk remains for the duration and can hold up to 500
    pounds. If more weight is placed on it, the spell ends, and everything on the disk falls to the ground.
    The disk is immobile while you are within 20 feet of it. If you move more than 20 feet away from it,
    the disk follows you so that it remains within 20 feet of you. It can move across uneven terrain,
    up or down stairs, slopes and the like, but it can’t cross an elevation change of 10 feet or more.
    For example, the disk can’t move across a 10-foot-deep pit, nor could it leave such a pit if it was
    created at the bottom. If you move more than 100 feet from the disk (typically because it can’t move
    around an obstacle to follow you), the spell ends.'
  sha256: 73e70bc94e8e6467736ffddce4264885225b68a3aef7b640bd95d19f654a5bb0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Floating Disk

<!-- SRD page 133 -->

Level 1 Conjuration (Wizard)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (a drop of mercury)\
Duration: 1 hour\
This spell creates a circular, horizontal plane of\
force, 3 feet in diameter and 1 inch thick, that floats\
3 feet above the ground in an unoccupied space\
of your choice that you can see within range. The\
disk remains for the duration and can hold up to\
500 pounds. If more weight is placed on it, the spell\
ends, and everything on the disk falls to the ground.\
The disk is immobile while you are within 20 feet\
of it. If you move more than 20 feet away from it, the\
disk follows you so that it remains within 20 feet of\
you. It can move across uneven terrain, up or down\
stairs, slopes and the like, but it can’t cross an ele-\
vation change of 10 feet or more. For example, the\
disk can’t move across a 10-foot-deep pit, nor could\
it leave such a pit if it was created at the bottom.\
If you move more than 100 feet from the disk (typ-\
ically because it can’t move around an obstacle to\
follow you), the spell ends.\

Original layout: [[../../srd-5.2.1/pages/133|page 133]].
