---
splat_schema: 1
splat_id: 51b5ed68-3c2c-5058-bcfe-8df2339c6679
splat_kind: spell
splat_key: dnd2024:spells/major_image
splat_name: Major Image
splat_origin:
  key: dnd2024:reference/spell/major_image
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: illusion
  classes:
  - bard
  - sorcerer
  - warlock
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
    feet: 120
  material:
    description: a bit of fleece
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/major_image
  implementation: dnd2024:spell_major_image
  parameters:
    illusion:
      depiction_choices:
      - object
      - creature
      - visible_phenomenon
      maximum_cube_side_feet: 20
      location:
        visible: true
        within_spell_range: true
      senses:
      - visual
      - sound
      - smell
      - temperature
      sensory_qualities: appropriate_to_depiction
      can_deal_damage: false
      can_cause_conditions: false
    manipulation:
      requires_caster_within_spell_range_of_illusion: true
      cost: magic_action
      move_to: spot_within_spell_range
      alter_appearance_for_natural_movement: true
      vary_sound_over_time: true
      conversation: true
    discern:
      physical_interaction:
        automatic: true
        objects_pass_through: true
      study:
        cost: study_action
        ability: intelligence
        skill: investigation
        dc:
          binding: save_dc
      on_discern:
        see_through_image: true
        other_sensory_qualities: faint
    duration_by_slot:
      '3':
        minutes: 10
        concentration: true
      4_plus:
        until: dispelled
        concentration: false
splat_contracts:
- id: dnd2024:spell_major_image
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.cast_completed
  - movement.completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: 52067a3cbe6ab42d951af592b4dc57321a7d02605a8f6bdeec234336963e16d0
  source_path: builtin/srd-5.2.1/spell/major_image.md
  pages:
  - 147
  engine_gaps: &id001
  - dnd2024:spell_major_image
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: f42dec450dd5c9658923c59652f18730d6b7df595b60ea5bb9919e10170465c2
splat_clauses:
- key: source_rules
  text: '# Major Image Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    120 feet Components: V, S, M (a bit of fleece) Duration: Concentration, up to 10 minutes You create
    the image of an object, a creature, or some other visible phenomenon that is no larger than a 20-foot
    Cube. The image appears at a spot that you can see within range and lasts for the duration. It seems
    real, including sounds, smells, and temperature appropriate to the thing depicted, but it can’t deal
    damage or cause conditions. If you are within range of the illusion, you can take a Magic action to
    cause the image to move to any other spot within range. As the image changes location, you can alter
    its appearance so that its movements appear natural for the image. For example, if you create an image
    of a creature and move it, you can alter the image so that it appears to be walking. Similarly, you
    can cause the illusion to make different sounds at different times, even making it carry on a conversation,
    for example. Physical interaction with the image reveals it to be an illusion, for things can pass
    through it. A creature that takes a Study action to examine the image can determine that it is an
    illusion with a successful Intelligence (Investigation) check against your spell save DC. If a creature
    discerns the illusion for what it is, the creature can see through the image, and its other sensory
    qualities become faint to the creature. Using a Higher-Level Spell Slot. The spell lasts until dispelled,
    without requiring Concentration, if cast with a level 4+ spell slot.'
  sha256: 58b53803cfd24aac8cc12ccac1fe2367f17d52cb14735a312e0322dcec5a8878
  status: contract_specified
splat_requires_mechanics: *id001
---

# Major Image

<!-- SRD page 147 -->

Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a bit of fleece)\
Duration: Concentration, up to 10 minutes\
You create the image of an object, a creature, or\
some other visible phenomenon that is no larger\
than a 20-foot Cube. The image appears at a spot\
that you can see within range and lasts for the du-\
ration. It seems real, including sounds, smells, and\
temperature appropriate to the thing depicted, but\
it can’t deal damage or cause conditions.\
If you are within range of the illusion, you can\
take a Magic action to cause the image to move to\
any other spot within range. As the image changes\
location, you can alter its appearance so that its\
movements appear natural for the image. For exam-\
ple, if you create an image of a creature and move\
it, you can alter the image so that it appears to be\
walking. Similarly, you can cause the illusion to\
make different sounds at different times, even mak-\
ing it carry on a conversation, for example.\
Physical interaction with the image reveals it to\
be an illusion, for things can pass through it. A crea-\
ture that takes a Study action to examine the image\
can determine that it is an illusion with a success-\
ful Intelligence (Investigation) check against your\
spell save DC. If a creature discerns the illusion for\
what it is, the creature can see through the image,\
and its other sensory qualities become faint to the\
creature.\
Using a Higher-Level Spell Slot. The spell lasts\
until dispelled, without requiring Concentration, if\
cast with a level 4+ spell slot.\

Original layout: [[../../srd-5.2.1/pages/147|page 147]].
