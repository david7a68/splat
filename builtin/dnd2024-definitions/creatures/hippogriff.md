---
splat_schema: 1
splat_id: 37ed5db6-ae78-5ea1-a23b-84b36c7808fb
splat_kind: creature
splat_key: dnd2024:creatures/hippogriff
splat_name: Hippogriff
splat_origin:
  key: dnd2024:reference/creature/hippogriff
splat_sources:
- title: SRD 5.2.1
  locator: pp. 298
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft., Fly 60 ft.
  skills_text: Perception +5
  senses_text: Passive Perception 15
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 26 (4d10 + 4)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 11
  max_hp:
    value: 26
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hippogriff
  implementation: dnd2024:creature_hippogriff
  parameters:
    skill_totals:
      perception: 5
    senses: []
    passive_perception: 15
    languages:
      speak: true
      known: []
    features:
      flyby:
        economy: passive
        op: opportunity_attack.suppress
        trigger: actor.leaves_enemy_reach
        movement_modes:
        - fly
      rend:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 5
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
                sides: 8
              add: 3
            average: 7
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: rend
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
- id: dnd2024:creature_hippogriff
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
  source_sha256: ddd87ecedc1e07ef3c7672dfcc80ece636e28515a68f29ddfc79f869b9b830ff
  source_path: builtin/srd-5.2.1/creature/hippogriff.md
  pages:
  - 298
  engine_gaps: &id001
  - dnd2024:creature_hippogriff
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: f67364af6e48c2320edb38158015bcc97f6cd39023da2de6c459e5a4ee6d9c17
splat_clauses:
- key: source_rules
  text: '# Hippogriff Large Monstrosity, Unaligned AC 11 Initiative +1 (11) HP 26 (4d10 + 4) Speed 40
    ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 13 +1 +1 Con 13 +1 +1 Int 2 -4 -4 Wis
    12 +1 +1 Cha 8 -1 -1 Skills Perception +5 Senses Passive Perception 15 Languages None CR 1 (XP 200;
    PB +2) ### Traits Flyby. The hippogriff doesn’t provoke an Opportunity Attack when it flies out of
    an enemy’s reach. ### Actions Multiattack. The hippogriff makes two Rend attacks. Rend. Melee Attack
    Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage.'
  sha256: 217ed943cb9f85d10dc3c4a03574ee23f15b2bf56caa81555ed5445f0a323de1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hippogriff

<!-- SRD page 298 -->

Large Monstrosity, Unaligned\
AC 11 Initiative +1 (11)\
HP 26 (4d10 + 4)\
Speed 40 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +5\
Senses Passive Perception 15\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Flyby. The hippogriff doesn’t provoke an Opportunity\
Attack when it flies out of an enemy’s reach.\

### Actions

Multiattack. The hippogriff makes two Rend attacks.\
Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/298|page 298]].
