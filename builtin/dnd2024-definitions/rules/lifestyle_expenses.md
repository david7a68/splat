---
splat_schema: 1
splat_id: 143844d5-cccb-5c09-ae59-0074289a55e7
splat_kind: rule
splat_key: dnd2024:rules/lifestyle_expenses
splat_name: Lifestyle Expenses
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/lifestyle_expenses
  parameters:
    payment_interval_choice:
      actor: gm
      options:
      - week
      - month
    selection_at: interval_start
    lifestyles:
    - wretched
    - squalid
    - poor
    - modest
    - comfortable
    - wealthy
    - aristocratic
    pay: daily_price_times_interval_days
    covers:
    - lodging
    - food
    - equipment_maintenance
    - necessities
    inherent_consequences: false
    gm_considerations:
    - risks
    - social_perception
  implementation: dnd2024:rule_lifestyle_expenses
splat_contracts:
- id: dnd2024:rule_lifestyle_expenses
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
  - dnd2024:rule_lifestyle_expenses
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: d76616fe1daee93c84fb691ab98b399f779e8e6a4892ff34bce1767aaa04bb78
  parent_name: Lifestyle Expenses
  line_span:
    first: 8778
    count: 11
splat_clauses:
- key: source_rules
  text: '# Lifestyle Expenses Lifestyle expenses summarize the cost of living in a fantasy world. They
    cover lodging, food, equipment maintenance, and other necessities. At the start of each week or month
    (GM’s choice), choose a lifestyle below—Wretched, Squalid, Poor, Modest, Comfortable, Wealthy, or
    Aristocratic—and pay the price to sustain that lifestyle. Lifestyles have no inherent consequences,
    but the GM might take them into account when determining risks or how others perceive your character.'
  sha256: 61804c404c2f1b2211c4dfbef290bf005aa622f430568f4eaf606ace2a6fb6e3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lifestyle Expenses

<!-- SRD page 101 -->

Lifestyle expenses summarize the cost of living in a\
fantasy world. They cover lodging, food, equipment\
maintenance, and other necessities.\
At the start of each week or month (GM’s choice),\
choose a lifestyle below—Wretched, Squalid, Poor,\
Modest, Comfortable, Wealthy, or Aristocratic—and\
pay the price to sustain that lifestyle.\
Lifestyles have no inherent consequences, but the\
GM might take them into account when determining\
risks or how others perceive your character.\
