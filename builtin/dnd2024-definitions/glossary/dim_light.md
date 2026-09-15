---
splat_schema: 1
splat_id: c233e9a2-7b91-561a-86b4-ff861efa34d7
splat_kind: rule
splat_key: dnd2024:glossary/dim_light
splat_name: Dim Light
splat_origin:
  key: dnd2024:reference/glossary/dim_light
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/dim_light
  parameters:
    illumination: dim
    obscurity: lightly_obscured
  implementation: dnd2024:glossary_dim_light
splat_contracts:
- id: dnd2024:glossary_dim_light
  inputs:
    location:
      type: location
      required: true
  outputs:
  - obscurity
  events: []
  capabilities:
  - visibility
splat_conversion:
  status: specified
  category: glossary
  source_sha256: d8281a0b0a4dc94cfbb111989b9968a2eead43f05288568a3927965aca8d27fc
  source_path: builtin/srd-5.2.1/glossary/dim_light.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_dim_light
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: b21e3ff61d0153a447876693aebcf7b9239f973ecb4a45cdc57c9267c8b2b59f
splat_clauses:
- key: source_rules
  text: '# Dim Light An area with Dim Light is Lightly Obscured. See also “Lightly Obscured” and “Playing
    the Game” (“Exploration”).'
  sha256: 9b62288d66f143778ce6d5ceaccce7934950f5d27347275af194ddfe5f9dfddc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dim Light

<!-- SRD page 181 -->

An area with Dim Light is Lightly Obscured. See\
also “Lightly Obscured” and “Playing the Game”\
(“Exploration”).\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
