---
splat_schema: 1
splat_id: 7ec5c6bc-1f82-5d91-a8ef-0125d960d85e
splat_kind: rule
splat_key: dnd2024:glossary/tremorsense
splat_name: Tremorsense
splat_origin:
  key: dnd2024:reference/glossary/tremorsense
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/tremorsense
  parameters:
    range: feature_specified
    detects:
    - creatures
    - moving_objects
    shared_contact_required_any:
    - same_surface
    - same_liquid
    surface_examples:
    - ground
    - wall
    - ceiling
    airborne_targets_detectable: false
    counts_as_sight: false
    result: pinpoint_location
  implementation: dnd2024:glossary_tremorsense
splat_contracts:
- id: dnd2024:glossary_tremorsense
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
    contact_facts:
      type: resolution
      required: true
  outputs:
  - detected_location
  events: []
  capabilities:
  - senses
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: d7112c46c3d51e544e20626c164801ad2f4bc21e1b1b9a116a17fc58f6c45573
  source_path: builtin/srd-5.2.1/glossary/tremorsense.md
  pages:
  - 190
  engine_gaps: &id001
  - dnd2024:glossary_tremorsense
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 982f622a17a35ffa76010f9e3fea64a6bf84d01268243a85a420ec7f17619d2f
splat_clauses:
- key: source_rules
  text: '# Tremorsense A creature with Tremorsense can pinpoint the location of creatures and moving objects
    within a specific range, provided that the creature with Tremorsense and anything it is detecting
    are both in contact with the same surface (such as the ground, a wall, or a ceiling) or the same liquid.
    Tremorsense can’t detect creatures or objects in the air, and it doesn’t count as a form of sight.'
  sha256: 2ce1540e7fe92e942b307fc841e4706eb317ed06e5fe47998638ca3f5a031634
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tremorsense

<!-- SRD page 190 -->

A creature with Tremorsense can pinpoint the loca-\
tion of creatures and moving objects within a spe-\
cific range, provided that the creature with Trem-\
orsense and anything it is detecting are both in\
contact with the same surface (such as the ground,\
a wall, or a ceiling) or the same liquid.\
Tremorsense can’t detect creatures or objects in\
the air, and it doesn’t count as a form of sight.\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].
