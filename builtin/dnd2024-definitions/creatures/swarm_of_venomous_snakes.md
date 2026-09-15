---
splat_schema: 1
splat_id: 1a810d5d-1e00-5c72-be2b-30118d102ace
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_venomous_snakes
splat_name: Swarm of Venomous Snakes
splat_origin:
  key: dnd2024:reference/creature/swarm_of_venomous_snakes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 362, 363
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  size_options_text: Medium Swarm of Tiny
  tags_text: s
  movement_text: 30 ft., Swim 30 ft.
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned
  senses_text: Blindsight 10 ft.; Passive Perception 10
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 36 (8d8)
splat_stats:
  strength:
    value: 8
  save_strength:
    value: -1
  dexterity:
    value: 18
  save_dexterity:
    value: 4
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 14
  max_hp:
    value: 36
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/swarm_of_venomous_snakes
  implementation: dnd2024:creature_swarm_of_venomous_snakes
  parameters:
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
    - sense: blindsight
      range_ft: 10
    passive_perception: 10
    languages:
      speak: true
      known: []
    features:
      swarm:
        economy: passive
        op: swarm.form
        shared_spaces_bidirectional: true
        minimum_opening:
          creature_size: tiny
          creature_shape: snake
        regain_hp: false
        gain_temporary_hp: false
      bites:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 4
            average: 8
          - type: poison
            amount:
              dice:
                count: 3
                sides: 6
              add: 0
            average: 10
          replacement_when:
            if:
              actor_bloodied: true
            components:
            - type: piercing
              amount:
                dice:
                  count: 1
                  sides: 4
                add: 4
              average: 6
            - type: poison
              amount:
                dice:
                  count: 3
                  sides: 6
                add: 0
              average: 10
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
- id: dnd2024:creature_swarm_of_venomous_snakes
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
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 777bacb0cda03bca612d13c8e530b1fa2c9c14fdf773deeb71b6f42a9a50112e
  source_path: builtin/srd-5.2.1/creature/swarm_of_venomous_snakes.md
  pages:
  - 362
  - 363
  engine_gaps: &id001
  - dnd2024:creature_swarm_of_venomous_snakes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 1204c325edb913c24f1b255a09afc059519f349a6382275159089016a65e4472
splat_clauses:
- key: source_rules
  text: '# Swarm of Venomous Snakes Medium Swarm of Tiny Beasts, Unaligned AC 14 Initiative +4 (14) HP
    36 (8d8) Speed 30 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 8 -1 -1 Dex 18 +4 +4 Con 11 +0 +0
    Int 1 -5 -5 Wis 10 +0 +0 Cha 3 -4 -4 Resistances Bludgeoning, Piercing, Slashing Immunities Charmed,
    Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned Senses Blindsight 10 ft.; Passive
    Perception 10 Languages None CR 2 (XP 450; PB +2) ### Traits Swarm. The swarm can occupy another creature’s
    space and vice versa, and the swarm can move through any opening large enough for a Tiny snake. The
    swarm can’t regain Hit Points or gain Temporary Hit Points. ### Actions Bites. Melee Attack Roll:
    +6, reach 5 ft. Hit: 8 (1d8 + 4) Piercing damage—or 6 (1d4 + 4) Piercing damage if the swarm is Bloodied—plus
    10 (3d6) Poison damage.'
  sha256: e94adc66ee34b336f83961cdb523001bf256fe665ffb20bac408556d8d373c7c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swarm of Venomous Snakes

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 14 Initiative +4 (14)\
HP 36 (8d8)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 18 +4 +4 Con 11 +0 +0\
Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Blindsight 10 ft.; Passive Perception 10\
Languages None\
CR 2 (XP 450; PB +2)\

<!-- SRD page 363 -->


### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny snake. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Bites. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 + 4)\
Piercing damage—or 6 (1d4 + 4) Piercing damage if the\
swarm is Bloodied—plus 10 (3d6) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/362|page 362]], [[../../srd-5.2.1/pages/363|page 363]].
