---
splat_schema: 1
splat_id: a151446c-de15-554e-87c3-b8d5ded116c4
splat_kind: item
splat_key: dnd2024:equipment/light_crossbow
splat_name: Light Crossbow
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
  properties_text: Ammunition (Range 80/320; Bolt), Loading, Two-Handed
  mastery: slow
  weapon_category: simple ranged
  price:
    value: 25
    unit: gp
  mass:
    value: 5.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/light_crossbow
  implementation: dnd2024:equipment_light_crossbow
  parameters:
    weapon_category: simple_ranged
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
        normal_ft: 80
        long_ft: 320
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      ammunition:
        key: dnd2024:equipment/ammunition_varies
        variant: bolts
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
- id: dnd2024:equipment_light_crossbow
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
  - dnd2024:equipment_light_crossbow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c74ce9b6bd7675cfaab3e0926aa9d3c8069e7d3328cbff9168ed70cf138ae2ca
splat_clauses:
- key: source_rules
  text: '# Light Crossbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type":
    "piercing" }, "properties_text": "Ammunition (Range 80/320; Bolt), Loading, Two-Handed", "mastery":
    "slow", "weapon_category": "simple ranged", "price": { "value": 25, "unit": "gp" }, "mass": { "value":
    5.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 1bdf9da11b7a81c66ceee844ba450d569cd72e6c2f6428950a5027980b62da35
  status: contract_specified
splat_requires_mechanics: *id001
---

# Light Crossbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 80/320; Bolt), Loading, Two-Handed",
  "mastery": "slow",
  "weapon_category": "simple ranged",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 5.0,
    "unit": "lb"
  },
  "category": "weapon"
}
