---
splat_schema: 1
splat_id: 37ab425c-b924-5442-a4d6-ec9a30294fb7
splat_kind: item
splat_key: dnd2024:equipment/hunting_trap_5_gp
splat_name: Hunting Trap (5 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97, 98
  url: https://www.dndbeyond.com/srd
---

# Hunting Trap (5 GP)

## Source text

<!-- SRD page 97 -->

As a Utilize action, you can set a Hunting Trap, which is a sawtooth steel ring that snaps shut when a creature steps on a pressure plate in the center. The trap is affixed by a heavy chain to an immobile object, such as a tree or a spike driven into the ground. A creature that steps on the plate must succeed on a DC 13 Dexterity saving throw or take 1d4 Piercing damage and have its Speed reduced to 0 until the start of its next turn. Thereafter, until the creature breaks free of the trap, its movement is limited by the length of the chain (typically 3 feet). A creature can use its action to make a DC 13 Strength (Athletics) check, freeing itself or another creature within its reach on a success. Each failed check deals 1 Piercing damage to the trapped creature.

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 25.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/hunting_trap_5_gp ` |
| ` implementation ` | ` dnd2024:equipment_hunting_trap_5_gp ` |

### Parameters

#### Set

<!-- splat:yaml path=splat_rule/parameters/set -->

```yaml
action: utilize
anchor: immobile_object
trigger: creature_steps_on_pressure_plate
save:
  ability: dexterity
  dc: 13
  success: no_effect
  failure:
    damage:
      amount:
        dice:
          count: 1
          sides: 4
        bonus: 0
      type: piercing
    speed_set: 0
    speed_expiry: target_next_turn_start
    afterward: movement_limited_to_chain_length
chain_length_ft:
  default: 3
  type: number
  authority: item_instance
```

#### Free

<!-- splat:yaml path=splat_rule/parameters/free -->

```yaml
economy: action
target: self_or_trapped_creature_in_reach
check:
  ability: strength
  skill: athletics
  dc: 13
success: free_target
failure:
  damage:
    target: trapped_creature
    amount: 1
    type: piercing
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 25.0 ` |
| ` unit ` | ` lb ` |
