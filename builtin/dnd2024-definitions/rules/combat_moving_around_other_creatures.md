---
splat_schema: 1
splat_id: 1391bf67-98a7-54ef-ae03-29cb242ada2c
splat_kind: rule
splat_key: dnd2024:rules/combat_moving_around_other_creatures
splat_name: Moving around Other Creatures
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_moving_around_other_creatures
  parameters:
    pass_through:
      any:
      - ally
      - occupant_incapacitated
      - occupant_tiny
      - absolute_size_category_difference_minimum: 2
    difficult_terrain:
      unless_any:
      - occupant_tiny
      - ally
    willing_end_move_in_occupied_space: false
    occupied_space_turn_end:
      condition: prone
      unless_any:
      - mover_tiny
      - mover_larger_than_occupant
  implementation: dnd2024:rules_combat_moving_around_other_creatures
splat_contracts:
- id: dnd2024:rules_combat_moving_around_other_creatures
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
  - dnd2024:rules_combat_moving_around_other_creatures
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: d7916bf30dd5c576098ace4831ea089de09913d288edf9a5fdbb4b650d193faa
  parent_name: Combat
  line_span:
    first: 938
    count: 13
splat_clauses:
- key: source_rules
  text: '# Moving around Other Creatures During your move, you can pass through the space of an ally,
    a creature that has the Incapacitated condition (see “Rules Glossary”), a Tiny creature, or a creature
    that is two sizes larger or smaller than you. Another creature’s space is Difficult Terrain for you
    unless that creature is Tiny or your ally. You can’t willingly end a move in a space occupied by another
    creature. If you somehow end a turn in a space with another creature, you have the Prone condition
    (see “Rules Glossary”) unless you are Tiny or are of a larger size than the other creature.'
  sha256: 579688c65f33ac253cf0ba6fcdb4cda152e5ef5df2f207e50c87b912f85ea424
  status: contract_specified
splat_requires_mechanics: *id001
---

# Moving around Other Creatures

<!-- SRD page 14 -->

During your move, you can pass through the space\
of an ally, a creature that has the Incapacitated\
condition (see “Rules Glossary”), a Tiny creature, or\
a creature that is two sizes larger or smaller than\
you.\
Another creature’s space is Difficult Terrain for\
you unless that creature is Tiny or your ally.\
You can’t willingly end a move in a space occupied\
by another creature. If you somehow end a turn in\
a space with another creature, you have the Prone\
condition (see “Rules Glossary”) unless you are Tiny\
or are of a larger size than the other creature.\
