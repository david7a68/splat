---
splat_schema: 1
splat_id: 6bf75f41-0a37-5908-b176-3bfafd2866b9
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_wealthy_4_gp_per_day
splat_name: Wealthy (4 GP per Day)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses_wealthy_4_gp_per_day
  parameters:
    lifestyle: wealthy
    cost_per_day:
      currency: gp
      amount: 4
  implementation: dnd2024:rule_lifestyle_expenses_wealthy_4_gp_per_day
splat_contracts:
- id: dnd2024:rule_lifestyle_expenses_wealthy_4_gp_per_day
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - costs
  events: []
  capabilities:
  - economy
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  engine_gaps: &id001
  - dnd2024:rule_lifestyle_expenses_wealthy_4_gp_per_day
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 6850f6ed070a3f81d5dee833bd75056de43e1c43946f33c1dfd3e835c7e402f9
  parent_name: Lifestyle Expenses
  line_span:
    first: 8812
    count: 3
splat_clauses:
- key: source_rules
  text: '# Wealthy (4 GP per Day) You’re accustomed to the finer things in life and might have servants.'
  sha256: 57958979a0b38d59c9bdb47b04f710a008276fe6560b5ebc73a73d12cffd082c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wealthy (4 GP per Day)

<!-- SRD page 101 -->

You’re accustomed to the finer things in life and\
might have servants.\
