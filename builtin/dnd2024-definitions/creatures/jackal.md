---
splat_schema: 2
splat_id: 3bfdbb1f-f2d2-523b-94c2-aa6f9d8362d4
splat_kind: creature
splat_key: dnd2024:creatures/jackal
splat_name: Jackal
splat_origin: dnd2024:reference/creature/jackal
splat_sources:
- SRD 5.2.1, pp. 356, https://www.dndbeyond.com/srd
---

# Jackal


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 12 |
| Hit Points | 3 (1d6) |
| Initiative | +2 (12) |
| Speed | 40 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +5, Stealth +4 |
| Senses | Darkvision 90 ft.; Passive Perception 15 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 8 | -1 | -1 |
| Dex | 15 | +2 | +2 |
| Con | 11 | +0 | +0 |
| Int | 3 | -4 | -4 |
| Wis | 12 | +1 | +1 |
| Cha | 6 | -2 | -2 |


## Source text

<!-- SRD page 356 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Beast, Unaligned\
> AC 12  Initiative +2 (12)\
> HP 3 (1d6)\
> Speed 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 8 −1 −1 Dex 15 +2 +2 Con 11 +0 +0\
> Int 3 −4 −4 Wis 12 +1 +1 Cha 6 −2 −2\
> Skills Perception +5, Stealth +4\
> Senses Darkvision 90 ft.; Passive Perception 15\
> Languages None\
> CR 0 (XP 10; PB +2)


### Actions

Bite. Melee Attack Roll: +1, reach 5 ft. Hit: 1 (1d4 – 1) Piercing damage.

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
| ` mechanic ` | ` dnd2024:creatures/jackal ` |
| ` implementation ` | ` dnd2024:creature_jackal ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
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
| ` 0 ` | ` darkvision ` | ` 90 ` |

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
attack_bonus: 1
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
      add: -1
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
