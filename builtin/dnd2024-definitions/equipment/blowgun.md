---
splat_schema: 1
splat_id: 25cd7903-6852-5d19-a127-ed37e95b775a
splat_kind: item
splat_key: dnd2024:equipment/blowgun
splat_name: Blowgun
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: '1'
    type: piercing
  properties_text: Ammunition (Range 25/100; Needle), Loading
  mastery: vex
  weapon_category: martial ranged
  price:
    value: 10
    unit: gp
  mass:
    value: 1.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/blowgun
  implementation: dnd2024:equipment_blowgun
  parameters:
    weapon_category: martial_ranged
    damage:
      expression: '1'
      type: piercing
      amount: 1
    properties:
    - id: ammunition
      rule:
        key: dnd2024:rules/weapons_ammunition
      range:
        normal_ft: 25
        long_ft: 100
        normal_range_mode: normal
        long_range_mode: disadvantage
        beyond_long_range: cannot_attack
        rule:
          key: dnd2024:rules/weapons_range
      ammunition:
        key: dnd2024:equipment/ammunition_varies
        variant: needles
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
- id: dnd2024:equipment_blowgun
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
  - dnd2024:equipment_blowgun
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 978acafdfab746d45d89a0a190516360908b36ee69e654a34ecdc2a0957d2b8d
splat_clauses:
- key: source_rules
  text: '# Blowgun Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1", "type": "piercing"
    }, "properties_text": "Ammunition (Range 25/100; Needle), Loading", "mastery": "vex", "weapon_category":
    "martial ranged", "price": { "value": 10, "unit": "gp" }, "mass": { "value": 1.0, "unit": "lb" },
    "category": "weapon" }'
  sha256: 37085ebcb1694eb1d73213ed817357fc4b6095544278eab0b32bc9cd73d60fd8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blowgun

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1",
    "type": "piercing"
  },
  "properties_text": "Ammunition (Range 25/100; Needle), Loading",
  "mastery": "vex",
  "weapon_category": "martial ranged",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 1.0,
    "unit": "lb"
  },
  "category": "weapon"
}
