---
splat_schema: 1
splat_id: 9c2f102d-04c3-55ac-a045-96c02796bf7e
splat_kind: rule
splat_key: dnd2024:rules/travel_pace_extended_travel
splat_name: Extended Travel
splat_origin:
  key: dnd2024:reference/pages/192
splat_sources:
- title: SRD 5.2.1
  locator: pp. 192
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/travel_pace_extended_travel
  parameters:
    safe_daily_travel_hours: 8
    trigger: each_additional_hour_end
    save:
      ability: constitution
      dc:
        op: add
        args:
        - 10
        - op: subtract
          args:
          - var: hours_traveled_today
          - 8
      failure:
        exhaustion_gain: 1
  implementation: dnd2024:rules_travel_pace_extended_travel
splat_contracts:
- id: dnd2024:rules_travel_pace_extended_travel
  inputs:
    subject:
      type: participant
      required: true
    hours_traveled_today:
      type: integer
      required: true
  outputs:
  - effects
  events:
  - travel.hour_finished
  capabilities:
  - travel
  - exhaustion
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 192
  engine_gaps: &id001
  - dnd2024:rules_travel_pace_extended_travel
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 4ece2e114502338d0b14c9e671cfa5e733bfc66a38fc5a216ab03fbeae4f670f
  parent_name: Travel Pace
  line_span:
    first: 9341
    count: 7
splat_clauses:
- key: source_rules
  text: '# Extended Travel Characters can push themselves to travel for more than 8 hours per day, at
    the risk of tiring. At the end of each additional hour of travel beyond 8 hours, each character must
    succeed on a Constitution saving throw or gain 1 Exhaustion level. The DC is 10 plus 1 for each hour
    past 8 hours.'
  sha256: 257cf2e33e52b19248769384703272ef375c7587cf76693de868ca4f4ade9d3d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Extended Travel

<!-- SRD page 192 -->

Characters can push themselves to travel for more\
than 8 hours per day, at the risk of tiring. At the end\
of each additional hour of travel beyond 8 hours,\
each character must succeed on a Constitution sav-\
ing throw or gain 1 Exhaustion level. The DC is 10\
plus 1 for each hour past 8 hours.\
