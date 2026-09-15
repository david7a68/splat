---
splat_schema: 1
splat_id: b48abbf4-267e-521b-9df5-36abfef6f063
splat_kind: rule
splat_key: dnd2024:glossary/cylinder_area_of_effect
splat_name: Cylinder [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/cylinder_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/cylinder_area_of_effect
  parameters:
    shape: cylinder
    propagation: straight_lines
    base_area_rule:
      key: dnd2024:glossary/area_of_effect
    origin: center_of_top_or_bottom
    origin_face:
      chooser: creator
      options:
      - top
      - bottom
    base_radius: effect_specified
    height: effect_specified
    origin_included: true
    geometry_provider: external_geometry
  implementation: dnd2024:glossary_cylinder_area_of_effect
splat_contracts:
- id: dnd2024:glossary_cylinder_area_of_effect
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
  source_sha256: 7f16faafa54972ca2fcd9592e0967e38930e9fc41e164a4953f30f0b59c87878
  source_path: builtin/srd-5.2.1/glossary/cylinder_area_of_effect.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_cylinder_area_of_effect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 245a549ba3a283721478cc5761ec97f069f94195414646811d4fd9d434702bef
splat_clauses:
- key: source_rules
  text: '# Cylinder [Area of Effect] A Cylinder is an area of effect that extends in straight lines from
    a point of origin located at the center of the circular top or bottom of the Cylinder. The effect
    that creates a Cylinder specifies the radius of the Cylinder’s base and the Cylinder’s height. A Cylinder’s
    point of origin is included in the area of effect.'
  sha256: 96681fa2b69e12d94f2b83d95af20e1fd153ba3c6252d2c51a1d30ee64c9a6e6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cylinder [Area of Effect]

<!-- SRD page 180 -->

A Cylinder is an area of effect that extends in\
straight lines from a point of origin located at the\
center of the circular top or bottom of the Cylinder.\
The effect that creates a Cylinder specifies the ra-\
dius of the Cylinder’s base and the Cylinder’s height.\
A Cylinder’s point of origin is included in the area\
of effect.\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
