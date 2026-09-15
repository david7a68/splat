---
splat_schema: 1
splat_id: 4a8712e4-58ae-52e5-9d1e-d363c4fcfb6c
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_squalid_1_sp_per_day
splat_name: Squalid (1 SP per Day)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses_squalid_1_sp_per_day
  parameters:
    lifestyle: squalid
    cost_gp_per_day: 0.1
  implementation: dnd2024:rules_lifestyle_expenses_squalid_1_sp_per_day
splat_contracts:
- id: dnd2024:rules_lifestyle_expenses_squalid_1_sp_per_day
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
  - dnd2024:rules_lifestyle_expenses_squalid_1_sp_per_day
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8cec685357abd2882882d16bae251e67509e6bdb8c1dd41c923d0dc2a4bf063a
  parent_name: Lifestyle Expenses
  line_span:
    first: 8801
    count: 4
splat_clauses:
- key: source_rules
  text: '# Squalid (1 SP per Day) You spend the bare minimum for your necessities. You might be exposed
    to unhealthy conditions and opportunistic criminals.'
  sha256: e92b0fe34243fcdce00bd0a35d0465038851f7aa90f5920542e0d761b39c6315
  status: contract_specified
splat_requires_mechanics: *id001
---

# Squalid (1 SP per Day)

<!-- SRD page 101 -->

You spend the bare minimum for your necessities.\
You might be exposed to unhealthy conditions and\
opportunistic criminals.\
