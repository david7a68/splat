---
splat_schema: 1
splat_id: 2accb2d8-94db-5675-b3b7-8834b493fbe2
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_damage_notation
splat_name: Damage Notation
splat_origin:
  key: dnd2024:reference/pages/257
splat_sources:
- title: SRD 5.2.1
  locator: pp. 257
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_damage_notation
  parameters:
    damage_mode_choice:
      options:
      - fixed_average
      - dice_expression
      count: 1
      authority: gm
    combine_modes: false
  implementation: dnd2024:rules_parts_of_a_stat_block_damage_notation
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_damage_notation
  inputs:
    damage:
      type: resolution
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - damage_amount
  events: []
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 257
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_damage_notation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 7ff8459a4cf1226d07645043d01005e74482370626ef433f93349a7062ef629c
  parent_name: Parts of a Stat Block
  line_span:
    first: 11158
    count: 6
splat_clauses:
- key: source_rules
  text: '# Damage Notation A stat block usually provides both a number and a die expression for each instance
    of damage. For example, an attack might deal 4 (1d4 + 2) damage on a hit. You decide whether to use
    the number or the die expression in parentheses; don’t use both.'
  sha256: 9b9482abb7788d95cf33ed5e95e5ad9300d049b67efc7837ab4f05b58a7dc8f1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage Notation

<!-- SRD page 257 -->

A stat block usually provides both a number and a\
die expression for each instance of damage. For ex-\
ample, an attack might deal 4 (1d4 + 2) damage on a\
hit. You decide whether to use the number or the die\
expression in parentheses; don’t use both.\
