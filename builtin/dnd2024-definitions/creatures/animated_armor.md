---
splat_schema: 2
splat_id: 7714e47d-87df-57e5-8a58-61c51ede02a6
splat_kind: creature
splat_key: dnd2024:creatures/animated_armor
splat_name: Animated Armor
splat_origin: dnd2024:reference/creature/animated_armor
splat_sources:
- SRD 5.2.1, pp. 259, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 16, Habitat
---

# Animated Armor


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Construct |
| Alignment | Unaligned |
| Armor Class | 18 |
| Hit Points | 33 (6d8 + 6) |
| Initiative | +2 (12) |
| Speed | 25 ft. |
| Challenge | 1 (XP 200; PB +2) |
| Senses | Blindsight 60 ft.; Passive Perception 6 |
| Languages | None |
| Immunities | Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned |
| Habitat | Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 11 | +0 | +0 |
| Con | 13 | +1 | +1 |
| Int | 1 | -5 | -5 |
| Wis | 3 | -4 | -4 |
| Cha | 1 | -5 | -5 |


## Source text

<!-- SRD page 259 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Construct, Unaligned\
> AC 18 Initiative +2 (12)\
> HP 33 (6d8 + 6)\
> Speed 25 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 14 +2 +2 Dex 11 +0 +0 Con 13 +1 +1\
> Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
> Immunities Poison, Psychic; Charmed, Deafened, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned\
> Senses Blindsight 60 ft.; Passive Perception 6\
> Languages None\
> CR 1 (XP 200; PB +2)


### Actions

Multiattack. The armor makes two Slam attacks.

Slam. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/259|page 259]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/animated_armor ` |
| ` implementation ` | ` dnd2024:creature_animated_armor ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_immunities ` | ` ["poison", "psychic"] ` |
| ` condition_immunities ` | ` ["charmed", "deafened", "exhaustion", "frightened", "paralyzed", "petrified", "poisoned"] ` |
| ` passive_perception ` | ` 6 ` |

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

##### Slam

<!-- splat:yaml path=splat_rule/parameters/features/slam -->

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
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 6
      add: 2
    average: 5
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: slam
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
