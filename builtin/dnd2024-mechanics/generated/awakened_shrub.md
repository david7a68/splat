---
splat_schema: 1
splat_id: 5afec75c-7981-5ce1-b7f7-3469de75b409
splat_kind: creature
splat_key: dnd2024:creatures/awakened_shrub
splat_name: Awakened Shrub
splat_origin:
  key: dnd2024:reference/creature/awakened_shrub
splat_sources:
- title: SRD 5.2.1
  locator: pp. 260
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: plant
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 9
  max_hp:
    value: 10
  initiative:
    value: -1
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
      - piercing
      vulnerabilities:
      - fire
splat_actions:
  rake:
    name: Rake
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
        modifier: 1
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

# Awakened Shrub

<!-- SRD page 260 -->

Small Plant, Neutral\
AC 9 Initiative −1 (9)\
HP 10 (3d6)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 8 −1 −1 Con 11 +0 +0\
Int 10 +0 +0 Wis 10 +0 +0 Cha 6 −2 −2\
Vulnerabilities Fire\
Resistances Piercing\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 0 (XP 10; PB +2)\

### Actions

Rake. Melee Attack Roll: +1, reach 5 ft. Hit: 1 Slash-\
ing damage.\

Original layout: [[../../srd-5.2.1/pages/260|page 260]].
