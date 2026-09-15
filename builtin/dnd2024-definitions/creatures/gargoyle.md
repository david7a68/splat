---
splat_schema: 1
splat_id: 9607ffda-788a-5ade-8b84-7afa39d5dfa5
splat_kind: creature
splat_key: dnd2024:creatures/gargoyle
splat_name: Gargoyle
splat_origin:
  key: dnd2024:reference/creature/gargoyle
splat_sources:
- title: SRD 5.2.1
  locator: pp. 286
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: elemental
  alignment: Chaotic Evil
  movement_text: 30 ft., Fly 60 ft.
  skills_text: Stealth +4
  immunities_text: Poison; Exhaustion, Petrified, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Terran)
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 67 (9d8 + 27)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 67
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/gargoyle
  implementation: dnd2024:creature_gargoyle
  parameters:
    skill_totals:
      stealth: 4
    damage_immunities:
    - poison
    condition_immunities:
    - exhaustion
    - petrified
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
      - terran
    features:
      flyby:
        economy: passive
        op: opportunity_attack.suppress
        trigger: actor.leaves_enemy_reach
        movement_modes:
        - fly
      claw:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 4
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
                count: 2
                sides: 4
              add: 2
            average: 7
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
- id: dnd2024:creature_gargoyle
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
  - actor.leaves_enemy_reach
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: ca927f138a7f71fe62d5eadb39975134c6b62b70687e915fcc59fc494eb65f05
  source_path: builtin/srd-5.2.1/creature/gargoyle.md
  pages:
  - 286
  engine_gaps: &id001
  - dnd2024:creature_gargoyle
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: d9ea4a19b73219047042b702b2b1e8fc76f8d8685f5f35877ff6e9102098d908
splat_clauses:
- key: source_rules
  text: '# Gargoyle Medium Elemental, Chaotic Evil AC 15 Initiative +2 (12) HP 67 (9d8 + 27) Speed 30
    ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 11 +0 +0 Con 16 +3 +3 Int 6 -2 -2 Wis
    11 +0 +0 Cha 7 -2 -2 Skills Stealth +4 Immunities Poison; Exhaustion, Petrified, Poisoned Senses Darkvision
    60 ft.; Passive Perception 10 Languages Primordial (Terran) CR 2 (XP 450; PB +2) ### Traits Flyby.
    The gargoyle doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach. ### Actions
    Multiattack. The gargoyle makes two Claw attacks. Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 7
    (2d4 + 2) Slashing damage.'
  sha256: 30db66ad71565e4dc4b432b01243b69e0168fb1fab1103ad1a93fe6586cbf476
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gargoyle

<!-- SRD page 286 -->

Medium Elemental, Chaotic Evil\
AC 15 Initiative +2 (12)\
HP 67 (9d8 + 27)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 11 +0 +0 Con 16 +3 +3\
Int 6 −2 −2 Wis 11 +0 +0 Cha 7 −2 −2\
Skills Stealth +4\
Immunities Poison; Exhaustion, Petrified, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Terran)\
CR 2 (XP 450; PB +2)\

### Traits

Flyby. The gargoyle doesn’t provoke an Opportunity\
Attack when it flies out of an enemy’s reach.\

### Actions

Multiattack. The gargoyle makes two Claw attacks.\
Claw. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (2d4 + 2)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/286|page 286]].
