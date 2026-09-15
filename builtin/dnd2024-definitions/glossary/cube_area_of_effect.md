---
splat_schema: 1
splat_id: d5841c57-96b9-5db8-a39b-317142aa3f25
splat_kind: rule
splat_key: dnd2024:glossary/cube_area_of_effect
splat_name: Cube [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/cube_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/cube_area_of_effect
  parameters:
    shape: cube
    propagation: straight_lines
    base_area_rule:
      key: dnd2024:glossary/area_of_effect
    origin: anywhere_on_face
    side_length: effect_specified
    origin_included:
      chooser: creator
      options:
      - false
      - true
      default: false
    geometry_provider: external_geometry
  implementation: dnd2024:glossary_cube_area_of_effect
splat_contracts:
- id: dnd2024:glossary_cube_area_of_effect
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
  source_sha256: bdd3f93cb93560a255268fb977ccf5cf906b6192de73f4c67dabcc50dabaa41b
  source_path: builtin/srd-5.2.1/glossary/cube_area_of_effect.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_cube_area_of_effect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 4283beb63fec33ebd06e96ea6a1c0a04fcbebbcfc9db6bd66c4599f77c06dc62
splat_clauses:
- key: source_rules
  text: '# Cube [Area of Effect] A Cube is an area of effect that extends in straight lines from a point
    of origin located anywhere on a face of the Cube. The effect that creates a Cube specifies its size,
    which is the length of each side. A Cube’s point of origin isn’t included in the area of effect unless
    its creator decides otherwise.'
  sha256: a6a0c34bf71983f03e75ffbcdf9d113ae694aa77d22367545a29c2193685a0d5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cube [Area of Effect]

<!-- SRD page 179 -->

A Cube is an area of effect that extends in straight\
lines from a point of origin located anywhere on a\
face of the Cube. The effect that creates a Cube spec-\
ifies its size, which is the length of each side.\
A Cube’s point of origin isn’t included in the area\
of effect unless its creator decides otherwise.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
