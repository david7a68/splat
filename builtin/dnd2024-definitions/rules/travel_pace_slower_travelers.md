---
splat_schema: 1
splat_id: 423fdb62-5c87-56e0-82c1-77de312beed8
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_slower_travelers
splat_name: Slower Travelers
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/travel_pace_slower_travelers
  parameters:
    group_pace: slow
    predicate:
      any_member_speed_ratio_lte: 0.5
  implementation: dnd2024:rules_travel_pace_slower_travelers
splat_contracts:
- id: dnd2024:rules_travel_pace_slower_travelers
  inputs:
    travelers:
      type: participant_list
      required: true
    vehicle:
      required: false
      type: object
    journey:
      type: resolution
      required: true
  outputs:
  - travel_rate
  - travel_limits
  events: []
  capabilities:
  - travel
  - vehicles
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 192
  engine_gaps: &id001
  - dnd2024:rules_travel_pace_slower_travelers
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 99e17e06433df218d4f6be7f5d70c22bfbae99e195bdf876a45155935862ea56
  parent_name: Travel Pace
  line_span:
    first: 9338
    count: 3
splat_clauses:
- key: source_rules
  text: '# Slower Travelers The group must move at a Slow pace if any group member’s Speed is reduced
    to half or less of normal.'
  sha256: 7bd46d9ea998263642c55d8615f2c0445f66326a9fbf76824ea64770ecd2c6e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Slower Travelers

<!-- SRD page 192 -->

The group must move at a Slow pace if any group\
member’s Speed is reduced to half or less of normal.\
