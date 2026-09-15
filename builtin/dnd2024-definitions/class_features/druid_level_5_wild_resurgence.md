---
splat_schema: 1
splat_id: e101c2c8-5dda-5491-9022-5b6a93455595
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_5_wild_resurgence
splat_name: 'Level 5: Wild Resurgence'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_5_wild_resurgence
  parameters:
    conversions:
    - from: spell_slot
      amount: 1
      to: wild_shape_use
      gain: 1
      requires_remaining_wild_shape: 0
      action: none
      limit:
        count: 1
        period: own_turn
    - from: wild_shape_use
      amount: 1
      to: spell_slot
      slot_level: 1
      gain: 1
      action: none
      uses:
        maximum: 1
        recover:
          event: rest.long_rest_completed
          amount: all
  implementation: dnd2024:class_features_druid_level_5_wild_resurgence
splat_contracts:
- id: dnd2024:class_features_druid_level_5_wild_resurgence
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - turn.active
  - resource.conversion_requested
  capabilities:
  - resources
  - spell_slots
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_5_wild_resurgence
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 852151cfb2f30af7450f2163e0905e3a6c1ed7de95c1b15b9f976907b6dce96d
  parent_name: Druid
  line_span:
    first: 3583
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 5: Wild Resurgence Once on each of your turns, if you have no uses of Wild Shape left,
    you can give yourself one use by expending a spell slot (no action required). In addition, you can
    expend one use of Wild Shape (no action required) to give yourself a level 1 spell slot, but you can’t
    do so again until you finish a Long Rest.'
  sha256: 4360248b99a54c6373355bcbed81abd7b9b038a283e3c940e9ff6bbad659fbe4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Wild Resurgence

<!-- SRD page 43 -->

Once on each of your turns, if you have no uses of\
Wild Shape left, you can give yourself one use by ex-\
pending a spell slot (no action required).\
In addition, you can expend one use of Wild Shape\
(no action required) to give yourself a level 1 spell\
slot, but you can’t do so again until you finish a Long\
Rest.\
