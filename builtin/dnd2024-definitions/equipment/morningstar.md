---
splat_schema: 1
splat_id: 4ad21685-ad9c-541d-aaa1-4272f5c22121
splat_kind: item
splat_key: dnd2024:equipment/morningstar
splat_name: Morningstar
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: piercing
  properties_text: —
  mastery: sap
  weapon_category: martial melee
  price:
    value: 15
    unit: gp
  mass:
    value: 4.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/morningstar
  implementation: dnd2024:equipment_morningstar
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d8
      type: piercing
      amount:
        dice:
          count: 1
          sides: 8
        bonus: 0
    properties: []
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: sap
      rule:
        key: dnd2024:rules/weapons_sap
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_creature
        effect:
          attack_roll_mode: disadvantage
          count: 1
          expiry: attacker_next_turn_start
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_morningstar
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    target:
      required: false
      type: participant
    event:
      type: event
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - attack_options
  - attack_modifiers
  - damage
  - mastery_effects
  - ammunition_updates
  events:
  - attack.pending
  - attack.hit
  - attack.miss
  - damage.resolved
  - combat.ended
  capabilities:
  - weapon_properties
  - weapon_mastery
  - attacks
  - effects
  - ammunition
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 91
  engine_gaps: &id001
  - dnd2024:equipment_morningstar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 48c2c33f2e74e608dbec866f2d2b05310bcea1ce7b00c938e7455df1f01b014c
splat_clauses:
- key: source_rules
  text: '# Morningstar Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "—", "mastery": "sap", "weapon_category": "martial melee", "price": { "value":
    15, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 8cc14139103dfb4ffec5a57280227670b428b76a989d424a26103751484be81e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Morningstar

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "—",
  "mastery": "sap",
  "weapon_category": "martial melee",
  "price": {
    "value": 15,
    "unit": "gp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}
