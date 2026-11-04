---
splat_schema: 2
splat_id: 37ed5db6-ae78-5ea1-a23b-84b36c7808fb
splat_kind: creature
splat_key: dnd2024:creatures/hippogriff
splat_name: Hippogriff
splat_origin: dnd2024:reference/creature/hippogriff
splat_sources:
- SRD 5.2.1, pp. 298, https://www.dndbeyond.com/srd
---

# Hippogriff


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Monstrosity |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 26 (4d10 + 4) |
| Initiative | +1 (11) |
| Speed | 40 ft., Fly 60 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Skills | Perception +5 |
| Senses | Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 17 | +3 | +3 |
| Dex | 13 | +1 | +1 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 8 | -1 | -1 |


## Source text

<!-- SRD page 298 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Monstrosity, Unaligned\
> AC 11 Initiative +1 (11)\
> HP 26 (4d10 + 4)\
> Speed 40 ft., Fly 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 17 +3 +3 Dex 13 +1 +1 Con 13 +1 +1\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 8 −1 −1\
> Skills Perception +5\
> Senses Passive Perception 15\
> Languages None\
> CR 1 (XP 200; PB +2)


### Traits

Flyby. The hippogriff doesn’t provoke an Opportunity Attack when it flies out of an enemy’s reach.

### Actions

Multiattack. The hippogriff makes two Rend attacks.

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/298|page 298]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/hippogriff ` |
| ` implementation ` | ` dnd2024:creature_hippogriff ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |

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

##### Flyby

<!-- splat:values path=splat_rule/parameters/features/flyby -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` opportunity_attack.suppress ` |
| ` trigger ` | ` actor.leaves_enemy_reach ` |
| ` movement_modes ` | ` ["fly"] ` |

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

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
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 8
      add: 3
    average: 7
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: rend
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
> ```

