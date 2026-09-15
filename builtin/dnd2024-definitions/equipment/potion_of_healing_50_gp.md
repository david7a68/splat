---
splat_schema: 1
splat_id: 2c7b0386-7ca1-5cdc-be8c-2728de2684a9
splat_kind: item
splat_key: dnd2024:equipment/potion_of_healing_50_gp
splat_name: Potion of Healing (50 GP)
splat_origin:
  key: dnd2024:reference/pages/099
splat_sources:
- title: SRD 5.2.1
  locator: pp. 99
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 50
    unit: gp
  mass:
    value: 0.5
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/potion_of_healing_50_gp
  implementation: dnd2024:equipment_potion_of_healing_50_gp
  parameters:
    magical: true
    use:
      economy: bonus_action
      consume: 1
      target: self_or_creature_within_5_ft
      operation: drink_or_administer
      healing:
        dice:
          count: 2
          sides: 4
        bonus: 2
    mass:
      value: 0.5
      unit: lb
splat_contracts:
- id: dnd2024:equipment_potion_of_healing_50_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    operation:
      required: false
      type: choice
    target:
      required: false
      type: participant
    target_object:
      required: false
      type: object
    location:
      required: false
      type: location
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - action_options
  - check_requests
  - item_effects
  - item_state
  events:
  - item.used
  capabilities:
  - item_actions
  - effects
  - ability_checks
  - consumables
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 99
  engine_gaps: &id001
  - dnd2024:equipment_potion_of_healing_50_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 5b00c8d66517812c4b0000cf7b6c81103c5ae92df959b2b12aa8b72f8e1882ca
  line_span:
    first: 8584
    count: 6
splat_clauses:
- key: source_rules
  text: '# Potion of Healing (50 GP) This potion is a magic item. As a Bonus Action, you can drink it
    or administer it to another creature within 5 feet of yourself. The creature that drinks the magical
    red fluid in this vial regains 2d4 + 2 Hit Points.'
  sha256: 002d64edbada6d728889c0c623efdf043201e30e5b7a364df237117ac0bc2e1a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Potion of Healing (50 GP)

<!-- SRD page 99 -->

This potion is a magic item. As a Bonus Action, you\
can drink it or administer it to another creature\
within 5 feet of yourself. The creature that drinks\
the magical red fluid in this vial regains 2d4 + 2 Hit\
Points.\
