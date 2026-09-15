---
splat_schema: 1
splat_id: 23793d47-d030-5aa6-8ced-c834314af839
splat_kind: item
splat_key: dnd2024:equipment/waterskin_2_sp
splat_name: Waterskin (2 SP)
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 2
    unit: sp
  mass:
    value: 5.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/waterskin_2_sp
  implementation: dnd2024:equipment_waterskin_2_sp
  parameters:
    capacity:
      liquid_pints: 4
    nutrition_rule:
      key: dnd2024:glossary/dehydration_hazard
    mass:
      value: 5.0
      unit: lb
      when: full
splat_contracts:
- id: dnd2024:equipment_waterskin_2_sp
  inputs:
    item:
      type: item_instance
      required: true
    contents:
      required: false
      type: item_list
  outputs:
  - capacity_constraints
  events: []
  capabilities:
  - containers
  - inventory
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 100
  engine_gaps: &id001
  - dnd2024:equipment_waterskin_2_sp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1089eb7e346b3b5f9d0269a5c9344182b6a448256fba9e8b501cba02d56e8ac0
  line_span:
    first: 8679
    count: 4
splat_clauses:
- key: source_rules
  text: '# Waterskin (2 SP) A Waterskin holds up to 4 pints. If you don’t drink sufficient water, you
    risk dehydration (see “Rules Glossary”).'
  sha256: 4df2ec2a54561936f573fecd9be4516d1d26fe2b5ea03ce9884c120cbeb44085
  status: contract_specified
splat_requires_mechanics: *id001
---

# Waterskin (2 SP)

<!-- SRD page 100 -->

A Waterskin holds up to 4 pints. If you don’t drink\
sufficient water, you risk dehydration (see “Rules\
Glossary”).\
