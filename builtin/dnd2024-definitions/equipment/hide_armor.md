---
splat_schema: 1
splat_id: 5470b619-0b2d-50b3-a625-e084a8bc64a7
splat_kind: item
splat_key: dnd2024:equipment/hide_armor
splat_name: Hide Armor
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
    base_ac: 12
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 10
    unit: gp
  mass:
    value: 12.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/hide_armor
  implementation: dnd2024:equipment_hide_armor
  parameters:
    armor:
      category: medium armor
      stealth_disadvantage: false
      base_ac: 12
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
- id: dnd2024:equipment_hide_armor
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
  - dnd2024:equipment_hide_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: e8c7e73c2767133471cf9c5fb7aee5c746bed729009b5b57b0b8ac177dd1b1e4
splat_clauses:
- key: source_rules
  text: '# Hide Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor", "stealth_disadvantage":
    false, "base_ac": 12, "add_dexterity_modifier": true, "dexterity_modifier_max": 2 }, "price": { "value":
    10, "unit": "gp" }, "mass": { "value": 12.0, "unit": "lb" }, "category": "armor" }'
  sha256: 5a15d02d0ca691b492165ee2154724fe7ba01c5b6a3933a38bc53234cf051fc2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hide Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": false,
    "base_ac": 12,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 12.0,
    "unit": "lb"
  },
  "category": "armor"
}
