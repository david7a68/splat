---
splat_schema: 1
splat_id: a3525aeb-3b3c-53c4-91a3-5d690a0171a4
splat_kind: creature
splat_key: dnd2024:creatures/young_gold_dragon
splat_name: Young Gold Dragon
splat_origin:
  key: dnd2024:reference/creature/young_gold_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 291
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Insight +5, Perception +9, Persuasion +9, Stealth +6
  immunities_text: Fire
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 19
  languages_text: Common, Draconic
  cr_text: 10 (XP 5,900; PB +4)
  hit_points_text: 178 (17d10 + 85)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 14
  save_dexterity:
    value: 6
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 13
  save_wisdom:
    value: 5
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 18
  max_hp:
    value: 178
  initiative:
    value: 6
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/young_gold_dragon
  implementation: dnd2024:creature_young_gold_dragon
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
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: weakening_breath
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 10
          reach_ft: 10
        hit:
          damage:
          - average: 17
            roll:
              count: 2
              sides: 10
              modifier: 6
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
        dc: 17
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 30
            origin: self
        failure:
          damage:
          - average: 55
            roll:
              count: 10
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
        dc: 17
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 30
            origin: self
          exclude_already_affected_by: this_breath
        failure:
          disadvantage_on:
            roll: d20_test
            ability: strength
          damage_roll_modifier:
            subtract:
              count: 1
              sides: 6
              modifier: 0
          repeat_save:
            event: target.turn_end
            success: end_effect
          automatic_success_after_seconds: 60
        success:
          effects: []
    skill_totals:
      insight: 5
      perception: 9
      persuasion: 9
      stealth: 6
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
      blindsight: 30
      darkvision: 120
      passive_perception: 19
    languages:
      speaks:
      - common
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
- id: dnd2024:creature_young_gold_dragon
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
  source_sha256: dcd3cb33ba176ba272d3bd7c90784c4c67b25cbf82e73fc3d23b858f225f816f
  source_path: builtin/srd-5.2.1/creature/young_gold_dragon.md
  pages:
  - 291
  engine_gaps: &id001
  - dnd2024:creature_young_gold_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: b6e1936635fc7fe8d7499c9e80f3f66933eb56259030183986b66c7e79517781
splat_clauses:
- key: source_rules
  text: '# Young Gold Dragon Large Dragon (Metallic), Lawful Good AC 18 Initiative +6 (16) HP 178 (17d10
    + 85) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 14 +2 +6 Con
    21 +5 +5 Int 16 +3 +3 Wis 13 +1 +5 Cha 20 +5 +5 Skills Insight +5, Perception +9, Persuasion +9, Stealth
    +6 Immunities Fire Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 19 Languages Common,
    Draconic CR 10 (XP 5,900; PB +4) ### Traits Amphibious. The dragon can breathe air and water. ###
    Actions Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of
    Weakening Breath. Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 17 (2d10 + 6) Slashing damage. Fire
    Breath (Recharge 5–6). Dexterity Saving Throw: DC 17, each creature in a 30-foot Cone. Failure: 55
    (10d10) Fire damage. Success: Half damage. Weakening Breath. Strength Saving Throw: DC 17, each creature
    that isn’t currently affected by this breath in a 30-foot Cone. Failure: The target has Disadvantage
    on Strength-based D20 Tests and subtracts 3 (1d6) from its damage rolls. It repeats the save at the
    end of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.'
  sha256: 90ee0b86f0cd5ac2a15f90e352d9f2f29b448a1ad8a35301e9da053c7527781d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Gold Dragon

<!-- SRD page 291 -->

Large Dragon (Metallic), Lawful Good\
AC 18 Initiative +6 (16)\
HP 178 (17d10 + 85)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 14 +2 +6 Con 21 +5 +5\
Int 16 +3 +3 Wis 13 +1 +5 Cha 20 +5 +5\
Skills Insight +5, Perception +9, Persuasion +9, Stealth +6\
Immunities Fire\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 19\
Languages Common, Draconic\
CR 10 (XP 5,900; PB +4)\

### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Weakening Breath.\
Rend. Melee Attack Roll: +10, reach 10 ft. Hit: 17 (2d10\
+ 6) Slashing damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 17, each creature in a 30-foot Cone. Failure: 55\
(10d10) Fire damage. Success: Half damage.\
Weakening Breath. Strength Saving Throw: DC 17,\
each creature that isn’t currently affected by this breath\
in a 30-foot Cone. Failure: The target has Disadvantage\
on Strength-based D20 Tests and subtracts 3 (1d6) from\
its damage rolls. It repeats the save at the end of each\
of its turns, ending the effect on itself on a success. Af-\
ter 1 minute, it succeeds automatically.\

Original layout: [[../../srd-5.2.1/pages/291|page 291]].
