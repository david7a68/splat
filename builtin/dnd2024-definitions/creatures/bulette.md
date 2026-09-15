---
splat_schema: 1
splat_id: 463e7b55-64bc-5b8a-80ee-af2711e4d4b0
splat_kind: creature
splat_key: dnd2024:creatures/bulette
splat_name: Bulette
splat_origin:
  key: dnd2024:reference/creature/bulette
splat_sources:
- title: SRD 5.2.1
  locator: pp. 272
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft., Burrow 40 ft.
  skills_text: Perception +6
  senses_text: Darkvision 60 ft., Tremorsense 120 ft.; Passive Perception 16
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 94 (9d10 + 45)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 2
  save_intelligence:
    value: -4
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
    value: 94
  initiative:
    value: 0
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/bulette
  implementation: dnd2024:creature_bulette
  parameters:
    skill_totals:
      perception: 6
    senses:
    - sense: darkvision
      range_ft: 60
    - sense: tremorsense
      range_ft: 120
    passive_perception: 16
    languages:
      speak: true
      known: []
    features:
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 7
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 12
              add: 4
            average: 17
      leap:
        economy: bonus_action
        op: movement.jump
        maximum_ft: 30
        movement_cost_ft: 10
      deadly_leap:
        economy: action
        op: sequence
        steps:
        - op: movement.jump
          movement_cost_ft: 5
          maximum_ft: 15
          destination_contains:
            minimum_creatures: 1
            maximum_size: large
        - op: saving_throw
          ability: dexterity
          dc: 15
          targets:
            kind: creature
            count: all
            location: actor_destination_space
          failure:
          - op: damage
            components:
            - type: bludgeoning
              amount:
                dice:
                  count: 3
                  sides: 12
                add: 0
              average: 19
          - op: condition.apply
            condition: prone
          success:
          - op: damage
            from: failure_damage
            multiplier: 0.5
            round: down
          - op: push
            distance_ft: 5
            direction: straight_away_from_actor
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: bite
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
- id: dnd2024:creature_bulette
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
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 767ae691b73bb03a642ee4d528e54bd686acfb999da712f2f7506b2d1f038de2
  source_path: builtin/srd-5.2.1/creature/bulette.md
  pages:
  - 272
  engine_gaps: &id001
  - dnd2024:creature_bulette
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 294462a64053d40c234c10630540db8d2a5eb23bc347a959ed339e799ff0d2be
splat_clauses:
- key: source_rules
  text: '# Bulette Large Monstrosity, Unaligned AC 17 Initiative +0 (10) HP 94 (9d10 + 45) Speed 40 ft.,
    Burrow 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 11 +0 +0 Con 21 +5 +5 Int 2 -4 -4 Wis 10
    +0 +0 Cha 5 -3 -3 Skills Perception +6 Senses Darkvision 60 ft., Tremorsense 120 ft.; Passive Perception
    16 Languages None CR 5 (XP 1,800; PB +3) ### Actions Multiattack. The bulette makes two Bite attacks.
    Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 17 (2d12 + 4) Piercing damage. Deadly Leap. The bulette
    spends 5 feet of movement to jump to a space within 15 feet that contains one or more Large or smaller
    creatures. Dexterity Saving Throw: DC 15, each creature in the bulette’s destination space. Failure:
    19 (3d12) Bludgeoning damage, and the target has the Prone condition. Success: Half damage, and the
    target is pushed 5 feet straight away from the bulette. ### Bonus Actions Leap. The bulette jumps
    up to 30 feet by spending 10 feet of movement.'
  sha256: a483513ec9d29eda02a099058220c07c2aa4add18e073b13344064e27ec529c5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bulette

<!-- SRD page 272 -->

Large Monstrosity, Unaligned\
AC 17 Initiative +0 (10)\
HP 94 (9d10 + 45)\
Speed 40 ft., Burrow 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 11 +0 +0 Con 21 +5 +5\
Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
Skills Perception +6\
Senses Darkvision 60 ft., Tremorsense 120 ft.;\
Passive Perception 16\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The bulette makes two Bite attacks.\
Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 17 (2d12\
+ 4) Piercing damage.\
Deadly Leap. The bulette spends 5 feet of movement\
to jump to a space within 15 feet that contains one\
or more Large or smaller creatures. Dexterity Saving\
Throw: DC 15, each creature in the bulette’s destina-\
tion space. Failure: 19 (3d12) Bludgeoning damage, and\
the target has the Prone condition. Success: Half dam-\
age, and the target is pushed 5 feet straight away from\
the bulette.\

### Bonus Actions

Leap. The bulette jumps up to 30 feet by spending 10\
feet of movement.\

Original layout: [[../../srd-5.2.1/pages/272|page 272]].
