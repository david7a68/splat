---
splat_schema: 1
splat_id: e5d54ebb-04c3-5399-8918-5d36b7621f4b
splat_kind: creature
splat_key: dnd2024:creatures/camel
splat_name: Camel
splat_origin:
  key: dnd2024:reference/creature/camel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 346
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 5
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 10
  max_hp:
    value: 17
  initiative:
    value: -1
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_actions:
  bite:
    name: Bite
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
          feet: 5
    rolls:
      attack:
        expression: 1d20
        modifier: 4
        scope: attack
      damage:
        expression: 1d4 + 2
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

# Camel

<!-- SRD page 346 -->

Large Beast, Unaligned\
AC 10   Initiative −1 (9)\
HP 17 (2d10 + 6)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 8 −1 −1 Con 17 +3 +5\
Int 2 −4 −4 Wis 11 +0 +0 Cha 5 −3 −3\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2)\
Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/346|page 346]].
