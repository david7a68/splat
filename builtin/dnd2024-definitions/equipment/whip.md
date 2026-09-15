---
splat_schema: 1
splat_id: 784710b1-91c1-5b33-9394-492f4b54b64e
splat_kind: item
splat_key: dnd2024:equipment/whip
splat_name: Whip
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d4
    type: slashing
  properties_text: Finesse, Reach
  mastery: slow
  weapon_category: martial melee
  price:
    value: 2
    unit: gp
  mass:
    value: 3.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/whip
  implementation: dnd2024:equipment_whip
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d4
      type: slashing
      amount:
        dice:
          count: 1
          sides: 4
        bonus: 0
    properties:
    - id: finesse
      rule:
        key: dnd2024:rules/weapons_finesse
      attack_and_damage_ability_choice:
      - strength
      - dexterity
      same_ability_for_both: true
    - id: reach
      rule:
        key: dnd2024:rules/weapons_reach
      reach_bonus_ft: 5
      applies_to:
      - attacks
      - opportunity_attack_reach
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
- id: dnd2024:equipment_whip
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
  - dnd2024:equipment_whip
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: f84cbc0bf769aeba7dfe83c5a9ed2706e2d1c236ac46ff0ab60a3964aa51dcdc
splat_clauses:
- key: source_rules
  text: '# Whip Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "slashing"
    }, "properties_text": "Finesse, Reach", "mastery": "slow", "weapon_category": "martial melee", "price":
    { "value": 2, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 14c236948746adeb1104776e6588df788bd8c6718d80a0c29f60f33ceb77f903
  status: contract_specified
splat_requires_mechanics: *id001
---

# Whip

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "slashing"
  },
  "properties_text": "Finesse, Reach",
  "mastery": "slow",
  "weapon_category": "martial melee",
  "price": {
    "value": 2,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
