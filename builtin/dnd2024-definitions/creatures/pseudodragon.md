---
splat_schema: 1
splat_id: 19456839-4a0c-5fe3-9215-4852c1a02028
splat_kind: creature
splat_key: dnd2024:creatures/pseudodragon
splat_name: Pseudodragon
splat_origin:
  key: dnd2024:reference/creature/pseudodragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 316
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: dragon
  alignment: Neutral Good
  movement_text: 15 ft., Fly 60 ft.
  skills_text: Perception +5, Stealth +4
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 15
  languages_text: Understands Common and Draconic but can’t speak
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 10 (3d4 + 3)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 15
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
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 14
  max_hp:
    value: 10
  initiative:
    value: 2
  speed:
    value: 15
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/pseudodragon
  implementation: dnd2024:creature_pseudodragon
  parameters:
    features:
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: bite
          count: 2
      bite:
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
          - average: 4
            roll:
              count: 1
              sides: 4
              modifier: 2
            type: piercing
      sting:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 12
        targets:
          kind: creature
          count: 1
          range_ft: 5
          visible: true
        failure:
          damage:
          - roll:
              count: 2
              sides: 4
              modifier: 0
            type: poison
          conditions:
          - condition: poisoned
            duration_seconds: 3600
        failure_by_5_or_more:
          additional_while_poisoned:
            condition: unconscious
            ends_on:
            - target.damage_taken
            - event: creature.action
              operation: wake_target
              within_ft: 5
        success:
          effects: []
    skill_totals:
      perception: 5
      stealth: 4
    senses:
      blindsight: 10
      darkvision: 60
      passive_perception: 15
    languages:
      speaks: []
      understands:
      - common
      - draconic
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
- id: dnd2024:creature_pseudodragon
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
splat_conversion:
  status: specified
  category: creature
  source_sha256: 965c6d3e39e2b31647c07e64f1c5cfb87ec4d404af8186d914a69417767589f2
  source_path: builtin/srd-5.2.1/creature/pseudodragon.md
  pages:
  - 316
  engine_gaps: &id001
  - dnd2024:creature_pseudodragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: b85363182aa7c1993fad3e3ef6317311d61cd250198e32ced30c4978252f420b
splat_clauses:
- key: source_rules
  text: '# Pseudodragon Tiny Dragon, Neutral Good AC 14 Initiative +2 (12) HP 10 (3d4 + 3) Speed 15 ft.,
    Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 15 +2 +2 Con 13 +1 +1 Int 10 +0 +0 Wis 12 +1
    +1 Cha 10 +0 +0 Skills Perception +5, Stealth +4 Senses Blindsight 10 ft., Darkvision 60 ft.; Passive
    Perception 15 Languages Understands Common and Draconic but can’t speak CR 1/4 (XP 50; PB +2) ###
    Traits Magic Resistance. The pseudodragon has Advantage on saving throws against spells and other
    magical effects. ### Actions Multiattack. The pseudodragon makes two Bite attacks. Bite. Melee Attack
    Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage. Sting. Constitution Saving Throw: DC 12, one
    creature the pseudodragon can see within 5 feet. Failure: 5 (2d4) Poison damage, and the target has
    the Poisoned condition for 1 hour. Failure by 5 or More: While Poisoned, the target also has the Unconscious
    condition, which ends early if the target takes damage or a creature within 5 feet of it takes an
    action to wake it.'
  sha256: e6d896911f6962910558b126fdc7f5610fba8082e76c3d260bfd94cdc9526c8a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Pseudodragon

<!-- SRD page 316 -->

Tiny Dragon, Neutral Good\
AC 14  Initiative +2 (12)\
HP 10 (3d4 + 3)\
Speed 15 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 15 +2 +2 Con 13 +1 +1\
Int 10 +0 +0 Wis 12 +1 +1 Cha 10 +0 +0\
Skills Perception +5, Stealth +4\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 15\
Languages Understands Common and Draconic but can’t\
speak\
CR 1/4 (XP 50; PB +2)\

### Traits

Magic Resistance. The pseudodragon has Advan-\
tage on saving throws against spells and other magi-\
cal effects.\

### Actions

Multiattack. The pseudodragon makes two Bite attacks.\
Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2)\
Piercing damage.\
Sting. Constitution Saving Throw: DC 12, one creature\
the pseudodragon can see within 5 feet. Failure: 5 (2d4)\
Poison damage, and the target has the Poisoned condi-\
tion for 1 hour. Failure by 5 or More: While Poisoned,\
the target also has the Unconscious condition, which\
ends early if the target takes damage or a creature\
within 5 feet of it takes an action to wake it.\

Original layout: [[../../srd-5.2.1/pages/316|page 316]].
