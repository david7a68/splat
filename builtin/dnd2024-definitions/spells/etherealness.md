---
splat_schema: 1
splat_id: c13d9fe6-2fcf-51a6-a8a0-c3a3e669942b
splat_kind: spell
splat_key: dnd2024:spells/etherealness
splat_name: Etherealness
splat_origin:
  key: dnd2024:reference/spell/etherealness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 128
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: conjuration
  classes:
  - bard
  - cleric
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Up to 8 hours
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/etherealness
  implementation: dnd2024:spell_etherealness
  parameters:
    target:
      kind: self
      count: 1
      visible_to_caster: false
    plane:
      destination: border_ethereal
      must_border_origin: true
      instant_end_if_origin:
      - ethereal
      - nonbordering_plane
    movement:
      any_direction: true
      vertical_additional_cost_per_foot: 1
    perception:
      origin_plane: true
      origin_plane_max_feet: 60
      origin_plane_color: gray
    interaction:
      can_affect_and_be_affected_by: ethereal_entities_only
      nonethereal_perception_and_interaction: requires_feature
    end:
      return: corresponding_origin_plane_space
      occupied: shunt_nearest_unoccupied
      force_damage_per_shunted_foot: 2
    upcast_targets:
      per_slot_above: 7
      maximum_per_level: 3
      includes_caster: true
      willing: true
      within_caster_feet: 10
splat_contracts:
- id: dnd2024:spell_etherealness
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
  - planar_transition
  - movement_and_perception_effect
  - return_transition
  - shunt_damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - movement
  - perception
  - time
splat_conversion:
  status: specified
  category: spell
  source_sha256: af47e75e6ca8ed88d4b83b636baf630d2899d7e000289ada523577ce8dd3fa02
  source_path: builtin/srd-5.2.1/spell/etherealness.md
  pages:
  - 128
  engine_gaps: &id001
  - dnd2024:spell_etherealness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 26cdea134148c350a6ac8bb06dadc0f43f87866bcc53d99a96c313f0753f3da8
splat_clauses:
- key: source_rules
  text: '# Etherealness Level 7 Conjuration (Bard, Cleric, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: Self Components: V, S Duration: Up to 8 hours You step into the border regions of the Ethereal
    Plane, where it overlaps with your current plane. You remain in the Border Ethereal for the duration.
    During this time, you can move in any direction. If you move up or down, every foot of movement costs
    an extra foot. You can perceive the plane you left, which looks gray, and you can’t see anything there
    more than 60 feet away. While on the Ethereal Plane, you can affect and be affected only by creatures,
    objects, and effects on that plane. Creatures that aren’t on the Ethereal Plane can’t perceive or
    interact with you unless a feature gives them the ability to do so. When the spell ends, you return
    to the plane you left in the spot that corresponds to your space in the Border Ethereal. If you appear
    in an occupied space, you are shunted to the nearest unoccupied space and take Force damage equal
    to twice the number of feet you are moved. This spell ends instantly if you cast it while you are
    on the Ethereal Plane or a plane that doesn’t border it, such as one of the Outer Planes. Using a
    Higher-Level Spell Slot. You can target up to three willing creatures (including yourself) for each
    spell slot level above 7. The creatures must be within 10 feet of you when you cast the spell.'
  sha256: 8b87c393f92501c7f555e5666947efcc0553bbe448d55e710e1030a5deda14ab
  status: contract_specified
splat_requires_mechanics: *id001
---

# Etherealness

<!-- SRD page 128 -->

Level 7 Conjuration (Bard, Cleric, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Up to 8 hours\
You step into the border regions of the Ethereal\
Plane, where it overlaps with your current plane.\
You remain in the Border Ethereal for the duration.\
During this time, you can move in any direction. If\
you move up or down, every foot of movement costs\
an extra foot. You can perceive the plane you left,\
which looks gray, and you can’t see anything there\
more than 60 feet away.\
While on the Ethereal Plane, you can affect and\
be affected only by creatures, objects, and effects\
on that plane. Creatures that aren’t on the Ethereal\
Plane can’t perceive or interact with you unless a\
feature gives them the ability to do so.\
When the spell ends, you return to the plane you\
left in the spot that corresponds to your space in the\
Border Ethereal. If you appear in an occupied space,\
you are shunted to the nearest unoccupied space\
and take Force damage equal to twice the number\
of feet you are moved.\
This spell ends instantly if you cast it while you\
are on the Ethereal Plane or a plane that doesn’t\
border it, such as one of the Outer Planes.\
Using a Higher-Level Spell Slot. You can target up\
to three willing creatures (including yourself) for\
each spell slot level above 7. The creatures must be\
within 10 feet of you when you cast the spell.\

Original layout: [[../../srd-5.2.1/pages/128|page 128]].
