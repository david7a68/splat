---
splat_schema: 1
splat_id: 5553e4ad-2727-5db7-a798-e8cd1e2acae7
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_ability_scores
splat_name: Ability Scores
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_ability_scores
  parameters:
    abilities:
    - strength
    - dexterity
    - constitution
    - intelligence
    - wisdom
    - charisma
    each_has:
    - score
    - ability_modifier
    - saving_throw_modifier
    source: stat_block
  implementation: dnd2024:rules_parts_of_a_stat_block_ability_scores
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_ability_scores
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - ability_statistics
  events: []
  capabilities:
  - monster_statistics
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_ability_scores
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 27ae23d74f65a8436780f5ae75632370722249beced5016d59fa549a6689414d
  parent_name: Parts of a Stat Block
  line_span:
    first: 10993
    count: 5
splat_clauses:
- key: source_rules
  text: '# Ability Scores Every monster has six ability scores along with corresponding ability score
    modifiers and saving throw modifiers. For more information on ability scores and saving throws, see
    “Playing the Game.”'
  sha256: 926657cb0071b04a97ae7972c2387b1107acdbf15321e7cb17c20ad0c10637ca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ability Scores

<!-- SRD page 255 -->

Every monster has six ability scores along with\
corresponding ability score modifiers and saving\
throw modifiers. For more information on ability\
scores and saving throws, see “Playing the Game.”\
