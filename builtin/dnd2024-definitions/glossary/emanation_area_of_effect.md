---
splat_schema: 1
splat_id: e710403e-84d2-569a-9a6f-b30c5277b55c
splat_kind: rule
splat_key: dnd2024:glossary/emanation_area_of_effect
splat_name: Emanation [Area of Effect]
splat_origin:
  key: dnd2024:reference/glossary/emanation_area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/emanation_area_of_effect
  parameters:
    shape: emanation
    propagation: straight_lines
    base_area_rule:
      key: dnd2024:glossary/area_of_effect
    origin_types:
    - creature
    - object
    direction: all
    distance: effect_specified
    moves_with_origin: true
    movement_exceptions:
    - instantaneous
    - stationary
    origin_included:
      chooser: creator
      options:
      - false
      - true
      default: false
    geometry_provider: external_geometry
  implementation: dnd2024:glossary_emanation_area_of_effect
splat_contracts:
- id: dnd2024:glossary_emanation_area_of_effect
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
  source_sha256: 07636b6c50b7d33ad78efbc74a2c8e5f32d6e456e743592a039ce509fdbf85ab
  source_path: builtin/srd-5.2.1/glossary/emanation_area_of_effect.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_emanation_area_of_effect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 0408dd484a8c8a18d22ad1a13fa3b80b9fabc7de3d1938229a6c4947b98008a5
splat_clauses:
- key: source_rules
  text: '# Emanation [Area of Effect] An Emanation is an area of effect that extends in straight lines
    from a creature or an object in all directions. The effect that creates an Emanation specifies the
    distance it extends. An Emanation moves with the creature or object that is its origin unless it is
    an instantaneous or a stationary effect. An Emanation’s origin (creature or object) isn’t included
    in the area of effect unless its creator decides otherwise.'
  sha256: 52125715d2c18d07885c5163b99654a2fe7ac1193564421ccf9d1d7d7b8f8379
  status: contract_specified
splat_requires_mechanics: *id001
---

# Emanation [Area of Effect]

<!-- SRD page 181 -->

An Emanation is an area of effect that extends in\
straight lines from a creature or an object in all di-\
rections. The effect that creates an Emanation spec-\
ifies the distance it extends.\
An Emanation moves with the creature or object\
that is its origin unless it is an instantaneous or a\
stationary effect.\
An Emanation’s origin (creature or object) isn’t\
included in the area of effect unless its creator de-\
cides otherwise.\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
