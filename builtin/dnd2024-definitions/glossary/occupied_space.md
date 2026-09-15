---
splat_schema: 1
splat_id: 1db9676a-f1d1-57d3-a524-129ff6e037ab
splat_kind: rule
splat_key: dnd2024:glossary/occupied_space
splat_name: Occupied Space
splat_origin:
  key: dnd2024:reference/glossary/occupied_space
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/occupied_space
  parameters:
    predicate:
      any:
      - creature_in_space
      - space_completely_filled_by_objects
  implementation: dnd2024:glossary_occupied_space
splat_contracts:
- id: dnd2024:glossary_occupied_space
  inputs:
    location:
      type: location
      required: true
    occupancy:
      type: resolution
      required: true
  outputs:
  - occupied
  events: []
  capabilities:
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 97a84e446e9d5abcb59f3c5fbe4db3e9202f42e6699d0480fda4e54350b7f36a
  source_path: builtin/srd-5.2.1/glossary/occupied_space.md
  pages:
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_occupied_space
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 7b36881cea76a3f0659e82778276c7dd06be92bee2de49b354fc646cbb78d67d
splat_clauses:
- key: source_rules
  text: '# Occupied Space A space is occupied if a creature is in it or if it is completely filled by
    objects.'
  sha256: fe589f47a5e607dc5941d111bd97e84f7927a9bd82a84cf68a5e54f90584fc0a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Occupied Space

<!-- SRD page 185 -->

A space is occupied if a creature is in it or if it is\
completely filled by objects.\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].
