---
splat_schema: 1
splat_id: 8f3a0f54-5093-5b76-bec2-1c9296d2b8ea
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_9_indomitable
splat_name: 'Level 9: Indomitable'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_9_indomitable
  parameters:
    trigger: self_saving_throw_failed
    optional_reroll: true
    reroll_bonus:
      stat: classes.fighter.level
    must_use_new_roll: true
    uses:
      maximum_by_fighter_level:
        '9': 1
        '13': 2
        '17': 3
      recover:
        event: rest.long_rest_completed
        amount: all_expended
  implementation: dnd2024:class_features_fighter_level_9_indomitable
splat_contracts:
- id: dnd2024:class_features_fighter_level_9_indomitable
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - resource_changes
  events:
  - saving_throw.failed
  capabilities:
  - saving_throws
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_9_indomitable
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 6a3600d80e20fb05a64838e715cf5f21db0a4e99c28f9d24f10ff27b7516e2bf
  parent_name: Fighter
  line_span:
    first: 4020
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 9: Indomitable If you fail a saving throw, you can reroll it with a bonus equal to your
    Fighter level. You must use the new roll, and you can’t use this feature again until you finish a
    Long Rest. You can use this feature twice before a Long Rest starting at level 13 and three times
    before a Long Rest starting at level 17.'
  sha256: 2a3f7eb1b8e50babafdf02829ca4093ea9113ba2a7938c3ac3e2f219ffb05311
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 9: Indomitable

<!-- SRD page 48 -->

If you fail a saving throw, you can reroll it with a\
bonus equal to your Fighter level. You must use the\
new roll, and you can’t use this feature again until\
you finish a Long Rest.\
You can use this feature twice before a Long Rest\
starting at level 13 and three times before a Long\
Rest starting at level 17.\
