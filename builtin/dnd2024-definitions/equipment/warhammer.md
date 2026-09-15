---
splat_schema: 1
splat_id: 37959a76-5c7b-5ef8-9952-5be893e32594
splat_kind: item
splat_key: dnd2024:equipment/warhammer
splat_name: Warhammer
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: bludgeoning
  properties_text: Versatile (1d10)
  mastery: push
  weapon_category: martial melee
  price:
    value: 15
    unit: gp
  mass:
    value: 5.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/warhammer
  implementation: dnd2024:equipment_warhammer
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d8
      type: bludgeoning
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
      property: push
      rule:
        key: dnd2024:rules/weapons_push
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_creature
        optional: true
        maximum_target_size: large
        push_maximum_ft: 10
        direction: straight_away_from_attacker
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_warhammer
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
  - dnd2024:equipment_warhammer
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 5788f7e3d1bf9665adf6a97e54a8a5aaa367bf16578a2f27151c26a1178ee1cb
splat_clauses:
- key: source_rules
  text: '# Warhammer Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "bludgeoning"
    }, "properties_text": "Versatile (1d10)", "mastery": "push", "weapon_category": "martial melee", "price":
    { "value": 15, "unit": "gp" }, "mass": { "value": 5.0, "unit": "lb" }, "category": "weapon" }'
  sha256: d53da4fa59a978d719ed389b0369d240efc2a79cdc2c3257d66ee21e33f04c73
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warhammer

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "bludgeoning"
  },
  "properties_text": "Versatile (1d10)",
  "mastery": "push",
  "weapon_category": "martial melee",
  "price": {
    "value": 15,
    "unit": "gp"
  },
  "mass": {
    "value": 5.0,
    "unit": "lb"
  },
  "category": "weapon"
}
