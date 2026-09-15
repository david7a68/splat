---
splat_schema: 1
splat_id: 69ba782f-9f7c-5021-b5c8-352cd3785c31
splat_kind: creature
splat_key: dnd2024:creatures/giant_vulture
splat_name: Giant Vulture
splat_origin:
  key: dnd2024:reference/creature/giant_vulture
splat_sources:
- title: SRD 5.2.1
  locator: pp. 354
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Neutral Evil
  movement_text: 10 ft., Fly 60 ft.
  skills_text: Perception +3
  resistances_text: Necrotic
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: Understands Common but can’t speak
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 25 (3d10 + 9)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 10
  max_hp:
    value: 25
  initiative:
    value: 0
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_vulture
  implementation: dnd2024:creature_giant_vulture
  parameters:
    features:
      pack_tactics:
        economy: trait
        operation: passive_rule
        attack_advantage:
          target_kind: creature
          ally_within_target_ft: 5
          ally_excludes_conditions:
          - incapacitated
      gouge:
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
          - average: 9
            roll:
              count: 2
              sides: 6
              modifier: 2
            type: piercing
          conditions:
          - condition: poisoned
            until: target.next_turn_end
    skill_totals:
      perception: 3
    damage_resistances:
    - necrotic
    senses:
      darkvision: 60
      passive_perception: 13
    languages:
      speaks: []
      understands:
      - common
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
- id: dnd2024:creature_giant_vulture
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
  - conditions
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: bce3b8de78b9ebc09f9032259edcaf00fb887f821d90f0f2425dab3b1be2d670
  source_path: builtin/srd-5.2.1/creature/giant_vulture.md
  pages:
  - 354
  engine_gaps: &id001
  - dnd2024:creature_giant_vulture
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: d91c70792fe72b79b963efbf39b201c03ca574821debcd9b3dd7da42b5aa87ea
splat_clauses:
- key: source_rules
  text: '# Giant Vulture Large Monstrosity, Neutral Evil AC 10 Initiative +0 (10) HP 25 (3d10 + 9) Speed
    10 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 10 +0 +0 Con 16 +3 +3 Int 6 -2 -2 Wis
    12 +1 +1 Cha 7 -2 -2 Skills Perception +3 Resistances Necrotic Senses Darkvision 60 ft.; Passive Perception
    13 Languages Understands Common but can’t speak CR 1 (XP 200; PB +2) ### Traits Pack Tactics. The
    vulture has Advantage on an attack roll against a creature if at least one of the vulture’s allies
    is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition. ### Actions
    Gouge. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6 + 2) Piercing damage, and the target has the
    Poisoned condition until the end of its next turn.'
  sha256: 50978fd80de40340b3ec82ee9b3a279be8d2dcbe046bfcd662208e2efa80bd07
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Vulture

<!-- SRD page 354 -->

Large Monstrosity, Neutral Evil\
AC 10 Initiative +0 (10)\
HP 25 (3d10 + 9)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 10 +0 +0 Con 16 +3 +3\
Int 6 −2 −2 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +3\
Resistances Necrotic\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages Understands Common but can’t speak\
CR 1 (XP 200; PB +2)\

### Traits

Pack Tactics. The vulture has Advantage on an attack\
roll against a creature if at least one of the vulture’s al-\
lies is within 5 feet of the creature and the ally doesn’t\
have the Incapacitated condition.\

### Actions

Gouge. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6\
+ 2) Piercing damage, and the target has the Poisoned\
condition until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/354|page 354]].
