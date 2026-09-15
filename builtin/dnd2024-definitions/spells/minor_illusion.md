---
splat_schema: 1
splat_id: 1fdd3059-d5ea-5f9b-b97a-4262ccdc3324
splat_kind: spell
splat_key: dnd2024:spells/minor_illusion
splat_name: Minor Illusion
splat_origin:
  key: dnd2024:reference/spell/minor_illusion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 149
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: illusion
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - somatic
  - material
  concentration: false
  duration: 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a bit of fleece
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/minor_illusion
  implementation: dnd2024:spell_minor_illusion
  parameters:
    illusion:
      within_spell_range: true
      duration: spell_duration
      end_on_recast: true
      choose_modes: 1
    modes:
      sound:
        volume_minimum: whisper
        volume_maximum: scream
        sound_choice: any
        timing_choices:
        - continuous
        - discrete_over_duration
      image:
        depiction: object
        maximum_cube_side_feet: 5
        prohibit_sensory_effects:
        - sound
        - light
        - smell
        - all_nonvisual
        physical_interaction:
          reveals_illusion: true
          things_pass_through: true
    discern:
      cost: study_action
      check:
        ability: intelligence
        skill: investigation
        dc:
          binding: save_dc
      success: recognize_illusion
      recognized_illusion_appearance: faint
splat_contracts:
- id: dnd2024:spell_minor_illusion
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
  - effect.ended
  - movement.completed
  - save.before
  - save.after
  - spell.cast_pending
  capabilities:
  - spellcasting
  - effects
  - spatial_relationships
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5b0c6e2de6f79039b60c00fd14dd62c9b89729206275a0c75e5b6ab86c73b427
  source_path: builtin/srd-5.2.1/spell/minor_illusion.md
  pages:
  - 149
  engine_gaps: &id001
  - dnd2024:spell_minor_illusion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 8bc7503a99fa9d0ca5e18e5ce0deb1f312fbf1f07b7a9c4cdc9184171760ee18
splat_clauses:
- key: source_rules
  text: '# Minor Illusion Illusion Cantrip (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    30 feet Components: S, M (a bit of fleece) Duration: 1 minute You create a sound or an image of an
    object within range that lasts for the duration. See the descriptions below for the effects of each.
    The illusion ends if you cast this spell again. If a creature takes a Study action to examine the
    sound or image, the creature can determine that it is an illusion with a successful Intelligence (Investigation)
    check against your spell save DC. If a creature discerns the illusion for what it is, the illusion
    becomes faint to the creature. Sound. If you create a sound, its volume can range from a whisper to
    a scream. It can be your voice, someone else’s voice, a lion’s roar, a beating of drums, or any other
    sound you choose. The sound continues unabated throughout the duration, or you can make discrete sounds
    at different times before the spell ends. Image. If you create an image of an object—such as a chair,
    muddy footprints, or a small chest—it must be no larger than a 5-foot Cube. The image can’t create
    sound, light, smell, or any other sensory effect. Physical interaction with the image reveals it to
    be an illusion, since things can pass through it.'
  sha256: c1690ea72eaf8ea4964a2e3859d4b91e2911c590ba1e92e2bcbd143573c5c8f2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Minor Illusion

<!-- SRD page 149 -->

Illusion Cantrip (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: S, M (a bit of fleece)\
Duration: 1 minute\
You create a sound or an image of an object within\
range that lasts for the duration. See the descrip-\
tions below for the effects of each. The illusion ends\
if you cast this spell again.\
If a creature takes a Study action to examine the\
sound or image, the creature can determine that it\
is an illusion with a successful Intelligence (Inves-\
tigation) check against your spell save DC. If a crea-\
ture discerns the illusion for what it is, the illusion\
becomes faint to the creature.\
Sound. If you create a sound, its volume can range\
from a whisper to a scream. It can be your voice,\
someone else’s voice, a lion’s roar, a beating of\
drums, or any other sound you choose. The sound\
continues unabated throughout the duration, or you\
can make discrete sounds at different times before\
the spell ends.\
Image. If you create an image of an object—such\
as a chair, muddy footprints, or a small chest—it\
must be no larger than a 5-foot Cube. The image\
can’t create sound, light, smell, or any other sensory\
effect. Physical interaction with the image reveals it\
to be an illusion, since things can pass through it.\

Original layout: [[../../srd-5.2.1/pages/149|page 149]].
