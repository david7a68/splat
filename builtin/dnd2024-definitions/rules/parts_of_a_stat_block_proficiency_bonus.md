---
splat_schema: 1
splat_id: 6222314d-f514-5a90-add2-894fef32491a
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_proficiency_bonus
splat_name: Proficiency Bonus
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_proficiency_bonus
  parameters:
    proficiency_bonus_by_cr:
    - cr_min: 0
      cr_max: 4
      bonus: 2
    - cr_min: 5
      cr_max: 8
      bonus: 3
    - cr_min: 9
      cr_max: 12
      bonus: 4
    - cr_min: 13
      cr_max: 16
      bonus: 5
    - cr_min: 17
      cr_max: 20
      bonus: 6
    - cr_min: 21
      cr_max: 24
      bonus: 7
    - cr_min: 25
      cr_max: 28
      bonus: 8
    - cr_min: 29
      cr_max: 30
      bonus: 9
    already_included_in_stat_block:
    - saving_throws
    - skills
    - other_proficient_statistics
  implementation: dnd2024:rules_parts_of_a_stat_block_proficiency_bonus
splat_contracts:
- id: dnd2024:rules_parts_of_a_stat_block_proficiency_bonus
  inputs:
    monster:
      type: participant
      required: true
  outputs:
  - proficiency_bonus
  events: []
  capabilities:
  - monster_statistics
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rules_parts_of_a_stat_block_proficiency_bonus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 31971144b67d4c117d17e04c32a7fcb5e042fc4a9d3e0b475d4242da16328f02
  parent_name: Parts of a Stat Block
  line_span:
    first: 11113
    count: 12
splat_clauses:
- key: source_rules
  text: '# Proficiency Bonus A monster’s Proficiency Bonus (PB) is determined by its CR, as shown in the
    Proficiency Bonus by Challenge Rating table, and is reflected in saving throws, skills, and other
    statistics where the monster’s exceptional aptitude is a factor. Proficiency Bonus by Challenge Rating
    CR PB CR PB 0–4 +2 17–20 +6 5–8 +3 21–24 +7 9–12 +4 25–28 +8 13–16 +5 29–30 +9'
  sha256: cd11d1572e9399c80518e0a7f90973567e54edc5a70af334eb1670222b76e409
  status: contract_specified
splat_requires_mechanics: *id001
---

# Proficiency Bonus

<!-- SRD page 256 -->

A monster’s Proficiency Bonus (PB) is determined\
by its CR, as shown in the Proficiency Bonus by\
Challenge Rating table, and is reflected in saving\
throws, skills, and other statistics where the mon-\
ster’s exceptional aptitude is a factor.\
Proficiency Bonus by Challenge Rating\
CR PB CR PB\
0–4 +2 17–20 +6\
5–8 +3 21–24 +7\
9–12 +4 25–28 +8\
13–16 +5 29–30 +9\
