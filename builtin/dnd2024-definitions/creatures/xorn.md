---
splat_schema: 1
splat_id: e5311ec7-f6b3-509e-847d-cbae8bc3a2bd
splat_kind: creature
splat_key: dnd2024:creatures/xorn
splat_name: Xorn
splat_origin:
  key: dnd2024:reference/creature/xorn
splat_sources:
- title: SRD 5.2.1
  locator: pp. 343
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: elemental
  alignment: Neutral
  movement_text: 20 ft., Burrow 20 ft.
  skills_text: Perception +6, Stealth +6
  immunities_text: Poison; Paralyzed, Petrified, Poisoned
  senses_text: Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 16
  languages_text: Primordial (Terran)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 84 (8d8 + 48)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 22
  save_constitution:
    value: 6
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 19
  max_hp:
    value: 84
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
  burrow_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/xorn
  implementation: dnd2024:creature_xorn
  parameters:
    features:
      earth_glide:
        economy: trait
        operation: passive_rule
        burrow_materials:
        - earth
        - stone
        requires:
          material_nonmagical: true
          material_unworked: true
        disturb_material: false
      treasure_sense:
        economy: trait
        operation: passive_rule
        pinpoint:
        - precious_metals
        - precious_stones
        range_ft: 60
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: bite
          count: 1
        - operation: attack_sequence
          feature: claw
          count: 3
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
          - average: 17
            roll:
              count: 4
              sides: 6
              modifier: 3
            type: piercing
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
          - average: 8
            roll:
              count: 1
              sides: 10
              modifier: 3
            type: slashing
      charge:
        economy: bonus_action
        operation: move
        maximum_ft:
          choose_stat:
          - speed
          - burrow_speed
        direction: straight_toward_enemy
        enemy_sensed: true
    skill_totals:
      perception: 6
      stealth: 6
    damage_immunities:
    - poison
    condition_immunities:
    - paralyzed
    - petrified
    - poisoned
    senses:
      darkvision: 60
      tremorsense: 60
      passive_perception: 16
    languages:
      speaks:
      - language: primordial
        dialects:
        - terran
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
- id: dnd2024:creature_xorn
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
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: b64a60c751b6d4b7b9c525541ee8c4ce2cdb4941d2bcfe7e245d708ee51c147e
  source_path: builtin/srd-5.2.1/creature/xorn.md
  pages:
  - 343
  engine_gaps: &id001
  - dnd2024:creature_xorn
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 018f4989a4df495aa2c6c65f63ecb880f77284d01376a36d6dda4bc7885c7626
splat_clauses:
- key: source_rules
  text: '# Xorn Medium Elemental, Neutral AC 19 Initiative +0 (10) HP 84 (8d8 + 48) Speed 20 ft., Burrow
    20 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 10 +0 +0 Con22 +6 +6 Int 11 +0 +0 Wis 10 +0 +0
    Cha 11 +0 +0 Skills Perception +6, Stealth +6 Immunities Poison; Paralyzed, Petrified, Poisoned Senses
    Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 16 Languages Primordial (Terran) CR 5 (XP
    1,800; PB +3) ### Traits Earth Glide. The xorn can burrow through nonmagical, unworked earth and stone.
    While doing so, the xorn doesn’t disturb the material it moves through. Treasure Sense. The xorn can
    pinpoint the location of precious metals and stones within 60 feet of itself. ### Actions Multiattack.
    The xorn makes one Bite attack and three Claw attacks. Bite. Melee Attack Roll: +6, reach 5 ft. Hit:
    17 (4d6 + 3) Piercing damage. Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d10 + 3) Slashing
    damage. ### Bonus Actions Charge. The xorn moves up to its Speed or Burrow Speed straight toward an
    enemy it can sense.'
  sha256: 363e91ca31705d00ae0645f55ec9c6b6202c741325d4774e56691f5e4f78fa0e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Xorn

<!-- SRD page 343 -->

Medium Elemental, Neutral\
AC 19 Initiative +0 (10)\
HP 84 (8d8 + 48)\
Speed 20 ft., Burrow 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 10 +0 +0 Con22 +6 +6\
Int 11 +0 +0 Wis 10 +0 +0 Cha 11 +0 +0\
Skills Perception +6, Stealth +6\
Immunities Poison; Paralyzed, Petrified, Poisoned\
Senses Darkvision 60 ft., Tremorsense 60 ft.;\
Passive Perception 16\
Languages Primordial (Terran)\
CR 5 (XP 1,800; PB +3)\

### Traits

Earth Glide. The xorn can burrow through nonmagical,\
unworked earth and stone. While doing so, the xorn\
doesn’t disturb the material it moves through.\
Treasure Sense. The xorn can pinpoint the location of\
precious metals and stones within 60 feet of itself.\

### Actions

Multiattack. The xorn makes one Bite attack and three\
Claw attacks.\
Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 17 (4d6 + 3)\
Piercing damage.\
Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (1d10 +\
3) Slashing damage.\

### Bonus Actions

Charge. The xorn moves up to its Speed or Burrow\
Speed straight toward an enemy it can sense.\

Original layout: [[../../srd-5.2.1/pages/343|page 343]].
