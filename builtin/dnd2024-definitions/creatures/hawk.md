---
splat_schema: 1
splat_id: 5a484d75-2077-5cd5-a2b5-e9a89e725e0e
splat_kind: creature
splat_key: dnd2024:creatures/hawk
splat_name: Hawk
splat_origin:
  key: dnd2024:reference/creature/hawk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 10 ft., Fly 60 ft.
  skills_text: Perception +6
  senses_text: Passive Perception 16
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 5
  save_strength:
    value: -3
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 1
  initiative:
    value: 3
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hawk
  implementation: dnd2024:creature_hawk
  parameters:
    features:
      talons:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 1
          amount:
            constant: 1
          damage_type: slashing
        reach_feet: 5
    skill_totals:
      perception: 6
    defenses: {}
    senses:
      passive_perception: 16
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
- id: dnd2024:creature_hawk
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
  events: []
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 3e840c22b144f2c12bfd6da711c3cb0cff2485fc308011ec6d5d4e0ab9a15482
  source_path: builtin/srd-5.2.1/creature/hawk.md
  pages:
  - 355
  engine_gaps: &id001
  - dnd2024:creature_hawk
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/hawk
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: b72dbf3df46660f8edcf2df1089b1b3e363bf629856d9c3c70678972facb9db7
splat_clauses:
- key: source_rules
  text: '# Hawk Tiny Beast, Unaligned AC 13 Initiative +3 (13) HP 1 (1d4 - 1) Speed 10 ft., Fly 60 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 5 -3 -3 Dex 16 +3 +3 Con 8 -1 -1 Int 2 -4 -4 Wis 14 +2 +2 Cha 6 -2
    -2 Skills Perception +6 Senses Passive Perception 16 Languages None CR 0 (XP 10; PB +2) ### Actions
    Talons. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Slashing damage.'
  sha256: ce1343c36aca2616015bedce9cfba57da0f21b56f2373f05f7623f5c3fece10c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hawk

<!-- SRD page 355 -->

Tiny Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 1 (1d4 − 1)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex 16 +3 +3 Con 8 −1 −1\
Int 2 −4 −4 Wis 14 +2 +2 Cha 6 −2 −2\
Skills Perception +6\
Senses Passive Perception 16\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Talons. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Slash-\
ing damage.\

Original layout: [[../../srd-5.2.1/pages/355|page 355]].
