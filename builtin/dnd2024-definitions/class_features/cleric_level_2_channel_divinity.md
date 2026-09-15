---
splat_schema: 1
splat_id: 938946e7-6520-5556-87f0-621c883cc5ce
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_2_channel_divinity
splat_name: 'Level 2: Channel Divinity'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_2_channel_divinity
  parameters:
    resource:
      id: cleric.channel_divinity
      maximum:
        table:
          key: dnd2024:tables/cleric_progression
        row:
          var: actor.class_levels.cleric
        column: channel_divinity
      recovery:
      - event: rest.short_completed
        amount: 1
      - event: rest.long_completed
        amount: all
    cost_per_use: 1
    choose_effect: 1
    dc:
      op: add
      args:
      - 8
      - var: actor.abilities.wisdom.modifier
      - var: actor.proficiency_bonus
    options:
      divine_spark:
        action: magic
        requires_item: holy_symbol
        target:
          type: creature
          self: false
          visible: true
          range_feet: 30
        amount:
          op: add
          args:
          - dice:
              count_by_cleric_level:
                '2': 1
                '7': 2
                '13': 3
                '18': 4
              sides: 8
          - var: actor.abilities.wisdom.modifier
        choice:
        - heal
        - damage
        damage:
          types_choice:
          - necrotic
          - radiant
          save: constitution
          success_multiplier: 0.5
          rounding: down
      turn_undead:
        action: magic
        requires_item: holy_symbol
        targets:
          type: undead
          selection: actor_choice
          range_feet: 30
        save: wisdom
        failure:
          conditions:
          - frightened
          - incapacitated
          duration:
            minutes: 1
          on_target_turn: move_as_far_as_possible_from_actor
          ends_on:
          - target.damage_taken
          - actor.incapacitated
          - actor.died
    class_scope:
      class: cleric
      minimum_level: 2
  implementation: dnd2024:class_features_cleric_level_2_channel_divinity
splat_contracts:
- id: dnd2024:class_features_cleric_level_2_channel_divinity
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
  - action.magic
  - rest.short_completed
  - rest.long_completed
  - turn.start
  - damage.taken
  - condition.applied
  - participant.died
  capabilities:
  - character_features
  - resources
  - damage
  - conditions
  - proficiencies
  - d20_tests
  - healing
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 37
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_2_channel_divinity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 3c14f8c7bbdc5bf78a1d376bfbaa9eb588a98eb38e9efb68e58ad63bf77de0ca
  parent_name: Cleric
  line_span:
    first: 3050
    count: 40
splat_clauses:
- key: source_rules
  text: '# Level 2: Channel Divinity You can channel divine energy directly from the Outer Planes to fuel
    magical effects. You start with two such effects: Divine Spark and Turn Undead, each of which is described
    below. Each time you use this class’s Channel Divinity, choose which Channel Divinity effect from
    this class to create. You gain additional effect options at higher Cleric levels. You can use this
    class’s Channel Divinity twice. You regain one of its expended uses when you finish a Short Rest,
    and you regain all expended uses when you finish a Long Rest. You gain additional uses when you reach
    certain Cleric levels, as shown in the Channel Divinity column of the Cleric Features table. If a
    Channel Divinity effect requires a saving throw, the DC equals the spell save DC from this class’s
    Spellcasting feature. Divine Spark. As a Magic action, you point your Holy Symbol at another creature
    you can see within 30 feet of yourself and focus divine energy at it. Roll 1d8 and add your Wisdom
    modifier. You either restore Hit Points to the creature equal to that total or force the creature
    to make a Constitution saving throw. On a failed save, the creature takes Necrotic or Radiant damage
    (your choice) equal to that total. On a successful save, the creature takes half as much damage (round
    down). You roll an additional d8 when you reach Cleric levels 7 (2d8), 13 (3d8), and 18 (4d8). Turn
    Undead. As a Magic action, you present your Holy Symbol and censure Undead creatures. Each Undead
    of your choice within 30 feet of you must make a Wisdom saving throw. If the creature fails its save,
    it has the Frightened and Incapacitated conditions for 1 minute. For that duration, it tries to move
    as far from you as it can on its turns. This effect ends early on the creature if it takes any damage,
    if you have the Incapacitated condition, or if you die.'
  sha256: 5dbde65f0e6937c4198ec3082e5f08294199a7bc2b8e0acf3ae024c0cb4e2da6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Channel Divinity

<!-- SRD page 37 -->

You can channel divine energy directly from the\
Outer Planes to fuel magical effects. You start with\
two such effects: Divine Spark and Turn Undead,\
each of which is described below. Each time you use\
this class’s Channel Divinity, choose which Channel\
Divinity effect from this class to create. You gain ad-\
ditional effect options at higher Cleric levels.\
You can use this class’s Channel Divinity twice.\
You regain one of its expended uses when you finish\
a Short Rest, and you regain all expended uses when\
you finish a Long Rest. You gain additional uses\
when you reach certain Cleric levels, as shown in\
the Channel Divinity column of the Cleric Features\
table.\
If a Channel Divinity effect requires a saving\
throw, the DC equals the spell save DC from this\
class’s Spellcasting feature.\
Divine Spark. As a Magic action, you point your\
Holy Symbol at another creature you can see within\
30 feet of yourself and focus divine energy at it.\
Roll 1d8 and add your Wisdom modifier. You either\
restore Hit Points to the creature equal to that total\
or force the creature to make a Constitution saving\
throw. On a failed save, the creature takes Necrotic\
or Radiant damage (your choice) equal to that to-\
tal. On a successful save, the creature takes half as\
much damage (round down).\
You roll an additional d8 when you reach Cleric\
levels 7 (2d8), 13 (3d8), and 18 (4d8).\
Turn Undead. As a Magic action, you present your\
Holy Symbol and censure Undead creatures. Each\
Undead of your choice within 30 feet of you must\
make a Wisdom saving throw. If the creature fails\
its save, it has the Frightened and Incapacitated\
conditions for 1 minute. For that duration, it tries\
to move as far from you as it can on its turns. This\
effect ends early on the creature if it takes any dam-\
age, if you have the Incapacitated condition, or if\
you die.\
