---
splat_schema: 1
splat_id: 0e1a1e99-fe60-5207-b8a1-cbbe840ac429
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_multiattack
splat_name: Multiattack
splat_origin:
  key: dnd2024:reference/pages/257
splat_sources:
- title: SRD 5.2.1
  locator: pp. 257
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_multiattack
  parameters:
    action: attack
    attack_count_and_types: multiattack_entry
    additional_abilities_allowed: multiattack_entry
  implementation: dnd2024:rules_parts_of_a_stat_block_multiattack
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_multiattack
  inputs:
    monster:
      type: participant
      required: true
    multiattack:
      type: definition
      required: true
  outputs:
  - attack_sequence
  - additional_abilities
  events: []
  capabilities:
  - monster_attacks
  - action_economy
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 257
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_multiattack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: a143a376a22ac4356ce9d69ed157079c65e0dcf5da335509248e00ece7933a7d
  parent_name: Parts of a Stat Block
  line_span:
    first: 11164
    count: 7
splat_clauses:
- key: source_rules
  text: '# Multiattack Some creatures can make more than one attack when they take the Attack action.
    Such creatures have the Multiattack entry in the “Actions” section of their stat block. This entry
    details the attacks a creature can make, as well as any additional abilities it can use, as part of
    the Attack action.'
  sha256: 8c477f9d4a6965394f4f64af8d5eefe33f90758d88a6c963f79a0a42635fd568
  status: contract_specified
splat_requires_mechanics: *id001
---

# Multiattack

<!-- SRD page 257 -->

Some creatures can make more than one attack\
when they take the Attack action. Such creatures\
have the Multiattack entry in the “Actions” section\
of their stat block. This entry details the attacks a\
creature can make, as well as any additional abili-\
ties it can use, as part of the Attack action.\
