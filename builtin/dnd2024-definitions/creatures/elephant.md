---
splat_schema: 1
splat_id: b630923a-c365-5ee2-9aee-68e460bdffaf
splat_kind: creature
splat_key: dnd2024:creatures/elephant
splat_name: Elephant
splat_origin:
  key: dnd2024:reference/creature/elephant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 348
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 76 (8d12 + 24)
splat_stats:
  strength:
    value: 22
  save_strength:
    value: 6
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 76
  initiative:
    value: -1
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/elephant
  implementation: dnd2024:creature_elephant
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: gore
          count: 2
      gore:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 8
          reach_ft: 5
        hit:
          damage:
          - average: 15
            roll:
              count: 2
              sides: 8
              modifier: 6
            type: piercing
          prone:
            condition: prone
            maximum_size: huge
            requires:
              immediately_preceding_straight_movement_toward_target_ft: 20
      trample:
        economy: bonus_action
        operation: saving_throw
        ability: dexterity
        dc: 16
        targets:
          kind: creature
          count: 1
          range_ft: 5
          required_conditions:
          - prone
        failure:
          damage:
          - average: 17
            roll:
              count: 2
              sides: 10
              modifier: 6
            type: bludgeoning
        success:
          damage_multiplier: 0.5
          other_effects: false
    senses:
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
- id: dnd2024:creature_elephant
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
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 1fdfb95a87eea5105bac82d8b2455e5323ec59529374f2aed2da6563f7561ab3
  source_path: builtin/srd-5.2.1/creature/elephant.md
  pages:
  - 348
  engine_gaps: &id001
  - dnd2024:creature_elephant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 555d88889b50bf8e39f71649e8c3661469c7feb1c92c4df93b446831126441ee
splat_clauses:
- key: source_rules
  text: '# Elephant Huge Beast, Unaligned AC 12 Initiative -1 (9) HP 76 (8d12 + 24) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 22 +6 +6 Dex 9 -1 -1 Con 17 +3 +3 Int 3 -4 -4 Wis 11 +0 +0 Cha 6 -2 -2 Senses
    Passive Perception 10 Languages None CR 4 (XP 1,100; PB +2) ### Actions Multiattack. The elephant
    makes two Gore attacks. Gore. Melee Attack Roll: +8, reach 5 ft. Hit: 15 (2d8 + 6) Piercing damage.
    If the target is a Huge or smaller creature and the elephant moved 20+ feet straight toward it immediately
    before the hit, the target has the Prone condition. ### Bonus Actions Trample. Dexterity Saving Throw:
    DC 16, one creature within 5 feet that has the Prone condition. Failure: 17 (2d10 + 6) Bludgeoning
    damage. Success: Half damage.'
  sha256: 2e655566b9d77cc383ecfb914546d73e883b1212b50def10c316961c418a48bb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Elephant

<!-- SRD page 348 -->

Huge Beast, Unaligned\
AC 12  Initiative −1 (9)\
HP 76 (8d12 + 24)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 9 −1 −1 Con 17 +3 +3\
Int 3 −4 −4 Wis 11 +0 +0 Cha 6 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The elephant makes two Gore attacks.\
Gore. Melee Attack Roll: +8, reach 5 ft. Hit: 15 (2d8\
+ 6) Piercing damage. If the target is a Huge or smaller\
creature and the elephant moved 20+ feet straight to-\
ward it immediately before the hit, the target has the\
Prone condition.\

### Bonus Actions

Trample. Dexterity Saving Throw: DC 16, one creature\
within 5 feet that has the Prone condition. Failure: 17\
(2d10 + 6) Bludgeoning damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/348|page 348]].
