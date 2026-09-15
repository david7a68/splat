---
splat_schema: 1
splat_id: 91c6c731-ec05-5094-91d8-e2ecc313ae6a
splat_kind: item
splat_key: dnd2024:equipment/maul
splat_name: Maul
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 2d6
    type: bludgeoning
  properties_text: Heavy, Two-Handed
  mastery: topple
  weapon_category: martial melee
  price:
    value: 10
    unit: gp
  mass:
    value: 10.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/maul
  implementation: dnd2024:equipment_maul
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 2d6
      type: bludgeoning
      amount:
        dice:
          count: 2
          sides: 6
        bonus: 0
    properties:
    - id: heavy
      rule:
        key: dnd2024:rules/weapons_heavy
      required_ability: strength
      minimum_score: 13
      below_minimum_attack_mode: disadvantage
    - id: two_handed
      rule:
        key: dnd2024:rules/weapons_two_handed
      hands_to_attack: 2
    attack:
      proficiency_bonus_if_proficient: true
      base_ability: strength
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: topple
      rule:
        key: dnd2024:rules/weapons_topple
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_creature
        optional: true
        save:
          ability: constitution
          dc:
            op: add
            operands:
            - 8
            - stat: attack_ability_modifier
            - stat: proficiency_bonus
          failure:
            condition: prone
          success: no_effect
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_maul
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
  - dnd2024:equipment_maul
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c00667d95de31024eca633050832bd1a70550f4e83edab45bceac6689e538c6a
splat_clauses:
- key: source_rules
  text: '# Maul Source table, SRD 5.2.1 page 91. { "damage": { "expression": "2d6", "type": "bludgeoning"
    }, "properties_text": "Heavy, Two-Handed", "mastery": "topple", "weapon_category": "martial melee",
    "price": { "value": 10, "unit": "gp" }, "mass": { "value": 10.0, "unit": "lb" }, "category": "weapon"
    }'
  sha256: 6fda1590809f6d473c050ac98b58d934c3eb5d1e0638a80392e20d70b5bec58c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Maul

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "2d6",
    "type": "bludgeoning"
  },
  "properties_text": "Heavy, Two-Handed",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 10.0,
    "unit": "lb"
  },
  "category": "weapon"
}
