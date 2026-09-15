---
splat_schema: 1
splat_id: 7a50a965-93f4-58ea-b359-afca41a8f7e5
splat_kind: creature
splat_key: dnd2024:creatures/minotaur_skeleton
splat_name: Minotaur Skeleton
splat_origin:
  key: dnd2024:reference/creature/minotaur_skeleton
splat_sources:
- title: SRD 5.2.1
  locator: pp. 326
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: undead
  alignment: Lawful Evil
  movement_text: 40 ft.
  vulnerabilities_text: Bludgeoning
  immunities_text: Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: Understands Abyssal but can’t speak
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 45
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/minotaur_skeleton
  implementation: dnd2024:creature_minotaur_skeleton
  parameters:
    features:
      gore:
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
            type: piercing
          prone:
            condition: prone
            maximum_size: large
            requires:
              immediately_preceding_straight_movement_toward_target_ft: 20
          conditional_damage:
            requires:
              maximum_target_size: large
              immediately_preceding_straight_movement_toward_target_ft: 20
            damage:
            - roll:
                count: 2
                sides: 8
                modifier: 0
              type: piercing
      slam:
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
          - average: 15
            roll:
              count: 2
              sides: 10
              modifier: 4
            type: bludgeoning
    damage_vulnerabilities:
    - bludgeoning
    damage_immunities:
    - poison
    condition_immunities:
    - exhaustion
    - poisoned
    senses:
      darkvision: 60
      passive_perception: 9
    languages:
      speaks: []
      understands:
      - abyssal
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
- id: dnd2024:creature_minotaur_skeleton
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
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: ad36f9cc99b7f29d473a154b52077dfac0972f94ccb63ea3fc82ce7296215ed2
  source_path: builtin/srd-5.2.1/creature/minotaur_skeleton.md
  pages:
  - 326
  engine_gaps: &id001
  - dnd2024:creature_minotaur_skeleton
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 574dabe9fd9ebf10c93015ee95194024a6852832788142579349332b61db45d2
splat_clauses:
- key: source_rules
  text: '# Minotaur Skeleton Large Undead, Lawful Evil AC 12 Initiative +0 (10) HP 45 (6d10 + 12) Speed
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 11 +0 +0 Con 15 +2 +2 Int 6 -2 -2 Wis 8 -1 -1 Cha
    5 -3 -3 Vulnerabilities Bludgeoning Immunities Poison; Exhaustion, Poisoned Senses Darkvision 60 ft.;
    Passive Perception 9 Languages Understands Abyssal but can’t speak CR 2 (XP 450; PB +2) ### Actions
    Gore. Melee Attack Roll: +6, reach 5 ft. Hit: 11 (2d6 + 4) Piercing damage. If the target is a Large
    or smaller creature and the skeleton moved 20+ feet straight toward it immediately before the hit,
    the target takes an extra 9 (2d8) Piercing damage and has the Prone condition. Slam. Melee Attack
    Roll: +6, reach 5 ft. Hit: 15 (2d10 + 4) Bludgeoning damage.'
  sha256: a69986c871f91e7c4fac315a6528d66703c1ad39e553c6d43c975b9335f5bdfb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Minotaur Skeleton

<!-- SRD page 326 -->

Large Undead, Lawful Evil\
AC 12 Initiative +0 (10)\
HP 45 (6d10 + 12)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 11 +0 +0 Con 15 +2 +2\
Int 6 −2 −2 Wis 8 −1 −1 Cha 5 −3 −3\
Vulnerabilities Bludgeoning\
Immunities Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Understands Abyssal but can’t speak\
CR 2 (XP 450; PB +2)\

### Actions

Gore. Melee Attack Roll: +6, reach 5 ft. Hit: 11 (2d6\
+ 4) Piercing damage. If the target is a Large or smaller\
creature and the skeleton moved 20+ feet straight\
toward it immediately before the hit, the target takes\
an extra 9 (2d8) Piercing damage and has the Prone\
condition.\
Slam. Melee Attack Roll: +6, reach 5 ft. Hit: 15 (2d10\
+ 4) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/326|page 326]].
