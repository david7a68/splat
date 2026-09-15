---
splat_schema: 1
splat_id: 459b4c2e-7c8a-5fef-9332-ee83c0b32ffd
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_3_channel_divinity
splat_name: 'Level 3: Channel Divinity'
splat_origin:
  key: dnd2024:reference/pages/054
splat_sources:
- title: SRD 5.2.1
  locator: pp. 54, 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_3_channel_divinity
  parameters:
    resource:
      id: paladin.channel_divinity
      maximum:
        by_paladin_level:
          '3': 2
          '11': 3
      recovery:
      - event: rest.short_completed
        amount: 1
      - event: rest.long_completed
        amount: all
    choose_effect_each_use: 1
    cost_each_use: 1
    save_dc:
      op: add
      args:
      - 8
      - var: actor.abilities.charisma.modifier
      - var: actor.proficiency_bonus
    divine_sense:
      action: bonus_action
      duration:
        minutes: 10
      ends_on:
      - actor.incapacitated
      radius_feet: 60
      detect_creatures:
        types:
        - celestial
        - fiend
        - undead
        reveal:
        - location
        - creature_type
      detect_objects_and_places:
        tags_any:
        - consecrated
        - desecrated
        reveal:
        - presence
    class_scope:
      class: paladin
      minimum_level: 3
  implementation: dnd2024:class_features_paladin_level_3_channel_divinity
splat_contracts:
- id: dnd2024:class_features_paladin_level_3_channel_divinity
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - action.bonus
  - rest.short_completed
  - rest.long_completed
  - condition.applied
  - participant.moved
  capabilities:
  - character_features
  - resources
  - proficiencies
  - d20_tests
  - senses
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 54
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_3_channel_divinity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: acef17f09bf96d574f711411e03c3ae0a3100a3d4756729300648b4b439de17f
  parent_name: Paladin
  line_span:
    first: 4557
    count: 37
splat_clauses:
- key: source_rules
  text: '# Level 3: Channel Divinity You can channel divine energy directly from the Outer Planes, using
    it to fuel magical effects. You start with one such effect: Divine Sense, which is described below.
    Other Paladin features give additional Channel Divinity effect options. Each time you use this class’s
    Channel Divinity, you choose which effect from this class to create. You can use this class’s Channel
    Divinity twice. You regain one of its expended uses when you finish a Short Rest, and you regain all
    expended uses when you finish a Long Rest. You gain an additional use when you reach Paladin level
    11. BrEakinG your oatH A Paladin tries to hold to the highest standards of conduct, but even the most
    dedicated are fallible. Sometimes a Paladin transgresses their oath. A Paladin who has broken a vow
    typically seeks absolution, spending an all-night vigil as a sign of penitence or undertaking a fast.
    After a rite of forgiveness, the Paladin starts fresh. If your Paladin unrepentantly violates their
    oath, talk to your GM. Your Paladin should probably take a more appropriate subclass or even abandon
    the class and adopt another one. If a Channel Divinity effect requires a saving throw, the DC equals
    the spell save DC from this class’s Spellcasting feature. Divine Sense. As a Bonus Action, you can
    open your awareness to detect Celestials, Fiends, and Undead. For the next 10 minutes or until you
    have the Incapacitated condition, you know the location of any creature of those types within 60 feet
    of yourself, and you know its creature type. Within the same radius, you also detect the presence
    of any place or object that has been consecrated or desecrated, as with the Hallow spell.'
  sha256: 9644a7978adc879b75b09daa4a821bb3cab2cdf77d05077f59209dbc25256329
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Channel Divinity

<!-- SRD page 54 -->

You can channel divine energy directly from the\
Outer Planes, using it to fuel magical effects. You\
start with one such effect: Divine Sense, which is\
described below. Other Paladin features give addi-\
tional Channel Divinity effect options. Each time\
you use this class’s Channel Divinity, you choose\
which effect from this class to create.\
You can use this class’s Channel Divinity twice.\
You regain one of its expended uses when you finish\
a Short Rest, and you regain all expended uses when\
you finish a Long Rest. You gain an additional use\
when you reach Paladin level 11.\
BrEakinG your oatH\
A Paladin tries to hold to the highest standards of\
conduct, but even the most dedicated are fallible.\
Sometimes a Paladin transgresses their oath.\
A Paladin who has broken a vow typically seeks\
absolution, spending an all-night vigil as a sign of pen-\
itence or undertaking a fast. After a rite of forgive-\
ness, the Paladin starts fresh.\
If your Paladin unrepentantly violates their oath,\
talk to your GM. Your Paladin should probably take a\
more appropriate subclass or even abandon the class\
and adopt another one.\

<!-- SRD page 55 -->

If a Channel Divinity effect requires a saving\
throw, the DC equals the spell save DC from this\
class’s Spellcasting feature.\
Divine Sense. As a Bonus Action, you can open\
your awareness to detect Celestials, Fiends, and\
Undead. For the next 10 minutes or until you have\
the Incapacitated condition, you know the location\
of any creature of those types within 60 feet of\
yourself, and you know its creature type. Within\
the same radius, you also detect the presence of any\
place or object that has been consecrated or dese-\
crated, as with the Hallow spell.\
