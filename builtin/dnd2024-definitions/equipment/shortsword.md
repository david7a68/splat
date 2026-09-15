---
splat_schema: 1
splat_id: d9b30ace-5934-5262-8603-88580f1712ff
splat_kind: item
splat_key: dnd2024:equipment/shortsword
splat_name: Shortsword
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
  properties_text: Finesse, Light
  mastery: vex
  weapon_category: martial melee
  price:
    value: 10
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/shortsword
  implementation: dnd2024:equipment_shortsword
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d6
      type: piercing
      amount:
        dice:
          count: 1
          sides: 6
        bonus: 0
    properties:
    - id: finesse
      rule:
        key: dnd2024:rules/weapons_finesse
      attack_and_damage_ability_choice:
      - strength
      - dexterity
      same_ability_for_both: true
    - id: light
      rule:
        key: dnd2024:rules/weapons_light
      trigger: attack_with_light_weapon_during_own_attack_action
      extra_attack:
        count: 1
        economy: bonus_action
        timing: later_same_turn
        weapon: different_light_weapon
        damage_ability_modifier: negative_only
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: vex
      rule:
        key: dnd2024:rules/weapons_vex
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_and_damage_creature
        effect:
          attacker_attack_roll_mode: advantage
          against: same_target
          count: 1
          expiry: attacker_next_turn_end
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_shortsword
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
  - dnd2024:equipment_shortsword
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 00cc2b0a6fe2dc165a4489e63bc7412599dca41fe97ae436ab520bc4c1bfe8b1
splat_clauses:
- key: source_rules
  text: '# Shortsword Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Finesse, Light", "mastery": "vex", "weapon_category": "martial melee", "price":
    { "value": 10, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 9e98890dde8da49dfce602cec0b9d193369b45b56c2881c0159e130583992612
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shortsword

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Finesse, Light",
  "mastery": "vex",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
