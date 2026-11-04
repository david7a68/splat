---
splat_schema: 2
splat_id: d6d1bfcf-d2a7-5ec1-a0c0-f17d5e56116c
splat_kind: creature
splat_key: dnd2024:creatures/riding_horse
splat_name: Riding Horse
splat_origin: dnd2024:reference/creature/riding_horse
splat_sources:
- SRD 5.2.1, pp. 360, https://www.dndbeyond.com/srd
---

# Riding Horse


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 13 (2d10 + 2) |
| Initiative | +1 (11) |
| Speed | 60 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Senses | Passive Perception 10 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 16 | +3 | +3 |
| Dex | 13 | +1 | +1 |
| Con | 12 | +1 | +1 |
| Int | 2 | -4 | -4 |
| Wis | 11 | +0 | +0 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 360 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 11  Initiative +1 (11)\
> HP 13 (2d10 + 2)\
> Speed 60 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 16 +3 +3 Dex 13 +1 +1 Con 12 +1 +1\
> Int 2 −4 −4 Wis 11 +0 +0 Cha 7 −2 −2\
> Senses Passive Perception 10\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Actions

Hooves. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/360|page 360]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/riding_horse ` |
| ` implementation ` | ` dnd2024:creature_riding_horse ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 10 ` |

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

##### Hooves

<!-- splat:yaml path=splat_rule/parameters/features/hooves -->

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
        sides: 8
      add: 3
    average: 7
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
