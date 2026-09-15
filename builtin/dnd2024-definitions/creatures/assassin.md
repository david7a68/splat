---
splat_schema: 1
splat_id: 71f360e6-f6cd-5406-a452-273caff9f01f
splat_kind: creature
splat_key: dnd2024:creatures/assassin
splat_name: Assassin
splat_origin:
  key: dnd2024:reference/creature/assassin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Acrobatics +7, Perception +6, Stealth +10
  resistances_text: Poison
  gear_text: Light Crossbow, Shortsword, Studded Leather Armor
  senses_text: Passive Perception 16
  languages_text: Common, Thieves’ Cant
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 97 (15d8 + 30)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 18
  save_dexterity:
    value: 7
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 16
  save_intelligence:
    value: 6
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 16
  max_hp:
    value: 97
  initiative:
    value: 10
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/assassin
  implementation: dnd2024:creature_assassin
  parameters:
    skill_totals:
      acrobatics: 7
      perception: 6
      stealth: 10
    damage_resistances:
    - poison
    gear:
    - item: light_crossbow
      count: 1
    - item: shortsword
      count: 1
    - item: studded_leather_armor
      count: 1
    senses: []
    passive_perception: 16
    languages:
      speak: true
      known:
      - common
      - thieves_cant
    features:
      shortsword:
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
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              add: 4
            average: 7
          - type: poison
            amount:
              dice:
                count: 5
                sides: 6
              add: 0
            average: 17
        - op: condition.apply
          condition: poisoned
          duration:
            until: actor.next_turn_start
      light_crossbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 7
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
                sides: 8
              add: 4
            average: 8
          - type: poison
            amount:
              dice:
                count: 6
                sides: 6
              add: 0
            average: 21
      cunning_action:
        economy: bonus_action
        op: action.choose
        choices:
        - dash
        - disengage
        - hide
      evasion:
        economy: passive
        when:
          save_ability: dexterity
          success_damage_fraction: 0.5
          actor_excluded_conditions:
          - incapacitated
        op: damage_fraction.override
        success: 0
        failure: 0.5
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 3
          choices:
          - shortsword
          - light_crossbow
          combination: any
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
- id: dnd2024:creature_assassin
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
  source_sha256: fa6d0c52ee0d108322f44125fa8a3bb55982786709f9a856028b554cd3f436c0
  source_path: builtin/srd-5.2.1/creature/assassin.md
  pages:
  - 260
  engine_gaps: &id001
  - dnd2024:creature_assassin
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: b27ea1866e3d6887dcde19fec300344fccb09842db659d7209b9370de1990c2e
splat_clauses:
- key: source_rules
  text: '# Assassin Medium or Small Humanoid, Neutral AC 16 Initiative +10 (20) HP 97 (15d8 + 30) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 18 +4 +7 Con 14 +2 +2 Int 16 +3 +6 Wis 11 +0 +0
    Cha 10 +0 +0 Skills Acrobatics +7, Perception +6, Stealth +10 Resistances Poison Gear Light Crossbow,
    Shortsword, Studded Leather Armor Senses Passive Perception 16 Languages Common, Thieves’ Cant CR
    8 (XP 3,900; PB +3) ### Traits Evasion. If the assassin is subjected to an effect that allows it to
    make a Dexterity saving throw to take only half damage, the assassin instead takes no damage if it
    succeeds on the save and only half damage if it fails. It can’t use this trait if it has the Incapacitated
    condition. ### Actions Multiattack. The assassin makes three attacks, using Shortsword or Light Crossbow
    in any combination. Shortsword. Melee Attack Roll: +7, reach 5 ft. Hit: 7 (1d6 + 4) Piercing damage
    plus 17 (5d6) Poison damage, and the target has the Poisoned condition until the start of the assassin’s
    next turn. Light Crossbow. Ranged Attack Roll: +7, range 80/320 ft. Hit: 8 (1d8 + 4) Piercing damage
    plus 21 (6d6) Poison damage. ### Bonus Actions Cunning Action. The assassin takes the Dash, Disengage,
    or Hide action.'
  sha256: 6c7df14e38672e42e87fd2f08e2d23b652415519e68a4172337bee945a879b39
  status: contract_specified
splat_requires_mechanics: *id001
---

# Assassin

<!-- SRD page 260 -->

Medium or Small Humanoid, Neutral\
AC 16 Initiative +10 (20)\
HP 97 (15d8 + 30)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 18 +4 +7 Con 14 +2 +2\
Int 16 +3 +6 Wis 11 +0 +0 Cha 10 +0 +0\
Skills Acrobatics +7, Perception +6, Stealth +10\
Resistances Poison\
Gear Light Crossbow, Shortsword, Studded Leather Armor\
Senses Passive Perception 16\
Languages Common, Thieves’ Cant\
CR 8 (XP 3,900; PB +3)\

### Traits

Evasion. If the assassin is subjected to an effect that\
allows it to make a Dexterity saving throw to take only\
half damage, the assassin instead takes no damage if it\
succeeds on the save and only half damage if it fails. It\
can’t use this trait if it has the Incapacitated condition.\

### Actions

Multiattack. The assassin makes three attacks, using\
Shortsword or Light Crossbow in any combination.\
Shortsword. Melee Attack Roll: +7, reach 5 ft. Hit: 7\
(1d6 + 4) Piercing damage plus 17 (5d6) Poison dam-\
age, and the target has the Poisoned condition until the\
start of the assassin’s next turn.\
Light Crossbow. Ranged Attack Roll: +7, range\
80/320 ft. Hit: 8 (1d8 + 4) Piercing damage plus 21\
(6d6) Poison damage.\

### Bonus Actions

Cunning Action. The assassin takes the Dash, Disen-\
gage, or Hide action.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].
