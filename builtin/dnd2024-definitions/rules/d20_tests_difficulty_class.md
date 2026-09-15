---
splat_schema: 1
splat_id: 3d42e922-7a23-57de-82c0-8a9e62741a01
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_difficulty_class
splat_name: Difficulty Class
splat_origin:
  key: dnd2024:reference/pages/006
splat_sources:
- title: SRD 5.2.1
  locator: pp. 6
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_difficulty_class
  parameters:
    dc_authority: gm
    specific_dc_priority: applicable_rule_subject_to_gm
    suggested_dc:
      very_easy: 5
      easy: 10
      medium: 15
      hard: 20
      very_hard: 25
      nearly_impossible: 30
  implementation: dnd2024:rules_d20_tests_difficulty_class
splat_contracts:
- id: dnd2024:rules_d20_tests_difficulty_class
  inputs:
    task:
      type: resolution
      required: true
    difficulty:
      type: choice
      required: true
  outputs:
  - difficulty_class
  events: []
  capabilities:
  - checks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 6
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_difficulty_class
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 26775a492bcb8c66ac5dec325879412b08cca487a6e3c1269cc7a25c2a51cdbf
  parent_name: D20 Tests
  line_span:
    first: 176
    count: 13
splat_clauses:
- key: source_rules
  text: '# Difficulty Class The Difficulty Class of an ability check represents the task’s difficulty.
    The more difficult the task, the higher its DC. The rules provide DCs for certain checks, but the
    GM ultimately sets them. The Typical Difficulty Classes table presents a range of possible DCs for
    ability checks. Typical Difficulty Classes Task Difficulty DC Task Difficulty DC Very easy 5 Very
    hard 25 Easy 10 Nearly impossible 30 Medium 15 Hard 20'
  sha256: 2551efaf065b4f2c93b976c65cb1173eff24d420a53c445f8e9d1da3d8743660
  status: contract_specified
splat_requires_mechanics: *id001
---

# Difficulty Class

<!-- SRD page 6 -->

The Difficulty Class of an ability check represents\
the task’s difficulty. The more difficult the task,\
the higher its DC. The rules provide DCs for certain\
checks, but the GM ultimately sets them. The Typi-\
cal Difficulty Classes table presents a range of pos-\
sible DCs for ability checks.\
Typical Difficulty Classes\
Task Difficulty DC Task Difficulty DC\
Very easy 5 Very hard 25\
Easy 10 Nearly impossible 30\
Medium 15\
Hard 20\
