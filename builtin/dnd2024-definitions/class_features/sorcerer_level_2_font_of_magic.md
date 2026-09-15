---
splat_schema: 1
splat_id: 0af4acc0-690b-54fe-9d36-1d97d04b31ea
splat_kind: rule
splat_key: dnd2024:class_features/sorcerer_level_2_font_of_magic
splat_name: 'Level 2: Font of Magic'
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/sorcerer_level_2_font_of_magic
  parameters:
    resource:
      id: sorcerer.sorcery_points
      maximum:
        var: actor.class_levels.sorcerer
      recovery:
      - event: rest.long_completed
        amount: all
    cannot_exceed_maximum: true
    slot_to_points:
      action: none
      expend_slot_count: 1
      restore_points:
        var: slot.level
    points_to_slot:
      action: bonus_action
      create_slot_count: 1
      options:
      - slot_level: 1
        point_cost: 2
        minimum_sorcerer_level: 2
      - slot_level: 2
        point_cost: 3
        minimum_sorcerer_level: 3
      - slot_level: 3
        point_cost: 5
        minimum_sorcerer_level: 5
      - slot_level: 4
        point_cost: 6
        minimum_sorcerer_level: 7
      - slot_level: 5
        point_cost: 7
        minimum_sorcerer_level: 9
      slot_expires: actor.rest.long_completed
    class_scope:
      class: sorcerer
      minimum_level: 2
  implementation: dnd2024:class_features_sorcerer_level_2_font_of_magic
splat_contracts:
- id: dnd2024:class_features_sorcerer_level_2_font_of_magic
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
  - resources
  events:
  - resource.convert
  - action.bonus
  - rest.long_completed
  capabilities:
  - character_features
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 66
  engine_gaps: &id001
  - dnd2024:class_features_sorcerer_level_2_font_of_magic
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 9ed777bcd40379096c1ba464086e993ebb34537c64aac10adf4b4e5c04833c4c
  parent_name: Sorcerer
  line_span:
    first: 5545
    count: 34
splat_clauses:
- key: source_rules
  text: '# Level 2: Font of Magic You can tap into the wellspring of magic within yourself. This wellspring
    is represented by Sorcery Points, which allow you to create a variety of magical effects. You have
    2 Sorcery Points, and you gain more as you reach higher levels, as shown in the Sorcery Points column
    of the Sorcerer Features table. You can’t have more Sorcery Points than the number shown in the table
    for your level. You regain all expended Sorcery Points when you finish a Long Rest. You can use your
    Sorcery Points to fuel the options below, along with other features, such as Metamagic, that use those
    points. Converting Spell Slots to Sorcery Points. You can expend a spell slot to gain a number of
    Sorcery Points equal to the slot’s level (no action required). Creating Spell Slots. As a Bonus Action,
    you can transform unexpended Sorcery Points into one spell slot. The Creating Spell Slots table shows
    the cost of creating a spell slot of a given level, and it lists the minimum Sorcerer level you must
    be to create a slot. You can create a spell slot no higher than level 5. Any spell slot you create
    with this feature vanishes when you finish a Long Rest. Creating Spell Slots Spell Slot Sorcery Point
    Min. Sorcerer Level Cost Level 1 2 2 2 3 3 3 5 5 4 6 7 5 7 9'
  sha256: 94d5d1386b178cf60473036a0f3fe16e555435eb9d2fb460785ee4c114ab1768
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Font of Magic

<!-- SRD page 66 -->

You can tap into the wellspring of magic within\
yourself. This wellspring is represented by Sorcery\
Points, which allow you to create a variety of magi-\
cal effects.\
You have 2 Sorcery Points, and you gain more as\
you reach higher levels, as shown in the Sorcery\
Points column of the Sorcerer Features table. You\
can’t have more Sorcery Points than the number\
shown in the table for your level. You regain all ex-\
pended Sorcery Points when you finish a Long Rest.\
You can use your Sorcery Points to fuel the op-\
tions below, along with other features, such as\
Metamagic, that use those points.\
Converting Spell Slots to Sorcery Points. You\
can expend a spell slot to gain a number of Sorcery\
Points equal to the slot’s level (no action required).\
Creating Spell Slots. As a Bonus Action, you can\
transform unexpended Sorcery Points into one spell\
slot. The Creating Spell Slots table shows the cost\
of creating a spell slot of a given level, and it lists\
the minimum Sorcerer level you must be to create\
a slot. You can create a spell slot no higher than\
level 5.\
Any spell slot you create with this feature van-\
ishes when you finish a Long Rest.\
Creating Spell Slots\
Spell Slot  Sorcery Point Min. Sorcerer\
Level Cost Level\
1 2 2\
2 3 3\
3 5 5\
4 6 7\
5 7 9\
