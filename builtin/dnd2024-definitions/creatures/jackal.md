---
splat_schema: 1
splat_id: 3bfdbb1f-f2d2-523b-94c2-aa6f9d8362d4
splat_kind: creature
splat_key: dnd2024:creatures/jackal
splat_name: Jackal
splat_origin:
  key: dnd2024:reference/creature/jackal
splat_sources:
- title: SRD 5.2.1
  locator: pp. 356
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  skills_text: Perception +5, Stealth +4
  senses_text: Darkvision 90 ft.; Passive Perception 15
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 3 (1d6)
splat_stats:
  strength:
    value: 8
  save_strength:
    value: -1
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 3
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/jackal
  implementation: dnd2024:creature_jackal
  parameters:
    skill_totals:
      perception: 5
      stealth: 4
    senses:
    - sense: darkvision
      range_ft: 90
    passive_perception: 15
    languages:
      speak: true
      known: []
    features:
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 1
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
                count: 1
                sides: 4
              add: -1
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
- id: dnd2024:creature_jackal
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
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: ab2af266d4de9719608131391500c944e9665c078cf6be88709fd24184895595
  source_path: builtin/srd-5.2.1/creature/jackal.md
  pages:
  - 356
  engine_gaps: &id001
  - dnd2024:creature_jackal
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: b11555068bbd3e51ff6dcce0302692be89d6ee151283d1b2e050d48a540a46a9
splat_clauses:
- key: source_rules
  text: '# Jackal Small Beast, Unaligned AC 12 Initiative +2 (12) HP 3 (1d6) Speed 40 ft. MOD SAVE MOD
    SAVE MOD SAVE Str 8 -1 -1 Dex 15 +2 +2 Con 11 +0 +0 Int 3 -4 -4 Wis 12 +1 +1 Cha 6 -2 -2 Skills Perception
    +5, Stealth +4 Senses Darkvision 90 ft.; Passive Perception 15 Languages None CR 0 (XP 10; PB +2)
    ### Actions Bite. Melee Attack Roll: +1, reach 5 ft. Hit: 1 (1d4 – 1) Piercing damage.'
  sha256: 80d7eb179b2f03fbc4076b261d05d63052fd9a3044a59d1ec2e2980b2c1b3f67
  status: contract_specified
splat_requires_mechanics: *id001
---

# Jackal

<!-- SRD page 356 -->

Small Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 3 (1d6)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 15 +2 +2 Con 11 +0 +0\
Int 3 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Skills Perception +5, Stealth +4\
Senses Darkvision 90 ft.; Passive Perception 15\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Bite. Melee Attack Roll: +1, reach 5 ft. Hit: 1 (1d4 – 1)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/356|page 356]].
