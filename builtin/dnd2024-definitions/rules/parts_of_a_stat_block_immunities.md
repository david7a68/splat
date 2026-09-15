---
splat_schema: 1
splat_id: 1247b5c7-49a9-5a6a-ae89-f338091b04ea
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_immunities
splat_name: Immunities
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_immunities
  parameters:
    immunity_kinds:
    - damage_type
    - condition
    entry_order:
    - damage_types
    - conditions
    damage_immunity: prevent_matching_damage
    condition_immunity: prevent_matching_condition
  implementation: dnd2024:rules_parts_of_a_stat_block_immunities
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_immunities
  inputs:
    monster:
      type: participant
      required: true
    effect:
      type: resolution
      required: true
  outputs:
  - immunity_result
  events: []
  capabilities:
  - immunities
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_immunities
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 7c29d3b1f53bbabb8623c439f6e88a63ad7232e5629108631e6d870e0ea09718
  parent_name: Parts of a Stat Block
  line_span:
    first: 11026
    count: 5
splat_clauses:
- key: source_rules
  text: '# Immunities This entry lists the monster’s Immunities, if any. If the monster has damage and
    condition Immunities, the damage types are listed before the conditions. See “Playing the Game” for
    details.'
  sha256: ba3cc3c191525c2c39766b7a1d7502ee5abfad8f1750bbec4b5fa959cdc6dfa2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Immunities

<!-- SRD page 255 -->

This entry lists the monster’s Immunities, if any. If\
the monster has damage and condition Immunities,\
the damage types are listed before the conditions.\
See “Playing the Game” for details.\
