---
splat_schema: 1
splat_id: 121b2a25-1d9a-5f2b-bb67-8db7c489c366
splat_kind: item
splat_key: dnd2024:equipment/splint_armor
splat_name: Splint Armor
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
    base_ac: 17
    add_dexterity_modifier: false
    strength_minimum: 15
  price:
    value: 200
    unit: gp
  mass:
    value: 60.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/splint_armor
  implementation: dnd2024:equipment_splint_armor
  parameters:
    armor:
      category: heavy armor
      stealth_disadvantage: true
      base_ac: 17
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
- id: dnd2024:equipment_splint_armor
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
  - dnd2024:equipment_splint_armor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 7d846109c5a696861a497e237899763d1b761bcfc701140fada71586028c7b16
splat_clauses:
- key: source_rules
  text: '# Splint Armor Source table, SRD 5.2.1 page 92. { "armor": { "category": "heavy armor", "stealth_disadvantage":
    true, "base_ac": 17, "add_dexterity_modifier": false, "strength_minimum": 15 }, "price": { "value":
    200, "unit": "gp" }, "mass": { "value": 60.0, "unit": "lb" }, "category": "armor" }'
  sha256: f6e82d2414e3b4021b0580a0cd562d27503f0460b509d3e3c02f044457ba889c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Splint Armor

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "heavy armor",
    "stealth_disadvantage": true,
    "base_ac": 17,
    "add_dexterity_modifier": false,
    "strength_minimum": 15
  },
  "price": {
    "value": 200,
    "unit": "gp"
  },
  "mass": {
    "value": 60.0,
    "unit": "lb"
  },
  "category": "armor"
}
