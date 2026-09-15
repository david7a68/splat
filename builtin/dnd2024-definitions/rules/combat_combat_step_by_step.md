---
splat_schema: 1
splat_id: cca19111-402b-530c-8d0a-fcbe7562f7c7
splat_kind: rule
splat_key: dnd2024:rules/combat_combat_step_by_step
splat_name: Combat Step by Step
splat_origin:
  key: dnd2024:reference/pages/013
splat_sources:
- title: SRD 5.2.1
  locator: pp. 13
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_combat_step_by_step
  parameters:
    sequence:
    - operation: assign_positions
      authority: gm
      inputs:
      - marching_order
      - stated_positions
    - operation: roll_initiative
      participants: all
    - operation: take_turns
      order: initiative
      round_end: all_participants_acted
      repeat_until: combat_ended
  implementation: dnd2024:rules_combat_combat_step_by_step
splat_contracts:
- id: dnd2024:rules_combat_combat_step_by_step
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
  - 13
  engine_gaps: &id001
  - dnd2024:rules_combat_combat_step_by_step
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: e71b88b9947c4a9bdf5869c68073bc8856bc7c1aa927685ff0e238d2fbdbb90f
  parent_name: Combat
  line_span:
    first: 776
    count: 15
splat_clauses:
- key: source_rules
  text: '# Combat Step by Step Combat unfolds in these steps: 1: Establish Positions. The Game Master
    determines where all the characters and monsters are located. Given the adventurers’ marching order
    or their stated positions in the room or other location, the GM figures out where the adversaries
    are—how far away and in what direction. 2: Roll Initiative. Everyone involved in the combat encounter
    rolls Initiative, determining the order of combatants’ turns. 3: Take Turns. Each participant in the
    battle takes a turn in Initiative order. When everyone involved in the combat has had a turn, the
    round ends. Repeat this step until the fighting stops.'
  sha256: 9974adfdb2dddcefe981c8b82bb67fb51307f9d4669113f2834b9f276adc4d98
  status: contract_specified
splat_requires_mechanics: *id001
---

# Combat Step by Step

<!-- SRD page 13 -->

Combat unfolds in these steps:\
1: Establish Positions. The Game Master deter-\
mines where all the characters and monsters are\
located. Given the adventurers’ marching order\
or their stated positions in the room or other lo-\
cation, the GM figures out where the adversaries\
are—how far away and in what direction.\
2: Roll Initiative. Everyone involved in the combat\
encounter rolls Initiative, determining the order\
of combatants’ turns.\
3: Take Turns. Each participant in the battle takes\
a turn in Initiative order. When everyone in-\
volved in the combat has had a turn, the round\
ends. Repeat this step until the fighting stops.\
