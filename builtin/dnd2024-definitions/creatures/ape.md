---
splat_schema: 2
splat_id: c764ea50-08dd-54ef-a072-39043928808f
splat_kind: creature
splat_key: dnd2024:creatures/ape
splat_name: Ape
splat_origin: dnd2024:reference/creature/ape
splat_sources:
- SRD 5.2.1, pp. 344, https://www.dndbeyond.com/srd
---

# Ape


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 19 (3d8 + 6) |
| Initiative | +2 (12) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 1/2 (XP 100; PB +2) |
| Skills | Athletics +5, Perception +3 |
| Senses | Passive Perception 13 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 14 | +2 | +2 |
| Con | 14 | +2 | +2 |
| Int | 6 | -2 | -2 |
| Wis | 12 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 344 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 12 Initiative +2 (12)\
> HP 19 (3d8 + 6)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2\
> Int 6 −2 −2 Wis 12 +1 +1 Cha 7 −2 −2\
> Skills Athletics +5, Perception +3\
> Senses Passive Perception 13\
> Languages None\
> CR 1/2 (XP 100; PB +2)


### Actions

Multiattack. The ape makes two Fist attacks.

Fist. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Bludgeoning damage.

Rock (Recharge 6). Ranged Attack Roll: +5, range 25/50 ft. Hit: 10 (2d6 + 3) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/344|page 344]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/ape ` |
| ` implementation ` | ` dnd2024:creature_ape ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` athletics ` | ` 5 ` |
| ` perception ` | ` 3 ` |

#### Senses

<!-- splat:yaml path=splat_rule/parameters/senses -->

```yaml
[]
```

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Fist

<!-- splat:yaml path=splat_rule/parameters/features/fist -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 5
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 4
      add: 3
    average: 5
```

##### Rock

<!-- splat:yaml path=splat_rule/parameters/features/rock -->

```yaml
economy: action
op: attack
modes:
- ranged
attack_bonus: 5
target:
  kind: creature_or_object
  count: 1
range_ft:
  normal: 25
  long: 50
hit:
- op: damage
  components:
  - type: bludgeoning
    amount:
      dice:
        count: 2
        sides: 6
      add: 3
    average: 10
recharge:
  die_sides: 6
  success_min: 6
  success_max: 6
  check_event: actor.turn_start
  maximum_uses: 1
  initial_uses: 1
  also_recovers_after:
  - short_rest
  - long_rest
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: fist
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
> climb_speed:
>   unit: ft
>   value: 30
> ```

