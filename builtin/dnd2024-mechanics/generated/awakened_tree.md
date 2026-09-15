---
splat_schema: 1
splat_id: 8a76ac78-98c3-5ce4-b9d8-7bf1312ce86f
splat_kind: creature
splat_key: dnd2024:creatures/awakened_tree
splat_name: Awakened Tree
splat_origin:
  key: dnd2024:reference/creature/awakened_tree
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: plant
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 6
  save_dexterity:
    value: -2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 59
  initiative:
    value: -2
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_traits:
  damage_affinities:
    rule: dnd2024:damage_affinities
    parameters:
      resistances:
      - bludgeoning
      - piercing
      vulnerabilities:
      - fire
splat_actions:
  slam:
    name: Slam
    activation:
      economy: action
      category: attack
      timing: on_turn
      cost_at: commit
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        allow_self: false
        range:
          feet: 10
    rolls:
      attack:
        expression: 1d20
        modifier: 6
        scope: attack
      damage:
        expression: 3d6 + 4
        scope: attack
    steps:
    - key: attack
      operation: dnd2024:attack
      kind: melee_weapon
      target: chosen
      roll: attack
      on_hit:
        damage:
        - type: bludgeoning
          amount:
            roll: damage
---

# Awakened Tree

<!-- SRD page 260 -->

Huge Plant, Neutral\
AC 13 Initiative −2 (8)\
HP 59 (7d12 + 14)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 6 −2 −2 Con 15 +2 +2\
Int 10 +0 +0 Wis 10 +0 +0 Cha 7 −2 −2\
Vulnerabilities Fire\
Resistances Bludgeoning, Piercing\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 2 (XP 450; PB +2)\

### Actions

Slam. Melee Attack Roll: +6, reach 10 ft. Hit: 14 (3d6\
+ 4) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].
