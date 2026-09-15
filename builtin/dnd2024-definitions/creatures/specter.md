---
splat_schema: 1
splat_id: e10dcd2a-10f4-535b-a264-adc2b5243358
splat_kind: creature
splat_key: dnd2024:creatures/specter
splat_name: Specter
splat_origin:
  key: dnd2024:reference/creature/specter
splat_sources:
- title: SRD 5.2.1
  locator: pp. 327
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Chaotic Evil
  movement_text: 30 ft., Fly 50 ft. (hover)
  resistances_text: Acid, Bludgeoning, Cold, Fire, Lightning, Piercing, Slashing, Thunder
  immunities_text: Necrotic, Poison; Charmed, Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone,
    Restrained, Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Understands Common plus one other language but can’t speak
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (5d8)
splat_stats:
  strength:
    value: 1
  save_strength:
    value: -5
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 12
  max_hp:
    value: 22
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/specter
  implementation: dnd2024:creature_specter
  parameters:
    features:
      incorporeal_movement:
        economy: trait
        operation: passive_rule
        pass_through:
        - creatures
        - objects
        movement_terrain: difficult
        end_turn_inside_object_damage:
          roll:
            count: 1
            sides: 10
            modifier: 0
          type: force
      sunlight_sensitivity:
        economy: trait
        operation: passive_rule
        while:
          illumination: sunlight
        disadvantage_on:
        - ability_checks
        - attack_rolls
      life_drain:
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
              count: 2
              sides: 6
              modifier: 0
            type: necrotic
          maximum_hp_reduction:
            requires_target_kind: creature
            equal: damage_taken
    damage_resistances:
    - acid
    - bludgeoning
    - cold
    - fire
    - lightning
    - piercing
    - slashing
    - thunder
    damage_immunities:
    - necrotic
    - poison
    condition_immunities:
    - charmed
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
      passive_perception: 10
    languages:
      speaks: []
      understands:
      - common
      additional_languages:
        count: 1
        chooser: creator
        can_speak: false
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
- id: dnd2024:creature_specter
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
  - attack.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 2fb5330b001ff1ff38642c0b862c1da590faf1560ad5f0b6b4f5d6ca98299c87
  source_path: builtin/srd-5.2.1/creature/specter.md
  pages:
  - 327
  engine_gaps: &id001
  - dnd2024:creature_specter
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 32c5b71cb06ac8fb803c1a252e1d1a6fb0f8659d7899e8621ce5cf62933c9d82
splat_clauses:
- key: source_rules
  text: '# Specter Medium Undead, Chaotic Evil AC 12 Initiative +2 (12) HP 22 (5d8) Speed 30 ft., Fly
    50 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 1 -5 -5 Dex 14 +2 +2 Con 11 +0 +0 Int 10 +0 +0 Wis 10
    +0 +0 Cha 11 +0 +0 Resistances Acid, Bludgeoning, Cold, Fire, Lightning, Piercing, Slashing, Thunder
    Immunities Necrotic, Poison; Charmed, Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone,
    Restrained, Unconscious Senses Darkvision 60 ft.; Passive Perception 10 Languages Understands Common
    plus one other language but can’t speak CR 1 (XP 200; PB +2) ### Traits Incorporeal Movement. The
    specter can move through other creatures and objects as if they were Difficult Terrain. It takes 5
    (1d10) Force damage if it ends its turn inside an object. Sunlight Sensitivity. While in sunlight,
    the specter has Disadvantage on ability checks and attack rolls. ### Actions Life Drain. Melee Attack
    Roll: +4, reach 5 ft. Hit: 7 (2d6) Necrotic damage. If the target is a creature, its Hit Point maximum
    decreases by an amount equal to the damage taken.'
  sha256: 169a0768d79e636af08e76af40bc6403b500653e28f661afd43e77b8fe1abe43
  status: contract_specified
splat_requires_mechanics: *id001
---

# Specter

<!-- SRD page 327 -->

Medium Undead, Chaotic Evil\
AC 12 Initiative +2 (12)\
HP 22 (5d8)\
Speed 30 ft., Fly 50 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 14 +2 +2 Con 11 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 11 +0 +0\
Resistances Acid, Bludgeoning, Cold, Fire, Lightning,\
Piercing, Slashing, Thunder\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Grappled, Paralyzed, Petrified, Poisoned, Prone,\
Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Understands Common plus one other language\
but can’t speak\
CR 1 (XP 200; PB +2)\

### Traits

Incorporeal Movement. The specter can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\
Sunlight Sensitivity. While in sunlight, the specter has\
Disadvantage on ability checks and attack rolls.\

### Actions

Life Drain. Melee Attack Roll: +4, reach 5 ft. Hit: 7\
(2d6) Necrotic damage. If the target is a creature, its Hit\
Point maximum decreases by an amount equal to the\
damage taken.\

Original layout: [[../../srd-5.2.1/pages/327|page 327]].
