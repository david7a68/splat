---
splat_schema: 1
splat_id: d2f84f14-27ba-52fa-a151-bf2c2d7a2c31
splat_kind: rule
splat_key: dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon
splat_name: 'Level 3: Sacred Weapon'
splat_origin:
  key: dnd2024:reference/pages/056
splat_sources:
- title: SRD 5.2.1
  locator: pp. 56
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/oath_of_devotion_level_3_sacred_weapon
  parameters:
    trigger: actor.attack_action
    optional: true
    resource_cost:
      id: paladin.channel_divinity
      amount: 1
    target_item:
      type: melee_weapon
      held_by_actor: true
      count: 1
    duration:
      minutes: 10
    ends_on:
    - feature.used_again
    - weapon.not_carried_by_actor
    - actor.voluntary_end_no_action
    attack_bonus:
      actor_only: true
      weapon: selected
      amount:
        op: max
        args:
        - 1
        - var: actor.abilities.charisma.modifier
    hit_damage_type_choice:
    - normal
    - radiant
    light:
      bright_radius_feet: 20
      additional_dim_radius_feet: 20
    class_scope:
      class: paladin
      minimum_level: 3
  implementation: dnd2024:class_features_oath_of_devotion_level_3_sacred_weapon
splat_contracts:
- id: dnd2024:class_features_oath_of_devotion_level_3_sacred_weapon
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - action.attack
  - attack.roll
  - attack.hit
  - item.carrying_changed
  capabilities:
  - character_features
  - resources
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 56
  engine_gaps: &id001
  - dnd2024:class_features_oath_of_devotion_level_3_sacred_weapon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 35e94194a4abc0c5273348b31ff287ffe85d27b53bc2d5454ab65c17858c968d
  parent_name: Oath of Devotion
  line_span:
    first: 4749
    count: 14
splat_clauses:
- key: source_rules
  text: '# Level 3: Sacred Weapon When you take the Attack action, you can expend one use of your Channel
    Divinity to imbue one Melee weapon that you are holding with positive energy. For 10 minutes or until
    you use this feature again, you add your Charisma modifier to attack rolls you make with that weapon
    (minimum bonus of +1), and each time you hit with it, you cause it to deal its normal damage type
    or Radiant damage. The weapon also emits Bright Light in a 20-foot radius and Dim Light 20 feet beyond
    that. You can end this effect early (no action required). This effect also ends if you aren’t carrying
    the weapon.'
  sha256: a5934ca6f5d2c8c368144e3e4eaf9b4c39f545ac0b83bc3c1af2386bdda4833e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Sacred Weapon

<!-- SRD page 56 -->

When you take the Attack action, you can expend\
one use of your Channel Divinity to imbue one\
Melee weapon that you are holding with positive\
energy. For 10 minutes or until you use this feature\
again, you add your Charisma modifier to attack\
rolls you make with that weapon (minimum bonus\
of +1), and each time you hit with it, you cause it to\
deal its normal damage type or Radiant damage.\
The weapon also emits Bright Light in a 20-foot\
radius and Dim Light 20 feet beyond that.\
You can end this effect early (no action required).\
This effect also ends if you aren’t carrying the\
weapon.\
