---
splat_schema: 1
splat_id: 66dd5ecc-0fb7-5919-ad05-d867cb25d41f
splat_kind: item
splat_key: dnd2024:equipment/mace
splat_name: Mace
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: bludgeoning
  properties_text: —
  mastery: sap
  weapon_category: simple melee
  price:
    value: 5
    unit: gp
  mass:
    value: 4.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/mace
  implementation: dnd2024:equipment_mace
  parameters:
    weapon_category: simple_melee
    damage:
      expression: 1d6
      type: bludgeoning
      amount:
        dice:
          count: 1
          sides: 6
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
- id: dnd2024:equipment_mace
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
  - dnd2024:equipment_mace
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 30a0de9841dd604e2988d1e09a4a0024218c3dca2d77c8b038e6528c3efcf157
splat_clauses:
- key: source_rules
  text: '# Mace Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "bludgeoning"
    }, "properties_text": "—", "mastery": "sap", "weapon_category": "simple melee", "price": { "value":
    5, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 64a084de497191307f5193d9b593eaaf801750d30b0642589f72dbddc1e6fb4b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mace

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "bludgeoning"
  },
  "properties_text": "—",
  "mastery": "sap",
  "weapon_category": "simple melee",
  "price": {
    "value": 5,
    "unit": "gp"
  },
  "mass": {
    "value": 4.0,
    "unit": "lb"
  },
  "category": "weapon"
}
