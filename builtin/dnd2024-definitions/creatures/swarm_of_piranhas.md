---
splat_schema: 1
splat_id: 9c5354de-82e7-5e8a-8bb0-7514dd3a73c0
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_piranhas
splat_name: Swarm of Piranhas
splat_origin:
  key: dnd2024:reference/creature/swarm_of_piranhas
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
  movement_text: 5 ft., Swim 40 ft.
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned
  senses_text: Darkvision 60 ft.; Passive Perception 8
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 28 (8d8 - 8)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 9
  save_constitution:
    value: -1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 2
  save_charisma:
    value: -4
  armor_class:
    value: 13
  max_hp:
    value: 28
  initiative:
    value: 3
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/swarm_of_piranhas
  implementation: dnd2024:creature_swarm_of_piranhas
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
    - sense: darkvision
      range_ft: 60
    passive_perception: 8
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
          creature_shape: piranha
        regain_hp: false
        gain_temporary_hp: false
      water_breathing:
        economy: passive
        op: breathing.allow
        media:
        - water
        exclusive: true
      bites:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 5
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
              add: 3
            average: 8
          replacement_when:
            if:
              actor_bloodied: true
            components:
            - type: piercing
              amount:
                dice:
                  count: 1
                  sides: 4
                add: 3
              average: 5
        advantage_when:
          target_hp_below_maximum: true
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
- id: dnd2024:creature_swarm_of_piranhas
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
  source_sha256: 44b991c189afa373ecae0e06413ed45f26ca3e70984c980e102fa01e83d2f474
  source_path: builtin/srd-5.2.1/creature/swarm_of_piranhas.md
  pages:
  - 362
  engine_gaps: &id001
  - dnd2024:creature_swarm_of_piranhas
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: f6780b683a009a31fb8491d06171496d422db09515dea6dbdf2f278dbd9672b1
splat_clauses:
- key: source_rules
  text: '# Swarm of Piranhas Medium Swarm of Tiny Beasts, Unaligned AC 13 Initiative +3 (13) HP 28 (8d8
    - 8) Speed 5 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 16 +3 +3 Con 9 -1 -1 Int
    1 -5 -5 Wis 7 -2 -2 Cha 2 -4 -4 Resistances Bludgeoning, Piercing, Slashing Immunities Charmed, Frightened,
    Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned Senses Darkvision 60 ft.; Passive Perception
    8 Languages None CR 1 (XP 200; PB +2) ### Traits Swarm. The swarm can occupy another creature’s space
    and vice versa, and the swarm can move through any opening large enough for a Tiny piranha. The swarm
    can’t regain Hit Points or gain Temporary Hit Points. Water Breathing. The swarm can breathe only
    underwater. ### Actions Bites. Melee Attack Roll: +5 (with Advantage if the target doesn’t have all
    its Hit Points), reach 5 ft. Hit: 8 (2d4 + 3) Piercing damage, or 5 (1d4 + 3) Piercing damage if the
    swarm is Bloodied.'
  sha256: 0f55473e7d5efa8874ef37bea31fb6a2c266bca37fc1121e1c1aeffad5c8d693
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swarm of Piranhas

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 13 Initiative +3 (13)\
HP 28 (8d8 − 8)\
Speed 5 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 16 +3 +3 Con 9 −1 −1\
Int 1 −5 −5 Wis 7 −2 −2 Cha 2 −4 −4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move\
through any opening large enough for a Tiny piranha.\
The swarm can’t regain Hit Points or gain Temporary\
Hit Points.\
Water Breathing. The swarm can breathe only\
underwater.\

### Actions

Bites. Melee Attack Roll: +5 (with Advantage if the\
target doesn’t have all its Hit Points), reach 5 ft. Hit: 8\
(2d4 + 3) Piercing damage, or 5 (1d4 + 3) Piercing dam-\
age if the swarm is Bloodied.\

Original layout: [[../../srd-5.2.1/pages/362|page 362]].
