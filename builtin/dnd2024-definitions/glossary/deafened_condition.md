---
splat_schema: 1
splat_id: 54122f4f-d930-54c4-83a6-391ba48a52e7
splat_kind: rule
splat_key: dnd2024:glossary/deafened_condition
splat_name: Deafened [Condition]
splat_origin:
  key: dnd2024:reference/glossary/deafened_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/deafened_condition
  parameters:
    condition: deafened
    can_hear: false
    automatic_failure:
      check_kind: ability_check
      requires_hearing: true
  implementation: dnd2024:glossary_deafened_condition
splat_contracts:
- id: dnd2024:glossary_deafened_condition
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - condition_effects
  events:
  - condition.changed
  - ability_check.pending
  capabilities:
  - conditions
  - senses
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: e1652625bf77dc65185d668a9eb140a123b03a7c4edeb362aa83c548047eb2fc
  source_path: builtin/srd-5.2.1/glossary/deafened_condition.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_deafened_condition
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: b9c7ea78ebd39647df8c879969a26512f5aaf1273ae36eb0a9d640e8595dde2b
splat_clauses:
- key: source_rules
  text: '# Deafened [Condition] While you have the Deafened condition, you experience the following effect.
    Can’t Hear. You can’t hear and automatically fail any ability check that requires hearing.'
  sha256: aff6af6be43d73e05bfb5f3efd543d0cbc6ed2bf5252d6af4ffad5fea8bfca7d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Deafened [Condition]

<!-- SRD page 181 -->

While you have the Deafened condition, you experi-\
ence the following effect.\
Can’t Hear. You can’t hear and automatically fail\
any ability check that requires hearing.\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
