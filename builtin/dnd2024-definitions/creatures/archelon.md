---
splat_schema: 1
splat_id: 6a1bb24a-f0ac-53c9-bcc2-e8639b61111b
splat_kind: creature
splat_key: dnd2024:creatures/archelon
splat_name: Archelon
splat_origin:
  key: dnd2024:reference/creature/archelon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 344, 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  tags_text: (Dinosaur)
  movement_text: 20 ft., Swim 80 ft.
  skills_text: Stealth +5
  senses_text: Passive Perception 12
  languages_text: None
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 90 (12d12 + 12)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 4
  save_intelligence:
    value: -3
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 17
  max_hp:
    value: 90
  initiative:
    value: 3
  speed:
    value: 20
    unit: ft
  swim_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/archelon
  implementation: dnd2024:creature_archelon
  parameters:
    features:
      amphibious:
        economy: trait
        operation: passive_rule
        breathing:
        - air
        - water
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
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 14
            roll:
              count: 3
              sides: 6
              modifier: 4
            type: piercing
    skill_totals:
      stealth: 5
    senses:
      passive_perception: 12
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
- id: dnd2024:creature_archelon
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
  source_sha256: 02020258c02294c095ad86d5534dceed551ced91bb2e5bf4c769736585b5ec24
  source_path: builtin/srd-5.2.1/creature/archelon.md
  pages:
  - 344
  - 345
  engine_gaps: &id001
  - dnd2024:creature_archelon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 52882ce4651af3732718e5aa7fe054155b22c12b5160f9904ea064521ea64bd2
splat_clauses:
- key: source_rules
  text: '# Archelon Huge Beast (Dinosaur), Unaligned AC 17 Initiative +3 (13) HP 90 (12d12 + 12) Speed
    20 ft., Swim 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 16 +3 +3 Con 13 +1 +1 Int 4 -3 -3
    Wis 14 +2 +2 Cha 6 -2 -2 Skills Stealth +5 Senses Passive Perception 12 Languages None CR 4 (XP 1,100;
    PB +2) ### Traits Amphibious. The archelon can breathe air and water. ### Actions Multiattack. The
    archelon makes two Bite attacks. Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 14 (3d6 + 4) Piercing
    damage.'
  sha256: 3acec564d9d2c26c7725b12e6178d9b145c5caedce5c9e7751e5cb5fd10c0a2d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Archelon

<!-- SRD page 344 -->

Huge Beast (Dinosaur), Unaligned\
AC 17  Initiative +3 (13)\
HP 90 (12d12 + 12)\
Speed 20 ft., Swim 80 ft.\

<!-- SRD page 345 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 16 +3 +3 Con 13 +1 +1\
Int 4 −3 −3 Wis 14 +2 +2 Cha 6 −2 −2\
Skills Stealth +5\
Senses Passive Perception 12\
Languages None\
CR 4 (XP 1,100; PB +2)\

### Traits

Amphibious. The archelon can breathe air and water.\

### Actions

Multiattack. The archelon makes two Bite attacks.\
Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 14 (3d6 + 4)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/344|page 344]], [[../../srd-5.2.1/pages/345|page 345]].
