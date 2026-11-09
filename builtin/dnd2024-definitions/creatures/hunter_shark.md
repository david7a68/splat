---
splat_schema: 2
splat_id: 7beff13b-3807-593a-8d30-b4cb938a787b
splat_kind: creature
splat_key: dnd2024:creatures/hunter_shark
splat_name: Hunter Shark
splat_origin: dnd2024:reference/creature/hunter_shark
splat_sources:
- SRD 5.2.1, pp. 356, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 363, Habitat
---

# Hunter Shark


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 45 (6d10 + 12) |
| Initiative | +2 (12) |
| Speed | 5 ft., Swim 40 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +2 |
| Senses | Blindsight 60 ft.; Passive Perception 12 |
| Languages | None |
| Habitat | Underwater |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 18 | +4 | +4 |
| Dex | 14 | +2 | +2 |
| Con | 15 | +2 | +2 |
| Int | 1 | -5 | -5 |
| Wis | 10 | +0 | +0 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 356 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 45 (6d10 + 12)\
> Speed 5 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 18 +4 +4 Dex 14 +2 +2 Con 15 +2 +2\
> Int 1 −5 −5 Wis 10 +0 +0 Cha 4 −3 −3\
> Skills Perception +2\
> Senses Blindsight 60 ft.; Passive Perception 12\
> Languages None\
> CR 2 (XP 450; PB +2)


### Traits

Water Breathing. The shark can breathe only underwater.

### Actions

Bite. Melee Attack Roll: +6 (with Advantage if the target doesn’t have all its Hit Points), reach 5 ft. Hit: 14 (3d6 + 4) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/356|page 356]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/hunter_shark ` |
| ` implementation ` | ` dnd2024:creature_hunter_shark ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 12 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 2 ` |

#### Senses

<!-- splat:rows path=splat_rule/parameters/senses -->

| Row | sense | range_ft |
| --- | --- | --- |
| ` 0 ` | ` blindsight ` | ` 60 ` |

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
attack_bonus: 6
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
        count: 3
        sides: 6
      add: 4
    average: 14
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

