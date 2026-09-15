---
splat_schema: 1
splat_id: 866cfefa-c2f4-5bb6-bfca-50429ed4d20f
splat_kind: item
splat_key: dnd2024:equipment/plate_armor
splat_name: Plate Armor
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
    base_ac: 18
    add_dexterity_modifier: false
    strength_minimum: 15
  price:
    value: 1500
    unit: gp
  mass:
    value: 65.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/plate_armor
  implementation: dnd2024:equipment_plate_armor
  parameters:
    armor:
      category: heavy armor
      stealth_disadvantage: true
      base_ac: 18
      add_dexterity_modifier: false
      strength_minimum: 15
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
      if_strength_below: 15
      speed_reduction_ft: 10
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_plate_armor
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
  - dnd2024:equipment_plate_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: b5daf87f0d3034bc14692fb4b2c232537d9fcbcea46b24d37cf8d5fd662f1667
splat_clauses:
- key: source_rules
  text: '# Plate Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 18, "add_dexterity_modifier": false, "strength_minimum": 15 }, "price": { "value":
    1500, "unit": "gp" }, "mass": { "value": 65.0, "unit": "lb" }, "category": "armor" }'
  sha256: 4c5cfc4c7b53a791eba8859f292f1103df84a8061c446ff874f28fb45f143a25
  status: contract_specified
splat_requires_mechanics: *id001
---

# Plate Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 18,
    "add_dexterity_modifier": false,
    "strength_minimum": 15
  },
  "price": {
    "value": 1500,
    "unit": "gp"
  },
  "mass": {
    "value": 65.0,
    "unit": "lb"
  },
  "category": "armor"
}
