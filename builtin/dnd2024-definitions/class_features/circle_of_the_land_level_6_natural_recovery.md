---
splat_schema: 1
splat_id: 40d2509b-e855-5fb9-a1a6-26dd032d01a1
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_6_natural_recovery
splat_name: 'Level 6: Natural Recovery'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/circle_of_the_land_level_6_natural_recovery
  parameters:
    free_cast:
      resource:
        id: land.natural_recovery_cast
        maximum: 1
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
      spell_eligibility:
        prepared_by:
          key: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
        minimum_spell_level: 1
      expend_slot: false
    slot_recovery:
      event: rest.short_completed
      resource:
        id: land.natural_recovery_slots
        maximum: 1
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
      choose_expended_slots: true
      maximum_combined_levels:
        op: ceil
        args:
        - op: divide
          args:
          - var: actor.class_levels.druid
          - 2
      maximum_slot_level: 5
    class_scope:
      class: druid
      minimum_level: 6
  implementation: dnd2024:class_features_circle_of_the_land_level_6_natural_recovery
splat_contracts:
- id: dnd2024:class_features_circle_of_the_land_level_6_natural_recovery
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
  - spellcasting
  - resources
  events:
  - spell.cast
  - rest.short_completed
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 46
  engine_gaps: &id001
  - dnd2024:class_features_circle_of_the_land_level_6_natural_recovery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 76d732b9ea28acd4a7fd5d92e0affa22e563c0aba3ff27cbfaaaf55deb8971d0
  parent_name: Circle of the Land
  line_span:
    first: 3839
    count: 15
splat_clauses:
- key: source_rules
  text: '# Level 6: Natural Recovery You can cast one of the level 1+ spells that you have prepared from
    your Circle Spells feature without expending a spell slot, and you must finish a Long Rest before
    you do so again. In addition, when you finish a Short Rest, you can choose expended spell slots to
    recover. The spell slots can have a combined level that is equal to or less than half your Druid level
    (round up), and none of them can be level 6+. For example, if you’re a level 6 Druid, you can recover
    up to three levels’ worth of spell slots. You can recover a level 3 spell slot, a level 2 and a level
    1 spell slot, or three level 1 spell slots. Once you recover spell slots with this feature, you can’t
    do so again until you finish a Long Rest.'
  sha256: 187990232d3d6ddcff3ec9856709d35261b5d194fc0fc297e0dcf3ae0bbe6627
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 6: Natural Recovery

<!-- SRD page 46 -->

You can cast one of the level 1+ spells that you have\
prepared from your Circle Spells feature without\
expending a spell slot, and you must finish a Long\
Rest before you do so again.\
In addition, when you finish a Short Rest, you can\
choose expended spell slots to recover. The spell\
slots can have a combined level that is equal to or\
less than half your Druid level (round up), and none\
of them can be level 6+. For example, if you’re a level\
6 Druid, you can recover up to three levels’ worth of\
spell slots. You can recover a level 3 spell slot, a level\
2 and a level 1 spell slot, or three level 1 spell slots.\
Once you recover spell slots with this feature, you\
can’t do so again until you finish a Long Rest.\
