---
splat_schema: 1
splat_id: 7bd71c86-f783-5366-b46c-79f87f366625
splat_kind: creature
splat_key: dnd2024:creatures/killer_whale
splat_name: Killer Whale
splat_origin:
  key: dnd2024:reference/creature/killer_whale
splat_sources:
- title: SRD 5.2.1
  locator: pp. 356
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Swim 60 ft.
  skills_text: Perception +3, Stealth +4
  senses_text: Blindsight 120 ft.; Passive Perception 13
  languages_text: None
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 90 (12d12 + 12)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 90
  initiative:
    value: 2
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/killer_whale
  implementation: dnd2024:creature_killer_whale
  parameters:
    features:
      hold_breath:
        economy: trait
        operation: passive_rule
        hold_breath_seconds: 1800
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
          - average: 21
            roll:
              count: 5
              sides: 6
              modifier: 4
            type: piercing
    skill_totals:
      perception: 3
      stealth: 4
    senses:
      blindsight: 120
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
- id: dnd2024:creature_killer_whale
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
  source_sha256: 9ded11fa422284feabfdfa12fbd356f87bbe6097e47ca5750183bbe849082629
  source_path: builtin/srd-5.2.1/creature/killer_whale.md
  pages:
  - 356
  engine_gaps: &id001
  - dnd2024:creature_killer_whale
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 410e7da1c5a8c3e201498bdd5cc852d77704d65e3b8c5a0a43758630f5d69ed5
splat_clauses:
- key: source_rules
  text: '# Killer Whale Huge Beast, Unaligned AC 12 Initiative +2 (12) HP 90 (12d12 + 12) Speed 5 ft.,
    Swim 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 14 +2 +2 Con 13 +1 +1 Int 3 -4 -4 Wis 12 +1
    +1 Cha 7 -2 -2 Skills Perception +3, Stealth +4 Senses Blindsight 120 ft.; Passive Perception 13 Languages
    None CR 3 (XP 700; PB +2) ### Traits Hold Breath. The whale can hold its breath for 30 minutes. ###
    Actions Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 21 (5d6 + 4) Piercing damage.'
  sha256: 601bce253b1af919bb7315a4e1899b822e08c53a892e68b11056d983c947e6e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Killer Whale

<!-- SRD page 356 -->

Huge Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 90 (12d12 + 12)\
Speed 5 ft., Swim 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +2 Con 13 +1 +1\
Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +3, Stealth +4\
Senses Blindsight 120 ft.; Passive Perception 13\
Languages None\
CR 3 (XP 700; PB +2)\

### Traits

Hold Breath. The whale can hold its breath for\
30 minutes.\

### Actions

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 21 (5d6\
+ 4) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/356|page 356]].
