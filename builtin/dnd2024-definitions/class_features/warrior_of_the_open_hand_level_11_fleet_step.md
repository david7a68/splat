---
splat_schema: 1
splat_id: 9ccb7da8-f94b-586e-af14-bd64723d0330
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step
splat_name: 'Level 11: Fleet Step'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_11_fleet_step
  parameters:
    trigger: actor.bonus_action_completed
    exclude: step_of_the_wind
    grant:
      feature:
        key: dnd2024:class_features/monk_level_2_monk_s_focus
      option: step_of_the_wind
      timing: immediately_after_trigger
      additional_bonus_action_cost: false
      normal_optional_focus_costs_apply: true
    class_scope:
      class: monk
      minimum_level: 11
  implementation: dnd2024:class_features_warrior_of_the_open_hand_level_11_fleet_step
splat_contracts:
- id: dnd2024:class_features_warrior_of_the_open_hand_level_11_fleet_step
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
  - action.bonus_completed
  capabilities:
  - character_features
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_warrior_of_the_open_hand_level_11_fleet_step
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: d2bf6abe91edbc998ce653c72016c3b53425dc0f74970e899ea823d264d0e06d
  parent_name: Warrior of the Open Hand
  line_span:
    first: 4376
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 11: Fleet Step When you take a Bonus Action other than Step of the Wind, you can also
    use Step of the Wind immediately after that Bonus Action.'
  sha256: c805085eaf06f7ab792baa89a93c284ba8253bdaf25a884142f8ee2ffd246abd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 11: Fleet Step

<!-- SRD page 52 -->

When you take a Bonus Action other than Step of\
the Wind, you can also use Step of the Wind imme-\
diately after that Bonus Action.\
