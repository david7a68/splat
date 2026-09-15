---
splat_schema: 1
splat_id: ba64b943-b28b-566d-b991-fc8bb7bb2c61
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_abilities
splat_name: Abilities
splat_origin:
  key: dnd2024:reference/pages/207
splat_sources:
- title: SRD 5.2.1
  locator: pp. 207
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sentient_magic_items_abilities
  parameters:
    abilities:
    - intelligence
    - wisdom
    - charisma
    generation_choice:
    - gm_select
    - random
    random_each:
      dice_count: 4
      die_sides: 6
      drop_lowest: 1
      sum_remaining: true
  implementation: dnd2024:rule_sentient_magic_items_abilities
splat_contracts:
- id: dnd2024:rule_sentient_magic_items_abilities
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - item_changes
  events: []
  capabilities:
  - magic_items
  - random_tables
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 207
  engine_gaps: &id001
  - dnd2024:rule_sentient_magic_items_abilities
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 7fb5825d2b2cb3aabdf0518a59af0ffe2a91657818eb72ddc39ea52735c4db76
  parent_name: Sentient Magic Items
  line_span:
    first: 10752
    count: 5
splat_clauses:
- key: source_rules
  text: '# Abilities A sentient magic item has Intelligence, Wisdom, and Charisma scores. Choose the item’s
    abilities, or determine them randomly as follows: roll 4d6 for each one, dropping the lowest roll
    and totaling the rest.'
  sha256: ddb627f940709db2856fbb5da0eddb9a7cb140ee66e6278e221f4201ec338045
  status: contract_specified
splat_requires_mechanics: *id001
---

# Abilities

<!-- SRD page 207 -->

A sentient magic item has Intelligence, Wisdom, and\
Charisma scores. Choose the item’s abilities, or de-\
termine them randomly as follows: roll 4d6 for each\
one, dropping the lowest roll and totaling the rest.\
