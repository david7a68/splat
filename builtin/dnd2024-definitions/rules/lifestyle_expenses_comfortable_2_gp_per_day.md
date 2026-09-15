---
splat_schema: 1
splat_id: 0684e074-2574-5b2e-ab43-07f5eaaa6aed
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_comfortable_2_gp_per_day
splat_name: Comfortable (2 GP per Day)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses_comfortable_2_gp_per_day
  parameters:
    lifestyle: comfortable
    cost_gp_per_day: 2
  implementation: dnd2024:rules_lifestyle_expenses_comfortable_2_gp_per_day
splat_contracts:
- id: dnd2024:rules_lifestyle_expenses_comfortable_2_gp_per_day
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - lifestyle_cost
  events: []
  capabilities:
  - currency
  - downtime
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  engine_gaps: &id001
  - dnd2024:rules_lifestyle_expenses_comfortable_2_gp_per_day
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8789786dc5a149b6812124ec1c6e1be20c8e7b05c5dc8e0ff3c28c2d9e3967d8
  parent_name: Lifestyle Expenses
  line_span:
    first: 8809
    count: 3
splat_clauses:
- key: source_rules
  text: '# Comfortable (2 GP per Day) You spend modestly for your necessities and enjoy a few luxuries.'
  sha256: 03608a3f8d12cd914a74af8165ad8d579891c0c3de5605a10225cc5188e673da
  status: contract_specified
splat_requires_mechanics: *id001
---

# Comfortable (2 GP per Day)

<!-- SRD page 101 -->

You spend modestly for your necessities and enjoy a\
few luxuries.\
