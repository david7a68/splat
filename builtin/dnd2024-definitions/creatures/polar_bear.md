---
splat_schema: 1
splat_id: 73aa9e60-ae93-5f97-b620-7ddc21ed2e3b
splat_kind: creature
splat_key: dnd2024:creatures/polar_bear
splat_name: Polar Bear
splat_origin:
  key: dnd2024:reference/creature/polar_bear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 359
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Swim 40 ft.
  skills_text: Perception +5, Stealth +4
  resistances_text: Cold
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 42 (5d10 + 15)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 42
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/polar_bear
  implementation: dnd2024:creature_polar_bear
  parameters:
    skill_totals:
      perception: 5
      stealth: 4
    damage_resistances:
    - cold
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 15
    languages:
      speak: true
      known: []
    features:
      rend:
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
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 8
              add: 5
            average: 9
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
- id: dnd2024:creature_polar_bear
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
  source_sha256: 9fc2f5a126f0de47bb66b04f1bcc3b93961bc4a8219074e1abaee72de7f5b0f3
  source_path: builtin/srd-5.2.1/creature/polar_bear.md
  pages:
  - 359
  engine_gaps: &id001
  - dnd2024:creature_polar_bear
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/polar_bear
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 69b49199a44909cd0d053e751d7d542ba74a2eb2d66beab9075e1a4c87b66521
splat_clauses:
- key: source_rules
  text: '# Polar Bear Large Beast, Unaligned AC 12 Initiative +2 (12) HP 42 (5d10 + 15) Speed 40 ft.,
    Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +5 Dex 14 +2 +2 Con 16 +3 +3 Int 2 -4 -4 Wis 13 +1
    +1 Cha 7 -2 -2 Skills Perception +5, Stealth +4 Resistances Cold Senses Darkvision 60 ft.; Passive
    Perception 15 Languages None CR 2 (XP 450; PB +2) ### Actions Multiattack. The bear makes two Rend
    attacks. Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 9 (1d8 + 5) Slashing damage.'
  sha256: 823c27da8369a0ad10f29b69efce2bfb9c1fe8fa8c7dfef5446a0222512c1cf7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Polar Bear

<!-- SRD page 359 -->

Large Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 42 (5d10 + 15)\
Speed 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 14 +2 +2 Con 16 +3 +3\
Int 2 −4 −4 Wis 13 +1 +1 Cha 7 −2 −2\
Skills Perception +5, Stealth +4\
Resistances Cold\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The bear makes two Rend attacks.\
Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 9 (1d8 + 5)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/359|page 359]].
