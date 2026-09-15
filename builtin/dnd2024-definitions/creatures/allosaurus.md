---
splat_schema: 1
splat_id: 6ff31d77-aeac-5666-8066-a74ed07ab885
splat_kind: creature
splat_key: dnd2024:creatures/allosaurus
splat_name: Allosaurus
splat_origin:
  key: dnd2024:reference/creature/allosaurus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  tags_text: (Dinosaur)
  movement_text: 60 ft.
  skills_text: Perception +5
  senses_text: Passive Perception 15
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 51 (6d10 + 18)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 51
  initiative:
    value: 1
  speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/allosaurus
  implementation: dnd2024:creature_allosaurus
  parameters:
    features:
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 15
            roll:
              count: 2
              sides: 10
              modifier: 4
            type: piercing
      claws:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 8
            roll:
              count: 1
              sides: 8
              modifier: 4
            type: slashing
          prone:
            condition: prone
            maximum_size: large
            requires:
              immediately_preceding_straight_movement_toward_target_ft: 30
          additional_attack:
            feature: bite
            target: same
            requires:
              maximum_target_size: large
              immediately_preceding_straight_movement_toward_target_ft: 30
            optional: true
    skill_totals:
      perception: 5
    senses:
      passive_perception: 15
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
- id: dnd2024:creature_allosaurus
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
  - conditions
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: b0fb58feac0002fe12a0422cbf890afddab05e6672ae3be764d346156097cf89
  source_path: builtin/srd-5.2.1/creature/allosaurus.md
  pages:
  - 344
  engine_gaps: &id001
  - dnd2024:creature_allosaurus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: a7a8be9dca269c265b3edb0cb8d8f7d6c4572696c35ab98935541a2369523f5e
splat_clauses:
- key: source_rules
  text: '# Allosaurus Large Beast (Dinosaur), Unaligned AC 13 Initiative +1 (11) HP 51 (6d10 + 18) Speed
    60 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 13 +1 +1 Con 17 +3 +3 Int 2 -4 -4 Wis 12 +1 +1
    Cha 5 -3 -3 Skills Perception +5 Senses Passive Perception 15 Languages None CR 2 (XP 450; PB +2)
    ### Actions Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 15 (2d10 + 4) Piercing damage. Claws. Melee
    Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 +4) Slashing damage. If the target is a Large or smaller
    creature and the allosaurus moved 30+ feet straight toward it immediately before the hit, the target
    has the Prone condition, and the allosaurus can make one Bite attack against it.'
  sha256: ed53c69800d1e061435cdd468d14a437ccba1e532ca8eba4c83d3c979ffa2a03
  status: contract_specified
splat_requires_mechanics: *id001
---

# Allosaurus

<!-- SRD page 344 -->

Large Beast (Dinosaur), Unaligned\
AC 13  Initiative +1 (11)\
HP 51 (6d10 + 18)\
Speed 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 13 +1 +1 Con 17 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +5\
Senses Passive Perception 15\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 15 (2d10 +\
4) Piercing damage.\
Claws. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8\
+4) Slashing damage. If the target is a Large or smaller\
creature and the allosaurus moved 30+ feet straight\
toward it immediately before the hit, the target has the\
Prone condition, and the allosaurus can make one Bite\
attack against it.\

Original layout: [[../../srd-5.2.1/pages/344|page 344]].
