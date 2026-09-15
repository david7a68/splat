---
splat_schema: 1
splat_id: 45736a8e-2b24-57d7-ba5a-3551a26cfee1
splat_kind: creature
splat_key: dnd2024:creatures/remorhaz
splat_name: Remorhaz
splat_origin:
  key: dnd2024:reference/creature/remorhaz
splat_sources:
- title: SRD 5.2.1
  locator: pp. 319, 320
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft., Burrow 30 ft.
  immunities_text: Cold, Fire
  senses_text: Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 10
  languages_text: None
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 195 (17d12 + 85)
splat_stats:
  strength:
    value: 24
  save_strength:
    value: 7
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 4
  save_intelligence:
    value: -3
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 17
  max_hp:
    value: 195
  initiative:
    value: 5
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/remorhaz
  implementation: dnd2024:creature_remorhaz
  parameters:
    features:
      heat_aura:
        economy: trait
        operation: passive_rule
        trigger: self.turn_end
        targets:
          kind: creature
          selection: each
          area:
            shape: emanation
            size_ft: 5
            origin: self
        damage:
        - roll:
            count: 3
            sides: 10
            modifier: 0
          type: fire
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 11
          reach_ft: 10
        hit:
          damage:
          - average: 18
            roll:
              count: 2
              sides: 10
              modifier: 7
            type: piercing
          - average: 14
            roll:
              count: 4
              sides: 6
              modifier: 0
            type: fire
          grapple:
            condition: grappled
            source: self
            escape_dc: 17
            maximum_size: large
            while_grappled:
              conditions:
              - restrained
      swallow:
        economy: bonus_action
        operation: saving_throw
        ability: strength
        dc: 19
        failure:
          end_grapple: true
          swallow:
            maximum_size: large
            conditions:
            - restrained
            - blinded
            cover_against_outside: total
            periodic_damage:
              event: self.turn_start
              damage:
              - roll:
                  count: 3
                  sides: 6
                  modifier: 0
                type: acid
              - roll:
                  count: 3
                  sides: 6
                  modifier: 0
                type: fire
            regurgitate:
              trigger: turn_end
              if_damage_on_single_turn_from_one_inside_creature_at_least: 30
              save:
                ability: constitution
                dc: 15
                failure:
                  eject: all
                  within_ft: 5
                  conditions:
                  - prone
            on_self_death:
              remove_conditions:
              - restrained
              escape_movement_ft: 15
              exit_condition: prone
        targets:
          kind: creature
          count: 1
          grappled_by: self
          maximum_size: large
        capacity: 2
        success:
          effects: []
    damage_immunities:
    - cold
    - fire
    condition_immunities: []
    senses:
      darkvision: 60
      tremorsense: 60
      passive_perception: 10
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
- id: dnd2024:creature_remorhaz
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
  - grappling
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 7e7b1c49b60332c99da4125af12e4010fb7429152fab541cfd1812492d1554f7
  source_path: builtin/srd-5.2.1/creature/remorhaz.md
  pages:
  - 319
  - 320
  engine_gaps: &id001
  - dnd2024:creature_remorhaz
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 3da223a72e8ddafef31e3f6f003fcdcae32e01ddb7bdaa9912cf3f1fabaca9b1
splat_clauses:
- key: source_rules
  text: '# Remorhaz Huge Monstrosity, Unaligned AC 17 Initiative +5 (15) HP 195 (17d12 + 85) Speed 40
    ft., Burrow 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 24 +7 +7 Dex 13 +1 +1 Con 21 +5 +5 Int 4 -3 -3 Wis
    10 +0 +0 Cha 5 -3 -3 Immunities Cold, Fire Senses Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception
    10 Languages None CR 11 (XP 7,200; PB +4) ### Traits Heat Aura. At the end of each of the remorhaz’s
    turns, each creature in a 5-foot Emanation originating from the remorhaz takes 16 (3d10) Fire damage.
    ### Actions Bite. Melee Attack Roll: +11, reach 10 ft. Hit: 18 (2d10 + 7) Piercing damage plus 14
    (4d6) Fire damage. If the target is a Large or smaller creature, it has the Grappled condition (escape
    DC 17), and it has the Restrained condition until the grapple ends. ### Bonus Actions Swallow. Strength
    Saving Throw: DC 19, one Large or smaller creature Grappled by the remorhaz (it can have up to two
    creatures swallowed at a time). Failure: The target is swallowed by the remorhaz, and the Grappled
    condition ends. A swallowed creature has the Blinded and Restrained conditions, it has Total Cover
    against attacks and other effects outside the remorhaz, and it takes 10 (3d6) Acid damage plus 10
    (3d6) Fire damage at the start of each of the remorhaz’s turns. If the remorhaz takes 30 damage or
    more on a single turn from a creature inside it, the remorhaz must succeed on a DC 15 Constitution
    saving throw at the end of that turn or regurgitate all swallowed creatures, each of which falls in
    a space within 5 feet of the remorhaz and has the Prone condition. If the remorhaz dies, any swallowed
    creature no longer has the Restrained condition and can escape from the corpse by using 15 feet of
    movement, exiting Prone.'
  sha256: 558747f34caae5043d41bbc8a19732a42c73e675346f32e0c4d6a46a1aec1d81
  status: contract_specified
splat_requires_mechanics: *id001
---

# Remorhaz

<!-- SRD page 319 -->

Huge Monstrosity, Unaligned\
AC 17 Initiative +5 (15)\
HP 195 (17d12 + 85)\
Speed 40 ft., Burrow 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 24 +7 +7 Dex 13 +1 +1 Con 21 +5 +5\
Int 4 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
Immunities Cold, Fire\
Senses Darkvision 60 ft., Tremorsense 60 ft.;\
Passive Perception 10\
Languages None\
CR 11 (XP 7,200; PB +4)\

### Traits

Heat Aura. At the end of each of the remorhaz’s turns,\
each creature in a 5-foot Emanation originating from\
the remorhaz takes 16 (3d10) Fire damage.\

### Actions

Bite. Melee Attack Roll: +11, reach 10 ft. Hit: 18 (2d10\
+ 7) Piercing damage plus 14 (4d6) Fire damage. If the\
target is a Large or smaller creature, it has the Grappled\
condition (escape DC 17), and it has the Restrained\
condition until the grapple ends.\

### Bonus Actions

Swallow. Strength Saving Throw: DC 19, one Large or\
smaller creature Grappled by the remorhaz (it can have\
up to two creatures swallowed at a time). Failure: The\
target is swallowed by the remorhaz, and the Grappled\
condition ends. A swallowed creature has the Blinded\

<!-- SRD page 320 -->

and Restrained conditions, it has Total Cover against\
attacks and other effects outside the remorhaz, and it\
takes 10 (3d6) Acid damage plus 10 (3d6) Fire damage\
at the start of each of the remorhaz’s turns.\
If the remorhaz takes 30 damage or more on a single\
turn from a creature inside it, the remorhaz must suc-\
ceed on a DC 15 Constitution saving throw at the end\
of that turn or regurgitate all swallowed creatures, each\
of which falls in a space within 5 feet of the remorhaz\
and has the Prone condition. If the remorhaz dies, any\
swallowed creature no longer has the Restrained condi-\
tion and can escape from the corpse by using 15 feet of\
movement, exiting Prone.\

Original layout: [[../../srd-5.2.1/pages/319|page 319]], [[../../srd-5.2.1/pages/320|page 320]].
