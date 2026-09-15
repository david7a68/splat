---
splat_schema: 1
splat_id: 889b763b-f4d3-54b4-9b29-a2e2af1dfa0a
splat_kind: item
splat_key: dnd2024:equipment/padded_armor
splat_name: Padded Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: light armor
    stealth_disadvantage: true
    base_ac: 11
    add_dexterity_modifier: true
  price:
    value: 5
    unit: gp
  mass:
    value: 8.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/padded_armor
  implementation: dnd2024:equipment_padded_armor
  parameters:
    armor:
      category: light armor
      stealth_disadvantage: true
      base_ac: 11
      add_dexterity_modifier: true
    while_equipped: true
    maximum_simultaneous_same_category: 1
    equip:
      don:
        value: 1
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
    stealth:
      ability: dexterity
      skill: stealth
      mode: disadvantage
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_padded_armor
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
  - dnd2024:equipment_padded_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f86fd1b0127ef5658a4292d406475b62f50dec6fa4b319d632ee0a171286cbb2
splat_clauses:
- key: source_rules
  text: '# Padded Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "light armor", "stealth_disadvantage":
    true, "base_ac": 11, "add_dexterity_modifier": true }, "price": { "value": 5, "unit": "gp" }, "mass":
    { "value": 8.0, "unit": "lb" }, "category": "armor" }'
  sha256: 3cd89e8420d0bc0d3dae85a34e3b03c0560b199d1af42e9b75a405f177a01ab0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Padded Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "light armor",
    "stealth_disadvantage": true,
    "base_ac": 11,
    "add_dexterity_modifier": true
  },
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 8.0,
    "unit": "lb"
  },
  "category": "armor"
}
