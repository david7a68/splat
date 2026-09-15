---
splat_schema: 1
splat_id: c540d98d-1960-508d-997f-581ab48f8d57
splat_kind: item
splat_key: dnd2024:equipment/chain_shirt
splat_name: Chain Shirt
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: medium armor
    stealth_disadvantage: false
    base_ac: 13
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 50
    unit: gp
  mass:
    value: 20.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/chain_shirt
  implementation: dnd2024:equipment_chain_shirt
  parameters:
    armor:
      category: medium armor
      stealth_disadvantage: false
      base_ac: 13
      add_dexterity_modifier: true
      dexterity_modifier_max: 2
    while_equipped: true
    maximum_simultaneous_same_category: 1
    equip:
      don:
        value: 5
        unit: minute
      doff:
        value: 1
        unit: minute
    training:
      without_training:
        d20_test_disadvantage_abilities:
        - strength
        - dexterity
        can_cast_spells: false
    stacking_group: body_armor
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_chain_shirt
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    event:
      type: event
      required: true
  outputs:
  - armor_class
  - equipment_constraints
  - roll_modifiers
  - speed_modifiers
  - spellcasting_eligibility
  events:
  - item.equipped
  - item.unequipped
  - ability_check.pending
  - spell.cast_pending
  capabilities:
  - equipment
  - armor_class
  - roll_modifiers
  - speed_modifiers
  - spellcasting_eligibility
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 92
  engine_gaps: &id001
  - dnd2024:equipment_chain_shirt
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f6ef26128914b46de83cfe1417c0f9ab2ce30d47a39390d77233673a949f09e2
splat_clauses:
- key: source_rules
  text: '# Chain Shirt Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor", "stealth_disadvantage":
    false, "base_ac": 13, "add_dexterity_modifier": true, "dexterity_modifier_max": 2 }, "price": { "value":
    50, "unit": "gp" }, "mass": { "value": 20.0, "unit": "lb" }, "category": "armor" }'
  sha256: e370f1b43d725825f9c6b719b99fcf7dc5b02d48a4d0055b29880a0def72e074
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chain Shirt

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": false,
    "base_ac": 13,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 20.0,
    "unit": "lb"
  },
  "category": "armor"
}
