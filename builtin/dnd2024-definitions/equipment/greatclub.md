---
splat_schema: 1
splat_id: f1b1ad07-9181-5438-a9a5-355447e5ccb0
splat_kind: item
splat_key: dnd2024:equipment/greatclub
splat_name: Greatclub
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: bludgeoning
  properties_text: Two-Handed
  mastery: push
  weapon_category: simple melee
  price:
    value: 2
    unit: sp
  mass:
    value: 10.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/greatclub
  implementation: dnd2024:equipment_greatclub
  parameters:
    weapon_category: simple_melee
    damage:
      expression: 1d8
      type: bludgeoning
      amount:
        dice:
          count: 1
          sides: 8
        bonus: 0
    properties:
    - id: two_handed
      rule:
        key: dnd2024:rules/weapons_two_handed
      hands_to_attack: 2
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
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
- id: dnd2024:equipment_greatclub
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
  - dnd2024:equipment_greatclub
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 64bcf689c3c8c311acb8896aa12c3efc4716247963692abbee6926fe987f88ff
splat_clauses:
- key: source_rules
  text: '# Greatclub Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "bludgeoning"
    }, "properties_text": "Two-Handed", "mastery": "push", "weapon_category": "simple melee", "price":
    { "value": 2, "unit": "sp" }, "mass": { "value": 10.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 7bbfef7e5052cae6b72479772882621f9d702e900ae292d270fcde86d044cda9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Greatclub

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "bludgeoning"
  },
  "properties_text": "Two-Handed",
  "mastery": "push",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "sp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}
