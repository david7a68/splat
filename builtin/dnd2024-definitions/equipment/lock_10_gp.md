---
splat_schema: 1
splat_id: 0099cb14-73ac-57e3-bd61-9ef53031d680
splat_kind: item
splat_key: dnd2024:equipment/lock_10_gp
splat_name: Lock (10 GP)
splat_origin:
  key: dnd2024:reference/pages/098
splat_sources:
- title: SRD 5.2.1
  locator: pp. 98
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 10
    unit: gp
  mass:
    value: 1.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/lock_10_gp
  implementation: dnd2024:equipment_lock_10_gp
  parameters:
    includes_key: true
    pick:
      requires_item:
        key: dnd2024:equipment/other_tools_thieves_tools_25_gp
      check:
        ability: dexterity
        skill: sleight_of_hand
        dc: 15
      success: unlock
    key_use:
      matching_key_required: true
      operation: unlock
    mass:
      value: 1.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_lock_10_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - item_properties
  - conditional_modifiers
  events: []
  capabilities:
  - equipment
  - conditional_modifiers
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 98
  engine_gaps: &id001
  - dnd2024:equipment_lock_10_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 1749f9c1631713b6012534cf86cec74c8b9f5286e80beab223b9f81580fe17b9
  line_span:
    first: 8481
    count: 4
splat_clauses:
- key: source_rules
  text: '# Lock (10 GP) A Lock comes with a key. Without the key, a creature can use Thieves’ Tools to
    pick this Lock with a successful DC 15 Dexterity (Sleight of Hand) check.'
  sha256: 22734eb83ffc95e38f01ef7248cdd2ec36e36872a8ede75ce562345d7f3655cd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lock (10 GP)

<!-- SRD page 98 -->

A Lock comes with a key. Without the key, a crea-\
ture can use Thieves’ Tools to pick this Lock with a\
successful DC 15 Dexterity (Sleight of Hand) check.\
