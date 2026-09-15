---
splat_schema: 1
splat_id: 7f141cfc-6930-523f-bbbd-dd9f1b4c6bb0
splat_kind: rule
splat_key: dnd2024:rules/warlock_witch_sight
splat_name: Witch Sight
splat_origin:
  key: dnd2024:reference/pages/074
splat_sources:
- title: SRD 5.2.1
  locator: pp. 74
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_witch_sight
  parameters:
    prerequisites:
      warlock_level_minimum: 15
    sense:
      type: truesight
      range_feet: 30
  implementation: dnd2024:rules_warlock_witch_sight
splat_contracts:
- id: dnd2024:rules_warlock_witch_sight
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - effects
  - resource_updates
  events: []
  capabilities:
  - class_features
  - effects
  - resources
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 74
  engine_gaps: &id001
  - dnd2024:rules_warlock_witch_sight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 2a3e4f472d5424d9e95e66dae399aaf7853e57b4098a2e2e81a9f3da7bdf1d6c
  parent_name: Warlock
  line_span:
    first: 6368
    count: 3
splat_clauses:
- key: source_rules
  text: '# Witch Sight Prerequisite: Level 15+ Warlock You have Truesight with a range of 30 feet.'
  sha256: ab1386bbe3abfa92670fd98c8c6be4c27f827c35e79a9ba10a76595418f334e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Witch Sight

<!-- SRD page 74 -->

Prerequisite: Level 15+ Warlock\
You have Truesight with a range of 30 feet.\
