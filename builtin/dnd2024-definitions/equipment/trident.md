---
splat_schema: 1
splat_id: 6c9dd52c-59b0-595d-8f70-19587d9c0690
splat_kind: item
splat_key: dnd2024:equipment/trident
splat_name: Trident
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d8
    type: piercing
  properties_text: Thrown (Range 20/60), Versatile (1d10)
  mastery: topple
  weapon_category: martial melee
  price:
    value: 5
    unit: gp
  mass:
    value: 4.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/trident
  implementation: dnd2024:equipment_trident
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d8
      type: piercing
      amount:
        dice:
          count: 1
          sides: 8
        bonus: 0
    properties:
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
    - id: versatile
      rule:
        key: dnd2024:rules/weapons_versatile
      two_hand_melee_damage:
        expression: 1d10
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
- id: dnd2024:equipment_trident
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
  - dnd2024:equipment_trident
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: b7145106d8a9b26dc5d11459353d8264455ef6d2ccd5169f1078ac644624a2e4
splat_clauses:
- key: source_rules
  text: '# Trident Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d8", "type": "piercing"
    }, "properties_text": "Thrown (Range 20/60), Versatile (1d10)", "mastery": "topple", "weapon_category":
    "martial melee", "price": { "value": 5, "unit": "gp" }, "mass": { "value": 4.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: 1b690ac20195da2eca99ce891b25c3829748ac4495b5926f0eed56016dd09776
  status: contract_specified
splat_requires_mechanics: *id001
---

# Trident

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d8",
    "type": "piercing"
  },
  "properties_text": "Thrown (Range 20/60), Versatile (1d10)",
  "mastery": "topple",
  "weapon_category": "martial melee",
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
