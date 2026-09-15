---
splat_schema: 1
splat_id: 2721dd7e-c898-5365-a241-a8822284546f
splat_kind: rule
splat_key: dnd2024:glossary/healing
splat_name: Healing
splat_origin:
  key: dnd2024:reference/glossary/healing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/healing
  parameters:
    hit_points_change:
      var: healing_amount
    base_bounds:
      key: dnd2024:glossary/hit_points
  implementation: dnd2024:glossary_healing
splat_contracts:
- id: dnd2024:glossary_healing
  inputs:
    subject:
      type: participant
      required: true
    healing_amount:
      type: integer
      required: true
  outputs:
  - hit_points_delta
  events:
  - healing.applied
  capabilities:
  - hit_points
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 9d3f1f6779f7e8ecc0651e3fa72e1c6b0f1cae0b43caee5528380d94c7e7817f
  source_path: builtin/srd-5.2.1/glossary/healing.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_healing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 06ae2b9079a31f3ce152a58b27d3c8d55b8f2a3657543f779e99b88e5fa6fcae
splat_clauses:
- key: source_rules
  text: '# Healing Healing is how you regain Hit Points. See also “Playing the Game” (“Damage and Healing”).'
  sha256: b7b3c75a63b8e5795c4544c71fdeb884bd953176e55225be2eeb15d3263f922e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Healing

<!-- SRD page 182 -->

Healing is how you regain Hit Points. See also “Play-\
ing the Game” (“Damage and Healing”).\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
