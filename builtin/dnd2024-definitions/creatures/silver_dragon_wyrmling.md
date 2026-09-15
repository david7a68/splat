---
splat_schema: 1
splat_id: 5ec64fe8-06de-53db-b4e8-36cfd3017b8c
splat_kind: creature
splat_key: dnd2024:creatures/silver_dragon_wyrmling
splat_name: Silver Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/silver_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 324
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 30 ft., Fly 60 ft.
  skills_text: Perception +4, Stealth +2
  immunities_text: Cold
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 2 (450 XP; PB +2)
  hit_points_text: 45 (6d8 + 18)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 2
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 17
  max_hp:
    value: 45
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/silver_dragon_wyrmling
  implementation: dnd2024:creature_silver_dragon_wyrmling
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
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 9
            roll:
              count: 1
              sides: 10
              modifier: 4
            type: piercing
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
        dc: 13
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 15
            origin: self
        failure:
          damage:
          - average: 18
            roll:
              count: 4
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
        dc: 13
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 15
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
      perception: 4
      stealth: 2
    damage_immunities:
    - cold
    condition_immunities: []
    senses:
      blindsight: 10
      darkvision: 60
      passive_perception: 14
    languages:
      speaks:
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
- id: dnd2024:creature_silver_dragon_wyrmling
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
  source_sha256: 181d16bcd30c781daf40fbc7876225d76d675f80a4067b4c418469f1bcfdc3c3
  source_path: builtin/srd-5.2.1/creature/silver_dragon_wyrmling.md
  pages:
  - 324
  engine_gaps: &id001
  - dnd2024:creature_silver_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: f4c02b0ea5b3e36861b36b9cf97543f4e8c30a3e71abd760b3176edd9ab47a60
splat_clauses:
- key: source_rules
  text: '# Silver Dragon Wyrmling Medium Dragon (Metallic), Lawful Good AC 17 Initiative +2 (12) HP 45
    (6d8 + 18) Speed 30 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 10 +0 +2 Con 17 +3
    +3 Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2 Skills Perception +4, Stealth +2 Immunities Cold Senses
    Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic CR 2 (450 XP; PB +2)
    ### Actions Multiattack. The dragon makes two Rend attacks. Rend. Melee Attack Roll: +6, reach 5 ft.
    Hit: 9 (1d10 + 4) Piercing damage. Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 13, each
    creature in a 15-foot Cone. Failure: 18 (4d8) Cold damage. Success: Half damage. Paralyzing Breath.
    Constitution Saving Throw: DC 13, each creature in a 15-foot Cone. First Failure: The target has the
    Incapacitated condition until the end of its next turn, when it repeats the save. Second Failure:
    The target has the Paralyzed condition, and it repeats the save at the end of each of its turns, ending
    the effect on itself on a success. After 1 minute, it succeeds automatically.'
  sha256: 33764dfaa3eddc9c8e5fd0758c3d8785ca6777b635cb49788991f4cf9e2d4b8c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Silver Dragon Wyrmling

<!-- SRD page 324 -->

Medium Dragon (Metallic), Lawful Good\
AC 17 Initiative +2 (12)\
HP 45 (6d8 + 18)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 10 +0 +2 Con 17 +3 +3\
Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2\
Skills Perception +4, Stealth +2\
Immunities Cold\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 2 (450 XP; PB +2)\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (1d10\
+ 4) Piercing damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 13, each creature in a 15-foot Cone. Failure:\
18 (4d8) Cold damage. Success: Half damage.\
Paralyzing Breath. Constitution Saving Throw: DC 13,\
each creature in a 15-foot Cone. First Failure: The target\
has the Incapacitated condition until the end of its next\
turn, when it repeats the save. Second Failure: The tar-\
get has the Paralyzed condition, and it repeats the save\
at the end of each of its turns, ending the effect on itself\
on a success. After 1 minute, it succeeds automatically.\

Original layout: [[../../srd-5.2.1/pages/324|page 324]].
