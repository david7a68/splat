---
splat_schema: 1
splat_id: d24c7037-1ffc-59fb-953e-7ba7ca684033
splat_kind: rule
splat_key: dnd2024:rules/starting_at_higher_levels
splat_name: Starting at Higher Levels
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/starting_at_higher_levels
  parameters:
    starting_level:
      authority: gm
      minimum: 1
    experienced_group_recommendation:
      level: 3
      binding: false
  implementation: dnd2024:rules_starting_at_higher_levels
splat_contracts:
- id: dnd2024:rules_starting_at_higher_levels
  inputs:
    campaign:
      type: campaign
      required: true
    level:
      type: integer
      required: true
  outputs:
  - starting_level
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 24
  engine_gaps: &id001
  - dnd2024:rules_starting_at_higher_levels
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 8ca17d9060aab008e800a118b7b821f02f75693adca3b0d89f08cd0117bed906
  parent_name: Starting at Higher Levels
  line_span:
    first: 1850
    count: 5
splat_clauses:
- key: source_rules
  text: '# Starting at Higher Levels Your GM might start your group’s characters at a level higher than
    1. It is particularly recommended to start at level 3 if your group is composed of seasoned D&D players.'
  sha256: 77e9f57133aa1431f4f32bb4ef422689d0d41eccfb072ca4661ca39da20ad7b5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Starting at Higher Levels

<!-- SRD page 24 -->

Your GM might start your group’s characters at a\
level higher than 1. It is particularly recommended\
to start at level 3 if your group is composed of sea-\
soned D&D players.\
