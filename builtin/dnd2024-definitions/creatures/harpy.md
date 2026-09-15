---
splat_schema: 1
splat_id: 3ddc1ac6-e9a6-55e1-86b8-acebc64ab05e
splat_kind: creature
splat_key: dnd2024:creatures/harpy
splat_name: Harpy
splat_origin:
  key: dnd2024:reference/creature/harpy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 297
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Chaotic Evil
  movement_text: 20 ft., Fly 40 ft.
  senses_text: Passive Perception 10
  languages_text: Common
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 38 (7d8 + 7)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 11
  max_hp:
    value: 38
  initiative:
    value: 1
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/harpy
  implementation: dnd2024:creature_harpy
  parameters:
    features:
      claw:
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
          - average: 6
            roll:
              count: 2
              sides: 4
              modifier: 1
            type: slashing
      luring_song:
        economy: action
        operation: saving_throw
        ability: wisdom
        dc: 11
        targets:
          kind: creature
          types:
          - humanoid
          - giant
          selection: each
          area:
            shape: emanation
            size_ft: 300
            origin: self
          trigger: song.starts
        failure:
          conditions:
          - condition: charmed
            until: song.ends
          repeat_save:
            events:
            - target.turn_end
            - before_target_enters_damaging_terrain
            - event: target.damage_taken
              source_excludes: self
            success: end_effect_and_grant_immunity
          while_charmed:
            conditions:
            - incapacitated
            ignore_other_harpy_luring_songs: true
            movement:
              event: target.turn
              if_distance_greater_ft: 5
              destination: within_5ft_of_self
              route: most_direct
              avoid_opportunity_attacks: false
        concentration: true
        magical: true
        success:
          immunity:
            effect: this_harpy_luring_song
            duration_seconds: 86400
    senses:
      passive_perception: 10
    languages:
      speaks:
      - common
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
- id: dnd2024:creature_harpy
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
  - damage.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: dbee13ac9f0e5de071a14741412bb5b46a0057d1104287ec7b9ea8b9e38aec90
  source_path: builtin/srd-5.2.1/creature/harpy.md
  pages:
  - 297
  engine_gaps: &id001
  - dnd2024:creature_harpy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 8fe990898ef35a5dfcd4b1552b11b2a728b35a90a5c1b5f340f471c8b0ff20a4
splat_clauses:
- key: source_rules
  text: '# Harpy Medium Monstrosity, Chaotic Evil AC 11 Initiative +1 (11) HP 38 (7d8 + 7) Speed 20 ft.,
    Fly 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 13 +1 +1 Con 12 +1 +1 Int 7 -2 -2 Wis 10 +0
    +0 Cha 13 +1 +1 Senses Passive Perception 10 Languages Common CR 1 (XP 200; PB +2) ### Actions Claw.
    Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1) Slashing damage. Luring Song. The harpy sings
    a magical melody, which lasts until the harpy’s Concentration ends on it. Wisdom Saving Throw: DC
    11, each Humanoid and Giant in a 300-foot Emanation originating from the harpy when the song starts.
    Failure: The target has the Charmed condition until the song ends and repeats the save at the end
    of each of its turns. While Charmed, the target has the Incapacitated condition and ignores the Luring
    Song of other harpies. If the target is more than 5 feet from the harpy, the target moves on its turn
    toward the harpy by the most direct route, trying to get within 5 feet of the harpy. It doesn’t avoid
    Opportunity Attacks; however, before moving into damaging terrain (such as lava or a pit) and whenever
    it takes damage from a source other than the harpy, the target repeats the save. Success: The target
    is immune to this harpy’s Luring Song for 24 hours.'
  sha256: 99b75f5e99b7df791f134cf3f7c97b0f0d038a4d0da72701db982b749b0e8096
  status: contract_specified
splat_requires_mechanics: *id001
---

# Harpy

<!-- SRD page 297 -->

Medium Monstrosity, Chaotic Evil\
AC 11 Initiative +1 (11)\
HP 38 (7d8 + 7)\
Speed 20 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 13 +1 +1 Con 12 +1 +1\
Int 7 −2 −2 Wis 10 +0 +0 Cha 13 +1 +1\
Senses Passive Perception 10\
Languages Common\
CR 1 (XP 200; PB +2)\

### Actions

Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1)\
Slashing damage.\
Luring Song. The harpy sings a magical melody, which\
lasts until the harpy’s Concentration ends on it. Wisdom\
Saving Throw: DC 11, each Humanoid and Giant in a\
300-foot Emanation originating from the harpy when\
the song starts. Failure: The target has the Charmed\
condition until the song ends and repeats the save at\
the end of each of its turns. While Charmed, the target\
has the Incapacitated condition and ignores the Luring\
Song of other harpies. If the target is more than 5 feet\
from the harpy, the target moves on its turn toward the\
harpy by the most direct route, trying to get within 5\
feet of the harpy. It doesn’t avoid Opportunity Attacks;\
however, before moving into damaging terrain (such\
as lava or a pit) and whenever it takes damage from a\
source other than the harpy, the target repeats the save.\
Success: The target is immune to this harpy’s Luring\
Song for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/297|page 297]].
