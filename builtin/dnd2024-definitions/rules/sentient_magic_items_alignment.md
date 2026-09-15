---
splat_schema: 1
splat_id: 24376721-4c66-52f8-9073-03ca25709563
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_alignment
splat_name: Alignment
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sentient_magic_items_alignment
  parameters:
    alignment_required: true
    methods:
    - creator_or_nature
    - choose
    - roll
    roll:
      count: 1
      sides: 100
      add: 0
    table:
    - roll_min: 1
      roll_max: 15
      alignment: lawful_good
    - roll_min: 16
      roll_max: 35
      alignment: neutral_good
    - roll_min: 36
      roll_max: 50
      alignment: chaotic_good
    - roll_min: 51
      roll_max: 63
      alignment: lawful_neutral
    - roll_min: 64
      roll_max: 73
      alignment: neutral
    - roll_min: 74
      roll_max: 85
      alignment: chaotic_neutral
    - roll_min: 86
      roll_max: 89
      alignment: lawful_evil
    - roll_min: 90
      roll_max: 96
      alignment: neutral_evil
    - roll_min: 97
      roll_max: 100
      alignment: chaotic_evil
  implementation: dnd2024:rules_sentient_magic_items_alignment
splat_contracts:
- id: dnd2024:rules_sentient_magic_items_alignment
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - item_updates
  - cost
  - effects
  events: []
  capabilities:
  - items
  - attunement
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 207
  engine_gaps: &id001
  - dnd2024:rules_sentient_magic_items_alignment
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 0559f0f214ec87a55d2af51b737d37631b965a651423aa09f29cf94d1c673f5b
  parent_name: Sentient Magic Items
  line_span:
    first: 10757
    count: 5
splat_clauses:
- key: source_rules
  text: '# Alignment A sentient magic item has an alignment. Its creator or nature might suggest an alignment.
    Otherwise, pick an alignment or roll on the Sentient Item’s Alignment table.'
  sha256: cbf5d0ad9ecdc1a277cd042d1b3acac1e24f98edcb5748e66a69afe28dbe62f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Alignment

<!-- SRD page 207 -->

A sentient magic item has an alignment. Its creator\
or nature might suggest an alignment. Otherwise,\
pick an alignment or roll on the Sentient Item’s\
Alignment table.\
