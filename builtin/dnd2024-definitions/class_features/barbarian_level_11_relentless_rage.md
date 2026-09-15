---
splat_schema: 1
splat_id: e4eff469-829a-502d-b0ef-343801956453
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_11_relentless_rage
splat_name: 'Level 11: Relentless Rage'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_11_relentless_rage
  parameters:
    trigger:
      hit_points_would_be: 0
      rage_active: true
      instant_death: false
    optional: true
    save:
      ability: constitution
      dc:
        op: add
        args:
        - 10
        - op: multiply
          args:
          - 5
          - stat: relentless_rage.uses_since_rest
      success:
        replace_hit_points:
          op: multiply
          args:
          - 2
          - stat: classes.barbarian.level
    on_use:
      increment: relentless_rage.uses_since_rest
    reset:
      events:
      - rest.short_rest_completed
      - rest.long_rest_completed
      uses_since_rest: 0
  implementation: dnd2024:class_features_barbarian_level_11_relentless_rage
splat_contracts:
- id: dnd2024:class_features_barbarian_level_11_relentless_rage
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
  - hit_points.zero_pending
  - rest.short_rest_completed
  - rest.long_rest_completed
  capabilities:
  - saving_throws
  - hit_points
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_11_relentless_rage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 396b3c1c84e20d22a4cf969801ac1c2493221e0c23a75c1fc43cabd228ed9c60
  parent_name: Barbarian
  line_span:
    first: 2403
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 11: Relentless Rage Your Rage can keep you fighting despite grievous wounds. If you drop
    to 0 Hit Points while your Rage is active and don’t die outright, you can make a DC 10 Constitution
    saving throw. If you succeed, your Hit Points instead change to a number equal to twice your Barbarian
    level. Each time you use this feature after the first, the DC increases by 5. When you finish a Short
    or Long Rest, the DC resets to 10.'
  sha256: a7898acac6c3247cd2241bcaa1170b621c2c18b1a279f2ebd615e1c373f12307
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 11: Relentless Rage

<!-- SRD page 30 -->

Your Rage can keep you fighting despite grievous\
wounds. If you drop to 0 Hit Points while your Rage\
is active and don’t die outright, you can make a DC\
10 Constitution saving throw. If you succeed, your\
Hit Points instead change to a number equal to\
twice your Barbarian level.\
Each time you use this feature after the first, the\
DC increases by 5. When you finish a Short or Long\
Rest, the DC resets to 10.\
