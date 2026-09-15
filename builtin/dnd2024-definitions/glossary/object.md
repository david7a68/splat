---
splat_schema: 1
splat_id: e4e21695-bdc2-5102-9875-0fc79aa154c9
splat_kind: rule
splat_key: dnd2024:glossary/object
splat_name: Object
splat_origin:
  key: dnd2024:reference/glossary/object
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/object
  parameters:
    living: false
    distinct: true
    composite_things: multiple_objects
  implementation: dnd2024:glossary_object
splat_contracts:
- id: dnd2024:glossary_object
  inputs:
    object:
      type: object
      required: true
  outputs:
  - object_classification
  events: []
  capabilities:
  - objects
  - targeting
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 8c7a5fb22d07154da1234f591ddd8f938c0904cb17cdf1bf635691f1a600743c
  source_path: builtin/srd-5.2.1/glossary/object.md
  pages:
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_object
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 1f8d9821007d012c90e8a5948b80571ab3351de8b242203194b7cc91f9d0c3e6
splat_clauses:
- key: source_rules
  text: '# Object An object is a nonliving, distinct thing. Composite things, like buildings, comprise
    more than one object. See also “Breaking Objects.”'
  sha256: 21aad6b320f174bb8e236a4975ebab29cfb471336daa34055eac1226e583feb7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Object

<!-- SRD page 185 -->

An object is a nonliving, distinct thing. Composite\
things, like buildings, comprise more than one ob-\
ject. See also “Breaking Objects.”\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].
