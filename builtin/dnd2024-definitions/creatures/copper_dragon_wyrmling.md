---
splat_schema: 1
splat_id: 70559458-749b-5963-9488-df9d343109c8
splat_kind: creature
splat_key: dnd2024:creatures/copper_dragon_wyrmling
splat_name: Copper Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/copper_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 275, 276
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 30 ft., Climb 30 ft., Fly 60 ft.
  skills_text: Perception +4, Stealth +3
  immunities_text: Acid
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (4d8 + 4)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 16
  max_hp:
    value: 22
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/copper_dragon_wyrmling
  implementation: dnd2024:creature_copper_dragon_wyrmling
  parameters:
    features:
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 7
            roll:
              count: 1
              sides: 10
              modifier: 2
            type: slashing
      acid_breath:
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
        dc: 11
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 20
            origin: self
            width_ft: 5
        failure:
          damage:
          - average: 18
            roll:
              count: 4
              sides: 8
              modifier: 0
            type: acid
        success:
          damage_multiplier: 0.5
          other_effects: false
      slowing_breath:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 11
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 15
            origin: self
        failure:
          can_take_reactions: false
          speed_multiplier: 0.5
          turn_economy:
            choose_one:
            - action
            - bonus_action
          until: target.next_turn_end
        success:
          effects: []
    skill_totals:
      perception: 4
      stealth: 3
    damage_immunities:
    - acid
    condition_immunities: []
    senses:
      blindsight: 10
      darkvision: 60
      passive_perception: 14
    languages:
      speaks:
      - draconic
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
- id: dnd2024:creature_copper_dragon_wyrmling
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
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 7b9e62976c42cd7243cfa17bcdb7088707362a6c6699a1c36951cbdfbb5ad4e2
  source_path: builtin/srd-5.2.1/creature/copper_dragon_wyrmling.md
  pages:
  - 275
  - 276
  engine_gaps: &id001
  - dnd2024:creature_copper_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: fffe728603e10086cce739653844203aa12ce41cdc01d87ef3b86343959bd90d
splat_clauses:
- key: source_rules
  text: '# Copper Dragon Wyrmling Medium Dragon (Metallic), Chaotic Good AC 16 Initiative +3 (13) HP 22
    (4d8 + 4) Speed 30 ft., Climb 30 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 12 +1
    +3 Con 13 +1 +1 Int 14 +2 +2 Wis 11 +0 +2 Cha 13 +1 +1 Skills Perception +4, Stealth +3 Immunities
    Acid Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic CR 1 (XP
    200; PB +2) ### Actions Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10 + 2) Slashing damage.
    Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 11, each creature in a 20-foot-long, 5-footwide
    Line. Failure: 18 (4d8) Acid damage. Success: Half damage. Slowing Breath. Constitution Saving Throw:
    DC 11, each creature in a 15-foot Cone. Failure: The target can’t take Reactions; its Speed is halved;
    and it can take either an action or a Bonus Action on its turn, not both. This effect lasts until
    the end of its next turn.'
  sha256: 5608ff0480230bc97f29e8ebbe6df2e3ccf636bee387e0bb4f8494070c82fc7c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Copper Dragon Wyrmling

<!-- SRD page 275 -->

Medium Dragon (Metallic), Chaotic Good\
AC 16 Initiative +3 (13)\
HP 22 (4d8 + 4)\
Speed 30 ft., Climb 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +3 Con 13 +1 +1\
Int 14 +2 +2 Wis 11 +0 +2 Cha 13 +1 +1\
Skills Perception +4, Stealth +3\
Immunities Acid\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 1 (XP 200; PB +2)\

<!-- SRD page 276 -->


### Actions

Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10\
+ 2) Slashing damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 11, each creature in a 20-foot-long, 5-foot-\
wide Line. Failure: 18 (4d8) Acid damage. Success:\
Half damage.\
Slowing Breath. Constitution Saving Throw: DC 11,\
each creature in a 15-foot Cone. Failure: The target\
can’t take Reactions; its Speed is halved; and it can take\
either an action or a Bonus Action on its turn, not both.\
This effect lasts until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/275|page 275]], [[../../srd-5.2.1/pages/276|page 276]].
