---
splat_schema: 1
splat_id: e8452e0e-fe4b-51b2-8c61-cf7fcfc7e4a7
splat_kind: rule
splat_key: dnd2024:glossary/swim_speed
splat_name: Swim Speed
splat_origin:
  key: dnd2024:reference/glossary/swim_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 189
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/swim_speed
  parameters:
    movement_mode: swim
    replaces_speed: true
    ignores_mode_extra_movement_cost: true
  implementation: dnd2024:glossary_swim_speed
splat_contracts:
- id: dnd2024:glossary_swim_speed
  inputs:
    subject:
      type: participant
      required: true
    distance_feet:
      type: number
      required: true
  outputs:
  - movement_cost
  events: []
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 12cc54d8080c9498f1443fb4b28bf7234e28dee47f81781797bc7bbb65ee5408
  source_path: builtin/srd-5.2.1/glossary/swim_speed.md
  pages:
  - 189
  engine_gaps: &id001
  - dnd2024:glossary_swim_speed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: e899bd805c59b322663e169b35ba0ef8c3a7ed5b85c132f21ab19f1ee444c686
splat_clauses:
- key: source_rules
  text: '# Swim Speed A Swim Speed can be used to swim without expending the extra movement normally associated
    with swimming. See also “Swimming” and “Speed.”'
  sha256: 7537c19377a27f3b69ab877fefca85f9856dcd1cb53b75125e6caa144df87cec
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swim Speed

<!-- SRD page 189 -->

A Swim Speed can be used to swim without expend-\
ing the extra movement normally associated with\
swimming. See also “Swimming” and “Speed.”\

Original layout: [[../../srd-5.2.1/pages/189|page 189]].
