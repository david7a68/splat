---
splat_schema: 1
splat_id: eb0e0267-2c45-5fe5-b03e-29f5af103f9c
splat_kind: creature
splat_key: dnd2024:creatures/violet_fungus
splat_name: Violet Fungus
splat_origin:
  key: dnd2024:reference/creature/violet_fungus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 286
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: plant
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 1
  save_dexterity:
    value: -5
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 3
  save_wisdom:
    value: -4
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 5
  max_hp:
    value: 18
  initiative:
    value: -5
  speed:
    value: 5
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_traits:
  condition_immunities:
    rule: dnd2024:condition_immunities
    parameters:
      conditions:
      - blinded
      - charmed
      - deafened
      - frightened
splat_actions:
  rotting_touch:
    name: Rotting Touch
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
          feet: 10
    rolls:
      attack:
        expression: 1d20
        modifier: 2
        scope: attack
      damage:
        expression: 1d8
        scope: attack
    steps:
    - key: attack
      operation: dnd2024:attack
      kind: melee_weapon
      target: chosen
      roll: attack
      on_hit:
        damage:
        - type: necrotic
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
          feet: 10
    rolls:
      attack:
        expression: 1d20
        modifier: 2
        scope: attack
      damage:
        expression: 1d8
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
          - type: necrotic
            amount:
              roll: damage
---

# Violet Fungus

<!-- SRD page 286 -->

Medium Plant, Unaligned\
AC 5 Initiative −5 (5)\
HP 18 (4d8)\
Speed 5 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 1 −5 −5 Con 10 +0 +0\
Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Blinded, Charmed, Deafened, Frightened\
Senses Blindsight 30 ft.; Passive Perception 6\
Languages None\
CR 1/4 (XP 50; PB +2)

### Actions

Multiattack. The fungus makes two Rotting Touch attacks.

Rotting Touch. Melee Attack Roll: +2, reach 10 ft. Hit: 4 (1d8) Necrotic damage.

Original layout: [[../../srd-5.2.1/pages/286|page 286]].
