---
splat_schema: 1
splat_id: 0a93c632-352e-53e7-b22a-6de2601d69af
splat_kind: rule
splat_key: dnd2024:glossary/crawling
splat_name: Crawling
splat_origin:
  key: dnd2024:reference/glossary/crawling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/crawling
  parameters:
    movement_mode: crawl
    per_foot_cost:
      normal: 2
      difficult_terrain: 3
  implementation: dnd2024:glossary_crawling
splat_contracts:
- id: dnd2024:glossary_crawling
  inputs:
    subject:
      type: participant
      required: true
    distance_feet:
      type: number
      required: true
    difficult_terrain:
      type: boolean
      required: true
  outputs:
  - movement_cost
  events: []
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 8aa0543f248541247162aec770649923744014decc253df4ae23015cd744b712
  source_path: builtin/srd-5.2.1/glossary/crawling.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_crawling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 08cffdda2461d7ba2331c7cfb057fecabd7dae477928cb46ebdd603c3becfd89
splat_clauses:
- key: source_rules
  text: '# Crawling While you’re crawling, each foot of movement costs 1 extra foot (2 extra feet in Difficult
    Terrain). See also “Speed.”'
  sha256: 6ac7049db9a6cc1486ba5a4b4ade9b4509d9e5359486bf149688e376f59ac0e9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crawling

<!-- SRD page 179 -->

While you’re crawling, each foot of movement costs\
1 extra foot (2 extra feet in Difficult Terrain). See\
also “Speed.”\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
