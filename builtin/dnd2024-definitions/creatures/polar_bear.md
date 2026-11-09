---
splat_schema: 2
splat_id: 73aa9e60-ae93-5f97-b620-7ddc21ed2e3b
splat_kind: creature
splat_key: dnd2024:creatures/polar_bear
splat_name: Polar Bear
splat_origin: dnd2024:reference/creature/polar_bear
splat_sources:
- SRD 5.2.1, pp. 359, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 367, Habitat
---

# Polar Bear


## Stat block

| Stat | Value |
| --- | --- |
| Size | Large |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 42 (5d10 + 15) |
| Initiative | +2 (12) |
| Speed | 40 ft., Swim 40 ft. |
| Challenge | 2 (XP 450; PB +2) |
| Skills | Perception +5, Stealth +4 |
| Senses | Darkvision 60 ft.; Passive Perception 15 |
| Languages | None |
| Resistances | Cold |
| Habitat | Arctic |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 20 | +5 | +5 |
| Dex | 14 | +2 | +2 |
| Con | 16 | +3 | +3 |
| Int | 2 | -4 | -4 |
| Wis | 13 | +1 | +1 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 359 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Large Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 42 (5d10 + 15)\
> Speed 40 ft., Swim 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 20 +5 +5 Dex 14 +2 +2 Con 16 +3 +3\
> Int 2 −4 −4 Wis 13 +1 +1 Cha 7 −2 −2\
> Skills Perception +5, Stealth +4\
> Resistances Cold\
> Senses Darkvision 60 ft.; Passive Perception 15\
> Languages None\
> CR 2 (XP 450; PB +2)


### Actions

Multiattack. The bear makes two Rend attacks.

Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 9 (1d8 + 5) Slashing damage.

Original layout: [[../../srd-5.2.1/pages/359|page 359]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/polar_bear ` |
| ` implementation ` | ` dnd2024:creature_polar_bear ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` damage_resistances ` | ` ["cold"] ` |
| ` passive_perception ` | ` 15 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 5 ` |
| ` stealth ` | ` 4 ` |

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

##### Rend

<!-- splat:yaml path=splat_rule/parameters/features/rend -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 7
target:
  kind: creature_or_object
  count: 1
reach_ft: 5
hit:
- op: damage
  components:
  - type: slashing
    amount:
      dice:
        count: 1
        sides: 8
      add: 5
    average: 9
```

##### Multiattack

<!-- splat:yaml path=splat_rule/parameters/features/multiattack -->

```yaml
economy: action
op: multiattack
sequence:
- count: 2
  feature: rend
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

