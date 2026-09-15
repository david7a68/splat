---
splat_schema: 1
splat_id: d56719fd-ca46-59a2-afb6-162639274fc9
splat_kind: creature
splat_key: dnd2024:creatures/draft_horse
splat_name: Draft Horse
splat_origin:
  key: dnd2024:reference/creature/draft_horse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347, 348
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 15
  save_constitution:
    value: 2
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
    value: 10
  max_hp:
    value: 15
  initiative:
    value: 0
  speed:
    value: 40
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
        modifier: 6
        scope: attack
      damage:
        expression: 1d4 + 4
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

# Draft Horse

<!-- SRD page 347 -->

Large Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 15 (2d10 + 4)\
Speed 40 ft.\

<!-- SRD page 348 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 10 +0 +0 Con 15 +2 +2\
Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
Senses Passive Perception 10\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Hooves. Melee Attack Roll: +6, reach 5 ft. Hit: 6 (1d4\
+ 4) Bludgeoning damage.\

Original layout: [[../../srd-5.2.1/pages/347|page 347]], [[../../srd-5.2.1/pages/348|page 348]].
