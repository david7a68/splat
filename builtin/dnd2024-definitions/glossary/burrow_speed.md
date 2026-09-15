---
splat_schema: 1
splat_id: 1eb79bfb-fa78-5a62-8292-7a5776bc557c
splat_kind: rule
splat_key: dnd2024:glossary/burrow_speed
splat_name: Burrow Speed
splat_origin:
  key: dnd2024:reference/glossary/burrow_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/burrow_speed
  parameters:
    movement_mode: burrow
    allowed_materials:
    - sand
    - earth
    - mud
    - ice
    solid_rock:
      allowed: false
      exception: trait_allows_rock_burrowing
    distance_budget: burrow_speed
  implementation: dnd2024:glossary_burrow_speed
splat_contracts:
- id: dnd2024:glossary_burrow_speed
  inputs:
    subject:
      type: participant
      required: true
    route:
      type: location_list
      required: true
  outputs:
  - movement_eligibility
  - movement_budget
  events: []
  capabilities:
  - movement
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 101a4c9213071de7884c01778f2cd757a34372a5dc41d2383b5087b8bb87ed35
  source_path: builtin/srd-5.2.1/glossary/burrow_speed.md
  pages:
  - 178
  engine_gaps: &id001
  - dnd2024:glossary_burrow_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 6ee6d306f2847d289e76e613486d6695354d5b66389957c159aa7e4aeadf9020
splat_clauses:
- key: source_rules
  text: '# Burrow Speed A creature that has a Burrow Speed can use that speed to move through sand, earth,
    mud, or ice. The creature can’t burrow through solid rock unless the creature has a trait that allows
    it to do so. See also “Speed.”'
  sha256: 8ef6bac22621737d62bc35d243f933b903ad52812b7ab45fb9973687997799a7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Burrow Speed

<!-- SRD page 178 -->

A creature that has a Burrow Speed can use that\
speed to move through sand, earth, mud, or ice. The\
creature can’t burrow through solid rock unless the\
creature has a trait that allows it to do so. See also\
“Speed.”\

Original layout: [[../../srd-5.2.1/pages/178|page 178]].
