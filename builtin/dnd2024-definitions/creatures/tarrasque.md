---
splat_schema: 1
splat_id: e6888c77-0bcb-516f-a305-13de712c5bd1
splat_kind: creature
splat_key: dnd2024:creatures/tarrasque
splat_name: Tarrasque
splat_origin:
  key: dnd2024:reference/creature/tarrasque
splat_sources:
- title: SRD 5.2.1
  locator: pp. 331, 332
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: monstrosity
  alignment: Unaligned
  tags_text: (Titan)
  movement_text: 60 ft., Burrow 40 ft., Climb 60 ft.
  skills_text: Perception +9
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Fire, Poison; Charmed, Deafened, Frightened, Paralyzed, Poisoned
  senses_text: Blindsight 120 ft.; Passive Perception 19
  languages_text: None
  cr_text: 30 (XP 155,000; PB +9)
  hit_points_text: 697 (34d20 + 340)
splat_stats:
  strength:
    value: 30
  save_strength:
    value: 10
  dexterity:
    value: 11
  save_dexterity:
    value: 9
  constitution:
    value: 30
  save_constitution:
    value: 10
  intelligence:
    value: 3
  save_intelligence:
    value: 5
  wisdom:
    value: 11
  save_wisdom:
    value: 9
  charisma:
    value: 11
  save_charisma:
    value: 9
  armor_class:
    value: 25
  max_hp:
    value: 697
  initiative:
    value: 18
  speed:
    value: 60
    unit: ft
  burrow_speed:
    value: 40
    unit: ft
  climb_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/tarrasque
  implementation: dnd2024:creature_tarrasque
  parameters:
    features:
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 6
          recovery: long_rest
        operation: replace_save_result
        trigger: self.save_failed
        optional: true
        result: success
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      reflective_carapace:
        economy: trait
        operation: passive_rule
        trigger: self.targeted_by_spell
        spell_eligibility:
          any_of:
          - spell:
              key: dnd2024:spells/magic_missile
          - requires_ranged_attack_roll: true
        roll:
          die: 6
          results:
          - faces:
            - 1
            - 2
            - 3
            - 4
            - 5
            self_affected: false
          - faces:
            - 6
            self_affected: false
            reflect_to: caster
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
          feature: bite
          count: 1
        - operation: attack_choice
          count: 3
          options:
          - claw
          - tail
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 19
          reach_ft: 15
        hit:
          damage:
          - average: 36
            roll:
              count: 4
              sides: 12
              modifier: 10
            type: piercing
          grapple:
            condition: grappled
            source: self
            escape_dc: 20
            while_grappled:
              conditions:
              - restrained
              can_teleport: false
      claw:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 19
          reach_ft: 15
        hit:
          damage:
          - average: 28
            roll:
              count: 4
              sides: 8
              modifier: 10
            type: slashing
      tail:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 19
          reach_ft: 30
        hit:
          damage:
          - average: 23
            roll:
              count: 3
              sides: 8
              modifier: 10
            type: bludgeoning
          prone:
            condition: prone
            maximum_size: huge
      thunderous_bellow:
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
        ability: constitution
        dc: 27
        targets:
          kinds:
          - creature
          - object
          objects_exclude:
          - worn
          - carried
          selection: each
          area:
            shape: cone
            size_ft: 150
            origin: self
        failure:
          damage:
          - average: 78
            roll:
              count: 12
              sides: 12
              modifier: 0
            type: thunder
          conditions:
          - condition: deafened
            until: target.next_turn_end
          - condition: frightened
            until: target.next_turn_end
        success:
          damage_multiplier: 0.5
          other_effects: false
      swallow:
        economy: bonus_action
        operation: saving_throw
        ability: strength
        dc: 27
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
                  count: 16
                  sides: 6
                  modifier: 0
                type: acid
            regurgitate:
              trigger: turn_end
              if_damage_on_single_turn_from_one_inside_creature_at_least: 60
              save:
                ability: constitution
                dc: 20
                failure:
                  eject: all
                  within_ft: 10
                  conditions:
                  - prone
            on_self_death:
              remove_conditions:
              - restrained
              escape_movement_ft: 20
              exit_condition: prone
            can_teleport: false
        targets:
          kind: creature
          count: 1
          grappled_by: self
          maximum_size: large
        capacity: 6
        success:
          effects: []
      onslaught:
        economy: legendary_action
        legendary_cost: 1
        operation: sequence
        effects:
        - operation: move
          maximum_ft:
            multiply:
            - stat: speed
            - 0.5
        - operation: attack_choice
          count: 1
          options:
          - claw
          - tail
      world_shaking_movement:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: sequence
        effects:
        - operation: move
          maximum_ft:
            multiply:
            - stat: speed
            - 1
        - operation: shock_wave
          area:
            shape: emanation
            size_ft: 60
            origin: self
          duration: instantaneous
          targets: creatures
          end_concentration: true
          prone:
            maximum_size: medium
    skill_totals:
      perception: 9
    damage_resistances:
    - bludgeoning
    - piercing
    - slashing
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - charmed
    - deafened
    - frightened
    - paralyzed
    - poisoned
    senses:
      blindsight: 120
      passive_perception: 19
    languages:
      speaks: []
      understands: []
    legendary_actions:
      uses: 3
      recovery: self.turn_start
      trigger: other_creature.turn_end
      cost_per_action: 1
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
- id: dnd2024:creature_tarrasque
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
  - attack.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - grappling
  - teleportation
  - spatial_queries
  - limited_resources
  - legendary_actions
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 516b5b36fbca2a26bc4c74cf85a83fb81c13e232429842225ec78cf0661b31cd
  source_path: builtin/srd-5.2.1/creature/tarrasque.md
  pages:
  - 331
  - 332
  engine_gaps: &id001
  - dnd2024:creature_tarrasque
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: e6bed3e16a7decd617d862c229ec4ed6ad6046a8297dd63cda1845700d2c0b53
splat_clauses:
- key: source_rules
  text: '# Tarrasque Gargantuan Monstrosity (Titan), Unaligned AC 25 Initiative +18 (28) HP 697 (34d20
    + 340) Speed 60 ft., Burrow 40 ft., Climb 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 30 +10 +10 Dex 11
    +0 +9 Con 30 +10 +10 Int 3 -4 +5 Wis 11 +0 +9 Cha 11 +0 +9 Skills Perception +9 Resistances Bludgeoning,
    Piercing, Slashing Immunities Fire, Poison; Charmed, Deafened, Frightened, Paralyzed, Poisoned Senses
    Blindsight 120 ft.; Passive Perception 19 Languages None CR 30 (XP 155,000; PB +9) ### Traits Legendary
    Resistance (6/Day). If the tarrasque fails a saving throw, it can choose to succeed instead. Magic
    Resistance. The tarrasque has Advantage on saving throws against spells and other magical effects.
    Reflective Carapace. If the tarrasque is targeted by a Magic Missile spell or a spell that requires
    a ranged attack roll, roll 1d6. On a 1–5, the tarrasque is unaffected. On a 6, the tarrasque is unaffected
    and reflects the spell, turning the caster into the target. Siege Monster. The tarrasque deals double
    damage to objects and structures. ### Actions Multiattack. The tarrasque makes one Bite attack and
    three other attacks, using Claw or Tail in any combination. Bite. Melee Attack Roll: +19, reach 15
    ft. Hit: 36 (4d12 + 10) Piercing damage, and the target has the Grappled condition (escape DC 20).
    Until the grapple ends, the target has the Restrained condition and can’t teleport. Claw. Melee Attack
    Roll: +19, reach 15 ft. Hit: 28 (4d8 + 10) Slashing damage. Tail. Melee Attack Roll: +19, reach 30
    ft. Hit: 23 (3d8 + 10) Bludgeoning damage. If the target is a Huge or smaller creature, it has the
    Prone condition. Thunderous Bellow (Recharge 5–6). Constitution Saving Throw: DC 27, each creature
    and each object that isn’t being worn or carried in a 150-foot Cone. Failure: 78 (12d12) Thunder damage,
    and the target has the Deafened and Frightened conditions until the end of its next turn. Success:
    Half damage only. ### Bonus Actions Swallow. Strength Saving Throw: DC 27, one Large or smaller creature
    Grappled by the tarrasque (it can have up to six creatures swallowed at a time). Failure: The target
    is swallowed, and the Grappled condition ends. A swallowed creature has the Blinded and Restrained
    conditions and can’t teleport, it has Total Cover against attacks and other effects outside the tarrasque,
    and it takes 56 (16d6) Acid damage at the start of each of the tarrasque’s turns. If the tarrasque
    takes 60 damage or more on a single turn from a creature inside it, the tarrasque must succeed on
    a DC 20 Constitution saving throw at the end of that turn or regurgitate all swallowed creatures,
    each of which falls in a space within 10 feet of the tarrasque and has the Prone condition. If the
    tarrasque dies, any swallowed creature no longer has the Restrained condition and can escape from
    the corpse using 20 feet of movement, exiting Prone. ### Legendary Actions Legendary Action Uses:
    3. Immediately after another creature’s turn, the tarrasque can expend a use to take one of the following
    actions. The tarrasque regains all expended uses at the start of each of its turns. Onslaught. The
    tarrasque moves up to half its Speed, and it makes one Claw or Tail attack. World-Shaking Movement.
    The tarrasque moves up to its Speed. At the end of this movement, the tarrasque creates an instantaneous
    shock wave in a 60-foot Emanation originating from itself. Creatures in that area lose Concentration
    and, if Medium or smaller, have the Prone condition. The tarrasque can’t take this action again until
    the start of its next turn.'
  sha256: 7c57ffc1e9edd17ac8ff78918a86f8b2d6b757b16bd38b8dfd5961c3838f5426
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tarrasque

<!-- SRD page 331 -->

Gargantuan Monstrosity (Titan), Unaligned\
AC 25 Initiative +18 (28)\
HP 697 (34d20 + 340)\
Speed 60 ft., Burrow 40 ft., Climb 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +10 Dex 11 +0 +9 Con 30 +10 +10\
Int 3 −4 +5 Wis 11 +0 +9 Cha 11 +0 +9\
Skills Perception +9\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Fire, Poison; Charmed, Deafened, Frightened,\
Paralyzed, Poisoned\
Senses Blindsight 120 ft.; Passive Perception 19\
Languages None\
CR 30 (XP 155,000; PB +9)\

### Traits

Legendary Resistance (6/Day). If the tarrasque fails a\
saving throw, it can choose to succeed instead.\
Magic Resistance. The tarrasque has Advantage on sav-\
ing throws against spells and other magical effects.\
Reflective Carapace. If the tarrasque is targeted by a\
Magic Missile spell or a spell that requires a ranged\
attack roll, roll 1d6. On a 1–5, the tarrasque is unaf-\
fected. On a 6, the tarrasque is unaffected and reflects\
the spell, turning the caster into the target.\
Siege Monster. The tarrasque deals double damage to\
objects and structures.\

### Actions

Multiattack. The tarrasque makes one Bite attack\
and three other attacks, using Claw or Tail in any\
combination.\
Bite. Melee Attack Roll: +19, reach 15 ft. Hit: 36 (4d12\
+ 10) Piercing damage, and the target has the Grappled\

<!-- SRD page 332 -->

condition (escape DC 20). Until the grapple ends, the\
target has the Restrained condition and can’t teleport.\
Claw. Melee Attack Roll: +19, reach 15 ft. Hit: 28 (4d8\
+ 10) Slashing damage.\
Tail. Melee Attack Roll: +19, reach 30 ft. Hit: 23 (3d8\
+ 10) Bludgeoning damage. If the target is a Huge or\
smaller creature, it has the Prone condition.\
Thunderous Bellow (Recharge 5–6). Constitution Sav-\
ing Throw: DC 27, each creature and each object that\
isn’t being worn or carried in a 150-foot Cone. Failure:\
78 (12d12) Thunder damage, and the target has the\
Deafened and Frightened conditions until the end of its\
next turn. Success: Half damage only.\

### Bonus Actions

Swallow. Strength Saving Throw: DC 27, one Large or\
smaller creature Grappled by the tarrasque (it can have\
up to six creatures swallowed at a time). Failure: The\
target is swallowed, and the Grappled condition ends.\
A swallowed creature has the Blinded and Restrained\
conditions and can’t teleport, it has Total Cover against\
attacks and other effects outside the tarrasque, and it\
takes 56 (16d6) Acid damage at the start of each of the\
tarrasque’s turns.\
If the tarrasque takes 60 damage or more on a single\
turn from a creature inside it, the tarrasque must suc-\
ceed on a DC 20 Constitution saving throw at the end\
of that turn or regurgitate all swallowed creatures, each\
of which falls in a space within 10 feet of the tarrasque\
and has the Prone condition. If the tarrasque dies, any\
swallowed creature no longer has the Restrained con-\
dition and can escape from the corpse using 20 feet of\
movement, exiting Prone.\

### Legendary Actions

Legendary Action Uses: 3. Immediately after another\
creature’s turn, the tarrasque can expend a use to take\
one of the following actions. The tarrasque regains all\
expended uses at the start of each of its turns.\
Onslaught. The tarrasque moves up to half its Speed,\
and it makes one Claw or Tail attack.\
World-Shaking Movement. The tarrasque moves up to\
its Speed. At the end of this movement, the tarrasque\
creates an instantaneous shock wave in a 60-foot Em-\
anation originating from itself. Creatures in that area\
lose Concentration and, if Medium or smaller, have the\
Prone condition. The tarrasque can’t take this action\
again until the start of its next turn.\

Original layout: [[../../srd-5.2.1/pages/331|page 331]], [[../../srd-5.2.1/pages/332|page 332]].
