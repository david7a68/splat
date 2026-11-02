---
splat_schema: 1
splat_id: 883a37d4-0528-5c57-a94c-cf5f8e56eb9d
splat_kind: creature
splat_key: dnd2024:creatures/badger
splat_name: Badger
splat_origin:
  key: dnd2024:reference/creature/badger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 345
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 11
  max_hp:
    value: 5
  initiative:
    value: 0
  speed:
    value: 20
    unit: ft
  burrow_speed:
    value: 5
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_traits:
  damage_affinities:
    rule: dnd2024:damage_affinities
    parameters:
      resistances:
      - poison
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
        modifier: 2
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

# Badger

<!-- SRD page 345 -->

Tiny Beast, Unaligned\
AC 11  Initiative +0 (10)\
HP 5 (1d4 + 3)\
Speed 20 ft., Burrow 5 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 11 +0 +0 Con 16 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +3\
Resistances Poison\
Senses Darkvision 30 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/345|page 345]].
