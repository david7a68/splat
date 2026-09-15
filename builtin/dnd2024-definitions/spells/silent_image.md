---
splat_schema: 1
splat_id: cc8b21ff-926e-5637-9552-80b7ff746200
splat_kind: spell
splat_key: dnd2024:spells/silent_image
splat_name: Silent Image
splat_origin:
  key: dnd2024:reference/spell/silent_image
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: illusion
  classes:
  - bard
  - sorcerer
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
    feet: 60
  material:
    description: a bit of fleece
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/silent_image
  implementation: dnd2024:spell_silent_image
  parameters:
    image:
      kind_choice:
      - object
      - creature
      - visible_phenomenon
      maximum_cube_feet: 15
      senses:
      - vision
      sound: false
      smell: false
      other_sensory_effects: false
    move:
      action: magic
      destination: any_point_in_range
      appearance_alteration: natural_movement
    discern:
      physical_interaction: automatic
      study:
        ability: intelligence
        skill: investigation
        dc:
          binding: save_dc
      after_discern: see_through
splat_contracts:
- id: dnd2024:spell_silent_image
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
  - image
  - discernment_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - saving_throws
  - movement
  - perception
  - inventory
  - choices
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: b0b90eee24cbcfe29f58b625bdb5df3b45475724b1e529d0cb7efe921ef6b6d1
  source_path: builtin/srd-5.2.1/spell/silent_image.md
  pages:
  - 162
  engine_gaps: &id001
  - dnd2024:spell_silent_image
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 65b43ebf18d211139b2c16427286ea73d2afcb94af20c440b86b53dca0d0fe41
splat_clauses:
- key: source_rules
  text: '# Silent Image Level 1 Illusion (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60 feet
    Components: V, S, M (a bit of fleece) Duration: Concentration, up to 10 minutes You create the image
    of an object, a creature, or some other visible phenomenon that is no larger than a 15-foot Cube.
    The image appears at a spot within range and lasts for the duration. The image is purely visual; it
    isn’t accompanied by sound, smell, or other sensory effects. As a Magic action, you can cause the
    image to move to any spot within range. As the image changes location, you can alter its appearance
    so that its movements appear natural for the image. For example, if you create an image of a creature
    and move it, you can alter the image so that it appears to be walking. Physical interaction with the
    image reveals it to be an illusion, since things can pass through it. A creature that takes a Study
    action to examine the image can determine that it is an illusion with a successful Intelligence (Investigation)
    check against your spell save DC. If a creature discerns the illusion for what it is, the creature
    can see through the image.'
  sha256: b137860922d2c6039b5aa21b0993d79c9f84a2fd8b275f5f7003a2aad62a4d74
  status: contract_specified
splat_requires_mechanics: *id001
---

# Silent Image

<!-- SRD page 162 -->

Level 1 Illusion (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a bit of fleece)\
Duration: Concentration, up to 10 minutes\
You create the image of an object, a creature, or\
some other visible phenomenon that is no larger\
than a 15-foot Cube. The image appears at a spot\
within range and lasts for the duration. The image\
is purely visual; it isn’t accompanied by sound,\
smell, or other sensory effects.\
As a Magic action, you can cause the image\
to move to any spot within range. As the image\
changes location, you can alter its appearance so\
that its movements appear natural for the image.\
For example, if you create an image of a creature\
and move it, you can alter the image so that it ap-\
pears to be walking.\
Physical interaction with the image reveals it to\
be an illusion, since things can pass through it. A\
creature that takes a Study action to examine the\
image can determine that it is an illusion with a\
successful Intelligence (Investigation) check against\
your spell save DC. If a creature discerns the illu-\
sion for what it is, the creature can see through the\
image.\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].
