---
splat_schema: 1
splat_id: d6d1bfcf-d2a7-5ec1-a0c0-f17d5e56116c
splat_kind: creature
splat_key: dnd2024:creatures/riding_horse
splat_name: Riding Horse
splat_origin:
  key: dnd2024:reference/creature/riding_horse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 360
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
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
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 11
  max_hp:
    value: 13
  initiative:
    value: 1
  speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_actions:
  hooves:
    name: Hooves
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
        expression: 1d8 + 3
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

# Riding Horse

<!-- SRD page 360 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 13 (2d10 + 2)\
Speed 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 12 +1 +1\
Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Hooves. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/360|page 360]].
