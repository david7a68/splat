---
splat_schema: 2
splat_id: 5d350a08-a081-5ac9-b116-d8e9d941d9fc
splat_kind: creature
splat_key: dnd2024:creatures/giant_rat
splat_name: Giant Rat
splat_origin: dnd2024:reference/creature/giant_rat
splat_sources:
- SRD 5.2.1, pp. 353, https://www.dndbeyond.com/srd
---

# Giant Rat


## Stat block

| Stat | Value |
| --- | --- |
| Size | Small |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 7 (2d6) |
| Initiative | +3 (13) |
| Speed | 30 ft., Climb 30 ft. |
| Challenge | 1/8 (XP 25; PB +2) |
| Skills | Perception +2 |
| Senses | Darkvision 60 ft.; Passive Perception 12 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 7 | -2 | -2 |
| Dex | 16 | +3 | +5 |
| Con | 11 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 353 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Small Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 7 (2d6)\
> Speed 30 ft., Climb 30 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 7 −2 −2 Dex 16 +3 +5 Con 11 +0 +0\
> Int 2 −4 −4 Wis 10 +0 +0 Cha 4 −3 −3\
> Skills Perception +2\
> Senses Darkvision 60 ft.; Passive Perception 12\
> Languages None\
> CR 1/8 (XP 25; PB +2)


### Traits

Pack Tactics. The rat has Advantage on an attack roll against a creature if at least one of the rat’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated condition.

### Actions

Bite. Melee Attack Roll: +5, reach 5 feet. Hit: 5 (1d4 + 3) Piercing damage.

Original layout: [[../../srd-5.2.1/pages/353|page 353]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/giant_rat ` |
| ` implementation ` | ` dnd2024:creature_giant_rat ` |

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
| ` 0 ` | ` darkvision ` | ` 60 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Pack tactics

<!-- splat:yaml path=splat_rule/parameters/features/pack_tactics -->

```yaml
economy: passive
op: roll.advantage
rolls:
- attack
target_kind: creature
when:
  ally_within_target_ft: 5
  ally_excluded_conditions:
  - incapacitated
```

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
      dice:
        count: 1
        sides: 4
      add: 3
    average: 5
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

