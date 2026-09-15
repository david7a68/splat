---
splat_schema: 1
splat_id: 7a6ad303-6841-5a1d-ae28-fe80b260ced7
splat_kind: creature
splat_key: dnd2024:creatures/ogre_zombie
splat_name: Ogre Zombie
splat_origin:
  key: dnd2024:reference/creature/ogre_zombie
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: undead
  alignment: Neutral Evil
  movement_text: 30 ft.
  immunities_text: Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 8
  languages_text: Understands Common and Giant but can’t speak
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 85 (9d10 + 36)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 6
  save_dexterity:
    value: -2
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 6
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 8
  max_hp:
    value: 85
  initiative:
    value: -2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ogre_zombie
  implementation: dnd2024:creature_ogre_zombie
  parameters:
    damage_immunities:
    - poison
    condition_immunities:
    - exhaustion
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 8
    languages:
      speak: false
      known:
      - common
      - giant
    features:
      undead_fortitude:
        economy: passive
        trigger: damage.reduces_actor_to_zero_hp
        exclude:
          damage_type: radiant
          critical_hit: true
        save:
          ability: constitution
          dc:
            add:
            - 5
            - event_field: damage_taken
          success:
            hp_instead: 1
      slam:
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
          - type: bludgeoning
            amount:
              dice:
                count: 2
                sides: 8
              add: 4
            average: 13
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
- id: dnd2024:creature_ogre_zombie
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
  - damage.reduces_actor_to_zero_hp
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
  source_sha256: ec9b43610997b5a79b2a6c977b1b5df4e59627ccea8780e8d5e9b9772f8e7ac8
  source_path: builtin/srd-5.2.1/creature/ogre_zombie.md
  pages:
  - 344
  engine_gaps: &id001
  - dnd2024:creature_ogre_zombie
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: b82f457a779635a8e643fe7e6a1169809443e927a68c58259a038b442e32189f
splat_clauses:
- key: source_rules
  text: '# Ogre Zombie Large Undead, Neutral Evil AC 8 Initiative -2 (8) HP 85 (9d10 + 36) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 6 -2 -2 Con 18 +4 +4 Int 3 -4 -4 Wis 6 -2 +0 Cha 5 -3
    -3 Immunities Poison; Exhaustion, Poisoned Senses Darkvision 60 ft.; Passive Perception 8 Languages
    Understands Common and Giant but can’t speak CR 2 (XP 450; PB +2) ### Traits Undead Fortitude. If
    damage reduces the zombie to 0 Hit Points, it makes a Constitution saving throw (DC 5 plus the damage
    taken) unless the damage is Radiant or from a Critical Hit. On a successful save, the zombie drops
    to 1 Hit Point instead. ### Actions Slam. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning
    damage.'
  sha256: 9f93426545c6e72deb4ff2491681d5f0e954a533b2c69bf62e78bec2a9cc5d1a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ogre Zombie

<!-- SRD page 344 -->

Large Undead, Neutral Evil\
AC 8 Initiative −2 (8)\
HP 85 (9d10 + 36)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 6 −2 −2 Con 18 +4 +4\
Int 3 −4 −4 Wis 6 −2 +0 Cha 5 −3 −3\
Immunities Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Understands Common and Giant but can’t\
speak\
CR 2 (XP 450; PB +2)\

### Traits

Undead Fortitude. If damage reduces the zombie to 0\
Hit Points, it makes a Constitution saving throw (DC 5\
plus the damage taken) unless the damage is Radiant or\
from a Critical Hit. On a successful save, the zombie\
drops to 1 Hit Point instead.\

### Actions

Slam. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8\
+ 4) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/344|page 344]].
