---
splat_schema: 1
splat_id: 463ae7df-d2d9-58a6-84b7-ff8761526d86
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_aristocratic_10_gp_per_day
splat_name: Aristocratic (10 GP per Day)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses_aristocratic_10_gp_per_day
  parameters:
    lifestyle:
      id: aristocratic
      cost_gp_per_day: 10
  implementation: dnd2024:rules_lifestyle_expenses_aristocratic_10_gp_per_day
splat_contracts:
- id: dnd2024:rules_lifestyle_expenses_aristocratic_10_gp_per_day
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
  - dnd2024:rules_lifestyle_expenses_aristocratic_10_gp_per_day
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 012cb3ac1c832bbeb700d702fbe0b663390c36090893e1abcb6b9764a183c7da
  parent_name: Lifestyle Expenses
  line_span:
    first: 8815
    count: 5
splat_clauses:
- key: source_rules
  text: '# Aristocratic (10 GP per Day) You pay for the best and might have a staff that supports your
    lifestyle. Others notice your wealth and might encourage you to share it, either legally or otherwise.'
  sha256: 560a6c6ee38932ebbd0fa407b5d9d50616a999db077bb0da882af3f11b0865a2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Aristocratic (10 GP per Day)

<!-- SRD page 101 -->

You pay for the best and might have a staff that sup-\
ports your lifestyle. Others notice your wealth and\
might encourage you to share it, either legally or\
otherwise.\
