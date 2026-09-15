---
splat_schema: 1
splat_id: e76f4d97-c2ca-599e-a26e-7ded8834ee22
splat_kind: creature
splat_key: dnd2024:creatures/black_bear
splat_name: Black Bear
splat_origin:
  key: dnd2024:reference/creature/black_bear
splat_sources:
- title: SRD 5.2.1
  locator: creature/black_bear
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
splat_stats:
  strength:
    value: 15
  dexterity:
    value: 12
  constitution:
    value: 14
  intelligence:
    value: 2
  wisdom:
    value: 12
  charisma:
    value: 7
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 30
splat_health:
  zero_hp: dnd2024:monster
splat_actions:
  rend:
    name: Rend
    activation: &id001
      economy: action
      category: attack
      timing: on_turn
      cost_at: commit
    targets: &id002
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        allow_self: false
        range:
          feet: 5
    rolls: &id003
      attack:
        expression: 1d20
        modifier: 4
        scope: attack
      damage:
        expression: 1d6 + 2
        scope: attack
    steps: &id004
    - key: attack
      operation: dnd2024:attack
      target: chosen
      kind: melee_weapon
      roll: attack
      on_hit:
        damage:
        - type: slashing
          amount:
            roll: damage
  multiattack:
    name: Multiattack
    activation: *id001
    targets: *id002
    rolls: *id003
    steps:
    - key: multiattack
      operation: splat:repeat
      count: 2
      steps: *id004
---

# Black Bear

Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.
