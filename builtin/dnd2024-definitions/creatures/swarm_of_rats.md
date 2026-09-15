---
splat_schema: 1
splat_id: 8f82174e-ed19-5709-85e6-a3e3d2a1d8a7
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_rats
splat_name: Swarm of Rats
splat_origin:
  key: dnd2024:reference/creature/swarm_of_rats
splat_sources:
- title: SRD 5.2.1
  locator: pp. 362
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  size_options_text: Medium Swarm of Tiny
  tags_text: s
  movement_text: 30 ft., Climb 30 ft.
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned
  senses_text: Darkvision 30 ft.; Passive Perception 10
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 14 (4d8 - 4)
splat_stats:
  strength:
    value: 9
  save_strength:
    value: -1
  dexterity:
    value: 11
  save_dexterity:
    value: 2
  constitution:
    value: 9
  save_constitution:
    value: -1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 10
  max_hp:
    value: 14
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/swarm_of_rats
  implementation: dnd2024:creature_swarm_of_rats
  parameters:
    features:
      swarm:
        economy: trait
        operation: passive_rule
        share_creature_spaces: true
        minimum_opening_size: tiny
        can_regain_hp: false
        can_gain_temporary_hp: false
      bites:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 2
          reach_ft: 5
        hit:
          damage:
            if:
              self_bloodied: true
            then:
            - average: 2
              roll:
                count: 1
                sides: 4
                modifier: 0
              type: piercing
            else:
            - average: 5
              roll:
                count: 2
                sides: 4
                modifier: 0
              type: piercing
    damage_resistances:
    - bludgeoning
    - piercing
    - slashing
    damage_immunities: []
    condition_immunities:
    - charmed
    - frightened
    - grappled
    - paralyzed
    - petrified
    - prone
    - restrained
    - stunned
    senses:
      darkvision: 30
      passive_perception: 10
    languages:
      speaks: []
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
- id: dnd2024:creature_swarm_of_rats
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
  - grappling
  - spatial_queries
  - limited_resources
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: c9e6bf7255d2409ea939b940bfe9175ceb27c97d96465d9e4d73652dc6f10183
  source_path: builtin/srd-5.2.1/creature/swarm_of_rats.md
  pages:
  - 362
  engine_gaps: &id001
  - dnd2024:creature_swarm_of_rats
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 930ae2531274fb708e13e6488deab61e230ff6f18fa918028cbb424d6e594da5
splat_clauses:
- key: source_rules
  text: '# Swarm of Rats Medium Swarm of Tiny Beasts, Unaligned AC 10 Initiative +0 (10) HP 14 (4d8 -
    4) Speed 30 ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 9 -1 -1 Dex 11 +0 +2 Con 9 -1 -1 Int
    2 -4 -4 Wis 10 +0 +0 Cha 3 -4 -4 Resistances Bludgeoning, Piercing, Slashing Immunities Charmed, Frightened,
    Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned Senses Darkvision 30 ft.; Passive Perception
    10 Languages None CR 1/4 (XP 50; PB +2) ### Traits Swarm. The swarm can occupy another creature’s
    space and vice versa, and the swarm can move through any opening large enough for a Tiny rat. The
    swarm can’t regain Hit Points or gain Temporary Hit Points. ### Actions Bites. Melee Attack Roll:
    +2, reach 5 ft. Hit: 5 (2d4) Piercing damage, or 2 (1d4) Piercing damage if the swarm is Bloodied.'
  sha256: f039047dbb93e871db928edc5d90f0990d87ac030df87880d4dd40ed00b90c9e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swarm of Rats

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 10 Initiative +0 (10)\
HP 14 (4d8 − 4)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 9 −1 −1 Dex 11 +0 +2 Con 9 −1 −1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Darkvision 30 ft.; Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny rat. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Bites. Melee Attack Roll: +2, reach 5 ft. Hit: 5 (2d4)\
Piercing damage, or 2 (1d4) Piercing damage if the\
swarm is Bloodied.\

Original layout: [[../../srd-5.2.1/pages/362|page 362]].
