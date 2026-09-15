---
splat_schema: 1
splat_id: 42bc1db8-12d7-57b5-893a-04f13cdf074e
splat_kind: creature
splat_key: dnd2024:creatures/saber_toothed_tiger
splat_name: Saber-Toothed Tiger
splat_origin:
  key: dnd2024:reference/creature/saber_toothed_tiger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 360
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  skills_text: Perception +5, Stealth +7
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 52 (7d10 + 14)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 6
  dexterity:
    value: 17
  save_dexterity:
    value: 5
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 52
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/saber_toothed_tiger
  implementation: dnd2024:creature_saber_toothed_tiger
  parameters:
    features:
      running_leap:
        economy: trait
        operation: passive_rule
        running_start_ft: 10
        long_jump_ft: 25
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
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: slashing
      nimble_escape:
        economy: bonus_action
        operation: choose_action
        options:
        - disengage
        - hide
        count: 1
    skill_totals:
      perception: 5
      stealth: 7
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
- id: dnd2024:creature_saber_toothed_tiger
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
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 55ae811c5b6048ed54e75f353d966264cb23f12ebc43e7fa9416daf29a5df08d
  source_path: builtin/srd-5.2.1/creature/saber_toothed_tiger.md
  pages:
  - 360
  engine_gaps: &id001
  - dnd2024:creature_saber_toothed_tiger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 459079c240cea45f440e3776ce3cf0cac663db551fe5870196b1175b8a00a56c
splat_clauses:
- key: source_rules
  text: '# Saber-Toothed Tiger Large Beast, Unaligned AC 13 Initiative +3 (13) HP 52 (7d10 + 14) Speed
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +6 Dex 17 +3 +5 Con 15 +2 +2 Int 3 -4 -4 Wis 12 +1 +1
    Cha 8 -1 -1 Skills Perception +5, Stealth +7 Senses Darkvision 60 ft.; Passive Perception 15 Languages
    None CR 2 (XP 450; PB +2) ### Traits Running Leap. With a 10-foot running start, the tiger can Long
    Jump up to 25 feet. ### Actions Multiattack. The tiger makes two Rend attacks. Rend. Melee Attack
    Roll: +6, reach 5 ft. Hit: 11 (2d6 + 4) Slashing damage. ### Bonus Actions Nimble Escape. The tiger
    takes the Disengage or Hide action.'
  sha256: ca0a81fc18742d3b5b0d09d1e78c7860ea9a2cf55be9747caa041f510c637ac7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Saber-Toothed Tiger

<!-- SRD page 360 -->

Large Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 52 (7d10 + 14)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +6 Dex 17 +3 +5 Con 15 +2 +2\
Int 3 −4 −4 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +5, Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Running Leap. With a 10-foot running start, the tiger\
can Long Jump up to 25 feet.\

### Actions

Multiattack. The tiger makes two Rend attacks.\
Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 11 (2d6\
+ 4) Slashing damage.\

### Bonus Actions

Nimble Escape. The tiger takes the Disengage or Hide\
action.\

Original layout: [[../../srd-5.2.1/pages/360|page 360]].
