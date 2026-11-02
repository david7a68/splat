---
splat_schema: 1
splat_id: 5a484d75-2077-5cd5-a2b5-e9a89e725e0e
splat_kind: creature
splat_key: dnd2024:creatures/hawk
splat_name: Hawk
splat_origin:
  key: dnd2024:reference/creature/hawk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 355
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
splat_stats:
  strength:
    value: 5
  save_strength:
    value: -3
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
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 1
  initiative:
    value: 3
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_actions:
  talons:
    name: Talons
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
        - type: slashing
          amount:
            roll: damage
---

# Hawk

<!-- SRD page 355 -->

Tiny Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 1 (1d4 − 1)\
Speed 10 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 5 −3 −3 Dex 16 +3 +3 Con 8 −1 −1\
Int 2 −4 −4 Wis 14 +2 +2 Cha 6 −2 −2\
Skills Perception +6\
Senses Passive Perception 16\
Languages None\
CR 0 (XP 10; PB +2)

### Actions

Talons. Melee Attack Roll: +5, reach 5 ft. Hit: 1 Slashing damage.

Original layout: [[../../srd-5.2.1/pages/355|page 355]].
