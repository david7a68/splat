---
splat_schema: 1
splat_id: 79520559-71ad-5f31-ae29-eec98d09f594
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_ammunition_and_ranged_attacks
splat_name: Ammunition and Ranged Attacks
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_ammunition_and_ranged_attacks
  parameters:
    stat_block_ranged_weapon_ammunition: necessary_ammunition_carried
  implementation: dnd2024:rules_parts_of_a_stat_block_ammunition_and_ranged_attacks
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_ammunition_and_ranged_attacks
  inputs:
    monster:
      type: participant
      required: true
    weapon:
      type: item_instance
      required: true
  outputs:
  - ammunition
  events: []
  capabilities:
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_ammunition_and_ranged_attacks
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 34ce57f3753e9cc01d12e09d978bc797badc389d5ce4475032a8e776d0585b68
  parent_name: Parts of a Stat Block
  line_span:
    first: 11048
    count: 3
splat_clauses:
- key: source_rules
  text: '# Ammunition and Ranged Attacks A monster that requires ammunition to make ranged attacks carries
    the necessary ammunition.'
  sha256: aff892aba941353c9b81139b15aad226589626fdf670525ffb8a3cc6273e5887
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ammunition and Ranged Attacks

<!-- SRD page 255 -->

A monster that requires ammunition to make\
ranged attacks carries the necessary ammunition.\
