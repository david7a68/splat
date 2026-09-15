---
splat_schema: 1
splat_id: 7c7d3bae-a1a3-5189-a524-3966385eced8
splat_kind: creature
splat_key: dnd2024:creatures/shambling_mound
splat_name: Shambling Mound
splat_origin:
  key: dnd2024:reference/creature/shambling_mound
splat_sources:
- title: SRD 5.2.1
  locator: pp. 323
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: plant
  alignment: Unaligned
  movement_text: 30 ft., Swim 20 ft.
  skills_text: Stealth +3
  resistances_text: Cold, Fire
  immunities_text: Lightning; Deafened, Exhaustion
  senses_text: Blindsight 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 110 (13d10 + 39)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 15
  max_hp:
    value: 110
  initiative:
    value: -1
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/shambling_mound
  implementation: dnd2024:creature_shambling_mound
  parameters:
    features:
      lightning_absorption:
        economy: trait
        operation: passive_rule
        trigger: self.subjected_to_lightning_damage
        heal:
          equal: lightning_damage_dealt
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: charged_tendril
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: engulf
      charged_tendril:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 7
          reach_ft: 10
        hit:
          damage:
          - average: 7
            roll:
              count: 1
              sides: 6
              modifier: 4
            type: bludgeoning
          - average: 5
            roll:
              count: 2
              sides: 4
              modifier: 0
            type: lightning
          pull:
            maximum_ft: 5
            direction: straight_toward_self
            maximum_size: medium
      engulf:
        economy: action
        operation: saving_throw
        ability: strength
        dc: 15
        targets:
          kind: creature
          count: 1
          range_ft: 5
          maximum_size: medium
        failure:
          move_target_to: self.space
          grapple:
            condition: grappled
            source: self
            escape_dc: 14
            while_grappled:
              conditions:
              - blinded
              - restrained
              damage_each_target_turn_start:
              - roll:
                  count: 3
                  sides: 6
                  modifier: 0
                type: lightning
              moves_with_self: true
              movement_extra_cost: 0
        capacity: 1
        success:
          effects: []
    skill_totals:
      stealth: 3
    damage_resistances:
    - cold
    - fire
    damage_immunities:
    - lightning
    condition_immunities:
    - deafened
    - exhaustion
    senses:
      blindsight: 60
      passive_perception: 10
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
- id: dnd2024:creature_shambling_mound
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
  - saving_throws
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 74a6f58a989cb367c71316b812a517d4a5adcacf392abef7061ecb4c93d08b7b
  source_path: builtin/srd-5.2.1/creature/shambling_mound.md
  pages:
  - 323
  engine_gaps: &id001
  - dnd2024:creature_shambling_mound
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 31b8a6f14abc2824effdc1f1b0f8d05dc8a12414148a638c6c387230792247f9
splat_clauses:
- key: source_rules
  text: '# Shambling Mound Large Plant, Unaligned AC 15 Initiative -1 (9) HP 110 (13d10 + 39) Speed 30
    ft., Swim 20 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 8 -1 -1 Con 16 +3 +3 Int 5 -3 -3 Wis
    10 +0 +0 Cha 5 -3 -3 Skills Stealth +3 Resistances Cold, Fire Immunities Lightning; Deafened, Exhaustion
    Senses Blindsight 60 ft.; Passive Perception 10 Languages None CR 5 (XP 1,800; PB +3) ### Traits Lightning
    Absorption. Whenever the shambling mound is subjected to Lightning damage, it regains a number of
    Hit Points equal to the Lightning damage dealt. ### Actions Multiattack. The shambling mound makes
    three Charged Tendril attacks. It can replace one attack with a use of Engulf. Charged Tendril. Melee
    Attack Roll: +7, reach 10 ft. Hit: 7 (1d6 + 4) Bludgeoning damage plus 5 (2d4) Lightning damage. If
    the target is a Medium or smaller creature, the shambling mound pulls the target 5 feet straight toward
    itself. Engulf. Strength Saving Throw: DC 15, one Medium or smaller creature within 5 feet. Failure:
    The target is pulled into the shambling mound’s space and has the Grappled condition (escape DC 14).
    Until the grapple ends, the target has the Blinded and Restrained conditions, and it takes 10 (3d6)
    Lightning damage at the start of each of its turns. When the shambling mound moves, the Grappled target
    moves with it, costing it no extra movement. The shambling mound can have only one creature Grappled
    by this action at a time.'
  sha256: a871cb87837967b5a34c9a7bc9395b0ff09a80082aff507cc6e5b64b7110e500
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shambling Mound

<!-- SRD page 323 -->

Large Plant, Unaligned\
AC 15  Initiative −1 (9)\
HP 110 (13d10 + 39)\
Speed 30 ft., Swim 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 8 −1 −1 Con 16 +3 +3\
Int 5 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Stealth +3\
Resistances Cold, Fire\
Immunities Lightning; Deafened, Exhaustion\
Senses Blindsight 60 ft.; Passive Perception 10\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Lightning Absorption. Whenever the shambling mound\
is subjected to Lightning damage, it regains a number of\
Hit Points equal to the Lightning damage dealt.\

### Actions

Multiattack. The shambling mound makes three\
Charged Tendril attacks. It can replace one attack with\
a use of Engulf.\
Charged Tendril. Melee Attack Roll: +7, reach 10 ft.\
Hit: 7 (1d6 + 4) Bludgeoning damage plus 5 (2d4)\
Lightning damage. If the target is a Medium or smaller\
creature, the shambling mound pulls the target 5 feet\
straight toward itself.\
Engulf. Strength Saving Throw: DC 15, one Medium\
or smaller creature within 5 feet. Failure: The target is\
pulled into the shambling mound’s space and has the\
Grappled condition (escape DC 14). Until the grapple\
ends, the target has the Blinded and Restrained con-\
ditions, and it takes 10 (3d6) Lightning damage at the\
start of each of its turns. When the shambling mound\
moves, the Grappled target moves with it, costing it no\
extra movement. The shambling mound can have only\
one creature Grappled by this action at a time.\

Original layout: [[../../srd-5.2.1/pages/323|page 323]].
