---
splat_schema: 2
splat_id: 703a8a36-2e37-5b7f-bc16-0ae31fb12cc7
splat_kind: creature
splat_key: dnd2024:creatures/bandit
splat_name: Bandit
splat_origin: dnd2024:reference/creature/bandit
splat_sources:
- SRD 5.2.1, pp. 261, https://www.dndbeyond.com/srd
---

# Bandit


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium or Small |
| Type | Humanoid |
| Alignment | Neutral |
| Armor Class | 12 |
| Hit Points | 11 (2d8 + 2) |
| Initiative | +1 (11) |
| Speed | 30 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Senses | Passive Perception 10 |
| Languages | Common, Thieves’ Cant |
| Gear | Leather Armor, Light Crossbow, Scimitar |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 12 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 10 | +0 | +0 |
| Wis | 10 | +0 | +0 |
| Cha | 10 | +0 | +0 |


## Source text

<!-- SRD page 261 -->

Medium or Small Humanoid, Neutral\
> [!quote]- Printed stat block (not read; edit the table above)
> AC 12 Initiative +1 (11)\
> HP 11 (2d8 + 2)\
> Speed 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 11 +0 +0 Dex 12 +1 +1 Con 12 +1 +1\
> Int 10 +0 +0 Wis 10 +0 +0 Cha 10 +0 +0\
> Gear Leather Armor, Light Crossbow, Scimitar\
> Senses Passive Perception 10\
> Languages Common, Thieves’ Cant\
> CR 1/8 (XP 25; PB +2)


### Actions

Scimitar. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Slashing damage.

Light Crossbow. Ranged Attack Roll: +3, range 80/320 ft. Hit: 5 (1d8 + 1) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/261|page 261]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/bandit ` |
| ` implementation ` | ` dnd2024:creature_bandit ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

#### Gear

<!-- splat:table
path: splat_rule/parameters/gear
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Item:
    path: item
    type: symbol
  Count:
    path: count
    type: integer
-->

| Key | Item | Count |
| --- | --- | --- |
| ` row_1 ` | ` Leather armor ` | ` 1 ` |
| ` row_2 ` | ` Light crossbow ` | ` 1 ` |
| ` row_3 ` | ` Scimitar ` | ` 1 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` speak ` | ` true ` |
| ` known ` | ` ["common", "thieves_cant"] ` |

#### Features

##### Scimitar

<!-- splat:yaml path=splat_rule/parameters/features/scimitar -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 6
      add: 1
    average: 4
```

##### Light crossbow

<!-- splat:yaml path=splat_rule/parameters/features/light_crossbow -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 3
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 80
  long: 320
hit:
- op: damage
  components:
  - type: piercing
    amount:
      dice:
        count: 1
        sides: 8
      add: 1
    average: 5
```

#### Damage resolution

<!-- splat:values path=splat_rule/parameters/damage_resolution -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["printed_average", "roll_expression"] ` |
| ` chooser ` | ` controller ` |
| ` exclusive ` | ` true ` |

#### Spatial resolution

<!-- splat:values path=splat_rule/parameters/spatial_resolution -->

| Field | Value |
| --- | --- |
| ` provider ` | ` caller ` |
| ` required_facts ` | ` ["distance", "area_membership", "visibility", "occupancy", "movement_path"] ` |
| ` validate_against_typed_constraints ` | ` true ` |
