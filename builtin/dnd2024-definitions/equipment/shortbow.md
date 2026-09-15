---
splat_schema: 1
splat_id: 0c114682-e678-5c96-857c-c90521f2f4d5
splat_kind: item
splat_key: dnd2024:equipment/shortbow
splat_name: Shortbow
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
  properties_text: Ammunition (Range 80/320; Arrow), Two-Handed
  mastery: vex
  weapon_category: simple ranged
  price:
    value: 25
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/shortbow
  implementation: dnd2024:equipment_shortbow
  parameters:
    weapon_category: simple_ranged
    damage:
      expression: 1d6
      type: piercing
      amount:
        dice:
          count: 1
          sides: 6
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
    - id: two_handed
      rule:
        key: dnd2024:rules/weapons_two_handed
      hands_to_attack: 2
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: dexterity
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
- id: dnd2024:equipment_shortbow
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
  - dnd2024:equipment_shortbow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f9e3ee2048c76ebada08cb95fbc0fc18ebc4737168223f8162057f2ff0dc67f1
splat_clauses:
- key: source_rules
  text: '# Shortbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "piercing"
    }, "properties_text": "Ammunition (Range 80/320; Arrow), Two-Handed", "mastery": "vex", "weapon_category":
    "simple ranged", "price": { "value": 25, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 972cffb42d110c1abc4519d7165c25a4524cb7bcb535ae6583915759a6da7854
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shortbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 80/320; Arrow), Two-Handed",
  "mastery": "vex",
  "weapon_category": "simple ranged",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
