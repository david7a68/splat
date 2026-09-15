---
splat_schema: 1
splat_id: ed577dd9-19e9-5948-81fc-a74e88e4e5f6
splat_kind: rule
splat_key: dnd2024:rules/combat_difficult_terrain
splat_name: Difficult Terrain
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_difficult_terrain
  parameters:
    movement_cost_extra_feet_per_foot: 1
    multiple_sources_stack: false
  implementation: dnd2024:rule_combat_difficult_terrain
splat_contracts:
- id: dnd2024:rule_combat_difficult_terrain
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - movement_budget
  events: []
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 14
  engine_gaps: &id001
  - dnd2024:rule_combat_difficult_terrain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 3804dda162ee952a8285d26d3b0ede2a715d30e3936684c81131085d9a5a9ea4
  parent_name: Combat
  line_span:
    first: 900
    count: 8
splat_clauses:
- key: source_rules
  text: '# Difficult Terrain Combatants are often slowed down by Difficult Terrain. Low furniture, rubble,
    undergrowth, steep stairs, snow, and shallow bogs are examples of Difficult Terrain. Every foot of
    movement in Difficult Terrain costs 1 extra foot, even if multiple things in a space count as Difficult
    Terrain.'
  sha256: 20f2ee670b2eea51b1adfc04d35817361a6681ae1b305618949902cbec97be30
  status: contract_specified
splat_requires_mechanics: *id001
---

# Difficult Terrain

<!-- SRD page 14 -->

Combatants are often slowed down by Difficult\
Terrain. Low furniture, rubble, undergrowth, steep\
stairs, snow, and shallow bogs are examples of Diffi-\
cult Terrain.\
Every foot of movement in Difficult Terrain costs\
1 extra foot, even if multiple things in a space count\
as Difficult Terrain.\
