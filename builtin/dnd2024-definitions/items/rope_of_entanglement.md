---
splat_schema: 1
splat_id: cecad3c6-628e-59ac-8143-0156183f5cb2
splat_kind: item
splat_key: dnd2024:items/rope_of_entanglement
splat_name: Rope of Entanglement
splat_origin:
  key: dnd2024:reference/magic_item/rope_of_entanglement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242, 243
  url: https://www.dndbeyond.com/srd
---

# Rope of Entanglement

## Source text

<!-- SRD page 242 -->

Wondrous Item, Rare

This rope is 30 feet long. While holding one end of the rope, you can take a Magic action to command the other end to dart forward and entangle one creature you can see within 20 feet of yourself. The target must succeed on a DC 15 Dexterity saving throw or have the Restrained condition. You can release the target by letting go of your end of the rope (causing the rope to coil up in the target’s space) or by using a Bonus Action to repeat the command (causing the rope to coil up in your hand).

<!-- SRD page 243 -->

A target Restrained by the rope can take an action to make its choice of a DC 15 Strength (Athletics) or Dexterity (Acrobatics) check. On a successful check, the target is no longer Restrained by the rope. If you’re still holding onto the rope when a target escapes from it, you can take a Reaction to command the rope to coil up in your hand; otherwise, the rope coils up in the target’s space.

The rope has AC 20, HP 20, and Immunity to Poison and Psychic damage. It regains 1 Hit Point every 5 minutes as long as it has at least 1 Hit Point. If the rope drops to 0 Hit Points, it is destroyed.

Original layout: [[../../srd-5.2.1/pages/242|page 242]], [[../../srd-5.2.1/pages/243|page 243]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/rope_of_entanglement ` |
| ` implementation ` | ` dnd2024:item_rope_of_entanglement ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` length_ft ` | ` 30 ` |
| ` equipped ` | ` holding_one_end ` |
| ` restrained_ends_on ` | ` ["rope_destroyed", "release", "successful_escape_check"] ` |
| ` attunement_required ` | ` false ` |

#### Activate

<!-- splat:yaml path=splat_rule/parameters/activate -->

```yaml
action: magic
command: true
target:
  visible: true
  range_ft: 20
  creature_count: 1
save:
  ability: dexterity
  dc: 15
  failure:
    condition: restrained
```

#### Release options

<!-- splat:table
path: splat_rule/parameters/release_options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Trigger:
    path: trigger
    type: symbol
    optional: true
  Rope location:
    path: rope_location
    type: symbol
  Rope state:
    path: rope_state
    type: symbol
  Action:
    path: action
    type: symbol
    optional: true
  Repeat command:
    path: repeat_command
    type: boolean
    optional: true
-->

| Key | Trigger | Rope location | Rope state | Action | Repeat command |
| --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` User lets go ` | ` Target space ` | ` Coiled ` |  |  |
| ` row_2 ` |  | ` User hand ` | ` Coiled ` | ` Bonus action ` | true |

#### Escape

<!-- splat:values path=splat_rule/parameters/escape -->

| Field | Value |
| --- | --- |
| ` action ` | ` action ` |
| ` success ` | ` end_restrained ` |
| ` otherwise_rope_location ` | ` target_space ` |

##### Check choice

<!-- splat:table
path: splat_rule/parameters/escape/check_choice
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Ability:
    path: ability
    type: symbol
  Skill:
    path: skill
    type: symbol
  Dc:
    path: dc
    type: integer
-->

| Key | Ability | Skill | Dc |
| --- | --- | --- | --- |
| ` row_1 ` | ` Strength ` | ` Athletics ` | ` 15 ` |
| ` row_2 ` | ` Dexterity ` | ` Acrobatics ` | ` 15 ` |

##### User reaction

<!-- splat:values path=splat_rule/parameters/escape/user_reaction -->

| Field | Value |
| --- | --- |
| ` requires ` | ` holding_rope ` |
| ` command ` | ` coil_into_user_hand ` |

#### Object statistics

<!-- splat:yaml path=splat_rule/parameters/object_statistics -->

```yaml
ac: 20
hp: 20
damage_immunities:
- poison
- psychic
healing:
  hp: 1
  interval:
    amount: 5
    unit: minutes
  requires_hp_minimum: 1
zero_hp: destroyed
```
