---
splat_schema: 1
splat_id: 1b37be75-c5c9-5acc-9039-e032862c647f
splat_kind: creature
splat_key: dnd2024:creatures/basilisk
splat_name: Basilisk
splat_origin:
  key: dnd2024:reference/creature/basilisk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 262
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Unaligned
  movement_text: 20 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 52 (8d8 + 16)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 52
  initiative:
    value: -1
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/basilisk
  implementation: dnd2024:creature_basilisk
  parameters:
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 9
    languages:
      speak: true
      known: []
    features:
      bite:
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
                sides: 6
              add: 3
            average: 10
          - type: poison
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
      petrifying_gaze:
        economy: bonus_action
        op: saving_throw
        ability: constitution
        dc: 12
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 30
        failure:
        - op: condition.apply
          condition: restrained
        - op: saving_throw.repeat
          event: target.next_turn_end
          only_if_condition: restrained
          ability: constitution
          dc: 12
          success:
          - op: effect.end
          failure:
          - op: condition.remove
            condition: restrained
          - op: condition.apply
            condition: petrified
        success: []
        include_actor_if_sees_own_reflection_in_area: true
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
- id: dnd2024:creature_basilisk
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
  - target.next_turn_end
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
  source_sha256: 8a5782180bc9c45eceedab36e357a3762ebedae0b0585af2b8c81e8f36dc6ae4
  source_path: builtin/srd-5.2.1/creature/basilisk.md
  pages:
  - 262
  engine_gaps: &id001
  - dnd2024:creature_basilisk
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: c6d04fe2397402cc085eb64cda5f64fca1a8d2df8f4439c9190c6d8b68b7e23d
splat_clauses:
- key: source_rules
  text: '# Basilisk Medium Monstrosity, Unaligned AC 15 Initiative -1 (9) HP 52 (8d8 + 16) Speed 20 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 8 -1 -1 Con 15 +2 +2 Int 2 -4 -4 Wis 8 -1 -1 Cha 7 -2
    -2 Senses Darkvision 60 ft.; Passive Perception 9 Languages None CR 3 (XP 700; PB +2) ### Actions
    Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage plus 7 (2d6) Poison damage.
    ### Bonus Actions Petrifying Gaze (Recharge 4–6). Constitution Saving Throw: DC 12, each creature
    in a 30-foot Cone. If the basilisk sees its reflection in the Cone, the basilisk must make this save.
    First Failure: The target has the Restrained condition and repeats the save at the end of its next
    turn if it is still Restrained, ending the effect on itself on a success. Second Failure: The target
    has the Petrified condition instead of the Restrained condition.'
  sha256: a50df939a8fcec315b461f5d6a5b8f04cb95c122a10d99a56c2016566e2ae5f0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Basilisk

<!-- SRD page 262 -->

Medium Monstrosity, Unaligned\
AC 15 Initiative −1 (9)\
HP 52 (8d8 + 16)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 8 −1 −1 Con 15 +2 +2\
Int 2 −4 −4 Wis 8 −1 −1 Cha 7 −2 −2\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages None\
CR 3 (XP 700; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3)\
Piercing damage plus 7 (2d6) Poison damage.\

### Bonus Actions

Petrifying Gaze (Recharge 4–6). Constitution Saving\
Throw: DC 12, each creature in a 30-foot Cone. If\
the basilisk sees its reflection in the Cone, the basilisk\
must make this save. First Failure: The target has the\
Restrained condition and repeats the save at the end of\
its next turn if it is still Restrained, ending the effect on\
itself on a success. Second Failure: The target has the\
Petrified condition instead of the Restrained condition.\

Original layout: [[../../srd-5.2.1/pages/262|page 262]].
