---
splat_schema: 1
splat_id: 9b83be13-c33e-586a-aeb4-2f3ba7817c6b
splat_kind: rule
splat_key: dnd2024:rules/crafting_nonmagical_items_tools
splat_name: Tools
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/crafting_nonmagical_items_tools
  parameters:
    required_tool_source: tool_craft_table
    unlisted_required_tool:
      actor: gm
      choice: tool
    must_use_required_tool: true
    proficiency_required_for:
    - crafter
    - every_assistant
  implementation: dnd2024:rule_crafting_nonmagical_items_tools
splat_contracts:
- id: dnd2024:rule_crafting_nonmagical_items_tools
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
  - crafted_items
  events: []
  capabilities:
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 103
  engine_gaps: &id001
  - dnd2024:rule_crafting_nonmagical_items_tools
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 13259806e5fb222941d307d21c531a2334ef8b04bba3ca22055feb37b1cfc0a4
  parent_name: Crafting Nonmagical Items
  line_span:
    first: 8965
    count: 7
splat_clauses:
- key: source_rules
  text: '# Tools The “Tools” section of “Equipment” lists which tools are required to make certain items.
    The GM assigns required tools for items not listed there. You must use the required tool to make an
    item and have proficiency with that tool. Anyone who helps you must also have proficiency with it.'
  sha256: e53841db2fffd80d71b1a27daa3716cbb5430fb833d3a2af1e36401b94b4412c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tools

<!-- SRD page 103 -->

The “Tools” section of “Equipment” lists which tools\
are required to make certain items. The GM assigns\
required tools for items not listed there.\
You must use the required tool to make an item\
and have proficiency with that tool. Anyone who\
helps you must also have proficiency with it.\
