---
splat_schema: 1
splat_id: 5677cfbd-fc93-53ed-bd55-361eacb4d4b0
splat_kind: creature
splat_key: dnd2024:creatures/warrior_veteran
splat_name: Warrior Veteran
splat_origin:
  key: dnd2024:reference/creature/warrior_veteran
splat_sources:
- title: SRD 5.2.1
  locator: pp. 337
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Athletics +5, Perception +2
  gear_text: Greatsword, Heavy Crossbow, Splint Armor
  senses_text: Passive Perception 12
  languages_text: Common plus one other language
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 65 (10d8 + 20)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 17
  max_hp:
    value: 65
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/warrior_veteran
  implementation: dnd2024:creature_warrior_veteran
  parameters:
    skill_totals:
      athletics: 5
      perception: 2
    gear:
    - item: greatsword
      count: 1
    - item: heavy_crossbow
      count: 1
    - item: splint_armor
      count: 1
    senses: []
    passive_perception: 12
    languages:
      speak: true
      known:
      - common
      additional_choice_count: 1
    features:
      greatsword:
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
                count: 2
                sides: 6
              add: 3
            average: 10
      heavy_crossbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 3
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 100
          long: 400
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 10
              add: 1
            average: 12
      parry:
        economy: reaction
        trigger: melee_attack.hit_actor
        requires:
          holding_weapon: true
        op: ac.add
        amount: 2
        scope: triggering_attack
        recheck_hit: true
      multiattack:
        economy: action
        op: multiattack
        choose_sequence:
        - feature: greatsword
          count: 2
        - feature: heavy_crossbow
          count: 2
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
- id: dnd2024:creature_warrior_veteran
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
  - melee_attack.hit_actor
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
  source_sha256: 514cfbd7fb4c45c2be9d59ff5b867272ec837aa2f405b47965277ee71f8ddb1b
  source_path: builtin/srd-5.2.1/creature/warrior_veteran.md
  pages:
  - 337
  engine_gaps: &id001
  - dnd2024:creature_warrior_veteran
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: feca0e74e99f1ac73db93c45d3b6deb379589d888624d2efd0a175ce45fd272e
splat_clauses:
- key: source_rules
  text: '# Warrior Veteran Medium or Small Humanoid, Neutral AC 17 Initiative +3 (13) HP 65 (10d8 + 20)
    Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 13 +1 +1 Con 14 +2 +2 Int 10 +0 +0 Wis 11
    +0 +0 Cha 10 +0 +0 Skills Athletics +5, Perception +2 Gear Greatsword, Heavy Crossbow, Splint Armor
    Senses Passive Perception 12 Languages Common plus one other language CR 3 (XP 700; PB +2) ### Actions
    Multiattack. The warrior makes two Greatsword or Heavy Crossbow attacks. Greatsword. Melee Attack
    Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage. Heavy Crossbow. Ranged Attack Roll: +3, range
    100/400 ft. Hit: 12 (2d10 + 1) Piercing damage. ### Reactions Parry. Trigger: The warrior is hit by
    a melee attack roll while holding a weapon. Response: The warrior adds 2 to its AC against that attack,
    possibly causing it to miss.'
  sha256: 409902e6d2138db9035db96f970e2c2ec8269e98b54ffd0cd5f8097f9d156f32
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warrior Veteran

<!-- SRD page 337 -->

Medium or Small Humanoid, Neutral\
AC 17 Initiative +3 (13)\
HP 65 (10d8 + 20)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 14 +2 +2\
Int 10 +0 +0 Wis 11 +0 +0 Cha 10 +0 +0\
Skills Athletics +5, Perception +2\
Gear Greatsword, Heavy Crossbow, Splint Armor\
Senses Passive Perception 12\
Languages Common plus one other language\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The warrior makes two Greatsword or\
Heavy Crossbow attacks.\
Greatsword. Melee Attack Roll: +5, reach 5 ft. Hit: 10\
(2d6 + 3) Slashing damage.\
Heavy Crossbow. Ranged Attack Roll: +3, range\
100/400 ft. Hit: 12 (2d10 + 1) Piercing damage.\

### Reactions

Parry. Trigger: The warrior is hit by a melee attack roll\
while holding a weapon. Response: The warrior adds 2\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/337|page 337]].
