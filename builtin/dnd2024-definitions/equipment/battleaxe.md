---
splat_schema: 1
splat_id: e6b67cd5-4d3e-50b6-b069-1beb2d886330
splat_kind: item
splat_key: dnd2024:equipment/battleaxe
splat_name: Battleaxe
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: slashing
  properties_text: Versatile (1d10)
  mastery: topple
  weapon_category: martial melee
  price:
    value: 10
    unit: gp
  mass:
    value: 4.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/battleaxe
  implementation: dnd2024:equipment_battleaxe
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d8
      type: slashing
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
      property: topple
      rule:
        key: dnd2024:rules/weapons_topple
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_creature
        optional: true
        save:
          ability: constitution
          dc:
            op: add
            operands:
            - 8
            - stat: attack_ability_modifier
            - stat: proficiency_bonus
          failure:
            condition: prone
          success: no_effect
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_battleaxe
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
  - dnd2024:equipment_battleaxe
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: d581b18f8ab35592683aae3e20d5c3f6daacfafd3decca6f24e40ca49a5c6f6c
splat_clauses:
- key: source_rules
  text: '# Battleaxe Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "slashing"
    }, "properties_text": "Versatile (1d10)", "mastery": "topple", "weapon_category": "martial melee",
    "price": { "value": 10, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: d2f5bb63f32e1bcc327288a14f89fc7059f0eda9ecf676fbc1274c548f9e02b5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Battleaxe

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "slashing"
  },
  "properties_text": "Versatile (1d10)",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}
