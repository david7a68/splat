---
splat_schema: 1
splat_id: 33a71319-c74a-5cb6-9de9-2cd913a6f3f4
splat_kind: creature
splat_key: dnd2024:creatures/kraken
splat_name: Kraken
splat_origin:
  key: dnd2024:reference/creature/kraken
splat_sources:
- title: SRD 5.2.1
  locator: pp. 303
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: monstrosity
  alignment: Chaotic Evil
  tags_text: (Titan)
  movement_text: 30 ft., Swim 120 ft.
  skills_text: History +13, Perception +11
  immunities_text: Cold, Lightning; Frightened, Grappled, Paralyzed, Restrained
  senses_text: Truesight 120 ft.; Passive Perception 21
  languages_text: Understands Abyssal, Celestial, Infernal, and Primordial but can’t speak; telepathy
    120 ft.
  cr_text: 23 (XP 50,000, or 62,000 in lair; PB +7)
  hit_points_text: 481 (26d20 + 208)
splat_stats:
  strength:
    value: 30
  save_strength:
    value: 17
  dexterity:
    value: 11
  save_dexterity:
    value: 7
  constitution:
    value: 26
  save_constitution:
    value: 15
  intelligence:
    value: 22
  save_intelligence:
    value: 6
  wisdom:
    value: 18
  save_wisdom:
    value: 11
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 18
  max_hp:
    value: 481
  initiative:
    value: 14
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 120
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/kraken
  implementation: dnd2024:creature_kraken
  parameters:
    features:
      amphibious:
        economy: trait
        operation: passive_rule
        breathing:
        - air
        - water
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 4
          recovery: long_rest
          lair_uses: 5
        operation: replace_save_result
        trigger: self.save_failed
        optional: true
        result: success
      siege_monster:
        economy: trait
        operation: passive_rule
        damage_multiplier: 2
        target_kinds:
        - object
        - structure
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: tentacle
          count: 2
        - operation: choice
          options:
          - operation: use_feature
            feature: fling
          - operation: use_feature
            feature: lightning_strike
          - operation: use_feature
            feature: swallow
      tentacle:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 17
          reach_ft: 30
        hit:
          damage:
          - average: 24
            roll:
              count: 4
              sides: 6
              modifier: 10
            type: bludgeoning
          grapple:
            condition: grappled
            source: self
            escape_dc: 20
            appendage_pool:
              count: 10
              type: tentacles
              cost: 1
            while_grappled:
              conditions:
              - restrained
      fling:
        economy: action
        operation: saving_throw
        ability: dexterity
        dc: 25
        targets:
          selection: thrown_creature_and_creatures_in_destination
        failure:
          damage:
          - roll:
              count: 4
              sides: 8
              modifier: 0
            type: bludgeoning
          conditions:
          - condition: prone
        success:
          damage_multiplier: 0.5
          other_effects: false
        before_save:
          throw:
            target:
              grappled_by: self
              maximum_size: large
            destination:
              self_visible: true
              within_self_ft: 60
              in_air: false
      lightning_strike:
        economy: action
        operation: saving_throw
        ability: dexterity
        dc: 23
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible: true
        failure:
          damage:
          - average: 33
            roll:
              count: 6
              sides: 10
              modifier: 0
            type: lightning
        success:
          damage_multiplier: 0.5
          other_effects: false
      swallow:
        economy: action
        operation: saving_throw
        ability: dexterity
        dc: 25
        failure:
          end_grapple: true
          swallow:
            maximum_size: large
            conditions:
            - restrained
            cover_against_outside: total
            periodic_damage:
              event: target.turn_start
              damage:
              - roll:
                  count: 7
                  sides: 6
                  modifier: 0
                type: acid
            regurgitate:
              trigger: turn_end
              if_damage_on_single_turn_from_one_inside_creature_at_least: 50
              save:
                ability: constitution
                dc: 25
                failure:
                  eject: all
                  within_ft: 10
                  conditions:
                  - prone
            on_self_death:
              remove_conditions:
              - restrained
              escape_movement_ft: 15
              exit_condition: prone
          damage:
          - roll:
              count: 3
              sides: 8
              modifier: 10
            type: piercing
          end_grapple_requires_swallowed: true
        targets:
          kind: creature
          count: 1
          grappled_by: self
        capacity: 4
        success:
          effects: []
      storm_bolt:
        economy: legendary_action
        legendary_cost: 1
        operation: use_feature
        feature: lightning_strike
      toxic_ink:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: saving_throw
        ability: constitution
        dc: 23
        targets:
          kind: creature
          selection: each
          area:
            shape: emanation
            size_ft: 15
            origin: self
        failure:
          conditions:
          - condition: blinded
            until: self.next_turn_end
          - condition: poisoned
            until: self.next_turn_end
          then_move:
            operation: move
            maximum_ft:
              multiply:
              - stat: speed
              - 1
        requires:
          self_underwater: true
        success:
          effects: []
    skill_totals:
      history: 13
      perception: 11
    damage_immunities:
    - cold
    - lightning
    condition_immunities:
    - frightened
    - grappled
    - paralyzed
    - restrained
    senses:
      truesight: 120
      passive_perception: 21
    languages:
      speaks: []
      understands:
      - abyssal
      - celestial
      - infernal
      - primordial
      telepathy_range_ft: 120
    legendary_actions:
      uses: 3
      recovery: self.turn_start
      trigger: other_creature.turn_end
      cost_per_action: 1
      lair_uses: 4
      requires:
        not_incapacitated: true
      maximum_actions_per_trigger: 1
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
- id: dnd2024:creature_kraken
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
  - rest.completed
  - saving_throw.resolved
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
  - legendary_actions
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 73f5f049a9dd06b3c569d71a3fd3246698650d69ae02cc86c4a2d94135d0f262
  source_path: builtin/srd-5.2.1/creature/kraken.md
  pages:
  - 303
  engine_gaps: &id001
  - dnd2024:creature_kraken
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: ed7f0d2ecbe41e2609e38b8eeb47bbfbc9bdb9274e4e47b7625e8f76896d406f
splat_clauses:
- key: source_rules
  text: '# Kraken Gargantuan Monstrosity (Titan), Chaotic Evil AC 18 Initiative +14 (24) HP 481 (26d20
    + 208) Speed 30 ft., Swim 120 ft. MOD SAVE MOD SAVE MOD SAVE Str 30 +10 +17 Dex 11 +0 +7 Con 26 +8
    +15 Int 22 +6 +6 Wis 18 +4 +11 Cha 20 +5 +5 Skills History +13, Perception +11 Immunities Cold, Lightning;
    Frightened, Grappled, Paralyzed, Restrained Senses Truesight 120 ft.; Passive Perception 21 Languages
    Understands Abyssal, Celestial, Infernal, and Primordial but can’t speak; telepathy 120 ft. CR 23
    (XP 50,000, or 62,000 in lair; PB +7) ### Traits Amphibious. The kraken can breathe air and water.
    Legendary Resistance (4/Day, or 5/Day in Lair). If the kraken fails a saving throw, it can choose
    to succeed instead. Siege Monster. The kraken deals double damage to objects and structures. ### Actions
    Multiattack. The kraken makes two Tentacle attacks and uses Fling, Lightning Strike, or Swallow. Tentacle.
    Melee Attack Roll: +17, reach 30 ft. Hit: 24 (4d6 + 10) Bludgeoning damage. The target has the Grappled
    condition (escape DC 20) from one of ten tentacles, and it has the Restrained condition until the
    grapple ends. Fling. The kraken throws a Large or smaller creature Grappled by it to a space it can
    see within 60 feet of itself that isn’t in the air. Dexterity Saving Throw: DC 25, the creature thrown
    and each creature in the destination space. Failure: 18 (4d8) Bludgeoning damage, and the target has
    the Prone condition. Success: Half damage only. Lightning Strike. Dexterity Saving Throw: DC 23, one
    creature the kraken can see within 120 feet. Failure: 33 (6d10) Lightning damage. Success: Half damage.
    Swallow. Dexterity Saving Throw: DC 25, one creature Grappled by the kraken (it can have up to four
    creatures swallowed at a time). Failure: 23 (3d8 + 10) Piercing damage. If the target is Large or
    smaller, it is swallowed and no longer Grappled. A swallowed creature has the Restrained condition,
    has Total Cover against attacks and other effects outside the kraken, and takes 24 (7d6) Acid damage
    at the start of each of its turns. If the kraken takes 50 damage or more on a single turn from a creature
    inside it, the kraken must succeed on a DC 25 Constitution saving throw at the end of that turn or
    regurgitate all swallowed creatures, each of which falls in a space within 10 feet of the kraken with
    the Prone condition. If the kraken dies, any swallowed creature no longer has the Restrained condition
    and can escape from the corpse using 15 feet of movement, exiting Prone. ### Legendary Actions Legendary
    Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the kraken can expend a use
    to take one of the following actions. The kraken regains all expended uses at the start of each of
    its turns. Storm Bolt. The kraken uses Lightning Strike. Toxic Ink. Constitution Saving Throw: DC
    23, each creature in a 15-foot Emanation originating from the kraken while it is underwater. Failure:
    The target has the Blinded and Poisoned conditions until the end of the kraken’s next turn. The kraken
    then moves up to its Speed. Failure or Success: The kraken can’t take this action again until the
    start of its next turn.'
  sha256: 873dee7ef210292837a6c85cac28b77fd19c8799aca14970331ad578c0d9377c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Kraken

<!-- SRD page 303 -->

Gargantuan Monstrosity (Titan), Chaotic Evil\
AC 18 Initiative +14 (24)\
HP 481 (26d20 + 208)\
Speed 30 ft., Swim 120 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +17 Dex 11 +0 +7 Con 26 +8 +15\
Int 22 +6 +6 Wis 18 +4 +11 Cha 20 +5 +5\
Skills History +13, Perception +11\
Immunities Cold, Lightning; Frightened, Grappled,\
Paralyzed, Restrained\
Senses Truesight 120 ft.; Passive Perception 21\
Languages Understands Abyssal, Celestial, Infernal, and\
Primordial but can’t speak; telepathy 120 ft.\
CR 23 (XP 50,000, or 62,000 in lair; PB +7)\

### Traits

Amphibious. The kraken can breathe air and water.\
Legendary Resistance (4/Day, or 5/Day in Lair). If\
the kraken fails a saving throw, it can choose to suc-\
ceed instead.\
Siege Monster. The kraken deals double damage to ob-\
jects and structures.\

### Actions

Multiattack. The kraken makes two Tentacle attacks\
and uses Fling, Lightning Strike, or Swallow.\
Tentacle. Melee Attack Roll: +17, reach 30 ft. Hit: 24\
(4d6 + 10) Bludgeoning damage. The target has the\
Grappled condition (escape DC 20) from one of ten\
tentacles, and it has the Restrained condition until the\
grapple ends.\
Fling. The kraken throws a Large or smaller creature\
Grappled by it to a space it can see within 60 feet of\
itself that isn’t in the air. Dexterity Saving Throw: DC\
25, the creature thrown and each creature in the des-\
tination space. Failure: 18 (4d8) Bludgeoning damage,\
and the target has the Prone condition. Success: Half\
damage only.\
Lightning Strike. Dexterity Saving Throw: DC 23, one\
creature the kraken can see within 120 feet. Failure: 33\
(6d10) Lightning damage. Success: Half damage.\
Swallow. Dexterity Saving Throw: DC 25, one creature\
Grappled by the kraken (it can have up to four crea-\
tures swallowed at a time). Failure: 23 (3d8 + 10) Pierc-\
ing damage. If the target is Large or smaller, it is swal-\
lowed and no longer Grappled. A swallowed creature\
has the Restrained condition, has Total Cover against\
attacks and other effects outside the kraken, and takes\
24 (7d6) Acid damage at the start of each of its turns.\
If the kraken takes 50 damage or more on a single\
turn from a creature inside it, the kraken must succeed\
on a DC 25 Constitution saving throw at the end of\
that turn or regurgitate all swallowed creatures, each of\
which falls in a space within 10 feet of the kraken with\
the Prone condition. If the kraken dies, any swallowed\
creature no longer has the Restrained condition and\
can escape from the corpse using 15 feet of movement,\
exiting Prone.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the kraken can expend a use to\
take one of the following actions. The kraken regains all\
expended uses at the start of each of its turns.\
Storm Bolt. The kraken uses Lightning Strike.\
Toxic Ink. Constitution Saving Throw: DC 23, each\
creature in a 15-foot Emanation originating from the\
kraken while it is underwater. Failure: The target has\
the Blinded and Poisoned conditions until the end of\
the kraken’s next turn. The kraken then moves up to\
its Speed. Failure or Success: The kraken can’t take this\
action again until the start of its next turn.\

Original layout: [[../../srd-5.2.1/pages/303|page 303]].
