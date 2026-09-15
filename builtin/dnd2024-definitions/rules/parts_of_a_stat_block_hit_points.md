---
splat_schema: 1
splat_id: e9611215-c32f-5b24-b81a-e17349713e6f
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_hit_points
splat_name: Hit Points
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_hit_points
  parameters:
    hit_point_generation_choice:
    - printed_number
    - roll_printed_expression
    combine_methods: false
    constitution_contribution:
      op: multiply
      args:
      - variable: constitution_modifier
      - variable: hit_dice_count
    typical_hit_die_by_size:
    - size: tiny
      die_sides: 4
      average_per_die: 2.5
    - size: small
      die_sides: 6
      average_per_die: 3.5
    - size: medium
      die_sides: 8
      average_per_die: 4.5
    - size: large
      die_sides: 10
      average_per_die: 5.5
    - size: huge
      die_sides: 12
      average_per_die: 6.5
    - size: gargantuan
      die_sides: 20
      average_per_die: 10.5
  implementation: dnd2024:rule_parts_of_a_stat_block_hit_points
splat_contracts:
- id: dnd2024:rule_parts_of_a_stat_block_hit_points
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    constitution_modifier:
      type: number
      required: true
    hit_dice_count:
      type: number
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - monster_parameters
  events: []
  capabilities:
  - monsters
  - hit_points
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rule_parts_of_a_stat_block_hit_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 695609e3c6b924af77d9b36f7626358a99b6286fa8793dbe22f92735be0ef9d2
  parent_name: Parts of a Stat Block
  line_span:
    first: 10963
    count: 25
splat_clauses:
- key: source_rules
  text: '# Hit Points A monster’s Hit Points are presented as a number followed by parentheses, where
    the monster’s Hit Point Dice are provided, along with any contribution from its Constitution. Either
    use the number for the monster’s Hit Points or roll the die expression in parentheses to determine
    the monster’s Hit Points randomly; don’t use both. A monster’s size typically determines the die used
    to calculate its Hit Points, as shown in the Hit Dice by Size table. Hit Dice by Size Monster Size
    Hit Die Average HP per Die Tiny d4 2½ Small d6 3½ Medium d8 4½ Large d10 5½ Huge d12 6½ Gargantuan
    d20 10½ A monster’s Constitution modifier is multiplied by the number of Hit Dice it possesses, and
    the result is added to its Hit Points. For example, if a monster has a Constitution of 12 (+1 modifier)
    and 2d8 Hit Dice, it has 2d8 + 2 Hit Points (average 11). For more on Hit Points, see “Playing the
    Game.”'
  sha256: 5940476e9b540ef67d2cfbcb8e642e2bdb1f0f267bfe1ab984647022d8c5adfb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hit Points

<!-- SRD page 255 -->

A monster’s Hit Points are presented as a number\
followed by parentheses, where the monster’s Hit\
Point Dice are provided, along with any contribu-\
tion from its Constitution. Either use the number for\
the monster’s Hit Points or roll the die expression in\
parentheses to determine the monster’s Hit Points\
randomly; don’t use both.\
A monster’s size typically determines the die used\
to calculate its Hit Points, as shown in the Hit Dice\
by Size table.\
Hit Dice by Size\
Monster Size Hit Die Average HP per Die\
Tiny d4 2½\
Small d6 3½\
Medium d8 4½\
Large d10 5½\
Huge d12 6½\
Gargantuan d20 10½\
A monster’s Constitution modifier is multiplied by\
the number of Hit Dice it possesses, and the result\
is added to its Hit Points. For example, if a monster\
has a Constitution of 12 (+1 modifier) and 2d8 Hit\
Dice, it has 2d8 + 2 Hit Points (average 11).\
For more on Hit Points, see “Playing the Game.”\
