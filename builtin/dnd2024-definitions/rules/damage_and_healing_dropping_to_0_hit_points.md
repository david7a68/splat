---
splat_schema: 1
splat_id: 00c4d51c-6a19-583b-a094-926faacd71a7
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_dropping_to_0_hit_points
splat_name: Dropping to 0 Hit Points
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_dropping_to_0_hit_points
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/dead
    - key: dnd2024:glossary/unconscious_condition
    trigger:
      hit_points: 0
    outcomes:
    - instant_death
    - unconscious
  implementation: dnd2024:rule_damage_and_healing_dropping_to_0_hit_points
splat_contracts:
- id: dnd2024:rule_damage_and_healing_dropping_to_0_hit_points
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - hit_points
  - death
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 17
  engine_gaps: &id001
  - dnd2024:rule_damage_and_healing_dropping_to_0_hit_points
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: e6b0782ac46600b1ecb622b0e705dc2d8f0586dc3de4c86aea6c8e55e6d20d0f
  parent_name: Damage and Healing
  line_span:
    first: 1224
    count: 3
splat_clauses:
- key: source_rules
  text: '# Dropping to 0 Hit Points When a creature drops to 0 Hit Points, it either dies outright or
    falls unconscious, as explained below.'
  sha256: ee6fa40a774e6d516124088bbb4d6384fc6ddf1399bc92ee0991c02beee81dd3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dropping to 0 Hit Points

<!-- SRD page 17 -->

When a creature drops to 0 Hit Points, it either dies\
outright or falls unconscious, as explained below.\
