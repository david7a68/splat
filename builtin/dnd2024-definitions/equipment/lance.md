---
splat_schema: 1
splat_id: 0b572249-2f3d-5e90-a2b7-1aa71dfef47d
splat_kind: item
splat_key: dnd2024:equipment/lance
splat_name: Lance
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d10
    type: piercing
  properties_text: Heavy, Reach, Two-Handed (unless mounted)
  mastery: topple
  weapon_category: martial melee
  price:
    value: 10
    unit: gp
  mass:
    value: 6.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/lance
  implementation: dnd2024:equipment_lance
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d10
      type: piercing
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
      exception:
        if: mounted
        hands_to_attack: 1
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
- id: dnd2024:equipment_lance
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
  - dnd2024:equipment_lance
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 6ccaacef05effc93f62f76f6c59ed1e62d539b950378550496896966ff776299
splat_clauses:
- key: source_rules
  text: '# Lance Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d10", "type": "piercing"
    }, "properties_text": "Heavy, Reach, Two-Handed (unless mounted)", "mastery": "topple", "weapon_category":
    "martial melee", "price": { "value": 10, "unit": "gp" }, "mass": { "value": 6.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: afc75d9ddcd43088873b82d96778c7cf99ad2a7e8b9b8f34c32262121cf6a483
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lance

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d10",
    "type": "piercing"
  },
  "properties_text": "Heavy, Reach, Two-Handed (unless mounted)",
  "mastery": "topple",
  "weapon_category": "martial melee",
  "price": {
    "value": 10,
    "unit": "gp"
  },
  "mass": {
    "value": 6.0,
    "unit": "lb"
  },
  "category": "weapon"
}
