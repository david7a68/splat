---
splat_schema: 1
splat_id: 5f8c100e-d4a7-5610-84ad-e82cee6acd16
splat_kind: rule
splat_key: dnd2024:rules/combat_breaking_up_your_move
splat_name: Breaking Up Your Move
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_breaking_up_your_move
  parameters:
    movement_segments_allowed: true
    interleave_with:
    - action
    - bonus_action
    - reaction
    turn_scope: same_turn
    movement_budget: remaining_movement
  implementation: dnd2024:rules_combat_breaking_up_your_move
splat_contracts:
- id: dnd2024:rules_combat_breaking_up_your_move
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
  - 14
  engine_gaps: &id001
  - dnd2024:rules_combat_breaking_up_your_move
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 6d431719197c26c18e32d410f03c9b9148fa860bd0a9fcd889580867424fe053
  parent_name: Combat
  line_span:
    first: 908
    count: 6
splat_clauses:
- key: source_rules
  text: '# Breaking Up Your Move You can break up your move, using some of its movement before and after
    any action, Bonus Action, or Reaction you take on the same turn. For example, if you have a Speed
    of 30 feet, you could go 10 feet, take an action, and then go 20 feet.'
  sha256: 573afd3186c79bbb0674776545de5741e1b152b34828b15552b44a26c0330e30
  status: contract_specified
splat_requires_mechanics: *id001
---

# Breaking Up Your Move

<!-- SRD page 14 -->

You can break up your move, using some of its\
movement before and after any action, Bonus Ac-\
tion, or Reaction you take on the same turn. For\
example, if you have a Speed of 30 feet, you could go\
10 feet, take an action, and then go 20 feet.\
