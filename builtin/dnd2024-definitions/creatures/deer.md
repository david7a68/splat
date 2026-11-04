---
splat_schema: 2
splat_id: ea02841c-a983-5671-9474-5f651ca54665
splat_kind: creature
splat_key: dnd2024:creatures/deer
splat_name: Deer
splat_origin: dnd2024:reference/creature/deer
splat_sources:
- SRD 5.2.1, pp. 347, https://www.dndbeyond.com/srd
---

# Deer


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 4 (1d8) |
| Initiative | +3 (13) |
| Speed | 50 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Perception +4 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 11 | +0 | +0 |
| Dex | 16 | +3 | +3 |
| Con | 11 | +0 | +0 |
| Int | 2 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 5 | -3 | -3 |


## Source text

<!-- SRD page 347 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 4 (1d8)\
> Speed 50 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 11 +0 +0 Dex 16 +3 +3 Con 11 +0 +0\
> Int 2 −4 −4 Wis 14 +2 +2 Cha 5 −3 −3\
> Skills Perception +4\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Agile. The deer doesn’t provoke an Opportunity Attack when it moves out of an enemy’s reach.

### Actions

Ram. Melee Attack Roll: +2, reach 5 ft. Hit: 2 (1d4) Bludgeoning damage.

Original layout: [[../../srd-5.2.1/pages/347|page 347]].







## Health

<!-- splat:values path=splat_health -->

| Field | Value |
| --- | --- |
| ` zero_hp ` | ` dnd2024:monster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:creatures/deer ` |
| ` implementation ` | ` dnd2024:creature_deer ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 14 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` perception ` | ` 4 ` |

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

##### Agile

<!-- splat:values path=splat_rule/parameters/features/agile -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` opportunity_attack.suppress ` |
| ` trigger ` | ` actor.leaves_enemy_reach ` |
| ` movement_modes ` | ` ["all"] ` |

##### Ram

<!-- splat:yaml path=splat_rule/parameters/features/ram -->

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
  - type: bludgeoning
    amount:
      dice:
        count: 1
        sides: 4
      add: 0
    average: 2
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
