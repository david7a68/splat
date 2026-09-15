---
splat_schema: 1
splat_id: 73df5740-aae0-5764-9992-52fca234c83a
splat_kind: creature
splat_key: dnd2024:creatures/hippopotamus
splat_name: Hippopotamus
splat_origin:
  key: dnd2024:reference/creature/hippopotamus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355, 356
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 30 ft.
  skills_text: Perception +3
  senses_text: Passive Perception 13
  languages_text: None
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 82 (11d10 + 22)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 7
  dexterity:
    value: 7
  save_dexterity:
    value: -2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 14
  max_hp:
    value: 82
  initiative:
    value: -2
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hippopotamus
  implementation: dnd2024:creature_hippopotamus
  parameters:
    features:
      hold_breath:
        economy: trait
        operation: passive_rule
        hold_breath_seconds: 600
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
          supplied_attack_total: 7
          reach_ft: 5
        hit:
          damage:
          - average: 16
            roll:
              count: 2
              sides: 10
              modifier: 5
            type: piercing
    skill_totals:
      perception: 3
    senses:
      passive_perception: 13
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
- id: dnd2024:creature_hippopotamus
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
  source_sha256: 3dd7a80b3d1fe8cfff3311a4cd5437e165f08e58870ad6a6b147a11fc14ea266
  source_path: builtin/srd-5.2.1/creature/hippopotamus.md
  pages:
  - 355
  - 356
  engine_gaps: &id001
  - dnd2024:creature_hippopotamus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: b993ef6fd1af7a8332403a5463c36f173ede26a6da6e8ee516305bd5c50a0977
splat_clauses:
- key: source_rules
  text: '# Hippopotamus Large Beast, Unaligned AC 14 Initiative -2 (8) HP 82 (11d10 + 22) Speed 30 ft.,
    Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +7 Dex 7 -2 -2 Con 15 +2 +2 Int 2 -4 -4 Wis 12 +1
    +1 Cha 4 -3 -3 Skills Perception +3 Senses Passive Perception 13 Languages None CR 4 (XP 1,100; PB
    +2) ### Traits Hold Breath. The hippopotamus can hold its breath for 10 minutes. ### Actions Multiattack.
    The hippopotamus makes two Bite attacks. Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 16 (2d10 +
    5) Piercing damage.'
  sha256: 711bce1e53f15a78dbca289042de7c9c41ebf422af03fca9479cce900c62b290
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hippopotamus

<!-- SRD page 355 -->

Large Beast, Unaligned\
AC 14  Initiative −2 (8)\
HP 82 (11d10 + 22)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +7 Dex 7 −2 −2 Con 15 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
Skills Perception +3\
Senses Passive Perception 13\
Languages None\
CR 4 (XP 1,100; PB +2)\

### Traits

Hold Breath. The hippopotamus can hold its breath for\
10 minutes.\

<!-- SRD page 356 -->


### Actions

Multiattack. The hippopotamus makes two Bite attacks.\
Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 16 (2d10 +\
5) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/355|page 355]], [[../../srd-5.2.1/pages/356|page 356]].
