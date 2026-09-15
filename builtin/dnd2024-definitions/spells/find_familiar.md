---
splat_schema: 1
splat_id: d87a20fc-113d-55bc-8803-7c508c15cc5c
splat_kind: spell
splat_key: dnd2024:spells/find_familiar
splat_name: Find Familiar
splat_origin:
  key: dnd2024:reference/spell/find_familiar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 130
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
  duration: Instantaneous
  casting_time: 1 hour or Ritual
  ritual: true
  range:
    feet: 10
  material:
    description: burning incense worth 10+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/find_familiar
  implementation: dnd2024:spell_find_familiar
  parameters:
    targeting:
      kind: location
      unoccupied: true
      within_range: true
    form:
      chooser: caster
      eligible:
        type: beast
        challenge_rating: 0
      examples:
      - bat
      - cat
      - frog
      - hawk
      - lizard
      - octopus
      - owl
      - rat
      - raven
      - spider
      - weasel
      statistics: chosen_form
      replace_type_choice:
      - celestial
      - fey
      - fiend
    behavior:
      independent: true
      obeys_commands: true
      allies:
      - caster
      - caster_allies
      initiative: own_roll
      turn: own
      attack_allowed: false
      other_actions: normal
    connection:
      telepathy_range_feet: 100
      shared_senses:
        economy: bonus_action
        duration: until_caster_next_turn_start
        channels:
        - vision
        - hearing
        special_senses: included
    deliver_spell:
      spell_range: touch
      familiar_distance_to_caster_max_feet: 100
      familiar_economy: reaction
      timing: caster_casts_spell
      delivery: familiar_touch
    disappearance:
      zero_hp: disappears_until_recast
      dismissal:
        economy: magic_action
        options:
        - temporary_pocket_dimension
        - permanent
      return:
        requires: temporarily_dismissed
        economy: magic_action
        unoccupied: true
        distance_feet: 30
      on_zero_hp_or_pocket_dimension: leave_worn_and_carried_items_in_space
    limit:
      per_caster: 1
      recast_with_existing: adopt_new_eligible_form
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_find_familiar
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
    locations:
      type: location_list
      required: true
    forms:
      type: definition_list
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - participants
  - locations
  events:
  - spell.cast_completed
  - health.changed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - participant_lifecycle
  - forms
  - perception
  - controlled_participants
splat_conversion:
  status: specified
  category: spell
  source_sha256: 3dcfbef9417a26a3587314b6d4f66823d68fcdb620e26ff7b99b160abc0370be
  source_path: builtin/srd-5.2.1/spell/find_familiar.md
  pages:
  - 130
  engine_gaps: &id001
  - dnd2024:spell_find_familiar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 0f156165dfb04b59d6b140212d21370f9abd7d9ad4c5b04b01a8fbfd65ecab09
splat_clauses:
- key: source_rules
  text: '# Find Familiar Level 1 Conjuration (Wizard) Casting Time: 1 hour or Ritual Range: 10 feet Components:
    V, S, M (burning incense worth 10+ GP, which the spell consumes) Duration: Instantaneous You gain
    the service of a familiar, a spirit that takes an animal form you choose: Bat, Cat, Frog, Hawk, Lizard,
    Octopus, Owl, Rat, Raven, Spider, Weasel, or another Beast that has a Challenge Rating of 0. Appearing
    in an unoccupied space within range, the familiar has the statistics of the chosen form (see “Monsters”),
    though it is a Celestial, Fey, or Fiend (your choice) instead of a Beast. Your familiar acts independently
    of you, but it obeys your commands. Telepathic Connection. While your familiar is within 100 feet
    of you, you can communicate with it telepathically. Additionally, as a Bonus Action, you can see through
    the familiar’s eyes and hear what it hears until the start of your next turn, gaining the benefits
    of any special senses it has. Finally, when you cast a spell with a range of touch, your familiar
    can deliver the touch. Your familiar must be within 100 feet of you, and it must take a Reaction to
    deliver the touch when you cast the spell. Combat. The familiar is an ally to you and your allies.
    It rolls its own Initiative and acts on its own turn. A familiar can’t attack, but it can take other
    actions as normal. Disappearance of the Familiar. When the familiar drops to 0 Hit Points, it disappears.
    It reappears after you cast this spell again. As a Magic action, you can temporarily dismiss the familiar
    to a pocket dimension. Alternatively, you can dismiss it forever. As a Magic action while it is temporarily
    dismissed, you can cause it to reappear in an unoccupied space within 30 feet of you. Whenever the
    familiar drops to 0 Hit Points or disappears into the pocket dimension, it leaves behind in its space
    anything it was wearing or carrying. One Familiar Only. You can’t have more than one familiar at a
    time. If you cast this spell while you have a familiar, you instead cause it to adopt a new eligible
    form.'
  sha256: 8244de8bd0dd43c6e3ab3366d3bdab60dbbb98c8bf1c31f935ae913a056d623a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Find Familiar

<!-- SRD page 130 -->

Level 1 Conjuration (Wizard)\
Casting Time: 1 hour or Ritual\
Range: 10 feet\
Components: V, S, M (burning incense worth 10+ GP,\
which the spell consumes)\
Duration: Instantaneous\
You gain the service of a familiar, a spirit that takes\
an animal form you choose: Bat, Cat, Frog, Hawk,\
Lizard, Octopus, Owl, Rat, Raven, Spider, Weasel,\
or another Beast that has a Challenge Rating of 0.\
Appearing in an unoccupied space within range, the\
familiar has the statistics of the chosen form (see\
“Monsters”), though it is a Celestial, Fey, or Fiend\
(your choice) instead of a Beast. Your familiar acts\
independently of you, but it obeys your commands.\
Telepathic Connection. While your familiar is\
within 100 feet of you, you can communicate with it\
telepathically. Additionally, as a Bonus Action, you\
can see through the familiar’s eyes and hear what it\
hears until the start of your next turn, gaining the\
benefits of any special senses it has.\
Finally, when you cast a spell with a range of\
touch, your familiar can deliver the touch. Your fa-\
miliar must be within 100 feet of you, and it must\
take a Reaction to deliver the touch when you cast\
the spell.\
Combat. The familiar is an ally to you and your\
allies. It rolls its own Initiative and acts on its own\
turn. A familiar can’t attack, but it can take other\
actions as normal.\
Disappearance of the Familiar. When the famil-\
iar drops to 0 Hit Points, it disappears. It reappears\
after you cast this spell again. As a Magic action,\
you can temporarily dismiss the familiar to a pocket\
dimension. Alternatively, you can dismiss it forever.\
As a Magic action while it is temporarily dismissed,\
you can cause it to reappear in an unoccupied space\
within 30 feet of you. Whenever the familiar drops\
to 0 Hit Points or disappears into the pocket dimen-\
sion, it leaves behind in its space anything it was\
wearing or carrying.\
One Familiar Only. You can’t have more than one\
familiar at a time. If you cast this spell while you\
have a familiar, you instead cause it to adopt a new\
eligible form.\

Original layout: [[../../srd-5.2.1/pages/130|page 130]].
