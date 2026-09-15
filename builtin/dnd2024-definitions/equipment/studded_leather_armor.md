---
splat_schema: 1
splat_id: 899853f5-26b9-5157-b5ae-f699c05369d0
splat_kind: item
splat_key: dnd2024:equipment/studded_leather_armor
splat_name: Studded Leather Armor
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: light armor
    stealth_disadvantage: false
    base_ac: 12
    add_dexterity_modifier: true
  price:
    value: 45
    unit: gp
  mass:
    value: 13.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/studded_leather_armor
  implementation: dnd2024:equipment_studded_leather_armor
  parameters:
    armor:
      category: light armor
      stealth_disadvantage: false
      base_ac: 12
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
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_studded_leather_armor
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
  - dnd2024:equipment_studded_leather_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 8fcd9243981dbc65405601e3d44bc4e185f2d2f0533f65f63b9970e7bd7ef757
splat_clauses:
- key: source_rules
  text: '# Studded Leather Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "light armor",
    "stealth_disadvantage": false, "base_ac": 12, "add_dexterity_modifier": true }, "price": { "value":
    45, "unit": "gp" }, "mass": { "value": 13.0, "unit": "lb" }, "category": "armor" }'
  sha256: 2f86707200ccfe7fc6d973261d1778d1f581796bbad4cce8ce0e1fddb9b6650e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Studded Leather Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "light armor",
    "stealth_disadvantage": false,
    "base_ac": 12,
    "add_dexterity_modifier": true
  },
  "price": {
    "value": 45,
    "unit": "gp"
  },
  "mass": {
    "value": 13.0,
    "unit": "lb"
  },
  "category": "armor"
}
