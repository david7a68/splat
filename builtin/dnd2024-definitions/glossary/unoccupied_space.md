---
splat_schema: 1
splat_id: 4631b98c-b97b-5963-b59b-93504593601c
splat_kind: rule
splat_key: dnd2024:glossary/unoccupied_space
splat_name: Unoccupied Space
splat_origin:
  key: dnd2024:reference/glossary/unoccupied_space
splat_sources:
- title: SRD 5.2.1
  locator: pp. 191
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/unoccupied_space
  parameters:
    predicate:
      all:
      - no_creatures_in_space
      - space_not_completely_filled_by_objects
  implementation: dnd2024:glossary_unoccupied_space
splat_contracts:
- id: dnd2024:glossary_unoccupied_space
  inputs:
    location:
      type: location
      required: true
    occupancy:
      type: resolution
      required: true
  outputs:
  - unoccupied
  events: []
  capabilities:
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: b8fd00f0a37f8a8887ddbfa6caa3f71a82a55e2c823dd2fe47c9b6ad5cbe0d7d
  source_path: builtin/srd-5.2.1/glossary/unoccupied_space.md
  pages:
  - 191
  engine_gaps: &id001
  - dnd2024:glossary_unoccupied_space
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 123106f9e488acf062df57ee0256856463ae8506d0683ae2e603bad5656bfdd5
splat_clauses:
- key: source_rules
  text: '# Unoccupied Space A space is unoccupied if no creatures are in it and it isn’t completely filled
    by objects.'
  sha256: 454bcaa92bc3897ea5eb2712234fcb47b5beaf36b864c382b51dd96fea87c6f6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Unoccupied Space

<!-- SRD page 191 -->

A space is unoccupied if no creatures are in it and it\
isn’t completely filled by objects.\

Original layout: [[../../srd-5.2.1/pages/191|page 191]].
