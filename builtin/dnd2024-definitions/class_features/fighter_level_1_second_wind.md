---
splat_schema: 1
splat_id: fafc4274-e260-5db6-8ba8-9f6932139a27
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_1_second_wind
splat_name: 'Level 1: Second Wind'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_1_second_wind
  parameters:
    action: bonus_action
    heal_self:
      op: add
      args:
      - dice:
          count: 1
          sides: 10
      - stat: classes.fighter.level
    resource:
      maximum_by_fighter_level:
        '1': 2
        '4': 3
        '10': 4
      cost: 1
      recover:
      - event: rest.short_rest_completed
        amount: 1
      - event: rest.long_rest_completed
        amount: all_expended
  implementation: dnd2024:class_features_fighter_level_1_second_wind
splat_contracts:
- id: dnd2024:class_features_fighter_level_1_second_wind
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
  - healing
  events:
  - action.bonus
  - rest.short_rest_completed
  - rest.long_rest_completed
  capabilities:
  - healing
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_1_second_wind
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f1c4cdbe6955977690fd542849e8ebe95cd1f21ebe7bd4bdd37d812028bd00ab
  parent_name: Fighter
  line_span:
    first: 3963
    count: 12
splat_clauses:
- key: source_rules
  text: '# Level 1: Second Wind You have a limited well of physical and mental stamina that you can draw
    on. As a Bonus Action, you can use it to regain Hit Points equal to 1d10 plus your Fighter level.
    You can use this feature twice. You regain one expended use when you finish a Short Rest, and you
    regain all expended uses when you finish a Long Rest. When you reach certain Fighter levels, you gain
    more uses of this feature, as shown in the Second Wind column of the Fighter Features table.'
  sha256: 5f008fabfd6a3e43fb88bbd37d0aa93dba54d7f537baaa19014b9b402ccaf1c1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Second Wind

<!-- SRD page 48 -->

You have a limited well of physical and mental\
stamina that you can draw on. As a Bonus Action,\
you can use it to regain Hit Points equal to 1d10\
plus your Fighter level.\
You can use this feature twice. You regain one\
expended use when you finish a Short Rest, and you\
regain all expended uses when you finish a Long\
Rest.\
When you reach certain Fighter levels, you gain\
more uses of this feature, as shown in the Second\
Wind column of the Fighter Features table.\
