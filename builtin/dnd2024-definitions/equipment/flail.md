---
splat_schema: 1
splat_id: d5b5cbee-22de-545c-b755-c4b91dc3f33e
splat_kind: item
splat_key: dnd2024:equipment/flail
splat_name: Flail
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
  properties_text: —
  mastery: sap
  weapon_category: martial melee
  price:
    value: 10
    unit: gp
  mass:
    value: 2.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/flail
  implementation: dnd2024:equipment_flail
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d8
      type: bludgeoning
      amount:
        dice:
          count: 1
          sides: 8
        bonus: 0
    properties: []
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: sap
      rule:
        key: dnd2024:rules/weapons_sap
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_creature
        effect:
          attack_roll_mode: disadvantage
          count: 1
          expiry: attacker_next_turn_start
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_flail
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
  - dnd2024:equipment_flail
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 0044626ae17f9a68eb5b766b1d1501966288c6a41bb97304f028205354ef5887
splat_clauses:
- key: source_rules
  text: '# Flail Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "bludgeoning"
    }, "properties_text": "—", "mastery": "sap", "weapon_category": "martial melee", "price": { "value":
    10, "unit": "gp" }, "mass": { "value": 2.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 3c621adc103cad3bb91e84208b5222cc921f4b6c5e1f301f45a7e9f4482b7a20
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flail

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "bludgeoning"
  },
  "properties_text": "—",
  "mastery": "sap",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 2.0,
    "unit": "lb"
  },
  "category": "weapon"
}
