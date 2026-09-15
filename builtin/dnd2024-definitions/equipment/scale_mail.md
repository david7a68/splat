---
splat_schema: 1
splat_id: 2138f9fc-5f7f-519d-a700-953e8a676046
splat_kind: item
splat_key: dnd2024:equipment/scale_mail
splat_name: Scale Mail
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: medium armor
    stealth_disadvantage: true
    base_ac: 14
    add_dexterity_modifier: true
    dexterity_modifier_max: 2
  price:
    value: 50
    unit: gp
  mass:
    value: 45.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/scale_mail
  implementation: dnd2024:equipment_scale_mail
  parameters:
    armor:
      category: medium armor
      stealth_disadvantage: true
      base_ac: 14
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
    stealth:
      ability: dexterity
      skill: stealth
      mode: disadvantage
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_scale_mail
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
  - dnd2024:equipment_scale_mail
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2dfdfcd76e6cb0196105401f1bebf9c318cb754b0d964dd963498a60f54815df
splat_clauses:
- key: source_rules
  text: '# Scale Mail Source table, SRD 5.2.1 page 92. { "armor": { "category": "medium armor", "stealth_disadvantage":
    true, "base_ac": 14, "add_dexterity_modifier": true, "dexterity_modifier_max": 2 }, "price": { "value":
    50, "unit": "gp" }, "mass": { "value": 45.0, "unit": "lb" }, "category": "armor" }'
  sha256: 0bfcde1e7975efdd69564c0ffdbeac867d31f1bcc2626839a66c208e0f70089c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scale Mail

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "medium armor",
    "stealth_disadvantage": true,
    "base_ac": 14,
    "add_dexterity_modifier": true,
    "dexterity_modifier_max": 2
  },
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 45.0,
    "unit": "lb"
  },
  "category": "armor"
}
