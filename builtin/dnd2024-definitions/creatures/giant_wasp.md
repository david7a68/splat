---
splat_schema: 1
splat_id: 31472007-19a6-5177-82fe-878e11e1740f
splat_kind: creature
splat_key: dnd2024:creatures/giant_wasp
splat_name: Giant Wasp
splat_origin:
  key: dnd2024:reference/creature/giant_wasp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 354, 355
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 10 ft., Fly 50 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 22 (5d8)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 13
  max_hp:
    value: 22
  initiative:
    value: 2
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_wasp
  implementation: dnd2024:creature_giant_wasp
  parameters:
    features:
      flyby:
        economy: passive
        opportunity_attack_immunity:
          trigger: fly_out_of_enemy_reach
      sting:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 6
            add: 2
          damage_type: piercing
        - op: damage
          average: 5
          amount:
            dice:
              count: 2
              sides: 4
            add: 0
          damage_type: poison
        reach_feet: 5
    defenses: {}
    senses:
      passive_perception: 10
      special: []
    communication:
      speaks: []
      understands: []
    stat_block_conventions:
      attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
      save_dc: supplied_total
      damage_mode_choice:
      - fixed_average
      - rolled_dice
      condition_source: self
      multiattack_child_economy_cost: 0
      multiattack_child_resource_costs_retained: true
      spellcasting:
        default_level: spell_minimum
        spell_slots_expended: false
        required_materials: available
        casting_time_at_least_one_minute:
          uses_normal_casting_duration: true
          magic_action_each_turn: true
          concentration_required: true
      geometry_assumptions:
        chooser: gm
        supplies:
        - range
        - visibility
        - line_of_sight
        - area_membership
        - reachable_locations
        - unoccupied_locations
        - terrain
        - plane
        - form_appearance
splat_contracts:
- id: dnd2024:creature_giant_wasp
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events:
  - fly_out_of_enemy_reach
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 2d90678348039425c109a2c3a4f4228d68ed7e12bffe58a7cb2f8dcb794a7583
  source_path: builtin/srd-5.2.1/creature/giant_wasp.md
  pages:
  - 354
  - 355
  engine_gaps: &id001
  - dnd2024:creature_giant_wasp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: dfbf9811aec1e0d787be4b9fe1fff2c2fb362e7eaca449e127ae129b1fd6a20e
splat_clauses:
- key: source_rules
  text: '# Giant Wasp Medium Beast, Unaligned AC 13 Initiative +2 (12) HP 22 (5d8) Speed 10 ft., Fly 50
    ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 14 +2 +2 Con 10 +0 +0 Int 1 -5 -5 Wis 10 +0 +0 Cha
    3 -4 -4 Senses Passive Perception 10 Languages None CR 1/2 (XP 100; PB +2) ### Traits Flyby. The wasp
    doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach. ### Actions Sting. Melee
    Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage plus 5 (2d4) Poison damage.'
  sha256: e2c523f4c3ae309b6b887557ecfdba56e89cd80b3a2302cba741e44efa0a61fa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Wasp

<!-- SRD page 354 -->

Medium Beast, Unaligned\
AC 13  Initiative +2 (12)\
HP 22 (5d8)\
Speed 10 ft., Fly 50 ft.\

<!-- SRD page 355 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 14 +2 +2 Con 10 +0 +0\
Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
Senses Passive Perception 10\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Flyby. The wasp doesn’t provoke an Opportunity At-\
tack when it flies out of an enemy’s reach.\

### Actions

Sting. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage plus 5 (2d4) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/354|page 354]], [[../../srd-5.2.1/pages/355|page 355]].
