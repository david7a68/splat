---
splat_schema: 1
splat_id: 7714e47d-87df-57e5-8a58-61c51ede02a6
splat_kind: creature
splat_key: dnd2024:creatures/animated_armor
splat_name: Animated Armor
splat_origin:
  key: dnd2024:reference/creature/animated_armor
splat_sources:
- title: SRD 5.2.1
  locator: pp. 259
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: construct
  alignment: Unaligned
  movement_text: 25 ft.
  immunities_text: Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Blindsight 60 ft.; Passive Perception 6
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 33 (6d8 + 6)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 3
  save_wisdom:
    value: -4
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 18
  max_hp:
    value: 33
  initiative:
    value: 2
  speed:
    value: 25
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/animated_armor
  implementation: dnd2024:creature_animated_armor
  parameters:
    damage_immunities:
    - poison
    - psychic
    condition_immunities:
    - charmed
    - deafened
    - exhaustion
    - frightened
    - paralyzed
    - petrified
    - poisoned
    senses:
    - sense: blindsight
      range_ft: 60
    passive_perception: 6
    languages:
      speak: true
      known: []
    features:
      slam:
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
          - type: bludgeoning
            amount:
              dice:
                count: 1
                sides: 6
              add: 2
            average: 5
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: slam
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
- id: dnd2024:creature_animated_armor
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
  source_sha256: 06b12c19243a833a01300c6a3ff97cfb120b564d22184a530047ed6e4dd4c7ae
  source_path: builtin/srd-5.2.1/creature/animated_armor.md
  pages:
  - 259
  engine_gaps: &id001
  - dnd2024:creature_animated_armor
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/animated_armor
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 9a314b2fcd47fb9fab624f480df023302a79d2239051eaa09a75c457acb9fa5d
splat_clauses:
- key: source_rules
  text: '# Animated Armor Medium Construct, Unaligned AC 18 Initiative +2 (12) HP 33 (6d8 + 6) Speed 25
    ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 11 +0 +0 Con 13 +1 +1 Int 1 -5 -5 Wis 3 -4 -4 Cha
    1 -5 -5 Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified,
    Poisoned Senses Blindsight 60 ft.; Passive Perception 6 Languages None CR 1 (XP 200; PB +2) ### Actions
    Multiattack. The armor makes two Slam attacks. Slam. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6
    + 2) Bludgeoning damage.'
  sha256: eda39414ec96129a7d30ecc4a4c8242117310e377e07aad0f6facb00a8a0bb86
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animated Armor

<!-- SRD page 259 -->

Medium Construct, Unaligned\
AC 18 Initiative +2 (12)\
HP 33 (6d8 + 6)\
Speed 25 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 11 +0 +0 Con 13 +1 +1\
Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Deafened,\
Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
Senses Blindsight 60 ft.; Passive Perception 6\
Languages None\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The armor makes two Slam attacks.\
Slam. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/259|page 259]].
