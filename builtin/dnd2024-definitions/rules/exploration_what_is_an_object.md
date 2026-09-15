---
splat_schema: 1
splat_id: 6eb7670c-062c-5c8f-a699-68fd07a69273
splat_kind: rule
splat_key: dnd2024:rules/exploration_what_is_an_object
splat_name: What Is an Object?
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_what_is_an_object
  parameters:
    object:
      discrete: true
      inanimate: true
    composite_excluded:
    - building
    - vehicle
    composite_parts_may_be_objects: true
  implementation: dnd2024:rules_exploration_what_is_an_object
splat_contracts:
- id: dnd2024:rules_exploration_what_is_an_object
  inputs:
    candidate:
      type: object
      required: true
  outputs:
  - object_classification
  events: []
  capabilities:
  - objects
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 12
  engine_gaps: &id001
  - dnd2024:rules_exploration_what_is_an_object
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: f5dcea46447b2bf2a86ae8c7298091d05662509e6098d32a96b505058e16c398
  parent_name: Exploration
  line_span:
    first: 667
    count: 5
splat_clauses:
- key: source_rules
  text: '# What Is an Object? For the purpose of the rules, an object is a discrete, inanimate item like
    a window, door, sword, book, table, chair, or stone. It isn’t a building or a vehicle, which are composed
    of many objects.'
  sha256: 8eeca670ac166e34e59d2cab2dd0de1089c2cb24bb24a00d9f4cd2665f29477f
  status: contract_specified
splat_requires_mechanics: *id001
---

# What Is an Object?

<!-- SRD page 12 -->

For the purpose of the rules, an object is a discrete,\
inanimate item like a window, door, sword, book,\
table, chair, or stone. It isn’t a building or a vehicle,\
which are composed of many objects.\
