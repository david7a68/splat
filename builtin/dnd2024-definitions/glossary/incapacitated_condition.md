---
splat_schema: 1
splat_id: 552a4ef9-25fc-50b3-be7d-860815b208d9
splat_kind: rule
splat_key: dnd2024:glossary/incapacitated_condition
splat_name: Incapacitated [Condition]
splat_origin:
  key: dnd2024:reference/glossary/incapacitated_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/incapacitated_condition
  parameters:
    condition: incapacitated
    forbidden_economy:
    - action
    - bonus_action
    - reaction
    concentration_broken: true
    can_speak: false
    initiative_roll: disadvantage
  implementation: dnd2024:glossary_incapacitated_condition
splat_contracts:
- id: dnd2024:glossary_incapacitated_condition
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - condition_effects
  events:
  - condition.applied
  - action.requested
  - bonus_action.requested
  - reaction.requested
  - initiative.rolled
  capabilities:
  - conditions
  - action_economy
  - concentration
  - initiative
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 4cf1a9beeb55dd4f32710198950c601c452f600ddeed4684b69e5ec48dd6be55
  source_path: builtin/srd-5.2.1/glossary/incapacitated_condition.md
  pages:
  - 184
  engine_gaps: &id001
  - dnd2024:glossary_incapacitated_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 9c38805b41f81f14ffa5fa0bc043a6d7cdf56c62712449810cbb81de342f7b2e
splat_clauses:
- key: source_rules
  text: '# Incapacitated [Condition] While you have the Incapacitated condition, you experience the following
    effects. Inactive. You can’t take any action, Bonus Action, or Reaction. No Concentration. Your Concentration
    is broken. Speechless. You can’t speak. Surprised. If you’re Incapacitated when you roll Initiative,
    you have Disadvantage on the roll.'
  sha256: ecd6ce7bdcb3a076c616a9bc654fa63263574bd06cca2612438a6d39ca7d0363
  status: contract_specified
splat_requires_mechanics: *id001
---

# Incapacitated [Condition]

<!-- SRD page 184 -->

While you have the Incapacitated condition, you ex-\
perience the following effects.\
Inactive. You can’t take any action, Bonus Action,\
or Reaction.\
No Concentration. Your Concentration is broken.\
Speechless. You can’t speak.\
Surprised. If you’re Incapacitated when you roll\
Initiative, you have Disadvantage on the roll.\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].
