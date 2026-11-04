---
splat_schema: 2
splat_id: 419b96b5-267e-51fa-83d8-2337a760cc3b
splat_kind: creature
splat_key: dnd2024:creatures/giant_weasel
splat_name: Giant Weasel
splat_origin: dnd2024:reference/creature/giant_weasel
splat_sources:
- SRD 5.2.1, pp. 355, https://www.dndbeyond.com/srd
---

# Giant Weasel


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 9 (2d8) |
| Initiative | +3 (13) |
| Speed | 40 ft., Climb 30 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Skills | Acrobatics +5, Perception +3, Stealth +5 |
| Senses | Darkvision 60 ft.; Passive Perception 13 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 17 | +3 | +3 |
| Con | 10 | +0 | +0 |
| Int | 4 | -3 | -3 |
| Wis | 12 | +1 | +1 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 355 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 9 (2d8)\
> Speed 40 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 11 +0 +0 Dex 17 +3 +3 Con 10 +0 +0\
> Int 4 −3 −3 Wis 12 +1 +1 Cha 5 −3 −3\
> Skills Acrobatics +5, Perception +3, Stealth +5\
> Senses Darkvision 60 ft.; Passive Perception 13\
> Languages None\
> CR 1/8 (XP 25; PB +2)


### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/355|page 355]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_weasel ` |
| ` implementation ` | ` dnd2024:creature_giant_weasel ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 13 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` acrobatics ` | ` 5 ` |
| ` perception ` | ` 3 ` |
| ` stealth ` | ` 5 ` |

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
attack_bonus: 5
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
        count: 1
        sides: 4
      add: 3
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
> climb_speed:
>   unit: ft
>   value: 30
> ```

