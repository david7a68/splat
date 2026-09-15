---
splat_schema: 1
splat_id: 4ad4d8a0-ac33-5953-a26d-9f48e8e902f7
splat_kind: creature
splat_key: dnd2024:creatures/guard
splat_name: Guard
splat_origin:
  key: dnd2024:reference/creature/guard
splat_sources:
- title: SRD 5.2.1
  locator: creature/guard
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
splat_stats:
  strength:
    value: 13
  dexterity:
    value: 12
  constitution:
    value: 12
  intelligence:
    value: 10
  wisdom:
    value: 11
  charisma:
    value: 10
  armor_class:
    value: 16
  max_hp:
    value: 11
  initiative:
    value: 1
  speed:
    value: 30
splat_health:
  zero_hp: dnd2024:monster
splat_actions:
  spear:
    name: Spear (melee)
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
      target: chosen
      kind: melee_weapon
      roll: attack
      on_hit:
        damage:
        - type: piercing
          amount:
            roll: damage
  throw_spear:
    name: Spear (ranged)
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
          feet: 20
          long: 60
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
      target: chosen
      kind: ranged_weapon
      roll: attack
      on_hit:
        damage:
        - type: piercing
          amount:
            roll: damage
---

# Guard

Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.
