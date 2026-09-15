---
splat_schema: 1
splat_id: 3129002f-0d89-51a8-a1d5-3774edd45fbc
splat_kind: creature
splat_key: dnd2024:creatures/young_brass_dragon
splat_name: Young Brass Dragon
splat_origin:
  key: dnd2024:reference/creature/young_brass_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 268
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 40 ft., Burrow 20 ft., Fly 80 ft.
  skills_text: Perception +6, Persuasion +5, Stealth +3
  immunities_text: Fire
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16
  languages_text: Common, Draconic
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 110 (13d10 + 39)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 3
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
    value: 3
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 17
  max_hp:
    value: 110
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 20
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/young_brass_dragon
  implementation: dnd2024:creature_young_brass_dragon
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
        - replace_count: 2
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: sleep_breath
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 7
          reach_ft: 10
        hit:
          damage:
          - average: 15
            roll:
              count: 2
              sides: 10
              modifier: 4
            type: slashing
      fire_breath:
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
        ability: dexterity
        dc: 14
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 40
            origin: self
            width_ft: 5
        failure:
          damage:
          - average: 38
            roll:
              count: 11
              sides: 6
              modifier: 0
            type: fire
        success:
          damage_multiplier: 0.5
          other_effects: false
      sleep_breath:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 14
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 30
            origin: self
        failure:
          conditions:
          - condition: incapacitated
            until: target.next_turn_end
          repeat_save:
            event: target.next_turn_end
            failure:
              conditions:
              - condition: unconscious
                duration_seconds: 60
              ends_on:
              - event: target.damage_taken
              - event: creature.action
                operation: wake_target
                within_ft: 5
            success: end_effect
        success:
          effects: []
    skill_totals:
      perception: 6
      persuasion: 5
      stealth: 3
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
      blindsight: 30
      darkvision: 120
      passive_perception: 16
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
- id: dnd2024:creature_young_brass_dragon
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
  source_sha256: bb41ae63a1889e1419860eb4eaaf8909e74beb8bc6c679d593b7da83deb883e4
  source_path: builtin/srd-5.2.1/creature/young_brass_dragon.md
  pages:
  - 268
  engine_gaps: &id001
  - dnd2024:creature_young_brass_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: de42e99b44a520b7cffced7543e28ea2215657b6e529458d55929d4d9e2627d3
splat_clauses:
- key: source_rules
  text: '# Young Brass Dragon Large Dragon (Metallic), Chaotic Good AC 17 Initiative +3 (13) HP 110 (13d10
    + 39) Speed 40 ft., Burrow 20 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 10 +0 +3
    Con 17 +3 +3 Int 12 +1 +1 Wis 11 +0 +3 Cha 15 +2 +2 Skills Perception +6, Persuasion +5, Stealth +3
    Immunities Fire Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16 Languages Common,
    Draconic CR 6 (XP 2,300; PB +3) ### Actions Multiattack. The dragon makes three Rend attacks. It can
    replace two attacks with a use of Sleep Breath. Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 15
    (2d10 + 4) Slashing damage. Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 14, each creature
    in a 40-foot-long, 5-footwide Line. Failure: 38 (11d6) Fire damage. Success: Half damage. Sleep Breath.
    Constitution Saving Throw: DC 14, each creature in a 30-foot Cone. Failure: The target has the Incapacitated
    condition until the end of its next turn, at which point it repeats the save. Second Failure: The
    target has the Unconscious condition for 1 minute. This effect ends for the target if it takes damage
    or a creature within 5 feet of it takes an action to wake it.'
  sha256: 044e94783bb84aa6b71926cb6ae3efbbf1fa9616c11aa2919a1c756fd3e73c46
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Brass Dragon

<!-- SRD page 268 -->

Large Dragon (Metallic), Chaotic Good\
AC 17 Initiative +3 (13)\
HP 110 (13d10 + 39)\
Speed 40 ft., Burrow 20 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 10 +0 +3 Con 17 +3 +3\
Int 12 +1 +1 Wis 11 +0 +3 Cha 15 +2 +2\
Skills Perception +6, Persuasion +5, Stealth +3\
Immunities Fire\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 16\
Languages Common, Draconic\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace two attacks with a use of Sleep Breath.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 15 (2d10\
+ 4) Slashing damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 14, each creature in a 40-foot-long, 5-foot-\
wide Line. Failure: 38 (11d6) Fire damage. Success:\
Half damage.\
Sleep Breath. Constitution Saving Throw: DC 14, each\
creature in a 30-foot Cone. Failure: The target has the\
Incapacitated condition until the end of its next turn,\
at which point it repeats the save. Second Failure: The\
target has the Unconscious condition for 1 minute. This\
effect ends for the target if it takes damage or a creature\
within 5 feet of it takes an action to wake it.\

Original layout: [[../../srd-5.2.1/pages/268|page 268]].
