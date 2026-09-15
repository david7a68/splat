---
splat_schema: 1
splat_id: 7ad9f262-223a-55e6-a517-c4b8f0277e37
splat_kind: rule
splat_key: dnd2024:rules/exploration_time_limited_object_interactions
splat_name: Time-Limited Object Interactions
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_time_limited_object_interactions
  parameters:
    free_object_interactions_per_turn: 1
    during:
    - movement
    - action
    additional_interaction_action: utilize
    when: time_limited
  implementation: dnd2024:rule_exploration_time_limited_object_interactions
splat_contracts:
- id: dnd2024:rule_exploration_time_limited_object_interactions
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - action_budget
  events: []
  capabilities:
  - actions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 12
  engine_gaps: &id001
  - dnd2024:rule_exploration_time_limited_object_interactions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: b9c0a3f0c974197cd9bd4c16b50c7c544572ac5151d619b0941814bb233b1dc6
  parent_name: Exploration
  line_span:
    first: 672
    count: 7
splat_clauses:
- key: source_rules
  text: '# Time-Limited Object Interactions When time is short, such as in combat, interactions with objects
    are limited: one free interaction per turn. That interaction must occur during a creature’s movement
    or action. Any additional interactions require the Utilize action, as explained in “Combat” later
    in “Playing the Game.”'
  sha256: 86146b68c08380608d77c0ed25b07d1d986b021b2b425a8fc6bc147b828551b0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Time-Limited Object Interactions

<!-- SRD page 12 -->

When time is short, such as in combat, interactions\
with objects are limited: one free interaction per\
turn. That interaction must occur during a crea-\
ture’s movement or action. Any additional inter-\
actions require the Utilize action, as explained in\
“Combat” later in “Playing the Game.”\
