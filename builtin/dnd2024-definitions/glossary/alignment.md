---
splat_schema: 1
splat_id: 6cbca415-8b1a-5a63-922e-5bd896d157bb
splat_kind: rule
splat_key: dnd2024:glossary/alignment
splat_name: Alignment
splat_origin:
  key: dnd2024:reference/glossary/alignment
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/alignment
  parameters:
    morality:
    - good
    - evil
    - neutral
    order:
    - lawful
    - chaotic
    - neutral
    combination: cartesian_product
    combination_count: 9
  implementation: dnd2024:glossary_alignment
splat_contracts:
- id: dnd2024:glossary_alignment
  inputs:
    subject:
      type: participant
      required: true
    morality:
      type: choice
      required: true
    order:
      type: choice
      required: true
  outputs:
  - alignment
  events: []
  capabilities:
  - statistics
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 17310cb1566f5c2d381f921ebc33035d5499994a7acddd534f23984f45495dc9
  source_path: builtin/srd-5.2.1/glossary/alignment.md
  pages:
  - 176
  engine_gaps: &id001
  - dnd2024:glossary_alignment
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 5161dd0ae62fa7e371b1e181b86f97c1634266368b2b7e3b49702e077b02f6f5
splat_clauses:
- key: source_rules
  text: '# Alignment A creature’s alignment broadly describes its ethical attitudes and ideals. Alignment
    is a combination of two factors: one identifies morality (good, evil, or neutral), and the other describes
    attitudes toward order (lawful, chaotic, or neutral). These factors allow for nine possible combinations,
    such as Lawful Good and Neutral Evil. See also “Character Creation” (“Create Your Character”).'
  sha256: 67748ef7f8bbc546224244e8be57c79a877eb44b6f741d3caad5770c3faaab9d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Alignment

<!-- SRD page 176 -->

A creature’s alignment broadly describes its ethical\
attitudes and ideals. Alignment is a combination of\
two factors: one identifies morality (good, evil, or\
neutral), and the other describes attitudes toward\
order (lawful, chaotic, or neutral). These factors al-\
low for nine possible combinations, such as Lawful\
Good and Neutral Evil. See also “Character Creation”\
(“Create Your Character”).\

Original layout: [[../../srd-5.2.1/pages/176|page 176]].
