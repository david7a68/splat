---
splat_schema: 2
splat_id: 344926f3-0b11-5607-90f7-157c741d6038
splat_kind: creature
splat_key: dnd2024:creatures/flying_snake
splat_name: Flying Snake
splat_origin: dnd2024:reference/creature/flying_snake
splat_sources:
- SRD 5.2.1, pp. 348, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 353, Habitat
---

# Flying Snake


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 5 (2d4) |
| Initiative | +2 (12) |
| Speed | 30 ft., Fly 60 ft., Swim 30 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Senses | Blindsight 10 ft.; Passive Perception 11 |
| Languages | None |
| Habitat | Desert, Forest, Grassland |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 4 | -3 | -3 |
| Dex | 15 | +2 | +2 |
| Con | 11 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 348 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Monstrosity, Unaligned\
> AC 14  Initiative +2 (12)\
> HP 5 (2d4)\
> Speed 30 ft., Fly 60 ft., Swim 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 4 −3 −3 Dex 15 +2 +2 Con 11 +0 +0\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
> Senses Blindsight 10 ft.; Passive Perception 11\
> Languages None\
> CR 1/8 (XP 25; PB +2)


### Traits

Flyby. The snake doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing damage plus 5 (2d4) Poison damage.

Original layout: [[../../srd-5.2.1/pages/348|page 348]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/flying_snake ` |
| ` implementation ` | ` dnd2024:creature_flying_snake ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 11 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 10 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Flyby

<!-- splat:values path=splat_rule/parameters/features/flyby -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` opportunity_attack.suppress ` |
| ` trigger ` | ` actor.leaves_enemy_reach ` |
| ` movement_modes ` | ` ["fly"] ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 4
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: piercing
    amount:
      constant: 1
    average: 1
  - type: poison
    amount:
      dice:
        count: 2
        sides: 4
      add: 0
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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_stats_extras -->
> 
> ```yaml
> fly_speed:
>   unit: ft
>   value: 60
> swim_speed:
>   unit: ft
>   value: 30
> ```

