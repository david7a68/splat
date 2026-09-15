---
splat_schema: 1
splat_id: 8f28f780-21f0-5845-824e-5145c40d7fe5
splat_kind: item
splat_key: dnd2024:equipment/musket
splat_name: Musket
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d12
    type: piercing
  properties_text: Ammunition (Range 40/120; Bullet), Loading, Two-Handed
  mastery: slow
  weapon_category: martial ranged
  price:
    value: 500
    unit: gp
  mass:
    value: 10.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/musket
  implementation: dnd2024:equipment_musket
  parameters:
    weapon_category: martial_ranged
    damage:
      expression: 1d12
      type: piercing
      amount:
        dice:
          count: 1
          sides: 12
        bonus: 0
    properties:
    - id: ammunition
      rule:
        key: dnd2024:rules/weapons_ammunition
      range:
        normal_ft: 40
        long_ft: 120
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      ammunition:
        key: dnd2024:equipment/ammunition_varies
        variant: firearm_bullets
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
    - id: loading
      rule:
        key: dnd2024:rules/weapons_loading
      ammunition_limit: 1
      per:
      - action
      - bonus_action
      - reaction
      ignores_extra_attacks: true
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
- id: dnd2024:equipment_musket
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
  - dnd2024:equipment_musket
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 97ff81aeaeca150c9cd5633ee77552263c3a54c2f163c85c7e2de797a1ec3462
splat_clauses:
- key: source_rules
  text: '# Musket Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d12", "type": "piercing"
    }, "properties_text": "Ammunition (Range 40/120; Bullet), Loading, Two-Handed", "mastery": "slow",
    "weapon_category": "martial ranged", "price": { "value": 500, "unit": "gp" }, "mass": { "value": 10.0,
    "unit": "lb" }, "category": "weapon" }'
  sha256: fb98819000cade7a8d201d8aa243650d2e386926fdae51a15bf0941546bea0a6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Musket

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d12",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 40/120; Bullet), Loading, Two-Handed",
  "mastery": "slow",
  "weapon_category": "martial ranged",
  "price": {
    "value": 500,
    "unit": "gp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}
