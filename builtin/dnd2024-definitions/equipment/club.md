---
splat_schema: 1
splat_id: 3c4b0347-3550-537b-ad54-8f2b43d25f9d
splat_kind: item
splat_key: dnd2024:equipment/club
splat_name: Club
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d4
    type: bludgeoning
  properties_text: Light
  mastery: slow
  weapon_category: simple melee
  price:
    value: 1
    unit: sp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/club
  implementation: dnd2024:equipment_club
  parameters:
    weapon_category: simple_melee
    damage:
      expression: 1d4
      type: bludgeoning
      amount:
        dice:
          count: 1
          sides: 4
        bonus: 0
    properties:
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
- id: dnd2024:equipment_club
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
  - dnd2024:equipment_club
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: b6d7365a95747e3b45f6c7f179987aadd8bd67454ca5af2751d2be391fbbf813
splat_clauses:
- key: source_rules
  text: '# Club Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "bludgeoning"
    }, "properties_text": "Light", "mastery": "slow", "weapon_category": "simple melee", "price": { "value":
    1, "unit": "sp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: d5e6fd26ffb9d7242dfa4a3ecfa855a2a2efbe19bb213c2dc4abad471752cac8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Club

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "bludgeoning"
  },
  "properties_text": "Light",
  "mastery": "slow",
  "weapon_category": "simple melee",
  "price": {
    "value": 1,
    "unit": "sp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
