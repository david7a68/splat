---
splat_schema: 1
splat_id: 37150ba9-bad3-505b-9d1b-70afd910b940
splat_kind: item
splat_key: dnd2024:equipment/scimitar
splat_name: Scimitar
splat_origin:
  key: dnd2024:reference/pages/091
splat_sources:
- title: SRD 5.2.1
  locator: pp. 91
  url: https://www.dndbeyond.com/srd
splat_item:
  damage:
    expression: 1d6
    type: slashing
  properties_text: Finesse, Light
  mastery: nick
  weapon_category: martial melee
  price:
    value: 25
    unit: gp
  mass:
    value: 3.0
    unit: lb
  category: weapon
splat_rule:
  mechanic: dnd2024:equipment/scimitar
  implementation: dnd2024:equipment_scimitar
  parameters:
    weapon_category: martial_melee
    damage:
      expression: 1d6
      type: slashing
      amount:
        dice:
          count: 1
          sides: 6
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
- id: dnd2024:equipment_scimitar
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
  - dnd2024:equipment_scimitar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: 9df0f9829fa632f97fc826acd242db5e51f02776e3f0479106038b2b163d81fb
splat_clauses:
- key: source_rules
  text: '# Scimitar Source table, SRD 5.2.1 page 91. { "damage": { "expression": "1d6", "type": "slashing"
    }, "properties_text": "Finesse, Light", "mastery": "nick", "weapon_category": "martial melee", "price":
    { "value": 25, "unit": "gp" }, "mass": { "value": 3.0, "unit": "lb" }, "category": "weapon" }'
  sha256: 1367fe3632add25fdaebc281e20d0a67d3de8a67ac9d6449296972494752fafa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scimitar

Source table, SRD 5.2.1 page 91.

{
  "damage": {
    "expression": "1d6",
    "type": "slashing"
  },
  "properties_text": "Finesse, Light",
  "mastery": "nick",
  "weapon_category": "martial melee",
  "price": {
    "value": 25,
    "unit": "gp"
  },
  "mass": {
    "value": 3.0,
    "unit": "lb"
  },
  "category": "weapon"
}
