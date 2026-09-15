---
splat_schema: 1
splat_id: e2e011ec-40d9-5e52-99eb-e06a89ee9bda
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses_poor_2_sp_per_day
splat_name: Poor (2 SP per Day)
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses_poor_2_sp_per_day
  parameters:
    lifestyle: poor
    cost_per_day:
      currency: sp
      amount: 2
  implementation: dnd2024:rule_lifestyle_expenses_poor_2_sp_per_day
splat_contracts:
- id: dnd2024:rule_lifestyle_expenses_poor_2_sp_per_day
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
  - dnd2024:rule_lifestyle_expenses_poor_2_sp_per_day
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 688de622eee86f1083fa642c370a79249d878a9834485b7b3cf0daa1088a3130
  parent_name: Lifestyle Expenses
  line_span:
    first: 8805
    count: 2
splat_clauses:
- key: source_rules
  text: '# Poor (2 SP per Day) You spend frugally for your necessities.'
  sha256: b4faa45d58deadbaa54604bad9ad6eeebcc17a69aede413933173e3320ebde63
  status: contract_specified
splat_requires_mechanics: *id001
---

# Poor (2 SP per Day)

<!-- SRD page 101 -->

You spend frugally for your necessities.\
