---
splat_schema: 1
splat_id: 661e684a-bd4e-5a77-9411-28c2f8881c4d
splat_kind: creature
splat_key: dnd2024:creatures/avatar_of_death
splat_name: Avatar of Death
splat_origin:
  key: dnd2024:reference/creature/avatar_of_death
splat_sources:
- title: SRD 5.2.1
  locator: pp. 232
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Neutral evil
  movement_text: 60 ft., Fly 60 ft. (hover)
  immunities_text: Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned,
    Unconscious
  senses_text: Truesight 60 ft., Passive Perception 13
  languages_text: All languages known to its summoner
  cr_text: None (XP 0; PB equals its summoner’s)
  hit_points_text: Half the HP maximum of its summoner
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 16
  save_charisma:
    value: 3
  armor_class:
    value: 20
  initiative:
    value: 3
  speed:
    value: 60
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/avatar_of_death
  implementation: dnd2024:creature_avatar_of_death
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
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: reaping_scythe
          count:
            ceil:
              divide:
              - stat: summoner.proficiency_bonus
              - 2
      reaping_scythe:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          automatic_hit: true
          reach_ft: 5
        hit:
          damage:
          - average: 7
            roll:
              count: 1
              sides: 8
              modifier: 3
            type: slashing
          - average: 4
            roll:
              count: 1
              sides: 8
              modifier: 0
            type: necrotic
    damage_immunities:
    - necrotic
    - poison
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    - paralyzed
    - petrified
    - poisoned
    - unconscious
    senses:
      truesight: 60
      passive_perception: 13
    languages:
      speaks:
        copy: summoner.languages
      understands: []
    derived_statistics:
      hp:
        divide:
        - stat: summoner.maximum_hp
        - 2
      proficiency_bonus:
        stat: summoner.proficiency_bonus
      languages:
        stat: summoner.languages
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
- id: dnd2024:creature_avatar_of_death
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
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 8d9b2b093624ffedb1884b0c9210ad94ca34ccb698b3125bcda1edb0d645b085
  source_path: builtin/srd-5.2.1/creature/avatar_of_death.md
  pages:
  - 232
  engine_gaps: &id001
  - dnd2024:creature_avatar_of_death
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: a2b958436bca695d1b6ff5c7b3a1815d2bd8dbd21257a150ef0c12c3e7f5f42b
splat_clauses:
- key: source_rules
  text: '# Avatar of Death Medium Undead, Neutral evil AC 20 Initiative +3 (13) HP Half the HP maximum
    of its summoner Speed 60 ft., Fly 60 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 16 +3
    +3 Con 16 +3 +3 Int 16 +3 +3 Wis 16 +3 +3 Cha 16 +3 +3 Immunities Necrotic, Poison; Charmed, Exhaustion,
    Frightened, Paralyzed, Petrified, Poisoned, Unconscious Senses Truesight 60 ft., Passive Perception
    13 Languages All languages known to its summoner CR None (XP 0; PB equals its summoner’s) ### Traits
    Incorporeal Movement. The avatar can move through other creatures and objects as if they were Difficult
    Terrain. It takes 5 (1d10) Force damage if it ends its turn inside an object. ### Actions Multiattack.
    The avatar makes a number of Reaping Scythe attacks equal to half the summoner’s Proficiency Bonus
    (rounded up). Reaping Scythe. Melee Attack Roll: Automatic hit, reach 5 ft. Hit: 7 (1d8 + 3) Slashing
    damage plus 4 (1d8) Necrotic damage.'
  sha256: 20ad5c418a41b7594fe8978a19610d16f239360d1fe814aa715affbb24945323
  status: contract_specified
splat_requires_mechanics: *id001
---

# Avatar of Death

<!-- SRD page 232 -->

Medium Undead, Neutral evil\
AC 20  Initiative +3 (13)\
HP Half the HP maximum of its summoner\
Speed 60 ft., Fly 60 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 16 +3 +3 Con 16 +3 +3\
Int 16 +3 +3 Wis 16 +3 +3 Cha 16 +3 +3\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned, Unconscious\
Senses Truesight 60 ft., Passive Perception 13\
Languages All languages known to its summoner\
CR None (XP 0; PB equals its summoner’s)\

### Traits

Incorporeal Movement. The avatar can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\

### Actions

Multiattack. The avatar makes a number of Reaping\
Scythe attacks equal to half the summoner’s Proficiency\
Bonus (rounded up).\
Reaping Scythe. Melee Attack Roll: Automatic hit,\
reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage plus 4 (1d8)\
Necrotic damage.\

Original layout: [[../../srd-5.2.1/pages/232|page 232]].
