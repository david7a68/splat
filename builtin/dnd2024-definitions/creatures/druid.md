---
splat_schema: 1
splat_id: e9b6ff57-018a-5931-b144-c9338a133035
splat_kind: creature
splat_key: dnd2024:creatures/druid
splat_name: Druid
splat_origin:
  key: dnd2024:reference/creature/druid
splat_sources:
- title: SRD 5.2.1
  locator: pp. 282
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  tags_text: (Druid)
  movement_text: 30 ft.
  skills_text: Medicine +5, Nature +3, Perception +5
  gear_text: Studded Leather Armor
  senses_text: Passive Perception 15
  languages_text: Common, Druidic, Sylvan
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 44 (8d8 + 8)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 44
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/druid
  implementation: dnd2024:creature_druid
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - vine_staff
          - verdant_wisp
      vine_staff:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 7
            roll:
              count: 1
              sides: 8
              modifier: 3
            type: bludgeoning
          - average: 2
            roll:
              count: 1
              sides: 4
              modifier: 0
            type: poison
      verdant_wisp:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          range_ft: 90
          long_range_ft: null
        hit:
          damage:
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: radiant
      spellcasting:
        economy: action
        operation: spellcasting
        ability: wisdom
        components_waived: []
        save_dc: 13
        options:
        - spell:
            key: dnd2024:spells/druidcraft
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/speak_with_animals
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/entangle
          usage:
            kind: daily
            uses: 2
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/thunderwave
          usage:
            kind: daily
            uses: 2
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/animal_messenger
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/longstrider
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/moonbeam
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        material_components_available: true
    skill_totals:
      medicine: 5
      nature: 3
      perception: 5
    gear:
    - item: studded_leather_armor
      quantity: 1
    senses:
      passive_perception: 15
    languages:
      speaks:
      - common
      - druidic
      - sylvan
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
- id: dnd2024:creature_druid
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
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - damage
  - spellcasting
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6d58623b1a162688500b0967485cf5dbad3ad980d75ed366e8e3f862a945f6b1
  source_path: builtin/srd-5.2.1/creature/druid.md
  pages:
  - 282
  engine_gaps: &id001
  - dnd2024:creature_druid
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 08e19af72e5fe6698d2d3114e9e6c859e7dae0a96ab0b621c5325c75dbfd752a
splat_clauses:
- key: source_rules
  text: '# Druid Medium or Small Humanoid (Druid), Neutral AC 13 Initiative +1 (11) HP 44 (8d8 + 8) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 12 +1 +1 Con 13 +1 +1 Int 12 +1 +1 Wis 16 +3 +3
    Cha 11 +0 +0 Skills Medicine +5, Nature +3, Perception +5 Gear Studded Leather Armor Senses Passive
    Perception 15 Languages Common, Druidic, Sylvan CR 2 (XP 450; PB +2) ### Actions Multiattack. The
    druid makes two attacks, using Vine Staff or Verdant Wisp in any combination. Vine Staff. Melee Attack
    Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Bludgeoning damage plus 2 (1d4) Poison damage. Verdant Wisp.
    Ranged Attack Roll: +5, range 90 ft. Hit: 10 (3d6) Radiant damage. Spellcasting. The druid casts one
    of the following spells, using Wisdom as the spellcasting ability (spell save DC 13): At Will: Druidcraft,
    Speak with Animals 2/Day Each: Entangle, Thunderwave 1/Day Each: Animal Messenger, Longstrider, Moonbeam'
  sha256: 252a5c28932a2374f3b7210a43ad1deb690159f33e60a4658cb002b89daa614a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Druid

<!-- SRD page 282 -->

Medium or Small Humanoid (Druid), Neutral\
AC 13 Initiative +1 (11)\
HP 44 (8d8 + 8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 12 +1 +1 Con 13 +1 +1\
Int 12 +1 +1 Wis 16 +3 +3 Cha 11 +0 +0\
Skills Medicine +5, Nature +3, Perception +5\
Gear Studded Leather Armor\
Senses Passive Perception 15\
Languages Common, Druidic, Sylvan\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The druid makes two attacks, using Vine\
Staff or Verdant Wisp in any combination.\
Vine Staff. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8\
+ 3) Bludgeoning damage plus 2 (1d4) Poison damage.\
Verdant Wisp. Ranged Attack Roll: +5, range 90 ft. Hit:\
10 (3d6) Radiant damage.\
Spellcasting. The druid casts one of the following\
spells, using Wisdom as the spellcasting ability (spell\
save DC 13):\
At Will: Druidcraft, Speak with Animals\
2/Day Each: Entangle, Thunderwave\
1/Day Each: Animal Messenger, Long-\
strider, Moonbeam\

Original layout: [[../../srd-5.2.1/pages/282|page 282]].
