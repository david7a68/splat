---
splat_schema: 1
splat_id: 412dad3e-1aae-5ba0-8a41-89a807f38d4f
splat_kind: rule
splat_key: dnd2024:rules/warlock_devil_s_sight
splat_name: Devil’s Sight
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_devil_s_sight
  parameters:
    prerequisites:
      warlock_level_minimum: 2
    vision:
      range_feet: 120
      normal_in:
      - dim_light
      - darkness
      magical: true
      nonmagical: true
  implementation: dnd2024:rules_warlock_devil_s_sight
splat_contracts:
- id: dnd2024:rules_warlock_devil_s_sight
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
  - 72
  engine_gaps: &id001
  - dnd2024:rules_warlock_devil_s_sight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 7eb7f5b1f5a74e48c9bcd6f8629a9d62056d2236dfe382fc56e0d334ed815600
  parent_name: Warlock
  line_span:
    first: 6160
    count: 5
splat_clauses:
- key: source_rules
  text: '# Devil’s Sight Prerequisite: Level 2+ Warlock You can see normally in Dim Light and Darkness—
    both magical and nonmagical—within 120 feet of yourself.'
  sha256: 3a0dafee0f77f0ce7537c2ff380d4a477981ac0063557521a7464a4c39f03926
  status: contract_specified
splat_requires_mechanics: *id001
---

# Devil’s Sight

<!-- SRD page 72 -->

Prerequisite: Level 2+ Warlock\
You can see normally in Dim Light and Darkness—\
both magical and nonmagical—within 120 feet of\
yourself.\
