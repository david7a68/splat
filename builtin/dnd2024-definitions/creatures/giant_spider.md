---
splat_schema: 1
splat_id: d00bd6b3-23c5-5121-81d2-def8a283e438
splat_kind: creature
splat_key: dnd2024:creatures/giant_spider
splat_name: Giant Spider
splat_origin:
  key: dnd2024:reference/creature/giant_spider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 353, 354
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Perception +4, Stealth +7
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 26 (4d10 + 4)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 14
  max_hp:
    value: 26
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_spider
  implementation: dnd2024:creature_giant_spider
  parameters:
    features:
      spider_climb:
        economy: trait
        operation: passive_rule
        climb_difficult_surfaces: true
        climb_ceilings: true
        ability_check_required: false
      web_walker:
        economy: trait
        operation: passive_rule
        ignore_web_movement_restrictions: true
        locate_creatures_on_same_web: true
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
          - average: 7
            roll:
              count: 1
              sides: 8
              modifier: 3
            type: piercing
          - average: 7
            roll:
              count: 2
              sides: 6
              modifier: 0
            type: poison
      web:
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
          count: 1
          range_ft: 60
          visible: true
        failure:
          conditions:
          - condition: restrained
            until: web.destroyed
          web_object:
            ac: 10
            hp: 5
            damage_vulnerabilities:
            - fire
            damage_immunities:
            - poison
            - psychic
        success:
          effects: []
    skill_totals:
      perception: 4
      stealth: 7
    senses:
      darkvision: 60
      passive_perception: 14
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
- id: dnd2024:creature_giant_spider
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
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: b306725f61da9def8dca3cf4f3ef480bfbd27bc3677991b0f1e02b668406aacb
  source_path: builtin/srd-5.2.1/creature/giant_spider.md
  pages:
  - 353
  - 354
  engine_gaps: &id001
  - dnd2024:creature_giant_spider
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: f507a546b8cf6dc54184df5fb368417a51b04ca77feeb6505874e1446284b698
splat_clauses:
- key: source_rules
  text: '# Giant Spider Large Beast, Unaligned AC 14 Initiative +3 (13) HP 26 (4d10 + 4) Speed 30 ft.,
    Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 16 +3 +3 Con 12 +1 +1 Int 2 -4 -4 Wis 11
    +0 +0 Cha 4 -3 -3 Skills Perception +4, Stealth +7 Senses Darkvision 60 ft.; Passive Perception 14
    Languages None CR 1 (XP 200; PB +2) ### Traits Spider Climb. The spider can climb difficult surfaces,
    including along ceilings, without needing to make an ability check. Web Walker. The spider ignores
    movement restrictions caused by webs, and it knows the location of any other creature in contact with
    the same web. ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Piercing damage
    plus 7 (2d6) Poison damage. Web (Recharge 5–6). Dexterity Saving Throw: DC 13, one creature the spider
    can see within 60 feet. Failure: The target has the Restrained condition until the web is destroyed
    (AC 10; HP 5; Vulnerability to Fire damage; Immunity to Poison and Psychic damage).'
  sha256: a6c981f1e67aa74b0fa00997f568f000b3205ad5cb941e90e04e7df88c040556
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Spider

<!-- SRD page 353 -->

Large Beast, Unaligned\
AC 14  Initiative +3 (13)\
HP 26 (4d10 + 4)\
Speed 30 ft., Climb 30 ft.\

<!-- SRD page 354 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 16 +3 +3 Con 12 +1 +1\
Int 2 −4 −4 Wis 11 +0 +0 Cha 4 −3 −3\
Skills Perception +4, Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Spider Climb. The spider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Web Walker. The spider ignores movement restrictions\
caused by webs, and it knows the location of any other\
creature in contact with the same web.\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Piercing damage plus 7 (2d6) Poison damage.\
Web (Recharge 5–6). Dexterity Saving Throw: DC 13,\
one creature the spider can see within 60 feet. Failure:\
The target has the Restrained condition until the web is\
destroyed (AC 10; HP 5; Vulnerability to Fire damage;\
Immunity to Poison and Psychic damage).\

Original layout: [[../../srd-5.2.1/pages/353|page 353]], [[../../srd-5.2.1/pages/354|page 354]].
