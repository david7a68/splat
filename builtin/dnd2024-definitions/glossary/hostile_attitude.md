---
splat_schema: 1
splat_id: 3c3f5909-070e-53de-87dd-451f85616a4b
splat_kind: rule
splat_key: dnd2024:glossary/hostile_attitude
splat_name: Hostile [Attitude]
splat_origin:
  key: dnd2024:reference/glossary/hostile_attitude
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/hostile_attitude
  parameters:
    attitude: hostile
    influence_ability_check_mode: disadvantage
  implementation: dnd2024:glossary_hostile_attitude
splat_contracts:
- id: dnd2024:glossary_hostile_attitude
  inputs:
    subject:
      type: participant
      required: true
    target:
      type: participant
      required: true
  outputs:
  - roll_mode
  events:
  - ability_check.pending
  capabilities:
  - relationships
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 7591e1548b2e14107051cddb5f8af44e85a79e0c6b22a28e6699193bee191c0d
  source_path: builtin/srd-5.2.1/glossary/hostile_attitude.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_hostile_attitude
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 8f1e4924e4f0f4c431cbedb012598b54f5442bf3cf2dbb57ec746b9fa12fa52f
splat_clauses:
- key: source_rules
  text: '# Hostile [Attitude] A Hostile creature views you unfavorably. You have Disadvantage on an ability
    check to influence a Hostile creature. See also “Influence.”'
  sha256: 0ea2ad9a4383efa9838fb5522bd92035db9e30b7ab1a484d38708a4f0378e834
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hostile [Attitude]

<!-- SRD page 183 -->

A Hostile creature views you unfavorably. You have\
Disadvantage on an ability check to influence a Hos-\
tile creature. See also “Influence.”\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
