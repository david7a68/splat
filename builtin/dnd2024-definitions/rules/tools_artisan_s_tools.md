---
splat_schema: 1
splat_id: 160711a9-e907-5591-8d9e-ce21611d792b
splat_kind: rule
splat_key: dnd2024:rules/tools_artisan_s_tools
splat_name: Artisan’s Tools
splat_origin:
  key: dnd2024:reference/pages/093
splat_sources:
- title: SRD 5.2.1
  locator: pp. 93
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/tools_artisan_s_tools
  parameters:
    tool_category: artisans_tools
    separate_proficiency_per_tool: true
  implementation: dnd2024:rules_tools_artisan_s_tools
splat_contracts:
- id: dnd2024:rules_tools_artisan_s_tools
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - tool_proficiency_constraints
  events: []
  capabilities:
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 93
  engine_gaps: &id001
  - dnd2024:rules_tools_artisan_s_tools
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 36f0d61490ebb8e0221a6cc15d6568cca96cd9e84e150391065ff5c2fa328011
  parent_name: Tools
  line_span:
    first: 8005
    count: 4
splat_clauses:
- key: source_rules
  text: '# Artisan’s Tools Artisan’s Tools are each focused on crafting items and pursuing a trade. Each
    of these tools requires a separate proficiency.'
  sha256: cf771b41aaf43345a896ff721b7cc04a078448ac0cf90a63bb3f4189e2720a6a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Artisan’s Tools

<!-- SRD page 93 -->

Artisan’s Tools are each focused on crafting items\
and pursuing a trade. Each of these tools requires a\
separate proficiency.\
