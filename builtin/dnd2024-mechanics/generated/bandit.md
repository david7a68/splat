---
splat_schema: 1
splat_id: 703a8a36-2e37-5b7f-bc16-0ae31fb12cc7
splat_kind: creature
splat_key: dnd2024:creatures/bandit
splat_name: Bandit
splat_origin:
  key: dnd2024:reference/creature/bandit
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 12
  max_hp:
    value: 11
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_actions:
  scimitar:
    name: Scimitar
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
        expression: 1d6 + 1
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
  light_crossbow:
    name: Light Crossbow
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
          feet: 80
          long: 320
    rolls:
      attack:
        expression: 1d20
        modifier: 3
        scope: attack
      damage:
        expression: 1d8 + 1
        scope: attack
    steps:
    - key: attack
      operation: dnd2024:attack
      kind: ranged_weapon
      target: chosen
      roll: attack
      on_hit:
        damage:
        - type: piercing
          amount:
            roll: damage
---

# Bandit

<!-- SRD page 261 -->

Medium or Small Humanoid, Neutral\
AC 12 Initiative +1 (11)\
HP 11 (2d8 + 2)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 12 +1 +1 Con 12 +1 +1\
Int 10 +0 +0 Wis 10 +0 +0 Cha 10 +0 +0\
Gear Leather Armor, Light Crossbow, Scimitar\
Senses Passive Perception 10\
Languages Common, Thieves’ Cant\
CR 1/8 (XP 25; PB +2)\

### Actions

Scimitar. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6\
+ 1) Slashing damage.\
Light Crossbow. Ranged Attack Roll: +3, range 80/320\
ft. Hit: 5 (1d8 + 1) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]].
