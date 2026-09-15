---
splat_schema: 1
splat_id: 5014d8a6-e3cc-50b0-986e-e8454591d066
splat_kind: item
splat_key: dnd2024:equipment/pike
splat_name: Pike
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d10
    type: piercing
  properties_text: Heavy, Reach, Two-Handed
  mastery: push
  weapon_category: martial melee
  price:
    value: 5
    unit: gp
  mass:
    value: 18.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/pike
  implementation: dnd2024:equipment_pike
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d10
      type: piercing
      amount:
        dice:
          count: 1
          sides: 10
        bonus: 0
    properties:
    - id: heavy
      rule:
        key: dnd2024:rules/weapons_heavy
      required_ability: strength
      minimum_score: 13
      below_minimum_attack_mode: disadvantage
    - id: reach
      rule:
        key: dnd2024:rules/weapons_reach
      reach_bonus_ft: 5
      applies_to:
      - attacks
      - opportunity_attack_reach
    - id: two_handed
      rule:
        key: dnd2024:rules/weapons_two_handed
      hands_to_attack: 2
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
- id: dnd2024:equipment_pike
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
  - dnd2024:equipment_pike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 12c46bdec6ee41f02666bf24d44cff5ad9105c658302fb1400662b6a0991b21b
splat_clauses:
- key: source_rules
  text: '# Pike Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "piercing"
    }, "properties_text": "Heavy, Reach, Two-Handed", "mastery": "push", "weapon_category": "martial melee",
    "price": { "value": 5, "unit": "gp" }, "mass": { "value": 18.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: cda144b4f10a870c0b82332419fe59a2acd5d347ceb1e4c361a9b7bc05f6e8f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pike

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Heavy, Reach, Two-Handed",
  "mastery": "push",
  "weapon_category": "martial melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 18.0,
    "unit": "lb"
  },
  "category": "weapon"
}
