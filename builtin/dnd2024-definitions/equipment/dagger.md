---
splat_schema: 1
splat_id: 3302150c-7356-5633-9215-20d96c893348
splat_kind: item
splat_key: dnd2024:equipment/dagger
splat_name: Dagger
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d4
    type: piercing
  properties_text: Finesse, Light, Thrown (Range 20/60)
  mastery: nick
  weapon_category: simple melee
  price:
    value: 2
    unit: gp
  mass:
    value: 1.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/dagger
  implementation: dnd2024:equipment_dagger
  parameters:
    weapon_category: simple_melee
    damage:
      expression: 1d4
      type: piercing
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
- id: dnd2024:equipment_dagger
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
  - dnd2024:equipment_dagger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 3cc7e8b8c9185a854079c7ec77f429b2e751b5b6ce5272221dcac92243e7de82
splat_clauses:
- key: source_rules
  text: '# Dagger Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "piercing"
    }, "properties_text": "Finesse, Light, Thrown (Range 20/60)", "mastery": "nick", "weapon_category":
    "simple melee", "price": { "value": 2, "unit": "gp" }, "mass": { "value": 1.0, "unit": "lb" }, "category":
    "weapon" }'
  sha256: cffc1783049b16e9bf8688e1ff506dd47aa0fb794bba453373a66ad8a638286e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dagger

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "piercing"
  },
  "properties_text": "Finesse, Light, Thrown (Range 20/60)",
  "mastery": "nick",
  "weapon_category": "simple melee",
  "price": {
    "value": 2,
    "unit": "gp"
  },
  "mass": {
    "value": 1.0,
    "unit": "lb"
  },
  "category": "weapon"
}
