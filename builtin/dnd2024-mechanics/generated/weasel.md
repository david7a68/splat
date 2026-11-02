---
splat_schema: 1
splat_id: 5c49d536-cb5a-59ea-821a-e5df052847eb
splat_kind: creature
splat_key: dnd2024:creatures/weasel
splat_name: Weasel
splat_origin:
  key: dnd2024:reference/creature/weasel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 364
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 16
  save_dexterity:
    value: 3
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
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 13
  max_hp:
    value: 1
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  climb_speed:
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
        modifier: 5
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

# Weasel

<!-- SRD page 364 -->

Tiny Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 1 (1d4 − 1)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 16 +3 +3 Con 8 −1 −1\
Int 2 −4 −4 Wis 12 +1 +1 Cha 3 −4 −4\
Skills Acrobatics +5, Perception +3, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/364|page 364]].
