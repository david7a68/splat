---
splat_schema: 1
splat_id: 6e3e7154-a70b-5ce0-9a88-aa2049957177
splat_kind: item
splat_key: dnd2024:equipment/spear
splat_name: Spear
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: piercing
  properties_text: Thrown (Range 20/60), Versatile (1d8)
  mastery: sap
  weapon_category: simple melee
  price:
    value: 1
    unit: gp
  mass:
    value: 3.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/spear
  implementation: dnd2024:equipment_spear
  parameters:
    weapon_category: simple_melee
    damage:
      expression: 1d6
      type: piercing
      amount:
        dice:
          count: 1
          sides: 6
        bonus: 0
    properties:
    - id: thrown
      rule:
        key: dnd2024:rules/weapons_thrown
      range:
        normal_ft: 20
        long_ft: 60
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      draw_as_part_of_attack: true
      attack_type: ranged
      melee_weapon_uses_melee_ability: true
    - id: versatile
      rule:
        key: dnd2024:rules/weapons_versatile
      two_hand_melee_damage:
        expression: 1d8
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
- id: dnd2024:equipment_spear
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
  - dnd2024:equipment_spear
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: a4ec20a2699c41d9c2f445e61510ccdc8d1be47e3cb9182e92cdae8c70e342d4
splat_clauses:
- key: source_rules
  text: '# Spear Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Thrown (Range 20/60), Versatile (1d8)", "mastery": "sap", "weapon_category":
    "simple melee", "price": { "value": 1, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 70457594c25c74066a503f2caf419e08c2852fc88a42181e99b795d8298c1dab
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spear

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 20/60), Versatile (1d8)",
  "mastery": "sap",
  "weapon_category": "simple melee",
  "price": {
    "value": 1,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
