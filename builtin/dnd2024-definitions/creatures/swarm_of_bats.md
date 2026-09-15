---
splat_schema: 1
splat_id: a252daaf-d3b9-5f4a-97fc-dcd183b40d3f
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_bats
splat_name: Swarm of Bats
splat_origin:
  key: dnd2024:reference/creature/swarm_of_bats
splat_sources:
- title: SRD 5.2.1
  locator: pp. 361
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  size_options_text: Large Swarm of Tiny
  tags_text: s
  movement_text: 5 ft., Fly 30 ft.
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned
  senses_text: Blindsight 60 ft.; Passive Perception 11
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 11 (2d10)
splat_stats:
  strength:
    value: 5
  save_strength:
    value: -3
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 11
  initiative:
    value: 2
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/swarm_of_bats
  implementation: dnd2024:creature_swarm_of_bats
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
      range_ft: 60
    passive_perception: 11
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
          creature_shape: bat
        regain_hp: false
        gain_temporary_hp: false
      bites:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 4
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
                count: 2
                sides: 4
              add: 0
            average: 5
          replacement_when:
            if:
              actor_bloodied: true
            components:
            - type: piercing
              amount:
                dice:
                  count: 1
                  sides: 4
                add: 0
              average: 2
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
- id: dnd2024:creature_swarm_of_bats
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
  source_sha256: e3a92f01202d09a5ccf81ee03f7207854f43983a19aa4acd5c789a34e6ed6674
  source_path: builtin/srd-5.2.1/creature/swarm_of_bats.md
  pages:
  - 361
  engine_gaps: &id001
  - dnd2024:creature_swarm_of_bats
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: f227d62a23bb769abc1a5ec2b61b8c029e174d8eeea6fc72b28fc40b6a630bd8
splat_clauses:
- key: source_rules
  text: '# Swarm of Bats Large Swarm of Tiny Beasts, Unaligned AC 12 Initiative +2 (12) HP 11 (2d10) Speed
    5 ft., Fly 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 5 -3 -3 Dex 15 +2 +2 Con 10 +0 +0 Int 2 -4 -4 Wis
    12 +1 +1 Cha 4 -3 -3 Resistances Bludgeoning, Piercing, Slashing Immunities Charmed, Frightened, Grappled,
    Paralyzed, Petrified, Prone, Restrained, Stunned Senses Blindsight 60 ft.; Passive Perception 11 Languages
    None CR 1/4 (XP 50; PB +2) ### Traits Swarm. The swarm can occupy another creature’s space and vice
    versa, and the swarm can move through any opening large enough for a Tiny bat. The swarm can’t regain
    Hit Points or gain Temporary Hit Points. ### Actions Bites. Melee Attack Roll: +4, reach 5 ft. Hit:
    5 (2d4) Piercing damage, or 2 (1d4) Piercing damage if the swarm is Bloodied.'
  sha256: cd98657c231d5b2ac6426a9d67d1ed3658b67c4232c76484fb13196f6deb1931
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swarm of Bats

<!-- SRD page 361 -->

Large Swarm of Tiny Beasts, Unaligned\
AC 12 Initiative +2 (12)\
HP 11 (2d10)\
Speed 5 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  5 −3 −3 Dex 15 +2 +2 Con 10 +0 +0\
Int 2 −4 −4 Wis 12 +1 +1 Cha  4 −3 −3\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Blindsight 60 ft.; Passive Perception 11\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny bat. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Bites. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (2d4)\
Piercing damage, or 2 (1d4) Piercing damage if the\
swarm is Bloodied.\

Original layout: [[../../srd-5.2.1/pages/361|page 361]].
