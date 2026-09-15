---
splat_schema: 1
splat_id: afa7549a-80b1-5fae-b129-44c01853b6c5
splat_kind: item
splat_key: dnd2024:equipment/greataxe
splat_name: Greataxe
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d12
    type: slashing
  properties_text: Heavy, Two-Handed
  mastery: cleave
  weapon_category: martial melee
  price:
    value: 30
    unit: gp
  mass:
    value: 7.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/greataxe
  implementation: dnd2024:equipment_greataxe
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d12
      type: slashing
      amount:
        dice:
          count: 1
          sides: 12
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
      property: cleave
      rule:
        key: dnd2024:rules/weapons_cleave
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: melee_attack_hits_creature
        optional: true
        extra_melee_attack:
          target: second_creature
          within_first_target_ft: 5
          within_weapon_reach: true
          damage_ability_modifier: negative_only
        maximum_per_turn: 1
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_greataxe
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
  - dnd2024:equipment_greataxe
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 81d79a6895bea0e561c0c467165f338f271f676fa7ca16e50dba7bdea4dc94ac
splat_clauses:
- key: source_rules
  text: '# Greataxe Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d12", "type": "slashing"
    }, "properties_text": "Heavy, Two-Handed", "mastery": "cleave", "weapon_category": "martial melee",
    "price": { "value": 30, "unit": "gp" }, "mass": { "value": 7.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: d7c057a9f70bc3aecc402876651718549e6db103071fd6aeee15f3fea36ccb1b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Greataxe

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d12",
    "type": "slashing"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "cleave",
  "weapon_category": "martial melee",
  "price": {
    "value": 30,
    "unit": "gp"
  },
  "mass": {
    "value": 7.0,
    "unit": "lb"
  },
  "category": "weapon"
}
