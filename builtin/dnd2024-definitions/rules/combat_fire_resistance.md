---
splat_schema: 1
splat_id: 587f2495-bf73-54a8-aec7-8bfc62010237
splat_kind: rule
splat_key: dnd2024:rules/combat_fire_resistance
splat_name: Fire Resistance
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_fire_resistance
  parameters:
    predicate: underwater
    damage_resistance:
    - fire
    applies_to:
    - creatures
    - objects
  implementation: dnd2024:rules_combat_fire_resistance
splat_contracts:
- id: dnd2024:rules_combat_fire_resistance
  inputs:
    target:
      type: participant
      required: true
    damage:
      required: false
      type: resolution
    event:
      type: event
      required: true
  outputs:
  - hit_point_updates
  - condition_updates
  - death_save_updates
  events: []
  capabilities:
  - damage
  - healing
  - death_saves
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rules_combat_fire_resistance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: f4ddd245d5a968046d61061122f7320daf827e5cce8c1414688b5ebb596a989a
  parent_name: Combat
  line_span:
    first: 1101
    count: 3
splat_clauses:
- key: source_rules
  text: '# Fire Resistance Anything underwater has Resistance to Fire damage (explained in “Damage and
    Healing”).'
  sha256: c6684868d619a71e201a5d0607b8dbd51e8b5865ad4dccb17f2602da4faee158
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fire Resistance

<!-- SRD page 16 -->

Anything underwater has Resistance to Fire dam-\
age (explained in “Damage and Healing”).\
