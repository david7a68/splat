---
splat_schema: 1
splat_id: 72cc0187-32ce-54cd-b408-09dc5fb4205c
splat_kind: creature
splat_key: dnd2024:creatures/owlbear
splat_name: Owlbear
splat_origin:
  key: dnd2024:reference/creature/owlbear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 59
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_actions:
  rend:
    name: Rend
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
        modifier: 7
        scope: attack
      damage:
        expression: 2d8 + 5
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
  multiattack:
    name: Multiattack
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
        modifier: 7
        scope: attack
      damage:
        expression: 2d8 + 5
        scope: attack
    steps:
    - key: multiattack
      operation: splat:repeat
      count: 2
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

# Owlbear

<!-- SRD page 313 -->

Large Monstrosity, Unaligned\
AC 13 Initiative +1 (11)\
HP 59 (7d10 + 21)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 12 +1 +1 Con 17 +3 +3\
Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 3 (XP 700; PB +2)

### Actions

Multiattack. The owlbear makes two Rend attacks.

Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/313|page 313]].
