---
splat_schema: 1
splat_id: 90504663-a874-50e5-9d69-049c43447321
splat_kind: creature
splat_key: dnd2024:creatures/brass_dragon_wyrmling
splat_name: Brass Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/brass_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 268
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 30 ft., Burrow 15 ft., Fly 60 ft.
  skills_text: Perception +4, Stealth +2
  immunities_text: Fire
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (4d8 + 4)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 10
  save_dexterity:
    value: 2
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 15
  max_hp:
    value: 22
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  burrow_speed:
    value: 15
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/brass_dragon_wyrmling
  implementation: dnd2024:creature_brass_dragon_wyrmling
  parameters:
    features:
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 7
            roll:
              count: 1
              sides: 10
              modifier: 2
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
        dc: 11
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 20
            origin: self
            width_ft: 5
        failure:
          damage:
          - average: 14
            roll:
              count: 4
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
        dc: 11
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 15
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
      perception: 4
      stealth: 2
    damage_immunities:
    - fire
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
- id: dnd2024:creature_brass_dragon_wyrmling
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
  source_sha256: afd34805d2b3c07c697f076eb9a7d96031da7f0aece6fe217ab18995dc910c2b
  source_path: builtin/srd-5.2.1/creature/brass_dragon_wyrmling.md
  pages:
  - 268
  engine_gaps: &id001
  - dnd2024:creature_brass_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: abe47e801c85cf4734556343ee5ffdf3d2602079fc62b6ee4e1b85c6428dd164
splat_clauses:
- key: source_rules
  text: '# Brass Dragon Wyrmling Medium Dragon (Metallic), Chaotic Good AC 15 Initiative +2 (12) HP 22
    (4d8 + 4) Speed 30 ft., Burrow 15 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 10 +0
    +2 Con 13 +1 +1 Int 10 +0 +0 Wis 11 +0 +2 Cha 13 +1 +1 Skills Perception +4, Stealth +2 Immunities
    Fire Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic CR 1 (XP
    200; PB +2) ### Actions Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10 + 2) Slashing damage.
    Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 11, each creature in a 20-foot-long, 5-footwide
    Line. Failure: 14 (4d6) Fire damage. Success: Half damage. Sleep Breath. Constitution Saving Throw:
    DC 11, each creature in a 15-foot Cone. Failure: The target has the Incapacitated condition until
    the end of its next turn, at which point it repeats the save. Second Failure: The target has the Unconscious
    condition for 1 minute. This effect ends for the target if it takes damage or a creature within 5
    feet of it takes an action to wake it.'
  sha256: c12bcb567ec838a1f204fdc61468a939a5e1b9ab6d42a5827132ec39cf56a3ff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Brass Dragon Wyrmling

<!-- SRD page 268 -->

Medium Dragon (Metallic), Chaotic Good\
AC 15 Initiative +2 (12)\
HP 22 (4d8 + 4)\
Speed 30 ft., Burrow 15 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 10 +0 +2 Con 13 +1 +1\
Int 10 +0 +0 Wis 11 +0 +2 Cha 13 +1 +1\
Skills Perception +4, Stealth +2\
Immunities Fire\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 1 (XP 200; PB +2)\

### Actions

Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10\
+ 2) Slashing damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 11, each creature in a 20-foot-long, 5-foot-\
wide Line. Failure: 14 (4d6) Fire damage. Success:\
Half damage.\
Sleep Breath. Constitution Saving Throw: DC 11, each\
creature in a 15-foot Cone. Failure: The target has the\
Incapacitated condition until the end of its next turn,\
at which point it repeats the save. Second Failure: The\
target has the Unconscious condition for 1 minute. This\
effect ends for the target if it takes damage or a creature\
within 5 feet of it takes an action to wake it.\

Original layout: [[../../srd-5.2.1/pages/268|page 268]].
