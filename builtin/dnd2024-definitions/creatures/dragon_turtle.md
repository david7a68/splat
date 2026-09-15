---
splat_schema: 1
splat_id: e6257a3a-9490-5c23-9b3c-7c8a2f10f404
splat_kind: creature
splat_key: dnd2024:creatures/dragon_turtle
splat_name: Dragon Turtle
splat_origin:
  key: dnd2024:reference/creature/dragon_turtle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 281
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Neutral
  movement_text: 20 ft., Swim 50 ft.
  resistances_text: Fire
  senses_text: Darkvision 120 ft.; Passive Perception 11
  languages_text: Draconic, Primordial (Aquan)
  cr_text: 17 (XP 18,000; PB +6)
  hit_points_text: 356 (23d20 + 115)
splat_stats:
  strength:
    value: 25
  save_strength:
    value: 7
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 20
  save_constitution:
    value: 11
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 7
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 20
  max_hp:
    value: 356
  initiative:
    value: 6
  speed:
    value: 20
    unit: ft
  swim_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/dragon_turtle
  implementation: dnd2024:creature_dragon_turtle
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
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: tail
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 13
          reach_ft: 15
        hit:
          damage:
          - average: 23
            roll:
              count: 3
              sides: 10
              modifier: 7
            type: piercing
          - average: 7
            roll:
              count: 2
              sides: 6
              modifier: 0
            type: fire
        underwater_fire_resistance_applies: false
      tail:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 13
          reach_ft: 15
        hit:
          damage:
          - average: 18
            roll:
              count: 2
              sides: 10
              modifier: 7
            type: bludgeoning
          prone:
            condition: prone
            maximum_size: huge
      steam_breath:
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
        dc: 19
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 60
            origin: self
        failure:
          damage:
          - average: 56
            roll:
              count: 16
              sides: 6
              modifier: 0
            type: fire
        success:
          damage_multiplier: 0.5
          other_effects: false
        underwater_fire_resistance_applies: false
    damage_resistances:
    - fire
    senses:
      darkvision: 120
      passive_perception: 11
    languages:
      speaks:
      - draconic
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
- id: dnd2024:creature_dragon_turtle
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
splat_conversion:
  status: specified
  category: creature
  source_sha256: 777609b936fc36a652d5d0013a3720ed4c714302352c0c64fe989d065e60ea0a
  source_path: builtin/srd-5.2.1/creature/dragon_turtle.md
  pages:
  - 281
  engine_gaps: &id001
  - dnd2024:creature_dragon_turtle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: dfe2dd3598a4b2c4ae3dd144608403dad948b3f6500e758286d6643efc6d1a83
splat_clauses:
- key: source_rules
  text: '# Dragon Turtle Gargantuan Dragon, Neutral AC 20 Initiative +6 (16) HP 356 (23d20 + 115) Speed
    20 ft., Swim 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 25 +7 +7 Dex 10 +0 +0 Con 20 +5 +11 Int 10 +0 +0
    Wis 12 +1 +7 Cha 12 +1 +1 Resistances Fire Senses Darkvision 120 ft.; Passive Perception 11 Languages
    Draconic, Primordial (Aquan) CR 17 (XP 18,000; PB +6) ### Traits Amphibious. The dragon can breathe
    air and water. ### Actions Multiattack. The dragon makes three Bite attacks. It can replace one attack
    with a Tail attack. Bite. Melee Attack Roll: +13, reach 15 ft. Hit: 23 (3d10 + 7) Piercing damage
    plus 7 (2d6) Fire damage. Being underwater doesn’t grant Resistance to this Fire damage. Tail. Melee
    Attack Roll: +13, reach 15 ft. Hit: 18 (2d10 + 7) Bludgeoning damage. If the target is a Huge or smaller
    creature, it has the Prone condition. Steam Breath (Recharge 5–6). Constitution Saving Throw: DC 19,
    each creature in a 60-foot Cone. Failure: 56 (16d6) Fire damage. Success: Half damage. Failure or
    Success: Being underwater doesn’t grant Resistance to this Fire damage.'
  sha256: ae80c142962f520f86383f1f28815523bb448237858fd98b6251e4ed300c4fff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dragon Turtle

<!-- SRD page 281 -->

Gargantuan Dragon, Neutral\
AC 20 Initiative +6 (16)\
HP 356 (23d20 + 115)\
Speed 20 ft., Swim 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 25 +7 +7 Dex 10 +0 +0 Con 20 +5 +11\
Int 10 +0 +0 Wis 12 +1 +7 Cha 12 +1 +1\
Resistances Fire\
Senses Darkvision 120 ft.; Passive Perception 11\
Languages Draconic, Primordial (Aquan)\
CR 17 (XP 18,000; PB +6)\

### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes three Bite attacks. It\
can replace one attack with a Tail attack.\
Bite. Melee Attack Roll: +13, reach 15 ft. Hit: 23\
(3d10 + 7) Piercing damage plus 7 (2d6) Fire damage.\
Being underwater doesn’t grant Resistance to this\
Fire damage.\
Tail. Melee Attack Roll: +13, reach 15 ft. Hit: 18 (2d10\
+ 7) Bludgeoning damage. If the target is a Huge or\
smaller creature, it has the Prone condition.\
Steam Breath (Recharge 5–6). Constitution Saving\
Throw: DC 19, each creature in a 60-foot Cone. Failure:\
56 (16d6) Fire damage. Success: Half damage. Failure or\
Success: Being underwater doesn’t grant Resistance to\
this Fire damage.\

Original layout: [[../../srd-5.2.1/pages/281|page 281]].
