---
splat_schema: 1
splat_id: 5333b55b-f367-5498-87ce-4043b633e717
splat_kind: creature
splat_key: dnd2024:creatures/mammoth
splat_name: Mammoth
splat_origin:
  key: dnd2024:reference/creature/mammoth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 357
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 126 (11d12 + 55)
splat_stats:
  strength:
    value: 24
  save_strength:
    value: 10
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 21
  save_constitution:
    value: 8
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
    value: 13
  max_hp:
    value: 126
  initiative:
    value: 2
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/mammoth
  implementation: dnd2024:creature_mammoth
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
          supplied_attack_total: 10
          reach_ft: 10
        hit:
          damage:
          - average: 18
            roll:
              count: 2
              sides: 10
              modifier: 7
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
        dc: 18
        targets:
          kind: creature
          count: 1
          range_ft: 5
          required_conditions:
          - prone
        failure:
          damage:
          - average: 29
            roll:
              count: 4
              sides: 10
              modifier: 7
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
- id: dnd2024:creature_mammoth
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
  source_sha256: 0d93fa0f85af7151a945f0abb87781c414b49096b0094e848fdb85d04f3e2e7f
  source_path: builtin/srd-5.2.1/creature/mammoth.md
  pages:
  - 357
  engine_gaps: &id001
  - dnd2024:creature_mammoth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 3cd392922e383b7b1762532de8996998147e700a5b6bf4341f71bb91565b55fb
splat_clauses:
- key: source_rules
  text: '# Mammoth Huge Beast, Unaligned AC 13 Initiative +2 (12) HP 126 (11d12 + 55) Speed 50 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 24 +7 +10 Dex 9 -1 -1 Con 21 +5 +8 Int 3 -4 -4 Wis 11 +0 +0 Cha 6 -2 -2
    Senses Passive Perception 10 Languages None CR 6 (XP 2,300; PB +3) ### Actions Multiattack. The mammoth
    makes two Gore attacks. Gore. Melee Attack Roll: +10, reach 10 ft. Hit: 18 (2d10 + 7) Piercing damage.
    If the target is a Huge or smaller creature and the mammoth moved 20+ feet straight toward it immediately
    before the hit, the target has the Prone condition. ### Bonus Actions Trample. Dexterity Saving Throw:
    DC 18, one creature within 5 feet that has the Prone condition. Failure: 29 (4d10 + 7) Bludgeoning
    damage. Success: Half damage.'
  sha256: 62f0ee3744196325f283d0a3e05c7405e1d68b4628fac55f688ba9d4e6bd43a5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mammoth

<!-- SRD page 357 -->

Huge Beast, Unaligned\
AC 13  Initiative +2 (12)\
HP 126 (11d12 + 55)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  24 +7 +10 Dex 9 −1 −1 Con 21 +5 +8\
Int 3 −4 −4 Wis 11 +0 +0 Cha 6 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The mammoth makes two Gore attacks.\
Gore. Melee Attack Roll: +10, reach 10 ft. Hit: 18 (2d10\
+ 7) Piercing damage. If the target is a Huge or smaller\
creature and the mammoth moved 20+ feet straight\
toward it immediately before the hit, the target has the\
Prone condition.\

### Bonus Actions

Trample. Dexterity Saving Throw: DC 18, one creature\
within 5 feet that has the Prone condition. Failure: 29\
(4d10 + 7) Bludgeoning damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/357|page 357]].
