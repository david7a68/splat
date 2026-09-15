---
splat_schema: 1
splat_id: 8d21c974-b51d-560a-8d82-c9d24c4dd0a8
splat_kind: rule
splat_key: dnd2024:rules/combat_mounted_combat
splat_name: Mounted Combat
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_mounted_combat
  parameters:
    mount_eligibility:
      willing: true
      size_categories_larger_minimum: 1
      anatomy_appropriate:
        authority: gm
        type: boolean
  implementation: dnd2024:rules_combat_mounted_combat
splat_contracts:
- id: dnd2024:rules_combat_mounted_combat
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events: []
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 15
  engine_gaps: &id001
  - dnd2024:rules_combat_mounted_combat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 0aa2b78bfae2a2f3ff22c786fd14740a5fededd16f4bc16388cf15c6db04f5d2
  parent_name: Combat
  line_span:
    first: 1059
    count: 4
splat_clauses:
- key: source_rules
  text: '# Mounted Combat A willing creature that is at least one size larger than a rider and that has
    an appropriate anatomy can serve as a mount, using the following rules.'
  sha256: e12a908ef75aaf878d17422c1c8f087a3fea902cd062fe09d9c05e1328e022c1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mounted Combat

<!-- SRD page 15 -->

A willing creature that is at least one size larger\
than a rider and that has an appropriate anatomy\
can serve as a mount, using the following rules.\
