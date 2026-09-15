---
splat_schema: 1
splat_id: d9dba9af-35dc-5cab-b2a5-997303b74c58
splat_kind: creature
splat_key: dnd2024:creatures/frog
splat_name: Frog
splat_origin:
  key: dnd2024:reference/creature/frog
splat_sources:
- title: SRD 5.2.1
  locator: pp. 348, 349
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 20 ft., Swim 20 ft.
  skills_text: Perception +1, Stealth +3
  senses_text: Darkvision 30 ft.; Passive Perception 11
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 1
  save_strength:
    value: -5
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 11
  max_hp:
    value: 1
  initiative:
    value: 1
  speed:
    value: 20
    unit: ft
  swim_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/frog
  implementation: dnd2024:creature_frog
  parameters:
    features:
      amphibious:
        economy: trait
        operation: passive_rule
        breathing:
        - air
        - water
      standing_leap:
        economy: trait
        operation: passive_rule
        long_jump_ft: 10
        high_jump_ft: 5
        running_start_required: false
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 3
          reach_ft: 5
        hit:
          damage:
          - average: 1
            roll:
              constant: 1
            type: piercing
    skill_totals:
      perception: 1
      stealth: 3
    senses:
      darkvision: 30
      passive_perception: 11
    languages:
      speaks: []
      understands: []
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
- id: dnd2024:creature_frog
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
  source_sha256: 0818cf624468cca969f209f844fa0bfbefc8b900e3aa31ad46c894d7736d34f3
  source_path: builtin/srd-5.2.1/creature/frog.md
  pages:
  - 348
  - 349
  engine_gaps: &id001
  - dnd2024:creature_frog
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 10e6828d781d283f69072d2b533c70e82149a45498db02621278826c4aa18df9
splat_clauses:
- key: source_rules
  text: '# Frog Tiny Beast, Unaligned AC 11 Initiative +1 (11) HP 1 (1d4 - 1) Speed 20 ft., Swim 20 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 1 -5 -5 Dex 13 +1 +1 Con 8 -1 -1 Int 1 -5 -5 Wis 8 -1 -1 Cha 3 -4 -4
    Skills Perception +1, Stealth +3 Senses Darkvision 30 ft.; Passive Perception 11 Languages None CR
    0 (XP 10; PB +2) ### Traits Amphibious. The frog can breathe air and water. Standing Leap. The frog’s
    Long Jump is up to 10 feet and its High Jump is up to 5 feet with or without a running start. ###
    Actions Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 1 Piercing damage.'
  sha256: 6915d7096f81b0292ddc02488d09c07aa7f3be1069030025b91c0c8735fc8efb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Frog

<!-- SRD page 348 -->

Tiny Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 1 (1d4 − 1)\
Speed 20 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 13 +1 +1 Con 8 −1 −1\
Int 1 −5 −5 Wis 8 −1 −1 Cha 3 −4 −4\
Skills Perception +1, Stealth +3\
Senses Darkvision 30 ft.; Passive Perception 11\
Languages None\
CR 0 (XP 10; PB +2)\

<!-- SRD page 349 -->


### Traits

Amphibious. The frog can breathe air and water.\
Standing Leap. The frog’s Long Jump is up to 10 feet\
and its High Jump is up to 5 feet with or without a run-\
ning start.\

### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 1 Piercing\
damage.\

Original layout: [[../../srd-5.2.1/pages/348|page 348]], [[../../srd-5.2.1/pages/349|page 349]].
