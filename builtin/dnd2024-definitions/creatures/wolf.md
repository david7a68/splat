---
splat_schema: 1
splat_id: bcb4f9e2-334c-5464-9a54-c62ae20ff5ba
splat_kind: creature
splat_key: dnd2024:creatures/wolf
splat_name: Wolf
splat_origin:
  key: dnd2024:reference/creature/wolf
splat_sources:
- title: SRD 5.2.1
  locator: pp. 364
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 40 ft.
  skills_text: Perception +5, Stealth +4
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
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
    value: 11
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/wolf
  implementation: dnd2024:creature_wolf
  parameters:
    skill_totals:
      perception: 5
      stealth: 4
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 15
    languages:
      speak: true
      known: []
    features:
      pack_tactics:
        economy: passive
        op: roll.advantage
        rolls:
        - attack
        target_kind: creature
        when:
          ally_within_target_ft: 5
          ally_excluded_conditions:
          - incapacitated
      bite:
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
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              add: 2
            average: 5
        - op: condition.apply
          condition: prone
          when:
            target_kind: creature
            target_maximum_size: medium
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
- id: dnd2024:creature_wolf
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
  source_sha256: 44172927fa655c0d56415dec18d56f7c428c082aba0b565e3ddba4529a4d8d42
  source_path: builtin/srd-5.2.1/creature/wolf.md
  pages:
  - 364
  engine_gaps: &id001
  - dnd2024:creature_wolf
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4e24aee21eb40be632c587aaf51b71563828350926444b60e66a6e417400d9ca
splat_clauses:
- key: source_rules
  text: '# Wolf Medium Beast, Unaligned AC 12 Initiative +2 (12) HP 11 (2d8 + 2) Speed 40 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 14 +2 +2 Dex 15 +2 +2 Con12 +1 +1 Int 3 -4 -4 Wis 12 +1 +1 Cha 6 -2 -2 Skills
    Perception +5, Stealth +4 Senses Darkvision 60 ft.; Passive Perception 15 Languages None CR 1/4 (XP
    50; PB +2) ### Traits Pack Tactics. The wolf has Advantage on attack rolls against a creature if at
    least one of the wolf’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated
    condition. ### Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage.
    If the target is a Medium or smaller creature, it has the Prone condition.'
  sha256: e83300094739959e5cf0d8abae120dc67eaaf82ac64a9a6d94c9e0722606be2f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wolf

<!-- SRD page 364 -->

Medium Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 11 (2d8 + 2)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  14 +2 +2 Dex 15 +2 +2 Con12 +1 +1\
Int 3 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
Skills Perception +5, Stealth +4\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Pack Tactics. The wolf has Advantage on attack rolls\
against a creature if at least one of the wolf’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Piercing damage. If the target is a Medium or smaller\
creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/364|page 364]].
