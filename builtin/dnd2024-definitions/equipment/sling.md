---
splat_schema: 1
splat_id: 951565b3-3df2-56cf-9aa0-e5cba90132d7
splat_kind: item
splat_key: dnd2024:equipment/sling
splat_name: Sling
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
  properties_text: Ammunition (Range 30/120; Bullet)
  mastery: slow
  weapon_category: simple ranged
  price:
    value: 1
    unit: sp
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/sling
  implementation: dnd2024:equipment_sling
  parameters:
    weapon_category: simple_ranged
    damage:
      expression: 1d4
      type: bludgeoning
      amount:
        dice:
          count: 1
          sides: 4
        bonus: 0
    properties:
    - id: ammunition
      rule:
        key: dnd2024:rules/weapons_ammunition
      range:
        normal_ft: 30
        long_ft: 120
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      ammunition:
        key: dnd2024:equipment/ammunition_varies
        variant: sling_bullets
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
    mass:
      value: 0
      unit: lb
      negligible: true
splat_contracts:
- id: dnd2024:equipment_sling
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
  - dnd2024:equipment_sling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 7142505757ff61f5780bb09262ed55d965ad45f324b705a81d2174396e2c5182
splat_clauses:
- key: source_rules
  text: '# Sling Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "bludgeoning"
    }, "properties_text": "Ammunition (Range 30/120; Bullet)", "mastery": "slow", "weapon_category": "simple
    ranged", "price": { "value": 1, "unit": "sp" }, "category": "weapon" }'
  sha256: 623cbdb72966c323a5cbd629582355e5d84182c1beb79841afa7a66196b73316
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sling

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "bludgeoning"
  },
  "properties_text": "Ammunition (Range 30/120; Bullet)",
  "mastery": "slow",
  "weapon_category": "simple ranged",
  "price": {
    "value": 1,
    "unit": "sp"
  },
  "category": "weapon"
}
