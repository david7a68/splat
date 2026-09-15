---
splat_schema: 1
splat_id: 2f94f18b-c79b-581d-85d2-0ad46fd926a3
splat_kind: item
splat_key: dnd2024:equipment/dart
splat_name: Dart
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
  properties_text: Finesse, Thrown (Range 20/60)
  mastery: vex
  weapon_category: simple ranged
  price:
    value: 5
    unit: cp
  mass:
    value: 0.25
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/dart
  implementation: dnd2024:equipment_dart
  parameters:
    weapon_category: simple_ranged
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
      base_ability: dexterity
      normal_hit_damage_ability_modifier: attack_ability_modifier
    mastery:
      property: vex
      rule:
        key: dnd2024:rules/weapons_vex
      requires_weapon_mastery_feature_for_this_weapon: true
      automatic_from_weapon_proficiency: false
      parameters:
        trigger: hit_and_damage_creature
        effect:
          attacker_attack_roll_mode: advantage
          against: same_target
          count: 1
          expiry: attacker_next_turn_end
    application:
      actor: equipper
      item: equipped_instance
      inactive_if_not_equipped: true
splat_contracts:
- id: dnd2024:equipment_dart
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
  - dnd2024:equipment_dart
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 6fe2a4a98331bc4fa8846212df896039b0636422b8360fc0cce4b8a66f34845c
splat_clauses:
- key: source_rules
  text: '# Dart Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d4", "type": "piercing"
    }, "properties_text": "Finesse, Thrown (Range 20/60)", "mastery": "vex", "weapon_category": "simple
    ranged", "price": { "value": 5, "unit": "cp" }, "mass": { "value": 0.25, "unit": "lb" }, "category":
    "weapon" }'
  sha256: c282e7a25ae029f86593a9c79d4b525b22befa885a7b8d98b389ababedd59c45
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dart

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d4",
    "type": "piercing"
  },
  "properties_text": "Finesse, Thrown (Range 20/60)",
  "mastery": "vex",
  "weapon_category": "simple ranged",
  "price": {
    "value": 5,
    "unit": "cp"
  },
  "mass": {
    "value": 0.25,
    "unit": "lb"
  },
  "category": "weapon"
}
