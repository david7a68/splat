---
splat_schema: 2
splat_id: 3af067d2-da31-5e22-ba37-bea6d7e884fb
splat_kind: creature
splat_key: dnd2024:creatures/giant_fire_beetle
splat_name: Giant Fire Beetle
splat_origin: dnd2024:reference/creature/giant_fire_beetle
splat_sources:
- SRD 5.2.1, pp. 351, https://www.dndbeyond.com/srd
---

# Giant Fire Beetle


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 4 (1d6 + 1) |
| Initiative | +0 (10) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Senses | Blindsight 30 ft.; Passive Perception 8 |
| Languages | None |
| Resistances | Fire |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 8 | -1 | -1 |
| Dex | 10 | +0 | +0 |
| Con | 12 | +1 | +1 |
| Int | 1 | -5 | -5 |
| Wis | 7 | -2 | -2 |
| Cha | 3 | -4 | -4 |


## Source text

<!-- SRD page 351 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Beast, Unaligned\
> AC 13  Initiative +0 (10)\
> HP 4 (1d6 + 1)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 8 −1 −1 Dex 10 +0 +0 Con 12 +1 +1\
> Int 1 −5 −5 Wis 7 −2 −2 Cha 3 −4 −4\
> Resistances Fire\
> Senses Blindsight 30 ft.; Passive Perception 8\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Illumination. The beetle sheds Bright Light in a 10-foot radius and Dim Light for an additional 10 feet.

### Actions

Bite. Melee Attack Roll: +1, reach 5 ft. Hit: 1 Fire damage.

Original layout: [[../../srd-5.2.1/pages/351|page 351]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_fire_beetle ` |
| ` implementation ` | ` dnd2024:creature_giant_fire_beetle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["fire"] ` |
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

##### Illumination

<!-- splat:values path=splat_rule/parameters/features/illumination -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` light.emit ` |
| ` bright_radius_ft ` | ` 10 ` |
| ` dim_additional_ft ` | ` 10 ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 1
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: fire
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
>   value: 30
> ```

