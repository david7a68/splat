---
splat_schema: 1
splat_id: 6b0fd1dd-b906-5d1e-8c19-8ee5ac25b492
splat_kind: item
splat_key: dnd2024:equipment/javelin
splat_name: Javelin
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
  properties_text: Thrown (Range 30/120)
  mastery: slow
  weapon_category: simple melee
  price:
    value: 5
    unit: sp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/javelin
  implementation: dnd2024:equipment_javelin
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
        normal_ft: 30
        long_ft: 120
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      draw_as_part_of_attack: true
      attack_type: ranged
      melee_weapon_uses_melee_ability: true
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: slow
      rule:
        key: dnd2024:rules/weapons_slow
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_and_damage_creature
        optional: true
        speed_reduction_ft: 10
        expiry: attacker_next_turn_start
        maximum_stacked_reduction_ft: 10
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_javelin
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
  - dnd2024:equipment_javelin
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 2b600e9c69835217b872fee7a85934110d56fcd9f4e9565c880084ab5f79d9d6
splat_clauses:
- key: source_rules
  text: '# Javelin Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Thrown (Range 30/120)", "mastery": "slow", "weapon_category": "simple melee",
    "price": { "value": 5, "unit": "sp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: 1bac0ae6cc31c423fd15203095b1a485dc9425280c201dc7d741a5b8f6f998c6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Javelin

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 30/120)",
  "mastery": "slow",
  "weapon_category": "simple melee",
  "price": {
    "value": 5,
    "unit": "sp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
