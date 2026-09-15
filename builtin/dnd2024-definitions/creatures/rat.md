---
splat_schema: 1
splat_id: 1d7d0518-86d7-59cf-8a19-085cea7e7829
splat_kind: creature
splat_key: dnd2024:creatures/rat
splat_name: Rat
splat_origin:
  key: dnd2024:reference/creature/rat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 359
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 20 ft., Climb 20 ft.
  skills_text: Perception +2
  senses_text: Darkvision 30 ft.; Passive Perception 12
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 9
  save_constitution:
    value: -1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 10
  max_hp:
    value: 1
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
  climb_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/rat
  implementation: dnd2024:creature_rat
  parameters:
    skill_totals:
      perception: 2
    senses:
    - sense: darkvision
      range_ft: 30
    passive_perception: 12
    languages:
      speak: true
      known: []
    features:
      agile:
        economy: passive
        op: opportunity_attack.suppress
        trigger: actor.leaves_enemy_reach
        movement_modes:
        - all
      bite:
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
          - type: piercing
            amount:
              constant: 1
            average: 1
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
- id: dnd2024:creature_rat
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
  source_sha256: fe6b20ec190b7b25b285b9a281ae30af523d9f8ed2dc0889657c0618bf9cbce4
  source_path: builtin/srd-5.2.1/creature/rat.md
  pages:
  - 359
  engine_gaps: &id001
  - dnd2024:creature_rat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 550e8a84c07e534cad4f0ee0234508710f5688678b9f5b29e14c6075c617cd6d
splat_clauses:
- key: source_rules
  text: '# Rat Tiny Beast, Unaligned AC 10 Initiative +0 (10) HP 1 (1d4 - 1) Speed 20 ft., Climb 20 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 2 -4 -4 Dex 11 +0 +0 Con 9 -1 -1 Int 2 -4 -4 Wis 10 +0 +0 Cha 4 -3
    -3 Skills Perception +2 Senses Darkvision 30 ft.; Passive Perception 12 Languages None CR 0 (XP 10;
    PB +2) ### Traits Agile. The rat doesn’t provoke an Opportunity Attack when it moves out of an enemy’s
    reach. ### Actions Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.'
  sha256: 548d5933e0df28517412f0ed71dc6b4b2e04869127bf26b9150be9ddc8185dfb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rat

<!-- SRD page 359 -->

Tiny Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 1 (1d4 − 1)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 11 +0 +0 Con 9 −1 −1\
Int 2 −4 −4 Wis 10 +0 +0 Cha  4 −3 −3\
Skills Perception +2\
Senses Darkvision 30 ft.; Passive Perception 12\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Agile. The rat doesn’t provoke an Opportunity Attack\
when it moves out of an enemy’s reach.\

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing\
damage.\

Original layout: [[../../srd-5.2.1/pages/359|page 359]].
