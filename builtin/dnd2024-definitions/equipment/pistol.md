---
splat_schema: 1
splat_id: ba85cb49-8b93-51b2-aa6d-cb0355c8bd1b
splat_kind: item
splat_key: dnd2024:equipment/pistol
splat_name: Pistol
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
  properties_text: Ammunition (Range 30/90; Bullet), Loading
  mastery: vex
  weapon_category: martial ranged
  price:
    value: 250
    unit: gp
  mass:
    value: 3.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/pistol
  implementation: dnd2024:equipment_pistol
  parameters:
    weapon_category: martial_ranged
    damage:
      expression: 1d10
      type: piercing
      amount:
        dice:
          count: 1
          sides: 10
        bonus: 0
    properties:
    - id: ammunition
      rule:
        key: dnd2024:rules/weapons_ammunition
      range:
        normal_ft: 30
        long_ft: 90
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
- id: dnd2024:equipment_pistol
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
  - dnd2024:equipment_pistol
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 8c0fc27c80c7eca5c09b616f27b9f136e4674e87a91d0737b13e08c6fd8338e4
splat_clauses:
- key: source_rules
  text: '# Pistol Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "piercing"
    }, "properties_text": "Ammunition (Range 30/90; Bullet), Loading", "mastery": "vex", "weapon_category":
    "martial ranged", "price": { "value": 250, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" },
    "category": "weapon" }'
  sha256: d9e2d1e1ff61317ac7ace1ab2f436be739b642853e650f4c3f1136aa29645c90
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pistol

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 30/90; Bullet), Loading",
  "mastery": "vex",
  "weapon_category": "martial ranged",
  "price": {
    "value": 250,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
