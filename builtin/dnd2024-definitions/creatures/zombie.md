---
splat_schema: 1
splat_id: cf88db2d-0998-5e7a-b360-41fc7b993a4e
splat_kind: creature
splat_key: dnd2024:creatures/zombie
splat_name: Zombie
splat_origin:
  key: dnd2024:reference/creature/zombie
splat_sources:
- title: SRD 5.2.1
  locator: pp. 343, 344
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Neutral Evil
  movement_text: 20 ft.
  immunities_text: Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 8
  languages_text: Understands Common plus one other language but can’t speak
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 15 (2d8 + 6)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 6
  save_dexterity:
    value: -2
  constitution:
    value: 16
  save_constitution:
    value: 3
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
    value: 15
  initiative:
    value: -2
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/zombie
  implementation: dnd2024:creature_zombie
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
      additional_choice_count: 1
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
        attack_bonus: 3
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
                count: 1
                sides: 8
              add: 1
            average: 5
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
- id: dnd2024:creature_zombie
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
  source_sha256: c750a5cd4af5d95f8d69ddc3d8fea882989ba098d1e14982990162a86ae97813
  source_path: builtin/srd-5.2.1/creature/zombie.md
  pages:
  - 343
  - 344
  engine_gaps: &id001
  - dnd2024:creature_zombie
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 694f84c60d6fa8823821bd007fe8f7e5c2d338d239e1e04dff5549026613f591
splat_clauses:
- key: source_rules
  text: '# Zombie Medium Undead, Neutral Evil AC 8 Initiative -2 (8) HP 15 (2d8 + 6) Speed 20 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 6 -2 -2 Con16 +3 +3 Int 3 -4 -4 Wis 6 -2 +0 Cha 5 -3 -3 Immunities
    Poison; Exhaustion, Poisoned Senses Darkvision 60 ft.; Passive Perception 8 Languages Understands
    Common plus one other language but can’t speak CR 1/4 (XP 50; PB +2) ### Traits Undead Fortitude.
    If damage reduces the zombie to 0 Hit Points, it makes a Constitution saving throw (DC 5 plus the
    damage taken) unless the damage is Radiant or from a Critical Hit. On a successful save, the zombie
    drops to 1 Hit Point instead. ### Actions Slam. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d8 + 1)
    Bludgeoning damage.'
  sha256: e638350aafb69d20a9cb7d004c284ab392802822454cce96621ad5fd8edca6a7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Zombie

<!-- SRD page 343 -->

Medium Undead, Neutral Evil\
AC 8   Initiative −2 (8)\
HP 15 (2d8 + 6)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 6 −2 −2 Con16 +3 +3\
Int 3 −4 −4 Wis 6 −2 +0 Cha 5 −3 −3\
Immunities Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages Understands Common plus one other language\
but can’t speak\
CR 1/4 (XP 50; PB +2)\

### Traits

Undead Fortitude. If damage reduces the zombie to 0\
Hit Points, it makes a Constitution saving throw (DC 5\
plus the damage taken) unless the damage is Radiant or\
from a Critical Hit. On a successful save, the zombie\
drops to 1 Hit Point instead.\

<!-- SRD page 344 -->


### Actions

Slam. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d8 + 1)\
Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/343|page 343]], [[../../srd-5.2.1/pages/344|page 344]].
