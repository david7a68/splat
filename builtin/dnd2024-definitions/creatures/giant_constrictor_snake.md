---
splat_schema: 1
splat_id: 5bc78e7c-531a-5d0e-8cb9-d4be2e2b6056
splat_kind: creature
splat_key: dnd2024:creatures/giant_constrictor_snake
splat_name: Giant Constrictor Snake
splat_origin:
  key: dnd2024:reference/creature/giant_constrictor_snake
splat_sources:
- title: SRD 5.2.1
  locator: pp. 350
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Swim 30 ft.
  skills_text: Perception +2
  senses_text: Blindsight 10 ft.; Passive Perception 12
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 60 (8d12 + 8)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 12
  max_hp:
    value: 60
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_constrictor_snake
  implementation: dnd2024:creature_giant_constrictor_snake
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: bite
          count: 1
        - operation: use_feature
          feature: constrict
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 10
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: piercing
      constrict:
        economy: action
        operation: saving_throw
        ability: strength
        dc: 14
        targets:
          kind: creature
          count: 1
          range_ft: 10
          visible: true
          maximum_size: large
        failure:
          damage:
          - average: 13
            roll:
              count: 2
              sides: 8
              modifier: 4
            type: bludgeoning
          grapple:
            condition: grappled
            source: self
            escape_dc: 14
        success:
          effects: []
    skill_totals:
      perception: 2
    senses:
      blindsight: 10
      passive_perception: 12
    languages:
      speaks: []
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
- id: dnd2024:creature_giant_constrictor_snake
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
  - saving_throws
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: afeb4b192781ff7a925c74c3fac6b2e72c5585d6c02681dcfe29366abe43a751
  source_path: builtin/srd-5.2.1/creature/giant_constrictor_snake.md
  pages:
  - 350
  engine_gaps: &id001
  - dnd2024:creature_giant_constrictor_snake
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 6e448a07bafe64e04c1c012abd402cbe1a1828a47e122e7d5becac6a8364c366
splat_clauses:
- key: source_rules
  text: '# Giant Constrictor Snake Huge Beast, Unaligned AC 12 Initiative +2 (12) HP 60 (8d12 + 8) Speed
    30 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 14 +2 +2 Con 12 +1 +1 Int 1 -5 -5
    Wis 10 +0 +0 Cha 3 -4 -4 Skills Perception +2 Senses Blindsight 10 ft.; Passive Perception 12 Languages
    None CR 2 (XP 450; PB +2) ### Actions Multiattack. The snake makes one Bite attack and uses Constrict.
    Bite. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 + 4) Piercing damage. Constrict. Strength Saving
    Throw: DC 14, one Large or smaller creature the snake can see within 10 feet. Failure: 13 (2d8 + 4)
    Bludgeoning damage, and the target has the Grappled condition (escape DC 14).'
  sha256: 581fe51b75fcd5798ef651cdfe5cea604a89c56f7feb51e3fab4202bd4b65958
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Constrictor Snake

<!-- SRD page 350 -->

Huge Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 60 (8d12 + 8)\
Speed 30 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +2 Con 12 +1 +1\
Int 1 −5 −5 Wis 10 +0 +0 Cha 3 −4 −4\
Skills Perception +2\
Senses Blindsight 10 ft.; Passive Perception 12\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The snake makes one Bite attack and uses\
Constrict.\
Bite. Melee Attack Roll: +6, reach 10 ft. Hit: 11 (2d6 +\
4) Piercing damage.\
Constrict. Strength Saving Throw: DC 14, one Large or\
smaller creature the snake can see within 10 feet. Fail-\
ure: 13 (2d8 + 4) Bludgeoning damage, and the target\
has the Grappled condition (escape DC 14).\

Original layout: [[../../srd-5.2.1/pages/350|page 350]].
