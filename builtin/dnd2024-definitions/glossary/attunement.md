---
splat_schema: 1
splat_id: 2ab77d10-2d87-547e-a68e-d3261669781e
splat_kind: rule
splat_key: dnd2024:glossary/attunement
splat_name: Attunement
splat_origin:
  key: dnd2024:reference/glossary/attunement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/attunement
  parameters:
    maximum_items: 3
    magical_properties_require_attunement_if_item_requires: true
  implementation: dnd2024:glossary_attunement
splat_contracts:
- id: dnd2024:glossary_attunement
  inputs:
    subject:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
  outputs:
  - attunement_eligibility
  - property_access
  events:
  - attunement.requested
  - item.activation_pending
  capabilities:
  - equipment
  - attunement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 013878e28bac934447b7dea22019d11f1a5becb522e99569dfd961c42b3f67c5
  source_path: builtin/srd-5.2.1/glossary/attunement.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_attunement
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: a40d809b60f0afac53d7cabf129365fa49015f90a80460a3cacf48b0c7c9e63b
splat_clauses:
- key: source_rules
  text: '# Attunement Some magic items require a creature to form a bond—called Attunement—with them before
    the creature can use an item’s magical properties. A creature can have Attunement with no more than
    three magic items at a time. See also “Equipment” (“Magic Items”).'
  sha256: 3ed1f6e4900c5e4063efa85e87540f615d5ea13b736d94357a3b1147697b64c6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attunement

<!-- SRD page 177 -->

Some magic items require a creature to form a\
bond—called Attunement—with them before the\
creature can use an item’s magical properties. A\
creature can have Attunement with no more than\
three magic items at a time. See also “Equipment”\
(“Magic Items”).\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].
