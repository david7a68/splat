---
splat_schema: 1
splat_id: 14183f5d-ec29-567b-af1b-b8cf66161e9b
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_legendary_actions
splat_name: Legendary Actions
splat_origin:
  key: dnd2024:reference/pages/257
splat_sources:
- title: SRD 5.2.1
  locator: pp. 257
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_legendary_actions
  parameters:
    timing: immediately_after_another_creature_turn_ends
    maximum_actions_per_trigger: 1
    prohibited_if_any:
    - incapacitated
    - unable_to_take_actions
    options_source: stat_block_legendary_actions
    uses_maximum_source: stat_block
    uses_spent_per_action: 1
    recharge:
      timing: start_of_monster_turn
      amount: all_expended_uses
  implementation: dnd2024:rule_parts_of_a_stat_block_legendary_actions
splat_contracts:
- id: dnd2024:rule_parts_of_a_stat_block_legendary_actions
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - monster_parameters
  events:
  - turn.ended
  - turn.started
  capabilities:
  - monsters
  - legendary_actions
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 257
  engine_gaps: &id001
  - dnd2024:rule_parts_of_a_stat_block_legendary_actions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 594d74a127ffb2238d2ffbf32f6368b15c67fa20037698f85393484b2024a218
  parent_name: Parts of a Stat Block
  line_span:
    first: 11205
    count: 14
splat_clauses:
- key: source_rules
  text: '# Legendary Actions If the monster has Legendary Action options, those are listed in this section.
    A Legendary Action is an action that a monster can take immediately after another creature’s turn.
    Only one of these actions can be taken at a time and only after another creature’s turn ends. The
    monster can’t take a Legendary Action if it has the Incapacitated condition or is otherwise unable
    to take actions. The monster has a limited number of Legendary Action uses, and that number is specified
    in the stat block. The monster expends one use whenever it takes a Legendary Action, and it regains
    all expended uses at the start of each of its turns.'
  sha256: 2b686e9b4ad871bb4b0023ee81f72880d5779fa3f3bb01556a92f0199fce4cd1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Legendary Actions

<!-- SRD page 257 -->

If the monster has Legendary Action options, those\
are listed in this section. A Legendary Action is an\
action that a monster can take immediately after\
another creature’s turn. Only one of these actions\
can be taken at a time and only after another crea-\
ture’s turn ends. The monster can’t take a Legend-\
ary Action if it has the Incapacitated condition or is\
otherwise unable to take actions.\
The monster has a limited number of Legendary\
Action uses, and that number is specified in the\
stat block. The monster expends one use whenever\
it takes a Legendary Action, and it regains all ex-\
pended uses at the start of each of its turns.\
