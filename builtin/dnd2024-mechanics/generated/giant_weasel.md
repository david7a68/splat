---
splat_schema: 1
splat_id: 419b96b5-267e-51fa-83d8-2337a760cc3b
splat_kind: creature
splat_key: dnd2024:creatures/giant_weasel
splat_name: Giant Weasel
splat_origin:
  key: dnd2024:reference/creature/giant_weasel
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 4
  save_intelligence:
    value: -3
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 13
  max_hp:
    value: 9
  initiative:
    value: 3
  speed:
    value: 40
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
        expression: 1d4 + 3
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

# Giant Weasel

<!-- SRD page 355 -->

Medium Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 9 (2d8)\
Speed 40 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 17 +3 +3 Con 10 +0 +0\
Int 4 −3 −3 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Acrobatics +5, Perception +3, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1/8 (XP 25; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/355|page 355]].
