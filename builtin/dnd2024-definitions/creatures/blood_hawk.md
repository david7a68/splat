---
splat_schema: 1
splat_id: 9e0eea97-c939-58d8-b586-c85d883f58eb
splat_kind: creature
splat_key: dnd2024:creatures/blood_hawk
splat_name: Blood Hawk
splat_origin:
  key: dnd2024:reference/creature/blood_hawk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345, 346
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: beast
  alignment: Unaligned
  movement_text: 10 ft., Fly 60 ft.
  skills_text: Perception +6
  senses_text: Passive Perception 16
  languages_text: None
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 7 (2d6)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 7
  initiative:
    value: 2
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/blood_hawk
  implementation: dnd2024:creature_blood_hawk
  parameters:
    skill_totals:
      perception: 6
    senses: []
    passive_perception: 16
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
      beak:
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
                sides: 4
              add: 2
            average: 4
          replacement_when:
            if:
              target_bloodied: true
            components:
            - type: piercing
              amount:
                dice:
                  count: 1
                  sides: 8
                add: 2
              average: 6
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
- id: dnd2024:creature_blood_hawk
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
  source_sha256: f6172bd050736b3723783f98c1d78c1aa089f7ffbe9c440ed5cca9fbf91fb16c
  source_path: builtin/srd-5.2.1/creature/blood_hawk.md
  pages:
  - 345
  - 346
  engine_gaps: &id001
  - dnd2024:creature_blood_hawk
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 47153d211d0e4bd9254438956f97ba7857b0b2e1170bb934cbdbdde9e4824a3d
splat_clauses:
- key: source_rules
  text: '# Blood Hawk Small Beast, Unaligned AC 12 Initiative +2 (12) HP 7 (2d6) Speed 10 ft., Fly 60
    ft. MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 14 +2 +2 Con 10 +0 +0 Int 3 -4 -4 Wis 14 +2 +2 Cha
    5 -3 -3 Skills Perception +6 Senses Passive Perception 16 Languages None CR 1/8 (XP 25; PB +2) ###
    Traits Pack Tactics. The hawk has Advantage on an attack roll against a creature if at least one of
    the hawk’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.
    ### Actions Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage, or 6 (1d8 +
    2) Piercing damage if the target is Bloodied.'
  sha256: d514509cf50966f79701058689976940846d3159e4f0ccfddc7601a408c1a199
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blood Hawk

<!-- SRD page 345 -->

Small Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 7 (2d6)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 14 +2 +2 Con 10 +0 +0\
Int 3 −4 −4 Wis 14 +2 +2 Cha 5 −3 −3\
Skills Perception +6\
Senses Passive Perception 16\
Languages None\
CR 1/8 (XP 25; PB +2)\

<!-- SRD page 346 -->


### Traits

Pack Tactics. The hawk has Advantage on an attack roll\
against a creature if at least one of the hawk’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Beak. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2)\
Piercing damage, or 6 (1d8 + 2) Piercing damage if the\
target is Bloodied.\

Original layout: [[../../srd-5.2.1/pages/345|page 345]], [[../../srd-5.2.1/pages/346|page 346]].
