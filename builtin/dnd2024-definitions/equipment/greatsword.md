---
splat_schema: 1
splat_id: 690a9342-67db-5b7d-bc34-05af8c204771
splat_kind: item
splat_key: dnd2024:equipment/greatsword
splat_name: Greatsword
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 2d6
    type: slashing
  properties_text: Heavy, Two-Handed
  mastery: graze
  weapon_category: martial melee
  price:
    value: 50
    unit: gp
  mass:
    value: 6.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/greatsword
  implementation: dnd2024:equipment_greatsword
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 2d6
      type: slashing
      amount:
        dice:
          count: 2
          sides: 6
        bonus: 0
    properties:
    - id: heavy
      rule:
        key: dnd2024:rules/weapons_heavy
      required_ability: strength
      minimum_score: 13
      below_minimum_attack_mode: disadvantage
    - id: two_handed
      rule:
        key: dnd2024:rules/weapons_two_handed
      hands_to_attack: 2
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: graze
      rule:
        key: dnd2024:rules/weapons_graze
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: attack_misses_creature
        optional: true
        damage:
          amount:
            stat: attack_ability_modifier
          type: weapon_damage_type
          increases_only_from: attack_ability_modifier
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_greatsword
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
  - dnd2024:equipment_greatsword
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 51fad0d320bdc8bd331beb17ebcf2c5d6f2d11c0e216ca7112f0a5f92b747f0b
splat_clauses:
- key: source_rules
  text: '# Greatsword Source table, SRD 5.2.1 page 91. { "damage": { "expression": "2d6", "type": "slashing"
    }, "properties_text": "Heavy, Two-Handed", "mastery": "graze", "weapon_category": "martial melee",
    "price": { "value": 50, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: b31222167cc8383c38afc2f1f2edac726be9c43346112723736ac00ca011bf75
  status: contract_specified
splat_requires_mechanics: *id001
---

# Greatsword

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "2d6",
    "type": "slashing"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "graze",
  "weapon_category": "martial melee",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}
