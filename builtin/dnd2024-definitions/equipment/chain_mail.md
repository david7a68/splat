---
splat_schema: 1
splat_id: dc25b6ca-51bc-5670-9d90-7e3ee3861b9a
splat_kind: item
splat_key: dnd2024:equipment/chain_mail
splat_name: Chain Mail
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: heavy armor
    stealth_disadvantage: true
    base_ac: 16
    add_dexterity_modifier: false
    strength_minimum: 13
  price:
    value: 75
    unit: gp
  mass:
    value: 55.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/chain_mail
  implementation: dnd2024:equipment_chain_mail
  parameters:
    armor:
      category: heavy armor
      stealth_disadvantage: true
      base_ac: 16
      add_dexterity_modifier: false
      strength_minimum: 13
    while_equipped: true
    maximum_simultaneous_same_category: 1
    equip:
      don:
        value: 10
        unit: minute
      doff:
        value: 5
        unit: minute
    training:
      without_training:
        d20_test_disadvantage_abilities:
        - strength
        - dexterity
        can_cast_spells: false
    stacking_group: body_armor
    stealth:
      ability: dexterity
      skill: stealth
      mode: disadvantage
    strength_penalty:
      if_strength_below: 13
      speed_reduction_ft: 10
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_chain_mail
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
  - dnd2024:equipment_chain_mail
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 67a84290db7bed86381877622e37dee7ac5709e3341dc877d9f341c3935b6237
splat_clauses:
- key: source_rules
  text: '# Chain Mail Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 16, "add_dexterity_modifier": false, "strength_minimum": 13 }, "price": { "value":
    75, "unit": "gp" }, "mass": { "value": 55.0, "unit": "lb" }, "category": "armor" }'
  sha256: ca28eb5a671748fe5701b2fd9ffaf9799c7087c226cbca4c714be804f85808c6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chain Mail

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 16,
    "add_dexterity_modifier": false,
    "strength_minimum": 13
  },
  "price": {
    "value": 75,
    "unit": "gp"
  },
  "mass": {
    "value": 55.0,
    "unit": "lb"
  },
  "category": "armor"
}
