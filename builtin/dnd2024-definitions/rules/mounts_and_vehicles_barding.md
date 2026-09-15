---
splat_schema: 1
splat_id: 9a362647-6bcd-5972-9f9c-12874f1ae760
splat_kind: rule
splat_key: dnd2024:rules/mounts_and_vehicles_barding
splat_name: Barding
splat_origin:
  key: dnd2024:reference/pages/100
splat_sources:
- title: SRD 5.2.1
  locator: pp. 100
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/mounts_and_vehicles_barding
  parameters:
    barding:
      armor_options: all_armor_table_armor
      cost_multiplier: 4
      weight_multiplier: 2
  implementation: dnd2024:rules_mounts_and_vehicles_barding
splat_contracts:
- id: dnd2024:rules_mounts_and_vehicles_barding
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - item_attributes
  - actor_modifiers
  events: []
  capabilities:
  - equipment
  - armor_class
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 100
  engine_gaps: &id001
  - dnd2024:rules_mounts_and_vehicles_barding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: da241686462cacfc25ff0da675db6e636648392e63dd6daf911ff67c72db3ba3
  parent_name: Mounts and Vehicles
  line_span:
    first: 8696
    count: 5
splat_clauses:
- key: source_rules
  text: '# Barding Barding is armor designed for a mount. Any type of armor on the Armor table in “Equipment”
    can be purchased as barding. The cost is four times the normal cost, and it weighs twice as much.'
  sha256: 6ce4ab9e1fd3e1e0b66700a33ae845ab518b7a9e8a592a70b2db72ea8419789b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Barding

<!-- SRD page 100 -->

Barding is armor designed for a mount. Any type\
of armor on the Armor table in “Equipment” can\
be purchased as barding. The cost is four times the\
normal cost, and it weighs twice as much.\
