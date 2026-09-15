---
splat_schema: 1
splat_id: 8fc06ef1-d07e-5c18-8c82-719596dcad1c
splat_kind: rule
splat_key: dnd2024:glossary/friendly_attitude
splat_name: Friendly [Attitude]
splat_origin:
  key: dnd2024:reference/glossary/friendly_attitude
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/friendly_attitude
  parameters:
    attitude: friendly
    influence_ability_check_mode: advantage
  implementation: dnd2024:glossary_friendly_attitude
splat_contracts:
- id: dnd2024:glossary_friendly_attitude
  inputs:
    subject:
      type: participant
      required: true
    target:
      type: participant
      required: true
  outputs:
  - roll_mode
  events:
  - ability_check.pending
  capabilities:
  - relationships
  - checks
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a267c104364a1241b1f9c718f6bba11fc655fcb28cadbaf722f3846d3e174032
  source_path: builtin/srd-5.2.1/glossary/friendly_attitude.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_friendly_attitude
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: e50e2cb54c7c3e609c62c551a6c29832038ff67c7a08a56cadba7c0f46b40c6a
splat_clauses:
- key: source_rules
  text: '# Friendly [Attitude] A Friendly creature views you favorably. You have Advantage on an ability
    check to influence a Friendly creature. See also “Influence.”'
  sha256: ed9c5778a001033d40ab923ed39b2cf11c7be876356d8d5dd3174873c1960107
  status: contract_specified
splat_requires_mechanics: *id001
---

# Friendly [Attitude]

<!-- SRD page 182 -->

A Friendly creature views you favorably. You\
have Advantage on an ability check to influence a\
Friendly creature. See also “Influence.”\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
