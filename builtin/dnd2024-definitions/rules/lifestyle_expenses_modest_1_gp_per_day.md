---
splat_schema: 1
splat_id: 234d9c45-9016-522c-bc89-8194436e55f9
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_modest_1_gp_per_day
splat_name: Modest (1 GP per Day)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses_modest_1_gp_per_day
  parameters:
    lifestyle:
      id: modest
      cost_gp_per_day: 1
  implementation: dnd2024:rules_lifestyle_expenses_modest_1_gp_per_day
splat_contracts:
- id: dnd2024:rules_lifestyle_expenses_modest_1_gp_per_day
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
    days:
      required: false
      type: integer
  outputs:
  - cost
  - availability
  events: []
  capabilities:
  - economy
  - services
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 101
  engine_gaps: &id001
  - dnd2024:rules_lifestyle_expenses_modest_1_gp_per_day
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 28fe70e80032d7640d0fcd92c966066a862516a56e95a7ea11116c883038934f
  parent_name: Lifestyle Expenses
  line_span:
    first: 8807
    count: 2
splat_clauses:
- key: source_rules
  text: '# Modest (1 GP per Day) You support yourself at an average level.'
  sha256: 0fc3f4f0cb06e5823bf90fab6bfab75b68a70663c2081085514e1657fd867843
  status: contract_specified
splat_requires_mechanics: *id001
---

# Modest (1 GP per Day)

<!-- SRD page 101 -->

You support yourself at an average level.\
