---
splat_schema: 1
splat_id: 461b291f-a76e-5371-97d7-09bcfdf5fbf5
splat_kind: creature
splat_key: dnd2024:creatures/young_silver_dragon
splat_name: Young Silver Dragon
splat_origin:
  key: dnd2024:reference/creature/young_silver_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 324
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 40 ft., Fly 80 ft.
  skills_text: History +6, Perception +8, Stealth +4
  immunities_text: Cold
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 18
  languages_text: Common, Draconic
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 168 (16d10 + 80)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 10
  save_dexterity:
    value: 4
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 11
  save_wisdom:
    value: 4
  charisma:
    value: 19
  save_charisma:
    value: 4
  armor_class:
    value: 18
  max_hp:
    value: 168
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/young_silver_dragon
  implementation: dnd2024:creature_young_silver_dragon
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: rend
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: paralyzing_breath
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 10
          reach_ft: 10
        hit:
          damage:
          - average: 15
            roll:
              count: 2
              sides: 8
              modifier: 6
            type: slashing
      cold_breath:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: constitution
        dc: 17
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 30
            origin: self
        failure:
          damage:
          - average: 49
            roll:
              count: 11
              sides: 8
              modifier: 0
            type: cold
        success:
          damage_multiplier: 0.5
          other_effects: false
      paralyzing_breath:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 17
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 30
            origin: self
        failure:
          damage: []
          conditions:
          - condition: incapacitated
            until: target.next_turn_end
          repeat_save:
            event: target.next_turn_end
            failure:
              conditions:
              - condition: paralyzed
              repeat_save:
                event: target.turn_end
                success: end_effect
              automatic_success_after_seconds: 60
              automatic_success_clock_origin: initial_failed_save
            success: end_effect
        success:
          effects: []
    skill_totals:
      history: 6
      perception: 8
      stealth: 4
    damage_immunities:
    - cold
    condition_immunities: []
    senses:
      blindsight: 30
      darkvision: 120
      passive_perception: 18
    languages:
      speaks:
      - common
      - draconic
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
- id: dnd2024:creature_young_silver_dragon
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
  - turn.started
  - turn.ended
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
  source_sha256: 1803a8fe6747f48f65201d02c7847467448482cdf70819a9327de26376025239
  source_path: builtin/srd-5.2.1/creature/young_silver_dragon.md
  pages:
  - 324
  engine_gaps: &id001
  - dnd2024:creature_young_silver_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: ca8a9d601b18a6ce9778d6c335131bb3ff65d242eff3b7134be478887c24d75e
splat_clauses:
- key: source_rules
  text: '# Young Silver Dragon Large Dragon (Metallic), Lawful Good AC 18 Initiative +4 (14) HP 168 (16d10
    + 80) Speed 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 10 +0 +4 Con 21 +5 +5 Int
    14 +2 +2 Wis 11 +0 +4 Cha 19 +4 +4 Skills History +6, Perception +8, Stealth +4 Immunities Cold Senses
    Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 18 Languages Common, Draconic CR 9 (XP 5,000;
    PB +4) ### Actions Multiattack. The dragon makes three Rend attacks. It can replace one attack with
    a use of Paralyzing Breath. Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (2d8 + 6) Slashing
    damage. Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 17, each creature in a 30-foot Cone.
    Failure: 49 (11d8) Cold damage. Success: Half damage. Paralyzing Breath. Constitution Saving Throw:
    DC 17, each creature in a 30-foot Cone. First Failure: The target has the Incapacitated condition
    until the end of its next turn, when it repeats the save. Second Failure: The target has the Paralyzed
    condition, and it repeats the save at the end of each of its turns, ending the effect on itself on
    a success. After 1 minute, it succeeds automatically.'
  sha256: eab8b093aabd5b81b65b5fa1227394005de7aa18b1a8d2d33fe3bcd6a9dee4ee
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Silver Dragon

<!-- SRD page 324 -->

Large Dragon (Metallic), Lawful Good\
AC 18 Initiative +4 (14)\
HP 168 (16d10 + 80)\
Speed 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 10 +0 +4 Con 21 +5 +5\
Int 14 +2 +2 Wis 11 +0 +4 Cha 19 +4 +4\
Skills History +6, Perception +8, Stealth +4\
Immunities Cold\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 18\
Languages Common, Draconic\
CR 9 (XP 5,000; PB +4)\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Paralyzing Breath.\
Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 15 (2d8\
+ 6) Slashing damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 17, each creature in a 30-foot Cone. Failure:\
49 (11d8) Cold damage. Success: Half damage.\
Paralyzing Breath. Constitution Saving Throw: DC 17,\
each creature in a 30-foot Cone. First Failure: The target\
has the Incapacitated condition until the end of its next\
turn, when it repeats the save. Second Failure: The tar-\
get has the Paralyzed condition, and it repeats the save\
at the end of each of its turns, ending the effect on itself\
on a success. After 1 minute, it succeeds automatically.\

Original layout: [[../../srd-5.2.1/pages/324|page 324]].
