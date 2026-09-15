---
splat_schema: 1
splat_id: 28b395b2-3cf1-51df-b7c9-4a3388036ea6
splat_kind: rule
splat_key: dnd2024:glossary/sphere_area_of_effect
splat_name: Sphere [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/sphere_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/sphere_area_of_effect
  parameters:
    shape: sphere
    propagation: straight_lines
    base_area_rule:
      key: dnd2024:glossary/area_of_effect
    origin: center
    direction: all
    radius: effect_specified
    origin_included: true
    geometry_provider: external_geometry
  implementation: dnd2024:glossary_sphere_area_of_effect
splat_contracts:
- id: dnd2024:glossary_sphere_area_of_effect
  inputs:
    creator:
      type: participant
      required: true
    origin:
      type: location
      required: true
    dimensions:
      type: choice
      required: true
    geometry:
      type: resolution
      required: true
    include_origin:
      required: false
      type: boolean
  outputs:
  - area
  - included_locations
  events: []
  capabilities:
  - areas
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: b245f631e1fb9df8eed6579ccb8ccc5e03a1bc7586d5ec192f94215065f62dca
  source_path: builtin/srd-5.2.1/glossary/sphere_area_of_effect.md
  pages:
  - 188
  engine_gaps: &id001
  - dnd2024:glossary_sphere_area_of_effect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 82e676dad0112ed7475dec028513e2377518bbfb2de34edf42018bf168bbafa2
splat_clauses:
- key: source_rules
  text: '# Sphere [Area of Effect] A Sphere is an area of effect that extends in straight lines from a
    point of origin outward in all directions. The effect that creates a Sphere specifies the distance
    it extends as the radius of the Sphere. A Sphere’s point of origin is included in the Sphere’s area
    of effect.'
  sha256: 6512593cd8f4239b3c84585ef447f84e73c07e9e403508c337c59c910757bc4d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sphere [Area of Effect]

<!-- SRD page 188 -->

A Sphere is an area of effect that extends in straight\
lines from a point of origin outward in all direc-\
tions. The effect that creates a Sphere specifies the\
distance it extends as the radius of the Sphere.\
A Sphere’s point of origin is included in the\
Sphere’s area of effect.\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].
