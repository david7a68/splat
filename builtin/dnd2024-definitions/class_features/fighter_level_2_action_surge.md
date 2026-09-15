---
splat_schema: 1
splat_id: 2d5dc600-fa36-5b63-a785-da2dcb2409a0
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_2_action_surge
splat_name: 'Level 2: Action Surge'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_2_action_surge
  parameters:
    on_own_turn: true
    additional_actions: 1
    excluded_actions:
    - magic
    uses:
      maximum_by_fighter_level:
        '2': 1
        '17': 2
      recover:
        events:
        - rest.short_rest_completed
        - rest.long_rest_completed
        amount: all_expended
    limit:
      count: 1
      period: turn
  implementation: dnd2024:class_features_fighter_level_2_action_surge
splat_contracts:
- id: dnd2024:class_features_fighter_level_2_action_surge
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  events:
  - turn.active
  capabilities:
  - action_economy
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_2_action_surge
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 15ca995c4e172e637461467695ddb3098a900d5930be50e5bed153e822de3602
  parent_name: Fighter
  line_span:
    first: 3985
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 2: Action Surge You can push yourself beyond your normal limits for a moment. On your
    turn, you can take one additional action, except the Magic action. Once you use this feature, you
    can’t do so again until you finish a Short or Long Rest. Starting at level 17, you can use it twice
    before a rest but only once on a turn.'
  sha256: acae6b97a6e46ae9839ae4c984767f7735ed4cc7b1c6922498ad1cd87a64f4c5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Action Surge

<!-- SRD page 48 -->

You can push yourself beyond your normal limits\
for a moment. On your turn, you can take one addi-\
tional action, except the Magic action.\
Once you use this feature, you can’t do so again\
until you finish a Short or Long Rest. Starting at\
level 17, you can use it twice before a rest but only\
once on a turn.\
