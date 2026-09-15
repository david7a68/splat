---
splat_schema: 1
splat_id: 0e0749ba-6361-5510-8362-459873fbd571
splat_kind: rule
splat_key: dnd2024:glossary/reach
splat_name: Reach
splat_origin:
  key: dnd2024:reference/glossary/reach
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/reach
  parameters:
    default_feet: 5
    explicit_rule_override_allowed: true
  implementation: dnd2024:glossary_reach
splat_contracts:
- id: dnd2024:glossary_reach
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - reach_feet
  events: []
  capabilities:
  - attacks
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 3f0a0cc9c69ae93194ea7e2950273b6dde205d6d88548009e0146d0205477e52
  source_path: builtin/srd-5.2.1/glossary/reach.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_reach
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: da74040ba795dd166859f131505b5aaf20fefaf68faed6563111fb0c98d37800
splat_clauses:
- key: source_rules
  text: '# Reach A creature has a reach of 5 feet unless a rule says otherwise.'
  sha256: a8ae442e62e2f43ca31a23f378d949d75b055d3a409b51f5012641e774ddec72
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reach

<!-- SRD page 186 -->

A creature has a reach of 5 feet unless a rule says\
otherwise.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
