---
splat_schema: 1
splat_id: 3c73c0a9-979d-5f24-abb3-9d5522c14f78
splat_kind: item
splat_key: dnd2024:equipment/shield
splat_name: Shield
splat_origin:
  key: dnd2024:reference/pages/092
splat_sources:
- title: SRD 5.2.1
  locator: pp. 92
  url: https://www.dndbeyond.com/srd
splat_item:
  armor:
    category: shield
    stealth_disadvantage: false
    ac_bonus: 2
  price:
    value: 10
    unit: gp
  mass:
    value: 6.0
    unit: lb
  category: armor
splat_rule:
  mechanic: dnd2024:equipment/shield
  implementation: dnd2024:equipment_shield
  parameters:
    armor:
      category: shield
      stealth_disadvantage: false
      ac_bonus: 2
    while_equipped: true
    maximum_simultaneous_same_category: 1
    equip:
      don: utilize
      doff: utilize
      requires_wielding: true
    training:
      ac_bonus_requires_training: true
    stacking_group: shield
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_shield
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
  - dnd2024:equipment_shield
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 30ce3d67b1e000675035b94842051ca8b60929390bf2f0872b4b8327223fd953
splat_clauses:
- key: source_rules
  text: '# Shield Source table, SRD 5.2.1 page 92. { "armor": { "category": "shield", "stealth_disadvantage":
    false, "ac_bonus": 2 }, "price": { "value": 10, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb"
    }, "category": "armor" }'
  sha256: bc6077240d365f6389d54c5823fd6b894f419c2b2307ea7838b17f8ee64f460c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shield

Source table, SRD 5.2.1 page 92.

{
  "armor": {
    "category": "shield",
    "stealth_disadvantage": false,
    "ac_bonus": 2
  },
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "armor"
}
