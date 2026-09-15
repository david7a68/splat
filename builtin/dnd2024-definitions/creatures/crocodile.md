---
splat_schema: 1
splat_id: c9f591c5-9263-50d8-89c0-8fbe20db6c14
splat_kind: creature
splat_key: dnd2024:creatures/crocodile
splat_name: Crocodile
splat_origin:
  key: dnd2024:reference/creature/crocodile
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 20 ft., Swim 30 ft.
  skills_text: Stealth +2
  senses_text: Passive Perception 10
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 13 (2d10 + 2)
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
    value: 13
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 13
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/crocodile
  implementation: dnd2024:creature_crocodile
  parameters:
    features:
      hold_breath:
        economy: trait
        operation: passive_rule
        hold_breath_seconds: 3600
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
          - average: 6
            roll:
              count: 1
              sides: 8
              modifier: 2
            type: piercing
          grapple:
            condition: grappled
            source: self
            escape_dc: 12
            maximum_size: medium
            while_grappled:
              conditions:
              - restrained
    skill_totals:
      stealth: 2
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
- id: dnd2024:creature_crocodile
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
  source_sha256: 9caee77751aa5ff5f9edbd730c4ceedec65a607f1ace6b9a5c02461a33b9e17b
  source_path: builtin/srd-5.2.1/creature/crocodile.md
  pages:
  - 347
  engine_gaps: &id001
  - dnd2024:creature_crocodile
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 6fedc25e558c11da134079388df2752f7584b17c5b061132e19a84e1a2656de0
splat_clauses:
- key: source_rules
  text: '# Crocodile Large Beast, Unaligned AC 12 Initiative +0 (10) HP 13 (2d10 + 2) Speed 20 ft., Swim
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 10 +0 +0 Con 13 +1 +3 Int 2 -4 -4 Wis 10 +0 +0
    Cha 5 -3 -3 Skills Stealth +2 Senses Passive Perception 10 Languages None CR 1/2 (XP 100; PB +2) ###
    Traits Hold Breath. The crocodile can hold its breath for 1 hour. ### Actions Bite. Melee Attack Roll:
    +4, reach 5 ft. Hit: 6 (1d8 + 2) Piercing damage. If the target is a Medium or smaller creature, it
    has the Grappled condition (escape DC 12). While Grappled, the target has the Restrained condition.'
  sha256: de3db45c66f88ed39560a40a057a2f0d90900388450d33bd523f09333be6f80f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Crocodile

<!-- SRD page 347 -->

Large Beast, Unaligned\
AC 12  Initiative +0 (10)\
HP 13 (2d10 + 2)\
Speed 20 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 10 +0 +0 Con 13 +1 +3\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Stealth +2\
Senses Passive Perception 10\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Hold Breath. The crocodile can hold its breath\
for 1 hour.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8\
+ 2) Piercing damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 12). While Grappled, the target has the Restrained\
condition.\

Original layout: [[../../srd-5.2.1/pages/347|page 347]].
