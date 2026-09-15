---
splat_schema: 1
splat_id: b0db3fcd-7eeb-57af-8bfc-24f15e948cb0
splat_kind: creature
splat_key: dnd2024:creatures/bugbear_stalker
splat_name: Bugbear Stalker
splat_origin:
  key: dnd2024:reference/creature/bugbear_stalker
splat_sources:
- title: SRD 5.2.1
  locator: pp. 271
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Chaotic Evil
  tags_text: (Goblinoid)
  movement_text: 30 ft.
  skills_text: Stealth +6, Survival +3
  gear_text: Chain Shirt, Javelins (6), Morningstar
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: Common, Goblin
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 65 (10d8 + 20)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 14
  save_constitution:
    value: 4
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 3
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 15
  max_hp:
    value: 65
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/bugbear_stalker
  implementation: dnd2024:creature_bugbear_stalker
  parameters:
    features:
      abduct:
        economy: trait
        operation: passive_rule
        grappled_creature_movement_extra_cost: 0
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - javelin
          - morningstar
      javelin:
        economy: action
        operation: attack
        attack:
          mode: melee_or_ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 10
          range_ft: 30
          long_range_ft: 120
        hit:
          damage:
          - average: 13
            roll:
              count: 3
              sides: 6
              modifier: 3
            type: piercing
      morningstar:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 10
          advantage_if:
            target_grappled_by: self
        hit:
          damage:
          - average: 12
            roll:
              count: 2
              sides: 8
              modifier: 3
            type: piercing
      quick_grapple:
        economy: bonus_action
        operation: saving_throw
        ability: dexterity
        dc: 13
        targets:
          kind: creature
          count: 1
          range_ft: 10
          visible: true
          maximum_size: medium
        failure:
          grapple:
            condition: grappled
            source: self
            escape_dc: 13
        success:
          effects: []
    skill_totals:
      stealth: 6
      survival: 3
    gear:
    - item: chain_shirt
      quantity: 1
    - item: javelins
      quantity: 6
    - item: morningstar
      quantity: 1
    senses:
      darkvision: 60
      passive_perception: 11
    languages:
      speaks:
      - common
      - goblin
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
- id: dnd2024:creature_bugbear_stalker
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
  - grappling
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6057eba685a118dca0fc6f7bdd1f4ce4912d8c2b9558f96defbc10c576346a6d
  source_path: builtin/srd-5.2.1/creature/bugbear_stalker.md
  pages:
  - 271
  engine_gaps: &id001
  - dnd2024:creature_bugbear_stalker
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 966c10e2b0f8fb4572216cf88f991b64143436f48d93cdf0104e778fdfd63062
splat_clauses:
- key: source_rules
  text: '# Bugbear Stalker Medium Fey (Goblinoid), Chaotic Evil AC 15 Initiative +2 (12) HP 65 (10d8 +
    20) Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 14 +2 +2 Con 14 +2 +4 Int 11 +0 +0 Wis
    12 +1 +3 Cha 11 +0 +0 Skills Stealth +6, Survival +3 Gear Chain Shirt, Javelins (6), Morningstar Senses
    Darkvision 60 ft.; Passive Perception 11 Languages Common, Goblin CR 3 (XP 700; PB +2) ### Traits
    Abduct. The bugbear needn’t spend extra movement to move a creature it is grappling. ### Actions Multiattack.
    The bugbear makes two Javelin or Morningstar attacks. Javelin. Melee or Ranged Attack Roll: +5, reach
    10 ft. or range 30/120 ft. Hit: 13 (3d6 + 3) Piercing damage. Morningstar. Melee Attack Roll: +5 (with
    Advantage if the target is Grappled by the bugbear), reach 10 ft. Hit: 12 (2d8 + 3) Piercing damage.
    ### Bonus Actions Quick Grapple. Dexterity Saving Throw: DC 13, one Medium or smaller creature the
    bugbear can see within 10 feet. Failure: The target has the Grappled condition (escape DC 13).'
  sha256: 0e3e6c844bc8e3c6c80c0444b0cc5d076fa0a572bc95354763228ed9757a2b9f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bugbear Stalker

<!-- SRD page 271 -->

Medium Fey (Goblinoid), Chaotic Evil\
AC 15 Initiative +2 (12)\
HP 65 (10d8 + 20)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 14 +2 +2 Con 14 +2 +4\
Int 11 +0 +0 Wis 12 +1 +3 Cha 11 +0 +0\
Skills Stealth +6, Survival +3\
Gear Chain Shirt, Javelins (6), Morningstar\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common, Goblin\
CR 3 (XP 700; PB +2)\

### Traits

Abduct. The bugbear needn’t spend extra movement to\
move a creature it is grappling.\

### Actions

Multiattack. The bugbear makes two Javelin or Morn-\
ingstar attacks.\
Javelin. Melee or Ranged Attack Roll: +5, reach 10 ft.\
or range 30/120 ft. Hit: 13 (3d6 + 3) Piercing damage.\
Morningstar. Melee Attack Roll: +5 (with Advantage if\
the target is Grappled by the bugbear), reach 10 ft. Hit:\
12 (2d8 + 3) Piercing damage.\

### Bonus Actions

Quick Grapple. Dexterity Saving Throw: DC 13, one\
Medium or smaller creature the bugbear can see within\
10 feet. Failure: The target has the Grappled condition\
(escape DC 13).\

Original layout: [[../../srd-5.2.1/pages/271|page 271]].
