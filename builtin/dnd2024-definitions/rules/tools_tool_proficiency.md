---
splat_schema: 1
splat_id: ac696a4c-7355-59a2-a72d-a4bb01a883a9
splat_kind: rule
splat_key: dnd2024:rules/tools_tool_proficiency
splat_name: Tool Proficiency
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/tools_tool_proficiency
  parameters:
    tool_check_proficiency_bonus: true
    also_proficient_in_used_skill: advantage
    monster_proficiency: all_stat_block_tools
  implementation: dnd2024:rules_tools_tool_proficiency
splat_contracts:
- id: dnd2024:rules_tools_tool_proficiency
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 93
  engine_gaps: &id001
  - dnd2024:rules_tools_tool_proficiency
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 98201122b79429d4b83099523e8e6528bc943ad8b4cd17136a98409af9068bc5
  parent_name: Tools
  line_span:
    first: 7996
    count: 9
splat_clauses:
- key: source_rules
  text: '# Tool Proficiency If you have proficiency with a tool, add your Proficiency Bonus to any ability
    check you make that uses the tool. If you have proficiency in a skill that’s used with that check,
    you have Advantage on the check too. Your features might give you proficiency with a tool. A monster
    has proficiency with any tool in its stat block.'
  sha256: 458d4c967153a2f76760b95eb57f59ab65efee2f6b41055f15a030ad52742601
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tool Proficiency

<!-- SRD page 93 -->

If you have proficiency with a tool, add your Pro-\
ficiency Bonus to any ability check you make that\
uses the tool. If you have proficiency in a skill that’s\
used with that check, you have Advantage on the\
check too.\
Your features might give you proficiency with a\
tool. A monster has proficiency with any tool in its\
stat block.\
