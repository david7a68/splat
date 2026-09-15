---
splat_schema: 1
splat_id: d989aa4d-b3f7-5d11-88ff-6def51fa42e1
splat_kind: creature
splat_key: dnd2024:creatures/steam_mephit
splat_name: Steam Mephit
splat_origin:
  key: dnd2024:reference/creature/steam_mephit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 308
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: elemental
  alignment: Neutral Evil
  movement_text: 30 ft., Fly 30 ft.
  skills_text: Stealth +2
  immunities_text: Fire, Poison; Exhaustion, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Aquan, Ignan)
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 17 (5d6)
splat_stats:
  strength:
    value: 5
  save_strength:
    value: -3
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 10
  max_hp:
    value: 17
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/steam_mephit
  implementation: dnd2024:creature_steam_mephit
  parameters:
    skill_totals:
      stealth: 2
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - exhaustion
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 10
    languages:
      speak: true
      known:
      - primordial
      primordial_dialects:
      - aquan
      - ignan
    features:
      blurred_form:
        economy: passive
        op: roll.disadvantage
        rolls:
        - attack_against_actor
        unless:
          actor_condition: incapacitated
      claw:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 2
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            average: 2
          - type: fire
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            average: 2
      death_burst:
        economy: passive
        op: saving_throw
        ability: dexterity
        dc: 10
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            origin: actor
            size_ft: 5
        failure:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 2
                sides: 4
              add: 0
            average: 5
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
        trigger: actor.died
      steam_breath:
        economy: action
        op: saving_throw
        ability: constitution
        dc: 10
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 15
        failure:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 2
                sides: 4
              add: 0
            average: 5
        - op: speed.add
          amount_ft: -10
          duration:
            until: actor.next_turn_end
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
        ignore_resistance_sources:
        - underwater
        recharge:
          die_sides: 6
          success_min: 6
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_steam_mephit
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - actor.died
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: d8ff314af0ca9fa94b410c87c2bf9e763f41376f3a156cd4a2cc3e417a4b1392
  source_path: builtin/srd-5.2.1/creature/steam_mephit.md
  pages:
  - 308
  engine_gaps: &id001
  - dnd2024:creature_steam_mephit
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: e4b5fdde9a28d39a8d969ed01df1d3101827e3210cc4a5a5cd281f6aae011670
splat_clauses:
- key: source_rules
  text: '# Steam Mephit Small Elemental, Neutral Evil AC 10 Initiative +0 (10) HP 17 (5d6) Speed 30 ft.,
    Fly 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 5 -3 -3 Dex 11 +0 +0 Con 10 +0 +0 Int 11 +0 +0 Wis 10 +0
    +0 Cha 12 +1 +1 Skills Stealth +2 Immunities Fire, Poison; Exhaustion, Poisoned Senses Darkvision
    60 ft.; Passive Perception 10 Languages Primordial (Aquan, Ignan) CR 1/4 (XP 50; PB +2) ### Traits
    Blurred Form. Attack rolls against the mephit are made with Disadvantage unless the mephit has the
    Incapacitated condition. Death Burst. The mephit explodes when it dies. Dexterity Saving Throw: DC
    10, each creature in a 5-foot Emanation originating from the mephit. Failure: 5 (2d4) Fire damage.
    Success: Half damage. ### Actions Claw. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4) Slashing damage
    plus 2 (1d4) Fire damage. Steam Breath (Recharge 6). Constitution Saving Throw: DC 10, each creature
    in a 15-foot Cone. Failure: 5 (2d4) Fire damage, and the target’s Speed decreases by 10 feet until
    the end of the mephit’s next turn. Success: Half damage only. Failure or Success: Being underwater
    doesn’t grant Resistance to this Fire damage.'
  sha256: dcc472a338ad5b25fb90e1c99ed642fe471c9bccf4aafd57071d215cb8dabc8e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Steam Mephit

<!-- SRD page 308 -->

Small Elemental, Neutral Evil\
AC 10 Initiative +0 (10)\
HP 17 (5d6)\
Speed 30 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex 11 +0 +0 Con 10 +0 +0\
Int 11 +0 +0 Wis 10 +0 +0 Cha 12 +1 +1\
Skills Stealth +2\
Immunities Fire, Poison; Exhaustion, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Aquan, Ignan)\
CR 1/4 (XP 50; PB +2)\

### Traits

Blurred Form. Attack rolls against the mephit are made\
with Disadvantage unless the mephit has the Incapaci-\
tated condition.\
Death Burst. The mephit explodes when it dies. Dex-\
terity Saving Throw: DC 10, each creature in a 5-foot\
Emanation originating from the mephit. Failure: 5 (2d4)\
Fire damage. Success: Half damage.\

### Actions

Claw. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4)\
Slashing damage plus 2 (1d4) Fire damage.\
Steam Breath (Recharge 6). Constitution Saving Throw:\
DC 10, each creature in a 15-foot Cone. Failure: 5 (2d4)\
Fire damage, and the target’s Speed decreases by 10\
feet until the end of the mephit’s next turn. Success:\
Half damage only. Failure or Success: Being underwater\
doesn’t grant Resistance to this Fire damage.\

Original layout: [[../../srd-5.2.1/pages/308|page 308]].
