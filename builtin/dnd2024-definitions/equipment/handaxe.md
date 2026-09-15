---
splat_schema: 1
splat_id: 34e4dda3-515b-589c-b53c-ff865efe81ce
splat_kind: item
splat_key: dnd2024:equipment/handaxe
splat_name: Handaxe
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: slashing
  properties_text: Light, Thrown (Range 20/60)
  mastery: vex
  weapon_category: simple melee
  price:
    value: 5
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/handaxe
  implementation: dnd2024:equipment_handaxe
  parameters:
    weapon_category: simple_melee
    damage:
      expression: 1d6
      type: slashing
      amount:
        dice:
          count: 1
          sides: 6
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
    - id: thrown
      rule:
        key: dnd2024:rules/weapons_thrown
      range:
        normal_ft: 20
        long_ft: 60
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      draw_as_part_of_attack: true
      attack_type: ranged
      melee_weapon_uses_melee_ability: true
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
- id: dnd2024:equipment_handaxe
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
  - dnd2024:equipment_handaxe
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 8e4e0c662b662115152d26ff7eada9693ce969fe56d951387994134c1ecc5c72
splat_clauses:
- key: source_rules
  text: '# Handaxe Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "slashing"
    }, "properties_text": "Light, Thrown (Range 20/60)", "mastery": "vex", "weapon_category": "simple
    melee", "price": { "value": 5, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 27c90d74ce0eb5e1636e76e99508d198a340c0624eb530cc83be4a0bdf965952
  status: contract_specified
splat_requires_mechanics: *id001
---

# Handaxe

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "slashing"
  },
  "properties_text": "Light, Thrown (Range 20/60)",
  "mastery": "vex",
  "weapon_category": "simple melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
