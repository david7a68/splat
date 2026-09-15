---
splat_schema: 1
splat_id: f3b29089-44cc-5d81-b254-7e8304b98d9e
splat_kind: creature
splat_key: dnd2024:creatures/griffon
splat_name: Griffon
splat_origin:
  key: dnd2024:reference/creature/griffon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 295
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Unaligned
  movement_text: 30 ft., Fly 80 ft.
  skills_text: Perception +5
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 59 (7d10 + 21)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 12
  max_hp:
    value: 59
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/griffon
  implementation: dnd2024:creature_griffon
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: rend
          count: 2
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 8
            roll:
              count: 1
              sides: 8
              modifier: 4
            type: piercing
          grapple:
            condition: grappled
            source: self
            escape_dc: 14
            maximum_size: medium
            appendage_pool:
              count: 2
              type: claws
              cost: 2
    skill_totals:
      perception: 5
    senses:
      darkvision: 60
      passive_perception: 15
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
- id: dnd2024:creature_griffon
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
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 876dcb64591f75dd9644b5a7aaccfd50a321c6726c75f3ea61c7ba7027ab27af
  source_path: builtin/srd-5.2.1/creature/griffon.md
  pages:
  - 295
  engine_gaps: &id001
  - dnd2024:creature_griffon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: b435535db55e2d29ec1a128a9435bd35a6bb3c430fa1c63d3701a010feb05fdb
splat_clauses:
- key: source_rules
  text: '# Griffon Large Monstrosity, Unaligned AC 12 Initiative +2 (12) HP 59 (7d10 + 21) Speed 30 ft.,
    Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3 Int 2 -4 -4 Wis 13 +1
    +1 Cha 8 -1 -1 Skills Perception +5 Senses Darkvision 60 ft.; Passive Perception 15 Languages None
    CR 2 (XP 450; PB +2) ### Actions Multiattack. The griffon makes two Rend attacks. Rend. Melee Attack
    Roll: +6, reach 5 ft. Hit: 8 (1d8 + 4) Piercing damage. If the target is a Medium or smaller creature,
    it has the Grappled condition (escape DC 14) from both of the griffon’s front claws.'
  sha256: 8fd07bee3ad9ad04be9b7d6ee1beca951faa985e33faf7f40a8c89e97ddc12ca
  status: contract_specified
splat_requires_mechanics: *id001
---

# Griffon

<!-- SRD page 295 -->

Large Monstrosity, Unaligned\
AC 12 Initiative +2 (12)\
HP 59 (7d10 + 21)\
Speed 30 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 2 −4 −4 Wis 13 +1 +1 Cha 8 −1 −1\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The griffon makes two Rend attacks.\
Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d8 + 4)\
Piercing damage. If the target is a Medium or smaller\
creature, it has the Grappled condition (escape DC 14)\
from both of the griffon’s front claws.\

Original layout: [[../../srd-5.2.1/pages/295|page 295]].
