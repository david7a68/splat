---
splat_schema: 1
splat_id: e95b1f68-6031-5ca5-8a44-c1e6784bb620
splat_kind: creature
splat_key: dnd2024:creatures/animated_rug_of_smothering
splat_name: Animated Rug of Smothering
splat_origin:
  key: dnd2024:reference/creature/animated_rug_of_smothering
splat_sources:
- title: SRD 5.2.1
  locator: pp. 259
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: construct
  alignment: Unaligned
  movement_text: 10 ft.
  immunities_text: Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Blindsight 60 ft.; Passive Perception 6
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 27 (5d10)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
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
    value: 12
  max_hp:
    value: 27
  initiative:
    value: 4
  speed:
    value: 10
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/animated_rug_of_smothering
  implementation: dnd2024:creature_animated_rug_of_smothering
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
      smother:
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
        - op: choice
          chooser: actor
          choices:
          - effects:
            - op: damage
              components:
              - type: bludgeoning
                amount:
                  dice:
                    count: 2
                    sides: 6
                  add: 3
                average: 10
          - requires:
              target_kind: creature
              target_maximum_size: medium
            effects:
            - op: grapple
              escape_dc: 13
              maximum_targets: 1
              while_grappled:
              - op: condition.apply
                condition: restrained
              - op: condition.apply
                condition: blinded
              - op: suffocation.start
              - trigger: target.turn_start
                effects:
                - op: damage
                  components:
                  - type: bludgeoning
                    amount:
                      dice:
                        count: 2
                        sides: 6
                      add: 3
                    average: 10
        unavailable_while_grappling: true
        while_grappling:
          incoming_damage_multiplier: 0.5
          round: down
          grappled_target_takes_same_damage: true
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
- id: dnd2024:creature_animated_rug_of_smothering
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
  - target.turn_start
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
  source_sha256: 3062dc7d1668c55a13080e7435db919b19663f682d8cbdb2b6db19d41e3bd6ff
  source_path: builtin/srd-5.2.1/creature/animated_rug_of_smothering.md
  pages:
  - 259
  engine_gaps: &id001
  - dnd2024:creature_animated_rug_of_smothering
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4de35d1bfefd325b42ead9d197d818492c1bfb22b59d67c2eb632faf31140c4b
splat_clauses:
- key: source_rules
  text: '# Animated Rug of Smothering Large Construct, Unaligned AC 12 Initiative +4 (14) HP 27 (5d10)
    Speed 10 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 14 +2 +2 Con 10 +0 +0 Int 1 -5 -5 Wis 3 -4
    -4 Cha 1 -5 -5 Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified,
    Poisoned Senses Blindsight 60 ft.; Passive Perception 6 Languages None CR 2 (XP 450; PB +2) ### Actions
    Smother. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Bludgeoning damage. If the target is
    a Medium or smaller creature, the rug can give it the Grappled condition (escape DC 13) instead of
    dealing damage. Until the grapple ends, the target has the Blinded and Restrained conditions, is suffocating,
    and takes 10 (2d6 + 3) Bludgeoning damage at the start of each of its turns. The rug can smother only
    one creature at a time. While grappling the target, the rug can’t take this action, the rug halves
    the damage it takes (round down), and the target takes the same amount of damage.'
  sha256: 3010a2d0fadbb00b808d63a46063b66d7e52c3a4912cf2a5bae175351d5c40de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animated Rug of Smothering

<!-- SRD page 259 -->

Large Construct, Unaligned\
AC 12 Initiative +4 (14)\
HP 27 (5d10)\
Speed 10 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 14 +2 +2 Con 10 +0 +0\
Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Deafened,\
Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
Senses Blindsight 60 ft.; Passive Perception 6\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Smother. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Bludgeoning damage. If the target is a Medium or\
smaller creature, the rug can give it the Grappled con-\
dition (escape DC 13) instead of dealing damage. Until\
the grapple ends, the target has the Blinded and Re-\
strained conditions, is suffocating, and takes 10 (2d6 +\
3) Bludgeoning damage at the start of each of its turns.\
The rug can smother only one creature at a time.\
While grappling the target, the rug can’t take this ac-\
tion, the rug halves the damage it takes (round down),\
and the target takes the same amount of damage.\

Original layout: [[../../srd-5.2.1/pages/259|page 259]].
