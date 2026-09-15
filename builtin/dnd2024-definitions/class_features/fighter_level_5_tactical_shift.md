---
splat_schema: 1
splat_id: 47f944dd-d983-5f6d-962f-459cbedb682c
splat_kind: rule
splat_key: dnd2024:class_features/fighter_level_5_tactical_shift
splat_name: 'Level 5: Tactical Shift'
splat_origin:
  key: dnd2024:reference/pages/048
splat_sources:
- title: SRD 5.2.1
  locator: pp. 48
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/fighter_level_5_tactical_shift
  parameters:
    trigger: actor.second_wind_activated_with_bonus_action
    optional_movement:
      maximum_feet:
        op: multiply
        args:
        - var: actor.speed
        - 0.5
      opportunity_attacks: false
    class_scope:
      class: fighter
      minimum_level: 5
  implementation: dnd2024:class_features_fighter_level_5_tactical_shift
splat_contracts:
- id: dnd2024:class_features_fighter_level_5_tactical_shift
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - second_wind.activated
  capabilities:
  - character_features
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 48
  engine_gaps: &id001
  - dnd2024:class_features_fighter_level_5_tactical_shift
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 65ce6150f98034b53934b42447e9bff6cdce90d1cd72792a7773ce297f1fee90
  parent_name: Fighter
  line_span:
    first: 4016
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 5: Tactical Shift Whenever you activate your Second Wind with a Bonus Action, you can
    move up to half your Speed without provoking Opportunity Attacks.'
  sha256: e68ac6222223f07a9eecb9f4eb555201fe6ed81459713361f1650abf25c81115
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Tactical Shift

<!-- SRD page 48 -->

Whenever you activate your Second Wind with a\
Bonus Action, you can move up to half your Speed\
without provoking Opportunity Attacks.\
