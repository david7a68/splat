---
splat_schema: 1
splat_id: 0bef5350-ad14-5b5c-95a7-be01b33718aa
splat_kind: creature
splat_key: dnd2024:creatures/giant_eagle
splat_name: Giant Eagle
splat_origin:
  key: dnd2024:reference/creature/giant_eagle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 350, 351
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Neutral Good
  movement_text: 10 ft., Fly 80 ft.
  skills_text: Perception +6
  resistances_text: Necrotic, Radiant
  senses_text: Passive Perception 16
  languages_text: Celestial; understands Common and Primordial (Auran) but can’t speak them
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 26 (4d10 + 4)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 26
  initiative:
    value: 3
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_eagle
  implementation: dnd2024:creature_giant_eagle
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: rend
          count: 2
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 4
              modifier: 3
            type: slashing
          - average: 3
            roll:
              count: 1
              sides: 6
              modifier: 0
            type: radiant
    skill_totals:
      perception: 6
    damage_resistances:
    - necrotic
    - radiant
    senses:
      passive_perception: 16
    languages:
      speaks:
      - celestial
      understands:
      - common
      - language: primordial
        dialects:
        - auran
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_giant_eagle
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 3a3e23160ca8368b2164750e7536cce5fb0ace4b1486bfd005779c0d12bf7ae0
  source_path: builtin/srd-5.2.1/creature/giant_eagle.md
  pages:
  - 350
  - 351
  engine_gaps: &id001
  - dnd2024:creature_giant_eagle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 37e0170b9811928ce1d97641665ccb016f6b2005912d372aeb444bcef719cc54
splat_clauses:
- key: source_rules
  text: '# Giant Eagle Large Celestial, Neutral Good AC 13 Initiative +3 (13) HP 26 (4d10 + 4) Speed 10
    ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 17 +3 +3 Con 13 +1 +1 Int 8 -1 -1 Wis
    14 +2 +2 Cha 10 +0 +0 Skills Perception +6 Resistances Necrotic, Radiant Senses Passive Perception
    16 Languages Celestial; understands Common and Primordial (Auran) but can’t speak them CR 1 (XP 200;
    PB +2) ### Actions Multiattack. The eagle makes two Rend attacks. Rend. Melee Attack Roll: +5, reach
    5 ft. Hit: 5 (1d4 + 3) Slashing damage plus 3 (1d6) Radiant damage.'
  sha256: de5d6f8651f19841386b8218a64561dcf953fbba3f859784f6b11631ec97ddda
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Eagle

<!-- SRD page 350 -->

Large Celestial, Neutral Good\
AC 13 Initiative +3 (13)\
HP 26 (4d10 + 4)\
Speed 10 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 17 +3 +3 Con 13 +1 +1\
Int 8 −1 −1 Wis 14 +2 +2 Cha 10 +0 +0\
Skills Perception +6\
Resistances Necrotic, Radiant\
Senses Passive Perception 16\
Languages Celestial; understands Common and Primordial\
(Auran) but can’t speak them\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The eagle makes two Rend attacks.\

<!-- SRD page 351 -->

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Slashing damage plus 3 (1d6) Radiant damage.\

Original layout: [[../../srd-5.2.1/pages/350|page 350]], [[../../srd-5.2.1/pages/351|page 351]].
