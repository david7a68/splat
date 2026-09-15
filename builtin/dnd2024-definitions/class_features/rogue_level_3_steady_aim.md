---
splat_schema: 1
splat_id: bc135be7-382d-5214-94a8-11e493bc924d
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_3_steady_aim
splat_name: 'Level 3: Steady Aim'
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_3_steady_aim
  parameters:
    action: bonus_action
    requires_movement_so_far_this_turn: 0
    next_attack_roll:
      advantage: true
      expires: current_turn_end
    speed_override:
      feet: 0
      until: current_turn_end
  implementation: dnd2024:class_features_rogue_level_3_steady_aim
splat_contracts:
- id: dnd2024:class_features_rogue_level_3_steady_aim
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
    destination:
      required: false
      type: location
  outputs:
  - feature_state
  - movement
  events:
  - action.bonus
  capabilities:
  - attacks
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 62
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_3_steady_aim
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 19b03b15e58b48b93864d36a6ee2a4c55b5a3af350ef5628e9c5a613bd7188d0
  parent_name: Rogue
  line_span:
    first: 5256
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 3: Steady Aim As a Bonus Action, you give yourself Advantage on your next attack roll
    on the current turn. You can use this feature only if you haven’t moved during this turn, and after
    you use it, your Speed is 0 until the end of the current turn.'
  sha256: c1b50a6ebe9ffcdb65fefd2572f33a6ea82d733ab706951bf2e09ec2b1cbe3af
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Steady Aim

<!-- SRD page 62 -->

As a Bonus Action, you give yourself Advantage on\
your next attack roll on the current turn. You can\
use this feature only if you haven’t moved during\
this turn, and after you use it, your Speed is 0 until\
the end of the current turn.\
