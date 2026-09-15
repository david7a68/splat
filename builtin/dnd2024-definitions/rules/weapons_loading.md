---
splat_schema: 1
splat_id: 840c7c4e-fc58-5081-8e5b-851090e4a0a4
splat_kind: rule
splat_key: dnd2024:rules/weapons_loading
splat_name: Loading
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_loading
  parameters:
    ammunition_fired_limit: 1
    limit_scope: each_action_bonus_action_or_reaction
    extra_attacks_override_limit: false
  implementation: dnd2024:rules_weapons_loading
splat_contracts:
- id: dnd2024:rules_weapons_loading
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events: []
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_loading
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: c06875e69761bff2c84e72849c08799705480d4e0d3f1b503f5482300b4cbff0
  parent_name: Weapons
  line_span:
    first: 7729
    count: 5
splat_clauses:
- key: source_rules
  text: '# Loading You can fire only one piece of ammunition from a Loading weapon when you use an action,
    a Bonus Action, or a Reaction to fire it, regardless of the number of attacks you can normally make.'
  sha256: b9319a6a4952aaa7372f3d8ba8288e88491a808ee2085f9b779dfbf207c68899
  status: contract_specified
splat_requires_mechanics: *id001
---

# Loading

<!-- SRD page 90 -->

You can fire only one piece of ammunition from a\
Loading weapon when you use an action, a Bonus\
Action, or a Reaction to fire it, regardless of the\
number of attacks you can normally make.\
