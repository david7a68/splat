---
splat_schema: 1
splat_id: e7f52e82-0918-5d23-8141-f95aaa5462ef
splat_kind: creature
splat_key: dnd2024:creatures/giant_scorpion
splat_name: Giant Scorpion
splat_origin:
  key: dnd2024:reference/creature/giant_scorpion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 353
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Blindsight 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 52 (7d10 + 14)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 9
  save_wisdom:
    value: -1
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 15
  max_hp:
    value: 52
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_scorpion
  implementation: dnd2024:creature_giant_scorpion
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: claw
          count: 2
        - operation: attack_sequence
          feature: sting
          count: 1
      claw:
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
            type: bludgeoning
          grapple:
            condition: grappled
            source: self
            escape_dc: 13
            maximum_size: large
            appendage_pool:
              count: 2
              type: claws
              cost: 1
      sting:
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
          - average: 7
            roll:
              count: 1
              sides: 8
              modifier: 3
            type: piercing
          - average: 11
            roll:
              count: 2
              sides: 10
              modifier: 0
            type: poison
    senses:
      blindsight: 60
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
- id: dnd2024:creature_giant_scorpion
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
  source_sha256: 9060ede350a088ac095484f5e24cb2d02899f309ad25d3b866f5f129b2e7c715
  source_path: builtin/srd-5.2.1/creature/giant_scorpion.md
  pages:
  - 353
  engine_gaps: &id001
  - dnd2024:creature_giant_scorpion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 87013a383e8a0aec5fb83e55d88d1528cfcda02d539d73dadfe455025a722983
splat_clauses:
- key: source_rules
  text: '# Giant Scorpion Large Beast, Unaligned AC 15 Initiative +1 (11) HP 52 (7d10 + 14) Speed 40 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 13 +1 +1 Con 15 +2 +2 Int 1 -5 -5 Wis 9 -1 -1 Cha 3 -4
    -4 Senses Blindsight 60 ft.; Passive Perception 9 Languages None CR 3 (XP 700; PB +2) ### Actions
    Multiattack. The scorpion makes two Claw attacks and one Sting attack. Claw. Melee Attack Roll: +5,
    reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage. If the target is a Large or smaller creature, it
    has the Grappled condition (escape DC 13) from one of two claws. Sting. Melee Attack Roll: +5, reach
    5 ft. Hit: 7 (1d8 + 3) Piercing damage plus 11 (2d10) Poison damage.'
  sha256: cd435b39a91a17a52924b9256f53822d235e177d30e889920145ec0c6b1ea97a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Scorpion

<!-- SRD page 353 -->

Large Beast, Unaligned\
AC 15  Initiative +1 (11)\
HP 52 (7d10 + 14)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 15 +2 +2\
Int 1 −5 −5 Wis 9 −1 −1 Cha 3 −4 −4\
Senses Blindsight 60 ft.; Passive Perception 9\
Languages None\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The scorpion makes two Claw attacks and\
one Sting attack.\
Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3)\
Bludgeoning damage. If the target is a Large or smaller\
creature, it has the Grappled condition (escape DC 13)\
from one of two claws.\
Sting. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage plus 11 (2d10) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/353|page 353]].
