---
splat_schema: 1
splat_id: 89c02567-57a8-53ea-9147-973c4316f99c
splat_kind: rule
splat_key: dnd2024:glossary/heavily_obscured
splat_name: Heavily Obscured
splat_origin:
  key: dnd2024:reference/glossary/heavily_obscured
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/heavily_obscured
  parameters:
    when: trying_to_see_something_in_heavily_obscured_space
    effective_condition: blinded
  implementation: dnd2024:glossary_heavily_obscured
splat_contracts:
- id: dnd2024:glossary_heavily_obscured
  inputs:
    subject:
      type: participant
      required: true
    target_location:
      type: location
      required: true
  outputs:
  - vision_effect
  events: []
  capabilities:
  - visibility
  - conditions
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 254e07753a8af34a4335af8cc46ee5694fc288a440ac80e621a6a3cf3e0898e0
  source_path: builtin/srd-5.2.1/glossary/heavily_obscured.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_heavily_obscured
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 5e3f7bc1667657e124d48a35c7de81541f5422023d10d853fe80e3f9646b08cd
splat_clauses:
- key: source_rules
  text: '# Heavily Obscured You have the Blinded condition while trying to see something in a Heavily
    Obscured space. See also “Blinded,” “Darkness,” and “Playing the Game” (“Exploration”).'
  sha256: 185ea0d4f750001238c77c0b81dbd519fa774f779c76fc5f1cef48a7aee25e89
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heavily Obscured

<!-- SRD page 182 -->

You have the Blinded condition while trying to see\
something in a Heavily Obscured space. See also\
“Blinded,” “Darkness,” and “Playing the Game”\
(“Exploration”).\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
