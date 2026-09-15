---
splat_schema: 1
splat_id: e6484aed-61b8-567b-835d-4f18c472deb2
splat_kind: creature
splat_key: dnd2024:creatures/half_dragon
splat_name: Half-Dragon
splat_origin:
  key: dnd2024:reference/creature/half_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 297
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Neutral
  movement_text: 40 ft.
  skills_text: Athletics +7, Perception +5, Stealth +5
  resistances_text: Damage type chosen for the Draconic Origin trait below
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 15
  languages_text: Common, Draconic
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 105 (14d8 + 42)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 5
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 15
  save_wisdom:
    value: 5
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 18
  max_hp:
    value: 105
  initiative:
    value: 5
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/half_dragon
  implementation: dnd2024:creature_half_dragon
  parameters:
    skill_totals:
      athletics: 7
      perception: 5
      stealth: 5
    damage_resistances:
    - parameter: draconic_origin
    senses:
    - sense: blindsight
      range_ft: 10
    - sense: darkvision
      range_ft: 60
    passive_perception: 15
    languages:
      speak: true
      known:
      - common
      - draconic
    features:
      claw:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 7
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 4
              add: 4
            average: 6
          - type:
              parameter: draconic_origin
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
      leap:
        economy: bonus_action
        op: movement.jump
        maximum_ft: 30
        movement_cost_ft: 10
      draconic_origin:
        economy: passive
        op: choice.bind
        parameter: draconic_origin
        chooser: gm
        choices:
        - acid
        - cold
        - fire
        - lightning
        - poison
        affects:
        - damage_resistance
        - claw_damage_type
        - dragons_breath_damage_type
      dragons_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 14
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 30
        failure:
        - op: damage
          components:
          - type:
              parameter: draconic_origin
            amount:
              dice:
                count: 8
                sides: 6
              add: 0
            average: 28
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
        recharge:
          die_sides: 6
          success_min: 5
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: claw
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
- id: dnd2024:creature_half_dragon
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
  source_sha256: dd7bc26472201e11e27f8667d2715634440a7ff4639669d1e3cb326fe25b24a6
  source_path: builtin/srd-5.2.1/creature/half_dragon.md
  pages:
  - 297
  engine_gaps: &id001
  - dnd2024:creature_half_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 54a1db0b42d6e64e8de8576179f64bc575d0ee0ecca25bf1d1c4abf1bde18fb3
splat_clauses:
- key: source_rules
  text: '# Half-Dragon Medium Dragon, Neutral AC 18 Initiative +5 (15) HP 105 (14d8 + 42) Speed 40 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 14 +2 +5 Con 16 +3 +3 Int 10 +0 +0 Wis 15 +2 +5 Cha 14
    +2 +2 Skills Athletics +7, Perception +5, Stealth +5 Resistances Damage type chosen for the Draconic
    Origin trait below Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 15 Languages Common,
    Draconic CR 5 (XP 1,800; PB +3) ### Traits Draconic Origin. The half-dragon is related to a type of
    dragon associated with one of the following damage types (GM’s choice): Acid, Cold, Fire, Lightning,
    or Poison. This choice affects other aspects of the stat block. ### Actions Multiattack. The half-dragon
    makes two Claw attacks. Claw. Melee Attack Roll: +7, reach 10 ft. Hit: 6 (1d4 + 4) Slashing damage
    plus 7 (2d6) damage of the type chosen for the Draconic Origin trait. Dragon’s Breath (Recharge 5–6).
    Dexterity Saving Throw: DC 14, each creature in a 30-foot Cone. Failure: 28 (8d6) damage of the type
    chosen for the Draconic Origin trait. Success: Half damage. ### Bonus Actions Leap. The half-dragon
    jumps up to 30 feet by spending 10 feet of movement.'
  sha256: d6aa2bcd3622993a2eb4d3bb0d073e39d6aea67ba76e4387a8b2a8dc8033e8f7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Half-Dragon

<!-- SRD page 297 -->

Medium Dragon, Neutral\
AC 18  Initiative +5 (15)\
HP 105 (14d8 + 42)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +5 Con 16 +3 +3\
Int 10 +0 +0 Wis 15 +2 +5 Cha 14 +2 +2\
Skills Athletics +7, Perception +5, Stealth +5\
Resistances Damage type chosen for the Draconic Origin\
trait below\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 15\
Languages Common, Draconic\
CR 5 (XP 1,800; PB +3)\

### Traits

Draconic Origin. The half-dragon is related to a type\
of dragon associated with one of the following damage\
types (GM’s choice): Acid, Cold, Fire, Lightning, or Poi-\
son. This choice affects other aspects of the stat block.\

### Actions

Multiattack. The half-dragon makes two Claw attacks.\
Claw. Melee Attack Roll: +7, reach 10 ft. Hit: 6 (1d4\
+ 4) Slashing damage plus 7 (2d6) damage of the type\
chosen for the Draconic Origin trait.\
Dragon’s Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 14, each creature in a 30-foot Cone. Failure:\
28 (8d6) damage of the type chosen for the Draconic\
Origin trait. Success: Half damage.\

### Bonus Actions

Leap. The half-dragon jumps up to 30 feet by spending\
10 feet of movement.\

Original layout: [[../../srd-5.2.1/pages/297|page 297]].
