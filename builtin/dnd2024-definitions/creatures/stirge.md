---
splat_schema: 1
splat_id: 27b3cb24-91c4-5726-baa2-73389c952cf6
splat_kind: creature
splat_key: dnd2024:creatures/stirge
splat_name: Stirge
splat_origin:
  key: dnd2024:reference/creature/stirge
splat_sources:
- title: SRD 5.2.1
  locator: pp. 329, 330
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: monstrosity
  alignment: Unaligned
  movement_text: 10 ft., Fly 40 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 5 (2d4)
splat_stats:
  strength:
    value: 4
  save_strength:
    value: -3
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 5
  initiative:
    value: 3
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/stirge
  implementation: dnd2024:creature_stirge
  parameters:
    features:
      proboscis:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 6
            roll:
              count: 1
              sides: 6
              modifier: 3
            type: piercing
          attach:
            target: hit_target
            while_attached:
              can_use_proboscis: false
              damage_each_self_turn_start:
              - roll:
                  count: 2
                  sides: 4
                  modifier: 0
                type: necrotic
            detach_self:
              movement_cost_ft: 5
            detach_other:
              economy: action
              actor_any_of:
              - target
              - within_target_ft: 5
    senses:
      darkvision: 60
      passive_perception: 9
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
- id: dnd2024:creature_stirge
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
  - turn.started
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: a6ebfc707f5b7bbd578449435329713f689209e7222670d4462309be7864a7f5
  source_path: builtin/srd-5.2.1/creature/stirge.md
  pages:
  - 329
  - 330
  engine_gaps: &id001
  - dnd2024:creature_stirge
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 81a344d068ad97e20cb6261b585a7b2535be9268f72aa29aad92e112bac612f0
splat_clauses:
- key: source_rules
  text: '# Stirge Tiny Monstrosity, Unaligned AC 13 Initiative +3 (13) HP 5 (2d4) Speed 10 ft., Fly 40
    ft. MOD SAVE MOD SAVE MOD SAVE Str 4 -3 -3 Dex 16 +3 +3 Con 11 +0 +0 Int 2 -4 -4 Wis 8 -1 -1 Cha 6
    -2 -2 Senses Darkvision 60 ft.; Passive Perception 9 Languages None CR 1/8 (XP 25; PB +2) ### Actions
    Proboscis. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Piercing damage, and the stirge attaches
    to the target. While attached, the stirge can’t make Proboscis attacks, and the target takes 5 (2d4)
    Necrotic damage at the start of each of the stirge’s turns. The stirge can detach itself by spending
    5 feet of its movement. The target or a creature within 5 feet of it can detach the stirge as an action.'
  sha256: 4cbc78001cb923a382d3788f05ad0e7f1ae928af1544c7a14f168dba75bb9a66
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stirge

<!-- SRD page 329 -->

Tiny Monstrosity, Unaligned\
AC 13 Initiative +3 (13)\
HP 5 (2d4)\
Speed 10 ft., Fly 40 ft.\

<!-- SRD page 330 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 4 −3 −3 Dex 16 +3 +3 Con 11 +0 +0\
Int 2 −4 −4 Wis 8 −1 −1 Cha 6 −2 −2\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Proboscis. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6\
+ 3) Piercing damage, and the stirge attaches to the\
target. While attached, the stirge can’t make Proboscis\
attacks, and the target takes 5 (2d4) Necrotic damage at\
the start of each of the stirge’s turns.\
The stirge can detach itself by spending 5 feet of its\
movement. The target or a creature within 5 feet of it\
can detach the stirge as an action.\

Original layout: [[../../srd-5.2.1/pages/329|page 329]], [[../../srd-5.2.1/pages/330|page 330]].
