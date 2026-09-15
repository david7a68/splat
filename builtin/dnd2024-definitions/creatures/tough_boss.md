---
splat_schema: 1
splat_id: e046c72b-0474-5e7d-abb7-02703b4ac38c
splat_kind: creature
splat_key: dnd2024:creatures/tough_boss
splat_name: Tough Boss
splat_origin:
  key: dnd2024:reference/creature/tough_boss
splat_sources:
- title: SRD 5.2.1
  locator: pp. 332
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  gear_text: Chain Mail, Heavy Crossbow, Warhammer
  senses_text: Passive Perception 10
  languages_text: Common plus one other language
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 82 (11d8 + 33)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 5
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 5
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 11
  save_charisma:
    value: 2
  armor_class:
    value: 16
  max_hp:
    value: 82
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/tough_boss
  implementation: dnd2024:creature_tough_boss
  parameters:
    features:
      pack_tactics:
        economy: trait
        operation: passive_rule
        attack_advantage:
          target_kind: creature
          ally_within_target_ft: 5
          ally_excludes_conditions:
          - incapacitated
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - warhammer
          - heavy_crossbow
      warhammer:
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
          - average: 12
            roll:
              count: 2
              sides: 8
              modifier: 3
            type: bludgeoning
          push:
            maximum_ft: 10
            direction: straight_away_from_self
            maximum_size: large
      heavy_crossbow:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          range_ft: 100
          long_range_ft: 400
        hit:
          damage:
          - average: 13
            roll:
              count: 2
              sides: 10
              modifier: 2
            type: piercing
    gear:
    - item: chain_mail
      quantity: 1
    - item: heavy_crossbow
      quantity: 1
    - item: warhammer
      quantity: 1
    senses:
      passive_perception: 10
    languages:
      speaks:
      - common
      understands: []
      additional_languages:
        count: 1
        chooser: creator
        can_speak: true
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
- id: dnd2024:creature_tough_boss
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
  source_sha256: ceb9e444575f2604f10bea544783eb96bf6a9a65255b3debd19fcf16e1c0763f
  source_path: builtin/srd-5.2.1/creature/tough_boss.md
  pages:
  - 332
  engine_gaps: &id001
  - dnd2024:creature_tough_boss
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: a0ebf034677da64f365a1eb9dff591fb0cdc7e0149622338194119703d2377b1
splat_clauses:
- key: source_rules
  text: '# Tough Boss Medium or Small Humanoid, Neutral AC 16 Initiative +2 (12) HP 82 (11d8 + 33) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +5 Dex 14 +2 +2 Con16 +3 +5 Int 11 +0 +0 Wis 10 +0 +0
    Cha 11 +0 +2 Gear Chain Mail, Heavy Crossbow, Warhammer Senses Passive Perception 10 Languages Common
    plus one other language CR 4 (XP 1,100; PB +2) ### Traits Pack Tactics. The tough has Advantage on
    an attack roll against a creature if at least one of the tough’s allies is within 5 feet of the creature
    and the ally doesn’t have the Incapacitated condition. ### Actions Multiattack. The tough makes two
    attacks, using Warhammer or Heavy Crossbow in any combination. Warhammer. Melee Attack Roll: +5, reach
    5 ft. Hit: 12 (2d8 + 3) Bludgeoning damage. If the target is a Large or smaller creature, the tough
    pushes the target up to 10 feet straight away from itself. Heavy Crossbow. Ranged Attack Roll: +4,
    range 100/400 ft. Hit: 13 (2d10 + 2) Piercing damage.'
  sha256: e06d3670b269fef9a5745142892c90b8040ba2ea973929ce5ac134aa2a54b28b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tough Boss

<!-- SRD page 332 -->

Medium or Small Humanoid, Neutral\
AC 16  Initiative +2 (12)\
HP 82 (11d8 + 33)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +5 Dex 14 +2 +2 Con16 +3 +5\
Int 11 +0 +0 Wis 10 +0 +0 Cha 11 +0 +2\
Gear Chain Mail, Heavy Crossbow, Warhammer\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 4 (XP 1,100; PB +2)\

### Traits

Pack Tactics. The tough has Advantage on an attack\
roll against a creature if at least one of the tough’s allies\
is within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Multiattack. The tough makes two attacks, using War-\
hammer or Heavy Crossbow in any combination.\
Warhammer. Melee Attack Roll: +5, reach 5 ft. Hit: 12\
(2d8 + 3) Bludgeoning damage. If the target is a Large\
or smaller creature, the tough pushes the target up to 10\
feet straight away from itself.\
Heavy Crossbow. Ranged Attack Roll: +4, range\
100/400 ft. Hit: 13 (2d10 + 2) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/332|page 332]].
