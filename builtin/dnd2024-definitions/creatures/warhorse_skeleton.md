---
splat_schema: 1
splat_id: 6dc83845-ae97-5f42-8cd6-657735b997a0
splat_kind: creature
splat_key: dnd2024:creatures/warhorse_skeleton
splat_name: Warhorse Skeleton
splat_origin:
  key: dnd2024:reference/creature/warhorse_skeleton
splat_sources:
- title: SRD 5.2.1
  locator: pp. 326
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: undead
  alignment: Lawful Evil
  movement_text: 60 ft.
  vulnerabilities_text: Bludgeoning
  immunities_text: Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 22 (3d10 + 6)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 22
  initiative:
    value: 1
  speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/warhorse_skeleton
  implementation: dnd2024:creature_warhorse_skeleton
  parameters:
    features:
      hooves:
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
          - average: 7
            roll:
              count: 1
              sides: 6
              modifier: 4
            type: bludgeoning
          prone:
            condition: prone
            maximum_size: large
            requires:
              immediately_preceding_straight_movement_toward_target_ft: 20
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
- id: dnd2024:creature_warhorse_skeleton
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
  source_sha256: edc8c476a22d4f3e0717ccf3b3d4b87b85ec7be46e406ff799a0763e1b7d8955
  source_path: builtin/srd-5.2.1/creature/warhorse_skeleton.md
  pages:
  - 326
  engine_gaps: &id001
  - dnd2024:creature_warhorse_skeleton
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 941e01df2d2907e5e9af84794e3678cdaeffd0493894681c23a6e7a43d77f4e5
splat_clauses:
- key: source_rules
  text: '# Warhorse Skeleton Large Undead, Lawful Evil AC 13 Initiative +1 (11) HP 22 (3d10 + 6) Speed
    60 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 12 +1 +1 Con 15 +2 +2 Int 2 -4 -4 Wis 8 -1 -1 Cha
    5 -3 -3 Vulnerabilities Bludgeoning Immunities Poison; Exhaustion, Poisoned Senses Darkvision 60 ft.;
    Passive Perception 9 Languages None CR 1/2 (XP 100; PB +2) ### Actions Hooves. Melee Attack Roll:
    +6, reach 5 ft. Hit: 7 (1d6 + 4) Bludgeoning damage. If the target is a Large or smaller creature
    and the skeleton moved 20+ feet straight toward it immediately before the hit, the target has the
    Prone condition.'
  sha256: 6a3c9bda353946dd5758ee53e90e254dd3e927fb675140f5d8c4bbafc3e534e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warhorse Skeleton

<!-- SRD page 326 -->

Large Undead, Lawful Evil\
AC 13 Initiative +1 (11)\
HP 22 (3d10 + 6)\
Speed 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 12 +1 +1 Con 15 +2 +2\
Int 2 −4 −4 Wis 8 −1 −1 Cha 5 −3 −3\
Vulnerabilities Bludgeoning\
Immunities Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 7 (1d6\
+ 4) Bludgeoning damage. If the target is a Large or\
smaller creature and the skeleton moved 20+ feet\
straight toward it immediately before the hit, the target\
has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/326|page 326]].
