---
splat_schema: 2
splat_id: a7e5a253-7e6a-582d-81f4-70d2a89a9d64
splat_kind: creature
splat_key: dnd2024:creatures/piranha
splat_name: Piranha
splat_origin: dnd2024:reference/creature/piranha
splat_sources:
- SRD 5.2.1, pp. 358, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 366, Habitat
---

# Piranha


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +3 (13) |
| Speed | 5 ft., Swim 40 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Senses | Darkvision 60 ft.; Passive Perception 8 |
| Languages | None |
| Habitat | Underwater |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 2 | -4 | -4 |
| Dex | 16 | +3 | +3 |
| Con | 9 | -1 | -1 |
| Int | 1 | -5 | -5 |
| Wis | 7 | -2 | -2 |
| Cha | 2 | -4 | -4 |


## Source text

<!-- SRD page 358 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 1 (1d4 − 1)\
> Speed 5 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 2 −4 −4 Dex 16 +3 +3 Con 9 −1 −1\
> Int 1 −5 −5 Wis 7 −2 −2 Cha 2 −4 −4\
> Senses Darkvision 60 ft.; Passive Perception 8\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Water Breathing. The piranha can breathe only underwater.

### Actions

Bite. Melee Attack Roll: +5 (with Advantage if the target doesn’t have all its Hit Points), reach 5 ft. Hit: 1 Piercing damage.

Original layout: [[../../srd-5.2.1/pages/358|page 358]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/piranha ` |
| ` implementation ` | ` dnd2024:creature_piranha ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 8 ` |

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

##### Water breathing

<!-- splat:values path=splat_rule/parameters/features/water_breathing -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["water"] ` |
| ` exclusive ` | ` true ` |

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
      constant: 1
    average: 1
advantage_when:
  target_hp_below_maximum: true
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
> swim_speed:
>   unit: ft
>   value: 40
> ```

