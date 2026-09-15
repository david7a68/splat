---
splat_schema: 1
splat_id: f8d05a63-ed18-577b-858e-18cd212df9df
splat_kind: rule
splat_key: dnd2024:glossary/cone_area_of_effect
splat_name: Cone [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/cone_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/cone_area_of_effect
  parameters:
    shape: cone
    propagation: straight_lines
    base_area_rule:
      key: dnd2024:glossary/area_of_effect
    origin: point
    direction:
      chooser: creator
    width_at_distance:
      var: distance_from_origin
    length: effect_specified
    origin_included:
      chooser: creator
      options:
      - false
      - true
      default: false
    geometry_provider: external_geometry
  implementation: dnd2024:glossary_cone_area_of_effect
splat_contracts:
- id: dnd2024:glossary_cone_area_of_effect
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
  source_sha256: 33ef0e9443efd6408fd996a1b3ccfdc87d77caa04a09fb261bc5db26e108f6f5
  source_path: builtin/srd-5.2.1/glossary/cone_area_of_effect.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_cone_area_of_effect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 3a0b6150e778dfe3737eb0dc66568b18bab004d367d2739c05b005d98b3a9641
splat_clauses:
- key: source_rules
  text: '# Cone [Area of Effect] A Cone is an area of effect that extends in straight lines from a point
    of origin in a direction its creator chooses. A Cone’s width at any point along its length is equal
    to that point’s distance from the point of origin. For example, a Cone is 15 feet wide at a point
    along its length that is 15 feet from the point of origin. The effect that creates a Cone specifies
    its maximum length. A Cone’s point of origin isn’t included in the area of effect unless its creator
    decides otherwise.'
  sha256: e023f78a2dbbc6ab4a8718e068e9385146a0473275e3942cb9bd151461289d38
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cone [Area of Effect]

<!-- SRD page 179 -->

A Cone is an area of effect that extends in straight\
lines from a point of origin in a direction its creator\
chooses. A Cone’s width at any point along its length\
is equal to that point’s distance from the point of\
origin. For example, a Cone is 15 feet wide at a point\
along its length that is 15 feet from the point of\
origin. The effect that creates a Cone specifies its\
maximum length.\
A Cone’s point of origin isn’t included in the area\
of effect unless its creator decides otherwise.\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
