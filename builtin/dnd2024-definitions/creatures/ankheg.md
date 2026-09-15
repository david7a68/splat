---
splat_schema: 1
splat_id: fd9787b0-eaf0-561d-84ab-0ad3568376fd
splat_kind: creature
splat_key: dnd2024:creatures/ankheg
splat_name: Ankheg
splat_origin:
  key: dnd2024:reference/creature/ankheg
splat_sources:
- title: SRD 5.2.1
  locator: pp. 259
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Unaligned
  movement_text: 30 ft., Burrow 10 ft.
  senses_text: Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 11
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 45 (6d10 + 12)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 14
  max_hp:
    value: 45
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  burrow_speed:
    value: 10
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ankheg
  implementation: dnd2024:creature_ankheg
  parameters:
    features:
      tunneler:
        economy: trait
        operation: passive_rule
        burrow_material: solid_rock
        speed_multiplier: 0.5
        tunnel_diameter_ft: 10
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
          advantage_if:
            target_grappled_by: self
        hit:
          damage:
          - average: 10
            roll:
              count: 2
              sides: 6
              modifier: 3
            type: slashing
          - average: 3
            roll:
              count: 1
              sides: 6
              modifier: 0
            type: acid
          grapple:
            condition: grappled
            source: self
            escape_dc: 13
            maximum_size: large
      acid_spray:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: dexterity
        dc: 12
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 30
            origin: self
            width_ft: 5
        failure:
          damage:
          - average: 14
            roll:
              count: 4
              sides: 6
              modifier: 0
            type: acid
        success:
          damage_multiplier: 0.5
          other_effects: false
    senses:
      darkvision: 60
      tremorsense: 60
      passive_perception: 11
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
- id: dnd2024:creature_ankheg
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
  - grappling
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: b949293f1ff9c29486d5fb019d894ba54958ca1e820db25e48474ecdbc5b5216
  source_path: builtin/srd-5.2.1/creature/ankheg.md
  pages:
  - 259
  engine_gaps: &id001
  - dnd2024:creature_ankheg
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 4adb54d788766fbfee6bc2b7027bd402a7299530aefdf9f15b7f44f7334f0ebb
splat_clauses:
- key: source_rules
  text: '# Ankheg Large Monstrosity, Unaligned AC 14 Initiative +0 (10) HP 45 (6d10 + 12) Speed 30 ft.,
    Burrow 10 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 11 +0 +0 Con 14 +2 +2 Int 1 -5 -5 Wis 13
    +1 +1 Cha 6 -2 -2 Senses Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 11 Languages None
    CR 2 (XP 450; PB +2) ### Traits Tunneler. The ankheg can burrow through solid rock at half its Burrow
    Speed and leaves a 10-foot-diameter tunnel in its wake. ### Actions Bite. Melee Attack Roll: +5 (with
    Advantage if the target is Grappled by the ankheg), reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage
    plus 3 (1d6) Acid damage. If the target is a Large or smaller creature, it has the Grappled condition
    (escape DC 13). Acid Spray (Recharge 6). Dexterity Saving Throw: DC 12, each creature in a 30-foot-long,
    5-foot-wide Line. Failure: 14 (4d6) Acid damage. Success: Half damage.'
  sha256: 8a091fe628d7e4d0d4129c8dcbb827a16235cc88df0abe0db303c694e0439a4e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ankheg

<!-- SRD page 259 -->

Large Monstrosity, Unaligned\
AC 14 Initiative +0 (10)\
HP 45 (6d10 + 12)\
Speed 30 ft., Burrow 10 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 11 +0 +0 Con 14 +2 +2\
Int 1 −5 −5 Wis 13 +1 +1 Cha 6 −2 −2\
Senses Darkvision 60 ft., Tremorsense 60 ft.;\
Passive Perception 11\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Tunneler. The ankheg can burrow through solid rock\
at half its Burrow Speed and leaves a 10-foot-diameter\
tunnel in its wake.\

### Actions

Bite. Melee Attack Roll: +5 (with Advantage if the target\
is Grappled by the ankheg), reach 5 ft. Hit: 10 (2d6 + 3)\
Slashing damage plus 3 (1d6) Acid damage. If the target\
is a Large or smaller creature, it has the Grappled con-\
dition (escape DC 13).\
Acid Spray (Recharge 6). Dexterity Saving Throw: DC\
12, each creature in a 30-foot-long, 5-foot-wide Line.\
Failure: 14 (4d6) Acid damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/259|page 259]].
