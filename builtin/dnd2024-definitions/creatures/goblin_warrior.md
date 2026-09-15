---
splat_schema: 1
splat_id: bd94d4ab-8881-5488-a5b9-c44a1b8d2c62
splat_kind: creature
splat_key: dnd2024:creatures/goblin_warrior
splat_name: Goblin Warrior
splat_origin:
  key: dnd2024:reference/creature/goblin_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 290
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: fey
  alignment: Chaotic Neutral
  tags_text: (Goblinoid)
  movement_text: 30 ft.
  skills_text: Stealth +6
  gear_text: Leather Armor, Scimitar, Shield, Shortbow
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: Common, Goblin
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 10 (3d6)
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
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 15
  max_hp:
    value: 10
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/goblin_warrior
  implementation: dnd2024:creature_goblin_warrior
  parameters:
    skill_totals:
      stealth: 6
    gear:
    - item: leather_armor
      count: 1
    - item: scimitar
      count: 1
    - item: shield
      count: 1
    - item: shortbow
      count: 1
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 9
    languages:
      speak: true
      known:
      - common
      - goblin
    features:
      scimitar:
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
                sides: 6
              add: 2
            average: 5
        - op: conditional
          when:
            attack_had_advantage: true
          effects:
          - op: damage
            components:
            - type: slashing
              amount:
                dice:
                  count: 1
                  sides: 4
                add: 0
              average: 2
      shortbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 80
          long: 320
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
        - op: conditional
          when:
            attack_had_advantage: true
          effects:
          - op: damage
            components:
            - type: piercing
              amount:
                dice:
                  count: 1
                  sides: 4
                add: 0
              average: 2
      nimble_escape:
        economy: bonus_action
        op: action.choose
        choices:
        - disengage
        - hide
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
- id: dnd2024:creature_goblin_warrior
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
  source_sha256: 39b0293c12fc3b857acf52dd061165138fb1355cc743fa61245d0e6145121c22
  source_path: builtin/srd-5.2.1/creature/goblin_warrior.md
  pages:
  - 290
  engine_gaps: &id001
  - dnd2024:creature_goblin_warrior
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4b9cb6dbcaca8616f52957a42029996280cd0bbca6038b20c76d4778f300111f
splat_clauses:
- key: source_rules
  text: '# Goblin Warrior Small Fey (Goblinoid), Chaotic Neutral AC 15 Initiative +2 (12) HP 10 (3d6)
    Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 8 -1 -1 Dex 15 +2 +2 Con 10 +0 +0 Int 10 +0 +0 Wis 8 -1
    -1 Cha 8 -1 -1 Skills Stealth +6 Gear Leather Armor, Scimitar, Shield, Shortbow Senses Darkvision
    60 ft.; Passive Perception 9 Languages Common, Goblin CR 1/4 (XP 50; PB +2) ### Actions Scimitar.
    Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage, plus 2 (1d4) Slashing damage
    if the attack roll had Advantage. Shortbow. Ranged Attack Roll: +4, range 80/320 ft. Hit: 5 (1d6 +
    2) Piercing damage, plus 2 (1d4) Piercing damage if the attack roll had Advantage. ### Bonus Actions
    Nimble Escape. The goblin takes the Disengage or Hide action.'
  sha256: b3b0bf61bcde473055ec4340c5040dbebfad865b0c52b1ff8ac424f0e0b9b736
  status: contract_specified
splat_requires_mechanics: *id001
---

# Goblin Warrior

<!-- SRD page 290 -->

Small Fey (Goblinoid), Chaotic Neutral\
AC 15 Initiative +2 (12)\
HP 10 (3d6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 15 +2 +2 Con 10 +0 +0\
Int 10 +0 +0 Wis 8 −1 −1 Cha 8 −1 −1\
Skills Stealth +6\
Gear Leather Armor, Scimitar, Shield, Shortbow\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Common, Goblin\
CR 1/4 (XP 50; PB +2)\

### Actions

Scimitar. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Slashing damage, plus 2 (1d4) Slashing damage if\
the attack roll had Advantage.\
Shortbow. Ranged Attack Roll: +4, range 80/320 ft.\
Hit: 5 (1d6 + 2) Piercing damage, plus 2 (1d4) Piercing\
damage if the attack roll had Advantage.\

### Bonus Actions

Nimble Escape. The goblin takes the Disengage or\
Hide action.\

Original layout: [[../../srd-5.2.1/pages/290|page 290]].
