---
splat_schema: 1
splat_id: 7be357ec-e71b-54e7-974a-3d8acad27110
splat_kind: creature
splat_key: dnd2024:creatures/animated_flying_sword
splat_name: Animated Flying Sword
splat_origin:
  key: dnd2024:reference/creature/animated_flying_sword
splat_sources:
- title: SRD 5.2.1
  locator: pp. 259
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: construct
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 15
  save_dexterity:
    value: 4
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 5
  save_wisdom:
    value: -3
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 17
  max_hp:
    value: 14
  initiative:
    value: 4
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_traits:
  condition_immunities:
    rule: dnd2024:condition_immunities
    parameters:
      conditions:
      - charmed
      - deafened
      - exhaustion
      - frightened
      - paralyzed
      - petrified
      - poisoned
  damage_affinities:
    rule: dnd2024:damage_affinities
    parameters:
      immunities:
      - poison
      - psychic
splat_actions:
  slash:
    name: Slash
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
        modifier: 4
        scope: attack
      damage:
        expression: 1d8 + 2
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

# Animated Flying Sword

<!-- SRD page 259 -->

Small Construct, Unaligned\
AC 17 Initiative +4 (14)\
HP 14 (4d6)\
Speed 5 ft., Fly 50 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 15 +2 +4 Con 11 +0 +0\
Int 1 −5 −5 Wis 5 −3 −3 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Deafened,\
Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
Senses Blindsight 60 ft.; Passive Perception 7\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Slash. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Slashing damage.\

Original layout: [[../../srd-5.2.1/pages/259|page 259]].
