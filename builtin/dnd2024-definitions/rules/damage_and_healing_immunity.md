---
splat_schema: 1
splat_id: f270af4b-014c-5e5d-8d68-6fdbb2da0c58
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_immunity
splat_name: Immunity
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_immunity
  parameters:
    damage_immunity:
      damage: 0
    condition_immunity:
      apply_condition: false
  implementation: dnd2024:rules_damage_and_healing_immunity
splat_contracts:
- id: dnd2024:rules_damage_and_healing_immunity
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
  - 17
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_immunity
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: f329c81bf54bea01b70dbde859f1e475b0d5db2d1afb31355301f03131d4a1d6
  parent_name: Damage and Healing
  line_span:
    first: 1198
    count: 6
splat_clauses:
- key: source_rules
  text: '# Immunity Some creatures and objects have Immunity to certain damage types and conditions. Immunity
    to a damage type means you don’t take damage of that type, and Immunity to a condition means you aren’t
    affected by it.'
  sha256: ee0ced87e858086f59ce0dda8d0447c80cc516d9991a1da6d0bfd534b340b9c0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Immunity

<!-- SRD page 17 -->

Some creatures and objects have Immunity to cer-\
tain damage types and conditions. Immunity to a\
damage type means you don’t take damage of that\
type, and Immunity to a condition means you aren’t\
affected by it.\
