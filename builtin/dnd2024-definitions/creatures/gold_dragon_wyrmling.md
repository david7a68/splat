---
splat_schema: 1
splat_id: ffc85f48-2e29-56ea-b9ba-62b03db90a4b
splat_kind: creature
splat_key: dnd2024:creatures/gold_dragon_wyrmling
splat_name: Gold Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/gold_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 290, 291
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 30 ft., Fly 60 ft., Swim 30 ft.
  skills_text: Perception +4, Stealth +4
  immunities_text: Fire
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 3 (700 XP; PB +2)
  hit_points_text: 60 (8d8 + 24)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 4
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 16
  save_charisma:
    value: 3
  armor_class:
    value: 17
  max_hp:
    value: 60
  initiative:
    value: 4
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
  mechanic: dnd2024:creatures/gold_dragon_wyrmling
  implementation: dnd2024:creature_gold_dragon_wyrmling
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
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 9
            roll:
              count: 1
              sides: 10
              modifier: 4
            type: slashing
      fire_breath:
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
        dc: 13
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 15
            origin: self
        failure:
          damage:
          - average: 22
            roll:
              count: 4
              sides: 10
              modifier: 0
            type: fire
        success:
          damage_multiplier: 0.5
          other_effects: false
      weakening_breath:
        economy: action
        operation: saving_throw
        ability: strength
        dc: 13
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 15
            origin: self
          exclude_already_affected_by: this_breath
        failure:
          disadvantage_on:
            roll: d20_test
            ability: strength
          damage_roll_modifier:
            subtract:
              count: 1
              sides: 4
              modifier: 0
          repeat_save:
            event: target.turn_end
            success: end_effect
          automatic_success_after_seconds: 60
        success:
          effects: []
    skill_totals:
      perception: 4
      stealth: 4
    damage_immunities:
    - fire
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
- id: dnd2024:creature_gold_dragon_wyrmling
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
  - turn.ended
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
  source_sha256: 975869cdfb8517efbdd3d3bb627ff9cee85449430d3709b750e3193295e111d2
  source_path: builtin/srd-5.2.1/creature/gold_dragon_wyrmling.md
  pages:
  - 290
  - 291
  engine_gaps: &id001
  - dnd2024:creature_gold_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 44a382e02028fe9b3ee814eab8864dadefd43ec3a6ea6b8fa719dc4213f8de5d
splat_clauses:
- key: source_rules
  text: '# Gold Dragon Wyrmling Medium Dragon (Metallic), Lawful Good AC 17 Initiative +4 (14) HP 60 (8d8
    + 24) Speed 30 ft., Fly 60 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 14 +2 +4 Con
    17 +3 +3 Int 14 +2 +2 Wis 11 +0 +2 Cha 16 +3 +3 Skills Perception +4, Stealth +4 Immunities Fire Senses
    Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic CR 3 (700 XP; PB +2)
    ### Traits Amphibious. The dragon can breathe air and water. ### Actions Multiattack. The dragon makes
    two Rend attacks. Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (1d10 + 4) Slashing damage. Fire
    Breath (Recharge 5–6). Dexterity Saving Throw: DC 13, each creature in a 15-foot Cone. Failure: 22
    (4d10) Fire damage. Success: Half damage. Weakening Breath. Strength Saving Throw: DC 13, each creature
    that isn’t currently affected by this breath in a 15-foot Cone. Failure: The target has Disadvantage
    on Strength-based D20 Tests and subtracts 2 (1d4) from its damage rolls. It repeats the save at the
    end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.'
  sha256: 4e797013df2330edd04c6a0cf4ed6ac176f91ad2d5de8b7d8209bbdf38470149
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gold Dragon Wyrmling

<!-- SRD page 290 -->

Medium Dragon (Metallic), Lawful Good\
AC 17 Initiative +4 (14)\
HP 60 (8d8 + 24)\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +4 Con 17 +3 +3\
Int 14 +2 +2 Wis 11 +0 +2 Cha 16 +3 +3\
Skills Perception +4, Stealth +4\
Immunities Fire\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 3 (700 XP; PB +2)\

<!-- SRD page 291 -->


### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +6, reach 5 ft. Hit: 9 (1d10\
+ 4) Slashing damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 13, each creature in a 15-foot Cone. Failure: 22\
(4d10) Fire damage. Success: Half damage.\
Weakening Breath. Strength Saving Throw: DC 13,\
each creature that isn’t currently affected by this breath\
in a 15-foot Cone. Failure: The target has Disadvantage\
on Strength-based D20 Tests and subtracts 2 (1d4) from\
its damage rolls. It repeats the save at the end of each\
of its turns, ending the effect on itself on a success. Af-\
ter 1 minute, it succeeds automatically.\

Original layout: [[../../srd-5.2.1/pages/290|page 290]], [[../../srd-5.2.1/pages/291|page 291]].
