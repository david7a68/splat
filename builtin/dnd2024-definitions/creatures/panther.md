---
splat_schema: 2
splat_id: 6cdc34e3-3dda-5178-8857-75a5d374f66a
splat_kind: creature
splat_key: dnd2024:creatures/panther
splat_name: Panther
splat_origin: dnd2024:reference/creature/panther
splat_sources:
- SRD 5.2.1, pp. 358, https://www.dndbeyond.com/srd
---

# Panther


## Stat block

| Stat | Value |
| --- | --- |
| Size | Medium |
| Type | Beast |
| Alignment | Unaligned |
| Armor Class | 13 |
| Hit Points | 13 (3d8) |
| Initiative | +3 (13) |
| Speed | 50 ft., Climb 40 ft. |
| Challenge | 1/4 (XP 50; PB +2) |
| Skills | Perception +4, Stealth +7 |
| Senses | Darkvision 60 ft.; Passive Perception 14 |
| Languages | None |

| Ability | Score | Mod | Save |
| --- | --- | --- | --- |
| Str | 14 | +2 | +2 |
| Dex | 16 | +3 | +3 |
| Con | 10 | +0 | +0 |
| Int | 3 | -4 | -4 |
| Wis | 14 | +2 | +2 |
| Cha | 7 | -2 | -2 |


## Source text

<!-- SRD page 358 -->

> [!quote]- Printed stat block (not read; edit the table above)
> Medium Beast, Unaligned\
> AC 13  Initiative +3 (13)\
> HP 13 (3d8)\
> Speed 50 ft., Climb 40 ft.\
> MOD SAVE MOD SAVE MOD SAVE\
> Str  14 +2 +2 Dex 16 +3 +3 Con 10 +0 +0\
> Int 3 −4 −4 Wis 14 +2 +2 Cha 7 −2 −2\
> Skills Perception +4, Stealth +7\
> Senses Darkvision 60 ft.; Passive Perception 14\
> Languages None\
> CR 1/4 (XP 50; PB +2)


### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Slashing damage.

### Bonus Actions

Nimble Escape. The panther takes the Disengage or Hide action.

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
| ` mechanic ` | ` dnd2024:creatures/panther ` |
| ` implementation ` | ` dnd2024:creature_panther ` |

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
| ` stealth ` | ` 7 ` |

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
attack_bonus: 5
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
        sides: 6
      add: 3
    average: 6
```

##### Nimble escape

<!-- splat:values path=splat_rule/parameters/features/nimble_escape -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` op ` | ` action.choose ` |
| ` choices ` | ` ["disengage", "hide"] ` |

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
>   value: 40
> ```

