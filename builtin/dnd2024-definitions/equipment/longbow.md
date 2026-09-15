---
splat_schema: 1
splat_id: 130bc64f-94ba-5ac3-bc03-c7fe74e42bc3
splat_kind: item
splat_key: dnd2024:equipment/longbow
splat_name: Longbow
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
  properties_text: Ammunition (Range 150/600; Arrow), Heavy, Two-Handed
  mastery: slow
  weapon_category: martial ranged
  price:
    value: 50
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/longbow
  implementation: dnd2024:equipment_longbow
  parameters:
    weapon_category: martial_ranged
    damage:
      expression: 1d8
      type: piercing
      amount:
        dice:
          count: 1
          sides: 8
        bonus: 0
    properties:
    - id: ammunition
      rule:
        key: dnd2024:rules/weapons_ammunition
      range:
        normal_ft: 150
        long_ft: 600
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      ammunition:
        key: dnd2024:equipment/ammunition_varies
        variant: arrows
      expend_per_attack: 1
      draw_as_part_of_attack: true
      free_hand_to_load_if_one_handed: true
      recovery:
        duration:
          value: 1
          unit: minute
        timing: after_fight
        fraction: 0.5
        rounding: down
        unrecovered: lost
    - id: heavy
      rule:
        key: dnd2024:rules/weapons_heavy
      required_ability: dexterity
      minimum_score: 13
      below_minimum_attack_mode: disadvantage
    - id: two_handed
      rule:
        key: dnd2024:rules/weapons_two_handed
      hands_to_attack: 2
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: dexterity
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
- id: dnd2024:equipment_longbow
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
  - dnd2024:equipment_longbow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: d9bc7807522903b079823c0099abaa5af56f7641f4653c4335efb481ebabca6c
splat_clauses:
- key: source_rules
  text: '# Longbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "Ammunition (Range 150/600; Arrow), Heavy, Two-Handed", "mastery": "slow", "weapon_category":
    "martial ranged", "price": { "value": 50, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" },
    "category": "weapon" }'
  sha256: 2fae0dd0cf45c7a4ed3bcdc8963e271a148fc231d22dce3b4c65ae98f8fa1455
  status: contract_specified
splat_requires_mechanics: *id001
---

# Longbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 150/600; Arrow), Heavy, Two-Handed",
  "mastery": "slow",
  "weapon_category": "martial ranged",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
