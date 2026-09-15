---
splat_schema: 1
splat_id: ed23dedf-b0f0-541b-bead-65706afdde4c
splat_kind: rule
splat_key: dnd2024:rules/the_next_dawn
splat_name: “The Next Dawn”
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/the_next_dawn
  parameters:
    recharge_event: item_specified
    if_recharge_event_absent_on_world_or_plane:
      replacement_time_authority: gm
      replacement_time_input: duration
  implementation: dnd2024:rules_the_next_dawn
splat_contracts:
- id: dnd2024:rules_the_next_dawn
  inputs:
    item:
      type: item_instance
      required: true
    campaign:
      type: campaign
      required: true
    replacement_time:
      required: false
      type: duration
  outputs:
  - recharge_schedule
  events: []
  capabilities:
  - item_recharge
  - calendar
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rules_the_next_dawn
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 2868cf2bdb4109d5be6b2c18a924f01d0822687ad7d078f478f77c3dda2d54dd
  parent_name: “The Next Dawn”
  line_span:
    first: 10637
    count: 6
splat_clauses:
- key: source_rules
  text: '# “The Next Dawn” Magic items often have charges or properties that recharge at the next dawn
    or some other specified time. If such an item is on a world or plane of existence where the specified
    event doesn’t occur, the GM determines when the item recharges.'
  sha256: 87f30f43d36304850c24a6fb110d0a2194ae060d9c61010b905ce0e5094b3338
  status: contract_specified
splat_requires_mechanics: *id001
---

# “The Next Dawn”

<!-- SRD page 206 -->

Magic items often have charges or properties that\
recharge at the next dawn or some other specified\
time. If such an item is on a world or plane of exis-\
tence where the specified event doesn’t occur, the\
GM determines when the item recharges.\
