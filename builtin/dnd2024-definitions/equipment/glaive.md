---
splat_schema: 1
splat_id: ac18c3fb-1491-5fe6-8af5-9f43d342ae4d
splat_kind: item
splat_key: dnd2024:equipment/glaive
splat_name: Glaive
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d10
    type: slashing
  properties_text: Heavy, Reach, Two-Handed
  mastery: graze
  weapon_category: martial melee
  price:
    value: 20
    unit: gp
  mass:
    value: 6.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/glaive
  implementation: dnd2024:equipment_glaive
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d10
      type: slashing
      amount:
        dice:
          count: 1
          sides: 10
        bonus: 0
    properties:
    - id: heavy
      rule:
        key: dnd2024:rules/weapons_heavy
      required_ability: strength
      minimum_score: 13
      below_minimum_attack_mode: disadvantage
    - id: reach
      rule:
        key: dnd2024:rules/weapons_reach
      reach_bonus_ft: 5
      applies_to:
      - attacks
      - opportunity_attack_reach
    - id: two_handed
      rule:
        key: dnd2024:rules/weapons_two_handed
      hands_to_attack: 2
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: graze
      rule:
        key: dnd2024:rules/weapons_graze
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: attack_misses_creature
        optional: true
        damage:
          amount:
            stat: attack_ability_modifier
          type: weapon_damage_type
          increases_only_from: attack_ability_modifier
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_glaive
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
  - dnd2024:equipment_glaive
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: e2bfc39a21f9de17a134b3fea1f7faaea771872bfe1be24ae876dd8a339eea39
splat_clauses:
- key: source_rules
  text: '# Glaive Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "slashing"
    }, "properties_text": "Heavy, Reach, Two-Handed", "mastery": "graze", "weapon_category": "martial
    melee", "price": { "value": 20, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 683019297cbcf217fe5469adc285002fe5d7f1347ec909e17d34489c9b378773
  status: contract_specified
splat_requires_mechanics: *id001
---

# Glaive

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "slashing"
  },
  "properties_text": "Heavy, Reach, Two-Handed",
  "mastery": "graze",
  "weapon_category": "martial melee",
  "price": {
    "value": 20,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}
