---
splat_schema: 1
splat_id: e4478f08-398a-5c14-a125-dfe9cac2c924
splat_kind: rule
splat_key: dnd2024:glossary/size
splat_name: Size
splat_origin:
  key: dnd2024:reference/glossary/size
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/size
  parameters:
    ordered_categories:
    - tiny
    - small
    - medium
    - large
    - huge
    - gargantuan
    applies_to:
    - creature
    - object
    creature_mechanical_use: combat_occupied_space
    object_mechanical_use: hit_points
  implementation: dnd2024:glossary_size
splat_contracts:
- id: dnd2024:glossary_size
  inputs:
    subject:
      type: object
      required: true
    size:
      type: choice
      required: true
  outputs:
  - size_category
  events: []
  capabilities:
  - statistics
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: b45d9714be99cba2edd0342cd86a2895007c06dee3edf92d5c7740987f079a4b
  source_path: builtin/srd-5.2.1/glossary/size.md
  pages:
  - 188
  engine_gaps: &id001
  - dnd2024:glossary_size
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 18b35f95f1a1a59ab88e85772be826a81bf15a445405512fd7827f57c3d98537
splat_clauses:
- key: source_rules
  text: '# Size A creature or an object belongs to a size category: Tiny, Small, Medium, Large, Huge,
    or Gargantuan. A creature’s size determines how much space the creature occupies in combat. An object’s
    size affects its Hit Points. See also “Breaking Objects” and “Playing the Game” (“Combat”).'
  sha256: 4d49cd1aab6e894b3c076971d3e2ca3f59b2b8a2c744c8fcd9e6d640fc768b0e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Size

<!-- SRD page 188 -->

A creature or an object belongs to a size category:\
Tiny, Small, Medium, Large, Huge, or Gargantuan.\
A creature’s size determines how much space the\
creature occupies in combat. An object’s size affects\
its Hit Points. See also “Breaking Objects” and “Play-\
ing the Game” (“Combat”).\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].
