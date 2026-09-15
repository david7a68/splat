---
splat_schema: 1
splat_id: 717bc686-4fb8-5665-b30a-c6a2b4a22fa4
splat_kind: creature
splat_key: dnd2024:creatures/skeleton
splat_name: Skeleton
splat_origin:
  key: dnd2024:reference/creature/skeleton
splat_sources:
- title: SRD 5.2.1
  locator: creature/skeleton
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
splat_stats:
  strength:
    value: 10
  dexterity:
    value: 16
  constitution:
    value: 15
  intelligence:
    value: 6
  wisdom:
    value: 8
  charisma:
    value: 5
  armor_class:
    value: 14
  max_hp:
    value: 13
  initiative:
    value: 3
  speed:
    value: 30
splat_health:
  zero_hp: dnd2024:monster
splat_traits:
  condition_immunities:
    rule: dnd2024:condition_immunities
    parameters: {conditions: [exhaustion, poisoned]}
  damage_affinities:
    rule: dnd2024:damage_affinities
    parameters:
      immunities:
      - poison
      vulnerabilities:
      - bludgeoning
splat_actions:
  shortsword:
    name: Shortsword
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
        modifier: 5
        scope: attack
      damage:
        expression: 1d6 + 3
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
  shortbow:
    name: Shortbow
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
        modifier: 5
        scope: attack
      damage:
        expression: 1d6 + 3
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

# Skeleton

Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.
