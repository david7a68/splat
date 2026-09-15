---
splat_schema: 1
splat_id: ab7e6e89-a62a-5318-aa98-fe6a0ece3d0b
splat_kind: spell
splat_key: dnd2024:spells/thaumaturgy
splat_name: Thaumaturgy
splat_origin:
  key: dnd2024:reference/spell/thaumaturgy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 0
  school: transmutation
  classes:
  - cleric
  components:
  - verbal
  concentration: false
  duration: Up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_rule:
  mechanic: dnd2024:spells/thaumaturgy
  implementation: dnd2024:spell_thaumaturgy
  parameters:
    effect_choice:
      count: 1
      within_range: true
    maximum_simultaneous_minute_effects: 3
    effects:
      altered_eyes:
        appearance_choice: caster
        minutes: 1
      booming_voice:
        volume_multiplier_maximum: 3
        minutes: 1
        charisma_intimidation_checks: advantage
      fire_play:
        choice:
        - flicker
        - brighten
        - dim
        - change_color
        minutes: 1
      invisible_hand:
        target:
        - door
        - window
        unlocked_required: true
        choice:
        - open
        - slam_shut
        duration: instantaneous
      phantom_sound:
        sound_choice: caster
        origin: chosen_point_in_range
        duration: instantaneous
      tremors:
        target: ground
        harmless: true
        minutes: 1
splat_contracts:
- id: dnd2024:spell_thaumaturgy
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
  - minor_effect
  - active_effect_count
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - perception
  - choices
  - time
  - checks
splat_conversion:
  status: specified
  category: spell
  source_sha256: c938381ef486e9790c827de73ada622e1418212dbabe7b79086cebecf482db6a
  source_path: builtin/srd-5.2.1/spell/thaumaturgy.md
  pages:
  - 169
  engine_gaps: &id001
  - dnd2024:spell_thaumaturgy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: 4811ca3714f692bf56cbea8dcbf1408aea65b457a5c1e07583b06fa9748edd21
splat_clauses:
- key: source_rules
  text: '# Thaumaturgy Transmutation Cantrip (Cleric) Casting Time: Action Range: 30 feet Components:
    V Duration: Up to 1 minute You manifest a minor wonder within range. You create one of the effects
    below within range. If you cast this spell multiple times, you can have up to three of its 1-minute
    effects active at a time. Altered Eyes. You alter the appearance of your eyes for 1 minute. Booming
    Voice. Your voice booms up to three times as loud as normal for 1 minute. For the duration, you have
    Advantage on Charisma (Intimidation) checks. Fire Play. You cause flames to flicker, brighten, dim,
    or change color for 1 minute. Invisible Hand. You instantaneously cause an unlocked door or window
    to fly open or slam shut. Phantom Sound. You create an instantaneous sound that originates from a
    point of your choice within range, such as a rumble of thunder, the cry of a raven, or ominous whispers.
    Tremors. You cause harmless tremors in the ground for 1 minute.'
  sha256: d15bae79c9a849bf83d4123962fb86f0b826b7747ee59173ebf8903520083947
  status: contract_specified
splat_requires_mechanics: *id001
---

# Thaumaturgy

<!-- SRD page 169 -->

Transmutation Cantrip (Cleric)\
Casting Time: Action\
Range: 30 feet\
Components: V\
Duration: Up to 1 minute\
You manifest a minor wonder within range. You cre-\
ate one of the effects below within range. If you cast\
this spell multiple times, you can have up to three of\
its 1-minute effects active at a time.\
Altered Eyes. You alter the appearance of your\
eyes for 1 minute.\
Booming Voice. Your voice booms up to three\
times as loud as normal for 1 minute. For the dura-\
tion, you have Advantage on Charisma (Intimida-\
tion) checks.\
Fire Play. You cause flames to flicker, brighten,\
dim, or change color for 1 minute.\
Invisible Hand. You instantaneously cause an un-\
locked door or window to fly open or slam shut.\
Phantom Sound. You create an instantaneous\
sound that originates from a point of your choice\
within range, such as a rumble of thunder, the cry of\
a raven, or ominous whispers.\
Tremors. You cause harmless tremors in the\
ground for 1 minute.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]].
