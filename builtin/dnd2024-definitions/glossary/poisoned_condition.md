---
splat_schema: 1
splat_id: 61eafb1b-773b-5cae-99d6-17e7671b0da8
splat_kind: rule
splat_key: dnd2024:glossary/poisoned_condition
splat_name: Poisoned [Condition]
splat_origin:
  key: dnd2024:reference/glossary/poisoned_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/poisoned_condition
  parameters:
    condition: poisoned
    own_attack_rolls: disadvantage
    ability_checks: disadvantage
  implementation: dnd2024:glossary_poisoned_condition
splat_contracts:
- id: dnd2024:glossary_poisoned_condition
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - condition_effects
  events:
  - attack.pending
  - ability_check.pending
  capabilities:
  - conditions
  - checks
  - attacks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 678b03e81971544f273f4ef3e91328cc78d7aee55b690457a846d2b183c5bdc8
  source_path: builtin/srd-5.2.1/glossary/poisoned_condition.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_poisoned_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 3920079a28d94264485b026729bde2c144aefa0f00e9607c1325d8d66f257683
splat_clauses:
- key: source_rules
  text: '# Poisoned [Condition] While you have the Poisoned condition, you experience the following effect.
    Ability Checks and Attacks Affected. You have Disadvantage on attack rolls and ability checks.'
  sha256: 7439a484b939ea5381997f526da492b2ddefeeb32a3573e16ff0094772a9c7c3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Poisoned [Condition]

<!-- SRD page 186 -->

While you have the Poisoned condition, you experi-\
ence the following effect.\
Ability Checks and Attacks Affected. You have\
Disadvantage on attack rolls and ability checks.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
