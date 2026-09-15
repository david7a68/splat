---
splat_schema: 1
splat_id: a3da0248-14fc-54fb-ba30-33a3e4e24d16
splat_kind: item
splat_key: dnd2024:equipment/war_pick
splat_name: War Pick
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
  properties_text: Versatile (1d10)
  mastery: sap
  weapon_category: martial melee
  price:
    value: 5
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/war_pick
  implementation: dnd2024:equipment_war_pick
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
    properties:
    - id: versatile
      rule:
        key: dnd2024:rules/weapons_versatile
      two_hand_melee_damage:
        expression: 1d10
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
- id: dnd2024:equipment_war_pick
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
  - dnd2024:equipment_war_pick
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c8d2685997a7311272ee41afad51f62e5d23dfde469b11fb55bae9ebd58376f7
splat_clauses:
- key: source_rules
  text: '# War Pick Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "Versatile (1d10)", "mastery": "sap", "weapon_category": "martial melee", "price":
    { "value": 5, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 9b36d1ea301dca84fb473b9b3065f3fa4922c03c86f46895aea224a72ba0f1a6
  status: contract_specified
splat_requires_mechanics: *id001
---

# War Pick

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Versatile (1d10)",
  "mastery": "sap",
  "weapon_category": "martial melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
