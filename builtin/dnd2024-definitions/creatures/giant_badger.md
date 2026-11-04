---
splat_schema: 2
splat_id: 3bc35e96-47b1-52f6-8872-2790bffce358
splat_kind: creature
splat_key: dnd2024:creatures/giant_badger
splat_name: Giant Badger
splat_origin: dnd2024:reference/creature/giant_badger
splat_sources:
- SRD 5.2.1, pp. 349, https://www.dndbeyond.com/srd
---

# Giant Badger


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 15 (2d8 + 6) |
| Initiative | +0 (10) |
| Speed | 30 ft., Burrow 10 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Perception +3 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |
| Resistances | Poison |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 13 | +1 | +1 |
| Dex | 10 | +0 | +0 |
| Con | 17 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 349 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 13  Initiative +0 (10)\
> HP 15 (2d8 + 6)\
> Speed 30 ft., Burrow 10 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 13 +1 +1 Dex 10 +0 +0 Con 17 +3 +3\
> Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
> Skills Perception +3\
> Resistances Poison\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Actions

Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/349|page 349]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_badger ` |
| ` implementation ` | ` dnd2024:creature_giant_badger ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["poison"] ` |
| ` passive_perception ` | ` 13 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 3 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

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
  - type: piercing
    amount:
      dice:
        count: 2
        sides: 4
      add: 1
    average: 6
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
> burrow_speed:
>   unit: ft
>   value: 10
> ```

