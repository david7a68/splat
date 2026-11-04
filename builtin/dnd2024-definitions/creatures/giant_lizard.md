---
splat_schema: 2
splat_id: f0a62d29-44ed-555c-b85f-e48889fa370a
splat_kind: creature
splat_key: dnd2024:creatures/giant_lizard
splat_name: Giant Lizard
splat_origin: dnd2024:reference/creature/giant_lizard
splat_sources:
- SRD 5.2.1, pp. 352, https://www.dndbeyond.com/srd
---

# Giant Lizard


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 19 (3d10 + 3) |
| Initiative | +1 (11) |
| Speed | 40 ft., Climb 40 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 10 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 15 | +2 | +2 |
| Dex | 12 | +1 | +3 |
| Con | 13 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 352 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 12  Initiative +1 (11)\
> HP 19 (3d10 + 3)\
> Speed 40 ft., Climb 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 15 +2 +2 Dex 12 +1 +3 Con 13 +1 +1\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 5 −3 −3\
> Senses Darkvision 60 ft.; Passive Perception 10\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Traits

Spider Climb. The lizard can climb difficult surfaces, including along ceilings, without needing to make an ability check.

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/352|page 352]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_lizard ` |
| ` implementation ` | ` dnd2024:creature_giant_lizard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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
      dice:
        count: 1
        sides: 8
      add: 2
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
> climb_speed:
>   unit: ft
>   value: 40
> ```

