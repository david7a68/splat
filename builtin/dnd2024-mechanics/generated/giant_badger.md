---
splat_schema: 1
splat_id: 3bc35e96-47b1-52f6-8872-2790bffce358
splat_kind: creature
splat_key: dnd2024:creatures/giant_badger
splat_name: Giant Badger
splat_origin:
  key: dnd2024:reference/creature/giant_badger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 349
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 17
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
    value: 13
  max_hp:
    value: 15
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  burrow_speed:
    value: 10
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
        modifier: 3
        scope: attack
      damage:
        expression: 2d4 + 1
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

# Giant Badger

<!-- SRD page 349 -->

Medium Beast, Unaligned\
AC 13  Initiative +0 (10)\
HP 15 (2d8 + 6)\
Speed 30 ft., Burrow 10 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 10 +0 +0 Con 17 +3 +3\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +3\
Resistances Poison\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/349|page 349]].
