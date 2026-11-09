---
splat_schema: 2
splat_id: 1d7d0518-86d7-59cf-8a19-085cea7e7829
splat_kind: creature
splat_key: dnd2024:creatures/rat
splat_name: Rat
splat_origin: dnd2024:reference/creature/rat
splat_sources:
- SRD 5.2.1, pp. 359, https://www.dndbeyond.com/srd
- Monster Manual (2024), p. 367, Habitat
---

# Rat


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 10 |
| Hit Points | 1 (1d4 - 1) |
| Initiative | +0 (10) |
| Speed | 20 ft., Climb 20 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +2 |
| Senses | Darkvision 30 ft.; Passive Perception 12 |
| Languages | None |
| Habitat | Forest, Swamp, Underdark, Urban |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 2 | -4 | -4 |
| Dex | 11 | +0 | +0 |
| Con | 9 | -1 | -1 |
| Int | 2 | -4 | -4 |
| Wis | 10 | +0 | +0 |
| Cha | 4 | -3 | -3 |


## Source text

<!-- SRD page 359 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 10  Initiative +0 (10)\
> HP 1 (1d4 − 1)\
> Speed 20 ft., Climb 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 2 −4 −4 Dex 11 +0 +0 Con 9 −1 −1\
> Int 2 −4 −4 Wis 10 +0 +0 Cha  4 −3 −3\
> Skills Perception +2\
> Senses Darkvision 30 ft.; Passive Perception 12\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Agile. The rat doesn’t provoke an Opportunity Attack when it moves out of an enemy’s reach.

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Piercing damage.

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
| ` mechanic ` | ` dnd2024:creatures/rat ` |
| ` implementation ` | ` dnd2024:creature_rat ` |

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
| ` 0 ` | ` darkvision ` | ` 30 ` |

#### Languages

<!-- splat:yaml path=splat_rule/parameters/languages -->

```yaml
speak: true
known: []
```

#### Features

##### Agile

<!-- splat:values path=splat_rule/parameters/features/agile -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` opportunity_attack.suppress ` |
| ` trigger ` | ` actor.leaves_enemy_reach ` |
| ` movement_modes ` | ` ["all"] ` |

##### Bite

<!-- splat:yaml path=splat_rule/parameters/features/bite -->

```yaml
economy: action
op: attack
modes:
- melee
attack_bonus: 2
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
>   value: 20
> ```

