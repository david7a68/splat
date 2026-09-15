---
splat_schema: 1
splat_id: 7ee7b424-c870-5191-9014-f20da4182cf3
splat_kind: rule
splat_key: dnd2024:rules/creating_a_background_4_choose_a_tool_proficiency
splat_name: '4: Choose a Tool Proficiency'
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/creating_a_background_4_choose_a_tool_proficiency
  parameters:
    tool_proficiency_choice:
      count: 1
      criterion_any:
      - used_in_background_practice
      - associated_with_background
  implementation: dnd2024:rule_creating_a_background_4_choose_a_tool_proficiency
splat_contracts:
- id: dnd2024:rule_creating_a_background_4_choose_a_tool_proficiency
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
  - background_definition
  events: []
  capabilities:
  - background_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 193
  engine_gaps: &id001
  - dnd2024:rule_creating_a_background_4_choose_a_tool_proficiency
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 01ad9567a81683e00c67d7c92f7964e3def30854528f8f87b962a769630cf931
  parent_name: Creating a Background
  line_span:
    first: 9424
    count: 3
splat_clauses:
- key: source_rules
  text: '# 4: Choose a Tool Proficiency Choose one tool used in the practice of the background or often
    associated with it.'
  sha256: 44a696f889aa3db99a83ee527d475dd37e21e6a7a7dab202473a4aa522518f25
  status: contract_specified
splat_requires_mechanics: *id001
---

# 4: Choose a Tool Proficiency

<!-- SRD page 193 -->

Choose one tool used in the practice of the back-\
ground or often associated with it.\
