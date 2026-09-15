---
splat_schema: 1
splat_id: f72fd22b-59c0-5ac7-acd1-5f9ed6b376bd
splat_kind: creature
splat_key: dnd2024:creatures/giant_hyena
splat_name: Giant Hyena
splat_origin:
  key: dnd2024:reference/creature/giant_hyena
splat_sources:
- title: SRD 5.2.1
  locator: pp. 352
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  skills_text: Perception +3
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 45
  initiative:
    value: 2
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_hyena
  implementation: dnd2024:creature_giant_hyena
  parameters:
    features:
      bite:
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
          - average: 10
            roll:
              count: 2
              sides: 6
              modifier: 3
            type: piercing
      rampage:
        economy: bonus_action
        resource:
          kind: daily
          uses: 1
          recovery: long_rest
        operation: sequence
        trigger: self.dealt_damage
        requires:
          target_already_bloodied: true
        immediate: true
        effects:
        - operation: move
          maximum_ft:
            multiply:
            - stat: speed
            - 0.5
        - operation: attack_sequence
          feature: bite
          count: 1
    skill_totals:
      perception: 3
    senses:
      darkvision: 60
      passive_perception: 13
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
- id: dnd2024:creature_giant_hyena
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
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6d79c67fea4b11b48a7a4d7a7d2575cc74e186f222bc99f8ad48323baf3e02a0
  source_path: builtin/srd-5.2.1/creature/giant_hyena.md
  pages:
  - 352
  engine_gaps: &id001
  - dnd2024:creature_giant_hyena
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: d6edd5f2a091ff584a08467c6a88625d3ddc895965e07cdbc43e395ee6dde58e
splat_clauses:
- key: source_rules
  text: '# Giant Hyena Large Beast, Unaligned AC 12 Initiative +2 (12) HP 45 (6d10 + 12) Speed 50 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2 Int 2 -4 -4 Wis 12 +1 +1 Cha 7 -2
    -2 Skills Perception +3 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR 1 (XP 200;
    PB +2) ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. ###
    Bonus Actions Rampage (1/Day). Immediately after dealing damage to a creature that was already Bloodied,
    the hyena can move up to half its Speed, and it makes one Bite attack.'
  sha256: bbca21909edd55ed8303d2e9fcce5134e42c4049eb285057593df552af1acaf3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Hyena

<!-- SRD page 352 -->

Large Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 45 (6d10 + 12)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2\
Int 2 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1 (XP 200; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3)\
Piercing damage.\

### Bonus Actions

Rampage (1/Day). Immediately after dealing damage\
to a creature that was already Bloodied, the hyena can\
move up to half its Speed, and it makes one Bite attack.\

Original layout: [[../../srd-5.2.1/pages/352|page 352]].
