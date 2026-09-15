---
splat_schema: 1
splat_id: 8cff2e8f-1067-58f4-ac6f-975eedd92bdd
splat_kind: creature
splat_key: dnd2024:creatures/merrow
splat_name: Merrow
splat_origin:
  key: dnd2024:reference/creature/merrow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 308
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Chaotic Evil
  movement_text: 10 ft., Swim 40 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Abyssal, Primordial (Aquan)
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 45
  initiative:
    value: 2
  speed:
    value: 10
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/merrow
  implementation: dnd2024:creature_merrow
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
        - operation: attack_choice
          count: 2
          options:
          - bite
          - claw
          - harpoon
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
          - average: 6
            roll:
              count: 1
              sides: 4
              modifier: 4
            type: piercing
          conditions:
          - condition: poisoned
            until: self.next_turn_end
      claw:
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
          - average: 9
            roll:
              count: 2
              sides: 4
              modifier: 4
            type: slashing
      harpoon:
        economy: action
        operation: attack
        attack:
          mode: melee_or_ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
          range_ft: 20
          long_range_ft: 60
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: piercing
          pull:
            maximum_ft: 15
            direction: straight_toward_self
            maximum_size: large
    senses:
      darkvision: 60
      passive_perception: 10
    languages:
      speaks:
      - abyssal
      - language: primordial
        dialects:
        - aquan
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
- id: dnd2024:creature_merrow
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
  source_sha256: 6eb113b18e3149ca0a2b9d258074f95bd4151d992a4b324d2225f35510bc20ee
  source_path: builtin/srd-5.2.1/creature/merrow.md
  pages:
  - 308
  engine_gaps: &id001
  - dnd2024:creature_merrow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 8b9a94763c2c2fbc6308f803188be34075f901e27dce48f5c612efa88222efc8
splat_clauses:
- key: source_rules
  text: '# Merrow Large Monstrosity, Chaotic Evil AC 13 Initiative +2 (12) HP 45 (6d10 + 12) Speed 10
    ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +2 Con 15 +2 +2 Int 8 -1 -1 Wis
    10 +0 +0 Cha 9 -1 -1 Senses Darkvision 60 ft.; Passive Perception 10 Languages Abyssal, Primordial
    (Aquan) CR 2 (XP 450; PB +2) ### Traits Amphibious. The merrow can breathe air and water. ### Actions
    Multiattack. The merrow makes two attacks, using Bite, Claw, or Harpoon in any combination. Bite.
    Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4) Piercing damage, and the target has the Poisoned
    condition until the end of the merrow’s next turn. Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 9
    (2d4 + 4) Slashing damage. Harpoon. Melee or Ranged Attack Roll: +6, reach 5 ft. or range 20/60 ft.
    Hit: 11 (2d6 + 4) Piercing damage. If the target is a Large or smaller creature, the merrow pulls
    the target up to 15 feet straight toward itself.'
  sha256: 9bc278e7de87867986daba959c3ad6480ad9946255e22c239eb0d7ef5e23ee31
  status: contract_specified
splat_requires_mechanics: *id001
---

# Merrow

<!-- SRD page 308 -->

Large Monstrosity, Chaotic Evil\
AC 13 Initiative +2 (12)\
HP 45 (6d10 + 12)\
Speed 10 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 15 +2 +2\
Int 8 −1 −1 Wis 10 +0 +0 Cha 9 −1 −1\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Abyssal, Primordial (Aquan)\
CR 2 (XP 450; PB +2)\

### Traits

Amphibious. The merrow can breathe air and water.\

### Actions

Multiattack. The merrow makes two attacks, using Bite,\
Claw, or Harpoon in any combination.\
Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4 + 4)\
Piercing damage, and the target has the Poisoned con-\
dition until the end of the merrow’s next turn.\
Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (2d4 + 4)\
Slashing damage.\
Harpoon. Melee or Ranged Attack Roll: +6, reach 5 ft.\
or range 20/60 ft. Hit: 11 (2d6 + 4) Piercing damage.\
If the target is a Large or smaller creature, the merrow\
pulls the target up to 15 feet straight toward itself.\

Original layout: [[../../srd-5.2.1/pages/308|page 308]].
