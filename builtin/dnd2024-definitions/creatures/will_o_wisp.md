---
splat_schema: 1
splat_id: be1743d1-eb35-553a-8b6c-392e94eb3535
splat_kind: creature
splat_key: dnd2024:creatures/will_o_wisp
splat_name: Will-o’-Wisp
splat_origin:
  key: dnd2024:reference/creature/will_o_wisp
splat_sources:
- title: SRD 5.2.1
  locator: pp. 341, 342
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: undead
  alignment: Chaotic Evil
  movement_text: 5 ft., Fly 50 ft. (hover)
  resistances_text: Acid, Bludgeoning, Cold, Fire, Necrotic, Piercing, Slashing
  immunities_text: Lightning, Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained,
    Unconscious
  senses_text: Darkvision 120 ft.; Passive Perception 12
  languages_text: Common plus one other language
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 27 (11d4)
splat_stats:
  strength:
    value: 1
  save_strength:
    value: -5
  dexterity:
    value: 28
  save_dexterity:
    value: 9
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 13
  save_intelligence:
    value: 1
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 19
  max_hp:
    value: 27
  initiative:
    value: 9
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/will_o_wisp
  implementation: dnd2024:creature_will_o_wisp
  parameters:
    features:
      ephemeral:
        economy: trait
        operation: passive_rule
        can_wear: false
        can_carry: false
      illumination:
        economy: trait
        operation: passive_rule
        bright_radius_ft: 20
        additional_dim_radius_ft: 20
      incorporeal_movement:
        economy: trait
        operation: passive_rule
        pass_through:
        - creatures
        - objects
        movement_terrain: difficult
        end_turn_inside_object_damage:
          roll:
            count: 1
            sides: 10
            modifier: 0
          type: force
      shock:
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
          - average: 11
            roll:
              count: 2
              sides: 8
              modifier: 2
            type: lightning
      consume_life:
        economy: bonus_action
        operation: saving_throw
        ability: constitution
        dc: 10
        targets:
          kind: creature
          count: 1
          range_ft: 5
          visible: true
          living: true
          hp: 0
        failure:
          target_dies: true
          heal_self:
            roll:
              count: 3
              sides: 6
              modifier: 0
            average: 10
        success:
          effects: []
      vanish:
        economy: bonus_action
        operation: apply_conditions
        targets:
        - self
        - self.light
        conditions:
        - invisible
        concentration: true
        ends_after:
        - self.attack_roll
        - self.use_consume_life
    damage_resistances:
    - acid
    - bludgeoning
    - cold
    - fire
    - necrotic
    - piercing
    - slashing
    damage_immunities:
    - lightning
    - poison
    condition_immunities:
    - exhaustion
    - grappled
    - paralyzed
    - petrified
    - poisoned
    - prone
    - restrained
    - unconscious
    senses:
      darkvision: 120
      passive_perception: 12
    languages:
      speaks:
      - common
      understands: []
      additional_languages:
        count: 1
        chooser: creator
        can_speak: true
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
- id: dnd2024:creature_will_o_wisp
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
  - attack.resolved
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
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 04ef82afc10a8eb9260b9fa6227806f3a48a541ad1a03ac9cc984f5c18084429
  source_path: builtin/srd-5.2.1/creature/will_o_wisp.md
  pages:
  - 341
  - 342
  engine_gaps: &id001
  - dnd2024:creature_will_o_wisp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 6ff5f23e2f91a2a76b2c2932370d53b1e0e978527d8967b27c4bc2b07b513bd5
splat_clauses:
- key: source_rules
  text: '# Will-o’-Wisp Tiny Undead, Chaotic Evil AC 19 Initiative +9 (19) HP 27 (11d4) Speed 5 ft., Fly
    50 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 1 -5 -5 Dex 28 +9 +9 Con10 +0 +0 Int 13 +1 +1 Wis 14
    +2 +2 Cha 11 +0 +0 Resistances Acid, Bludgeoning, Cold, Fire, Necrotic, Piercing, Slashing Immunities
    Lightning, Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious
    Senses Darkvision 120 ft.; Passive Perception 12 Languages Common plus one other language CR 2 (XP
    450; PB +2) ### Traits Ephemeral. The wisp can’t wear or carry anything. Illumination. The wisp sheds
    Bright Light in a 20-foot radius and Dim Light for an additional 20 feet. Incorporeal Movement. The
    wisp can move through other creatures and objects as if they were Difficult Terrain. It takes 5 (1d10)
    Force damage if it ends its turn inside an object. ### Actions Shock. Melee Attack Roll: +4, reach
    5 ft. Hit: 11 (2d8 + 2) Lightning damage. ### Bonus Actions Consume Life. Constitution Saving Throw:
    DC 10, one living creature the wisp can see within 5 feet that has 0 Hit Points. Failure: The target
    dies, and the wisp regains 10 (3d6) Hit Points. Vanish. The wisp and its light have the Invisible
    condition until the wisp’s Concentration ends on this effect, which ends early immediately after the
    wisp makes an attack roll or uses Consume Life.'
  sha256: c1aa31bd3c5ff2f7dd911baae76cf55110ff1fdc104da0d1f2e165e28e8ac4f5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Will-o’-Wisp

<!-- SRD page 341 -->

Tiny Undead, Chaotic Evil\
AC 19 Initiative +9 (19)\
HP 27 (11d4)\
Speed 5 ft., Fly 50 ft. (hover)\

<!-- SRD page 342 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 28 +9 +9 Con10 +0 +0\
Int 13 +1 +1 Wis 14 +2 +2 Cha 11 +0 +0\
Resistances Acid, Bludgeoning, Cold, Fire, Necrotic,\
Piercing, Slashing\
Immunities Lightning, Poison; Exhaustion, Grappled,\
Paralyzed, Petrified, Poisoned, Prone, Restrained,\
Unconscious\
Senses Darkvision 120 ft.; Passive Perception 12\
Languages Common plus one other language\
CR 2 (XP 450; PB +2)\

### Traits

Ephemeral. The wisp can’t wear or carry anything.\
Illumination. The wisp sheds Bright Light in a 20-foot\
radius and Dim Light for an additional 20 feet.\
Incorporeal Movement. The wisp can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\

### Actions

Shock. Melee Attack Roll: +4, reach 5 ft. Hit: 11 (2d8\
+ 2) Lightning damage.\

### Bonus Actions

Consume Life. Constitution Saving Throw: DC 10, one\
living creature the wisp can see within 5 feet that has 0\
Hit Points. Failure: The target dies, and the wisp regains\
10 (3d6) Hit Points.\
Vanish. The wisp and its light have the Invisible condi-\
tion until the wisp’s Concentration ends on this effect,\
which ends early immediately after the wisp makes an\
attack roll or uses Consume Life.\

Original layout: [[../../srd-5.2.1/pages/341|page 341]], [[../../srd-5.2.1/pages/342|page 342]].
