---
splat_schema: 2
splat_id: 15457a07-7534-57dd-8d2d-e338cc32d8bc
splat_kind: creature
splat_key: dnd2024:creatures/giant_centipede
splat_name: Giant Centipede
splat_origin: dnd2024:reference/creature/giant_centipede
splat_sources:
- SRD 5.2.1, pp. 349, 350, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 355, Habitat
---

# Giant Centipede


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 14 |
| Hit Points | 9 (2d6 + 2) |
| Initiative | +2 (12) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Blindsight 30 ft.; Passive Perception 8 |
| Languages | None |
| Habitat | Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 5 | -3 | -3 |
| Dex | 14 | +2 | +2 |
| Con | 12 | +1 | +1 |
| Int | 1 | -5 | -5 |
| Wis | 7 | -2 | -2 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 349 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Beast, Unaligned\
> AC 14  Initiative +2 (12)\
> HP 9 (2d6 + 2)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 5 −3 −3 Dex 14 +2 +2 Con 12 +1 +1\
> Int 1 −5 −5 Wis 7 −2 −2 Cha 3 −4 −4\
> Senses Blindsight 30 ft.; Passive Perception 8\
> Languages None\
> CR 1/4 (XP 50; PB +2)


<!-- SRD page 350 -->

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4 + 2) Piercing damage, and the target has the Poisoned condition until the start of the centipede’s next turn.

Original layout: [[../../srd-5.2.1/pages/349|page 349]], [[../../srd-5.2.1/pages/350|page 350]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_centipede ` |
| ` implementation ` | ` dnd2024:creature_giant_centipede ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 8 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 30 ` |

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
        sides: 4
      add: 2
    average: 4
- op: condition.apply
  condition: poisoned
  duration:
    until: actor.next_turn_start
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

