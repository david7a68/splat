---
splat_schema: 1
splat_id: 0f27f279-f32f-5eea-8c1d-292d9f6b9101
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_damage_threshold
splat_name: Damage Threshold
splat_origin:
  key: dnd2024:reference/pages/101
splat_sources:
- title: SRD 5.2.1
  locator: pp. 101
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_damage_threshold
  parameters:
    threshold_source: vehicle_table
    threshold_rule:
      key: dnd2024:glossary/damage_threshold
    vehicle_thresholds:
      airship: null
      galley: 20
      keelboat: 10
      longship: 15
      rowboat: null
      sailing_ship: 15
      warship: 20
  implementation: dnd2024:rules_mounts_and_vehicles_damage_threshold
splat_contracts:
- id: dnd2024:rules_mounts_and_vehicles_damage_threshold
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
  - 101
  engine_gaps: &id001
  - dnd2024:rules_mounts_and_vehicles_damage_threshold
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 6b1dfcc50def2e2ef4480c4d8dd5ace37f6433637d313dbe72ba95b81d469a0b
  parent_name: Mounts and Vehicles
  line_span:
    first: 8768
    count: 3
splat_clauses:
- key: source_rules
  text: '# Damage Threshold If a vehicle has a damage threshold (see “Rules Glossary”), it’s noted in
    the table.'
  sha256: 0e51c3267756ba14bb1aecf391ec75cac32fd22e0b8ca110233413fce67469fa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage Threshold

<!-- SRD page 101 -->

If a vehicle has a damage threshold (see “Rules Glos-\
sary”), it’s noted in the table.\
