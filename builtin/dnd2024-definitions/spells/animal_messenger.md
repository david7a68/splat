---
splat_schema: 1
splat_id: 09a6df3c-5bd5-5498-8636-eb626e962461
splat_kind: spell
splat_key: dnd2024:spells/animal_messenger
splat_name: Animal Messenger
splat_origin:
  key: dnd2024:reference/spell/animal_messenger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 108
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: enchantment
  classes:
  - bard
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 24 hours
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 30
  material:
    description: a morsel of food
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/animal_messenger
  implementation: dnd2024:spell_animal_messenger
  parameters:
    targets:
      kind: creature
      maximum: 1
      within_spell_range: true
      visible: true
      size: tiny
      creature_type: beast
    save:
      ability: charisma
      dc:
        binding: save_dc
      failure:
        effect: deliver_message
      success:
        effect: none
    automatic_save_success:
      challenge_rating_not_equal: 0
    message:
      word_limit: 25
      destination:
        visited_by_caster: true
      recipient:
        match: caster_general_description
      delivery: mimic_caster_communication
    travel:
      duration_hours:
        op: add
        args:
        - 24
        - op: multiply
          args:
          - 48
          - op: subtract
            args:
            - binding: cast_level
            - 2
      miles_per_24_hours:
        flying: 50
        otherwise: 25
      rate_precision: approximate
      on_arrival: deliver_to_matching_creature
      on_duration_expired_before_arrival:
        message: lost
        beast_destination: casting_location
      on_arrival_duration_remaining: spell_duration_remaining
splat_contracts:
- id: dnd2024:spell_animal_messenger
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    campaign:
      required: false
      type: campaign
    save_dc:
      type: integer
      required: true
    message:
      required: false
      type: text
    choices:
      required: false
      type: choice
    items:
      required: false
      type: item_list
  outputs:
  - effects
  - knowledge
  - items
  events:
  - spell.cast_completed
  - save.before
  - save.after
  - time.elapsed
  capabilities:
  - spellcasting
  - effects
  - dice
  - saving_throws
  - knowledge
  - inventory
  - behavior_constraints
splat_conversion:
  status: specified
  category: spell
  source_sha256: 7d535947ad4e02ad9914329816896930541204fb9f13bf204a48404c24ebe3e6
  source_path: builtin/srd-5.2.1/spell/animal_messenger.md
  pages:
  - 108
  engine_gaps: &id001
  - dnd2024:spell_animal_messenger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: ec9c4979fd1bdb229a95db1140dec15cf61e21be3f08dd0ed7652b10f935268d
splat_clauses:
- key: source_rules
  text: '# Animal Messenger Level 2 Enchantment (Bard, Druid, Ranger) Casting Time: Action or Ritual Range:
    30 feet Components: V, S, M (a morsel of food) Duration: 24 hours A Tiny Beast of your choice that
    you can see within range must succeed on a Charisma saving throw, or it attempts to deliver a message
    for you (if the target’s Challenge Rating isn’t 0, it automatically succeeds). You specify a location
    you have visited and a recipient who matches a general description, such as “a person dressed in the
    uniform of the town guard” or “a red-haired dwarf wearing a pointed hat.” You also communicate a message
    of up to twenty-five words. The Beast travels for the duration toward the specified location, covering
    about 25 miles per 24 hours or 50 miles if the Beast can fly. When the Beast arrives, it delivers
    your message to the creature that you described, mimicking your communication. If the Beast doesn’t
    reach its destination before the spell ends, the message is lost, and the Beast returns to where you
    cast the spell. Using a Higher-Level Spell Slot. The spell’s duration increases by 48 hours for each
    spell slot level above 2.'
  sha256: 88ce7fc2433a1160e6709905ec3deed42aa9bdacce05bf1ccdf9861dae1ba133
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animal Messenger

<!-- SRD page 108 -->

Level 2 Enchantment (Bard, Druid, Ranger)\
Casting Time: Action or Ritual\
Range: 30 feet\
Components: V, S, M (a morsel of food)\
Duration: 24 hours\
A Tiny Beast of your choice that you can see within\
range must succeed on a Charisma saving throw,\
or it attempts to deliver a message for you (if the\
target’s Challenge Rating isn’t 0, it automatically\
succeeds). You specify a location you have visited\
and a recipient who matches a general descrip-\
tion, such as “a person dressed in the uniform of\
the town guard” or “a red-haired dwarf wearing a\
pointed hat.” You also communicate a message of\
up to twenty-five words. The Beast travels for the\
duration toward the specified location, covering\
about 25 miles per 24 hours or 50 miles if the Beast\
can fly.\
When the Beast arrives, it delivers your message\
to the creature that you described, mimicking your\
communication. If the Beast doesn’t reach its desti-\
nation before the spell ends, the message is lost, and\
the Beast returns to where you cast the spell.\
Using a Higher-Level Spell Slot. The spell’s dura-\
tion increases by 48 hours for each spell slot level\
above 2.\

Original layout: [[../../srd-5.2.1/pages/108|page 108]].
