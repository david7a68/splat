---
splat_schema: 1
splat_id: fe1f5e23-0df2-50cc-a56c-77412a9a8bdb
splat_kind: creature
splat_key: dnd2024:creatures/sphinx_of_wonder
splat_name: Sphinx of Wonder
splat_origin:
  key: dnd2024:reference/creature/sphinx_of_wonder
splat_sources:
- title: SRD 5.2.1
  locator: pp. 327
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: celestial
  alignment: Lawful Good
  movement_text: 20 ft., Fly 40 ft.
  skills_text: Arcana +4, Religion +4, Stealth +5
  resistances_text: Necrotic, Psychic, Radiant
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: Celestial, Common
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 24 (7d4 + 7)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 15
  save_intelligence:
    value: 2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 24
  initiative:
    value: 3
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/sphinx_of_wonder
  implementation: dnd2024:creature_sphinx_of_wonder
  parameters:
    features:
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
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
          - average: 5
            roll:
              count: 1
              sides: 4
              modifier: 3
            type: slashing
          - average: 7
            roll:
              count: 2
              sides: 6
              modifier: 0
            type: radiant
      burst_of_ingenuity:
        economy: reaction
        resource:
          kind: daily
          uses: 2
          recovery: long_rest
        operation: modify_roll
        trigger: ability_check_or_saving_throw
        targets:
          any_of:
          - self: true
          - within_ft: 30
        bonus: 2
    skill_totals:
      arcana: 4
      religion: 4
      stealth: 5
    damage_resistances:
    - necrotic
    - psychic
    - radiant
    senses:
      darkvision: 60
      passive_perception: 11
    languages:
      speaks:
      - celestial
      - common
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
- id: dnd2024:creature_sphinx_of_wonder
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
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 9c3a33079e43dba5d0a3591fb518d20acc5249f1756ed1411630f12bbda023b4
  source_path: builtin/srd-5.2.1/creature/sphinx_of_wonder.md
  pages:
  - 327
  engine_gaps: &id001
  - dnd2024:creature_sphinx_of_wonder
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 354bac0d7fee30bc4637050306ae5dc6bbba154149f30dac318d40d3eec49499
splat_clauses:
- key: source_rules
  text: '# Sphinx of Wonder Tiny Celestial, Lawful Good AC 13 Initiative +3 (13) HP 24 (7d4 + 7) Speed
    20 ft., Fly 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 17 +3 +3 Con 13 +1 +1 Int 15 +2 +2 Wis
    12 +1 +1 Cha 11 +0 +0 Skills Arcana +4, Religion +4, Stealth +5 Resistances Necrotic, Psychic, Radiant
    Senses Darkvision 60 ft.; Passive Perception 11 Languages Celestial, Common CR 1 (XP 200; PB +2) ###
    Traits Magic Resistance. The sphinx has Advantage on saving throws against spells and other magical
    effects. ### Actions Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Slashing damage plus
    7 (2d6) Radiant damage. ### Reactions Burst of Ingenuity (2/Day). Trigger: The sphinx or another creature
    within 30 feet makes an ability check or a saving throw. Response: The sphinx adds 2 to the roll.'
  sha256: 8b2fa00fb9d3c215ef7c312c60aa144a221e882236c4077690f05abc5d928463
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sphinx of Wonder

<!-- SRD page 327 -->

Tiny Celestial, Lawful Good\
AC 13  Initiative +3 (13)\
HP 24 (7d4 + 7)\
Speed 20 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 17 +3 +3 Con 13 +1 +1\
Int 15 +2 +2 Wis 12 +1 +1 Cha 11 +0 +0\
Skills Arcana +4, Religion +4, Stealth +5\
Resistances Necrotic, Psychic, Radiant\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Celestial, Common\
CR 1 (XP 200; PB +2)\

### Traits

Magic Resistance. The sphinx has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Slashing damage plus 7 (2d6) Radiant damage.\

### Reactions

Burst of Ingenuity (2/Day). Trigger: The sphinx or an-\
other creature within 30 feet makes an ability check or\
a saving throw. Response: The sphinx adds 2 to the roll.\

Original layout: [[../../srd-5.2.1/pages/327|page 327]].
