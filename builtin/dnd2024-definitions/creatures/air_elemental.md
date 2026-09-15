---
splat_schema: 1
splat_id: 27dce8d8-ae3a-5153-999b-db85e2a98284
splat_kind: creature
splat_key: dnd2024:creatures/air_elemental
splat_name: Air Elemental
splat_origin:
  key: dnd2024:reference/creature/air_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 258, 259
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  movement_text: 10 ft., Fly 90 ft. (hover)
  resistances_text: Bludgeoning, Lightning, Piercing, Slashing
  immunities_text: Poison, Thunder; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained,
    Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Auran)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 90 (12d10 + 24)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 20
  save_dexterity:
    value: 5
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 90
  initiative:
    value: 5
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/air_elemental
  implementation: dnd2024:creature_air_elemental
  parameters:
    damage_resistances:
    - bludgeoning
    - lightning
    - piercing
    - slashing
    damage_immunities:
    - poison
    - thunder
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
    - sense: darkvision
      range_ft: 60
    passive_perception: 10
    languages:
      speak: true
      known:
      - primordial
      primordial_dialects:
      - auran
    features:
      air_form:
        economy: passive
        op: movement.form
        minimum_opening_inches: 1
        extra_movement_cost: 0
        can_enter_and_end_in_creature_space: true
      thunderous_slam:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 8
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: thunder
            amount:
              dice:
                count: 2
                sides: 8
              add: 5
            average: 14
      whirlwind:
        economy: action
        op: saving_throw
        ability: strength
        dc: 13
        targets:
          kind: creature
          count: 1
          maximum_size: medium
          shares_actor_space: true
        failure:
        - op: damage
          components:
          - type: thunder
            amount:
              dice:
                count: 4
                sides: 10
              add: 2
            average: 24
        - op: push
          maximum_ft: 20
          direction: straight_away_from_actor
        - op: condition.apply
          condition: prone
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
        recharge:
          die_sides: 6
          success_min: 4
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: thunderous_slam
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_air_elemental
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 4c2573fab6820602294d9f2cb3b402f3aed864703c0aa03731900ab113cf9988
  source_path: builtin/srd-5.2.1/creature/air_elemental.md
  pages:
  - 258
  - 259
  engine_gaps: &id001
  - dnd2024:creature_air_elemental
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4be4a42894d8e8943254cef91dc76a51b5eb6142eddb2db8959a282e8ccccd57
splat_clauses:
- key: source_rules
  text: '# Air Elemental Large Elemental, Neutral AC 15 Initiative +5 (15) HP 90 (12d10 + 24) Speed 10
    ft., Fly 90 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 20 +5 +5 Con 14 +2 +2 Int 6 -2
    -2 Wis 10 +0 +0 Cha 6 -2 -2 Resistances Bludgeoning, Lightning, Piercing, Slashing Immunities Poison,
    Thunder; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious Senses
    Darkvision 60 ft.; Passive Perception 10 Languages Primordial (Auran) CR 5 (XP 1,800; PB +3) ### Traits
    Air Form. The elemental can enter a creature’s space and stop there. It can move through a space as
    narrow as 1 inch without expending extra movement to do so. ### Actions Multiattack. The elemental
    makes two Thunderous Slam attacks. Thunderous Slam. Melee Attack Roll: +8, reach 10 ft. Hit: 14 (2d8
    + 5) Thunder damage. Whirlwind (Recharge 4–6). Strength Saving Throw: DC 13, one Medium or smaller
    creature in the elemental’s space. Failure: 24 (4d10 + 2) Thunder damage, and the target is pushed
    up to 20 feet straight away from the elemental and has the Prone condition. Success: Half damage only.'
  sha256: 50f5b76fee650cc7233824dd8e6d509538c3b3f0b641699ab5cd9a193b01ebae
  status: contract_specified
splat_requires_mechanics: *id001
---

# Air Elemental

<!-- SRD page 258 -->

Large Elemental, Neutral\
AC 15 Initiative +5 (15)\
HP 90 (12d10 + 24)\
Speed 10 ft., Fly 90 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 20 +5 +5 Con 14 +2 +2\
Int 6 −2 −2 Wis 10 +0 +0 Cha 6 −2 −2\
Resistances Bludgeoning, Lightning, Piercing, Slashing\
Immunities Poison, Thunder; Exhaustion, Grappled,\
Paralyzed, Petrified, Poisoned, Prone, Restrained,\
Unconscious\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Auran)\
CR 5 (XP 1,800; PB +3)\

### Traits

Air Form. The elemental can enter a creature’s space\
and stop there. It can move through a space as narrow\
as 1 inch without expending extra movement to do so.\

### Actions

Multiattack. The elemental makes two Thunderous\
Slam attacks.\
Thunderous Slam. Melee Attack Roll: +8, reach 10 ft.\
Hit: 14 (2d8 + 5) Thunder damage.\

<!-- SRD page 259 -->

Whirlwind (Recharge 4–6). Strength Saving Throw: DC\
13, one Medium or smaller creature in the elemental’s\
space. Failure: 24 (4d10 + 2) Thunder damage, and the\
target is pushed up to 20 feet straight away from the\
elemental and has the Prone condition. Success: Half\
damage only.\

Original layout: [[../../srd-5.2.1/pages/258|page 258]], [[../../srd-5.2.1/pages/259|page 259]].
