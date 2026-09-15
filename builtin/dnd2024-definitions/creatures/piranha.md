---
splat_schema: 1
splat_id: a7e5a253-7e6a-582d-81f4-70d2a89a9d64
splat_kind: creature
splat_key: dnd2024:creatures/piranha
splat_name: Piranha
splat_origin:
  key: dnd2024:reference/creature/piranha
splat_sources:
- title: SRD 5.2.1
  locator: pp. 358
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Swim 40 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 8
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 1 (1d4 - 1)
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 9
  save_constitution:
    value: -1
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 2
  save_charisma:
    value: -4
  armor_class:
    value: 13
  max_hp:
    value: 1
  initiative:
    value: 3
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/piranha
  implementation: dnd2024:creature_piranha
  parameters:
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 8
    languages:
      speak: true
      known: []
    features:
      water_breathing:
        economy: passive
        op: breathing.allow
        media:
        - water
        exclusive: true
      bite:
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
          - type: piercing
            amount:
              constant: 1
            average: 1
        advantage_when:
          target_hp_below_maximum: true
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
- id: dnd2024:creature_piranha
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
  source_sha256: 8d901dbc850b48c30216c41e32ead67ceb46ef5fbabfb374f918c98e8a678784
  source_path: builtin/srd-5.2.1/creature/piranha.md
  pages:
  - 358
  engine_gaps: &id001
  - dnd2024:creature_piranha
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: c6dd3a6a27ae5a6eec8751d640d7b0dfb4de293f97e02c928b35cbc0be3c9dab
splat_clauses:
- key: source_rules
  text: '# Piranha Tiny Beast, Unaligned AC 13 Initiative +3 (13) HP 1 (1d4 - 1) Speed 5 ft., Swim 40
    ft. MOD SAVE MOD SAVE MOD SAVE Str 2 -4 -4 Dex 16 +3 +3 Con 9 -1 -1 Int 1 -5 -5 Wis 7 -2 -2 Cha 2
    -4 -4 Senses Darkvision 60 ft.; Passive Perception 8 Languages None CR 0 (XP 10; PB +2) ### Traits
    Water Breathing. The piranha can breathe only underwater. ### Actions Bite. Melee Attack Roll: +5
    (with Advantage if the target doesn’t have all its Hit Points), reach 5 ft. Hit: 1 Piercing damage.'
  sha256: 5fb78262628084cd8733bb639779362e267f8c5743c722b951bb74c1b055e2a1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Piranha

<!-- SRD page 358 -->

Tiny Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 1 (1d4 − 1)\
Speed 5 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 16 +3 +3 Con 9 −1 −1\
Int 1 −5 −5 Wis 7 −2 −2 Cha 2 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 8\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Water Breathing. The piranha can breathe only\
underwater.\

### Actions

Bite. Melee Attack Roll: +5 (with Advantage if the target\
doesn’t have all its Hit Points), reach 5 ft. Hit: 1 Pierc-\
ing damage.\

Original layout: [[../../srd-5.2.1/pages/358|page 358]].
