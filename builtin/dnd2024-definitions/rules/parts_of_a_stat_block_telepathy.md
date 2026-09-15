---
splat_schema: 1
splat_id: 4b3c1411-6929-5b24-bb41-4c6b21638266
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_telepathy
splat_name: Telepathy
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_telepathy
  parameters:
    rule:
      key: dnd2024:glossary/telepathy
    communication: mental
    magical: true
    range_source: stat_block
  implementation: dnd2024:rules_parts_of_a_stat_block_telepathy
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_telepathy
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events: []
  capabilities:
  - telepathy
  - languages
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_telepathy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 53ec535f4d45cc70bdcfa8ff8a06eb9433aa610770b122866795b7ac188ecd97
  parent_name: Parts of a Stat Block
  line_span:
    first: 11075
    count: 5
splat_clauses:
- key: source_rules
  text: '# Telepathy Telepathy is a magical ability that allows a creature to communicate mentally with
    another creature within a specified range. See “Rules Glossary” for more information.'
  sha256: 2ff6425a302355b2118cc4adb455d1c99e0911252767c326c149617e1adce55a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Telepathy

<!-- SRD page 256 -->

Telepathy is a magical ability that allows a creature\
to communicate mentally with another creature\
within a specified range. See “Rules Glossary” for\
more information.\
