---
splat_schema: 1
splat_id: 0756754f-1ab7-52fc-9e78-b5d5636dca96
splat_kind: creature
splat_key: dnd2024:creatures/blue_dragon_wyrmling
splat_name: Blue Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/blue_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 266
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 30 ft., Burrow 15 ft., Fly 60 ft.
  skills_text: Perception +4, Stealth +2
  immunities_text: Lightning
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 65 (10d8 + 20)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 10
  save_dexterity:
    value: 2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 17
  max_hp:
    value: 65
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  burrow_speed:
    value: 15
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/blue_dragon_wyrmling
  implementation: dnd2024:creature_blue_dragon_wyrmling
  parameters:
    features:
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
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 8
            roll:
              count: 1
              sides: 10
              modifier: 3
            type: slashing
          - average: 3
            roll:
              count: 1
              sides: 6
              modifier: 0
            type: lightning
      lightning_breath:
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
        ability: dexterity
        dc: 12
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 30
            origin: self
            width_ft: 5
        failure:
          damage:
          - average: 21
            roll:
              count: 6
              sides: 6
              modifier: 0
            type: lightning
        success:
          damage_multiplier: 0.5
          other_effects: false
    skill_totals:
      perception: 4
      stealth: 2
    damage_immunities:
    - lightning
    condition_immunities: []
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
- id: dnd2024:creature_blue_dragon_wyrmling
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
  source_sha256: 13557e7a19f27709700b051aa9fb6b0611ec6fe71cfdd42ca461f7bb48ce2fcf
  source_path: builtin/srd-5.2.1/creature/blue_dragon_wyrmling.md
  pages:
  - 266
  engine_gaps: &id001
  - dnd2024:creature_blue_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: b52a1f6e3f53a5649fe5987db7e40526476719abe327fffecd0af56f5b803e72
splat_clauses:
- key: source_rules
  text: '# Blue Dragon Wyrmling Medium Dragon (Chromatic), Lawful Evil AC 17 Initiative +2 (12) HP 65
    (10d8 + 20) Speed 30 ft., Burrow 15 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 10
    +0 +2 Con 15 +2 +2 Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2 Skills Perception +4, Stealth +2 Immunities
    Lightning Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic CR
    3 (XP 700; PB +2) ### Actions Multiattack. The dragon makes two Rend attacks. Rend. Melee Attack Roll:
    +5, reach 5 ft. Hit: 8 (1d10 + 3) Slashing damage plus 3 (1d6) Lightning damage. Lightning Breath
    (Recharge 5–6). Dexterity Saving Throw: DC 12, each creature in a 30-foot-long, 5-footwide Line. Failure:
    21 (6d6) Lightning damage. Success: Half damage.'
  sha256: 3c740b16e393f5ec8e765ff1628bf1c95181ee167811ebc464a260bbabfd5dcf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blue Dragon Wyrmling

<!-- SRD page 266 -->

Medium Dragon (Chromatic), Lawful Evil\
AC 17 Initiative +2 (12)\
HP 65 (10d8 + 20)\
Speed 30 ft., Burrow 15 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 10 +0 +2 Con 15 +2 +2\
Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2\
Skills Perception +4, Stealth +2\
Immunities Lightning\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10\
+ 3) Slashing damage plus 3 (1d6) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 12, each creature in a 30-foot-long, 5-foot-\
wide Line. Failure: 21 (6d6) Lightning damage. Success:\
Half damage.\

Original layout: [[../../srd-5.2.1/pages/266|page 266]].
