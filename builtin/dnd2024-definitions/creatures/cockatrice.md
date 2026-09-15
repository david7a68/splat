---
splat_schema: 1
splat_id: 3f631fda-bb06-535a-8dd9-bb98f9c56127
splat_kind: creature
splat_key: dnd2024:creatures/cockatrice
splat_name: Cockatrice
splat_origin:
  key: dnd2024:reference/creature/cockatrice
splat_sources:
- title: SRD 5.2.1
  locator: pp. 275
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: monstrosity
  alignment: Unaligned
  movement_text: 20 ft., Fly 40 ft.
  immunities_text: Petrified
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 22 (5d6 + 5)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 11
  max_hp:
    value: 22
  initiative:
    value: 1
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/cockatrice
  implementation: dnd2024:creature_cockatrice
  parameters:
    features:
      petrifying_bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 3
          reach_ft: 5
        hit:
          damage:
          - average: 3
            roll:
              count: 1
              sides: 4
              modifier: 1
            type: piercing
          save:
            operation: saving_throw
            ability: constitution
            dc: 11
            targets:
              kind: creature
            failure:
              conditions:
              - condition: restrained
              repeat_save:
                event: target.next_turn_end
                if_still_restrained: true
                success: end_effect
                failure:
                  replace_condition:
                    from: restrained
                    to: petrified
                  duration_seconds: 86400
    damage_immunities: []
    condition_immunities:
    - petrified
    senses:
      darkvision: 60
      passive_perception: 11
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
- id: dnd2024:creature_cockatrice
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
  - turn.ended
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: d374d459c74c5d36f69f93e220f2ee7fa6df13fe991e2a419888fd4f5170d08e
  source_path: builtin/srd-5.2.1/creature/cockatrice.md
  pages:
  - 275
  engine_gaps: &id001
  - dnd2024:creature_cockatrice
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 65c5daaceb2e99e66958fe849ee9088312b05128e4ad851df276efb93227dcd3
splat_clauses:
- key: source_rules
  text: '# Cockatrice Small Monstrosity, Unaligned AC 11 Initiative +1 (11) HP 22 (5d6 + 5) Speed 20 ft.,
    Fly 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 12 +1 +1 Con 12 +1 +1 Int 2 -4 -4 Wis 13 +1
    +1 Cha 5 -3 -3 Immunities Petrified Senses Darkvision 60 ft.; Passive Perception 11 Languages None
    CR 1/2 (XP 100; PB +2) ### Actions Petrifying Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 3 (1d4
    + 1) Piercing damage. If the target is a creature, it is subjected to the following effect. Constitution
    Saving Throw: DC 11. First Failure: The target has the Restrained condition. The target repeats the
    save at the end of its next turn if it is still Restrained, ending the effect on itself on a success.
    Second Failure: The target has the Petrified condition, instead of the Restrained condition, for 24
    hours.'
  sha256: 96a6f941a0e0c057270079e9db3924be0b9e53dbec5b973d1d86219cbdd99e7b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cockatrice

<!-- SRD page 275 -->

Small Monstrosity, Unaligned\
AC 11 Initiative +1 (11)\
HP 22 (5d6 + 5)\
Speed 20 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 12 +1 +1 Con 12 +1 +1\
Int 2 −4 −4 Wis 13 +1 +1 Cha 5 −3 −3\
Immunities Petrified\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Petrifying Bite. Melee Attack Roll: +3, reach 5 ft. Hit:\
3 (1d4 + 1) Piercing damage. If the target is a creature,\
it is subjected to the following effect. Constitution\
Saving Throw: DC 11. First Failure: The target has the\
Restrained condition. The target repeats the save at the\
end of its next turn if it is still Restrained, ending the\
effect on itself on a success. Second Failure: The target\
has the Petrified condition, instead of the Restrained\
condition, for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/275|page 275]].
