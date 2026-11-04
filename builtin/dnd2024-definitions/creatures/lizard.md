---
splat_schema: 2
splat_id: 2f497705-dfff-5dc6-a052-42a1327dcf87
splat_kind: creature
splat_key: dnd2024:creatures/lizard
splat_name: Lizard
splat_origin: dnd2024:reference/creature/lizard
splat_sources:
- SRD 5.2.1, pp. 357, https://www.dndbeyond.com/srd
---

# Lizard


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 2 (1d4) |
| Initiative | +0 (10) |
| Speed | 20 ft., Climb 20 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Senses | Darkvision 30 ft.; Passive Perception 9 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 2 | -4 | -4 |
| Dex | 11 | +0 | +0 |
| Con | 10 | +0 | +0 |
| Int | 1 | -5 | -5 |
| Wis | 8 | -1 | -1 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 357 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 10  Initiative +0 (10)\
> HP 2 (1d4)\
> Speed 20 ft., Climb 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 2 −4 −4 Dex 11 +0 +0 Con 10 +0 +0\
> Int 1 −5 −5 Wis 8 −1 −1 Cha 3 −4 −4\
> Senses Darkvision 30 ft.; Passive Perception 9\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Spider Climb. The lizard can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/357|page 357]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/lizard ` |
| ` implementation ` | ` dnd2024:creature_lizard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` darkvision ` | ` 30 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Spider climb

<!-- splat:values path=splat_rule/parameters/features/spider_climb -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` movement.climb ` |
| ` difficult_surfaces ` | ` true ` |
| ` ceilings ` | ` true ` |
| ` ability_check_required ` | ` false ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 2
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
>   value: 20
> ```

