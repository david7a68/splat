---
splat_schema: 1
splat_id: b228ce26-626e-5cf9-a96d-9834db85a2bb
splat_kind: item
splat_key: dnd2024:equipment/ring_mail
splat_name: Ring Mail
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
    base_ac: 14
    add_dexterity_modifier: false
  price:
    value: 30
    unit: gp
  mass:
    value: 40.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/ring_mail
  implementation: dnd2024:equipment_ring_mail
  parameters:
    armor:
      category: heavy armor
      stealth_disadvantage: true
      base_ac: 14
      add_dexterity_modifier: false
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
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_ring_mail
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
  - dnd2024:equipment_ring_mail
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 21c1a6c515c624c45e0d1cbd7d86378328866d2f4320cabfd72c554127ec6915
splat_clauses:
- key: source_rules
  text: '# Ring Mail Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 14, "add_dexterity_modifier": false }, "price": { "value": 30, "unit": "gp" }, "mass":
    { "value": 40.0, "unit": "lb" }, "category": "armor" }'
  sha256: 7a9e04a57301c5a10d147ec3880a9769ee088bd677ddb307a07dc0ebc756329f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ring Mail

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 14,
    "add_dexterity_modifier": false
  },
  "price": {
    "value": 30,
    "unit": "gp"
  },
  "mass": {
    "value": 40.0,
    "unit": "lb"
  },
  "category": "armor"
}
