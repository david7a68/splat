---
splat_schema: 1
splat_id: e042044b-02ee-5924-a25f-5af0cc5317c0
splat_kind: creature
splat_key: dnd2024:creatures/storm_giant
splat_name: Storm Giant
splat_origin:
  key: dnd2024:reference/creature/storm_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 330, 331
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: giant
  alignment: Chaotic Good
  movement_text: 50 ft., Fly 25 ft. (hover), Swim 50 ft.
  skills_text: Arcana +8, Athletics +14, History +8, Perception +10
  resistances_text: Cold
  immunities_text: Lightning, Thunder
  senses_text: Darkvision 120 ft., Truesight 30 ft.; Passive Perception 20
  languages_text: Common, Giant
  cr_text: 13 (XP 10,000; PB +5)
  hit_points_text: 230 (20d12 + 100)
splat_stats:
  strength:
    value: 29
  save_strength:
    value: 14
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 20
  save_constitution:
    value: 10
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 20
  save_wisdom:
    value: 10
  charisma:
    value: 18
  save_charisma:
    value: 9
  armor_class:
    value: 16
  max_hp:
    value: 230
  initiative:
    value: 7
  speed:
    value: 50
    unit: ft
  fly_speed:
    value: 25
    unit: ft
  swim_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/storm_giant
  implementation: dnd2024:creature_storm_giant
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
          - storm_sword
          - thunderbolt
      storm_sword:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 14
          reach_ft: 10
        hit:
          damage:
          - average: 23
            roll:
              count: 4
              sides: 6
              modifier: 9
            type: slashing
          - average: 13
            roll:
              count: 3
              sides: 8
              modifier: 0
            type: lightning
      thunderbolt:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 14
          range_ft: 500
          long_range_ft: null
        hit:
          damage:
          - average: 22
            roll:
              count: 2
              sides: 12
              modifier: 9
            type: lightning
          conditions:
          - condition: blinded
            until: self.next_turn_start
          - condition: deafened
            until: self.next_turn_start
      lightning_storm:
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
        dc: 18
        targets:
          kind: creature
          selection: each
          area:
            shape: cylinder
            radius_ft: 10
            height_ft: 40
            origin:
              visible: true
              range_ft: 500
        failure:
          damage:
          - average: 55
            roll:
              count: 10
              sides: 10
              modifier: 0
            type: lightning
        success:
          damage_multiplier: 0.5
          other_effects: false
      spellcasting:
        economy: action
        operation: spellcasting
        ability: wisdom
        components_waived:
        - material
        save_dc: 18
        options:
        - spell:
            key: dnd2024:spells/detect_magic
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/light
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/control_weather
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        material_components_available: true
    skill_totals:
      arcana: 8
      athletics: 14
      history: 8
      perception: 10
    damage_resistances:
    - cold
    damage_immunities:
    - lightning
    - thunder
    condition_immunities: []
    senses:
      darkvision: 120
      truesight: 30
      passive_perception: 20
    languages:
      speaks:
      - common
      - giant
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
- id: dnd2024:creature_storm_giant
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
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spellcasting
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: e42c9b7b1ce7c4a37a5412d7394e412b3a76ddfbc6e2a7d5e0aa4d15943ec0c6
  source_path: builtin/srd-5.2.1/creature/storm_giant.md
  pages:
  - 330
  - 331
  engine_gaps: &id001
  - dnd2024:creature_storm_giant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 87c993a127f944054fc8cad64fad29f2558e18b612b0ff7fea0745a368408637
splat_clauses:
- key: source_rules
  text: '# Storm Giant Huge Giant, Chaotic Good AC 16 Initiative +7 (17) HP 230 (20d12 + 100) Speed 50
    ft., Fly 25 ft. (hover), Swim 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 29 +9 +14 Dex 14 +2 +2 Con 20
    +5 +10 Int 16 +3 +3 Wis 20 +5 +10 Cha 18 +4 +9 Skills Arcana +8, Athletics +14, History +8, Perception
    +10 Resistances Cold Immunities Lightning, Thunder Senses Darkvision 120 ft., Truesight 30 ft.; Passive
    Perception 20 Languages Common, Giant CR 13 (XP 10,000; PB +5) ### Traits Amphibious. The giant can
    breathe air and water. ### Actions Multiattack. The giant makes two attacks, using Storm Sword or
    Thunderbolt in any combination. Storm Sword. Melee Attack Roll: +14, reach 10 ft. Hit: 23 (4d6 + 9)
    Slashing damage plus 13 (3d8) Lightning damage. Thunderbolt. Ranged Attack Roll: +14, range 500 ft.
    Hit: 22 (2d12 + 9) Lightning damage, and the target has the Blinded and Deafened conditions until
    the start of the giant’s next turn. Lightning Storm (Recharge 5–6). Dexterity Saving Throw: DC 18,
    each creature in a 10-foot-radius, 40-foot-high Cylinder originating from a point the giant can see
    within 500 feet. Failure: 55 (10d10) Lightning damage. Success: Half damage. Spellcasting. The giant
    casts one of the following spells, requiring no Material components and using Wisdom as the spellcasting
    ability (spell save DC 18): At Will: Detect Magic, Light 1/Day: Control Weather'
  sha256: afcfa6d8e959c0517c502cae308f9ff58710007e23e5b79bcf6d264ec127a22f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Storm Giant

<!-- SRD page 330 -->

Huge Giant, Chaotic Good\
AC 16 Initiative +7 (17)\
HP 230 (20d12 + 100)\
Speed 50 ft., Fly 25 ft. (hover), Swim 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 29 +9 +14 Dex 14 +2 +2 Con 20 +5 +10\
Int 16 +3 +3 Wis 20 +5 +10 Cha 18 +4 +9\
Skills Arcana +8, Athletics +14, History +8, Perception +10\
Resistances Cold\
Immunities Lightning, Thunder\
Senses Darkvision 120 ft., Truesight 30 ft.;\
Passive Perception 20\
Languages Common, Giant\
CR 13 (XP 10,000; PB +5)\

<!-- SRD page 331 -->


### Traits

Amphibious. The giant can breathe air and water.\

### Actions

Multiattack. The giant makes two attacks, using Storm\
Sword or Thunderbolt in any combination.\
Storm Sword. Melee Attack Roll: +14, reach 10 ft.\
Hit: 23 (4d6 + 9) Slashing damage plus 13 (3d8) Light-\
ning damage.\
Thunderbolt. Ranged Attack Roll: +14, range 500 ft.\
Hit: 22 (2d12 + 9) Lightning damage, and the target has\
the Blinded and Deafened conditions until the start of\
the giant’s next turn.\
Lightning Storm (Recharge 5–6). Dexterity Saving\
Throw: DC 18, each creature in a 10-foot-radius,\
40-foot-high Cylinder originating from a point the giant\
can see within 500 feet. Failure: 55 (10d10) Lightning\
damage. Success: Half damage.\
Spellcasting. The giant casts one of the following spells,\
requiring no Material components and using Wisdom\
as the spellcasting ability (spell save DC 18):\
At Will: Detect Magic, Light\
1/Day: Control Weather\

Original layout: [[../../srd-5.2.1/pages/330|page 330]], [[../../srd-5.2.1/pages/331|page 331]].
