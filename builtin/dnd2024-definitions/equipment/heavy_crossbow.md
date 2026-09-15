---
splat_schema: 1
splat_id: 2617805c-1c9e-5706-b9e2-3c769e36b321
splat_kind: item
splat_key: dnd2024:equipment/heavy_crossbow
splat_name: Heavy Crossbow
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
  properties_text: Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed
  mastery: push
  weapon_category: martial ranged
  price:
    value: 50
    unit: gp
  mass:
    value: 18.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/heavy_crossbow
  implementation: dnd2024:equipment_heavy_crossbow
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
        normal_ft: 100
        long_ft: 400
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
    - id: heavy
      rule:
        key: dnd2024:rules/weapons_heavy
      required_ability: dexterity
      minimum_score: 13
      below_minimum_attack_mode: disadvantage
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
- id: dnd2024:equipment_heavy_crossbow
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
  - dnd2024:equipment_heavy_crossbow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 417dbd8f74ad5a55223f319eb28b9c47f4d86c04c6c239ba5f3d824804ac27b7
splat_clauses:
- key: source_rules
  text: '# Heavy Crossbow Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type":
    "piercing" }, "properties_text": "Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed", "mastery":
    "push", "weapon_category": "martial ranged", "price": { "value": 50, "unit": "gp" }, "mass": { "value":
    18.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 8597294fb3f9f22ab99ce60ffc796e65ff35239ba251073e5b327df0a1608b0f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Heavy Crossbow

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 100/400; Bolt), Heavy, Loading, Two-Handed",
  "mastery": "push",
  "weapon_category": "martial ranged",
  "price": {
    "value": 50,
    "unit": "gp"
  },
  "mass": {
    "value": 18.0,
    "unit": "lb"
  },
  "category": "weapon"
}
