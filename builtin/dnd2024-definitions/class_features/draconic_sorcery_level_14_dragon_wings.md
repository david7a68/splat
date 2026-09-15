---
splat_schema: 1
splat_id: 8def3014-60c5-55ce-8487-eadf262a4a14
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_14_dragon_wings
splat_name: 'Level 14: Dragon Wings'
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/draconic_sorcery_level_14_dragon_wings
  parameters:
    action: bonus_action
    resource:
      id: sorcerer.dragon_wings
      maximum: 1
      recovery:
      - event: rest.long_completed
        amount: all
    cost: 1
    duration:
      hours: 1
    fly_speed_feet: 60
    dismiss:
      action: none
    recharge_alternative:
      action: none
      resource_cost:
        id: sorcerer.sorcery_points
        amount: 3
      restore_uses: 1
    class_scope:
      class: sorcerer
      minimum_level: 14
  implementation: dnd2024:class_features_draconic_sorcery_level_14_dragon_wings
splat_contracts:
- id: dnd2024:class_features_draconic_sorcery_level_14_dragon_wings
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
  - action.bonus
  - resource.convert
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - movement
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:class_features_draconic_sorcery_level_14_dragon_wings
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 06730c53ad72f0e710b08a8a41438ca872c8c12b240aa551d0ec9431176d302a
  parent_name: Draconic Sorcery
  line_span:
    first: 5915
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 14: Dragon Wings As a Bonus Action, you can cause draconic wings to appear on your back.
    The wings last for 1 hour or until you dismiss them (no action required). For the duration, you have
    a Fly Speed of 60 feet. Once you use this feature, you can’t use it again until you finish a Long
    Rest unless you spend 3 Sorcery Points (no action required) to restore your use of it.'
  sha256: 26aff1e14435fd3a1bfc741e9495788d71a59b2efff6e01db61925b23f2823cc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Dragon Wings

<!-- SRD page 70 -->

As a Bonus Action, you can cause draconic wings to\
appear on your back. The wings last for 1 hour or\
until you dismiss them (no action required). For the\
duration, you have a Fly Speed of 60 feet.\
Once you use this feature, you can’t use it again\
until you finish a Long Rest unless you spend 3 Sor-\
cery Points (no action required) to restore your use\
of it.\
