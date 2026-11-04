---
splat_schema: 2
splat_id: eaba7510-634f-5ec3-ab74-10343020a7ba
splat_kind: creature
splat_key: dnd2024:creatures/crab
splat_name: Crab
splat_origin: dnd2024:reference/creature/crab
splat_sources:
- SRD 5.2.1, pp. 347, https://www.dndbeyond.com/srd
---

# Crab


## Stat block

| Stat | Value |
| --- | --- |
| Size | Tiny |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 11 |
| Hit Points | 3 (1d4 + 1) |
| Initiative | +0 (10) |
| Speed | 20 ft., Swim 20 ft. |
| Challenge | 0 (XP 10; PB +2) |
| Skills | Stealth +2 |
| Senses | Blindsight 30 ft.; Passive Perception 9 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 6 | -2 | -2 |
| Dex | 11 | +0 | +0 |
| Con | 12 | +1 | +1 |
| Int | 1 | -5 | -5 |
| Wis | 8 | -1 | -1 |
| Cha | 2 | -4 | -4 |


## Source text

<!-- SRD page 347 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Tiny Beast, Unaligned\
> AC 11  Initiative +0 (10)\
> HP 3 (1d4 + 1)\
> Speed 20 ft., Swim 20 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str 6 −2 −2 Dex 11 +0 +0 Con 12 +1 +1\
> Int 1 −5 −5 Wis 8 −1 −1 Cha 2 −4 −4\
> Skills Stealth +2\
> Senses Blindsight 30 ft.; Passive Perception 9\
> Languages None\
> CR 0 (XP 10; PB +2)


### Traits

Amphibious. The crab can breathe air and water.

### Actions

Claw. Melee Attack Roll: +2, reach 5 ft. Hit: 1 Bludgeoning damage.

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
| ` mechanic ` | ` dnd2024:creatures/crab ` |
| ` implementation ` | ` dnd2024:creature_crab ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` passive_perception ` | ` 9 ` |

#### Skill totals

<!-- splat:values path=splat_rule/parameters/skill_totals -->

| Field | Value |
| --- | --- |
| ` stealth ` | ` 2 ` |

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

##### Amphibious

<!-- splat:values path=splat_rule/parameters/features/amphibious -->

| Field | Value |
| --- | --- |
| ` economy ` | ` passive ` |
| ` op ` | ` breathing.allow ` |
| ` media ` | ` ["air", "water"] ` |

##### Claw

<!-- splat:yaml path=splat_rule/parameters/features/claw -->

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
> swim_speed:
>   unit: ft
>   value: 20
> ```

