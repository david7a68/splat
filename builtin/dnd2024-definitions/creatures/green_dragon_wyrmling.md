---
splat_schema: 1
splat_id: e7e31ec4-f899-574a-9ca7-62067bb43913
splat_kind: creature
splat_key: dnd2024:creatures/green_dragon_wyrmling
splat_name: Green Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/green_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 293
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 30 ft., Fly 60 ft., Swim 30 ft.
  skills_text: Perception +4, Stealth +3
  immunities_text: Poison; Poisoned
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 38 (7d8 + 7)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 17
  max_hp:
    value: 38
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/green_dragon_wyrmling
  implementation: dnd2024:creature_green_dragon_wyrmling
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
          feature: rend
          count: 2
      rend:
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
          - average: 7
            roll:
              count: 1
              sides: 10
              modifier: 2
            type: slashing
          - average: 3
            roll:
              count: 1
              sides: 6
              modifier: 0
            type: poison
      poison_breath:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: constitution
        dc: 11
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 15
            origin: self
        failure:
          damage:
          - average: 21
            roll:
              count: 6
              sides: 6
              modifier: 0
            type: poison
        success:
          damage_multiplier: 0.5
          other_effects: false
    skill_totals:
      perception: 4
      stealth: 3
    damage_immunities:
    - poison
    condition_immunities:
    - poisoned
    senses:
      blindsight: 10
      darkvision: 60
      passive_perception: 14
    languages:
      speaks:
      - draconic
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
- id: dnd2024:creature_green_dragon_wyrmling
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
  - turn.started
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 7fc3bfecfc95f7ed055003212a60de36e7e968a9acd8b4438d81b9489e1e8d86
  source_path: builtin/srd-5.2.1/creature/green_dragon_wyrmling.md
  pages:
  - 293
  engine_gaps: &id001
  - dnd2024:creature_green_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 4f93598241a783d124e00bc69b02090f72030d9654f2b6b383e4ce447124a2b7
splat_clauses:
- key: source_rules
  text: '# Green Dragon Wyrmling Medium Dragon (Chromatic), Lawful Evil AC 17 Initiative +3 (13) HP 38
    (7d8 + 7) Speed 30 ft., Fly 60 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 12 +1
    +3 Con 13 +1 +1 Int 14 +2 +2 Wis 11 +0 +2 Cha 13 +1 +1 Skills Perception +4, Stealth +3 Immunities
    Poison; Poisoned Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic
    CR 2 (XP 450; PB +2) ### Traits Amphibious. The dragon can breathe air and water. ### Actions Multiattack.
    The dragon makes two Rend attacks. Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10 + 2) Slashing
    damage plus 3 (1d6) Poison damage. Poison Breath (Recharge 5–6). Constitution Saving Throw: DC 11,
    each creature in a 15-foot Cone. Failure: 21 (6d6) Poison damage. Success: Half damage.'
  sha256: 72a756af7a8f2ff95e49a01a7485d056a22cb221c19bcb3988bcb2aa229b53a6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Green Dragon Wyrmling

<!-- SRD page 293 -->

Medium Dragon (Chromatic), Lawful Evil\
AC 17 Initiative +3 (13)\
HP 38 (7d8 + 7)\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +3 Con 13 +1 +1\
Int 14 +2 +2 Wis 11 +0 +2 Cha 13 +1 +1\
Skills Perception +4, Stealth +3\
Immunities Poison; Poisoned\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 2 (XP 450; PB +2)\

### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10\
+ 2) Slashing damage plus 3 (1d6) Poison damage.\
Poison Breath (Recharge 5–6). Constitution Saving\
Throw: DC 11, each creature in a 15-foot Cone. Failure:\
21 (6d6) Poison damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/293|page 293]].
