---
splat_schema: 1
splat_id: 0a69bbd7-90e7-52ff-8275-9d8cbc5959be
splat_kind: item
splat_key: dnd2024:equipment/sickle
splat_name: Sickle
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
  properties_text: Light
  mastery: nick
  weapon_category: simple melee
  price:
    value: 1
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/sickle
  implementation: dnd2024:equipment_sickle
  parameters:
    weapon_category: simple_melee
    damage:
      expression: 1d4
      type: slashing
      amount:
        dice:
          count: 1
          sides: 4
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
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: nick
      rule:
        key: dnd2024:rules/weapons_nick
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: light_property_extra_attack
        replace_economy: bonus_action
        economy: part_of_attack_action
        maximum_per_turn: 1
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_sickle
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
  - dnd2024:equipment_sickle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 22088cc1d3584b58899ee1973e61d558edcbcc964ad7186ab6e6894117e2db97
splat_clauses:
- key: source_rules
  text: '# Sickle Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "slashing"
    }, "properties_text": "Light", "mastery": "nick", "weapon_category": "simple melee", "price": { "value":
    1, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 96021af32cb7d7575508b069b5a8ef27d579e5682879da7e4baa7b8c4eae483f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sickle

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "slashing"
  },
  "properties_text": "Light",
  "mastery": "nick",
  "weapon_category": "simple melee",
  "price": {
    "value": 1,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
