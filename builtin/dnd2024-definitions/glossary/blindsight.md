---
splat_schema: 1
splat_id: 409feeac-c398-5165-908b-69d2c6eafe0a
splat_kind: rule
splat_key: dnd2024:glossary/blindsight
splat_name: Blindsight
splat_origin:
  key: dnd2024:reference/glossary/blindsight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/blindsight
  parameters:
    range: feature_specified
    physical_sight_required: false
    sees_while:
    - blinded
    - darkness
    sees_invisible: true
    total_cover_blocks: true
  implementation: dnd2024:glossary_blindsight
splat_contracts:
- id: dnd2024:glossary_blindsight
  inputs:
    subject:
      type: participant
      required: true
    target:
      type: object
      required: true
    range_feet:
      type: number
      required: true
    geometry:
      type: resolution
      required: true
  outputs:
  - can_see
  events: []
  capabilities:
  - senses
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 7cd0fbd1964e056f9ddc13e1256b7f2490ed54f8e7e3ac1362a4a79089f3b2cc
  source_path: builtin/srd-5.2.1/glossary/blindsight.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_blindsight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 3330bb525d39604134e75f9c1a07ac8d769ddace1dc4fea520323134a5f96788
splat_clauses:
- key: source_rules
  text: '# Blindsight If you have Blindsight, you can see within a specific range without relying on physical
    sight. Within that range, you can see anything that isn’t behind Total Cover even if you have the
    Blinded condition or are in Darkness. Moreover, in that range, you can see something that has the
    Invisible condition.'
  sha256: 5e6bbd27746443d42dd662f6c642fe14c22c7e98467fe5943b74f6b649966f45
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blindsight

<!-- SRD page 177 -->

If you have Blindsight, you can see within a specific\
range without relying on physical sight. Within that\
range, you can see anything that isn’t behind Total\
Cover even if you have the Blinded condition or are\
in Darkness. Moreover, in that range, you can see\
something that has the Invisible condition.\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].
