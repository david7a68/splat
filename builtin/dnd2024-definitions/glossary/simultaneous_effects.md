---
splat_schema: 1
splat_id: 2c8a934c-b8f9-5a47-b274-d843b1687006
splat_kind: rule
splat_key: dnd2024:glossary/simultaneous_effects
splat_name: Simultaneous Effects
splat_origin:
  key: dnd2024:reference/glossary/simultaneous_effects
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/simultaneous_effects
  parameters:
    when:
      same_time: true
      during_turn: true
      minimum_effects: 2
    order_choice:
      chooser: current_turn_creature_controller
      options: all_permutations_of_simultaneous_effects
  implementation: dnd2024:glossary_simultaneous_effects
splat_contracts:
- id: dnd2024:glossary_simultaneous_effects
  inputs:
    turn_owner:
      type: participant
      required: true
    effects:
      type: definition_list
      required: true
    ordering:
      type: choice
      required: true
  outputs:
  - ordered_effects
  events:
  - effects.simultaneous
  capabilities:
  - events
splat_conversion:
  status: specified
  category: glossary
  source_sha256: ed90405ba923ca4c49358d370bf1c6db7fda8bdf13929aef527606ac2e152128
  source_path: builtin/srd-5.2.1/glossary/simultaneous_effects.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_simultaneous_effects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 73389fc272e21e12c0842794d824b908b54b39c8586d3ea465454c6646c28852
splat_clauses:
- key: source_rules
  text: '# Simultaneous Effects If two or more things happen at the same time on a turn, the person at
    the game table—player or GM— whose turn it is decides the order in which those things happen. For
    example, if two effects occur at the start of a player character’s turn, the player decides which
    of the effects happens first.'
  sha256: 62d04b76e24eeff5bfe2bed237da9c8740d74048edb16100325eb3d680ba598b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Simultaneous Effects

<!-- SRD page 187 -->

If two or more things happen at the same time on a\
turn, the person at the game table—player or GM—\
whose turn it is decides the order in which those\
things happen. For example, if two effects occur at\
the start of a player character’s turn, the player de-\
cides which of the effects happens first.\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
