---
splat_schema: 1
splat_id: 91576b90-7b8c-5204-a710-e0f7a3d88545
splat_kind: creature
splat_key: dnd2024:creatures/bat
splat_name: Bat
splat_origin:
  key: dnd2024:reference/creature/bat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
splat_stats:
  strength:
    value: 2
  save_strength:
    value: -4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 8
  save_constitution:
    value: -1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 4
  save_charisma:
    value: -3
  armor_class:
    value: 12
  max_hp:
    value: 1
  initiative:
    value: 2
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 30
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
        expression: '1'
        scope: attack
    steps:
    - key: attack
      operation: dnd2024:attack
      kind: melee_weapon
      target: chosen
      roll: attack
      on_hit:
        damage:
        - type: piercing
          amount:
            roll: damage
---

# Bat

<!-- SRD page 345 -->

Tiny Beast, Unaligned\
AC 12 Initiative +2 (12)\
HP 1 (1d4 − 1)\
Speed 5 ft., Fly 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 2 −4 −4 Dex 15 +2 +2 Con 8 −1 −1\
Int 2 −4 −4 Wis 12 +1 +1 Cha 4 −3 −3\
Senses Blindsight 60 ft.; Passive Perception 11\
Languages None\
CR 0 (XP 10; PB +2)\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing\
damage.\

Original layout: [[../../srd-5.2.1/pages/345|page 345]].
