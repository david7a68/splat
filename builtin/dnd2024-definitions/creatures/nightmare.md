---
splat_schema: 1
splat_id: 9ba2827f-99b6-59da-a8c3-d724f26f2d8b
splat_kind: creature
splat_key: dnd2024:creatures/nightmare
splat_name: Nightmare
splat_origin:
  key: dnd2024:reference/creature/nightmare
splat_sources:
- title: SRD 5.2.1
  locator: pp. 311
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Neutral Evil
  movement_text: 60 ft., Fly 90 ft. (hover)
  immunities_text: Fire
  senses_text: Passive Perception 11
  languages_text: Understands Abyssal, Common, and Infernal but can’t speak
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 68 (8d10 + 24)
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
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 13
  max_hp:
    value: 68
  initiative:
    value: 2
  speed:
    value: 60
    unit: ft
  fly_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/nightmare
  implementation: dnd2024:creature_nightmare
  parameters:
    features:
      confer_fire_resistance:
        economy: trait
        operation: passive_rule
        optional: true
        grant:
          damage_resistances:
          - fire
          target: rider
          while: mounted_on_self
      illumination:
        economy: trait
        operation: passive_rule
        bright_radius_ft: 10
        additional_dim_radius_ft: 10
      hooves:
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
          - average: 13
            roll:
              count: 2
              sides: 8
              modifier: 4
            type: bludgeoning
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: fire
      ethereal_stride:
        economy: action
        operation: planar_teleport
        targets:
          self: true
          additional:
            count_maximum: 3
            willing: true
            range_ft: 5
        plane_pairs:
        - - material
          - ethereal
        - - ethereal
          - material
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
      passive_perception: 11
    languages:
      speaks: []
      understands:
      - abyssal
      - common
      - infernal
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
- id: dnd2024:creature_nightmare
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
  - teleportation
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 367f7f597f2a2bea3105a677683199cd06f7f117f375fd28b6d76a6e0847d14c
  source_path: builtin/srd-5.2.1/creature/nightmare.md
  pages:
  - 311
  engine_gaps: &id001
  - dnd2024:creature_nightmare
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: c1db638d65bbac9e6864667f5a5d670f27bf282cafccf908fc5330af457d7ba9
splat_clauses:
- key: source_rules
  text: '# Nightmare Large Fiend, Neutral Evil AC 13 Initiative +2 (12) HP 68 (8d10 + 24) Speed 60 ft.,
    Fly 90 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3 Int 10 +0 +0
    Wis 13 +1 +1 Cha 15 +2 +2 Immunities Fire Senses Passive Perception 11 Languages Understands Abyssal,
    Common, and Infernal but can’t speak CR 3 (XP 700; PB +2) ### Traits Confer Fire Resistance. The nightmare
    can grant Resistance to Fire damage to a rider while it is on the nightmare. Illumination. The nightmare
    sheds Bright Light in a 10- foot radius and Dim Light for an additional 10 feet. ### Actions Hooves.
    Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage plus 10 (3d6) Fire damage.
    Ethereal Stride. The nightmare and up to three willing creatures within 5 feet of it teleport to the
    Ethereal Plane from the Material Plane or vice versa.'
  sha256: b0ed60214708166cbcb099852122361c5e98f233f0cc9d427fdff2aa49047eb5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Nightmare

<!-- SRD page 311 -->

Large Fiend, Neutral Evil\
AC 13 Initiative +2 (12)\
HP 68 (8d10 + 24)\
Speed 60 ft., Fly 90 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 10 +0 +0 Wis 13 +1 +1 Cha 15 +2 +2\
Immunities Fire\
Senses Passive Perception 11\
Languages Understands Abyssal, Common, and Infernal\
but can’t speak\
CR 3 (XP 700; PB +2)\

### Traits

Confer Fire Resistance. The nightmare can grant Re-\
sistance to Fire damage to a rider while it is on the\
nightmare.\
Illumination. The nightmare sheds Bright Light in a 10-\
foot radius and Dim Light for an additional 10 feet.\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 13 (2d8\
+ 4) Bludgeoning damage plus 10 (3d6) Fire damage.\
Ethereal Stride. The nightmare and up to three willing\
creatures within 5 feet of it teleport to the Ethereal\
Plane from the Material Plane or vice versa.\

Original layout: [[../../srd-5.2.1/pages/311|page 311]].
