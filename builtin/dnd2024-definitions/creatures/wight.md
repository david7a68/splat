---
splat_schema: 1
splat_id: 5e3982d6-f3ff-5853-9522-57c4d3de919a
splat_kind: creature
splat_key: dnd2024:creatures/wight
splat_name: Wight
splat_origin:
  key: dnd2024:reference/creature/wight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 341
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Neutral Evil
  movement_text: 30 ft.
  skills_text: Perception +3, Stealth +4
  resistances_text: Necrotic
  immunities_text: Poison; Exhaustion, Poisoned
  gear_text: Studded Leather Armor
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: Common plus one other language
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 82 (11d8 + 33)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 14
  max_hp:
    value: 82
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/wight
  implementation: dnd2024:creature_wight
  parameters:
    skill_totals:
      perception: 3
      stealth: 4
    damage_resistances:
    - necrotic
    damage_immunities:
    - poison
    condition_immunities:
    - exhaustion
    - poisoned
    gear:
    - item: studded_leather_armor
      count: 1
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 13
    languages:
      speak: true
      known:
      - common
      additional_choice_count: 1
    features:
      sunlight_sensitivity:
        economy: passive
        op: roll.disadvantage
        when:
          in_sunlight: true
        rolls:
        - ability_check
        - attack
      necrotic_sword:
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
                count: 1
                sides: 8
              add: 2
            average: 6
          - type: necrotic
            amount:
              dice:
                count: 1
                sides: 8
              add: 0
            average: 4
      necrotic_bow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 150
          long: 600
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 2
            average: 6
          - type: necrotic
            amount:
              dice:
                count: 1
                sides: 8
              add: 0
            average: 4
      life_drain:
        economy: action
        op: saving_throw
        ability: constitution
        dc: 13
        targets:
          kind: creature
          count: 1
          range_ft: 5
        failure:
        - op: damage
          components:
          - type: necrotic
            amount:
              dice:
                count: 1
                sides: 8
              add: 2
            average: 6
        - op: hp_maximum.reduce
          amount:
            event_field: damage_taken
        success: []
        on_humanoid_slain:
          op: undead.create
          definition:
            key: dnd2024:creatures/zombie
          delay:
            amount: 24
            unit: hour
          unless_any:
          - restored_to_life
          - body_destroyed
          controller: actor
          control_maximum: 12
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 2
          choices:
          - necrotic_sword
          - necrotic_bow
          combination: any
        replace_attack:
          maximum: 1
          choices:
          - op: feature.use
            feature: life_drain
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
- id: dnd2024:creature_wight
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
  - creature_lifecycle
  - creature_rules
  - damage
  - effects
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 878693e38938ea8abf2d8b618899fbb5d67829a58f915e1e1b74a9c9002dba3b
  source_path: builtin/srd-5.2.1/creature/wight.md
  pages:
  - 341
  engine_gaps: &id001
  - dnd2024:creature_wight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 288a3c2fd13b5556b753b95033a6acfc027937f3585a1cd931fced7c15784529
splat_clauses:
- key: source_rules
  text: '# Wight Medium Undead, Neutral Evil AC 14 Initiative +4 (14) HP 82 (11d8 + 33) Speed 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 14 +2 +2 Con 16 +3 +3 Int 10 +0 +0 Wis 13 +1 +1 Cha 15 +2
    +2 Skills Perception +3, Stealth +4 Resistances Necrotic Immunities Poison; Exhaustion, Poisoned Gear
    Studded Leather Armor Senses Darkvision 60 ft.; Passive Perception 13 Languages Common plus one other
    language CR 3 (XP 700; PB +2) ### Traits Sunlight Sensitivity. While in sunlight, the wight has Disadvantage
    on ability checks and attack rolls. ### Actions Multiattack. The wight makes two attacks, using Necrotic
    Sword or Necrotic Bow in any combination. It can replace one attack with a use of Life Drain. Necrotic
    Sword. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage plus 4 (1d8) Necrotic damage.
    Necrotic Bow. Ranged Attack Roll: +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage plus 4 (1d8)
    Necrotic damage. Life Drain. Constitution Saving Throw: DC 13, one creature within 5 feet. Failure:
    6 (1d8 + 2) Necrotic damage, and the target’s Hit Point maximum decreases by an amount equal to the
    damage taken. A Humanoid slain by this attack rises 24 hours later as a Zombie under the wight’s control,
    unless the Humanoid is restored to life or its body is destroyed. The wight can have no more than
    twelve zombies under its control at a time.'
  sha256: dd750ef88c329f55bc3180dea875b8dd3010182752e40a19a6dbfc6a41ad3c4a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wight

<!-- SRD page 341 -->

Medium Undead, Neutral Evil\
AC 14 Initiative +4 (14)\
HP 82 (11d8 + 33)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 16 +3 +3\
Int 10 +0 +0 Wis 13 +1 +1 Cha 15 +2 +2\
Skills Perception +3, Stealth +4\
Resistances Necrotic\
Immunities Poison; Exhaustion, Poisoned\
Gear Studded Leather Armor\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages Common plus one other language\
CR 3 (XP 700; PB +2)\

### Traits

Sunlight Sensitivity. While in sunlight, the wight has\
Disadvantage on ability checks and attack rolls.\

### Actions

Multiattack. The wight makes two attacks, using Ne-\
crotic Sword or Necrotic Bow in any combination. It\
can replace one attack with a use of Life Drain.\
Necrotic Sword. Melee Attack Roll: +4, reach 5 ft.\
Hit: 6 (1d8 + 2) Slashing damage plus 4 (1d8) Ne-\
crotic damage.\
Necrotic Bow. Ranged Attack Roll: +4, range\
150/600 ft. Hit: 6 (1d8 + 2) Piercing damage plus 4\
(1d8) Necrotic damage.\
Life Drain. Constitution Saving Throw: DC 13, one\
creature within 5 feet. Failure: 6 (1d8 + 2) Necrotic\
damage, and the target’s Hit Point maximum decreases\
by an amount equal to the damage taken.\
A Humanoid slain by this attack rises 24 hours later\
as a Zombie under the wight’s control, unless the Hu-\
manoid is restored to life or its body is destroyed. The\
wight can have no more than twelve zombies under its\
control at a time.\

Original layout: [[../../srd-5.2.1/pages/341|page 341]].
