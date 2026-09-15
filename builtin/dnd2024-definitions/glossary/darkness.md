---
splat_schema: 1
splat_id: eee7a472-7354-54cb-bc75-ceb2f42462e8
splat_kind: rule
splat_key: dnd2024:glossary/darkness
splat_name: Darkness
splat_origin:
  key: dnd2024:reference/glossary/darkness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/darkness
  parameters:
    illumination: darkness
    obscurity: heavily_obscured
  implementation: dnd2024:glossary_darkness
splat_contracts:
- id: dnd2024:glossary_darkness
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
  source_sha256: cd5ecface4436e4d518160f5104e1b68332555b5fb6bacd3a0404b3de17e6dc3
  source_path: builtin/srd-5.2.1/glossary/darkness.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_darkness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 210586c6a51348c4d6ce7fd91502f99c8bc5eaf9f99dd1cca780a48b240461fb
splat_clauses:
- key: source_rules
  text: '# Darkness An area of Darkness is Heavily Obscured. See also “Heavily Obscured” and “Playing
    the Game” (“Exploration”).'
  sha256: d375eb96f7b046a33d8aab2dac9fb66e28f98c1f1ca3bc709e04d57c66292f51
  status: contract_specified
splat_requires_mechanics: *id001
---

# Darkness

<!-- SRD page 180 -->

An area of Darkness is Heavily Obscured. See\
also “Heavily Obscured” and “Playing the Game”\
(“Exploration”).\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
