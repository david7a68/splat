---
splat_schema: 1
splat_id: 44a28709-8b1c-5801-814a-505dabb20225
splat_kind: spell
splat_key: dnd2024:spells/resilient_sphere
splat_name: Resilient Sphere
splat_origin:
  key: dnd2024:reference/spell/resilient_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: abjuration
  classes:
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a glass sphere
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/resilient_sphere
  implementation: dnd2024:spell_resilient_sphere
  parameters:
    target:
      kind: creature_or_object
      count: 1
      visible_to_caster: false
      maximum_size: large
    unwilling_creature:
      save:
        ability: dexterity
        dc:
          binding: save_dc
        failure:
          enclosed: true
        success:
          effect: none
    enclosure:
      blocks_in_and_out:
      - physical_objects
      - energy
      - spell_effects
      breathing_allowed: true
      sphere_damage_immunity: all
      outside_origin_damage_to_inside: false
      inside_creature_damage_to_outside: false
      weightless: true
      size: just_fits_target
    movement:
      enclosed_creature_action: action
      roll_maximum_speed_fraction:
        numerator: 1
        denominator: 2
      other_creatures_can_pick_up_and_move: true
    disintegrate:
      target_sphere: destroy_sphere
      harm_contents: false
    automatic_enclosure:
      eligible:
      - willing_creature
      - object
splat_contracts:
- id: dnd2024:spell_resilient_sphere
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
  - save
  - sphere_effect
  - movement
  - destruction
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - spatial_areas
  - movement
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: f3fbf310c891fd9d6ae3367e570cf1c59b641866eca58596a694e85dcce9c4fc
  source_path: builtin/srd-5.2.1/spell/resilient_sphere.md
  pages:
  - 158
  engine_gaps: &id001
  - dnd2024:spell_resilient_sphere
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 81e3858b59492867efe9f3e8541fbcb8cef195d025659563888aff39cad3ecca
splat_clauses:
- key: source_rules
  text: '# Resilient Sphere Level 4 Abjuration (Wizard) Casting Time: Action Range: 30 feet Components:
    V, S, M (a glass sphere) Duration: Concentration, up to 1 minute A shimmering sphere encloses a Large
    or smaller creature or object within range. An unwilling creature must succeed on a Dexterity saving
    throw or be enclosed for the duration. Nothing—not physical objects, energy, or other spell effects—can
    pass through the barrier, in or out, though a creature in the sphere can breathe there. The sphere
    is immune to all damage, and a creature or object inside can’t be damaged by attacks or effects originating
    from outside, nor can a creature inside the sphere damage anything outside it. The sphere is weightless
    and just large enough to contain the creature or object inside. An enclosed creature can take an action
    to push against the sphere’s walls and thus roll the sphere at up to half the creature’s Speed. Similarly,
    the globe can be picked up and moved by other creatures. A Disintegrate spell targeting the globe
    destroys it without harming anything inside.'
  sha256: 6a320c61443169e6e307617d3776d317ccbb01a46805acabdc322526ef676d6b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Resilient Sphere

<!-- SRD page 158 -->

Level 4 Abjuration (Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a glass sphere)\
Duration: Concentration, up to 1 minute\
A shimmering sphere encloses a Large or smaller\
creature or object within range. An unwilling crea-\
ture must succeed on a Dexterity saving throw or\
be enclosed for the duration.\
Nothing—not physical objects, energy, or other\
spell effects—can pass through the barrier, in or\
out, though a creature in the sphere can breathe\
there. The sphere is immune to all damage, and a\
creature or object inside can’t be damaged by at-\
tacks or effects originating from outside, nor can a\
creature inside the sphere damage anything outside\
it.\
The sphere is weightless and just large enough to\
contain the creature or object inside. An enclosed\
creature can take an action to push against the\
sphere’s walls and thus roll the sphere at up to half\
the creature’s Speed. Similarly, the globe can be\
picked up and moved by other creatures.\
A Disintegrate spell targeting the globe destroys it\
without harming anything inside.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].
