---
splat_schema: 1
splat_id: 288eb685-d6ab-5bec-8aa0-b56547a5433a
splat_kind: creature
splat_key: dnd2024:creatures/ghoul
splat_name: Ghoul
splat_origin:
  key: dnd2024:reference/creature/ghoul
splat_sources:
- title: SRD 5.2.1
  locator: pp. 288
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Chaotic Evil
  movement_text: 30 ft.
  immunities_text: Poison; Charmed, Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Common
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (5d8)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 22
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ghoul
  implementation: dnd2024:creature_ghoul
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: bite
          count: 2
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 6
              modifier: 2
            type: piercing
          - average: 3
            roll:
              count: 1
              sides: 6
              modifier: 0
            type: necrotic
      claw:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 4
            roll:
              count: 1
              sides: 4
              modifier: 2
            type: slashing
          save:
            operation: saving_throw
            ability: constitution
            dc: 10
            targets:
              kind: creature
              exclude:
              - undead
              - elf
            failure:
              conditions:
              - condition: paralyzed
                until: target.next_turn_end
    damage_immunities:
    - poison
    condition_immunities:
    - charmed
    - exhaustion
    - poisoned
    senses:
      darkvision: 60
      passive_perception: 10
    languages:
      speaks:
      - common
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
- id: dnd2024:creature_ghoul
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
  source_sha256: 8a8b46d08aff8aa3059c548eaf2a8e6f813eb0bbe1d251fa0744750ef359e4af
  source_path: builtin/srd-5.2.1/creature/ghoul.md
  pages:
  - 288
  engine_gaps: &id001
  - dnd2024:creature_ghoul
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: aab4e2b564fcabdc94b7edbf348918cccdc153c5b43080efdb78435a33a03216
splat_clauses:
- key: source_rules
  text: '# Ghoul Medium Undead, Chaotic Evil AC 12 Initiative +2 (12) HP 22 (5d8) Speed 30 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 13 +1 +1 Dex 15 +2 +2 Con 10 +0 +0 Int 7 -2 -2 Wis 10 +0 +0 Cha 6 -2 -2 Immunities
    Poison; Charmed, Exhaustion, Poisoned Senses Darkvision 60 ft.; Passive Perception 10 Languages Common
    CR 1 (XP 200; PB +2) ### Actions Multiattack. The ghoul makes two Bite attacks. Bite. Melee Attack
    Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage plus 3 (1d6) Necrotic damage. Claw. Melee Attack
    Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Slashing damage. If the target is a creature that isn’t an
    Undead or elf, it is subjected to the following effect. Constitution Saving Throw: DC 10. Failure:
    The target has the Paralyzed condition until the end of its next turn.'
  sha256: 07858cba079769ac5b91e48c94e6252d8c393ba17c208999ebe2051ba91d11aa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ghoul

<!-- SRD page 288 -->

Medium Undead, Chaotic Evil\
AC 12 Initiative +2 (12)\
HP 22 (5d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 15 +2 +2 Con 10 +0 +0\
Int 7 −2 −2 Wis 10 +0 +0 Cha 6 −2 −2\
Immunities Poison; Charmed, Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Common\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The ghoul makes two Bite attacks.\
Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage plus 3 (1d6) Necrotic damage.\
Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4\
+ 2) Slashing damage. If the target is a creature that\
isn’t an Undead or elf, it is subjected to the following\
effect. Constitution Saving Throw: DC 10. Failure: The\
target has the Paralyzed condition until the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/288|page 288]].
