---
splat_schema: 1
splat_id: 82786c88-b002-5db2-9dfc-d88e8caab0b3
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_armor_class
splat_name: Armor Class
splat_origin:
  key: dnd2024:reference/pages/254
splat_sources:
- title: SRD 5.2.1
  locator: pp. 254
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_armor_class
  parameters:
    stat_block_ac_includes:
    - natural_armor
    - dexterity
    - gear
    - other_defenses
    do_not_readd_included_bonuses: true
  implementation: dnd2024:rules_parts_of_a_stat_block_armor_class
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_armor_class
  inputs:
    monster:
      type: participant
      required: true
  outputs:
  - armor_class
  events: []
  capabilities:
  - armor_class
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 254
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_armor_class
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 2ebe43c3dbaf6142f341eaa4302e46c616c5fe5bc40a3cf6d7d95a8b8dc96ca4
  parent_name: Parts of a Stat Block
  line_span:
    first: 10946
    count: 4
splat_clauses:
- key: source_rules
  text: '# Armor Class A monster’s Armor Class (AC) includes its natural armor, Dexterity, gear, and other
    defenses. See “Playing the Game” for information on Armor Class.'
  sha256: fd29c2e40da059b1f2d9ca08d86a02ffa56b661afcc1c09b1f0a222941588e37
  status: contract_specified
splat_requires_mechanics: *id001
---

# Armor Class

<!-- SRD page 254 -->

A monster’s Armor Class (AC) includes its natural\
armor, Dexterity, gear, and other defenses. See\
“Playing the Game” for information on Armor Class.\
