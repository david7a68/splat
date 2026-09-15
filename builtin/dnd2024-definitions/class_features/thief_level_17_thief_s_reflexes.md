---
splat_schema: 1
splat_id: 28692370-9533-5df1-a69a-db6fa755fb0b
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_17_thief_s_reflexes
splat_name: 'Level 17: Thief’s Reflexes'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/thief_level_17_thief_s_reflexes
  parameters:
    combat_round: 1
    turns:
    - initiative: normal
    - initiative:
        op: subtract
        args:
        - var: actor.initiative
        - 10
    class_scope:
      class: rogue
      minimum_level: 17
  implementation: dnd2024:class_features_thief_level_17_thief_s_reflexes
splat_contracts:
- id: dnd2024:class_features_thief_level_17_thief_s_reflexes
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
  - turn_schedule
  events:
  - combat.started
  - initiative.ordered
  capabilities:
  - character_features
  - initiative
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 64
  engine_gaps: &id001
  - dnd2024:class_features_thief_level_17_thief_s_reflexes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 4ebbdcdd4a826def212fb79871d98366a402d413630df8a565dce4e34e1123e2
  parent_name: Thief
  line_span:
    first: 5396
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 17: Thief’s Reflexes You are adept at laying ambushes and quickly escaping danger. You
    can take two turns during the first round of any combat. You take your first turn at your normal Initiative
    and your second turn at your Initiative minus 10.'
  sha256: 5ce2e4d79074e6a6569d8ebd842c89de9926d74b294bf9c0bd8054f544d575ab
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 17: Thief’s Reflexes

<!-- SRD page 64 -->

You are adept at laying ambushes and quickly es-\
caping danger. You can take two turns during the\
first round of any combat. You take your first turn\
at your normal Initiative and your second turn at\
your Initiative minus 10.\
