---
splat_schema: 1
splat_id: da6a0d9a-9e54-5118-a7c0-0508b28c00fe
splat_kind: creature
splat_key: dnd2024:creatures/fire_giant
splat_name: Fire Giant
splat_origin:
  key: dnd2024:reference/creature/fire_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 285
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: giant
  alignment: Lawful Evil
  movement_text: 30 ft.
  skills_text: Athletics +11, Perception +6
  immunities_text: Fire
  senses_text: Passive Perception 16
  languages_text: Giant
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 162 (13d12 + 78)
splat_stats:
  strength:
    value: 25
  save_strength:
    value: 7
  dexterity:
    value: 9
  save_dexterity:
    value: 3
  constitution:
    value: 23
  save_constitution:
    value: 10
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 13
  save_charisma:
    value: 5
  armor_class:
    value: 18
  max_hp:
    value: 162
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/fire_giant
  implementation: dnd2024:creature_fire_giant
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - flame_sword
          - hammer_throw
      flame_sword:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 11
          reach_ft: 10
        hit:
          damage:
          - average: 21
            roll:
              count: 4
              sides: 6
              modifier: 7
            type: slashing
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: fire
      hammer_throw:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 11
          range_ft: 60
          long_range_ft: 240
        hit:
          damage:
          - average: 23
            roll:
              count: 3
              sides: 10
              modifier: 7
            type: bludgeoning
          - average: 4
            roll:
              count: 1
              sides: 8
              modifier: 0
            type: fire
          push:
            maximum_ft: 15
            direction: straight_away_from_self
          next_attack_modifier:
            disadvantage: true
            expires: target.next_turn_end
            uses: 1
    skill_totals:
      athletics: 11
      perception: 6
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
      passive_perception: 16
    languages:
      speaks:
      - giant
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
- id: dnd2024:creature_fire_giant
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
  - turn.ended
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6ab30769915a4cd685aaece62dce5e7fc18dd1831f4ec239d0ad115484a6d270
  source_path: builtin/srd-5.2.1/creature/fire_giant.md
  pages:
  - 285
  engine_gaps: &id001
  - dnd2024:creature_fire_giant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 5c3f7fef806717e6bc71621a5ac3deed111c05fbef61a3bcb581c99d8d2f9201
splat_clauses:
- key: source_rules
  text: '# Fire Giant Huge Giant, Lawful Evil AC 18 Initiative +3 (13) HP 162 (13d12 + 78) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 25 +7 +7 Dex 9 -1 +3 Con 23 +6 +10 Int 10 +0 +0 Wis 14 +2 +2 Cha 13
    +1 +5 Skills Athletics +11, Perception +6 Immunities Fire Senses Passive Perception 16 Languages Giant
    CR 9 (XP 5,000; PB +4) ### Actions Multiattack. The giant makes two attacks, using Flame Sword or
    Hammer Throw in any combination. Flame Sword. Melee Attack Roll: +11, reach 10 ft. Hit: 21 (4d6 +
    7) Slashing damage plus 10 (3d6) Fire damage. Hammer Throw. Ranged Attack Roll: +11, range 60/240
    ft. Hit: 23 (3d10 + 7) Bludgeoning damage plus 4 (1d8) Fire damage, and the target is pushed up to
    15 feet straight away from the giant and has Disadvantage on the next attack roll it makes before
    the end of its next turn.'
  sha256: c0312bb796586cfe5ef0c1ac515dfdd06d2d980d6d246d1d507d6febe2f1845a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fire Giant

<!-- SRD page 285 -->

Huge Giant, Lawful Evil\
AC 18 Initiative +3 (13)\
HP 162 (13d12 + 78)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 25 +7 +7 Dex 9 −1 +3 Con 23 +6 +10\
Int 10 +0 +0 Wis 14 +2 +2 Cha 13 +1 +5\
Skills Athletics +11, Perception +6\
Immunities Fire\
Senses Passive Perception 16\
Languages Giant\
CR 9 (XP 5,000; PB +4)\

### Actions

Multiattack. The giant makes two attacks, using Flame\
Sword or Hammer Throw in any combination.\
Flame Sword. Melee Attack Roll: +11, reach 10 ft.\
Hit: 21 (4d6 + 7) Slashing damage plus 10 (3d6)\
Fire damage.\
Hammer Throw. Ranged Attack Roll: +11, range 60/240\
ft. Hit: 23 (3d10 + 7) Bludgeoning damage plus 4 (1d8)\
Fire damage, and the target is pushed up to 15 feet\
straight away from the giant and has Disadvantage\
on the next attack roll it makes before the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/285|page 285]].
