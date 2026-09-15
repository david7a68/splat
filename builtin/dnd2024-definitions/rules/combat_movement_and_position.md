---
splat_schema: 1
splat_id: 9c89020a-0301-503f-a7b5-d4783212276d
splat_kind: rule
splat_key: dnd2024:rules/combat_movement_and_position
splat_name: Movement and Position
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_movement_and_position
  parameters:
    on_own_turn: true
    movement_budget:
      var: subject.speed
    minimum_distance: 0
    mixed_modes_allowed:
    - walk
    - climb
    - crawl
    - jump
    - swim
    spend_budget_per_segment: true
    speed_source:
      character: character_creation
      monster: stat_block
    special_speed_modes:
    - climb
    - fly
    - swim
    movement_mode_rules:
    - key: dnd2024:glossary/climbing
    - key: dnd2024:glossary/crawling
    - key: dnd2024:glossary/jumping
    - key: dnd2024:glossary/swimming
    - key: dnd2024:glossary/speed
  implementation: dnd2024:rules_combat_movement_and_position
splat_contracts:
- id: dnd2024:rules_combat_movement_and_position
  inputs:
    subject:
      type: participant
      required: true
    segments:
      type: resolution
      required: true
  outputs:
  - movement_remaining
  - position
  events:
  - movement.requested
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 14
  engine_gaps: &id001
  - dnd2024:rules_combat_movement_and_position
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 939000a53ebee8948e277fea857f4fa9e2311c4bcbb9f07714ef56a4643de398
  parent_name: Combat
  line_span:
    first: 883
    count: 17
splat_clauses:
- key: source_rules
  text: '# Movement and Position On your turn, you can move a distance equal to your Speed or less. Or
    you can decide not to move. Your movement can include climbing, crawling, jumping, and swimming (each
    explained in “Rules Glossary”). These different modes of movement can be combined with your regular
    movement, or they can constitute your entire move. However you’re moving with your Speed, you deduct
    the distance of each part of your move from it until it is used up or until you are done moving, whichever
    comes first. A character’s Speed is determined during character creation. A monster’s Speed is noted
    in the monster’s stat block. See “Rules Glossary” for more about Speed as well as about special speeds,
    such as a Climb Speed, Fly Speed, or Swim Speed.'
  sha256: c6f03fdda922e838578f2ebb123098f76ad67933c9ab4bb51a68d5d44044bca0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Movement and Position

<!-- SRD page 14 -->

On your turn, you can move a distance equal to your\
Speed or less. Or you can decide not to move.\
Your movement can include climbing, crawling,\
jumping, and swimming (each explained in “Rules\
Glossary”). These different modes of movement can\
be combined with your regular movement, or they\
can constitute your entire move.\
However you’re moving with your Speed, you\
deduct the distance of each part of your move from\
it until it is used up or until you are done moving,\
whichever comes first.\
A character’s Speed is determined during char-\
acter creation. A monster’s Speed is noted in the\
monster’s stat block. See “Rules Glossary” for more\
about Speed as well as about special speeds, such as\
a Climb Speed, Fly Speed, or Swim Speed.\
