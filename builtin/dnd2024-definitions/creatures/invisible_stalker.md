---
splat_schema: 1
splat_id: e2791c4d-9116-5342-8718-e3f3d9ffb2fc
splat_kind: creature
splat_key: dnd2024:creatures/invisible_stalker
splat_name: Invisible Stalker
splat_origin:
  key: dnd2024:reference/creature/invisible_stalker
splat_sources:
- title: SRD 5.2.1
  locator: pp. 301, 302
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  movement_text: 50 ft., Fly 50 ft. (hover)
  skills_text: Perception +8, Stealth +10
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 18
  languages_text: Common, Primordial (Auran)
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 97 (13d10 + 26)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 19
  save_dexterity:
    value: 4
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 15
  save_wisdom:
    value: 2
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 14
  max_hp:
    value: 97
  initiative:
    value: 7
  speed:
    value: 50
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/invisible_stalker
  implementation: dnd2024:creature_invisible_stalker
  parameters:
    features:
      air_form:
        economy: trait
        operation: passive_rule
        occupy_enemy_space: true
        minimum_gap_inches: 1
        extra_movement_cost: 0
      invisibility:
        economy: trait
        operation: passive_rule
        conditions:
        - condition: invisible
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: wind_swipe
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: vortex
      wind_swipe:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 7
          reach_ft: 5
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: force
      vortex:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 14
        targets:
          kind: creature
          count: 1
          maximum_size: large
          location: self.space
        failure:
          damage:
          - average: 7
            roll:
              count: 1
              sides: 8
              modifier: 3
            type: thunder
          grapple:
            condition: grappled
            source: self
            escape_dc: 13
            while_grappled:
              can_cast_verbal_spells: false
              damage_each_self_turn_start:
              - roll:
                  count: 2
                  sides: 6
                  modifier: 0
                type: thunder
        success:
          effects: []
    skill_totals:
      perception: 8
      stealth: 10
    damage_resistances:
    - bludgeoning
    - piercing
    - slashing
    damage_immunities:
    - poison
    condition_immunities:
    - exhaustion
    - grappled
    - paralyzed
    - petrified
    - poisoned
    - prone
    - restrained
    - unconscious
    senses:
      darkvision: 60
      passive_perception: 18
    languages:
      speaks:
      - common
      - language: primordial
        dialects:
        - auran
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
- id: dnd2024:creature_invisible_stalker
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
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: fbab1143397e67ea044df2d05f6dc3d5398cc31723b181138751274f55957a2a
  source_path: builtin/srd-5.2.1/creature/invisible_stalker.md
  pages:
  - 301
  - 302
  engine_gaps: &id001
  - dnd2024:creature_invisible_stalker
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 5256cf75f7dc7015ffba9d9bbff35cb889c6da84e6662696e00fc834ea29afc8
splat_clauses:
- key: source_rules
  text: '# Invisible Stalker Large Elemental, Neutral AC 14 Initiative +7 (22) HP 97 (13d10 + 26) Speed
    50 ft., Fly 50 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 19 +4 +4 Con 14 +2 +2 Int 10
    +0 +0 Wis 15 +2 +2 Cha 11 +0 +0 Skills Perception +8, Stealth +10 Resistances Bludgeoning, Piercing,
    Slashing Immunities Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained,
    Unconscious Senses Darkvision 60 ft.; Passive Perception 18 Languages Common, Primordial (Auran) CR
    6 (XP 2,300; PB +3) ### Traits Air Form. The stalker can enter an enemy’s space and stop there. It
    can move through a space as narrow as 1 inch without expending extra movement to do so. Invisibility.
    The stalker has the Invisible condition. ### Actions Multiattack. The stalker makes three Wind Swipe
    attacks. It can replace one attack with a use of Vortex. Wind Swipe. Melee Attack Roll: +7, reach
    5 ft. Hit: 11 (2d6 + 4) Force damage. Vortex. Constitution Saving Throw: DC 14, one Large or smaller
    creature in the stalker’s space. Failure: 7 (1d8 + 3) Thunder damage, and the target has the Grappled
    condition (escape DC 13). Until the grapple ends, the target can’t cast spells with a Verbal component
    and takes 7 (2d6) Thunder damage at the start of each of the stalker’s turns.'
  sha256: 44575a0e5fd5086a8b40ef020a031118af373973a72ac05674b76814e668d573
  status: contract_specified
splat_requires_mechanics: *id001
---

# Invisible Stalker

<!-- SRD page 301 -->

Large Elemental, Neutral\
AC 14 Initiative +7 (22)\
HP 97 (13d10 + 26)\
Speed 50 ft., Fly 50 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 19 +4 +4 Con 14 +2 +2\
Int 10 +0 +0 Wis 15 +2 +2 Cha 11 +0 +0\
Skills Perception +8, Stealth +10\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Poison; Exhaustion, Grappled, Paralyzed,\
Petrified, Poisoned, Prone, Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 18\
Languages Common, Primordial (Auran)\
CR 6 (XP 2,300; PB +3)\

### Traits

Air Form. The stalker can enter an enemy’s space and\
stop there. It can move through a space as narrow as 1\
inch without expending extra movement to do so.\
Invisibility. The stalker has the Invisible condition.\

### Actions

Multiattack. The stalker makes three Wind Swipe at-\
tacks. It can replace one attack with a use of Vortex.\
Wind Swipe. Melee Attack Roll: +7, reach 5 ft. Hit: 11\
(2d6 + 4) Force damage.\
Vortex. Constitution Saving Throw: DC 14, one Large\
or smaller creature in the stalker’s space. Failure: 7 (1d8\
+ 3) Thunder damage, and the target has the Grappled\
condition (escape DC 13). Until the grapple ends, the\
target can’t cast spells with a Verbal component and\

<!-- SRD page 302 -->

takes 7 (2d6) Thunder damage at the start of each of the\
stalker’s turns.\

Original layout: [[../../srd-5.2.1/pages/301|page 301]], [[../../srd-5.2.1/pages/302|page 302]].
